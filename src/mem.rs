use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

use crate::alias::getalias;
use crate::all::{
    bit, isarg, isload, isret, isstore, kbase, to_s, Alias, AliasIdx, AliasT, AliasU, Bits, Blk,
    BlkIdx, Fn, Ins, InsIdx, KExt, Ref, RubeResult, Tmp, TmpIdx, Use, UseT, CON_Z, J, KL, KW, KX,
    NBIT, O, OALLOC, OALLOC1, TMP0, UNDEF,
};
use crate::cfg::loopiter;
use crate::load::{loadsz, storesz};
use crate::optab::OPTAB;

#[derive(Debug)]
struct MemError {
    msg: String,
}

impl MemError {
    fn new(msg: &str) -> MemError {
        MemError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for MemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for MemError {
    fn description(&self) -> &str {
        &self.msg
    }
}

pub fn promote(f: &mut Fn) -> RubeResult<()> {
    /* promote uniform stack slots to temporaries */
    let bi: BlkIdx = f.start;
    'ins_loop: for ii in 0..f.blk(bi).ins.len() {
        let (i_op, i_to) = {
            let i: &Ins = &f.blk(bi).ins[ii];
            (i.op, i.to)
        };
        if OALLOC > i_op || i_op > OALLOC1 {
            continue;
        }
        /* specific to NAlign == 3 */
        let ti = if let Ref::RTmp(ti0) = i_to {
            ti0
        } else {
            // MUST be an RTmp - TODO - not sure why?
            assert!(false);
            continue 'ins_loop;
        };

        if f.tmp(ti).ndef != 1 {
            continue 'ins_loop;
        }
        let mut k: KExt = KX;
        let mut s: i32 = -1; // TODO - what is this actually?

        for u in &f.tmp(ti).uses {
            if let UseT::UIns(li) = u.type_ {
                let l: &Ins = &f.blk(u.bi).ins[li.0 as usize];
                if isload(l.op) {
                    if s == -1 || s == loadsz(l) {
                        s = loadsz(l);
                        continue;
                    }
                } else if isstore(l.op) {
                    if (i_to == l.args[1] && i_to != l.args[0])
                        && (s == -1 || s == storesz(l))
                        && (k == KX || k == OPTAB[l.op as usize].argcls[0][0])
                    {
                        s = storesz(l);
                        k = OPTAB[l.op as usize].argcls[0][0];
                        continue;
                    }
                } else {
                    continue 'ins_loop;
                }
            } else {
                continue 'ins_loop;
            }
        }

        /* get rid of the alloc and replace uses */
        f.blk_mut(bi).ins[ii] = Ins::new0(O::Onop, KW, Ref::R);
        f.tmp_mut(ti).ndef -= 1;

        for ui in 0..f.tmp(ti).uses.len() {
            let (utype_, ubi) = {
                let u: &Use = &f.tmp(ti).uses[ui];
                (u.type_, u.bi)
            };
            let li = if let UseT::UIns(li0) = utype_ {
                li0
            } else {
                // Checked above that uses are only UIns
                assert!(false);
                continue;
            };
            let (l_op, l_cls, l_arg0, l_arg1) = {
                let l: &Ins = &f.blk(ubi).ins[li.0 as usize];
                (l.op, l.cls, l.args[0], l.args[1])
            };
            //         l = u->u.ins;
            if isstore(l_op) {
                f.blk_mut(ubi).ins[li.0 as usize] = Ins::new1(O::Ocopy, k, l_arg1, [l_arg0]);
                let t: &mut Tmp = f.tmp_mut(ti);
                //t.nuse -= 1; // Hrmmm... TODO
                t.ndef += 1;
            } else {
                // Skipped all instruction other than load/store above
                assert!(isload(l_op));

                if k == KX {
                    let t_name: &[u8] = {
                        if let Ref::RTmp(l_arg0_ti) = l_arg0 {
                            &f.tmp(l_arg0_ti).name
                        } else {
                            b"<unknown>"
                        }
                    };
                    return Err(Box::new(MemError::new(&format!(
                        "slot %{} is read but never stored to in function {}",
                        to_s(t_name),
                        to_s(&f.name)
                    ))));
                }

                let use_extend: bool = match l_op {
                    O::Oloadsw | O::Oloaduw => k == KL,
                    O::Oload => false,
                    _ => true,
                };
                f.blk_mut(ubi).ins[li.0 as usize].op = {
                    if use_extend {
                        O::from_repr((O::Oextsb as u8) + ((l_op as u8) - (O::Oloadsb as u8)))
                            .unwrap()
                    } else {
                        if kbase(k) == kbase(l_cls) {
                            O::Ocopy
                        } else {
                            O::Ocast
                        }
                    }
                };
            }
        }
    }
    // TODO:
    // if (debug['M']) {
    //     fprintf(stderr, "\n> After slot promotion:\n");
    //     printfn(fn, stderr);
    // }
    Ok(())
}

/* [a, b) with 0 <= a */
#[derive(Clone, Copy, Debug)]
struct Range {
    a: i32,
    b: i32,
}

#[derive(Clone, Debug)]
struct Store {
    ip: i32,
    bi: BlkIdx,
    ii: InsIdx,
}

#[derive(Clone, Debug)]
struct Slot {
    ti: TmpIdx,
    sz: i32,
    m: Bits,
    l: Bits,
    r: Range,
    si: SlotIdx,
    st: Vec<Store>,
    //nst: i32,
}

// Index into coalesce::sl vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SlotIdx(pub u32);

impl SlotIdx {
    pub const NONE: SlotIdx = SlotIdx(u32::MAX);
}

fn rin(r: &Range, n: i32) -> bool {
    r.a <= n && n < r.b
}

fn rovlap(r0: &Range, r1: &Range) -> bool {
    r0.b != 0 && r1.b != 0 && r0.a < r1.b && r1.a < r0.b
}

fn radd(r: &mut Range, n: i32) {
    if r.b == 0 {
        *r = Range { a: n, b: n + 1 };
    } else if n < r.a {
        r.a = n;
    } else if n >= r.b {
        r.b = n + 1;
    }
}

// Return maybe slot-idx, off
fn slot(f: &Fn, r: Ref) -> Option<(SlotIdx, i64)> {
    // Alias a;
    // Tmp *t;

    let a: Alias = getalias(f, &Alias::default(), r);
    if a.type_ != AliasT::ALoc {
        return None;
    }
    let ti: TmpIdx = a.base;
    let vi: TmpIdx = f.tmp(ti).visit; // TODO should be SlotIdx here
    if f.tmp(ti).visit == TmpIdx::NONE {
        return None;
    }

    Some((SlotIdx(vi.0), a.offset))
}

fn load(f: &Fn, r: Ref, x: Bits, ip: i32, sl: &mut [Slot]) {
    if let Some((si, off)) = slot(f, r) {
        let s: &mut Slot = &mut sl[si.0 as usize];
        s.l |= x << off;
        s.l &= s.m;
        if s.l != 0 {
            radd(&mut s.r, ip);
        }
    }
}

fn store(f: &Fn, r: Ref, x: Bits, ip: i32, bi: BlkIdx, ii: InsIdx, sl: &mut [Slot]) {
    if let Some((si, off)) = slot(f, r) {
        let s: &mut Slot = &mut sl[si.0 as usize];
        if s.l != 0 {
            radd(&mut s.r, ip);
            s.l &= !(x << off);
        } else {
            s.st.push(Store { ip, bi, ii });
        }
    }
}

fn scmp(a: &Slot, b: &Slot) -> Ordering {
    if a.sz != b.sz {
        return b.sz.cmp(&a.sz);
    }
    a.r.a.cmp(&b.r.a)
}

fn maxrpo(f: &mut Fn, hdi: BlkIdx, bi: BlkIdx) {
    let bid = f.blk(bi).id;
    let hd: &mut Blk = f.blk_mut(hdi);
    if (hd.loop_ as i32) < (bid as i32) {
        // Mmm maybe the cast is ^^^ to include hd.loop == u32::MAX?
        hd.loop_ = bid;
    }
}

pub fn coalesce(f: &mut Fn) {
    // Range r, *br;
    // Slot *s, *s0, *sl;
    // Blk *b, **ps, *succ[3];
    // Ins *i, **bl;
    // Use *u;
    // Tmp *t, *ts;
    // Ref *arg;
    // bits x;
    // int64_t off0, off1;
    // int n, m, ip, sz, nsl, nbl, *stk;
    // uint total, freed, fused;

    /* minimize the stack usage
     * by coalescing slots
     */
    // nsl = 0;
    let mut sl: Vec<Slot> = vec![];
    for n in TMP0..f.tmps.len() {
        let ti: TmpIdx = TmpIdx::new(n as usize);
        f.tmp_mut(ti).visit = TmpIdx::NONE; // Ugh, this is a slot index in sl here
        let ai: AliasIdx = f.tmp(ti).alias;
        let a: &Alias = f.alias(ai);
        if a.type_ == AliasT::ALoc && a.slot == ai && f.tmp(ti).bid == f.blk(f.start).id {
            if let AliasU::ALoc(aloc) = a.u {
                if aloc.sz != -1 {
                    f.tmp_mut(ti).visit = TmpIdx::new(sl.len()); // TODO - this is NOT a TmpIdx
                    sl.push(Slot {
                        ti,
                        sz: aloc.sz,
                        m: aloc.m,
                        l: 0,
                        r: Range { a: 0, b: 0 },
                        si: SlotIdx::NONE,
                        st: vec![],
                    });
                }
            }
        }
    }

    /* one-pass liveness analysis */
    {
        let mut bi: BlkIdx = f.start;
        while bi != BlkIdx::NONE {
            let b: &mut Blk = f.blk_mut(bi);
            b.loop_ = u32::MAX;
            bi = b.link;
        }
    }
    loopiter(f, maxrpo);
    let mut bl: Vec<(BlkIdx, InsIdx)> = vec![]; // Mmm
    {
        let mut br: Vec<Range> = vec![Range { a: 0, b: 0 }; f.nblk as usize];
        let mut ip: i32 = i32::MAX - 1; // ???
        for n in (0..f.nblk).rev() {
            let bi: BlkIdx = f.rpo[n as usize];
            let (s1, s2) = f.blk(bi).s1_s2();
            let succ: [BlkIdx; 3] = [s1, s2, BlkIdx::NONE];
            br[n as usize].b = ip;
            ip -= 1;
            for s in &mut sl {
                s.l = 0;
                for psi in succ {
                    if psi == BlkIdx::NONE {
                        break;
                    }
                    let m = f.blk(psi).id;
                    if m > n && rin(&s.r, br[m as usize].a) {
                        s.l = s.m;
                        radd(&mut s.r, ip);
                    }
                }
            }
            if f.blk(bi).jmp.type_ == J::Jretc {
                ip -= 1;
                load(f, f.blk(bi).jmp.arg, u64::MAX, ip, &mut sl);
            }
            for iii in (0..f.blk(bi).ins.len()).rev() {
                let i: Ins = f.blk(bi).ins[iii]; // note copy
                let ii: InsIdx = InsIdx(iii as u32);
                if i.op == O::Oargc {
                    ip -= 1;
                    load(f, i.args[1], u64::MAX, ip, &mut sl);
                }
                if isload(i.op) {
                    let x: Bits = bit(loadsz(&i) as u32) - 1;
                    ip -= 1;
                    load(f, i.args[0], x, ip, &mut sl);
                }
                if isstore(i.op) {
                    let x: Bits = bit(storesz(&i) as u32) - 1;
                    store(f, i.args[1], x, ip, bi, ii, &mut sl);
                    ip -= 1;
                }
                if i.op == O::Oblit0 {
                    assert!(f.blk(bi).ins[iii + 1].op == O::Oblit1); // TODO bounds check
                    if let Ref::RInt(rsval) = f.blk(bi).ins[iii + 1].args[0] {
                        let sz: i32 = rsval.abs();
                        let x: Bits = if sz as u32 >= NBIT {
                            u64::MAX
                        } else {
                            bit(sz as u32) - 1
                        };
                        store(f, i.args[1], x, ip, bi, ii, &mut sl);
                        ip -= 1;
                        load(f, i.args[0], x, ip, &mut sl);
                        bl.push((bi, ii));
                    } else {
                        // Oblit1 arg0 MUST be an RInt
                        assert!(false);
                    }
                }
            }
            let bloop = f.blk(bi).loop_;
            for s in &mut sl {
                if s.l != 0 {
                    radd(&mut s.r, ip);
                    if bloop != u32::MAX {
                        assert!(bloop > n);
                        radd(&mut s.r, br[bloop as usize].b - 1);
                    }
                }
            }
            br[n as usize].a = ip;
        }
    }

    /* kill dead stores */
    for s in &mut sl {
        for n in 0..s.st.len() {
            if !rin(&s.r, s.st[n].ip) {
                let bi: BlkIdx = s.st[n].bi;
                let ii: InsIdx = s.st[n].ii;
                if f.blk(bi).ins[ii.0 as usize].op == O::Oblit0 {
                    f.blk_mut(bi).ins[(ii.0 as usize) + 1] = Ins::new0(O::Onop, KX, Ref::R);
                }
                f.blk_mut(bi).ins[ii.0 as usize] = Ins::new0(O::Onop, KX, Ref::R);
            }
        }
    }

    /* kill slots with an empty live range */
    let mut total: i32 = 0;
    let mut freed: i32 = 0;
    let mut stk: Vec<TmpIdx> = vec![];
    //let n: usize = 0;
    let mut s0: usize = 0;
    // TODO - use retain_mut()
    for s in 0..sl.len() {
        total += sl[s].sz;
        if sl[s].r.b == 0 {
            stk.push(sl[s].ti);
            freed += sl[s].sz;
        } else {
            sl[s0] = sl[s].clone(); // Ugh, cloning for vec st
            s0 += 1;
        }
    }

    // TODO
    if true
    /*debug['M']*/
    {
        /*e*/
        println!("\n> Slot coalescing:");
        if !stk.is_empty() {
            /*e*/
            print!("\tkill [");
            for ti in &stk {
                /*e*/
                print!(" %{}", to_s(&f.tmp(*ti).name));
            }
            /*e*/
            println!(" ]");
        }
    }

    loop {
        match stk.pop() {
            None => break,
            Some(ti) => {
                let (t_def_ii, t_def_bi) = {
                    let t: &Tmp = f.tmp(ti);
                    assert!(t.ndef == 1 && t.def != InsIdx::NONE);
                    (t.def, f.rpo[t.bid as usize])
                };
                let i: Ins = f.blk(t_def_bi).ins[t_def_ii.0 as usize]; /* Note - copy */
                if isload(i.op) {
                    f.blk_mut(t_def_bi).ins[t_def_ii.0 as usize] =
                        Ins::new1(O::Ocopy, i.cls, i.to, [UNDEF]);
                    continue;
                }
                f.blk_mut(t_def_bi).ins[t_def_ii.0 as usize] = Ins::new0(O::Onop, KX, Ref::R);
                for ui in 0..f.tmp(ti).uses.len() {
                    let u: Use = f.tmp(ti).uses[ui]; // Note - copy
                    match u.type_ {
                        UseT::UJmp => {
                            let bi: BlkIdx = f.rpo[u.bid as usize];
                            let b: &mut Blk = f.blk_mut(bi);
                            assert!(isret(b.jmp.type_));
                            b.jmp.type_ = J::Jret0;
                            b.jmp.arg = Ref::R;
                        }
                        UseT::UIns(ii) => {
                            let bi: BlkIdx = f.rpo[u.bid as usize];
                            let b: &mut Blk = f.blk_mut(bi);
                            let i: Ins = b.ins[ii.0 as usize]; // Note - copy
                            match i.to {
                                Ref::R => {
                                    if isarg(i.op) {
                                        assert!(i.op == O::Oargc);
                                        b.ins[ii.0 as usize].args[1] = CON_Z; /* crash */
                                    } else {
                                        if i.op == O::Oblit0 {
                                            b.ins[(ii.0 + 1) as usize] =
                                                Ins::new0(O::Onop, KX, Ref::R);
                                        }
                                        b.ins[ii.0 as usize] = Ins::new0(O::Onop, KX, Ref::R);
                                    }
                                }
                                Ref::RTmp(ti) => {
                                    stk.push(ti);
                                }
                                _ => {
                                    assert!(false);
                                }
                            }
                        }
                        _ => {
                            assert!(false);
                        }
                    }
                }
            }
        }
    }

    // /* fuse slots by decreasing size */
    sl.sort_by(scmp);
    // qsort(sl, nsl, sizeof *sl, scmp);
    let mut fused: i32 = 0;
    'outer: for s0i in 0..sl.len() {
        if sl[s0i].si != SlotIdx::NONE {
            continue;
        }
        sl[s0i].si = SlotIdx(s0i as u32);
        let mut r: Range = sl[s0i].r;
        for si in (s0i + 1)..sl.len() {
            if sl[si].si != SlotIdx::NONE || sl[si].r.b == 0 {
                continue 'outer;
            }
            if rovlap(&r, &sl[si].r) {
                /* O(n); can be approximated
                 * by 'goto Skip;' if need be
                 */
                for mi in s0i..si {
                    if sl[mi].si == SlotIdx(s0i as u32) && rovlap(&sl[mi].r, &sl[si].r) {
                        continue 'outer;
                    }
                }
            }
            radd(&mut r, sl[si].r.a);
            radd(&mut r, sl[si].r.b - 1);
            sl[si].si = SlotIdx(s0i as u32);
            fused += sl[si].sz;
        }
    }

