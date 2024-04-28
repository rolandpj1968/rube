use derive_new::new;
use std::error::Error;
use std::fmt;
use std::io::stdout;

use crate::all::Ref::{RTmp, R};
use crate::all::K::{Kw, Kx};
use crate::all::{
    bshas, isext, isload, isparbh, to_s, BSet, BlkIdx, Blks, Fn, InsIdx, Phi, PhiIdx, Ref, RpoIdx,
    RubeResult, Target, Tmp, TmpIdx, TmpWdth, Typ, Use, UseT, K, O, TMP0, UNDEF,
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

fn adduse(tmp: &mut Tmp, ty: UseT, bi: BlkIdx, bid: RpoIdx) {
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
        tmp.def = InsIdx::NONE;
        tmp.bid = RpoIdx::NONE;
        tmp.ndef = 0;
        tmp.cls = Kw;
        tmp.phi = TmpIdx::NONE;
        tmp.width = TmpWdth::WFull;
        tmp.uses.clear();
    }

    blks.for_each_bi(|bi| {
        blks.with(bi, |b| {
            let mut pi: PhiIdx = b.phi;
            while pi != PhiIdx::NONE {
                let p: &Phi = &phis[pi];
                let cls = p.cls;
                assert!(matches!(p.to, RTmp(_)));
                if let RTmp(mut pti) = p.to {
                    let tmp: &mut Tmp = &mut tmps[pti];
                    tmp.bid = b.id;
                    tmp.ndef += 1;
                    tmp.cls = cls;

                    pti = phicls(pti, tmps);
                    for a in &p.args {
                        if let RTmp(mut ati) = a {
                            adduse(&mut tmps[ati], UseT::UPhi(pi), bi, b.id);
                            ati = phicls(ati, tmps);
                            if ati != pti {
                                tmps[ati].phi = pti;
                            }
                        }
                    }
                }
                pi = p.link;
            }

            for (ii, i) in b.ins().iter().enumerate() {
                assert!(i.to == R || matches!(i.to, RTmp(_)));
                if let RTmp(ti) = i.to {
                    let mut w: TmpWdth = TmpWdth::WFull;
                    if isparbh(i.op) {
                        w = TmpWdth::from_parbh(i.op);
                    } else if isload(i.op) && i.op != O::Oload {
                        w = TmpWdth::from_loadbh(i.op);
                    } else if isext(i.op) {
                        w = TmpWdth::from_ext(i.op);
                    }
                    if w == TmpWdth::Wsw || w == TmpWdth::Wuw {
                        if i.cls == Kw {
                            w = TmpWdth::WFull;
                        }
                    }
                    let tmp: &mut Tmp = &mut tmps[ti];
                    tmp.width = w;
                    tmp.def = InsIdx::new(ii);
                    tmp.bid = b.id;
                    tmp.ndef += 1;
                    tmp.cls = i.cls;
                }
                for arg in i.args {
                    if let RTmp(ti) = arg {
                        adduse(&mut tmps[ti], UseT::UIns(InsIdx::new(ii)), bi, b.id);
                    }
                }
            }

            if let RTmp(ti) = b.jmp().arg {
                adduse(&mut tmps[ti], UseT::UJmp, bi, b.id);
            }
        });
    });
}

fn refindex(tmps: &mut Vec<Tmp>, ti: TmpIdx) -> Ref {
    let prfx: Vec<u8> = tmps[ti].name.clone();
    let cls: K = tmps[ti].cls;
    newtmpref2(tmps, &prfx, true, cls)
}

