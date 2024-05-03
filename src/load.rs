use derive_new::new;
use std::cell;
use std::cmp::Ordering;
use std::io::stdout;

use crate::alias::{alias, escapes};
use crate::all::Ref::{RCon, RInt, RTmp, R};
use crate::all::K::{Kd, Kl, Ks, Kw, Kx};
use crate::all::{
    bit, for_each_bi_mut, isload, isstore, kwide, Alias, AliasT, AliasU, Bits, Blk, BlkIdx,
    CanAlias, Con, Fn, Ins, InsIdx, Phi, PhiIdx, Ref, RpoIdx, Tmp, TmpIdx, Typ, K, O,
};
use crate::cfg::dom;
use crate::parse::printfn;
use crate::util::{getcon2, newcon2, newtmp2, newtmpref2, Bucket};

// TODO i32 is dodgy
fn genmask(w: i32) -> Bits {
    assert!(0 <= w && w <= 8);
    bit((8 * w - 1) as usize).wrapping_mul(2).wrapping_sub(1) /* must work when w==8 */
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum LocT {
    LRoot,   /* right above the original load */
    LLoad,   /* inserting a load is allowed */
    LNoLoad, /* only scalar operations allowed */
}

#[derive(Clone, Copy, Debug)]
struct Loc {
    typ: LocT,
    off: InsIdx,
    bi: BlkIdx,
}

#[derive(Clone, Copy, Debug)]
struct Slice {
    r: Ref,
    off: i32,
    sz: i16, // Dodgy i16
    cls: K,  /* load class */
}

#[derive(Debug)]
struct UPhi {
    m: Slice,
    pi: PhiIdx,
}

#[derive(Debug)]
#[repr(u8)]
enum InsertU {
    Ins(InsIdx, Ins),
    Phi(UPhi),
}

#[derive(Debug, new)]
struct Insert {
    num: TmpIdx, // TODO rename to ti
    bid: RpoIdx,
    new: InsertU,
}

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

fn iins(
    blks: &[Blk],
    tmps: &mut Vec<Tmp>,
    ilog: &mut Vec<Insert>,
    cls: K,
    op: O,
    a0: Ref,
    a1: Ref,
    l: &Loc,
) -> Ref {
    let ti: TmpIdx = newtmp2(tmps, b"ld", true, cls);
    let to: Ref = RTmp(ti);
    let ins: Ins = Ins::new2(op, cls, to, [a0, a1]);
    ilog.push(Insert::new(ti, blks[l.bi].id, InsertU::Ins(l.off, ins)));
    to
}

fn cast(blks: &[Blk], tmps: &mut Vec<Tmp>, ilog: &mut Vec<Insert>, r: &mut Ref, cls: K, l: &Loc) {
    match *r {
        RCon(_) => (), /*ok*/
        RTmp(ti) => {
            let cls0: K = tmps[ti].cls;
            if cls0 == cls || (cls == Kw && cls0 == Kl) {
                return;
            }
            if kwide(cls0) < kwide(cls) {
                if cls0 == Ks {
                    *r = iins(blks, tmps, ilog, Kw, O::Ocast, *r, R, l);
                }
                *r = iins(blks, tmps, ilog, Kl, O::Oextuw, *r, R, l);
                if cls == Kd {
                    *r = iins(blks, tmps, ilog, Kd, O::Ocast, *r, R, l);
                }
            } else {
                if cls0 == Kd && cls != Kl {
                    *r = iins(blks, tmps, ilog, Kl, O::Ocast, *r, R, l);
                }
                if cls0 != Kd || cls != Kw {
                    *r = iins(blks, tmps, ilog, cls, O::Ocast, *r, R, l);
                }
            }
        }
        _ => assert!(false), // r MUST be RCon or RTmp
    }
}

fn mask(
    blks: &[Blk],
    tmps: &mut Vec<Tmp>,
    cons: &mut Vec<Con>,
    ilog: &mut Vec<Insert>,
    cls: K,
    r: &mut Ref,
    msk: Bits,
    l: &Loc,
) {
    cast(blks, tmps, ilog, r, cls, l);
    let c = getcon2(cons, msk as i64);
    *r = iins(blks, tmps, ilog, cls, O::Oand, *r, c, l);
}

fn load(
    blks: &[Blk],
    tmps: &mut Vec<Tmp>,
    cons: &mut Vec<Con>,
    ilog: &mut Vec<Insert>,
    sl: &Slice,
    msk: Bits,
    l: &Loc,
) -> Ref {
    let ld: O = match sl.sz {
        1 => O::Oloadub,
        2 => O::Oloaduh,
        4 => O::Oloaduw,
        8 => O::Oload,
        _ => {
            assert!(false);
            O::Oxxx
        }
    };
    let all: bool = msk == genmask(sl.sz as i32);
    let cls: K = if all {
        sl.cls
    } else {
        if sl.sz > 4 {
            Kl
        } else {
            Kw
        }
    };
    let mut r: Ref = sl.r;
    /* sl.ref might not be live here,
     * but its alias base ref will be
     * (see killsl() below) */
    if let RTmp(ti) = r {
        let a: Alias = tmps[ti].alias; //*f.alias(ai); // Note - copy!
        match a.typ {
            AliasT::ALoc | AliasT::AEsc | AliasT::AUnk => {
                r = RTmp(a.base);
                if a.offset != 0 {
                    let r1: Ref = getcon2(cons, a.offset);
                    r = iins(blks, tmps, ilog, Kl, O::Oadd, r, r1, l);
                }
            }
            AliasT::ACon | AliasT::ASym => {
                if let AliasU::ASym(sym) = a.u {
                    r = newcon2(cons, Con::CAddr(sym, a.offset));
                } else {
                    assert!(false);
                    r = R; // Ugh, TODO
                }
            }
            _ => {
                // unreachable
                assert!(false);
                r = R;
            }
        }
    }
    r = iins(blks, tmps, ilog, cls, ld, r, R, l);
    if !all {
        mask(blks, tmps, cons, ilog, cls, &mut r, msk, l);
    }
    r
}

fn killsl(tmps: &[Tmp], r: Ref, sl: &Slice) -> bool {
    if let RTmp(_ti) = r {
        if let RTmp(slti) = sl.r {
            let a: &Alias = &tmps[slti].alias;
            return match a.typ {
                AliasT::ALoc | AliasT::AEsc | AliasT::AUnk => r == RTmp(a.base),
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
    blks: &[Blk],
    phis: &mut Vec<Phi>,
    tmps: &mut Vec<Tmp>,
    cons: &mut Vec<Con>,
    ilog: &mut Vec<Insert>,
    sl: &Slice,
    msk: Bits,
    bi: BlkIdx,
    mut ii: InsIdx,
    il: &Loc,
) -> Ref {
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
    assert!(dom(blks, bi, il.bi));
    let oldl: usize = ilog.len();
    let oldt: usize = tmps.len();

    if ii == InsIdx::NONE {
        // Bit naughty - this is out of range
        ii = InsIdx::from(blks.borrow(bi).ins().len());
    }
    let cls: K = if sl.sz > 4 { Kl } else { Kw };
    let msks: Bits = genmask(sl.sz as i32);

    let mut goto_load: bool = false;
    while ii != InsIdx::from(0) && !goto_load {
        ii = InsIdx::from(ii.usize() - 1);
        let mut i: Ins = blks.borrow(bi).ins()[ii.0 as usize]; /* Note: copy! */
        if killsl(tmps, i.to, sl) || (i.op == O::Ocall && escapes(tmps, sl.r)) {
            // println!("                              killsl or escaping call");
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
                if let RInt(blit1_i) = i.args[0] {
                    assert!(ii != InsIdx::from(0));
                    ii = InsIdx::from(ii.usize() - 1);
                    i = blks.borrow(bi).ins()[ii.0 as usize];
                    assert!(i.op == O::Oblit0);
                    (blit1_i.abs(), i.args[1], R)
                } else {
                    // OBlit1 arg MUST be RInt
                    assert!(false);
                    continue;
                }
            } else {
                continue;
            }
        };
        let (can_alias, mut off) = alias(tmps, cons, sl.r, sl.off, sl.sz as i32, r1, sz);
        match can_alias {
            CanAlias::Must => {
                let mut sl1: Slice = *sl; /*for Oblit0 only, ugh!*/
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
                    r = def(blks, phis, tmps, cons, ilog, &sl1, genmask(sz), bi, ii, il);
                    if r == R {
                        goto_load = true;
                        continue;
                    }
                }
                if off != 0 {
                    let cls1: K = if op == O::Oshr && off + (sl.sz as i32) > 4 {
                        Kl
                    } else {
                        cls
                    };
                    cast(blks, tmps, ilog, &mut r, cls1, il);
                    r1 = getcon2(cons, 8 * (off as i64));
                    r = iins(blks, tmps, ilog, cls1, op, r, r1, il);
                }
                if (msk1 & msk) != msk1 || off + sz < sl.sz as i32 {
                    mask(blks, tmps, cons, ilog, cls, &mut r, msk1 & msk, il);
                }
                if (msk & !msk1) != 0 {
                    r1 = def(blks, phis, tmps, cons, ilog, sl, msk & !msk1, bi, ii, il);
                    if r1 == R {
                        goto_load = true;
                        continue;
                    }
                    r = iins(blks, tmps, ilog, cls, O::Oor, r, r1, il);
                }
                if msk == msks {
                    cast(blks, tmps, ilog, &mut r, sl.cls, il);
                }
                return r;
            }
            CanAlias::May => {
                if !ld {
                    goto_load = true;
                }
                continue;
            }
            CanAlias::No => {
                continue;
            }
        }
    }

    if !goto_load {
        let bid = blks.borrow(bi).id;

        for isti in 0..ilog.len() {
            let ist: &Insert = &ilog[isti];
            if let InsertU::Phi(uphi) = &ist.new {
                if ist.bid == bid && uphi.m.r == sl.r && uphi.m.off == sl.off && uphi.m.sz == sl.sz
                {
                    let mut r = phis[uphi.pi].to;
                    if msk != msks {
                        mask(blks, tmps, cons, ilog, cls, &mut r, msk, il);
                    } else {
                        cast(blks, tmps, ilog, &mut r, sl.cls, il);
                    }
                    return r;
                }
            }
        }

        let mut pi = blks.borrow(bi).phi;
        while pi != PhiIdx::NONE {
            let p_to: Ref = phis[pi].to;
            if killsl(tmps, p_to, sl) {
                /* scanning predecessors in that
                 * case would be unsafe */
                goto_load = true;
                break;
            }
            pi = phis[pi].link;
        }
    }

    goto_load = goto_load || blks.borrow(bi).preds.is_empty();

    if !goto_load {
        if blks.borrow(bi).preds.len() == 1 {
            let bpi = blks.borrow(bi).preds[0];
            assert!(blks.borrow(bpi).loop_ >= blks.borrow(il.bi).loop_);
            let mut l: Loc = *il;
            if blks.borrow(bpi).s2 != BlkIdx::NONE {
                l.typ = LocT::LNoLoad;
            }
            let r1: Ref = def(blks, phis, tmps, cons, ilog, sl, msk, bpi, InsIdx::NONE, &l);
            if r1 == R {
                goto_load = true;
            } else {
                return r1;
            }
        }
    }

    let mut r: Ref = R;
    if !goto_load {
        r = newtmpref2(tmps, b"ld", true, sl.cls);
        let p: Phi = Phi::new(r, vec![], vec![], sl.cls, PhiIdx::NONE);
        let pi: PhiIdx = PhiIdx::from(phis.len());
        phis.push(p);
        // TODO - notify QBE? QBE doesn't seem to set ist.num (i.e. ti). Nor off
        // I suspect to should be r's ti, not 0???
        // Maybe for phi's, QBE gets "to" from UPhi(p.to)
        ilog.push(Insert::new(
            TmpIdx::from(0), /*TODO*/
            blks.borrow(bi).id,
            InsertU::Phi(UPhi { m: *sl, pi }),
        ));
        let preds_len = blks.borrow(bi).preds.len();
        for np in 0..preds_len {
            let bpi: BlkIdx = blks.borrow(bi).preds[np];
            let l_type: LocT;
            if blks.borrow(bpi).s2 == BlkIdx::NONE
                && il.typ != LocT::LNoLoad
                && blks.borrow(bpi).loop_ < blks.borrow(il.bi).loop_
            {
                l_type = LocT::LLoad;
            } else {
                l_type = LocT::LNoLoad;
            }
            let l: Loc = Loc {
                typ: l_type,
                bi: bpi,
                off: InsIdx::from(blks.borrow(bpi).ins().len()),
            };
            let r1: Ref = def(
                blks,
                phis,
                tmps,
                cons,
                ilog,
                &sl,
                msks,
                bpi,
                InsIdx::NONE,
                &l, /*, indent + 1, debug*/
            );
            if r1 == R {
                goto_load = true;
                break;
            }
            phis[pi].args.push(r1);
            phis[pi].blks.push(bpi);
        }
    }

    if goto_load {
        tmps.truncate(oldt);
        ilog.truncate(oldl);
        if il.typ != LocT::LLoad {
            return R;
        }
        load(blks, tmps, cons, ilog, sl, msk, il)
    } else {
        if msk != msks {
            mask(blks, tmps, cons, ilog, cls, &mut r, msk, il);
        }
        r
    }
}

fn icmp(a: &Insert, b: &Insert) -> Ordering {
    let bid_cmp = a.bid.cmp(&b.bid);
    if bid_cmp != Ordering::Equal {
        return bid_cmp;
    }
    let a_isphi: bool = matches!(a.new, InsertU::Phi(_));
    let b_isphi: bool = matches!(b.new, InsertU::Phi(_));
    if a_isphi && b_isphi {
        return Ordering::Equal;
    }
    if a_isphi {
        return Ordering::Less;
    }
    if b_isphi {
        return Ordering::Greater;
    }
    if let (InsertU::Ins(aii, _), InsertU::Ins(bii, _)) = (&a.new, &b.new) {
        let off_cmp = aii.0.cmp(&bii.0);
        if off_cmp != Ordering::Equal {
            return off_cmp;
        }
    }
    a.num.0.cmp(&b.num.0)
}

/* require rpo ssa alias */
pub fn loadopt(f: &mut Fn, typ: &[Typ], itbl: &[Bucket]) {
    let blks: &[Blk] = &f.blks;
    let phis: &mut Vec<Phi> = &mut f.phis;
    let tmps: &mut Vec<Tmp> = &mut f.tmps;
    let cons: &mut Vec<Con> = &mut f.cons;

    let mut ilog: Vec<Insert> = vec![];

    for_each_bi_mut(blks, |bi| {
        let ins_len = blks.borrow(bi).ins().len();
        for iii in 0..ins_len {
            let i_arg1 = {
                let i: Ins = blks.borrow(bi).ins()[iii]; // Note - copy
                if !isload(i.op) {
                    continue;
                }
                let sz: i32 = loadsz(&i);
                let sl: Slice = Slice {
                    r: i.args[0],
                    off: 0,
                    sz: sz as i16,
                    cls: i.cls,
                };
                let ii: InsIdx = InsIdx::from(iii);
                let l: Loc = Loc {
                    typ: LocT::LRoot,
                    off: ii,
                    bi,
                };
                def(
                    blks,
                    phis,
                    tmps,
                    cons,
                    &mut ilog,
                    &sl,
                    genmask(sz),
                    bi,
                    ii,
                    &l,
                )
            };
            blks.borrow_mut(bi).ins_mut()[iii].args[1] = i_arg1;
        }
    });
    ilog.sort_by(icmp);
    let sentinal_ins = Ins::new0(O::Oxxx, Kx, R);
    /* add a sentinel */
    // TODO - why???
    ilog.push(Insert::new(
        TmpIdx::NONE,
        RpoIdx::from(f.nblk as usize), // RpoIdx::NONE???
        //InsIdx::NONE,
        InsertU::Ins(InsIdx::NONE, sentinal_ins),
    ));
    let mut isti: usize = 0;
    let mut n: RpoIdx = RpoIdx::from(0);
    // Ugh, fixme
    while n.usize() < f.nblk as usize {
        let mut ist: &mut Insert = &mut ilog[isti];
        let bi: BlkIdx = f.rpo[n];
        while ist.bid == n {
            if let InsertU::Phi(uphi) = &mut ist.new {
                let pi = blks.borrow(bi).phi;
                phis[uphi.pi].link = pi;
                blks.borrow_mut(bi).phi = uphi.pi;
            } else {
                break;
            }
            isti += 1;
            ist = &mut ilog[isti];
        }
        let mut ni: InsIdx = InsIdx::from(0);
        let mut ib: Vec<Ins> = vec![];
        loop {
            let mut i: Ins;
            let (ni0, i0) = if let InsertU::Ins(ni1, i1) = ist.new {
                (ni1, i1)
            } else {
                // MUST be InsertU::Ins
                //assert!(false); TODO... triggering???
                (InsIdx::NONE, Ins::new0(O::Oxxx, Kx, R))
            };
            if ist.bid == n && ni == ni0 {
                i = i0; // Copy
                isti += 1;
                ist = &mut ilog[isti];
            } else {
                if ni == InsIdx::from(blks.borrow(bi).ins().len()) {
                    break;
                }
                i = blks.borrow(bi).ins()[ni.0 as usize];
                ni = ni.next();
                if isload(i.op) && i.args[1] != R {
                    // TODO same code in mem.rs
                    let ext: O =
                        O::from_repr((O::Oextsb as u8) + ((i.op as u8) - (O::Oloadsb as u8)))
                            .unwrap();
                    match i.op {
                        O::Oloadsb | O::Oloadub | O::Oloadsh | O::Oloaduh => {
                            i.op = ext;
                        }
                        O::Oloadsw | O::Oloaduw => {
                            if i.cls == Kl {
                                i.op = ext;
                            } else {
                                i.op = O::Ocopy;
                            }
                        }
                        O::Oload => {
                            i.op = O::Ocopy;
                        }
                        _ => {
                            // unreachable
                            assert!(false);
                            i.op = O::Oxxx;
                        }
                    }
                    i.args[0] = i.args[1];
                    i.args[1] = R;
                }
            }
            ib.push(i);
        }
        blks.borrow_mut(bi).ins = cell::RefCell::new(ib);
        n = n.next();
    }
    if true
    /*TODO debug['M']*/
    {
        /*e*/
        println!("\n> After load elimination:");
        printfn(/*stderr*/ &mut stdout(), f, typ, itbl);
    }
}
