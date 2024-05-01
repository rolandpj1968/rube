use std::io::stdout;

use crate::all::Ref::{RCon, RTmp, R};
use crate::all::K::{Kd, Kl, Ks, Kw};
use crate::all::{
    isret, kwide, to_s, Blk, BlkIdx, Blks, CmpF, CmpI, Con, ConIdx, ConPP, Fn, Idx, Ins, Phi,
    PhiIdx, Ref, RpoIdx, Tmp, TmpIdx, Typ, Use, UseT, J, K, O, OCMPD, OCMPD1, OCMPL1, OCMPS,
    OCMPS1, OCMPW, OCMPW1, TMP0,
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
    match (*cl, *cr) {
        (Con::CAddr(sym1, off1), Con::CAddr(sym2, off2)) => {
            if op == O::Osub && sym1 == sym2 {
                Some(Con::CBits(off1 - off2, ConPP::I))
            } else {
                None
            }
        }
        (Con::CAddr(sym, off), Con::CBits(i, _)) | (Con::CBits(i, _), Con::CAddr(sym, off)) => {
            if op == O::Oadd {
                Some(Con::CAddr(sym, off + i))
            } else {
                None
            }
        }
        (Con::CBits(mut li64, _), Con::CBits(mut ri64, _)) => {
            if op == O::Odiv || op == O::Orem || op == O::Oudiv || op == O::Ourem {
                if iscon(cr, w, 0) {
                    return None;
                }
                if op == O::Odiv || op == O::Orem {
                    let x: i64 = if w { i64::MIN } else { i32::MIN as i64 };
                    if (iscon(cr, w, -1i64 as u64)) && (iscon(cl, w, x as u64)) {
                        return None;
                    }
                }
            }
            let lfs: f32 = f32::from_bits(li64 as u32);
            let rfs: f32 = f32::from_bits(ri64 as u32);
            let lfd: f64 = f64::from_bits(li64 as u64);
            let rfd: f64 = f64::from_bits(ri64 as u64);
            let mut lu64: u64 = li64 as u64;
            let mut ru64: u64 = ri64 as u64;
            let lu32: u32 = li64 as u32;
            let ru32: u32 = ri64 as u32;
            let li32: i32 = li64 as i32;
            let ri32: i32 = ri64 as i32;
            let shmask: u64 = if w { 63 } else { 31 };
            let x: u64 = match op {
                O::Oadd => lu64 + ru64,
                O::Osub => lu64 - ru64,
                O::Oneg => (-li64) as u64,
                O::Odiv => (if w { li64 / ri64 } else { (li32 / ri32) as i64 }) as u64,
                O::Orem => (if w { li64 % ri64 } else { (li32 % ri32) as i64 }) as u64,
                O::Oudiv => (if w { lu64 / ru64 } else { (lu32 / ru32) as u64 }),
                O::Ourem => (if w { lu64 % ru64 } else { (lu32 % ru32) as u64 }),
                O::Omul => lu64 * ru64,
                O::Oand => lu64 & ru64,
                O::Oor => lu64 | ru64,
                O::Oxor => lu64 ^ ru64,
                O::Osar => ((if w { li64 } else { li32 as i64 }) >> (ru64 & shmask)) as u64,
                O::Oshr => (if w { lu64 } else { lu32 as u64 }) >> (ru64 & shmask),
                O::Oshl => lu64 << (ru64 & shmask),
                O::Oextsb => lu64 as i8 as i64 as u64,
                O::Oextub => lu64 as i8 as u64,
                O::Oextsh => lu64 as i16 as i64 as u64,
                O::Oextuh => lu64 as u16 as u64,
                O::Oextsw => li32 as i64 as u64,
                O::Oextuw => lu32 as u64,
                O::Ostosi => (if w { lfs as i64 } else { lfs as i32 as i64 }) as u64,
                O::Ostoui => (if w { lfs as u64 } else { lfs as u32 as u64 }),
                O::Odtosi => (if w { lfd as i64 } else { lfd as i32 as i64 }) as u64,
                O::Odtoui => (if w { lfd as u64 } else { lfd as u32 as u64 }),
                O::Ocast => {
                    lu64
                    // TODO
                    //     if (cl->type == CAddr) {
                    //         typ = CAddr;
                    //         sym = cl->sym;
                    //     }
                }
                _ => {
                    if OCMPW <= op && op <= OCMPL1 {
                        let cmpi: CmpI;
                        if op <= OCMPW1 {
                            cmpi = CmpI::from_op_w(op);
                            lu64 = li32 as u64;
                            ru64 = ri32 as u64;
                            // TODO: QBE doesn't do this - bug?
                            li64 = li32 as i64;
                            ri64 = ri32 as i64;
                        } else {
                            cmpi = CmpI::from_op_l(op);
                        }
                        (match cmpi {
                            CmpI::Ciule => lu64 <= ru64,
                            CmpI::Ciult => lu64 < ru64,
                            CmpI::Cisle => li64 <= ri64,
                            CmpI::Cislt => li64 < ri64,
                            CmpI::Cisgt => li64 > ri64,
                            CmpI::Cisge => li64 >= ri64,
                            CmpI::Ciugt => lu64 > ru64,
                            CmpI::Ciuge => lu64 >= ru64,
                            CmpI::Cieq => lu64 == ru64,
                            CmpI::Cine => lu64 != ru64,
                            _ => {
                                assert!(false); // unreachable
                                false
                            }
                        }) as u64
                    } else if OCMPS <= op && op <= OCMPS1 {
                        (match CmpF::from_op_s(op) {
                            CmpF::Cfle => lfs <= rfs,
                            CmpF::Cflt => lfs < rfs,
                            CmpF::Cfgt => lfs > rfs,
                            CmpF::Cfge => lfs >= rfs,
                            CmpF::Cfne => lfs != rfs,
                            CmpF::Cfeq => lfs == rfs,
                            CmpF::Cfo => lfs < rfs || lfs >= rfs,
                            CmpF::Cfuo => !(lfs < rfs || lfs >= rfs),
                            _ => {
                                assert!(false); // unreachable
                                false
                            }
                        }) as u64
                    } else if OCMPD <= op && op <= OCMPD1 {
                        (match CmpF::from_op_d(op) {
                            CmpF::Cfle => lfd <= rfd,
                            CmpF::Cflt => lfd < rfd,
                            CmpF::Cfgt => lfd > rfd,
                            CmpF::Cfge => lfd >= rfd,
                            CmpF::Cfne => lfd != rfd,
                            CmpF::Cfeq => lfd == rfd,
                            CmpF::Cfo => lfd < rfd || lfd >= rfd,
                            CmpF::Cfuo => !(lfd < rfd || lfd >= rfd),
                            _ => {
                                assert!(false); // unreachable
                                false
                            }
                        }) as u64
                    } else {
                        assert!(false); // unreachable
                        0
                    }
                }
            };
            Some(Con::CBits(x as i64, ConPP::I))
        }
        _ => None,
    }
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
