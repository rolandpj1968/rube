use crate::all::{for_each_bi_mut, for_each_blk_mut, Blk, BlkIdx, Fn, Phi, PhiIdx, RpoIdx};

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

pub fn edgedel(blks: &mut [Blk], phis: &mut [Phi], bsi: BlkIdx, bdi: BlkIdx) {
    if bdi != BlkIdx::NONE {
        succsdel(&mut blks[bsi], bdi);
        phisdel(phis, blks[bdi].phi, bsi);
        preddel(&mut blks[bdi], bsi);
    }
}

pub fn fillpreds(f: &Fn) {
    f.for_each_blk_mut(|b| b.preds.clear());
    let blks: &mut [Blk] = &mut f.blks;
    for_each_bi_mut(blks, |bi| {
        let succs = blks[bi].succs();
        succs.iter().for_each(|si| {
            if *si != BlkIdx::NONE {
                blks[si].preds.push(bi);
            }
        });
    });
}

fn rporec(blks: &mut [Blk], bi: BlkIdx, mut x: RpoIdx) -> RpoIdx {
    if bi == BlkIdx::NONE || blks[bi].id != RpoIdx::NONE {
        return x;
    }

    // Borrow immutably here cos s1, s2 could be same as bi
    let swap_succs: bool = {
        let b: &Blk = &blks[bi];
        // TODO - check signedness of .loop_ - might need u32::MAX < 0
        b.s1 != BlkIdx::NONE
            && b.s2 != BlkIdx::NONE
            && b.s1 != b.s2 // Not actually needed
            && blks[b.s1].loop_ > blks[b.s2].loop_
    };

    let succs = {
        let b: &mut Blk = &mut blks[bi];
        b.id = RpoIdx::from(1); // placeholder
        if swap_succs {
            (b.s1, b.s2) = (b.s2, b.s1);
        }
        b.succs()
    };
    succs.iter().for_each(|si| x = rporec(blks, *si, x));
    assert!(x != RpoIdx::NONE);

    blks[bi].id = x;

    // Deliberately wraps to u32:MAX
    x.prev()
}

// use crate::all::to_s;
// fn printlives(start: BlkIdx, blks: &[Blk]) {
//     //let blks = &f.blks;
//     println!("live blocks according to is_dead");
//     blks.for_each_bi(|bi| {
//         println!(
//             "    {:?} id {} {}",
//             bi,
//             blks[bi].id.0,
//             to_s(&blks.borrow(bi).name)
//         );
//     });
//     println!("live blocks according to link");
//     let mut bi = start;
//     while bi != BlkIdx::NONE {
//         println!(
//             "    {:?} id {} {}",
//             bi,
//             blks[bi].id.0,
//             to_s(&blks.borrow(bi).name)
//         );
//         bi = blks.borrow(bi).link;
//     }
// }

