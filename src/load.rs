use derive_new::new;
use std::cmp::Ordering;

use crate::alias::{alias, escapes};
use crate::all::{
    bit, isload, isstore, kwide, Alias, AliasIdx, AliasT, AliasU, Bits, BlkIdx, CanAlias, Con, Fn,
    Ins, InsIdx, KExt, Phi, PhiIdx, Ref, TmpIdx, KD, KL, KS, KW, KX, O,
};
use crate::cfg::dom;
use crate::util::{getcon, newcon, newtmp, newtmpref};

// TODO remove
use crate::all::{to_s, Typ};
use crate::optab::OPTAB;
use crate::parse::printref;
use crate::util::Bucket;
use std::io::stdout;

/*
#include "all.h"
 */

// TODO i32 is dodgy
fn genmask(w: i32) -> Bits {
    assert!(0 <= w && w <= 8);
    bit((8 * w - 1) as u32).wrapping_mul(2).wrapping_sub(1) /* must work when w==8 */
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
    type_: LocT,
    off: InsIdx,
    bi: BlkIdx,
}

#[derive(Clone, Copy, Debug)]
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

fn load(f: &mut Fn, ilog: &mut Vec<Insert>, sl: &Slice, msk: Bits, l: &Loc) -> Ref {
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
    let cls: KExt = if all {
        sl.cls
    } else {
        if sl.sz > 4 {
            KL
        } else {
            KW
        }
    };
    let mut r: Ref = sl.r;
    /* sl.ref might not be live here,
     * but its alias base ref will be
     * (see killsl() below) */
    if let Ref::RTmp(ti) = r {
        let ai = f.tmp(ti).alias;
        let a: Alias = *f.alias(ai); // Note - copy!
        match a.type_ {
            AliasT::ALoc | AliasT::AEsc | AliasT::AUnk => {
                r = Ref::RTmp(a.base);
                if a.offset != 0 {
                    let r1: Ref = getcon(f, a.offset);
                    r = iins(f, ilog, KL, O::Oadd, r, r1, l);
                }
            }
            AliasT::ACon | AliasT::ASym => {
                if let AliasU::ASym(sym) = a.u {
                    r = newcon(f, Con::new_sym(sym, crate::all::ConBits::I(a.offset)));
                } else {
                    assert!(false);
                    r = Ref::R; // Ugh, TODO
                }
            }
            _ => {
                // unreachable
                assert!(false);
                r = Ref::R;
            }
        }
    }
    r = iins(f, ilog, cls, ld, r, Ref::R, l);
    if !all {
        mask(f, ilog, cls, &mut r, msk, l);
    }
    r
}

