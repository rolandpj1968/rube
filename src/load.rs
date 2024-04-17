use crate::alias::escapes;
use crate::all::{
    bit, isload, isstore, kwide, Alias, AliasIdx, AliasT, Bits, BlkIdx, Fn, Ins, InsIdx, KExt,
    PhiIdx, Ref, KL, KW, O,
};
use crate::cfg::dom;

/*
#include "all.h"
 */

fn mask(w: u32) -> Bits {
    bit(8 * w - 1) * 2 - 1 /* must work when w==8 */
}

enum LocT {
    LRoot,   /* right above the original load */
    LLoad,   /* inserting a load is allowed */
    LNoLoad, /* only scalar operations allowed */
}

struct Loc {
    type_: LocT,
    off: InsIdx,
    bi: BlkIdx,
}

struct Slice {
    r: Ref,
    off: i32,
    sz: i16,
    cls: KExt, /* load class */
}

struct UPhi {
    m: Slice,
    pi: PhiIdx,
}

enum InsertU {
    Ins(Ins),
    Phi(UPhi),
}

struct Insert {
    // isphi: bool, covered by InsertU
    num: u32,
    bid: u32,
    off: u32,
    new: InsertU,
}

/*
static Fn *curf;
static uint inum;    /* current insertion number */
static Insert *ilog; /* global insertion log */
static uint nlog;    /* number of entries in the log */
 */

pub fn loadsz(l: &Ins) -> i32 {
    assert!(isload(l.op));
    match l.op {
        O::Oloadsb | O::Oloadub => 1,
        O::Oloadsh | O::Oloaduh => 2,
        O::Oloadsw | O::Oloaduw => 4,
        O::Oload => {
            if kwide(l.cls) == 0 {
                4
            } else {
                8
            }
        }
        _ => {
            assert!(false);
            -1
        }
    }
}

pub fn storesz(s: &Ins) -> i32 {
    assert!(isstore(s.op));
    match s.op {
        O::Ostoreb => 1,
        O::Ostoreh => 2,
        O::Ostorew | O::Ostores => 4,
        O::Ostorel | O::Ostored => 8,
        _ => {
            assert!(false);
            -1
        }
    }
}

/*
static Ref
iins(int cls, int op, Ref a0, Ref a1, Loc *l)
{
    Insert *ist;

    vgrow(&ilog, ++nlog);
    ist = &ilog[nlog-1];
    ist->isphi = 0;
    ist->num = inum++;
    ist->bid = l->blk->id;
    ist->off = l->off;
    ist->new.ins = (Ins){op, cls, R, {a0, a1}};
    return ist->new.ins.to = newtmp("ld", cls, curf);
}

static void
cast(Ref *r, int cls, Loc *l)
{
    int cls0;

    if (rtype(*r) == RCon)
        return;
    assert(rtype(*r) == RTmp);
    cls0 = curf->tmp[r->val].cls;
    if (cls0 == cls || (cls == Kw && cls0 == Kl))
        return;
    if (KWIDE(cls0) < KWIDE(cls)) {
        if (cls0 == Ks)
            *r = iins(Kw, Ocast, *r, R, l);
        *r = iins(Kl, Oextuw, *r, R, l);
        if (cls == Kd)
            *r = iins(Kd, Ocast, *r, R, l);
    } else {
        if (cls0 == Kd && cls != Kl)
            *r = iins(Kl, Ocast, *r, R, l);
        if (cls0 != Kd || cls != Kw)
            *r = iins(cls, Ocast, *r, R, l);
    }
}

static inline void
mask(int cls, Ref *r, bits msk, Loc *l)
{
    cast(r, cls, l);
    *r = iins(cls, Oand, *r, getcon(msk, curf), l);
}

static Ref
load(Slice sl, bits msk, Loc *l)
{
    Alias *a;
    Ref r, r1;
    int ld, cls, all;
    Con c;

    ld = (int[]){
        [1] = Oloadub,
        [2] = Oloaduh,
        [4] = Oloaduw,
        [8] = Oload
    }[sl.sz];
    all = msk == MASK(sl.sz);
    if (all)
        cls = sl.cls;
    else
        cls = sl.sz > 4 ? Kl : Kw;
    r = sl.ref;
    /* sl.ref might not be live here,
     * but its alias base ref will be
     * (see killsl() below) */
    if (rtype(r) == RTmp) {
        a = &curf->tmp[r.val].alias;
        switch (a->type) {
        default:
            die("unreachable");
        case ALoc:
        case AEsc:
        case AUnk:
            r = TMP(a->base);
            if (!a->offset)
                break;
            r1 = getcon(a->offset, curf);
            r = iins(Kl, Oadd, r, r1, l);
            break;
        case ACon:
        case ASym:
            memset(&c, 0, sizeof c);
            c.type = CAddr;
            c.sym = a->u.sym;
            c.bits.i = a->offset;
            r = newcon(&c, curf);
            break;
        }
    }
    r = iins(cls, ld, r, R, l);
    if (!all)
        mask(cls, &r, msk, l);
    return r;
}
 */