fn phiins(f: &mut Fn) -> RubeResult<()> {
    let blks = &f.blks;
    let phis: &mut Vec<Phi> = &mut f.phis;
    let tmps: &mut Vec<Tmp> = &mut f.tmps;

    let mut blist: Vec<BlkIdx> = vec![BlkIdx::NONE; blks.len()];
    let be: usize = blks.len();
    let nt: usize = tmps.len();
    let start_id: RpoIdx = blks.id_of(f.start);
    for tii in TMP0..nt {
        let ti: TmpIdx = TmpIdx::new(tii);
        {
            let t: &mut Tmp = &mut tmps[ti];
            t.tvisit = TmpIdx::NONE;
            if t.phi != TmpIdx::NONE {
                continue;
            }
            if t.ndef == 1 {
                let defb: RpoIdx = t.bid;
                let ok = t.uses.iter().all(|u| u.bid == defb);
                if ok || defb == start_id {
                    continue;
                }
            }
        }
        let mut u: BSet = bsinit(blks.len());
        let mut k: K = Kx;
        let mut bp: usize = be;
        let rt: Ref = RTmp(ti);
        let mut bi = f.start;
        while bi != BlkIdx::NONE {
            blks.with_mut(bi, |b| {
                b.ivisit = 0;
                let mut r: Ref = R;
                for i in b.ins_mut().iter_mut() {
                    if r != R {
                        for arg in &mut i.args {
                            if *arg == rt {
                                *arg = r;
                            }
                        }
                    }
                    if i.to == rt {
                        if !bshas(&b.out, ti.usize()) {
                            r = refindex(tmps, ti);
                            i.to = r;
                        } else {
                            if !bshas(&u, b.id.usize()) {
                                bsset(&mut u, b.id.usize());
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
                if r != R && b.jmp().arg == rt {
                    b.jmp_mut().arg = r;
                }
                bi = b.link;

                Ok(())
            })?;
        }
        let defs: BSet = u.clone();
        while bp != be {
            tmps[ti].tvisit = ti;
            let bi: BlkIdx = blist[bp];
            bp += 1;
            bsclr(&mut u, blks.borrow(bi).id.usize());
            let frons_len = blks.borrow(bi).frons.len();
            for n in 0..frons_len {
                let ai: BlkIdx = blks.borrow(bi).frons[n];
                blks.with_mut(ai, |a| {
                    a.ivisit += 1;
                    if a.ivisit == 1 && bshas(&a.in_, ti.usize()) {
                        let pi: PhiIdx = PhiIdx::new(phis.len());
                        phis.push(Phi::new(rt, vec![], vec![], k, a.phi));
                        a.phi = pi;
                        if !bshas(&defs, a.id.usize()) && !bshas(&u, a.id.usize()) {
                            bsset(&mut u, a.id.usize());
                            bp -= 1;
                            blist[bp] = ai;
                        }
                    }
                });
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
        names.push(Name::new(R, BlkIdx::NONE, NameIdx::INVALID));
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
    assert!(r == R || matches!(r, RTmp(_)));
    match r {
        RTmp(ti) => {
            if tmps[ti].tvisit == TmpIdx::NONE {
                return r;
            }
            let r1: Ref = refindex(tmps, ti);
            if let RTmp(t1i) = r1 {
                tmps[t1i].tvisit = ti;
                let ni: NameIdx = nnew(r1, bi, namel, names, stk[ti.0 as usize]);
                stk[ti.0 as usize] = ni;
            }
            r1
        }
        _ => r,
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
    blks.with(bi, |b| {
        let mut pi = b.phi;
        while pi != PhiIdx::NONE {
            let to: Ref = phis[pi].to;
            let to_new = rendef(tmps, bi, to, namel, names, stk);
            phis[pi].to = to_new;

            pi = phis[pi].link;
        }
        let ins_len = b.ins().len();
        for ii in 0..ins_len {
            for m in 0..2 {
                let arg = b.ins()[ii].args[m];
                if let RTmp(ti) = arg {
                    if tmps[ti].tvisit != TmpIdx::NONE {
                        let new_arg = getstk(blks, bi, ti, namel, names, stk);
                        b.ins_mut()[ii].args[m] = new_arg;
                    }
                }
            }
            let to: Ref = b.ins()[ii].to;
            b.ins_mut()[ii].to = rendef(tmps, bi, to, namel, names, stk);
        }
        let jmp_arg: Ref = b.jmp().arg;
        if let RTmp(ti) = jmp_arg {
            if tmps[ti].tvisit != TmpIdx::NONE {
                b.jmp_mut().arg = getstk(blks, bi, ti, namel, names, stk);
            }
        }
    });

    let succs = blks.succs_of(bi);
    for si in succs {
        if si == BlkIdx::NONE {
            continue;
        }
        let mut pi: PhiIdx = blks.phi_of(si);
        while pi != PhiIdx::NONE {
            let p: &mut Phi = &mut phis[pi];
            assert!(matches!(p.to, RTmp(_)));
            if let RTmp(to_ti) = p.to {
                let ti: TmpIdx = tmps[to_ti].tvisit;
                if ti != TmpIdx::NONE {
                    let arg: Ref = getstk(blks, bi, ti, namel, names, stk);
                    p.args.push(arg);
                    p.blks.push(bi);
                }
            }
            pi = p.link;
        }
    }

    let mut si: BlkIdx = blks.dom_of(bi);
    while si != BlkIdx::NONE {
        renblk(blks, phis, tmps, si, namel, names, stk);
        si = blks.dlink_of(si);
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
        /*e*/
        println!("\n> Dominators:");
        f.blks.for_each(|b1| {
            if b1.dom != BlkIdx::NONE {
                /*e*/
                print!("{:>10}:", to_s(&b1.name));
                let mut bi: BlkIdx = b1.dom;
                while bi != BlkIdx::NONE {
                    let b = f.blks.borrow(bi);
                    /*e*/
                    print!(" {}", to_s(&b.name));
                    bi = b.dlink;
                }
                /*e*/
                println!();
            }
        });
    }
    fillfron(f);
    filllive(f, targ);
    phiins(f)?;
    let mut namel: NameIdx = NameIdx::INVALID;
    let mut names: Vec<Name> = vec![];
    let mut stk: Vec<NameIdx> = vec![NameIdx::INVALID; f.tmps.len()];
    assert!(f.start == BlkIdx::START);
    renblk(
        &f.blks,
        &mut f.phis,
        &mut f.tmps,
        BlkIdx::START,
        &mut namel,
        &mut names,
        &mut stk,
    );
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
            let bui: BlkIdx = rpo[t.uses[0].bid];
            return Err(ssacheck_err(f, t, bui));
        }
    }
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        blks.with(bi, |b| {
            let mut pi: PhiIdx = b.phi;
            while pi != PhiIdx::NONE {
                let p: &Phi = &phis[pi];
                let r: Ref = p.to;
                let ti: TmpIdx = if let RTmp(ti0) = r {
                    ti0
                } else {
                    return Err(Box::new(SsaError::new(&format!(
                        "phi does not define a temporary in @{}",
                        to_s(&b.name)
                    ))));
                };
                let t: &Tmp = &tmps[ti];
                for u in &t.uses {
                    let bui: BlkIdx = rpo[u.bid];

                    if let UseT::UPhi(upi) = u.typ {
                        if phicheck(blks, &phis[upi], bi, r) {
                            return Err(ssacheck_err(f, t, bui));
                        }
                    } else {
                        if bui != bi && !sdom(blks, bi, bui) {
                            return Err(ssacheck_err(f, t, bui));
                        }
                    }
                }
                for (ii, i) in b.ins().iter().enumerate() {
                    if let RTmp(ti) = i.to {
                        let t: &Tmp = &tmps[ti];
                        for u in &t.uses {
                            let bui: BlkIdx = rpo[u.bid];
                            match u.typ {
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

            Ok(())
        })?;
    }
    Ok(())
}

fn ssacheck_err(f: &Fn, t: &Tmp, bui: BlkIdx) -> Box<SsaError> {
    Box::new(SsaError::new(&{
        if t.tvisit != TmpIdx::NONE {
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
