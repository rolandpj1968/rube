use crate::all::{bshas, kbase, BSet, Blk, BlkIdx, Fn, Phi, PhiIdx, Ref, Target, Tmp, TmpIdx};

use crate::util::{bsclr, bscopy, bscount, bsequal, bsinit, bsiter, bsset, bsunion};

pub fn liveon(f: &mut Fn, v: &mut BSet, bi: BlkIdx, si: BlkIdx) {
    bscopy(v, &f.blk(si).in_);
    let mut pi: PhiIdx = f.blk(si).phi;
    while pi != PhiIdx::INVALID {
        let p: &Phi = f.phi(pi);
        if let Ref::RTmp(ti) = p.to {
            bsclr(v, ti.0);
        }
        pi = f.phi(pi).link;
    }
    let mut pi = f.blk(si).phi;
    while pi != PhiIdx::INVALID {
        assert!(f.phi(pi).args.len() == f.phi(pi).blks.len());
        for a in 0..f.phi(pi).args.len() {
            if f.phi(pi).blks[a] == bi {
                if let Ref::RTmp(ati) = f.phi(pi).args[a] {
                    bsset(v, ati.0);
                    bsset(&mut f.blk_mut(bi).gen, ati.0);
                }
            }
        }
        pi = f.phi(pi).link;
    }
}

// Hrmm, tmps is f.tmps???
fn bset(f: &mut Fn, r: Ref, bi: BlkIdx, nlv: &mut [u32; 2]) {
    if let Ref::RTmp(ti) = r {
        bsset(&mut f.blk_mut(bi).gen, ti.0);
        if !bshas(&f.blk(bi).in_, ti.0) {
            nlv[kbase(f.tmp(ti).cls) as usize] += 1;
            bsset(&mut f.blk_mut(bi).in_, ti.0);
        }
    }
}

/* liveness analysis
 * requires rpo computation
 */
pub fn filllive(f: &mut Fn, targ: &Target) {
    // Blk *b;
    // Ins *i;
    // int k, t, m[2], n, chg, nlv[2];
    // BSet u[1], v[1];
    // Mem *ma;

    let ntmps: usize = f.tmps.len();
    let mut u: BSet = bsinit(ntmps);
    let mut v: BSet = bsinit(ntmps);

    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::INVALID {
        let b: &mut Blk = f.blk_mut(bi);
        b.in_ = bsinit(ntmps);
        b.out = bsinit(ntmps);
        b.gen = bsinit(ntmps);
        bi = b.link;
    }
    let mut chg: bool = true;

    loop {
        for n in (0..f.rpo.len()).rev() {
            let bi: BlkIdx = f.rpo[n];
            bscopy(&mut u, &f.blk(bi).out);
            let (s1, s2) = f.blk(bi).s1_s2();
            if s1 != BlkIdx::INVALID {
                liveon(f, &mut v, bi, s1);
                bsunion(&mut f.blk_mut(bi).out, &v);
            }
            if s2 != BlkIdx::INVALID {
                liveon(f, &mut v, bi, s2);
                bsunion(&mut f.blk_mut(bi).out, &v);
            }
            chg = chg || !bsequal(&f.blk(bi).out, &u);

            let mut nlv: [u32; 2] = [0; 2];
            {
                let b: &mut Blk = f.blk_mut(bi);
                b.out[0] |= targ.rglob;
                bscopy(&mut b.in_, &b.out);
            }
            {
                let mut ti: u32 = 0;
                while bsiter(&f.blk(bi).in_, &mut ti) {
                    nlv[kbase(f.tmp(TmpIdx(ti)).cls) as usize] += 1;
                }
            }
            {
                let jmp_arg: Ref = f.blk(bi).jmp.arg; // Copying...
                if let Ref::RCall(c) = jmp_arg {
                    let b: &mut Blk = f.blk_mut(bi);
                    assert!(bscount(&b.in_) == targ.nrglob && b.in_[0] == targ.rglob);
                    b.in_[0] |= (targ.retregs)(jmp_arg, &nlv); // TODO not implemented
                } else {
                    bset(f, jmp_arg, bi, &mut nlv);
                }
            }
            //         for (k=0; k<2; k++)
            //             b->nlive[k] = nlv[k];
            //         for (i=&b->ins[b->nins]; i!=b->ins;) {
            //             if ((--i)->op == Ocall && rtype(i->arg[1]) == RCall) {
            //                 b->in->t[0] &= ~T.retregs(i->arg[1], m);
            //                 for (k=0; k<2; k++) {
            //                     nlv[k] -= m[k];
            //                     /* caller-save registers are used
            //                      * by the callee, in that sense,
            //                      * right in the middle of the call,
            //                      * they are live: */
            //                     nlv[k] += T.nrsave[k];
            //                     if (nlv[k] > b->nlive[k])
            //                         b->nlive[k] = nlv[k];
            //                 }
            //                 b->in->t[0] |= T.argregs(i->arg[1], m);
            //                 for (k=0; k<2; k++) {
            //                     nlv[k] -= T.nrsave[k];
            //                     nlv[k] += m[k];
            //                 }
            //             }
            //             if (!req(i->to, R)) {
            //                 assert(rtype(i->to) == RTmp);
            //                 t = i->to.val;
            //                 if (bshas(b->in, t))
            //                     nlv[KBASE(f->tmp[t].cls)]--;
            //                 bsset(b->gen, t);
            //                 bsclr(b->in, t);
            //             }
            //             for (k=0; k<2; k++)
            //                 switch (rtype(i->arg[k])) {
            //                 case RMem:
            //                     ma = &f->mem[i->arg[k].val];
            //                     bset(ma->base, b, nlv, f->tmp);
            //                     bset(ma->index, b, nlv, f->tmp);
            //                     break;
            //                 default:
            //                     bset(i->arg[k], b, nlv, f->tmp);
            //                     break;
            //                 }
            //             for (k=0; k<2; k++)
            //                 if (nlv[k] > b->nlive[k])
            //                     b->nlive[k] = nlv[k];
            //         }
            //     }
        }
        if chg {
            chg = false;
        } else {
            break;
        }
    }

    //     if (debug['L']) {
    //         fprintf(stderr, "\n> Liveness analysis:\n");
    //         for (b=f->start; b; b=b->link) {
    //             fprintf(stderr, "\t%-10sin:   ", b->name);
    //             dumpts(b->in, f->tmp, stderr);
    //             fprintf(stderr, "\t          out:  ");
    //             dumpts(b->out, f->tmp, stderr);
    //             fprintf(stderr, "\t          gen:  ");
    //             dumpts(b->gen, f->tmp, stderr);
    //             fprintf(stderr, "\t          live: ");
    //             fprintf(stderr, "%d %d\n", b->nlive[0], b->nlive[1]);
    //         }
    //     }
}
