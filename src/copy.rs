use std::io::stdout;

use crate::all::Ref::{RCon, RTmp, R};
use crate::all::TmpWdth::{Wsb, Wsh, Wsw, Wub, Wuh, Wuw};
use crate::all::K::{Kl, Kw, Kx};
use crate::all::{
    bit, bshas, isext, kbase, to_s, BSet, Bits, BlkIdx, Blks, Con, ConBits, ConT, Fn, Ins, Phi,
    PhiIdx, Ref, RpoIdx, Tmp, TmpIdx, Typ, Use, UseT, O, TMP0, UNDEF,
};
use crate::cfg::dom;
use crate::parse::{printfn, printref};
use crate::util::{bscount, bsdiff, bsinit, bsiter, bsset, Bucket};

fn iscon(cons: &[Con], r: Ref, bits: i64) -> bool {
    if let RCon(ci) = r {
        let con: &Con = &cons[ci.0 as usize];
        if con.typ == ConT::CBits {
            if let ConBits::I(i) = con.bits {
                return bits == i;
            }
        }
    }
    false
}

const EXTCPY: [Bits; Wuw as usize + 1] = [
    /*[WFull]*/ 0,
    /*[Wsb]*/ bit(Wsb as usize) | bit(Wsh as usize) | bit(Wsw as usize),
    /*[Wub]*/ bit(Wub as usize) | bit(Wuh as usize) | bit(Wuw as usize),
    /*[Wsh]*/ bit(Wsh as usize) | bit(Wsw as usize),
    /*[Wuh]*/ bit(Wuh as usize) | bit(Wuw as usize),
    /*[Wsw]*/ bit(Wsw as usize),
    /*[Wuw]*/ bit(Wuw as usize),
];

fn iscopy(tmps: &[Tmp], cons: &[Con], i: &Ins, r: Ref) -> bool {
    match i.op {
        O::Ocopy => return true,
        O::Omul | O::Odiv | O::Oudiv => return iscon(cons, i.args[1], 1),
        O::Oadd | O::Osub | O::Oor | O::Oxor | O::Osar | O::Oshl | O::Oshr => {
            return iscon(cons, i.args[1], 0);
        }
        _ => (),
    }
    if !isext(i.op) {
        return false;
    }
    if let RTmp(ti) = r {
        if (i.op == O::Oextsw || i.op == O::Oextuw) && i.cls == Kw {
            return true;
        }
        let t: &Tmp = &tmps[ti];
        assert!(kbase(t.cls) == 0);
        if i.cls == Kl && t.cls == Kw {
            return false;
        }
        let b: Bits = EXTCPY[t.width as usize];
        return (bit((Wsb as usize) + ((i.op as usize) - (O::Oextsb as usize))) & b) != 0;
    }
    false
}

fn copyof(r: Ref, cpy: &[Ref]) -> Ref {
    if let RTmp(ti) = r {
        if cpy[ti.usize()] != R {
            return cpy[ti.usize()];
        }
    }
    r
}

/* detects a cluster of phis/copies redundant with 'r';
 * the algorithm is inspired by Section 3.2 of "Simple
 * and Efficient SSA Construction" by Braun M. et al.
 */
fn phisimpl(
    blks: &Blks,
    tmps: &[Tmp],
    phis: &[Phi],
    cons: &[Con],
    pi: PhiIdx,
    r: Ref,
    cpy: &mut [Ref],
) {
    let mut p: &Phi = &phis[pi];
    let mut ti: TmpIdx = TmpIdx::NONE;
    let mut ts: BSet = bsinit(tmps.len());
    let mut as_: BSet = bsinit(tmps.len());
    let p0 = &mut Phi::new(R, vec![], vec![], Kx, PhiIdx::NONE);
    let mut stk: Vec<Use> = vec![];
    stk.push(Use::new(UseT::UPhi(pi), BlkIdx::NONE, RpoIdx::NONE));
    loop {
        match stk.pop() {
            None => break,
            Some(u) => {
                match u.typ {
                    UseT::UIns(ii) => {
                        blks.with(u.bi, |b| {
                            let i: &Ins = &b.ins()[ii];
                            if iscopy(tmps, cons, i, r) {
                                p = p0;
                                assert!(matches!(i.to, RTmp(_)));
                                if let RTmp(ti0) = i.to {
                                    ti = ti0;
                                }
                            }
                        });
                    }
                    UseT::UPhi(pi) => {
                        p = &phis[pi];
                        assert!(matches!(p.to, RTmp(_)));
                        if let RTmp(ti0) = p.to {
                            ti = ti0;
                        }
                    }
                    _ => continue,
                }
                if bshas(&ts, ti.usize()) {
                    continue;
                }
                bsset(&mut ts, ti.usize());
                for a in &p.args {
                    let r1: Ref = copyof(*a, cpy);
                    if r1 == r {
                        continue;
                    }
                    if let RTmp(r1ti) = r1 {
                        bsset(&mut as_, r1ti.usize());
                    } else {
                        return;
                    }
                }
                stk.extend_from_slice(&tmps[ti].uses);
            }
        }
    }
    bsdiff(&mut as_, &ts);
    if bscount(&as_) == 0 {
        let mut tii: usize = 0;
        while bsiter(&ts, &mut tii) {
            cpy[tii] = r;
            tii += 1;
        }
    }
}

