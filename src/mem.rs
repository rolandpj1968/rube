use std::error::Error;
use std::fmt;

use crate::all::{
    isload, isstore, kbase, to_s, Alias, AliasIdx, AliasT, AliasU, Bits, Blk, BlkIdx, Fn, Ins,
    InsIdx, KExt, Ref, RubeResult, Tmp, TmpIdx, Use, UseT, KL, KW, KX, O, OALLOC, OALLOC1, TMP0,
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

struct Store {
    // TODO
    // int ip;
    // Ins *i;
}

struct Slot {
    ti: TmpIdx,
    sz: i32,
    m: Bits,
    l: Bits,
    r: Range,
    // Slot *s; // TODO
    // Store *st; // TODO
    nst: i32,
}

fn rin(r: &Range, n: i32) -> bool {
    r.a <= n && n < r.b
}
/*
static inline int
rovlap(Range r0, Range r1)
{
    return r0.b && r1.b && r0.a < r1.b && r1.a < r0.b;
}
*/
fn radd(r: &mut Range, n: i32) {
    if r.b == 0 {
        *r = Range { a: n, b: n + 1 };
    } else if n < r.a {
        r.a = n;
    } else if n >= r.b {
        r.b = n + 1;
    }
}
/*
static int
slot(Slot **ps, int64_t *off, Ref r, Fn *fn, Slot *sl)
{
    Alias a;
    Tmp *t;

    getalias(&a, r, fn);
    if (a.type != ALoc)
        return 0;
    t = &fn->tmp[a.base];
    if (t->visit < 0)
        return 0;
    *off = a.offset;
    *ps = &sl[t->visit];
    return 1;
}
 */

fn load(r: Ref, x: Bits, ip: i32, f: &Fn, sl: &Slot) {
    // int64_t off;
    // Slot *s;

    if (slot(&s, &off, r, fn, sl)) {
        s->l |= x << off;
        s->l &= s->m;
        if (s->l)
            radd(&s->r, ip);
    }
}

/*
static void
store(Ref r, bits x, int ip, Ins *i, Fn *fn, Slot *sl)
{
    int64_t off;
    Slot *s;

    if (slot(&s, &off, r, fn, sl)) {
        if (s->l) {
            radd(&s->r, ip);
            s->l &= ~(x << off);
        } else {
            vgrow(&s->st, ++s->nst);
            s->st[s->nst-1].ip = ip;
            s->st[s->nst-1].i = i;
        }
    }
}

static int
scmp(const void *pa, const void *pb)
{
    Slot *a, *b;

    a = (Slot *)pa, b = (Slot *)pb;
    if (a->sz != b->sz)
        return b->sz - a->sz;
    return a->r.a - b->r.a;
}
 */
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
    for n in TMP0..(f.tmps.len() as u32) {
        let ti: TmpIdx = TmpIdx(n);
        f.tmp_mut(ti).visit = TmpIdx::NONE; // Ugh, this is a slot index in sl here
        let ai: AliasIdx = f.tmp(ti).alias;
        let a: &Alias = f.alias(ai);
        if a.type_ == AliasT::ALoc && a.slot == ai && f.tmp(ti).bid == f.blk(f.start).id {
            if let AliasU::ALoc(aloc) = a.u {
                if aloc.sz != -1 {
                    f.tmp_mut(ti).visit = TmpIdx(sl.len() as u32); // TODO - this is NOT a TmpIdx
                    sl.push(Slot {
                        ti,
                        sz: aloc.sz,
                        m: aloc.m,
                        l: 0,
                        r: Range { a: 0, b: 0 },
                        // TODO s.s = 0,
                        // TODO s.st = vnew(0, sizeof s.st[0], PHeap),
                        nst: 0,
                    });
                }
            }
        }
    }

    // /* one-pass liveness analysis */
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let b: &mut Blk = f.blk_mut(bi);
        b.loop_ = u32::MAX;
        bi = b.link;
    }
    loopiter(f, maxrpo);
    // nbl = 0;
    let mut bl: Vec<InsIdx> = vec![]; // Mmm
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
        //     if (b.jmp.type == Jretc)
        //         load(b.jmp.arg, -1, --ip, f, sl);
        //     for (i=&b.ins[b.nins]; i!=b.ins;) {
        //         --i;
        //         arg = i.arg;
        //         if (i.op == Oargc) {
        //             load(arg[1], -1, --ip, f, sl);
        //         }
        //         if (isload(i.op)) {
        //             x = BIT(loadsz(i)) - 1;
        //             load(arg[0], x, --ip, f, sl);
        //         }
        //         if (isstore(i.op)) {
        //             x = BIT(storesz(i)) - 1;
        //             store(arg[1], x, ip--, i, f, sl);
        //         }
        //         if (i.op == Oblit0) {
        //             assert((i+1).op == Oblit1);
        //             assert(rtype((i+1).arg[0]) == RInt);
        //             sz = abs(rsval((i+1).arg[0]));
        //             x = sz >= NBit ? (bits)-1 : BIT(sz) - 1;
        //             store(arg[1], x, ip--, i, f, sl);
        //             load(arg[0], x, ip, f, sl);
        //             vgrow(&bl, ++nbl);
        //             bl[nbl-1] = i;
        //         }
        //     }
        //     for (s=sl; s<&sl[nsl]; s++)
        //         if (s.l) {
        //             radd(&s.r, ip);
        //             if (b.loop != -1) {
        //                 assert(b.loop > n);
        //                 radd(&s.r, br[b.loop].b - 1);
        //             }
        //         }
        //     br[n].a = ip;
    }
    // free(br);

    // /* kill dead stores */
    // for (s=sl; s<&sl[nsl]; s++)
    //     for (n=0; n<s.nst; n++)
    //         if (!rin(s.r, s.st[n].ip)) {
    //             i = s.st[n].i;
    //             if (i.op == Oblit0)
    //                 *(i+1) = (Ins){.op = Onop};
    //             *i = (Ins){.op = Onop};
    //         }

    // /* kill slots with an empty live range */
    // total = 0;
    // freed = 0;
    // stk = vnew(0, sizeof stk[0], PHeap);
    // n = 0;
    // for (s=s0=sl; s<&sl[nsl]; s++) {
    //     total += s.sz;
    //     if (!s.r.b) {
    //         vfree(s.st);
    //         vgrow(&stk, ++n);
    //         stk[n-1] = s.t;
    //         freed += s.sz;
    //     } else
    //         *s0++ = *s;
    // }
    // nsl = s0-sl;
    // if (debug['M']) {
    //     fputs("\n> Slot coalescing:\n", stderr);
    //     if (n) {
    //         fputs("\tkill [", stderr);
    //         for (m=0; m<n; m++)
    //             fprintf(stderr, " %%%s",
    //                 f.tmp[stk[m]].name);
    //         fputs(" ]\n", stderr);
    //     }
    // }
    // while (n--) {
    //     t = &f.tmp[stk[n]];
    //     assert(t.ndef == 1 && t.def);
    //     i = t.def;
    //     if (isload(i.op)) {
    //         i.op = Ocopy;
    //         i.arg[0] = UNDEF;
    //         continue;
    //     }
    //     *i = (Ins){.op = Onop};
    //     for (u=t.use; u<&t.use[t.nuse]; u++) {
    //         if (u.type == UJmp) {
    //             b = f.rpo[u.bid];
    //             assert(isret(b.jmp.type));
    //             b.jmp.type = Jret0;
    //             b.jmp.arg = R;
    //             continue;
    //         }
    //         assert(u.type == UIns);
    //         i = u.u.ins;
    //         if (!req(i.to, R)) {
    //             assert(rtype(i.to) == RTmp);
    //             vgrow(&stk, ++n);
    //             stk[n-1] = i.to.val;
    //         } else if (isarg(i.op)) {
    //             assert(i.op == Oargc);
    //             i.arg[1] = CON_Z;  /* crash */
    //         } else {
    //             if (i.op == Oblit0)
    //                 *(i+1) = (Ins){.op = Onop};
    //             *i = (Ins){.op = Onop};
    //         }
    //     }
    // }
    // vfree(stk);

    // /* fuse slots by decreasing size */
    // qsort(sl, nsl, sizeof *sl, scmp);
    // fused = 0;
    // for (n=0; n<nsl; n++) {
    //     s0 = &sl[n];
    //     if (s0.s)
    //         continue;
    //     s0.s = s0;
    //     r = s0.r;
    //     for (s=s0+1; s<&sl[nsl]; s++) {
    //         if (s.s || !s.r.b)
    //             goto Skip;
    //         if (rovlap(r, s.r))
    //             /* O(n); can be approximated
    //              * by 'goto Skip;' if need be
    //              */
    //             for (m=n; &sl[m]<s; m++)
    //                 if (sl[m].s == s0)
    //                 if (rovlap(sl[m].r, s.r))
    //                     goto Skip;
    //         radd(&r, s.r.a);
    //         radd(&r, s.r.b - 1);
    //         s.s = s0;
    //         fused += s.sz;
    //     Skip:;
    //     }
    // }

    // /* substitute fused slots */
    // for (s=sl; s<&sl[nsl]; s++) {
    //     t = &f.tmp[s.t];
    //     /* the visit link is stale,
    //      * reset it before the slot()
    //      * calls below
    //      */
    //     t.visit = s-sl;
    //     assert(t.ndef == 1 && t.def);
    //     if (s.s == s)
    //         continue;
    //     *t.def = (Ins){.op = Onop};
    //     ts = &f.tmp[s.s.t];
    //     assert(t.bid == ts.bid);
    //     if (t.def < ts.def) {
    //         /* make sure the slot we
    //          * selected has a def that
    //          * dominates its new uses
    //          */
    //         *t.def = *ts.def;
    //         *ts.def = (Ins){.op = Onop};
    //         ts.def = t.def;
    //     }
    //     for (u=t.use; u<&t.use[t.nuse]; u++) {
    //         if (u.type == UJmp) {
    //             b = f.rpo[u.bid];
    //             b.jmp.arg = TMP(s.s.t);
    //             continue;
    //         }
    //         assert(u.type == UIns);
    //         arg = u.u.ins.arg;
    //         for (n=0; n<2; n++)
    //             if (req(arg[n], TMP(s.t)))
    //                 arg[n] = TMP(s.s.t);
    //     }
    // }

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
