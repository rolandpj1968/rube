use crate::all::{Blk, BlkIdx, Fn, Phi, PhiIdx};

/*
#include "all.h"

Blk *
newblk()
{
    static Blk z;
    Blk *b;

    b = alloc(sizeof *b);
    *b = z;
    return b;
}
 */

fn edgedel(f: &mut Fn, bsi: BlkIdx, bdi: BlkIdx) {
    if bdi == BlkIdx::INVALID {
        return;
    }
    {
        let b: &mut Blk = f.blk_mut(bsi);
        if b.s1 == bdi {
            b.s1 = BlkIdx::INVALID;
        }
        if b.s2 == bdi {
            b.s2 = BlkIdx::INVALID;
        }
    }
    let mut pi: PhiIdx = f.blk(bdi).phi;
    while pi != PhiIdx::INVALID {
        let p: &mut Phi = f.phi_mut(pi);
        let mut a: usize = 0;
        while p.blks[a] != bsi {
            assert!(a + 1 < p.blks.len());
            a += 1;
        }
        p.blks.remove(a);
        p.args.remove(a);

        pi = p.link;
    }
    let bd: &mut Blk = f.blk_mut(bdi);
    if !bd.preds.is_empty() {
        let mut a: usize = 0;
        while bd.preds[a] != bsi {
            assert!(a + 1 < bd.preds.len());
            a += 1;
        }
        bd.preds.remove(a);
    }
}

pub fn fillpreds(f: &mut Fn) {
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::INVALID {
        let b: &mut Blk = f.blk_mut(bi);
        b.preds.clear();
        bi = b.link;
    }
    bi = f.start;
    while bi != BlkIdx::INVALID {
        let (s1, s2) = f.blk(bi).s1_s2();
        if s1 != BlkIdx::INVALID {
            f.blk_mut(s1).preds.push(bi);
        }
        if s2 != BlkIdx::INVALID && s1 != s2 {
            f.blk_mut(s2).preds.push(bi);
        }
        bi = f.blk(bi).link;
    }
}

fn rporec(f: &mut Fn, bi: BlkIdx, mut x: u32) -> u32 {
    if bi == BlkIdx::INVALID || f.blk(bi).id != u32::MAX {
        return x;
    }

    f.blk_mut(bi).id = 1;

    let (mut s1, mut s2) = f.blk(bi).s1_s2();
    if s1 != BlkIdx::INVALID && s2 != BlkIdx::INVALID && f.blk(s1).loop_ > f.blk(s2).loop_ {
        (s1, s2) = (s2, s1);
    }

    x = rporec(f, s1, x);
    x = rporec(f, s2, x);
    assert!(x != u32::MAX);

    f.blk_mut(bi).id = x;

    // Deliberately wraps to u32:MAX
    x.wrapping_sub(1)
}

/* fill the reverse post-order (rpo) information */
pub fn fillrpo(f: &mut Fn) {
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::INVALID {
        let b: &mut Blk = f.blk_mut(bi);
        b.id = u32::MAX;
        bi = b.link;
    }

    // Deliberately wraps from u32::MAX
    let n: u32 = rporec(f, f.start, f.nblk - 1).wrapping_add(1);
    f.nblk -= n;
    f.rpo = vec![BlkIdx::INVALID; f.nblk as usize];
    let mut prev_bi = BlkIdx::INVALID;
    let mut bi = f.start;
    while bi != BlkIdx::INVALID {
        let (id, s1, s2, next_bi) = {
            let b: &Blk = f.blk(bi);
            (b.id, b.s1, b.s2, b.link)
        };
        if id == u32::MAX {
            // Unreachable Blk
            edgedel(f, bi, s1);
            edgedel(f, bi, s2);
            f.set_blk_link(prev_bi, next_bi);
            bi = next_bi;
        } else {
            let (rpo_idx, next_bi) = {
                let b: &mut Blk = f.blk_mut(bi);
                b.id -= n;
                (b.id, b.link)
            };
            f.rpo[rpo_idx as usize] = bi;
            prev_bi = bi;
            bi = next_bi;
        }
    }
}

/* for dominators computation, read
 * "A Simple, Fast Dominance Algorithm"
 * by K. Cooper, T. Harvey, and K. Kennedy.
 */