fn killsl(f: &Fn, r: Ref, sl: &Slice) -> bool {
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

fn prindent(indent: usize) {
    for n in 0..indent {
        print!("  ");
    }
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
    indent: usize,
    debug: bool,
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

    if debug {
        prindent(indent);
        println!(
            "                         def - for @{} ins {}",
            to_s(&f.blk(bi).name),
            ii.0
        );
    }

    if indent > 32 {
        panic!("Gone too deep");
    }

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

    if ii == InsIdx::NONE {
        // Bit naughty - this is out of range
        ii = InsIdx(f.blk(bi).ins.len() as u32);
    }
    let cls: KExt = if sl.sz > 4 { KL } else { KW };
    let msks: Bits = genmask(sl.sz as i32);

    let mut goto_load: bool = false;
    while ii != InsIdx(0) && !goto_load {
        ii = InsIdx(ii.0 - 1);
        if debug {
            prindent(indent);
            println!(
                "                         def -    looking at @{} ins {}",
                to_s(&f.blk(bi).name),
                ii.0
            );
        }
        let mut i: Ins = f.blk(bi).ins[ii.0 as usize]; /* Note: copy! */
        if killsl(f, i.to, &sl) || (i.op == O::Ocall && escapes(f, sl.r)) {
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
                if debug {
                    prindent(indent);
                    println!("                                     MUST alias");
                }
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
                    r = def(f, ilog, &sl1, genmask(sz), bi, ii, il, indent + 1, debug);
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
                    r1 = def(f, ilog, sl, msk & !msk1, bi, ii, il, indent + 1, debug);
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
                if debug {
                    prindent(indent);
                    println!("                                     may alias");
                }
                if !ld {
                    // println!("                                         ... and not a load");
                    goto_load = true;
                }
                continue;
            }
            CanAlias::No => {
                if debug {
                    prindent(indent);
                    println!("                                     no alias");
                }
                continue;
            }
        }
    }

    // if goto_load {
    //     f.tmps.truncate(oldt);
    //     ilog.truncate(oldl);
    //     if il.type_ != LocT::LLoad {
    //         return Ref::R;
    //     }
    //     return load(f, ilog, sl, msk, il);
    // }

    if !goto_load {
        if debug {
            prindent(indent);
            println!(
                "                         def - got through preceding instructions of @{}",
                to_s(&f.blk(bi).name)
            );
        }
    }

    if !goto_load {
        let bid = f.blk(bi).id;

        for isti in 0..ilog.len() {
            if !goto_load {
                if debug {
                    prindent(indent);
                    println!(
                        "                                         checking Insert {}\n",
                        isti
                    );
                }
            }
            let ist: &Insert = &ilog[isti];
            if let InsertU::Phi(uphi) = &ist.new {
                if ist.bid == bid && uphi.m.r == sl.r && uphi.m.off == sl.off && uphi.m.sz == sl.sz
                {
                    let mut r = f.phi(uphi.pi).to;
                    if msk != msks {
                        mask(f, ilog, cls, &mut r, msk, il);
                    } else {
                        cast(f, ilog, &mut r, sl.cls, il);
                    }
                    return r;
                }
            }
        }

        let mut pi = f.blk(bi).phi;
        while pi != PhiIdx::NONE {
            let p_to: Ref = f.phi(pi).to;
            if killsl(f, p_to, &sl) {
                /* scanning predecessors in that
                 * case would be unsafe */
                goto_load = true;
                break;
            }
            pi = f.phi(pi).link;
        }
    }

    if !goto_load {
        if f.blk(bi).preds.is_empty() {
            goto_load = true;
        }
    }

    if !goto_load {
        if f.blk(bi).preds.len() == 1 {
            let bpi = f.blk(bi).preds[0];
            assert!(f.blk(bpi).loop_ >= f.blk(il.bi).loop_);
            let mut l: Loc = *il;
            if f.blk(bpi).s2 != BlkIdx::NONE {
                l.type_ = LocT::LNoLoad;
            }
            let r1: Ref = def(f, ilog, &sl, msk, bpi, InsIdx::NONE, &l, indent + 1, debug);
            if r1 == Ref::R {
                goto_load = true;
            } else {
                return r1;
            }
        }
    }

    let mut r: Ref = Ref::R;
    if !goto_load {
        r = newtmpref(b"ld", true, sl.cls, f); // TODO - this needs to be outside the if
                                               // p = alloc(sizeof *p);
                                               // vgrow(&ilog, ++nlog);
                                               // ist = &ilog[nlog-1];
                                               // ist.isphi = 1;
                                               // ist.bid = b.id;
                                               // ist.new.phi.m = sl;
                                               // ist.new.phi.p = p;
        let p: Phi = Phi::new(r, vec![], vec![], sl.cls, PhiIdx::NONE);
        let pi: PhiIdx = f.add_phi(p);
        // TODO - notify QBE? QBE doesn't seem to set ist.num (i.e. ti). Nor off
        // I suspect to should be r's ti, not 0???
        // Maybe for phi's, QBE gets "to" from UPhi(p.to)
        ilog.push(Insert::new(
            TmpIdx(0), /*TODO*/
            f.blk(bi).id,
            InsIdx(0),
            InsertU::Phi(UPhi { m: *sl, pi }),
        ));
        for np in 0..f.blk(bi).preds.len() {
            let bpi: BlkIdx = f.blk(bi).preds[np];
            let l_type: LocT;
            if f.blk(bpi).s2 == BlkIdx::NONE
                && il.type_ != LocT::LNoLoad
                && f.blk(bpi).loop_ < f.blk(il.bi).loop_
            {
                l_type = LocT::LLoad;
            } else {
                l_type = LocT::LNoLoad;
            }
            let l: Loc = Loc {
                type_: l_type,
                bi: bpi,
                off: InsIdx(f.blk(bpi).ins.len() as u32),
            };
            let r1: Ref = def(f, ilog, &sl, msks, bpi, InsIdx::NONE, &l, indent + 1, debug);
            if r1 == Ref::R {
                goto_load = true;
                break;
            }
            f.phi_mut(pi).args.push(r1);
            f.phi_mut(pi).blks.push(bpi);
        }
    }

    if goto_load {
        f.tmps.truncate(oldt);
        ilog.truncate(oldl);
        if il.type_ != LocT::LLoad {
            return Ref::R;
        }
        load(f, ilog, sl, msk, il)
    } else {
        if msk != msks {
            mask(f, ilog, cls, &mut r, msk, il);
        }
        r
    }
}