    // /* substitute fused slots */
    for si in 0..sl.len() {
        // for (s=sl; s<&sl[nsl]; s++) {
        let sti: TmpIdx = sl[si].ti;
        let (t_def_ii, t_bid) = {
            let t: &mut Tmp = f.tmp_mut(sti);
            /* the visit link is stale,
             * reset it before the slot()
             * calls below
             */
            t.visit = TmpIdx::new(si); // Not actually a TmpIdx here :(
            assert!(t.ndef == 1 && t.def != InsIdx::NONE);
            (t.def, t.bid)
        };
        let t_def_bi: BlkIdx = f.rpo[t_bid as usize];
        if sl[si].si == SlotIdx(si as u32) {
            continue;
        }
        f.blk_mut(t_def_bi).ins[t_def_ii.0 as usize] = Ins::new0(O::Onop, KX, Ref::R);
        let ssi: SlotIdx = sl[si].si;
        let ssti: TmpIdx = sl[ssi.0 as usize].ti;
        let (ts_def_ii, ts_bid) = {
            let ts: &Tmp = f.tmp(ssti);
            (ts.def, ts.bid)
        };
        assert!(t_bid == ts_bid);
        if t_def_ii < ts_def_ii {
            /* make sure the slot we
             * selected has a def that
             * dominates its new uses
             */
            let tsi: Ins = f.blk(t_def_bi).ins[ts_def_ii.0 as usize]; // Note copy
            f.blk_mut(t_def_bi).ins[t_def_ii.0 as usize] = tsi;
            f.blk_mut(t_def_bi).ins[ts_def_ii.0 as usize] = Ins::new0(O::Onop, KX, Ref::R);
            f.tmp_mut(ssti).def = t_def_ii;
        }
        for ui in 0..f.tmp(sti).uses.len() {
            let u: Use = f.tmp(sti).uses[ui]; // Note - copy
            match u.type_ {
                UseT::UJmp => {
                    let bi: BlkIdx = f.rpo[u.bid as usize];
                    f.blk_mut(bi).jmp.arg = Ref::RTmp(ssti);
                }
                UseT::UIns(ii) => {
                    let bi: BlkIdx = f.rpo[u.bid as usize];
                    let args: &mut [Ref; 2] = &mut f.blk_mut(bi).ins[ii.0 as usize].args;
                    for arg in args {
                        //         for (n=0; n<2; n++)
                        if *arg == Ref::RTmp(sti) {
                            *arg = Ref::RTmp(ssti);
                        }
                    }
                }
                _ => {
                    assert!(false);
                }
            }
        }
    }

