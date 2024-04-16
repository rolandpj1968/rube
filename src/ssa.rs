use derive_new::new;
use std::error::Error;
use std::fmt;
use std::io::stdout;

use crate::all::{
    bshas, isext, isload, isparbh, to_s, BSet, Blk, BlkIdx, Fn, Ins, InsIdx, KExt, Phi, PhiIdx,
    Ref, RubeResult, Target, Tmp, TmpIdx, TmpWdth, Typ, Use, UseT, KW, KX, O, TMP0, UNDEF,
};
use crate::cfg::{dom, filldom, fillfron};
use crate::live::filllive;
use crate::parse::printfn;
use crate::util::{bsclr, bsinit, bsset, clsmerge, newtmpref, phicls, Bucket};

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
    /* todo, is this the correct file? */
    for tmp in f.tmps.iter_mut().skip(TMP0 as usize) {
        // TODO - Tmp::clear()???
        tmp.def = InsIdx::INVALID; // QBE initialises with 0
        tmp.bid = u32::MAX;
        tmp.ndef = 0;
        tmp.cls = KW; // QBE sets to 0
        tmp.phi = TmpIdx::INVALID; // QBE sets to 0
        tmp.width = TmpWdth::WFull;
        tmp.uses.clear();
    }

    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::INVALID {
        let (bid, mut pi) = {
            let b: &Blk = f.blk(bi);
            (b.id, b.phi)
        };
        while pi != PhiIdx::INVALID {
            let cls = f.phi(pi).cls;
            if let Ref::RTmp(mut tip) = f.phi(pi).to {
                {
                    let tmp: &mut Tmp = f.tmp_mut(tip);
                    tmp.bid = bid;
                    tmp.ndef += 1;
                    tmp.cls = cls;
                }
                tip = phicls(tip, &mut f.tmps);
                for a in 0..f.phi(pi).args.len() {
                    if let Ref::RTmp(mut ti) = f.phi(pi).args[a] {
                        adduse(f.tmp_mut(ti), UseT::UPhi(pi), bi, bid);
                        ti = phicls(ti, &mut f.tmps);
                        if ti != tip {
                            f.tmp_mut(ti).phi = tip;
                        }
                    }
                }
            } else {
                // p.to MUST be an RTmp
                assert!(false);
            }

            pi = f.phi(pi).link;
        }

        for ii in 0..f.blk(bi).ins.len() {
            let (to, op, cls) = {
                let i: &Ins = &f.blk(bi).ins[ii];
                (i.to, i.op, i.cls)
            };
            if to != Ref::R {
                if let Ref::RTmp(ti) = to {
                    let mut w: TmpWdth = TmpWdth::WFull;
                    if isparbh(op) {
                        w = TmpWdth::from_parbh(op);
                    } else if isload(op) && op != O::Oload {
                        w = TmpWdth::from_loadbh(op);
                    } else if isext(op) {
                        w = TmpWdth::from_ext(op);
                    }
                    if w == TmpWdth::Wsw || w == TmpWdth::Wuw {
                        if cls == KW {
                            w = TmpWdth::WFull;
                        }
                    }
                    let tmp: &mut Tmp = f.tmp_mut(ti);
                    tmp.width = w;
                    tmp.def = InsIdx(ii as u32);
                    tmp.bid = bid;
                    tmp.ndef += 1;
                    tmp.cls = cls;
                } else {
                    // Ins to must be R or RTmp
                    assert!(false);
                }
            }
            for arg in f.blk(bi).ins[ii].args {
                if let Ref::RTmp(ti) = arg {
                    adduse(f.tmp_mut(ti), UseT::UIns(InsIdx(ii as u32)), bi, bid);
                }
            }
        }

        if let Ref::RTmp(ti) = f.blk(bi).jmp.arg {
            adduse(f.tmp_mut(ti), UseT::UJmp, bi, bid);
        }

        bi = f.blk(bi).link;
    }
}

fn refindex(f: &mut Fn, ti: TmpIdx) -> Ref {
    let prfx: Vec<u8> = f.tmp(ti).name.clone();
    let cls: KExt = f.tmp(ti).cls;
    newtmpref(&prfx, true, cls, f)
}

