use std::io::stdout;

use crate::all::Ref::{RCon, RTmp, R};
use crate::all::TmpWdth::{Wsb, Wsh, Wsw, Wub, Wuh, Wuw};
use crate::all::K::{Kl, Kw, Kx};
use crate::all::{
    bit, bshas, for_each_blk_mut, isext, kbase, to_s, BSet, Bits, Blk, BlkIdx, Con, Fn, Ins, Phi,
    PhiIdx, Ref, RpoIdx, Tmp, TmpIdx, Typ, Use, UseT, O, TMP0, UNDEF,
};
use crate::cfg::dom;
use crate::parse::{printfn, printref};
use crate::util::{bscount, bsdiff, bsinit, bsiter, bsset, Bucket};

fn iscon(cons: &[Con], r: Ref, bits: i64) -> bool {
    if let RCon(ci) = r {
        let con: &Con = &cons[ci];
        if let Con::CBits(i, _) = con {
            return bits == *i;
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
    blks: &mut [Blk],
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
                        let i: &Ins = &blks[u.bi].ins[ii];
                        if iscopy(tmps, cons, i, r) {
                            p = p0;
                            assert!(matches!(i.to, RTmp(_)));
                            if let RTmp(ti0) = i.to {
                                ti = ti0;
                            }
                        }
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
    let blks: &mut [Blk] = &mut f.blks;
    let rpo: &[BlkIdx] = &f.rpo;
    let phis: &mut [Phi] = &mut f.phis;
    let tmps: &[Tmp] = &f.tmps;
    let cons: &[Con] = &f.cons;

    let mut cpy: Vec<Ref> = vec![R; tmps.len()];

    assert!(f.nblk as usize == rpo.len());
    /* 1. build the copy-of map */
    for n in 0..rpo.len() {
        let bi: BlkIdx = rpo[n];
        //let b: &Blk = &blks[bi];
        // println!("Building copy-map using @{}", to_s(&b.name));
        let mut pi: PhiIdx = blks[bi].phi;
        while pi != PhiIdx::NONE {
            let p: &Phi = &phis[pi];
            assert!(matches!(p.to, RTmp(_)));
            // println!("  phi for {:?}", p.to);
            if let RTmp(ti) = p.to {
                // println!("      it is for %{}", to_s(&tmps[ti].name));
                if cpy[ti.0 as usize] != R {
                    // are we gonna allow TmpIdx indexing???
                    // println!("        ... cpy is already {:?}", cpy[ti.0 as usize]);
                    pi = p.link;
                    continue;
                }
                let mut eq: usize = 0;
                let mut r: Ref = R;
                for a in 0..p.args.len() {
                    let abi: BlkIdx = p.blks[a];
                    let ab: &Blk = &blks[abi];
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
                    cpy[ti.usize()] = p.to;
                } else if eq == p.args.len() {
                    cpy[ti.usize()] = r;
                } else {
                    cpy[ti.usize()] = p.to;
                    phisimpl(blks, tmps, phis, cons, pi, r, &mut cpy);
                }
            }
            pi = p.link;
        }
        for i in &blks[bi].ins {
            assert!(i.to == R || matches!(i.to, RTmp(_)));
            if let RTmp(ti) = i.to {
                // println!("  ins for {:?} %{}", i.to, to_s(&tmps[ti].name));
                let r: Ref = copyof(i.args[0], &cpy);
                if iscopy(tmps, cons, i, r) {
                    cpy[ti.usize()] = r;
                } else {
                    cpy[ti.usize()] = i.to;
                }
            }
        }
    }

    /* 2. remove redundant phis/copies
     * and rewrite their uses */
    for_each_blk_mut(blks, |b| {
        // println!("Copy elimination on @{}", to_s(&b.name));
        let mut ppi: PhiIdx = PhiIdx::NONE;
        let mut pi: PhiIdx = b.phi;
        while pi != PhiIdx::NONE {
            let p_to: Ref = phis[pi].to;
            let p_link: PhiIdx = phis[pi].link;
            assert!(matches!(p_to, RTmp(_)));
            if let RTmp(ti) = p_to {
                // println!("  phi for %{}", to_s(&tmps[ti].name));
                // println!("      it is for %{}", to_s(&tmps[ti].name));
                let r: Ref = cpy[ti.usize()];
                // if let RTmp(rti) = r {
                //     println!("          copy r is %{}", to_s(&tmps[rti].name))
                // }
                // println!("          copy r is {:?}", r);
                if r != p_to {
                    // println!("              removing phi!");
                    if ppi == PhiIdx::NONE {
                        // println!("              ... it is the first phi!");
                        b.phi = p_link;
                    } else {
                        // println!("              ... it is not the first phi!");
                        phis[ppi].link = p_link;
                    }
                    pi = p_link;
                    continue;
                }
            }
            for a in &mut phis[pi].args {
                // println!("          arg subst for {:?}", *a);
                subst(a, &cpy);
            }
            ppi = pi;
            pi = p_link;
        }
        for i in &mut b.ins {
            // Hrmmm, this only works for RTmp - what about QBE and void ops?
            // assert!(matches!(i.to, RTmp(_)));
            if let RTmp(ti) = i.to {
                let r: Ref = cpy[ti.usize()];
                if r != i.to {
                    *i = Ins::NOP;
                    continue;
                }
            }
            subst(&mut i.args[0], &cpy);
            subst(&mut i.args[1], &cpy);
        }
        subst(&mut b.jmp.arg, &cpy);
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
            } else if cpy[tii] != RTmp(TmpIdx::from(tii)) {
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
