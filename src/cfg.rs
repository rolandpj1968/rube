use crate::all::{Blk, BlkIdx, Fn, Phi, PhiIdx};

// Not pretty - would be better if s1, s2 were [BlkIndex; 2]
fn succsdel(b: &mut Blk, bdi: BlkIdx) {
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

fn preddel(b: &mut Blk, bsi: BlkIdx) {
    if let Some(a) = b.preds.iter().position(|pbi| *pbi == bsi) {
        b.preds.remove(a);
    }
}

fn edgedel(blks: &mut [Blk], phis: &mut [Phi], bsi: BlkIdx, bdi: BlkIdx) {
    if bdi != BlkIdx::NONE {
        succsdel(&mut blks[bsi], bdi);
        phisdel(phis, blks[bdi].phi, bsi);
        preddel(&mut blks[bdi], bsi);
    }
}

pub fn fillpreds(f: &mut Fn) {
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let b: &mut Blk = f.blk_mut(bi);
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

fn rporec(blks: &mut Vec<Blk>, bi: BlkIdx, mut x: u32) -> u32 {
    if bi == BlkIdx::NONE || blks[bi].id != u32::MAX {
        return x;
    }

    let swap_s1_s2: bool = {
        let b: &Blk = &blks[bi];
        b.s1 != BlkIdx::NONE && b.s2 != BlkIdx::NONE && blks[b.s1].loop_ > blks[b.s2].loop_
    };
    if swap_s1_s2 {
        (blks[bi].s1, blks[bi].s2) = (blks[bi].s2, blks[bi].s1);
    }

    blks[bi].id = 1;
    x = rporec(blks, blks[bi].s1, x);
    x = rporec(blks, blks[bi].s2, x);
    assert!(x != u32::MAX);

    blks[bi].id = x;

    // Deliberately wraps to u32:MAX
    x.wrapping_sub(1)
}

/* fill the reverse post-order (rpo) information */
pub fn fillrpo(f: &mut Fn) {
    f.blks.iter_mut().for_each(|b| b.id = u32::MAX);

    // Deliberately wraps from u32::MAX
    let n: u32 = rporec(&mut f.blks, f.start, f.nblk - 1).wrapping_add(1);
    f.nblk -= n;
    f.rpo = vec![BlkIdx::NONE; f.nblk as usize];
    let mut prev_bi = BlkIdx::NONE;
    let mut bi = f.start;
    while bi != BlkIdx::NONE {
        let (id, s1, s2, next_bi) = {
            let b: &Blk = f.blk(bi);
            (b.id, b.s1, b.s2, b.link)
        };
        if id == u32::MAX {
            // Unreachable Blk
            // edgedel(f, bi, s1);
            // edgedel(f, bi, s2);
            edgedel(&mut f.blks, &mut f.phis, bi, s1);
            edgedel(&mut f.blks, &mut f.phis, bi, s2);
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
    if bi1 == BlkIdx::NONE {
        return bi2;
    }

    while bi1 != bi2 {
        if f.blk(bi1).id < f.blk(bi2).id {
            (bi1, bi2) = (bi2, bi1);
        }
        while f.blk(bi1).id > f.blk(bi2).id {
            bi1 = f.blk(bi1).idom;
            assert!(bi1 != BlkIdx::NONE);
        }
    }
    return bi1;
}

pub fn filldom(f: &mut Fn) {
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let b: &mut Blk = f.blk_mut(bi);
        b.idom = BlkIdx::NONE;
        b.dom = BlkIdx::NONE;
        b.dlink = BlkIdx::NONE;

        bi = b.link;
    }
    loop {
        let mut ch: u32 = 0;
        for n in 1..f.rpo.len() {
            bi = f.rpo[n];
            let mut di: BlkIdx = BlkIdx::NONE;
            for p in 0..f.blk(bi).preds.len() {
                let b: &Blk = f.blk(bi);
                if f.blk(b.preds[p]).idom != BlkIdx::NONE || b.preds[p] == f.start {
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
    while bi != BlkIdx::NONE {
        let di: BlkIdx = f.blk(bi).idom;
        if di != BlkIdx::NONE {
            assert!(di != bi);
            let ddomi = f.blk(di).dom;
            f.blk_mut(bi).dlink = ddomi;
            f.blk_mut(di).dom = bi;
        }

        bi = f.blk(bi).link;
    }
}

pub fn sdom(f: &Fn, b1i: BlkIdx, mut b2i: BlkIdx) -> bool {
    assert!(b1i != BlkIdx::NONE && b2i != BlkIdx::NONE);
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
    while bi != BlkIdx::NONE {
        let b: &mut Blk = f.blk_mut(bi);
        b.frons.clear();
        bi = b.link;
    }
    bi = f.start;
    while bi != BlkIdx::NONE {
        let (s1, s2) = f.blk(bi).s1_s2();
        if s1 != BlkIdx::NONE {
            let mut ai = bi;
            while !sdom(f, ai, s1) {
                addfron(f, ai, s1);
                ai = f.blk(ai).idom;
            }
        }
        if s2 != BlkIdx::NONE {
            let mut ai = bi;
            while !sdom(f, ai, s2) {
                addfron(f, ai, s2);
                ai = f.blk(ai).idom;
            }
        }
        bi = f.blk(bi).link;
    }
}

fn loopmark(f: &mut Fn, hdi: BlkIdx, bi: BlkIdx, func: fn(&mut Fn, BlkIdx, BlkIdx)) {
    {
        let hd: &Blk = f.blk(hdi);
        let b: &Blk = f.blk(bi);
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
        let b: &mut Blk = f.blk_mut(bi);
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