    // /* fix newly overlapping blits */
    // for (n=0; n<nbl; n++) {
    //     i = bl[n];
    //     if (i.op == Oblit0)
    //     if (slot(&s, &off0, i.arg[0], f, sl))
    //     if (slot(&s0, &off1, i.arg[1], f, sl))
    //     if (s.s == s0.s) {
    //         if (off0 < off1) {
    //             sz = rsval((i+1).arg[0]);
    //             assert(sz >= 0);
    //             (i+1).arg[0] = INT(-sz);
    //         } else if (off0 == off1) {
    //             *i = (Ins){.op = Onop};
    //             *(i+1) = (Ins){.op = Onop};
    //         }
    //     }
    // }
    // vfree(bl);

    // if (debug['M']) {
    //     for (s0=sl; s0<&sl[nsl]; s0++) {
    //         if (s0.s != s0)
    //             continue;
    //         fprintf(stderr, "\tfuse (% 3db) [", s0.sz);
    //         for (s=s0; s<&sl[nsl]; s++) {
    //             if (s.s != s0)
    //                 continue;
    //             fprintf(stderr, " %%%s", f.tmp[s.t].name);
    //             if (s.r.b)
    //                 fprintf(stderr, "[%d,%d)",
    //                     s.r.a-ip, s.r.b-ip);
    //             else
    //                 fputs("{}", stderr);
    //         }
    //         fputs(" ]\n", stderr);
    //     }
    //     fprintf(stderr, "\tsums %u/%u/%u (killed/fused/total)\n\n",
    //         freed, fused, total);
    //     printfn(f, stderr);
    // }

    // for (s=sl; s<&sl[nsl]; s++)
    //     vfree(s.st);
    // vfree(sl);
}