/* fill the reverse post-order (rpo) information */
pub fn fillrpo(f: &mut Fn) {
    let blks: &mut [Blk] = &mut f.blks;
    let rpo: &mut [BlkIdx] = &mut f.rpo;
    let phis: &mut [Phi] = &mut f.phis;

    for_each_blk_mut(blks, |b| b.id = RpoIdx::NONE);

    let mut x = RpoIdx::from((f.nblk as usize) - 1);
    // Deliberately wraps from u32::MAX
    x = rporec(blks, f.start, x);
    let n = x.next().usize();
    f.nblk -= n as u32;
    f.rpo = vec![BlkIdx::NONE; f.nblk as usize];
    let mut prev_bi = BlkIdx::NONE;
    for_each_bi_mut(blks, |bi| {
        let (id, succs, link) = (blks[bi].id, blks[bi].succs(), blks[bi].link);
        if id == RpoIdx::NONE {
            // Unreachable Blk
            succs.iter().for_each(|si| edgedel(blks, phis, bi, *si));
            blks[prev_bi].link = link;
            blks[bi].is_dead = true;
        } else {
            let b: &mut Blk = &mut blks[bi];
            b.id = RpoIdx::from(b.id.usize() - n);
            rpo[b.id] = bi;
            prev_bi = bi;
        }
    });
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

    for_each_blk_mut(blks, |b| {
        b.idom = BlkIdx::NONE;
        b.dom = BlkIdx::NONE;
        b.dlink = BlkIdx::NONE;
    });
    loop {
        let mut ch: u32 = 0;
        for bi in rpo.iter().skip(1) {
            let di = {
                let mut di0: BlkIdx = BlkIdx::NONE;
                for pi in &blks[bi].preds {
                    if blks[pi].idom != BlkIdx::NONE || *pi == BlkIdx::START {
                        di0 = inter(blks, di0, *pi);
                    }
                }
                di0
            };
            if di != blks[bi].idom {
                ch += 1;
                blks[bi].idom = di;
            }
        }
        if ch == 0 {
            break;
        }
    }
    for_each_bi_mut(blks, |bi| {
        let di: BlkIdx = blks[bi].idom;
        if di != BlkIdx::NONE {
            assert!(di != bi);
            blks[bi].dlink = blks[di].dom;
            blks[di].dom = bi;
        }
    });
}

pub fn sdom(blks: &[Blk], b1i: BlkIdx, mut b2i: BlkIdx) -> bool {
    assert!(b1i != BlkIdx::NONE && b2i != BlkIdx::NONE);
    if b1i == b2i {
        return false;
    }
    while blks[b2i].id > blks[b1i].id {
        b2i = blks[b2i].idom;
    }
    b1i == b2i
}

pub fn dom(blks: &[Blk], b1i: BlkIdx, b2i: BlkIdx) -> bool {
    b1i == b2i || sdom(blks, b1i, b2i)
}

fn addfron(a: &mut Blk, bi: BlkIdx) {
    if !a.frons.contains(&bi) {
        a.frons.push(bi);
    }
}

fn fillfron_for_succ(blks: &mut [Blk], bi: BlkIdx, si: BlkIdx) {
    if si != BlkIdx::NONE {
        let mut ai = bi;
        while !sdom(blks, ai, si) {
            addfron(&mut blks[ai], si);
            ai = blks[ai].idom;
        }
    }
}

/* fill the dominance frontier */
pub fn fillfron(f: &mut Fn) {
    f.for_each_blk_mut(|b| b.frons.clear());
    let blks: &mut [Blk] = &mut f.blks;
    for_each_bi_mut(blks, |bi| {
        let succs = blks[bi].succs();
        succs.iter().for_each(|si| fillfron_for_succ(blks, bi, *si));
    });
}

fn loopmark(blks: &mut [Blk], hdi: BlkIdx, bi: BlkIdx, func: fn(&mut [Blk], BlkIdx, BlkIdx)) {
    let (hd_id, b_id, b_visit) = (blks[hdi].id, blks[bi].id, blks[bi].visit);
    if b_id < hd_id || b_visit == hd_id {
        return;
    }
    blks[bi].visit = hd_id;
    func(blks, hdi, bi);
    let preds_len = blks[bi].preds.len();
    for p in 0..preds_len {
        let predi: BlkIdx = blks[bi].preds[p];
        loopmark(blks, hdi, predi, func);
    }
}

pub fn loopiter(blks: &mut [Blk], rpo: &[BlkIdx], func: fn(&mut [Blk], BlkIdx, BlkIdx)) {
    for_each_blk_mut(blks, |b| b.visit = RpoIdx::NONE);

    for n in 0..rpo.len() {
        let bi: BlkIdx = rpo[n];
        let preds_len = blks[bi].preds.len();
        for p in 0..preds_len {
            let predi: BlkIdx = blks[bi].preds[p];
            if blks[predi].id.usize() >= n {
                loopmark(blks, bi, predi, func);
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