fn icmp(a: &Insert, b: &Insert) -> Ordering {
    let bid_cmp = a.bid.cmp(&b.bid);
    if bid_cmp != Ordering::Equal {
        return bid_cmp;
    }
    let a_isphi: bool = if let InsertU::Phi(_) = a.new {
        true
    } else {
        false
    };
    let b_isphi: bool = if let InsertU::Phi(_) = b.new {
        true
    } else {
        false
    };
    if a_isphi && b_isphi {
        return Ordering::Equal;
    }
    if a_isphi {
        return Ordering::Less;
    }
    if b_isphi {
        return Ordering::Greater;
    }
    let off_cmp = a.off.0.cmp(&b.off.0);
    if off_cmp != Ordering::Equal {
        return off_cmp;
    }
    a.num.0.cmp(&b.num.0)
}

/* require rpo ssa alias */
// TODO remove type, itbl - just for debug
pub fn loadopt(f: &mut Fn, typ: &[Typ], itbl: &[Bucket]) {
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
            println!(
                "                     loadopt: bi {} bid {} @{} ins {} {}",
                bi.0,
                f.blk(bi).id,
                to_s(&f.blk(bi).name),
                iii,
                to_s(OPTAB[f.blk(bi).ins[iii].op as usize].name)
            );
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
                let debug: bool = bi.0 == 183 && iii == 1;
                def(f, &mut ilog, &sl, genmask(sz), bi, ii, &l, 0, debug)
            };
            f.blk_mut(bi).ins[iii].args[1] = i_arg1;
            // print!(
            //     "                     loadopt: @{} ins {} {} - arg1 is now ",
            //     to_s(&f.blk(bi).name),
            //     iii,
            //     to_s(OPTAB[f.blk(bi).ins[iii].op as usize].name)
            // );
            // if i_arg1 == Ref::R {
            //     print!("R");
            // } else {
            //     printref(&mut stdout(), f, typ, itbl, &i_arg1);
            // }
            // println!();
        }
        bi = f.blk(bi).link;
    }
    ilog.sort_by(icmp);
    let sentinal_ins = Ins::new0(O::Oxxx, KX, Ref::R);
    /* add a sentinel */
    ilog.push(Insert::new(
        TmpIdx::NONE,
        f.nblk,
        InsIdx::NONE,
        InsertU::Ins(sentinal_ins),
    ));
    let mut isti: usize = 0;
    let mut n: u32 = 0;
    while n < f.nblk {
        let mut ist: &mut Insert = &mut ilog[isti];
        let bi: BlkIdx = f.rpo[n as usize];
        while ist.bid == n {
            if let InsertU::Phi(uphi) = &mut ist.new {
                f.phi_mut(uphi.pi).link = f.blk(bi).phi;
                f.blk_mut(bi).phi = uphi.pi;
            } else {
                break;
            }
            isti += 1;
            ist = &mut ilog[isti];
        }
        let mut ni: InsIdx = InsIdx(0);
        // nt = 0; ??? what's this
        let mut ib: Vec<Ins> = vec![];
        loop {
            let mut i: Ins;
            if ist.bid == n && ist.off == ni {
                if let InsertU::Ins(i0) = &ist.new {
                    i = *i0; // Copy
                } else {
                    // MUST be InsertU::Ins
                    assert!(false);
                    i = Ins::new0(O::Oxxx, KX, Ref::R);
                }
                isti += 1;
                ist = &mut ilog[isti];
            } else {
                if ni == InsIdx(f.blk(bi).ins.len() as u32) {
                    break;
                }
                i = f.blk(bi).ins[ni.0 as usize];
                ni = InsIdx(ni.0 + 1);
                if isload(i.op) && i.args[1] != Ref::R {
                    // TODO same code in mem.rs
                    let ext: O =
                        O::from_repr((O::Oextsb as u8) + ((i.op as u8) - (O::Oloadsb as u8)))
                            .unwrap();
                    match i.op {
                        O::Oloadsb | O::Oloadub | O::Oloadsh | O::Oloaduh => {
                            i.op = ext;
                        }
                        O::Oloadsw | O::Oloaduw => {
                            if i.cls == KL {
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
                    i.args[1] = Ref::R;
                }
            }
            ib.push(i);
        }
        f.blk_mut(bi).ins = ib;
        n += 1;
    }
    // TODO
    // if (debug['M']) {
    //     fprintf(stderr, "\n> After load elimination:\n");
    //     printfn(fn, stderr);
    // }
}
