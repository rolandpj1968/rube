use std::{borrow::BorrowMut, cell};

use crate::all::{Blk, BlkIdx, Blks, Fn, Phi, PhiIdx};

// Not pretty - would be better if s1, s2 were [BlkIndex; 2]
fn succsdel(mut b: cell::RefMut<Blk>, bdi: BlkIdx) {
    let mut succs = [&mut b.s1, &mut b.s2];
    for si in succs.iter_mut().filter(|si| ***si == bdi) {
        **si = BlkIdx::NONE;
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

fn preddel(b: cell::RefMut<Blk>, bsi: BlkIdx) {
    if let Some(a) = b.preds.iter().position(|pbi| *pbi == bsi) {
        b.preds.remove(a);
    }
}

fn edgedel(blks: &Blks, phis: &mut [Phi], bsi: BlkIdx, bdi: BlkIdx) {
    if bdi != BlkIdx::NONE {
        succsdel(blks.borrow_mut(bsi), bdi);
        phisdel(phis, blks.borrow_mut(bdi).phi, bsi);
        preddel(blks.borrow_mut(bdi), bsi);
    }
}

pub fn fillpreds(f: &Fn) {
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let mut b = f.blk_mut(bi);
        b.preds.clear();
        bi = b.link;
    }
    bi = f.start;
    while bi != BlkIdx::NONE {
        let (s1, s2) = f.blk(bi).s1_s2();
        if s1 != BlkIdx::NONE {
            f.blk_mut(s1).preds.push(bi);
        }
        if s2 != BlkIdx::NONE && s1 != s2 {
            f.blk_mut(s2).preds.push(bi);
        }
        bi = f.blk(bi).link;
    }
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

    let (s1, s2) = blks.with_mut(bi, |b| {
        if swap_succs {
            (b.s1, b.s2) = (b.s2, b.s1);
        }

        b.id = 1;

        (b.s1, b.s2)
    });

    x = rporec(blks, s1, x);
    x = rporec(blks, s2, x);
    assert!(x != u32::MAX);

    blks.with_mut(bi, |b| {
        b.id = x;
    });

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
        if blks.borrow(bi).id == u32::MAX {
            // Unreachable Blk
            edgedel(blks, phis, bi, blks.borrow(bi).s1);
            edgedel(blks, phis, bi, blks.borrow(bi).s2);
            blks.borrow_mut(prev_bi).link = blks.borrow(bi).link;
        } else {
            let b: cell::RefMut<Blk> = blks.borrow_mut(bi);
            b.id -= n;
            f.rpo[b.id as usize] = bi;
            prev_bi = bi;
        }
        bi = blks.borrow(bi).link;
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
        if blks.borrow(bi1).id < blks.borrow(bi2).id {
            (bi1, bi2) = (bi2, bi1);
        }
        while blks.borrow(bi1).id > blks.borrow(bi2).id {
            bi1 = blks.borrow(bi1).idom;
            assert!(bi1 != BlkIdx::NONE);
        }
    }
    bi1
}

pub fn filldom(f: &mut Fn) {
    let blks: &Blks = &f.blks;
    let rpo: &[BlkIdx] = &f.rpo;

    // TODO - live blocks only
    blks.for_each_mut(|b| {
        b.idom = BlkIdx::NONE;
        b.dom = BlkIdx::NONE;
        b.dlink = BlkIdx::NONE;
    });
    loop {
        let mut ch: u32 = 0;
        for bi in rpo.iter().skip(1) {
            let b = &blks.borrow(*bi);
            let mut di: BlkIdx = BlkIdx::NONE;
            for pi in &b.preds {
                if blks.borrow(*pi).idom != BlkIdx::NONE || *pi == f.start {
                    di = inter(blks, di, *pi);
                }
            }
            if di != b.idom {
                ch += 1;
                blks.borrow_mut(*bi).idom = di;
            }
        }

        if ch == 0 {
            break;
        }
    }
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let di: BlkIdx = blks.borrow(bi).idom;
        if di != BlkIdx::NONE {
            assert!(di != bi);
            let ddomi = blks.borrow(di).dom;
            blks.borrow_mut(bi).dlink = ddomi;
            blks.borrow_mut(di).dom = bi;
        }

        bi = blks.borrow(bi).link;
    }
}

pub fn sdom(blks: &Blks, b1i: BlkIdx, mut b2i: BlkIdx) -> bool {
    assert!(b1i != BlkIdx::NONE && b2i != BlkIdx::NONE);
    if b1i == b2i {
        return false;
    }
    while blks.borrow(b2i).id > blks.borrow(b1i).id {
        b2i = blks.borrow(b2i).idom;
    }
    b1i == b2i
}

pub fn dom(blks: &Blks, b1i: BlkIdx, b2i: BlkIdx) -> bool {
    b1i == b2i || sdom(blks, b1i, b2i)
}

fn addfron(a: cell::RefMut<Blk>, bi: BlkIdx) {
    if !a.frons.contains(&bi) {
        a.frons.push(bi);
    }
}

fn fillfron_for_succ(blks: &Blks, bi: BlkIdx, si: BlkIdx) {
    if si != BlkIdx::NONE {
        let mut ai = bi;
        while !sdom(blks, ai, si) {
            addfron(blks.borrow_mut(ai), si);
            ai = blks.borrow(ai).idom;
        }
    }
}

/* fill the dominance frontier */
pub fn fillfron(f: &mut Fn) {
    let blks: &Blks = &f.blks;

    // TODO live blks only (but it doesn't matter)
    blks.for_each_mut(|b| b.frons.clear());

    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        fillfron_for_succ(blks, bi, blks.borrow(bi).s1);
        fillfron_for_succ(blks, bi, blks.borrow(bi).s2);
        bi = blks.borrow(bi).link;
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
    for p in 0..f.blk(bi).preds.len() {
        let predi: BlkIdx = f.blk(bi).preds[p];
        loopmark(f, hdi, predi, func);
    }
}

pub fn loopiter(f: &mut Fn, func: fn(&mut Fn, BlkIdx, BlkIdx)) {
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let b: cell::RefMut<Blk> = f.blk_mut(bi);
        b.visit = u32::MAX;
        bi = b.link;
    }

    for n in 0..f.rpo.len() {
        let bi: BlkIdx = f.rpo[n];
        for p in 0..f.blk(bi).preds.len() {
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
