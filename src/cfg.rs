use std::cell;

use crate::all::{Blk, BlkIdx, Blks, Fn, Phi, PhiIdx};

// Not pretty - would be better if s1, s2 were [BlkIndex; 2]
fn succsdel(b: &mut Blk, bdi: BlkIdx) {
    if b.s1 == bdi {
        b.s1 = BlkIdx::NONE;
    }
    if b.s2 == bdi {
        b.s2 = BlkIdx::NONE;
    }
}

fn phisdel(phis: &mut [Phi], mut pi: PhiIdx, bsi: BlkIdx) {
    while pi != PhiIdx::NONE {
        let p: &mut Phi = &mut phis[pi];
        if let Some(a) = p.blks.iter().position(|bi| *bi == bsi) {
            p.blks.remove(a);
            p.args.remove(a);
        }
        pi = p.link;
    }
}

fn preddel(b: &mut Blk, bsi: BlkIdx) {
    if let Some(a) = b.preds.iter().position(|pbi| *pbi == bsi) {
        b.preds.remove(a);
    }
}

fn edgedel(blks: &Blks, phis: &mut [Phi], bsi: BlkIdx, bdi: BlkIdx) {
    if bdi != BlkIdx::NONE {
        blks.with_mut(bsi, |bs| succsdel(bs, bdi));
        phisdel(phis, blks.phi_of(bdi), bsi);
        blks.with_mut(bdi, |bd| preddel(bd, bsi));
    }
}

pub fn fillpreds(f: &Fn) {
    let blks = &f.blks;
    blks.for_each_mut(|b| b.preds.clear());
    blks.for_each_bi(|bi| {
        // let mut bi: BlkIdx = f.start;
        // while bi != BlkIdx::NONE {
        let (succs, link) = blks.with(bi, |b| (b.succs(), b.link));
        succs.iter().for_each(|si| {
            if *si != BlkIdx::NONE {
                blks.with_mut(*si, |s| s.preds.push(bi));
            }
        });
        //bi = link;
    });
}

fn rporec(blks: &Blks, bi: BlkIdx, mut x: u32) -> u32 {
    if bi == BlkIdx::NONE || blks.borrow(bi).id != u32::MAX {
        return x;
    }

    // Borrow immutably here cos s1, s2 could be same as bi
    let swap_succs = blks.with(bi, |b| {
        // TODO - check signedness of .loop_ - might need u32::MAX < 0
        b.s1 != BlkIdx::NONE
            && b.s2 != BlkIdx::NONE
            && b.s1 != b.s2 // Not actually needed
            && blks.borrow(b.s1).loop_ > blks.borrow(b.s2).loop_
    });

    let succs = blks.with_mut(bi, |b| {
        b.id = 1;
        if swap_succs {
            (b.s1, b.s2) = (b.s2, b.s1);
        }
        b.succs()
    });
    succs.iter().for_each(|si| x = rporec(blks, *si, x));
    assert!(x != u32::MAX);

    blks.with_mut(bi, |b| b.id = x);

    // Deliberately wraps to u32:MAX
    x.wrapping_sub(1)
}

/* fill the reverse post-order (rpo) information */
pub fn fillrpo(f: &mut Fn) {
    let blks = &f.blks;
    let phis: &mut [Phi] = &mut f.phis;

    blks.for_each_mut(|b| b.id = u32::MAX);

    // Deliberately wraps from u32::MAX
    let n: u32 = rporec(blks, f.start, f.nblk - 1).wrapping_add(1);
    f.nblk -= n;
    f.rpo = vec![BlkIdx::NONE; f.nblk as usize];
    let mut prev_bi = BlkIdx::NONE;
    let mut bi = f.start;
    while bi != BlkIdx::NONE {
        let (id, succs, link) = blks.with(bi, |b| (b.id, b.succs(), b.link));
        if id == u32::MAX {
            // Unreachable Blk
            succs.iter().for_each(|si| edgedel(blks, phis, bi, *si));
            blks.with_mut(prev_bi, |pb| pb.link = link);
            blks.with_mut(bi, |b| b.is_dead = true);
        } else {
            blks.with_mut(bi, |b| {
                b.id -= n;
                f.rpo[b.id as usize] = bi;
                prev_bi = bi;
            });
        }
        bi = link;
    }
}

/* for dominators computation, read
 * "A Simple, Fast Dominance Algorithm"
 * by K. Cooper, T. Harvey, and K. Kennedy.
 */

fn inter(blks: &Blks, mut bi1: BlkIdx, mut bi2: BlkIdx) -> BlkIdx {
    if bi1 == BlkIdx::NONE {
        return bi2;
    }
    while bi1 != bi2 {
        if blks.id_of(bi1) < blks.id_of(bi2) {
            (bi1, bi2) = (bi2, bi1);
        }
        while blks.id_of(bi1) > blks.id_of(bi2) {
            bi1 = blks.idom_of(bi1);
            assert!(bi1 != BlkIdx::NONE);
        }
    }
    bi1
}

