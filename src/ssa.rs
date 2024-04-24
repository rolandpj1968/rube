use derive_new::new;
use std::error::Error;
use std::fmt;
use std::io::stdout;

use crate::all::{
    bshas, isext, isload, isparbh, to_s, BSet, BlkIdx, Blks, Fn, InsIdx, KExt, Phi, PhiIdx, Ref,
    RubeResult, Target, Tmp, TmpIdx, TmpWdth, Typ, Use, UseT, KW, KX, O, TMP0, UNDEF,
};
use crate::cfg::{dom, filldom, fillfron, sdom};
use crate::live::filllive;
use crate::parse::printfn;
use crate::util::{bsclr, bsinit, bsset, clsmerge, newtmpref2, phicls, Bucket};

#[derive(Debug)]
struct SsaError {
    msg: String,
}

impl SsaError {
    fn new(msg: &str) -> SsaError {
        SsaError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for SsaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for SsaError {
    fn description(&self) -> &str {
        &self.msg
    }
}

fn adduse(tmp: &mut Tmp, ty: UseT, bi: BlkIdx, bid: u32) {
    tmp.uses.push(Use::new(ty, bi, bid));
}

/* fill usage, width, phi, and class information
 * must not change .visit fields
 */
pub fn filluse(f: &mut Fn) {
    let blks = &f.blks;
    let phis: &mut [Phi] = &mut f.phis;
    let tmps: &mut [Tmp] = &mut f.tmps;

    /* todo, is this the correct file? */
    for tmp in tmps.iter_mut().skip(TMP0 as usize) {
        // TODO - Tmp::clear()???
        tmp.def = InsIdx::NONE; // QBE initialises with 0
        tmp.bid = u32::MAX;
        tmp.ndef = 0;
        tmp.cls = KW; // QBE sets to 0
        tmp.phi = TmpIdx::NONE; // QBE sets to 0
        tmp.width = TmpWdth::WFull;
        tmp.uses.clear();
    }

    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let bid: u32 = blks.borrow(bi).id;
        let mut pi: PhiIdx = blks.borrow(bi).phi;
        while pi != PhiIdx::NONE {
            let p: &Phi = &phis[pi];
            let cls = p.cls;
            if let Ref::RTmp(mut pti) = p.to {
                {
                    let tmp: &mut Tmp = &mut tmps[pti];
                    tmp.bid = bid;
                    tmp.ndef += 1;
                    tmp.cls = cls;
                }
                pti = phicls(pti, tmps);
                for a in &p.args {
                    if let Ref::RTmp(mut ati) = a {
                        adduse(&mut tmps[ati], UseT::UPhi(pi), bi, bid);
                        ati = phicls(ati, tmps);
                        if ati != pti {
                            tmps[ati].phi = pti;
                        }
                    }
                }
            } else {
                // p.to MUST be an RTmp
                assert!(false);
            }

            pi = p.link;
        }

        for (ii, i) in blks.borrow(bi).ins.iter().enumerate() {
            if let Ref::RTmp(ti) = i.to {
                let mut w: TmpWdth = TmpWdth::WFull;
                if isparbh(i.op) {
                    w = TmpWdth::from_parbh(i.op);
                } else if isload(i.op) && i.op != O::Oload {
                    w = TmpWdth::from_loadbh(i.op);
                } else if isext(i.op) {
                    w = TmpWdth::from_ext(i.op);
                }
                if w == TmpWdth::Wsw || w == TmpWdth::Wuw {
                    if i.cls == KW {
                        w = TmpWdth::WFull;
                    }
                }
                let tmp: &mut Tmp = &mut tmps[ti];
                tmp.width = w;
                tmp.def = InsIdx::new(ii);
                tmp.bid = bid;
                tmp.ndef += 1;
                tmp.cls = i.cls;
            } else {
                // Ins i.to must be R or RTmp
                assert!(i.to == Ref::R);
            }
            for arg in /*blks.borrow(bi).ins[ii]*/ i.args {
                if let Ref::RTmp(ti) = arg {
                    adduse(&mut tmps[ti], UseT::UIns(InsIdx::new(ii)), bi, bid);
                }
            }
        }

        if let Ref::RTmp(ti) = blks.borrow(bi).jmp.arg {
            adduse(&mut tmps[ti], UseT::UJmp, bi, bid);
        }

        bi = blks.borrow(bi).link;
    }
}

fn refindex(tmps: &mut Vec<Tmp>, ti: TmpIdx) -> Ref {
    let prfx: Vec<u8> = tmps[ti].name.clone();
    let cls: KExt = tmps[ti].cls;
    newtmpref2(tmps, &prfx, true, cls)
}

fn phiins(f: &mut Fn) -> RubeResult<()> {
    let blks = &f.blks;
    let phis: &mut Vec<Phi> = &mut f.phis;
    let tmps: &mut Vec<Tmp> = &mut f.tmps;

    let mut blist: Vec<BlkIdx> = vec![BlkIdx::NONE; blks.len()];
    let be: usize = blks.len();
    let nt: usize = tmps.len();
    for tii in TMP0..nt {
        let ti: TmpIdx = TmpIdx::new(tii);
        tmps[ti].visit = TmpIdx::NONE;
        if tmps[ti].phi != TmpIdx::NONE {
            continue;
        }
        if tmps[ti].ndef == 1 {
            let defb: u32 = tmps[ti].bid;
            let ok = tmps[ti].uses.iter().all(|u| u.bid == defb);
            if ok || defb == blks.borrow(f.start).id {
                continue;
            }
        }
        let mut u: BSet = bsinit(blks.len());
        let mut k: KExt = KX;
        let mut bp: usize = be;
        let rt: Ref = Ref::RTmp(ti);
        let mut bi = f.start;
        while bi != BlkIdx::NONE {
            let mut b = blks.borrow_mut(bi);
            let bid = b.id;
            let b_out = &b.out;
            b.visit = 0;
            let mut r: Ref = Ref::R;
            for i in &mut b.ins {
                if r != Ref::R {
                    for arg in &mut i.args {
                        if *arg == rt {
                            *arg = r;
                        }
                    }
                }
                if i.to == rt {
                    if !bshas(/*&b.out*/ b_out, ti.usize()) {
                        r = refindex(tmps, ti);
                        i.to = r;
                    } else {
                        if !bshas(&u, /*b.id*/ bid as usize) {
                            bsset(&mut u, /*b.id*/ bid as usize);
                            bp -= 1;
                            blist[bp] = bi;
                        }
                        if clsmerge(&mut k, i.cls) {
                            // TODO - better msg
                            return Err(Box::new(SsaError::new("invalid input")));
                        }
                    }
                }
            }
            if r != Ref::R && b.jmp.arg == rt {
                b.jmp.arg = r;
            }
            bi = b.link;
        }
        let defs: BSet = u.clone();
        while bp != be {
            tmps[ti].visit = ti;
            let bi: BlkIdx = blist[bp];
            bp += 1;
            bsclr(&mut u, blks.borrow(bi).id as usize);
            for n in 0..blks.borrow(bi).frons.len() {
                let ai: BlkIdx = blks.borrow(bi).frons[n];
                let a_visit = blks.borrow(ai).visit;
                blks.borrow_mut(ai).visit += 1;
                if a_visit == 0 && bshas(&blks.borrow(ai).in_, ti.usize()) {
                    let a_pi: PhiIdx = blks.borrow(ai).phi;
                    let pi: PhiIdx = PhiIdx::new(phis.len());
                    phis.push(Phi::new(rt, vec![], vec![], k, a_pi));
                    blks.borrow_mut(ai).phi = pi;
                    let a_id = blks.borrow(ai).id;
                    if !bshas(&defs, a_id as usize) && !bshas(&u, a_id as usize) {
                        bsset(&mut u, a_id as usize);
                        bp -= 1;
                        blist[bp] = ai;
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(new)]
struct Name {
    r: Ref,
    bi: BlkIdx,
    up: NameIdx,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct NameIdx(u32);

impl NameIdx {
    const INVALID: NameIdx = NameIdx(u32::MAX);
}

/*
static Name *namel;
 */
fn nnew(r: Ref, bi: BlkIdx, namel: &mut NameIdx, names: &mut Vec<Name>, up: NameIdx) -> NameIdx {
    let ni: NameIdx;

    if *namel == NameIdx::INVALID {
        ni = NameIdx(names.len() as u32);
        names.push(Name::new(Ref::R, BlkIdx::NONE, NameIdx::INVALID));
    } else {
        ni = *namel;
        *namel = names[ni.0 as usize].up;
    }
    names[ni.0 as usize] = Name::new(r, bi, up);

    ni
}

fn nfree(ni: NameIdx, namel: &mut NameIdx, names: &mut Vec<Name>) {
    names[ni.0 as usize].up = *namel;
    *namel = ni;
}

fn rendef(
    tmps: &mut Vec<Tmp>,
    bi: BlkIdx,
    r: Ref,
    namel: &mut NameIdx,
    names: &mut Vec<Name>,
    stk: &mut [NameIdx],
) -> Ref {
    if r == Ref::R {
        return r;
    }
    if let Ref::RTmp(ti) = r {
        if tmps[ti].visit == TmpIdx::NONE {
            return r;
        }
        let r1: Ref = refindex(tmps, ti);
        // TODO - there must be a better way of indicating that refindex() returns Ref::RTmp
        if let Ref::RTmp(t1i) = r1 {
            tmps[t1i].visit = ti;
            let ni: NameIdx = nnew(r1, bi, namel, names, stk[ti.0 as usize]);
            stk[ti.0 as usize] = ni;
        } else {
            assert!(false);
        }
        r1
    } else {
        // r must be R or RTmp
        assert!(false);
        r
    }
}

fn getstk(
    blks: &Blks,
    bi: BlkIdx,
    ti: TmpIdx,
    namel: &mut NameIdx,
    names: &mut Vec<Name>,
    stk: &mut [NameIdx],
) -> Ref {
    let mut ni: NameIdx = stk[ti.0 as usize];
    while ni != NameIdx::INVALID && !dom(blks, names[ni.0 as usize].bi, bi) {
        let ni1: NameIdx = ni;
        ni = names[ni.0 as usize].up;
        nfree(ni1, namel, names);
    }
    stk[ti.0 as usize] = ni;
    if ni == NameIdx::INVALID {
        /* uh, oh, warn */
        UNDEF
    } else {
        names[ni.0 as usize].r
    }
}

fn renblk(
    blks: &Blks,
    phis: &mut Vec<Phi>,
    tmps: &mut Vec<Tmp>,
    bi: BlkIdx,
    namel: &mut NameIdx,
    names: &mut Vec<Name>,
    stk: &mut [NameIdx],
) {
    let mut pi = blks.borrow(bi).phi;
    while pi != PhiIdx::NONE {
        let to: Ref = phis[pi].to;
        let to_new = rendef(tmps, bi, to, namel, names, stk);
        phis[pi].to = to_new;

        pi = phis[pi].link;
    }
    for ii in 0..blks.borrow(bi).ins.len() {
        for m in 0..2 {
            if let Ref::RTmp(ti) = blks.borrow(bi).ins[ii].args[m] {
                if tmps[ti].visit != TmpIdx::NONE {
                    blks.borrow_mut(bi).ins[ii].args[m] = getstk(blks, bi, ti, namel, names, stk);
                }
            }
        }
        let to: Ref = blks.borrow(bi).ins[ii].to;
        let new_to: Ref = rendef(tmps, bi, to, namel, names, stk);
        blks.borrow_mut(bi).ins[ii].to = new_to;
    }
    let jmp_arg: Ref = blks.borrow(bi).jmp.arg;
    if let Ref::RTmp(ti) = jmp_arg {
        if tmps[ti].visit != TmpIdx::NONE {
            blks.borrow_mut(bi).jmp.arg = getstk(blks, bi, ti, namel, names, stk);
        }
    }
    let (s1, s2) = blks.borrow(bi).s1_s2();
    let succ: [BlkIdx; 2] = [s1, if s1 == s2 { BlkIdx::NONE } else { s2 }];
    for si in succ {
        if si == BlkIdx::NONE {
            continue; // QBE effectively break's
        }
        let mut pi: PhiIdx = blks.borrow(si).phi;
        while pi != PhiIdx::NONE {
            if let Ref::RTmp(to_ti) = phis[pi].to {
                let ti: TmpIdx = tmps[to_ti].visit;
                if ti != TmpIdx::NONE {
                    let arg: Ref = getstk(blks, bi, ti, namel, names, stk);
                    phis[pi].args.push(arg);
                    phis[pi].blks.push(bi);
                }
            } else {
                // phi to MUST be an RTmp (TODO is there a better way?)
                assert!(false);
            }
            pi = phis[pi].link;
        }
    }
    let mut si: BlkIdx = blks.borrow(bi).dom;
    while si != BlkIdx::NONE {
        renblk(blks, phis, tmps, si, namel, names, stk);
        si = blks.borrow(si).dlink;
    }
}

/* require rpo and use */
pub fn ssa(f: &mut Fn, targ: &Target, typ: &[Typ], itbl: &[Bucket]) -> RubeResult<()> {
    // TODO
    // d = debug['L'];
    // debug['L'] = 0;
    filldom(f);
    if true
    /*debug['N']*/
    {
        // TODO obviously
        eprintln!("\n> Dominators:");
        let mut b1i: BlkIdx = f.start;
        while b1i != BlkIdx::NONE {
            let b1 = f.blk(b1i);
            if b1.dom != BlkIdx::NONE {
                /*e*/
                print!("{:>10}:", to_s(&b1.name));
                let mut bi: BlkIdx = b1.dom;
                while bi != BlkIdx::NONE {
                    let b = f.blk(bi);
                    /*e*/
                    print!(" {}", to_s(&b.name));
                    bi = b.dlink;
                }
                /*e*/
                println!();
            }

            b1i = f.blk(b1i).link;
        }
    }
    fillfron(f);
    filllive(f, targ);
    phiins(f)?;
    let mut namel: NameIdx = NameIdx::INVALID;
    let mut names: Vec<Name> = vec![];
    let mut stk: Vec<NameIdx> = vec![NameIdx::INVALID; f.tmps.len()];
    let blks: &Blks = &f.blks;
    let phis: &mut Vec<Phi> = &mut f.phis;
    let tmps: &mut Vec<Tmp> = &mut f.tmps;
    renblk(blks, phis, tmps, f.start, &mut namel, &mut names, &mut stk);
    // TODO
    //debug['L'] = d;
    if false
    /*TODO: debug['N']*/
    {
        /*e*/
        println!("\n> After SSA construction:");
        printfn(/*stderr*/ &mut stdout(), f, typ, itbl);
    }

    Ok(())
}

fn phicheck(blks: &Blks, p: &Phi, bi: BlkIdx, t: Ref) -> bool {
    for n in 0..p.args.len() {
        if p.args[n] == t {
            let bi1 = p.blks[n];
            if bi1 != bi && !sdom(blks, bi, bi1) {
                return true;
            }
        }
    }
    false
}

/* require use and ssa */
pub fn ssacheck(f: &Fn) -> RubeResult<()> {
    let blks = &f.blks;
    let rpo: &[BlkIdx] = &f.rpo;
    let phis: &[Phi] = &f.phis;
    let tmps: &[Tmp] = &f.tmps;

    for t in tmps.iter().skip(TMP0 as usize) {
        if t.ndef > 1 {
            return Err(Box::new(SsaError::new(&format!(
                "ssa temporary %{} defined more than once",
                to_s(&t.name)
            ))));
        }
        if !t.uses.is_empty() && t.ndef == 0 {
            let bui: BlkIdx = rpo[t.uses[0].bid as usize];
            return Err(ssacheck_err(f, t, bui));
        }
    }
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let b = blks.borrow(bi);
        let mut pi: PhiIdx = b.phi;
        while pi != PhiIdx::NONE {
            let p: &Phi = &phis[pi];
            let r: Ref = p.to;
            let ti: TmpIdx = if let Ref::RTmp(ti0) = r {
                ti0
            } else {
                return Err(Box::new(SsaError::new(&format!(
                    "phi does not define a temporary in @{}",
                    to_s(&b.name)
                ))));
            };
            let t: &Tmp = &tmps[ti];
            for u in &t.uses {
                let bui: BlkIdx = rpo[u.bid as usize];

                if let UseT::UPhi(upi) = u.type_ {
                    if phicheck(blks, &phis[upi], bi, r) {
                        return Err(ssacheck_err(f, t, bui));
                    }
                } else {
                    if bui != bi && !sdom(blks, bi, bui) {
                        return Err(ssacheck_err(f, t, bui));
                    }
                }
            }
            for (ii, i) in b.ins.iter().enumerate() {
                if let Ref::RTmp(ti) = i.to {
                    let t: &Tmp = &tmps[ti];
                    for u in &t.uses {
                        let bui: BlkIdx = rpo[u.bid as usize];
                        match u.type_ {
                            UseT::UPhi(upi) => {
                                if phicheck(blks, &phis[upi], bi, r) {
                                    return Err(ssacheck_err(f, t, bui));
                                }
                            }
                            UseT::UIns(uii) => {
                                if bui == bi && uii.0 <= (ii as u32) {
                                    return Err(ssacheck_err(f, t, bui));
                                }
                            }
                            _ => {
                                if bui != bi && !sdom(blks, bi, bui) {
                                    return Err(ssacheck_err(f, t, bui));
                                }
                            }
                        }
                    }
                }
            }
            pi = p.link;
        }
        bi = b.link;
    }
    Ok(())
}

fn ssacheck_err(f: &Fn, t: &Tmp, bui: BlkIdx) -> Box<SsaError> {
    Box::new(SsaError::new(&{
        if t.visit != TmpIdx::NONE {
            format!("%{} violates ssa invariant", to_s(&t.name))
        } else {
            format!(
                "ssa temporary %{} is used undefined in @{}",
                to_s(&t.name),
                to_s(&f.blk(bui).name)
            )
        }
    }))
}
