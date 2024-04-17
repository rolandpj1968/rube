use derive_new::new;

use crate::alias::{alias, escapes};
use crate::all::{
    bit, isload, isstore, kwide, Alias, AliasIdx, AliasT, Bits, BlkIdx, CanAlias, Fn, Ins, InsIdx,
    KExt, PhiIdx, Ref, TmpIdx, KD, KL, KS, KW, O,
};
use crate::cfg::dom;
use crate::util::{getcon, newtmp};

/*
#include "all.h"
 */

// TODO i32 is dodgy
fn genmask(w: i32) -> Bits {
    assert!(0 <= w && w <= 8);
    bit((8 * w - 1) as u32) * 2 - 1 /* must work when w==8 */
}

#[derive(Debug)]
#[repr(u8)]
enum LocT {
    LRoot,   /* right above the original load */
    LLoad,   /* inserting a load is allowed */
    LNoLoad, /* only scalar operations allowed */
}

#[derive(Debug)]
struct Loc {
    type_: LocT,
    off: InsIdx,
    bi: BlkIdx,
}

#[derive(Clone, Debug)]
struct Slice {
    r: Ref,
    off: i32,
    sz: i16,   // Dodgy i16
    cls: KExt, /* load class */
}

#[derive(Debug)]
struct UPhi {
    m: Slice,
    pi: PhiIdx,
}

#[derive(Debug)]
#[repr(u8)]
enum InsertU {
    Ins(Ins),
    Phi(UPhi),
}

