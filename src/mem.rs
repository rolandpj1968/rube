use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::io::stdout;
use std::ops::{Index, IndexMut};

use crate::alias::getalias;
use crate::all::Ref::{RInt, RTmp, R};
use crate::all::K::{Kl, Kx};
use crate::all::{
    bit, isarg, isload, isret, isstore, kbase, to_s, Alias, AliasT, AliasU, Bits, BlkIdx, Blks,
    Con, Fn, Idx, Ins, InsIdx, Ref, RubeResult, Tmp, TmpIdx, Typ, Use, UseT, CON_Z, J, K, NBIT, O,
    OALLOC, OALLOC1, TMP0, UNDEF,
};
use crate::cfg::loopiter;
use crate::load::{loadsz, storesz};
use crate::optab::OPTAB;
use crate::parse::printfn;
use crate::util::Bucket;

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

pub fn promote(f: &mut Fn, typ: &[Typ], itbl: &[Bucket]) -> RubeResult<()> {
    let blks = &f.blks;
    let tmps: &mut [Tmp] = &mut f.tmps;

    /* promote uniform stack slots to temporaries */
    let bi: BlkIdx = f.start;
    let ins_len = blks.borrow(bi).ins().len();
    'ins_loop: for ii in 0..ins_len {
        let t: &mut Tmp;
        let mut k: K = Kx;
        let mut s: i32 = -1; // TODO sz types in general :(
        {
            let b = blks.borrow(bi);
            let i: &Ins = &b.ins()[ii];
            // TODO !isalloc
            if OALLOC > i.op || i.op > OALLOC1 {
                continue;
            }
            /* specific to NAlign == 3 */
            /* TODO - what does this comment ^^^ mean */
            assert!(matches!(i.to, RTmp(_)));
            if let RTmp(ti) = i.to {
                t = &mut tmps[ti];
            } else {
                continue;
            }
            if t.ndef != 1 {
                continue;
            }

            for u in &t.uses {
                if let UseT::UIns(li) = u.typ {
                    let ub = blks.borrow(u.bi);
                    let l: &Ins = &ub.ins()[li];
                    if isload(l.op) {
                        if s == -1 || s == loadsz(l) {
                            s = loadsz(l);
                            continue;
                        }
                    } else if isstore(l.op) {
                        if (i.to == l.args[1] && i.to != l.args[0])
                            && (s == -1 || s == storesz(l))
                            && (k == Kx || k == OPTAB[l.op as usize].argcls[0][0])
                        {
                            s = storesz(l);
                            k = OPTAB[l.op as usize].argcls[0][0];
                            continue;
                        }
                    }
                }
                continue 'ins_loop;
            }
        }

        /* get rid of the alloc and replace uses */
        blks.borrow_mut(bi).ins_mut()[ii] = Ins::NOP;
        t.ndef -= 1;

        for u in &t.uses {
            let ub = blks.borrow_mut(u.bi);
            let mut ub_ins = ub.ins_mut();
            let l: &mut Ins = {
                assert!(matches!(u.typ, UseT::UIns(_)));
                if let UseT::UIns(li) = u.typ {
                    &mut ub_ins/*ub.ins_mut()*/[li]
                } else {
                    continue;
                }
            };
            if isstore(l.op) {
                *l = Ins::new1(O::Ocopy, k, l.args[1], [l.args[0]]);
                //t.nuse -= 1; // Hrmmm... TODO TODO TODO; this seems dodge cos it's not the last use
                t.ndef += 1;
            } else {
                assert!(isload(l.op));

                if k == Kx {
                    let t_name: &[u8] = {
                        if let RTmp(l_arg0_ti) = l.args[0] {
                            &tmps[l_arg0_ti].name
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

                let use_extend: bool = match l.op {
                    O::Oloadsw | O::Oloaduw => k == Kl,
                    O::Oload => false,
                    _ => true,
                };
                l.op = {
                    if use_extend {
                        O::from_repr((O::Oextsb as u8) + ((l.op as u8) - (O::Oloadsb as u8)))
                            .unwrap()
                    } else {
                        if kbase(k) == kbase(l.cls) {
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
    if true
    /*debug['M']*/
    {
        /*e*/
        println!("\n> After slot promotion:");
        printfn(/*stderr*/ &mut stdout(), f, typ, itbl);
    }
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SlotTag();
// Index into coalesce::sl vector
pub type SlotIdx = Idx<SlotTag>;

impl Index<SlotIdx> for [Slot] {
    type Output = Slot;
    fn index(&self, index: SlotIdx) -> &Self::Output {
        debug_assert!(index != SlotIdx::NONE);
        self.index(index.0 as usize)
    }
}

impl Index<SlotIdx> for Vec<Slot> {
    type Output = Slot;
    fn index(&self, index: SlotIdx) -> &Self::Output {
        debug_assert!(index != SlotIdx::NONE);
        self.index(index.0 as usize)
    }
}

impl IndexMut<SlotIdx> for [Slot] {
    fn index_mut(&mut self, index: SlotIdx) -> &mut Self::Output {
        debug_assert!(index != SlotIdx::NONE);
        self.index_mut(index.0 as usize)
    }
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
fn slot(tmps: &[Tmp], cons: &[Con], r: Ref) -> Option<(SlotIdx, i64)> {
    let a: Alias = getalias(tmps, cons, &Alias::default(), r);
    if a.typ != AliasT::ALoc {
        return None;
    }
    let tvsi: SlotIdx = tmps[a.base].svisit;
    if tvsi == SlotIdx::NONE {
        return None;
    }
    Some((tvsi, a.offset))
}

fn load(tmps: &[Tmp], cons: &[Con], r: Ref, x: Bits, ip: i32, sl: &mut [Slot]) {
    if let Some((si, off)) = slot(tmps, cons, r) {
        let s: &mut Slot = &mut sl[si];
        s.l |= x << off;
        s.l &= s.m;
        if s.l != 0 {
            radd(&mut s.r, ip);
        }
    }
}

fn store(
    tmps: &[Tmp],
    cons: &[Con],
    r: Ref,
    x: Bits,
    ip: i32,
    bi: BlkIdx,
    ii: InsIdx,
    sl: &mut [Slot],
) {
    if let Some((si, off)) = slot(tmps, cons, r) {
        let s: &mut Slot = &mut sl[si];
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

fn maxrpo(blks: &Blks, hdi: BlkIdx, bi: BlkIdx) {
    let b_id = blks.id_of(bi);
    blks.with_mut(hdi, |hd| {
        // Ugh, fixme
        if hd.loop_ < (b_id.usize() as i32) {
            hd.loop_ = b_id.usize() as i32;
        }
    });
}

pub fn coalesce(f: &mut Fn, typ: &[Typ], itbl: &[Bucket]) {
    let blks: &Blks = &f.blks;
    let rpo: &[BlkIdx] = &f.rpo;
    let nblk = rpo.len();
    assert!(nblk == f.nblk as usize);
    let cons: &[Con] = &f.cons;

    let mut total: i32 = 0;
    let mut freed: i32 = 0;
    let mut fused: i32 = 0;

    /* minimize the stack usage
     * by coalescing slots
     */
    // nsl = 0;
    let mut sl: Vec<Slot> = vec![];
    {
        let tmps: &mut [Tmp] = &mut f.tmps;
        let start_id = blks.borrow(f.start).id;
        for n in TMP0..tmps.len() {
            let ti: TmpIdx = TmpIdx::new(n);
            let t: &mut Tmp = &mut tmps[ti];
            t.svisit = SlotIdx::NONE;
            if t.alias.typ == AliasT::ALoc && t.alias.slot == ti && t.bid == start_id {
                if let AliasU::ALoc(aloc) = t.alias.u {
                    if aloc.sz != -1 {
                        t.svisit = SlotIdx::new(sl.len());
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
    }

    let mut bl: Vec<(BlkIdx, InsIdx)> = vec![]; // Mmm
    let mut ip: i32 = i32::MAX - 1; // ???
    {
        let tmps: &[Tmp] = &f.tmps;
        /* one-pass liveness analysis */
        blks.for_each_mut(|b| b.loop_ = -1);
        loopiter(blks, rpo, maxrpo);
        {
            let mut br: Vec<Range> = vec![Range { a: 0, b: 0 }; nblk];
            for n in (0..nblk).rev() {
                let bi: BlkIdx = rpo[n];
                blks.with(bi, |b| {
                    // TODO - this dedups s1, s2 - does it matter?
                    let succs = b.succs();
                    br[n].b = ip;
                    ip -= 1;
                    for s in &mut sl {
                        s.l = 0;
                        for psi in succs {
                            if psi == BlkIdx::NONE {
                                break;
                            }
                            let m = blks.id_of(psi);
                            if (m.usize()) > n && rin(&s.r, br[m.usize()].a) {
                                s.l = s.m;
                                radd(&mut s.r, ip);
                            }
                        }
                    }
                    if b.jmp().typ == J::Jretc {
                        ip -= 1;
                        load(tmps, cons, b.jmp().arg, u64::MAX, ip, &mut sl);
                    }
                    for iii in (0..b.ins().len()).rev() {
                        let i: &Ins = &b.ins()[iii];
                        let ii: InsIdx = InsIdx::new(iii);
                        if i.op == O::Oargc {
                            ip -= 1;
                            load(tmps, cons, i.args[1], u64::MAX, ip, &mut sl);
                        }
                        if isload(i.op) {
                            let x: Bits = bit(loadsz(i) as usize) - 1;
                            ip -= 1;
                            load(tmps, cons, i.args[0], x, ip, &mut sl);
                        }
                        if isstore(i.op) {
                            let x: Bits = bit(storesz(i) as usize) - 1;
                            store(tmps, cons, i.args[1], x, ip, bi, ii, &mut sl);
                            ip -= 1;
                        }
                        if i.op == O::Oblit0 {
                            assert!(iii + 1 < b.ins().len());
                            let blit1: &Ins = &b.ins()[iii + 1];
                            assert!(blit1.op == O::Oblit1); // TODO bounds check
                            assert!(matches!(blit1.args[0], RInt(_)));
                            if let RInt(rsval) = blit1.args[0] {
                                let sz: i32 = rsval.abs();
                                let x: Bits = if sz >= (NBIT as i32) {
                                    u64::MAX
                                } else {
                                    bit(sz as usize) - 1
                                };
                                store(tmps, cons, i.args[1], x, ip, bi, ii, &mut sl);
                                ip -= 1;
                                load(tmps, cons, i.args[0], x, ip, &mut sl);
                                bl.push((bi, ii));
                            }
                        }
                    }
                    for s in &mut sl {
                        if s.l != 0 {
                            radd(&mut s.r, ip);
                            if b.loop_ != -1 {
                                assert!(b.loop_ > n as i32);
                                radd(&mut s.r, br[b.loop_ as usize].b - 1);
                            }
                        }
                    }
                    br[n].a = ip;
                });
            }
        }

        /* kill dead stores */
        for s in &mut sl {
            for n in 0..s.st.len() {
                if !rin(&s.r, s.st[n].ip) {
                    blks.with_mut(s.st[n].bi, |b| {
                        let ii: InsIdx = s.st[n].ii;
                        if b.ins()[ii].op == O::Oblit0 {
                            b.ins_mut()[(ii.0 as usize) + 1] = Ins::NOP;
                        }
                        b.ins_mut()[ii] = Ins::NOP;
                    });
                }
            }
        }

        /* kill slots with an empty live range */
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
                    print!(" %{}", to_s(&tmps[*ti].name));
                }
                /*e*/
                println!(" ]");
            }
        }

        loop {
            match stk.pop() {
                None => break,
                Some(ti) => {
                    let t: &Tmp = &tmps[ti];
                    assert!(t.ndef == 1 && t.def != InsIdx::NONE);
                    let def_bi = f.rpo[t.bid];
                    let mut is_load = false;
                    blks.with_mut(def_bi, |b| {
                        let i: &mut Ins = &mut b.ins_mut()[t.def];
                        if isload(i.op) {
                            *i = Ins::new1(O::Ocopy, i.cls, i.to, [UNDEF]);
                            is_load = true;
                        } else {
                            *i = Ins::NOP;
                        }
                    });
                    if is_load {
                        continue;
                    }
                    for u in &t.uses {
                        assert!(!matches!(u.typ, UseT::UPhi(_)));
                        let bi: BlkIdx = rpo[u.bid];
                        blks.with_mut(bi, |b| {
                            match u.typ {
                                UseT::UJmp => {
                                    assert!(isret(b.jmp().typ));
                                    b.jmp_mut().typ = J::Jret0;
                                    b.jmp_mut().arg = R;
                                }
                                UseT::UIns(ii) => {
                                    let i: &mut Ins = &mut b.ins_mut()[ii];
                                    assert!(i.to == R || matches!(i.to, RTmp(_)));
                                    match i.to {
                                        R => {
                                            if isarg(i.op) {
                                                assert!(i.op == O::Oargc);
                                                i.args[1] = CON_Z; // crash
                                            } else {
                                                if i.op == O::Oblit0 {
                                                    // This is not gonna work :(
                                                    //panic!("fixme");
                                                    b.ins_mut()[ii.next()] = Ins::NOP;
                                                }
                                                *i = Ins::NOP;
                                            }
                                        }
                                        RTmp(ti) => {
                                            stk.push(ti);
                                        }
                                        _ => (),
                                    }
                                }
                                _ => (),
                            }
                        });
                    }
                }
            }
        }

        // /* fuse slots by decreasing size */
        sl.sort_by(scmp);
        // qsort(sl, nsl, sizeof *sl, scmp);
        'outer: for s0i in 0..sl.len() {
            if sl[s0i].si != SlotIdx::NONE {
                continue;
            }
            sl[s0i].si = SlotIdx::new(s0i);
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
                        if sl[mi].si == SlotIdx::new(s0i) && rovlap(&sl[mi].r, &sl[si].r) {
                            continue 'outer;
                        }
                    }
                }
                radd(&mut r, sl[si].r.a);
                radd(&mut r, sl[si].r.b - 1);
                sl[si].si = SlotIdx::new(s0i);
                fused += sl[si].sz;
            }
        }
    }

    {
        let tmps: &mut [Tmp] = &mut f.tmps;
        // /* substitute fused slots */
        for si in 0..sl.len() {
            // for (s=sl; s<&sl[nsl]; s++) {
            let sti: TmpIdx = sl[si].ti;
            let (t_def_ii, t_bid) = {
                let t: &mut Tmp = &mut tmps[sti];
                /* the visit link is stale,
                 * reset it before the slot()
                 * calls below
                 */
                t.svisit = SlotIdx::new(si);
                assert!(t.ndef == 1 && t.def != InsIdx::NONE);
                (t.def, t.bid)
            };
            let t_def_bi: BlkIdx = f.rpo[t_bid];
            if sl[si].si == SlotIdx::new(si) {
                continue;
            }
            blks.with_mut(t_def_bi, |b| b.ins_mut()[t_def_ii] = Ins::NOP);
            //f.blk_mut(t_def_bi).ins_mut()[t_def_ii.0 as usize] = Ins::NOP;
            let ssi: SlotIdx = sl[si].si;
            let ssti: TmpIdx = sl[ssi.0 as usize].ti;
            let (ts_def_ii, ts_bid) = {
                let ts: &Tmp = &tmps[ssti];
                (ts.def, ts.bid)
            };
            assert!(t_bid == ts_bid);
            if t_def_ii < ts_def_ii {
                /* make sure the slot we
                 * selected has a def that
                 * dominates its new uses
                 */
                blks.with_mut(t_def_bi, |b| {
                    let tsi: Ins = /*f.blk(t_def_bi)*/b.ins()[ts_def_ii]; // Note copy
                                                                          /*f.blk_mut(t_def_bi)*/
                    b.ins_mut()[t_def_ii] = tsi;
                    /*f.blk_mut(t_def_bi)*/
                    b.ins_mut()[ts_def_ii] = Ins::NOP;
                });
                tmps[ssti].def = t_def_ii;
            }
            for ui in 0..tmps[sti].uses.len() {
                let u: &Use = &tmps[sti].uses[ui];
                assert!(!matches!(u.typ, UseT::UPhi(_)));
                match u.typ {
                    UseT::UJmp => {
                        let bi: BlkIdx = rpo[u.bid];
                        blks.with_mut(bi, |b| /*f.blk_mut(bi)*/b.jmp_mut().arg = RTmp(ssti));
                    }
                    UseT::UIns(ii) => {
                        let bi: BlkIdx = rpo[u.bid];
                        //let b = f.blk_mut(bi);
                        blks.with_mut(bi, |b| {
                            let args: &mut [Ref; 2] = &mut b.ins_mut()[ii.0 as usize].args;
                            for arg in args {
                                if *arg == RTmp(sti) {
                                    *arg = RTmp(ssti);
                                }
                            }
                        });
                    }
                    _ => (),
                }
            }
        }
    }

    let tmps: &[Tmp] = &f.tmps;
    /* fix newly overlapping blits */
    for (bi, ii) in bl {
        blks.with_mut(bi, |b| {
            let i: Ins = b.ins_mut()[ii]; // Note - copy Ugh fixme
            if i.op == O::Oblit0 {
                if let Some((s0, off0)) = slot(tmps, cons, i.args[0]) {
                    if let Some((s1, off1)) = slot(tmps, cons, i.args[1]) {
                        if sl[s0].si == sl[s1].si {
                            if off0 < off1 {
                                assert!(ii.next().usize() < b.ins().len());
                                let blit1: &mut Ins = &mut b.ins_mut()[ii.next()];
                                assert!(blit1.op == O::Oblit1 && matches!(blit1.args[0], RInt(_)));
                                if let RInt(sz) = blit1.args[0] {
                                    assert!(sz >= 0);
                                    blit1.args[0] = RInt(-sz); // What are you doing Quentin???
                                }
                            } else if off0 == off1 {
                                b.ins_mut()[ii] = Ins::NOP;
                                b.ins_mut()[ii.next()] = Ins::NOP;
                            }
                        }
                    }
                }
            }
        });
    }

    if true
    /*TODO debug['M']*/
    {
        for (s0ii, s0) in sl.iter().enumerate() {
            //for (s0=sl; s0<&sl[nsl]; s0++) {
            let s0i = SlotIdx::new(s0ii);
            if s0.si != s0i {
                continue;
            }
            /*e*/
            print!("\tfuse ({:>3}b) [", s0.sz);
            for s in &sl {
                //for (s=s0; s<&sl[nsl]; s++) {
                if s.si != s0i {
                    continue;
                }
                /*e*/
                print!(" %{}", to_s(&tmps[s.ti].name));
                if s.r.b != 0 {
                    /*e*/
                    print!("[{},{})", s.r.a - ip, s.r.b - ip);
                } else {
                    /*e*/
                    print!("{{}}");
                }
            }
            /*e*/
            println!(" ]");
        }
        /*e*/
        println!(
            "\tsums {}/{}/{} (killed/fused/total)\n",
            freed, fused, total
        );
        printfn(/*stderr*/ &mut stdout(), f, typ, itbl);
    }
}