fn subst(pr: &mut Ref, cpy: &[Ref]) {
    assert!({
        if let RTmp(ti) = *pr {
            cpy[ti.usize()] != R
        } else {
            true
        }
    });
    *pr = copyof(*pr, cpy);
}

/* requires use and dom, breaks use */
pub fn copy(f: &mut Fn, typ: &[Typ], itbl: &[Bucket]) {
    let blks: &Blks = &f.blks;
    let rpo: &[BlkIdx] = &f.rpo;
    let phis: &mut [Phi] = &mut f.phis;
    let tmps: &[Tmp] = &f.tmps;
    let cons: &[Con] = &f.cons;
    // BSet ts[1], as[1];
    // Use **stk;
    // Phi *p, **pp;
    // Ins *i;
    // Blk *b;
    // uint n, a, eq;
    // Ref *cpy, r, r1;
    // int t;

    let mut cpy: Vec<Ref> = vec![R; tmps.len()];

    assert!(f.nblk as usize == rpo.len());
    /* 1. build the copy-of map */
    // for (n=0; n<fn->nblk; n++) {
    for n in 0..rpo.len() {
        //     b = fn->rpo[n];
        let bi: BlkIdx = rpo[n];
        blks.with(bi, |b| {
            //     for (p=b->phi; p; p=p->link) {
            let mut pi: PhiIdx = b.phi;
            while pi != PhiIdx::NONE {
                let p: &Phi = &phis[pi];
                //         assert(rtype(p->to) == RTmp);
                assert!(matches!(p.to, RTmp(_)));
                if let RTmp(ti) = p.to {
                    if cpy[ti.0 as usize] != R {
                        // are we gonna allow TmpIdx indexing???
                        continue;
                    }
                    let mut eq: usize = 0;
                    let mut r: Ref = R;
                    //         for (a=0; a<p->narg; a++)
                    for a in 0..p.args.len() {
                        let abi: BlkIdx = p.blks[a];
                        let ab = blks.borrow(abi);
                        if ab.id.usize() < n {
                            let r1: Ref = copyof(p.args[a], &cpy);
                            if r == R || r == UNDEF {
                                r = r1;
                            }
                            if r1 == r || r1 == UNDEF {
                                eq += 1;
                            }
                        }
                    }
                    assert!(r != R);
                    let mut rti: TmpIdx = TmpIdx::NONE;
                    if let RTmp(rti0) = r {
                        rti = rti0;
                    }
                    if rti != TmpIdx::NONE && !dom(blks, rpo[tmps[rti].bid], bi) {
                        cpy[rti.usize()] = p.to;
                    } else if eq == p.args.len() {
                        cpy[ti.usize()] = r;
                    } else {
                        cpy[ti.usize()] = p.to;
                        phisimpl(blks, tmps, phis, cons, pi, r, &mut cpy);
                    }
                    for i in b.ins().iter() {
                        assert!(i.to == R || matches!(i.to, RTmp(_)));
                        if let RTmp(ti) = i.to {
                            r = copyof(i.args[0], &cpy);
                            if iscopy(tmps, cons, i, r) {
                                cpy[ti.usize()] = r;
                            } else {
                                cpy[ti.usize()] = i.to;
                            }
                        }
                    }
                }
                pi = p.link;
            }
        });
    }

    /* 2. remove redundant phis/copies
     * and rewrite their uses */
    let mut ppi: PhiIdx = PhiIdx::NONE;
    blks.for_each_mut(|b| {
        let mut pi: PhiIdx = b.phi;
        while pi != PhiIdx::NONE {
            //let p: &mut Phi = &mut phis[pi];
            let p_to: Ref = phis[pi].to;
            let p_link: PhiIdx = phis[pi].link;
            assert!(matches!(p_to, RTmp(_)));
            if let RTmp(ti) = p_to {
                let r: Ref = cpy[ti.usize()];
                if r == p_to {
                    for a in &mut phis[pi].args {
                        subst(a, &cpy);
                    }
                } else {
                    if ppi == PhiIdx::NONE {
                        b.phi = p_link;
                    } else {
                        phis[ppi].link = p_link;
                    }
                }
            }
            ppi = pi;
            pi = p_link;
        }
        for i in &mut b.ins_mut().iter_mut() {
            // Hrmmm, this only works for RTmp - what about QBE and void ops?
            // assert!(matches!(i.to, RTmp(_)));
            if let RTmp(ti) = i.to {
                let r: Ref = cpy[ti.usize()];
                if r != i.to {
                    *i = Ins::NOP;
                    continue;
                }
                subst(&mut i.args[0], &cpy);
                subst(&mut i.args[1], &cpy);
            }
        }
        subst(&mut b.jmp.borrow_mut().arg, &cpy);
    });

    if true
    /*debug['C']*/
    {
        /*e*/
        print!("\n> Copy information:");
        for tii in TMP0..tmps.len() {
            if cpy[tii] == R {
                /*e*/
                print!("\n{:>10} not seen!", to_s(&tmps[tii].name));
            } else if !matches!(cpy[tii], RTmp(_)) {
                /*e*/
                print!("\n{:>10} copy of ", to_s(&tmps[tii].name));
                printref(/*stderr*/ &mut stdout(), f, typ, itbl, cpy[tii]);
            }
        }
        /*e*/
        println!("\n\n> After copy elimination:");
        printfn(/*stderr*/ &mut stdout(), f, typ, itbl);
    }
}