fn phiins(f: &mut Fn) -> RubeResult<()> {
    // BSet u[1], defs[1];
    // Blk *a, *b, **blist, **be, **bp;
    // Ins *i;
    // Phi *p;
    // Use *use;
    // Ref r;
    // int t, nt, ok;
    // uint n, defb;
    // short k;

    let mut blist: Vec<BlkIdx> = vec![BlkIdx::INVALID; f.blks.len()];
    let be: usize = f.blks.len();
    let nt: u32 = f.tmps.len() as u32;
    for tii in TMP0..nt {
        let ti: TmpIdx = TmpIdx(tii);
        f.tmp_mut(ti).visit = TmpIdx::INVALID;
        if f.tmp(ti).phi != TmpIdx::INVALID {
            continue;
        }
        if f.tmp(ti).ndef == 1 {
            let mut ok: bool = true;
            let defb: u32 = f.tmp(ti).bid;
            //use = f.tmp(ti).use;
            for usei in (0..f.tmp(ti).uses.len()).rev() {
                ok = ok && f.tmp(ti).uses[usei].bid == defb;
            }
            if ok || defb == f.blk(f.start).id {
                continue;
            }
        }
        let mut u: BSet = bsinit(f.blks.len());
        let mut k: KExt = KX;
        let mut bp: usize = be;
        let rt: Ref = Ref::RTmp(ti);
        let mut bi = f.start;
        while bi != BlkIdx::INVALID {
            f.blk_mut(bi).visit = 0;
            let mut r: Ref = Ref::R;
            for ii in 0..f.blk(bi).ins.len() {
                let (to, cls, arg0, arg1) = {
                    let i: &Ins = &f.blk(bi).ins[ii];
                    (i.to, i.cls, i.args[0], i.args[1])
                };
                if r != Ref::R {
                    if arg0 == rt {
                        f.blk_mut(bi).ins[ii].args[0] = r;
                    }
                    if arg1 == rt {
                        f.blk_mut(bi).ins[ii].args[1] = r;
                    }
                }
                if to == rt {
                    if !bshas(&f.blk(bi).out, tii) {
                        r = refindex(f, ti);
                        f.blk_mut(bi).ins[ii].to = r;
                    } else {
                        if !bshas(&u, f.blk(bi).id) {
                            bsset(&mut u, f.blk(bi).id);
                            bp -= 1;
                            blist[bp] = bi;
                        }
                        if clsmerge(&mut k, cls) {
                            // TODO - better msg
                            return Err(Box::new(SsaError::new("invalid input")));
                        }
                    }
                }
            }
            if r != Ref::R && f.blk(bi).jmp.arg == rt {
                f.blk_mut(bi).jmp.arg = r;
            }
            bi = f.blk(bi).link;
        }
        let defs: BSet = u.clone();
        while bp != be {
            f.tmp_mut(ti).visit = ti;
            let bi: BlkIdx = blist[bp];
            bp += 1;
            bsclr(&mut u, f.blk(bi).id);
            for n in 0..f.blk(bi).frons.len() {
                let ai: BlkIdx = f.blk(bi).frons[n];
                let a_visit = f.blk(ai).visit;
                f.blk_mut(ai).visit += 1;
                if a_visit == 0 && bshas(&f.blk(ai).in_, ti.0) {
                    let a_pi: PhiIdx = f.blk(ai).phi;
                    let pi: PhiIdx = f.add_phi(Phi::new(rt, vec![], vec![], k, a_pi));
                    f.blk_mut(ai).phi = pi;
                    let a_id = f.blk(ai).id;
                    if !bshas(&defs, a_id) && !bshas(&u, a_id) {
                        bsset(&mut u, a_id);
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
        names.push(Name::new(Ref::R, BlkIdx::INVALID, NameIdx::INVALID));
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
    f: &mut Fn,
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
        if f.tmp(ti).visit == TmpIdx::INVALID {
            return r;
        }
        let r1: Ref = refindex(f, ti);
        // TODO - there must be a better way of indicating that refindex() returns Ref::RTmp
        if let Ref::RTmp(t1i) = r1 {
            f.tmp_mut(t1i).visit = ti;
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
    f: &Fn,
    bi: BlkIdx,
    ti: TmpIdx,
    namel: &mut NameIdx,
    names: &mut Vec<Name>,
    stk: &mut [NameIdx],
) -> Ref {
    let mut ni: NameIdx = stk[ti.0 as usize];
    while ni != NameIdx::INVALID && !dom(f, names[ni.0 as usize].bi, bi) {
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

fn renblk(f: &mut Fn, bi: BlkIdx, namel: &mut NameIdx, names: &mut Vec<Name>, stk: &mut [NameIdx]) {
    let mut pi = f.blk(bi).phi;
    while pi != PhiIdx::INVALID {
        let to: Ref = f.phi(pi).to;
        let to_new = rendef(f, bi, to, namel, names, stk);
        f.phi_mut(pi).to = to_new;

        pi = f.phi(pi).link;
    }
    for ii in 0..f.blk(bi).ins.len() {
        for m in 0..2 {
            if let Ref::RTmp(ti) = f.blk(bi).ins[ii].args[m] {
                if f.tmp(ti).visit != TmpIdx::INVALID {
                    f.blk_mut(bi).ins[ii].args[m] = getstk(f, bi, ti, namel, names, stk);
                }
            }
        }
        let to: Ref = f.blk(bi).ins[ii].to;
        let new_to: Ref = rendef(f, bi, to, namel, names, stk);
        f.blk_mut(bi).ins[ii].to = new_to;
    }
    let jmp_arg: Ref = f.blk(bi).jmp.arg;
    if let Ref::RTmp(ti) = jmp_arg {
        if f.tmp(ti).visit != TmpIdx::INVALID {
            f.blk_mut(bi).jmp.arg = getstk(f, bi, ti, namel, names, stk);
        }
    }
    let (s1, s2) = f.blk(bi).s1_s2();
    let succ: [BlkIdx; 2] = [s1, if s1 == s2 { BlkIdx::INVALID } else { s2 }];
    for si in succ {
        if si == BlkIdx::INVALID {
            continue; // QBE effectively break's
        }
        let mut pi: PhiIdx = f.blk(si).phi;
        while pi != PhiIdx::INVALID {
            if let Ref::RTmp(to_ti) = f.phi(pi).to {
                let ti: TmpIdx = f.tmp(to_ti).visit;
                if ti != TmpIdx::INVALID {
                    let arg: Ref = getstk(f, bi, ti, namel, names, stk);
                    let p: &mut Phi = f.phi_mut(pi);
                    p.args.push(arg);
                    p.blks.push(bi);
                }
            } else {
                // phi to MUST be an RTmp (TODO is there a better way?)
                assert!(false);
            }
            pi = f.phi(pi).link;
        }
    }
    let mut si: BlkIdx = f.blk(bi).dom;
    while si != BlkIdx::INVALID {
        renblk(f, si, namel, names, stk);
        si = f.blk(si).dlink;
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
        while b1i != BlkIdx::INVALID {
            let b1: &Blk = f.blk(b1i);
            if b1.dom != BlkIdx::INVALID {
                /*e*/
                print!("{:>10}:", to_s(&b1.name));
                let mut bi: BlkIdx = b1.dom;
                while bi != BlkIdx::INVALID {
                    let b: &Blk = f.blk(bi);
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
    if true {
        /*e*/
        println!("\n> After Phi insertion:");
        printfn(/*stderr*/ &mut stdout(), f, typ, itbl);
    }
    let mut namel: NameIdx = NameIdx::INVALID;
    let mut names: Vec<Name> = vec![];
    let mut stk: Vec<NameIdx> = vec![NameIdx::INVALID; f.tmps.len()];
    renblk(f, f.start, &mut namel, &mut names, &mut stk);
    // TODO
    //debug['L'] = d;
    if true
    /*TODO: debug['N']*/
    {
        /*e*/
        println!("\n> After SSA construction:");
        printfn(/*stderr*/ &mut stdout(), f, typ, itbl);
    }

    Ok(())
}

/*
static int
phicheck(Phi *p, Blk *b, Ref t)
{
    Blk *b1;
    uint n;

    for (n=0; n<p->narg; n++)
        if (req(p->arg[n], t)) {
            b1 = p->blk[n];
            if (b1 != b && !sdom(b, b1))
                return 1;
        }
    return 0;
}

/* require use and ssa */
void
ssacheck(Fn *fn)
{
    Tmp *t;
    Ins *i;
    Phi *p;
    Use *u;
    Blk *b, *bu;
    Ref r;

    for (t=&fn->tmp[Tmp0]; t-fn->tmp < fn->ntmp; t++) {
        if (t->ndef > 1)
            err("ssa temporary %%%s defined more than once",
                t->name);
        if (t->nuse > 0 && t->ndef == 0) {
            bu = fn->rpo[t->use[0].bid];
            goto Err;
        }
    }
    for (b=fn->start; b; b=b->link) {
        for (p=b->phi; p; p=p->link) {
            r = p->to;
            t = &fn->tmp[r.val];
            for (u=t->use; u<&t->use[t->nuse]; u++) {
                bu = fn->rpo[u->bid];
                if (u->type == UPhi) {
                    if (phicheck(u->u.phi, b, r))
                        goto Err;
                } else
                    if (bu != b && !sdom(b, bu))
                        goto Err;
            }
        }
        for (i=b->ins; i<&b->ins[b->nins]; i++) {
            if (rtype(i->to) != RTmp)
                continue;
            r = i->to;
            t = &fn->tmp[r.val];
            for (u=t->use; u<&t->use[t->nuse]; u++) {
                bu = fn->rpo[u->bid];
                if (u->type == UPhi) {
                    if (phicheck(u->u.phi, b, r))
                        goto Err;
                } else {
                    if (bu == b) {
                        if (u->type == UIns)
                            if (u->u.ins <= i)
                                goto Err;
                    } else
                        if (!sdom(b, bu))
                            goto Err;
                }
            }
        }
    }
    return;
Err:
    if (t->visit)
        die("%%%s violates ssa invariant", t->name);
    else
        err("ssa temporary %%%s is used undefined in @%s",
            t->name, bu->name);
}
 */
