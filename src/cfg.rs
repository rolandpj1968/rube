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

fn rporec(blks: &mut [Blk], bi: BlkIdx, mut x: u32) -> u32 {
    if bi == BlkIdx::NONE || blks[bi].id != u32::MAX {
        return x;
    }

    let swap_s1_s2: bool = {
        let b: &Blk = &blks[bi];
        // TODO - check signedness of .loop_ - might need u32::MAX < 0
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
    let blks: &mut [Blk] = &mut f.blks;
    let phis: &mut [Phi] = &mut f.phis;

    blks.iter_mut().for_each(|b| b.id = u32::MAX);

    // Deliberately wraps from u32::MAX
    let n: u32 = rporec(blks, f.start, f.nblk - 1).wrapping_add(1);
    f.nblk -= n;
    f.rpo = vec![BlkIdx::NONE; f.nblk as usize];
    let mut prev_bi = BlkIdx::NONE;
    let mut bi = f.start;
    while bi != BlkIdx::NONE {
        if blks[bi].id == u32::MAX {
            // Unreachable Blk
            edgedel(blks, phis, bi, blks[bi].s1);
            edgedel(blks, phis, bi, blks[bi].s2);
            blks[prev_bi].link = blks[bi].link;
        } else {
            let b: &mut Blk = &mut blks[bi];
            b.id -= n;
            f.rpo[b.id as usize] = bi;
            prev_bi = bi;
        }
        bi = blks[bi].link;
    }
}

/* for dominators computation, read
 * "A Simple, Fast Dominance Algorithm"
 * by K. Cooper, T. Harvey, and K. Kennedy.
 */

fn inter(blks: &[Blk], mut bi1: BlkIdx, mut bi2: BlkIdx) -> BlkIdx {
    if bi1 == BlkIdx::NONE {
        return bi2;
    }
    while bi1 != bi2 {
        if blks[bi1].id < blks[bi2].id {
            (bi1, bi2) = (bi2, bi1);
        }
        while blks[bi1].id > blks[bi2].id {
            bi1 = blks[bi1].idom;
            assert!(bi1 != BlkIdx::NONE);
        }
    }
    bi1
}

pub fn filldom(f: &mut Fn) {
    let blks: &mut [Blk] = &mut f.blks;
    let rpo: &[BlkIdx] = &f.rpo;

    // TODO - live blocks only
    for b in blks.iter_mut() {
        b.idom = BlkIdx::NONE;
        b.dom = BlkIdx::NONE;
        b.dlink = BlkIdx::NONE;
    }
    loop {
        let mut ch: u32 = 0;
        for bi in rpo.iter().skip(1) {
            let b: &Blk = &blks[bi];
            let mut di: BlkIdx = BlkIdx::NONE;
            for pi in &b.preds {
                if blks[pi].idom != BlkIdx::NONE || *pi == f.start {
                    di = inter(blks, di, *pi);
                }
            }
            if di != b.idom {
                ch += 1;
                blks[bi].idom = di;
            }
        }

        if ch == 0 {
            break;
        }
    }
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let di: BlkIdx = blks[bi].idom;
        if di != BlkIdx::NONE {
            assert!(di != bi);
            let ddomi = blks[di].dom;
            blks[bi].dlink = ddomi;
            blks[di].dom = bi;
        }

        bi = blks[bi].link;
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
