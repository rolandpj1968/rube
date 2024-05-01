use std::io::stdout;

use crate::all::Ref::{RCon, RTmp, R};
use crate::all::K::{Kd, Kl, Ks, Kw};
use crate::all::{
    isret, kwide, to_s, Blk, BlkIdx, Blks, Con, ConIdx, ConPP, Fn, Idx, Ins, Phi, PhiIdx, Ref,
    RpoIdx, Tmp, TmpIdx, Typ, Use, UseT, J, K, O, TMP0,
};
use crate::optab::OPTAB;
use crate::parse::printref;
use crate::util::{newconcon2, Bucket};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i8)]
enum Lat {
    Bot = -1, /* lattice bottom */
    Top = 0,  /* lattice top (matches UNDEF) */
    Con(ConIdx),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    dest: RpoIdx,
    dead: bool,
    work: EdgeIdx,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct EdgeTag {}
type EdgeIdx = Idx<EdgeTag>;

fn iscon(c: &Con, w: bool, k: u64) -> bool {
    if let Con::CBits(i, _) = *c {
        if w {
            i as u64 == k
        } else {
            i as u32 == k as u32
        }
    } else {
        false
    }
}

fn latval(val: &[Lat], r: Ref) -> Lat {
    match r {
        RTmp(ti) => val[ti.usize()],
        RCon(ci) => Lat::Con(ci),
        _ => {
            // unreachable
            Lat::Bot
        }
    }
}

fn latmerge(v: Lat, m: Lat) -> Lat {
    if m == Lat::Top {
        v
    } else if v == Lat::Top || v == m {
        m
    } else {
        Lat::Bot
    }
}

fn update(
    tmps: &[Tmp],
    usewrk: &mut Vec<(TmpIdx, u32 /*UseIdx*/)>,
    val: &mut [Lat],
    ti: TmpIdx,
    m: Lat,
) {
    let m: Lat = latmerge(val[ti.usize()], m);
    if m != val[ti.usize()] {
        let t: &Tmp = &tmps[ti];
        for u in 0..t.uses.len() {
            usewrk.push((ti, u as u32));
        }
        val[ti.usize()] = m;
    }
}

fn deadedge(edge: &[Edge], s: RpoIdx, d: RpoIdx) -> bool {
    let e0: &Edge = &edge[s.usize() * 2];
    if e0.dest == d && !e0.dead {
        return false;
    }
    let e1: &Edge = &edge[s.usize() * 2 + 1];
    if e1.dest == d && !e1.dead {
        return false;
    }
    true
}

fn visitphi(
    blks: &Blks,
    tmps: &[Tmp],
    usewrk: &mut Vec<(TmpIdx, u32 /*UseIdx*/)>,
    val: &mut [Lat],
    edge: &[Edge],
    p: &Phi,
    n: RpoIdx,
) {
    let mut v: Lat = Lat::Top;
    assert!(p.args.len() == p.blks.len());
    for a in 0..p.args.len() {
        if !deadedge(edge, blks.id_of(p.blks[a]), n) {
            v = latmerge(v, latval(val, p.args[a]));
        }
    }
    assert!(matches!(p.to, RTmp(_)));
    if let RTmp(ti) = p.to {
        update(tmps, usewrk, val, ti, v);
    }
}

fn visitins(
    tmps: &[Tmp],
    cons: &mut Vec<Con>,
    usewrk: &mut Vec<(TmpIdx, u32 /*UseIdx*/)>,
    val: &mut [Lat],
    i: &Ins,
) {
    if let RTmp(ti) = i.to {
        let mut v: Lat = Lat::Bot;
        if OPTAB[i.op].canfold {
            let l: Lat = latval(val, i.args[0]);
            let r: Lat = if i.args[1] != R {
                latval(val, i.args[1])
            } else {
                Lat::Con(ConIdx::CON_Z)
            };
            v = match (l, r) {
                (Lat::Bot, _) | (_, Lat::Bot) => Lat::Bot,
                (Lat::Top, _) | (_, Lat::Top) => Lat::Top,
                (Lat::Con(lci), Lat::Con(rci)) => opfold(cons, i.op, i.cls, lci, rci),
            };
        }
        update(tmps, usewrk, val, ti, v);
    }
}

fn visitjmp(
    cons: &[Con],
    val: &[Lat],
    edge: &mut [Edge],
    flowrk: &mut EdgeIdx,
    b: &Blk,
    n: RpoIdx,
) {
    assert!(b.id == n); // ???
    match b.jmp().typ {
        J::Jjnz => {
            let l: Lat = latval(val, b.jmp().arg);
            match l {
                Lat::Bot => {
                    edge[n.usize() * 2 + 1].work = *flowrk;
                    edge[n.usize() * 2].work = EdgeIdx::from(n.usize() * 2 + 1);
                    *flowrk = EdgeIdx::from(n.usize() * 2);
                }
                Lat::Con(ci) => {
                    if iscon(&cons[ci], false, 0) {
                        assert!(edge[n.usize() * 2].dead);
                        edge[n.usize() * 2 + 1].work = *flowrk;
                        *flowrk = EdgeIdx::from(n.usize() * 2 + 1);
                    } else {
                        assert!(edge[n.usize() * 2 + 1].dead);
                        edge[n.usize() * 2].work = *flowrk;
                        *flowrk = EdgeIdx::from(n.usize() * 2);
                    }
                }
                Lat::Top => assert!(false),
            }
        }
        J::Jjmp => {
            edge[n.usize() * 2].work = *flowrk;
            *flowrk = EdgeIdx::from(n.usize() * 2);
        }
        J::Jhlt => (),
        _ => assert!(isret(b.jmp().typ)),
    }
}

fn initedge(blks: &Blks, e: &mut Edge, s: BlkIdx) {
    if s != BlkIdx::NONE {
        e.dest = blks.borrow(s).id;
    } else {
        e.dest = RpoIdx::NONE;
    }
    e.dead = true;
    e.work = EdgeIdx::NONE;
}

/*
static int
renref(Ref *r)
{
    int l;

    if (rtype(*r) == RTmp)
        if ((l=val[r->val]) != Bot) {
            *r = CON(l);
            return 1;
        }
    return 0;
}
 */

/* require rpo, use, pred */
fn fold(f: &mut Fn, typ: &[Typ], itbl: &[Bucket]) {
    //     Edge *e, start;
    //     Use *u;
    //     Blk *b, **pb;
    //     Phi *p, **pp;
    //     Ins *i;
    //     int t, d;
    //     uint n, a;

    let blks: &Blks = &f.blks;
    let rpo: &[BlkIdx] = &f.rpo;
    let phis: &[Phi] = &f.phis;
    let tmps: &[Tmp] = &f.tmps;
    let cons: &mut Vec<Con> = &mut f.cons;

    let mut val: Vec<Lat> = vec![Lat::Top; tmps.len()];
    assert!(f.nblk as usize == rpo.len());
    let mut edge: Vec<Edge> = vec![
            Edge {
                dest: RpoIdx::NONE,
                dead: false,
                work: EdgeIdx::NONE
            }
        ;
        rpo.len()*2 + 1 /* use edge[len*2] for start */
    ];
    let mut usewrk: Vec<(TmpIdx, u32 /*UseIdx*/)> = vec![];

    for n in 0..rpo.len() {
        blks.with_mut(rpo[n], |b| {
            b.ivisit = 0;
            initedge(blks, &mut edge[n * 2], b.s1);
            initedge(blks, &mut edge[n * 2 + 1], b.s2);
        });
    }
    assert!(f.start == BlkIdx::START);
    initedge(blks, &mut edge[rpo.len() * 2], BlkIdx::START);
    let mut flowrk: EdgeIdx = EdgeIdx::from(rpo.len() * 2);

    /* 1. find out constants and dead cfg edges */
    loop {
        let ei: EdgeIdx = flowrk;
        if ei != EdgeIdx::NONE {
            let e: &mut Edge = &mut edge[ei.usize()];
            flowrk = e.work;
            e.work = EdgeIdx::NONE;
            if e.dest == RpoIdx::NONE || !e.dead {
                continue;
            }
            e.dead = false;
            let n: RpoIdx = e.dest;
            let bi: BlkIdx = rpo[n];
            let mut pi: PhiIdx = blks.phi_of(bi);
            while pi != PhiIdx::NONE {
                let p: &Phi = &phis[pi];
                visitphi(blks, tmps, &mut usewrk, &mut val, &edge, p, n);
                pi = p.link;
            }
            if blks.ivisit_of(bi) == 0 {
                blks.with(bi, |b| {
                    for i in b.ins().iter() {
                        visitins(tmps, cons, &mut usewrk, &mut val, i);
                    }
                    visitjmp(cons, &val, &mut edge, &mut flowrk, b, n);
                });
            }
            blks.with_mut(bi, |b| {
                b.ivisit += 1;
                assert!(
                    b.jmp.borrow().typ != J::Jjmp
                        || !edge[n.usize() * 2].dead
                        || flowrk.usize() == n.usize() * 2
                );
            });
        } else {
            match usewrk.pop() {
                None => break,
                Some((ti, ui)) => {
                    let u: &Use = &tmps[ti].uses[ui as usize];
                    let n: RpoIdx = u.bid;
                    let bi: BlkIdx = rpo[n];
                    if blks.ivisit_of(bi) == 0 {
                        continue;
                    }
                    match u.typ {
                        UseT::UPhi(pi) => {
                            visitphi(blks, tmps, &mut usewrk, &mut val, &edge, &phis[pi], n)
                        }
                        UseT::UIns(ii) => {
                            blks.with(bi, |b| {
                                visitins(tmps, cons, &mut usewrk, &mut val, &b.ins()[ii]);
                            });
                        }
                        UseT::UJmp => {
                            blks.with(bi, |b| {
                                visitjmp(cons, &val, &mut edge, &mut flowrk, b, n);
                            });
                        }
                        _ => {
                            // unreachable
                            assert!(false);
                        }
                    }
                }
            }
        }
    }

    if true
    /*debug['F']*/
    {
        /*e*/
        print!("\n> SCCP findings:");
        for tii in TMP0..tmps.len() {
            let ti: TmpIdx = TmpIdx::from(tii);
            if val[ti.usize()] == Lat::Bot {
                continue;
            }
            /*e*/
            print!("\n{:>10}: ", to_s(&tmps[ti].name));
            match val[ti.usize()] {
                Lat::Bot => (),
                Lat::Top =>
                /*e*/
                {
                    print!("Top")
                }
                Lat::Con(ci) => printref(/*stderr*/ &mut stdout(), f, typ, itbl, RCon(ci)),
            }
        }
        /*e*/
        print!("\n dead code: ");
    }

    //     /* 2. trim dead code, replace constants */
    //     d = 0;
    //     for (pb=&f.start; (b=*pb);) {
    //         if (b.ivisit == 0) {
    //             d = 1;
    //             if (debug['F'])
    //                 fprintf(stderr, "%s ", b.name);
    //             edgedel(b, &b.s1);
    //             edgedel(b, &b.s2);
    //             *pb = b.link;
    //             continue;
    //         }
    //         for (pp=&b.phi; (p=*pp);)
    //             if (val[p.to.val] != Bot)
    //                 *pp = p.link;
    //             else {
    //                 for (a=0; a<p.narg; a++)
    //                     if (!deadedge(p.blk[a].id, b.id))
    //                         renref(&p.arg[a]);
    //                 pp = &p.link;
    //             }
    //         for (i=b.ins; i<&b.ins[b.nins]; i++)
    //             if (renref(&i.to))
    //                 *i = (Ins){.op = Onop};
    //             else {
    //                 for (n=0; n<2; n++)
    //                     renref(&i.arg[n]);
    //                 if (isstore(i.op))
    //                 if (req(i.arg[0], UNDEF))
    //                     *i = (Ins){.op = Onop};
    //             }
    //         renref(&b.jmp.arg);
    //         if (b.jmp.type == Jjnz && rtype(b.jmp.arg) == RCon) {
    //                 if (iscon(&f.con[b.jmp.arg.val], 0, 0)) {
    //                     edgedel(b, &b.s1);
    //                     b.s1 = b.s2;
    //                     b.s2 = 0;
    //                 } else
    //                     edgedel(b, &b.s2);
    //                 b.jmp.type = Jjmp;
    //                 b.jmp.arg = R;
    //         }
    //         pb = &b.link;
    //     }

    //     if (debug['F']) {
    //         if (!d)
    //             fprintf(stderr, "(none)");
    //         fprintf(stderr, "\n\n> After constant folding:\n");
    //         printfn(f, stderr);
    //     }

    //     free(val);
    //     free(edge);
    //     vfree(usewrk);
}

/* boring folding code */

fn foldint(op: O, w: bool, cl: &Con, cr: &Con) -> Option<Con> {
    // union {
    //     int64_t s;
    //     uint64_t u;
    //     float fs;
    //     double fd;
    // } l, r;
    // uint64_t x;
    // Sym sym;
    // int typ;

    // memset(&sym, 0, sizeof sym);
    // typ = CBits;
    // l.s = cl->bits.i;
    // r.s = cr->bits.i;
    // if (op == Oadd) {
    //     if (cl->type == CAddr) {
    //         if (cr->type == CAddr)
    //             return 1;
    //         typ = CAddr;
    //         sym = cl->sym;
    //     }
    //     else if (cr->type == CAddr) {
    //         typ = CAddr;
    //         sym = cr->sym;
    //     }
    // }
    // else if (op == Osub) {
    //     if (cl->type == CAddr) {
    //         if (cr->type != CAddr) {
    //             typ = CAddr;
    //             sym = cl->sym;
    //         } else if (!symeq(cl->sym, cr->sym))
    //             return 1;
    //     }
    //     else if (cr->type == CAddr)
    //         return 1;
    // }
    // else if (cl->type == CAddr || cr->type == CAddr)
    //     return 1;
    // if (op == Odiv || op == Orem || op == Oudiv || op == Ourem) {
    //     if (iscon(cr, w, 0))
    //         return 1;
    //     if (op == Odiv || op == Orem) {
    //         x = w ? INT64_MIN : INT32_MIN;
    //         if (iscon(cr, w, -1))
    //         if (iscon(cl, w, x))
    //             return 1;
    //     }
    // }
    // switch (op) {
    // case Oadd:  x = l.u + r.u; break;
    // case Osub:  x = l.u - r.u; break;
    // case Oneg:  x = -l.u; break;
    // case Odiv:  x = w ? l.s / r.s : (int32_t)l.s / (int32_t)r.s; break;
    // case Orem:  x = w ? l.s % r.s : (int32_t)l.s % (int32_t)r.s; break;
    // case Oudiv: x = w ? l.u / r.u : (uint32_t)l.u / (uint32_t)r.u; break;
    // case Ourem: x = w ? l.u % r.u : (uint32_t)l.u % (uint32_t)r.u; break;
    // case Omul:  x = l.u * r.u; break;
    // case Oand:  x = l.u & r.u; break;
    // case Oor:   x = l.u | r.u; break;
    // case Oxor:  x = l.u ^ r.u; break;
    // case Osar:  x = (w ? l.s : (int32_t)l.s) >> (r.u & (31|w<<5)); break;
    // case Oshr:  x = (w ? l.u : (uint32_t)l.u) >> (r.u & (31|w<<5)); break;
    // case Oshl:  x = l.u << (r.u & (31|w<<5)); break;
    // case Oextsb: x = (int8_t)l.u;   break;
    // case Oextub: x = (uint8_t)l.u;  break;
    // case Oextsh: x = (int16_t)l.u;  break;
    // case Oextuh: x = (uint16_t)l.u; break;
    // case Oextsw: x = (int32_t)l.u;  break;
    // case Oextuw: x = (uint32_t)l.u; break;
    // case Ostosi: x = w ? (int64_t)cl->bits.s : (int32_t)cl->bits.s; break;
    // case Ostoui: x = w ? (uint64_t)cl->bits.s : (uint32_t)cl->bits.s; break;
    // case Odtosi: x = w ? (int64_t)cl->bits.d : (int32_t)cl->bits.d; break;
    // case Odtoui: x = w ? (uint64_t)cl->bits.d : (uint32_t)cl->bits.d; break;
    // case Ocast:
    //     x = l.u;
    //     if (cl->type == CAddr) {
    //         typ = CAddr;
    //         sym = cl->sym;
    //     }
    //     break;
    // default:
    //     if (Ocmpw <= op && op <= Ocmpl1) {
    //         if (op <= Ocmpw1) {
    //             l.u = (int32_t)l.u;
    //             r.u = (int32_t)r.u;
    //         } else
    //             op -= Ocmpl - Ocmpw;
    //         switch (op - Ocmpw) {
    //         case Ciule: x = l.u <= r.u; break;
    //         case Ciult: x = l.u < r.u;  break;
    //         case Cisle: x = l.s <= r.s; break;
    //         case Cislt: x = l.s < r.s;  break;
    //         case Cisgt: x = l.s > r.s;  break;
    //         case Cisge: x = l.s >= r.s; break;
    //         case Ciugt: x = l.u > r.u;  break;
    //         case Ciuge: x = l.u >= r.u; break;
    //         case Cieq:  x = l.u == r.u; break;
    //         case Cine:  x = l.u != r.u; break;
    //         default: die("unreachable");
    //         }
    //     }
    //     else if (Ocmps <= op && op <= Ocmps1) {
    //         switch (op - Ocmps) {
    //         case Cfle: x = l.fs <= r.fs; break;
    //         case Cflt: x = l.fs < r.fs;  break;
    //         case Cfgt: x = l.fs > r.fs;  break;
    //         case Cfge: x = l.fs >= r.fs; break;
    //         case Cfne: x = l.fs != r.fs; break;
    //         case Cfeq: x = l.fs == r.fs; break;
    //         case Cfo: x = l.fs < r.fs || l.fs >= r.fs; break;
    //         case Cfuo: x = !(l.fs < r.fs || l.fs >= r.fs); break;
    //         default: die("unreachable");
    //         }
    //     }
    //     else if (Ocmpd <= op && op <= Ocmpd1) {
    //         switch (op - Ocmpd) {
    //         case Cfle: x = l.fd <= r.fd; break;
    //         case Cflt: x = l.fd < r.fd;  break;
    //         case Cfgt: x = l.fd > r.fd;  break;
    //         case Cfge: x = l.fd >= r.fd; break;
    //         case Cfne: x = l.fd != r.fd; break;
    //         case Cfeq: x = l.fd == r.fd; break;
    //         case Cfo: x = l.fd < r.fd || l.fd >= r.fd; break;
    //         case Cfuo: x = !(l.fd < r.fd || l.fd >= r.fd); break;
    //         default: die("unreachable");
    //         }
    //     }
    //     else
    //         die("unreachable");
    // }
    // *res = (Con){.type=typ, .sym=sym, .bits={.i=x}};
    // return 0;
    None // for now
}

// TODO Result<Con>
fn invalidop(op: O, isaddr: bool) -> Con {
    // TODO...
    if isaddr {
        //err("invalid address operand for '%s'", optab[op].name);
    } else {
        //err("invalid operand type for '%s'", optab[op].name);
    }
    assert!(false);
    Con::CUndef
}

fn foldflt(op: O, w: bool, cl: &Con, cr: &Con) -> Con {
    match (*cl, *cr) {
        (Con::CBits(li, _), Con::CBits(ri, _)) => {
            if w {
                let ld: f64 = f64::from_bits(li as u64);
                let rd: f64 = f64::from_bits(ri as u64);
                let xd: f64 = match op {
                    O::Oadd => ld + rd,
                    O::Osub => ld - rd,
                    O::Oneg => -ld,
                    O::Odiv => ld / rd,
                    O::Omul => ld * rd,
                    O::Oswtof => (li as i32) as f64,
                    O::Ouwtof => (li as u32) as f64,
                    O::Osltof => (li as i64) as f64,
                    O::Oultof => (li as u64) as f64,
                    O::Oexts => f32::from_bits(li as u32) as f64,
                    O::Ocast => f64::from_bits(li as u64),
                    _ => return invalidop(op, false),
                };
                Con::CBits(xd.to_bits() as i64, ConPP::D)
            } else {
                let ls: f32 = f32::from_bits(li as u32);
                let rs: f32 = f32::from_bits(ri as u32);
                let xs: f32 = match op {
                    O::Oadd => ls + rs,
                    O::Osub => ls - rs,
                    O::Oneg => -ls,
                    O::Odiv => ls / rs,
                    O::Omul => ls * rs,
                    O::Oswtof => (li as i32) as f32,
                    O::Ouwtof => (li as u32) as f32,
                    O::Osltof => (li as i64) as f32,
                    O::Oultof => (li as u64) as f32,
                    O::Otruncd => f64::from_bits(li as u64) as f32,
                    O::Ocast => f32::from_bits(li as u32),
                    _ => return invalidop(op, false),
                };
                Con::CBits(xs.to_bits() as i64, ConPP::S)
            }
        }
        _ => {
            return invalidop(op, true);
        }
    }
}

fn opfold(cons: &mut Vec<Con>, op: O, cls: K, cli: ConIdx, cri: ConIdx) -> Lat {
    let mut c: Con = {
        if cls == Kw || cls == Kl {
            match foldint(op, cls == Kl, &cons[cli], &cons[cri]) {
                None => return Lat::Bot,
                Some(c0) => c0,
            }
        } else {
            foldflt(op, cls == Kd, &cons[cli], &cons[cri])
        }
    };
    // TODO - this is a bit weird
    if kwide(cls) == 0 {
        if let Con::CBits(i, _) = &mut c {
            *i &= 0xffffffff;
        }
    }
    let ci: ConIdx = newconcon2(cons, c);
    assert!((cls == Ks || cls == Kd) != matches!(c, Con::CBits(_, ConPP::I)));
    Lat::Con(ci)
}