fn inter(f: &Fn, mut bi1: BlkIdx, mut bi2: BlkIdx) -> BlkIdx {
    if bi1 == BlkIdx::INVALID {
        return bi2;
    }

    while bi1 != bi2 {
        if f.blk(bi1).id < f.blk(bi2).id {
            (bi1, bi2) = (bi2, bi1);
        }
        while f.blk(bi1).id > f.blk(bi2).id {
            bi1 = f.blk(bi1).idom;
            assert!(bi1 != BlkIdx::INVALID);
        }
    }
    return bi1;
}

pub fn filldom(f: &mut Fn) {
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::INVALID {
        let b: &mut Blk = f.blk_mut(bi);
        b.idom = BlkIdx::INVALID;
        b.dom = BlkIdx::INVALID;
        b.dlink = BlkIdx::INVALID;

        bi = b.link;
    }
    loop {
        let mut ch: u32 = 0;
        for n in 1..f.rpo.len() {
            bi = f.rpo[n];
            let mut di: BlkIdx = BlkIdx::INVALID;
            for p in 0..f.blk(bi).preds.len() {
                let b: &Blk = f.blk(bi);
                if f.blk(b.preds[p]).idom != BlkIdx::INVALID || b.preds[p] == f.start {
                    di = inter(f, di, b.preds[p]);
                }
            }
            if di != f.blk(bi).idom {
                ch += 1;
                f.blk_mut(bi).idom = di;
            }
        }

        if ch == 0 {
            break;
        }
    }
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::INVALID {
        let di: BlkIdx = f.blk(bi).idom;
        if di != BlkIdx::INVALID {
            assert!(di != bi);
            let ddomi = f.blk(di).dom;
            f.blk_mut(bi).dlink = ddomi;
            f.blk_mut(di).dom = bi;
        }

        bi = f.blk(bi).link;
    }
}

pub fn sdom(f: &Fn, b1i: BlkIdx, mut b2i: BlkIdx) -> bool {
    assert!(b1i != BlkIdx::INVALID && b2i != BlkIdx::INVALID);
    if b1i == b2i {
        return false;
    }
    while f.blk(b2i).id > f.blk(b1i).id {
        b2i = f.blk(b2i).idom;
    }
    b1i == b2i
}

pub fn dom(f: &Fn, b1i: BlkIdx, b2i: BlkIdx) -> bool {
    b1i == b2i || sdom(f, b1i, b2i)
}

pub fn addfron(f: &mut Fn, ai: BlkIdx, bi: BlkIdx) {
    for froni in &f.blk(ai).frons {
        if *froni == bi {
            return;
        }
    }

    f.blk_mut(ai).frons.push(bi);
}

/* fill the dominance frontier */
pub fn fillfron(f: &mut Fn) {
    let mut bi = f.start;
    while bi != BlkIdx::INVALID {
        let b: &mut Blk = f.blk_mut(bi);
        b.frons.clear();
        bi = b.link;
    }
    bi = f.start;
    while bi != BlkIdx::INVALID {
        let (s1, s2) = f.blk(bi).s1_s2();
        if s1 != BlkIdx::INVALID {
            let mut ai = bi;
            while !sdom(f, ai, s1) {
                addfron(f, ai, s1);
                ai = f.blk(ai).idom;
            }
        }
        if s2 != BlkIdx::INVALID {
            let mut ai = bi;
            while !sdom(f, ai, s2) {
                addfron(f, ai, s2);
                ai = f.blk(ai).idom;
            }
        }
        bi = f.blk(bi).link;
    }
}

/*
static void
loopmark(Blk *hd, Blk *b, void f(Blk *, Blk *))
{
    uint p;

    if (b->id < hd->id || b->visit == hd->id)
        return;
    b->visit = hd->id;
    f(hd, b);
    for (p=0; p<b->npred; ++p)
        loopmark(hd, b->pred[p], f);
}

void
loopiter(Fn *fn, void f(Blk *, Blk *))
{
    uint n, p;
    Blk *b;

    for (b=fn->start; b; b=b->link)
        b->visit = -1u;
    for (n=0; n<fn->nblk; ++n) {
        b = fn->rpo[n];
        for (p=0; p<b->npred; ++p)
            if (b->pred[p]->id >= n)
                loopmark(b, b->pred[p], f);
    }
}

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