pub fn filldom(f: &mut Fn) {
    let blks: &Blks = &f.blks;
    let rpo: &[BlkIdx] = &f.rpo;

    blks.for_each_mut(|b| {
        b.idom = BlkIdx::NONE;
        b.dom = BlkIdx::NONE;
        b.dlink = BlkIdx::NONE;
    });
    loop {
        let mut ch: u32 = 0;
        for bi in rpo.iter().skip(1) {
            let di = blks.with(*bi, |b| {
                let mut di0: BlkIdx = BlkIdx::NONE;
                for pi in &b.preds {
                    if blks.idom_of(*pi) != BlkIdx::NONE || *pi == f.start {
                        di0 = inter(blks, di0, *pi);
                    }
                }
                di0
            });
            if di != blks.idom_of(*bi) {
                ch += 1;
                blks.with_mut(*bi, |b| b.idom = di);
            }
        }
        if ch == 0 {
            break;
        }
    }
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        blks.with_mut(bi, |b| {
            let di: BlkIdx = b.idom;
            if di != BlkIdx::NONE {
                assert!(di != bi);
                blks.with_mut(di, |d| {
                    b.dlink = d.dom;
                    d.dom = bi;
                });
            }

            bi = b.link;
        });
    }
}

pub fn sdom(blks: &Blks, b1i: BlkIdx, mut b2i: BlkIdx) -> bool {
    assert!(b1i != BlkIdx::NONE && b2i != BlkIdx::NONE);
    if b1i == b2i {
        return false;
    }
    while blks.id_of(b2i) > blks.id_of(b1i) {
        b2i = blks.idom_of(b2i);
    }
    b1i == b2i
}

pub fn dom(blks: &Blks, b1i: BlkIdx, b2i: BlkIdx) -> bool {
    b1i == b2i || sdom(blks, b1i, b2i)
}

fn addfron(a: &mut Blk, bi: BlkIdx) {
    if !a.frons.contains(&bi) {
        a.frons.push(bi);
    }
}

fn fillfron_for_succ(blks: &Blks, bi: BlkIdx, si: BlkIdx) {
    if si != BlkIdx::NONE {
        let mut ai = bi;
        while !sdom(blks, ai, si) {
            blks.with_mut(ai, |a| {
                addfron(a, si);
                ai = a.idom;
            });
        }
    }
}

/* fill the dominance frontier */
pub fn fillfron(f: &mut Fn) {
    let blks: &Blks = &f.blks;

    blks.for_each_mut(|b| b.frons.clear());

    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let (succs, link) = blks.with(bi, |b| (b.succs(), b.link));
        succs.iter().for_each(|si| fillfron_for_succ(blks, bi, *si));
        bi = link;
    }
}

fn loopmark(f: &mut Fn, hdi: BlkIdx, bi: BlkIdx, func: fn(&mut Fn, BlkIdx, BlkIdx)) {
    {
        let hd = f.blk(hdi);
        let b = f.blk(bi);
        if b.id < hd.id || b.visit == hd.id {
            return;
        }
    }
    f.blk_mut(bi).visit = f.blk(hdi).id;
    func(f, hdi, bi);
    let preds_len = f.blk(bi).preds.len();
    for p in 0..preds_len
    /*f.blk(bi).preds.len()*/
    {
        let predi: BlkIdx = f.blk(bi).preds[p];
        loopmark(f, hdi, predi, func);
    }
}

pub fn loopiter(f: &mut Fn, func: fn(&mut Fn, BlkIdx, BlkIdx)) {
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let mut b: cell::RefMut<Blk> = f.blk_mut(bi);
        b.visit = u32::MAX;
        bi = b.link;
    }

    for n in 0..f.rpo.len() {
        let bi: BlkIdx = f.rpo[n];
        let preds_len = f.blk(bi).preds.len();
        for p in 0..preds_len
        /*0..f.blk(bi).preds.len()*/
        {
            let predi: BlkIdx = f.blk(bi).preds[p];
            if f.blk(predi).id >= n as u32 {
                loopmark(f, bi, predi, func);
            }
        }
    }
}
/*
void
multloop(Blk *hd, Blk *b)
{
    (void)hd;
    b->loop *= 10;
}

void
fillloop(Fn *fn)
{
    Blk *b;

    for (b=fn->start; b; b=b->link)
        b->loop = 1;
    loopiter(fn, multloop);
}

static void
uffind(Blk **pb, Blk **uf)
{
    Blk **pb1;

    pb1 = &uf[(*pb)->id];
    if (*pb1) {
        uffind(pb1, uf);
        *pb = *pb1;
    }
}

/* requires rpo and no phis, breaks cfg */
void
simpljmp(Fn *fn)
{

    Blk **uf; /* union-find */
    Blk **p, *b, *ret;

    ret = newblk();
    ret->id = fn->nblk++;
    ret->jmp.type = Jret0;
    uf = emalloc(fn->nblk * sizeof uf[0]);
    for (b=fn->start; b; b=b->link) {
        assert(!b->phi);
        if (b->jmp.type == Jret0) {
            b->jmp.type = Jjmp;
            b->s1 = ret;
        }
        if (b->nins == 0)
        if (b->jmp.type == Jjmp) {
            uffind(&b->s1, uf);
            if (b->s1 != b)
                uf[b->id] = b->s1;
        }
    }
    for (p=&fn->start; (b=*p); p=&b->link) {
        if (b->s1)
            uffind(&b->s1, uf);
        if (b->s2)
            uffind(&b->s2, uf);
        if (b->s1 && b->s1 == b->s2) {
            b->jmp.type = Jjmp;
            b->s2 = 0;
        }
    }
    *p = ret;
    free(uf);
}
 */