fn killsl(f: &Fn, r: Ref, sl: &Slice) -> bool {
    //Alias *a;

    if let Ref::RTmp(_ti) = r {
        if let Ref::RTmp(slti) = sl.r {
            let ai: AliasIdx = f.tmp(slti).alias;
            let a: &Alias = f.alias(ai);
            return match a.type_ {
                AliasT::ALoc | AliasT::AEsc | AliasT::AUnk => r == Ref::RTmp(a.base),
                AliasT::ACon | AliasT::ASym => false,
                _ => {
                    // unreachable
                    assert!(false);
                    false
                }
            };
        }
    }
    // Neither r or sl.r are RTmp
    false
}

/* returns a ref containing the contents of the slice
 * passed as argument, all the bits set to 0 in the
 * mask argument are zeroed in the result;
 * the returned ref has an integer class when the
 * mask does not cover all the bits of the slice,
 * otherwise, it has class sl.cls
 * the procedure returns R when it fails */
fn def(
    f: &mut Fn,
    ilog: &mut Vec<Insert>,
    sl: Slice,
    msk: Bits,
    bi: BlkIdx,
    mut ii: InsIdx,
    il: &Loc,
) -> Ref {
    // Slice sl1;
    // Blk *bp;
    // bits msk1, msks;
    // int off, cls, cls1, op, sz, ld;
    // uint np, oldl, oldt;
    // Ref r, r1;
    // Phi *p;
    // Insert *ist;
    // Loc l;

    /* invariants:
     * -1- Blk bi dominates Blk il.bi; so we can use
     *     temporaries of Blk bi in Blk il.bi
     * -2- if il.type_ != LNoLoad, then blk il.bi
     *     postdominates the original load; so it
     *     is safe to load in Blk il.bi
     * -3- if il.type_ != LNoLoad, then blk bi
     *     postdominates Blk il.bi (and by 2, the
     *     original load)
     */
    assert!(dom(f, bi, il.bi));
    let oldl: usize = ilog.len();
    let oldt: usize = f.tmps.len();
    // if (0) {
    // Load:
    //     f->ntmp = oldt;
    //     nlog = oldl;
    //     if (il->type != LLoad)
    //         return R;
    //     return load(sl, msk, il);
    // }

    if ii == InsIdx::NONE {
        // Bit naughty - this is out of range
        ii = InsIdx(f.blk(bi).ins.len() as u32);
    }
    let cls: KExt = if sl.sz > 4 { KL } else { KW };
    let msks: Bits = mask(sl.sz as u32);

    let mut goto_load: bool = false;
    while ii != InsIdx(0) && !goto_load {
        // Hrmm
        // while (i > b->ins) {
        ii = InsIdx(ii.0 - 1);
        let mut i: Ins = f.blk(bi).ins[ii.0 as usize]; /* Note: copy! */
        if killsl(f, i.to, &sl) || (i.op == O::Ocall && escapes(f, sl.r)) {
            goto_load = true;
            continue;
        }
        let ld: bool = isload(i.op);
        let (sz, r1, r) = {
            if ld {
                (loadsz(&i), i.args[0], i.to)
            } else if isstore(i.op) {
                (storesz(&i), i.args[1], i.args[0])
            } else if i.op == O::Oblit1 {
                if let Ref::RInt(blit1_i) = i.args[0] {
                    assert!(ii != InsIdx(0));
                    ii = InsIdx(ii.0 - 1);
                    i = f.blk(bi).ins[ii.0 as usize];
                    assert!(i.op == O::Oblit0);
                    (blit1_i.abs(), i.args[1], Ref::R) // Hrmmm, QBE does not set r here
                } else {
                    // OBlit1 arg MUST be RInt
                    assert!(false);
                    continue;
                }
            } else {
                continue;
            }
        };
        //     switch (alias(sl.ref, sl.off, sl.sz, r1, sz, &off, f)) {
        //     case MustAlias:
        //         if (i->op == Oblit0) {
        //             sl1 = sl;
        //             sl1.ref = i->arg[0];
        //             if (off >= 0) {
        //                 assert(off < sz);
        //                 sl1.off = off;
        //                 sz -= off;
        //                 off = 0;
        //             } else {
        //                 sl1.off = 0;
        //                 sl1.sz += off;
        //             }
        //             if (sz > sl1.sz)
        //                 sz = sl1.sz;
        //             assert(sz <= 8);
        //             sl1.sz = sz;
        //         }
        //         if (off < 0) {
        //             off = -off;
        //             msk1 = (MASK(sz) << 8*off) & msks;
        //             op = Oshl;
        //         } else {
        //             msk1 = (MASK(sz) >> 8*off) & msks;
        //             op = Oshr;
        //         }
        //         if ((msk1 & msk) == 0)
        //             continue;
        //         if (i->op == Oblit0) {
        //             r = def(sl1, MASK(sz), b, i, il);
        //             if (req(r, R))
        //                 goto Load;
        //         }
        //         if (off) {
        //             cls1 = cls;
        //             if (op == Oshr && off + sl.sz > 4)
        //                 cls1 = Kl;
        //             cast(&r, cls1, il);
        //             r1 = getcon(8*off, f);
        //             r = iins(cls1, op, r, r1, il);
        //         }
        //         if ((msk1 & msk) != msk1 || off + sz < sl.sz)
        //             mask(cls, &r, msk1 & msk, il);
        //         if ((msk & ~msk1) != 0) {
        //             r1 = def(sl, msk & ~msk1, b, i, il);
        //             if (req(r1, R))
        //                 goto Load;
        //             r = iins(cls, Oor, r, r1, il);
        //         }
        //         if (msk == msks)
        //             cast(&r, sl.cls, il);
        //         return r;
        //     case MayAlias:
        //         if (ld)
        //             continue;
        //         else
        //             goto Load;
        //     case NoAlias:
        //         continue;
        //     default:
        //         die("unreachable");
        //     }
    }

    if goto_load {
        panic!("Implement me");
    }

    // for (ist=ilog; ist<&ilog[nlog]; ++ist)
    //     if (ist->isphi && ist->bid == b->id)
    //     if (req(ist->new.phi.m.ref, sl.ref))
    //     if (ist->new.phi.m.off == sl.off)
    //     if (ist->new.phi.m.sz == sl.sz) {
    //         r = ist->new.phi.p->to;
    //         if (msk != msks)
    //             mask(cls, &r, msk, il);
    //         else
    //             cast(&r, sl.cls, il);
    //         return r;
    //     }

    // for (p=b->phi; p; p=p->link)
    //     if (killsl(p->to, sl))
    //         /* scanning predecessors in that
    //          * case would be unsafe */
    //         goto Load;

    // if (b->npred == 0)
    //     goto Load;
    // if (b->npred == 1) {
    //     bp = b->pred[0];
    //     assert(bp->loop >= il->blk->loop);
    //     l = *il;
    //     if (bp->s2)
    //         l.type = LNoLoad;
    //     r1 = def(sl, msk, bp, 0, &l);
    //     if (req(r1, R))
    //         goto Load;
    //     return r1;
    // }

    // r = newtmp("ld", sl.cls, f);
    // p = alloc(sizeof *p);
    // vgrow(&ilog, ++nlog);
    // ist = &ilog[nlog-1];
    // ist->isphi = 1;
    // ist->bid = b->id;
    // ist->new.phi.m = sl;
    // ist->new.phi.p = p;
    // p->to = r;
    // p->cls = sl.cls;
    // p->narg = b->npred;
    // p->arg = vnew(p->narg, sizeof p->arg[0], PFn);
    // p->blk = vnew(p->narg, sizeof p->blk[0], PFn);
    // for (np=0; np<b->npred; ++np) {
    //     bp = b->pred[np];
    //     if (!bp->s2
    //     && il->type != LNoLoad
    //     && bp->loop < il->blk->loop)
    //         l.type = LLoad;
    //     else
    //         l.type = LNoLoad;
    //     l.blk = bp;
    //     l.off = bp->nins;
    //     r1 = def(sl, msks, bp, 0, &l);
    //     if (req(r1, R))
    //         goto Load;
    //     p->arg[np] = r1;
    //     p->blk[np] = bp;
    // }
    // if (msk != msks)
    //     mask(cls, &r, msk, il);
    // return r;
    Ref::R // for now...
}
/*
static int
icmp(const void *pa, const void *pb)
{
    Insert *a, *b;
    int c;

    a = (Insert *)pa;
    b = (Insert *)pb;
    if ((c = a->bid - b->bid))
        return c;
    if (a->isphi && b->isphi)
        return 0;
    if (a->isphi)
        return -1;
    if (b->isphi)
        return +1;
    if ((c = a->off - b->off))
        return c;
    return a->num - b->num;
}
 */
