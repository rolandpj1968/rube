use std::error::Error;
use std::fmt;

use crate::all::{
    bshas, isext, isload, isparbh, to_s, BSet, Blk, BlkIdx, Fn, Ins, InsIdx, KExt, Phi, PhiIdx,
    Ref, RubeResult, Target, Tmp, TmpIdx, TmpWdth, Use, UseT, KW, KX, O, TMP0,
};
use crate::cfg::{filldom, fillfron};
use crate::live::filllive;
use crate::util::{bsclr, bsinit, bsset, clsmerge, newtmpref, phicls};

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

struct Name {
    r: Ref,
    bi: BlkIdx,
    up: NameIdx,
}

struct NameIdx(u32);

/*
static Name *namel;

static Name *
nnew(Ref r, Blk *b, Name *up)
{
    Name *n;

    if (namel) {
        n = namel;
        namel = n->up;
    } else
        /* could use alloc, here
         * but namel should be reset
         */
        n = emalloc(sizeof *n);
    n->r = r;
    n->b = b;
    n->up = up;
    return n;
}

static void
nfree(Name *n)
{
    n->up = namel;
    namel = n;
}

static void
rendef(Ref *r, Blk *b, Name **stk, Fn *fn)
{
    Ref r1;
    int t;

    t = r->val;
    if (req(*r, R) || !fn->tmp[t].visit)
        return;
    r1 = refindex(t, fn);
    fn->tmp[r1.val].visit = t;
    stk[t] = nnew(r1, b, stk[t]);
    *r = r1;
}

static Ref
getstk(int t, Blk *b, Name **stk)
{
    Name *n, *n1;

    n = stk[t];
    while (n && !dom(n->b, b)) {
        n1 = n;
        n = n->up;
        nfree(n1);
    }
    stk[t] = n;
    if (!n) {
        /* uh, oh, warn */
        return UNDEF;
    } else
        return n->r;
}

static void
renblk(Blk *b, Name **stk, Fn *fn)
{
    Phi *p;
    Ins *i;
    Blk *s, **ps, *succ[3];
    int t, m;

    for (p=b->phi; p; p=p->link)
        rendef(&p->to, b, stk, fn);
    for (i=b->ins; i<&b->ins[b->nins]; i++) {
        for (m=0; m<2; m++) {
            t = i->arg[m].val;
            if (rtype(i->arg[m]) == RTmp)
            if (fn->tmp[t].visit)
                i->arg[m] = getstk(t, b, stk);
        }
        rendef(&i->to, b, stk, fn);
    }
    t = b->jmp.arg.val;
    if (rtype(b->jmp.arg) == RTmp)
    if (fn->tmp[t].visit)
        b->jmp.arg = getstk(t, b, stk);
    succ[0] = b->s1;
    succ[1] = b->s2 == b->s1 ? 0 : b->s2;
    succ[2] = 0;
    for (ps=succ; (s=*ps); ps++)
        for (p=s->phi; p; p=p->link) {
            t = p->to.val;
            if ((t=fn->tmp[t].visit)) {
                m = p->narg++;
                vgrow(&p->arg, p->narg);
                vgrow(&p->blk, p->narg);
                p->arg[m] = getstk(t, b, stk);
                p->blk[m] = b;
            }
        }
    for (s=b->dom; s; s=s->dlink)
        renblk(s, stk, fn);
}
 */

/* require rpo and use */
pub fn ssa(f: &mut Fn, targ: &Target) -> RubeResult<()> {
    // Name **stk, *n;
    // int d, nt;
    // Blk *b, *b1;

    // nt = fn->ntmp;
    // stk = emalloc(nt * sizeof stk[0]);
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
    // renblk(fn->start, stk, fn);
    // while (nt--)
    //     while ((n=stk[nt])) {
    //         stk[nt] = n->up;
    //         nfree(n);
    //     }
    // debug['L'] = d;
    // free(stk);
    // if (debug['N']) {
    //     fprintf(stderr, "\n> After SSA construction:\n");
    //     printfn(fn, stderr);
    // }

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