#[derive(Debug, new)]
struct Insert {
    // isphi: bool, covered by InsertU
    num: TmpIdx, // TODO rename to ti
    bid: u32,
    off: InsIdx, // TODO rename to ii
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

fn iins(f: &mut Fn, ilog: &mut Vec<Insert>, cls: KExt, op: O, a0: Ref, a1: Ref, l: &Loc) -> Ref {
    let ti: TmpIdx = newtmp(b"ld", true, cls, f);
    let to: Ref = Ref::RTmp(ti);
    let ins: Ins = Ins::new2(op, cls, to, [a0, a1]);
    ilog.push(Insert::new(ti, f.blk(l.bi).id, l.off, InsertU::Ins(ins)));
    to
}

fn cast(f: &mut Fn, ilog: &mut Vec<Insert>, r: &mut Ref, cls: KExt, l: &Loc) {
    match *r {
        Ref::RCon(_) => (), /*ok*/
        Ref::RTmp(ti) => {
            let cls0: KExt = f.tmp(ti).cls;
            if cls0 == cls || (cls == KW && cls0 == KL) {
                return;
            }
            if kwide(cls0) < kwide(cls) {
                if cls0 == KS {
                    *r = iins(f, ilog, KW, O::Ocast, *r, Ref::R, l);
                }
                *r = iins(f, ilog, KL, O::Oextuw, *r, Ref::R, l);
                if cls == KD {
                    *r = iins(f, ilog, KD, O::Ocast, *r, Ref::R, l);
                }
            } else {
                if cls0 == KD && cls != KL {
                    *r = iins(f, ilog, KL, O::Ocast, *r, Ref::R, l);
                }
                if cls0 != KD || cls != KW {
                    *r = iins(f, ilog, cls, O::Ocast, *r, Ref::R, l);
                }
            }
        }
        _ => assert!(false), // r MUST be RCon or RTmp
    }
}

fn mask(f: &mut Fn, ilog: &mut Vec<Insert>, cls: KExt, r: &mut Ref, msk: Bits, l: &Loc) {
    cast(f, ilog, r, cls, l);
    let c = getcon(f, msk as i64);
    *r = iins(f, ilog, cls, O::Oand, *r, c, l);
}

/*
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
    sl: &Slice,
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
    let msks: Bits = genmask(sl.sz as i32);

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
        let (mut sz, mut r1, mut r) = {
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
                    (blit1_i.abs(), i.args[1], Ref::R)
                } else {
                    // OBlit1 arg MUST be RInt
                    assert!(false);
                    continue;
                }
            } else {
                continue;
            }
        };
        let (can_alias, mut off) = alias(f, sl.r, sl.off, sl.sz as i32, r1, sz);
        match can_alias {
            CanAlias::Must => {
                let mut sl1: Slice = sl.clone(); /*for Oblit0 only, ugh!*/
                if i.op == O::Oblit0 {
                    //sl1 = sl;
                    sl1.r = i.args[0];
                    if off >= 0 {
                        assert!(off < sz);
                        sl1.off = off;
                        sz -= off;
                        off = 0;
                    } else {
                        sl1.off = 0;
                        sl1.sz += off as i16; // Dodgy
                    }
                    if sz > (sl1.sz as i32) {
                        sz = sl1.sz as i32;
                    }
                    assert!(sz <= 8);
                    sl1.sz = sz as i16; // Dodgy
                }
                let (msk1, op) = if off < 0 {
                    off = -off; // ???
                    ((genmask(sz) << (8 * off)) & msks, O::Oshl)
                } else {
                    ((genmask(sz) >> (8 * off)) & msks, O::Oshr)
                };
                if (msk1 & msk) == 0 {
                    continue;
                }
                if i.op == O::Oblit0 {
                    r = def(f, ilog, &sl1, genmask(sz), bi, ii, il);
                    if r == Ref::R {
                        goto_load = true;
                        continue;
                    }
                }
                if off != 0 {
                    let cls1: KExt = if op == O::Oshr && off + (sl.sz as i32) > 4 {
                        KL
                    } else {
                        cls
                    };
                    cast(f, ilog, &mut r, cls1, il);
                    r1 = getcon(f, 8 * (off as i64));
                    r = iins(f, ilog, cls1, op, r, r1, il);
                }
                if (msk1 & msk) != msk1 || off + sz < sl.sz as i32 {
                    mask(f, ilog, cls, &mut r, msk1 & msk, il);
                }
                if (msk & !msk1) != 0 {
                    r1 = def(f, ilog, sl, msk & !msk1, bi, ii, il);
                    if r1 == Ref::R {
                        goto_load = true;
                        continue;
                    }
                    r = iins(f, ilog, cls, O::Oor, r, r1, il);
                }
                if msk == msks {
                    cast(f, ilog, &mut r, sl.cls, il);
                }
                return r;
            }
            CanAlias::May => {
                if !ld {
                    goto_load = true;
                }
                continue;
            }
            CanAlias::No => continue,
        }
    }

    if goto_load {
        f.tmps.truncate(oldt);
        ilog.truncate(oldl);
        return if il.type_ == LLoad {
            load(sl, msk, il)
        } else {
            Ref::R
        };
    }

    // for (ist=ilog; ist<&ilog[nlog]; ++ist)
    //     if (ist.isphi && ist.bid == b.id)
    //     if (req(ist.new.phi.m.ref, sl.ref))
    //     if (ist.new.phi.m.off == sl.off)
    //     if (ist.new.phi.m.sz == sl.sz) {
    //         r = ist.new.phi.p.to;
    //         if (msk != msks)
    //             mask(cls, &r, msk, il);
    //         else
    //             cast(&r, sl.cls, il);
    //         return r;
    //     }

    // for (p=b.phi; p; p=p.link)
    //     if (killsl(p.to, sl))
    //         /* scanning predecessors in that
    //          * case would be unsafe */
    //         goto Load;

    // if (b.npred == 0)
    //     goto Load;
    // if (b.npred == 1) {
    //     bp = b.pred[0];
    //     assert!(bp.loop >= il.blk.loop);
    //     l = *il;
    //     if (bp.s2)
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
    // ist.isphi = 1;
    // ist.bid = b.id;
    // ist.new.phi.m = sl;
    // ist.new.phi.p = p;
    // p.to = r;
    // p.cls = sl.cls;
    // p.narg = b.npred;
    // p.arg = vnew(p.narg, sizeof p.arg[0], PFn);
    // p.blk = vnew(p.narg, sizeof p.blk[0], PFn);
    // for (np=0; np<b.npred; ++np) {
    //     bp = b.pred[np];
    //     if (!bp.s2
    //     && il.type != LNoLoad
    //     && bp.loop < il.blk.loop)
    //         l.type = LLoad;
    //     else
    //         l.type = LNoLoad;
    //     l.blk = bp;
    //     l.off = bp.nins;
    //     r1 = def(sl, msks, bp, 0, &l);
    //     if (req(r1, R))
    //         goto Load;
    //     p.arg[np] = r1;
    //     p.blk[np] = bp;
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
    if ((c = a.bid - b.bid))
        return c;
    if (a.isphi && b.isphi)
        return 0;
    if (a.isphi)
        return -1;
    if (b.isphi)
        return +1;
    if ((c = a.off - b.off))
        return c;
    return a.num - b.num;
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
                def(f, &mut ilog, &sl, genmask(sz), bi, ii, &l)
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