/* require rpo ssa alias */
pub fn loadopt(f: &mut Fn) {
    // Ins *i, *ib;
    // Blk *b;
    // int sz;
    // uint n, ni, ext, nt;
    // Insert *ist;
    // Slice sl;
    // Loc l;

    //curf = fn;
    let mut ilog: Vec<Insert> = vec![];
    //nlog = 0;
    //let inum: usize = 0;

    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        for iii in 0..f.blk(bi).ins.len() {
            let i_arg1 = {
                let i: &Ins = &f.blk(bi).ins[iii];
                if !isload(i.op) {
                    continue;
                }
                let sz: i32 = loadsz(i);
                let sl: Slice = Slice {
                    r: i.args[0],
                    off: 0,
                    sz: sz as i16,
                    cls: i.cls,
                };
                let ii: InsIdx = InsIdx(iii as u32);
                let l: Loc = Loc {
                    type_: LocT::LRoot,
                    off: ii,
                    bi,
                };
                def(f, &mut ilog, sl, mask(sz as u32), bi, ii, &l)
            };
            f.blk_mut(bi).ins[iii].args[1] = i_arg1;
        }
        bi = f.blk(bi).link;
    }
    /*
        qsort(ilog, nlog, sizeof ilog[0], icmp);
        vgrow(&ilog, nlog+1);
        ilog[nlog].bid = fn->nblk; /* add a sentinel */
        ib = vnew(0, sizeof(Ins), PHeap);
        for (ist=ilog, n=0; n<fn->nblk; ++n) {
            b = fn->rpo[n];
            for (; ist->bid == n && ist->isphi; ++ist) {
                ist->new.phi.p->link = b->phi;
                b->phi = ist->new.phi.p;
            }
            ni = 0;
            nt = 0;
            for (;;) {
                if (ist->bid == n && ist->off == ni)
                    i = &ist++->new.ins;
                else {
                    if (ni == b->nins)
                        break;
                    i = &b->ins[ni++];
                    if (isload(i->op)
                    && !req(i->arg[1], R)) {
                        ext = Oextsb + i->op - Oloadsb;
                        switch (i->op) {
                        default:
                            die("unreachable");
                        case Oloadsb:
                        case Oloadub:
                        case Oloadsh:
                        case Oloaduh:
                            i->op = ext;
                            break;
                        case Oloadsw:
                        case Oloaduw:
                            if (i->cls == Kl) {
                                i->op = ext;
                                break;
                            }
                            /* fall through */
                        case Oload:
                            i->op = Ocopy;
                            break;
                        }
                        i->arg[0] = i->arg[1];
                        i->arg[1] = R;
                    }
                }
                vgrow(&ib, ++nt);
                ib[nt-1] = *i;
            }
            b->nins = nt;
            idup(&b->ins, ib, nt);
        }
        vfree(ib);
        vfree(ilog);
        if (debug['M']) {
            fprintf(stderr, "\n> After load elimination:\n");
            printfn(fn, stderr);
        }
    */
}
