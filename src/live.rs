use std::io::stdout;

use crate::all::{
    bshas, kbase, to_s, BSet, Blk, BlkIdx, Fn, Ins, Mem, Phi, PhiIdx, Ref, Target, TmpIdx, O,
};

use crate::util::{bsclr, bscopy, bscount, bsequal, bsinit, bsiter, bsset, bsunion, dumpts};

pub fn liveon(f: &mut Fn, v: &mut BSet, bi: BlkIdx, si: BlkIdx) {
    bscopy(v, &f.blk(si).in_);
    let mut pi: PhiIdx = f.blk(si).phi;
    while pi != PhiIdx::NONE {
        let p: &Phi = f.phi(pi);
        if let Ref::RTmp(ti) = p.to {
            bsclr(v, ti.0);
        }
        pi = f.phi(pi).link;
    }
    let mut pi = f.blk(si).phi;
    while pi != PhiIdx::NONE {
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
    while bi != BlkIdx::NONE {
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
            if s1 != BlkIdx::NONE {
                liveon(f, &mut v, bi, s1);
                bsunion(&mut f.blk_mut(bi).out, &v);
            }
            if s2 != BlkIdx::NONE {
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
                    ti += 1;
                }
            }

            {
                let jmp_arg: Ref = f.blk(bi).jmp.arg; // Copying...
                if let Ref::RCall(_) = jmp_arg {
                    let b: &mut Blk = f.blk_mut(bi);
                    assert!(bscount(&b.in_) == targ.nrglob && b.in_[0] == targ.rglob);
                    b.in_[0] |= (targ.retregs)(jmp_arg, &nlv); // TODO not implemented
                } else {
                    bset(f, jmp_arg, bi, &mut nlv);
                }
            }

            f.blk_mut(bi).nlive.copy_from_slice(&nlv);
            for ii in (0..f.blk(bi).ins.len()).rev() {
                let (op, to, arg0, arg1) = {
                    let i: &Ins = &f.blk(bi).ins[ii];
                    (i.op, i.to, i.args[0], i.args[1])
                };
                if op == O::Ocall {
                    if let Ref::RCall(_) = arg1 {
                        let mut m: [u32; 2] = [0; 2];
                        f.blk_mut(bi).in_[0] &= (targ.retregs)(arg1, &mut m);
                        for k in 0..2 {
                            nlv[k] -= m[k];
                            /* caller-save registers are used
                             * by the callee, in that sense,
                             * right in the middle of the call,
                             * they are live: */
                            nlv[k] += targ.nrsave[k];
                            if nlv[k] > f.blk(bi).nlive[k] {
                                f.blk_mut(bi).nlive[k] = nlv[k];
                            }
                        }
                        f.blk_mut(bi).in_[0] |= (targ.argregs)(arg1, &mut m);
                        for k in 0..2 {
                            nlv[k] -= targ.nrsave[k];
                            nlv[k] += m[k];
                        }
                    }
                }
                if to != Ref::R {
                    let ti: TmpIdx = {
                        if let Ref::RTmp(ti0) = to {
                            ti0
                        } else {
                            // to MUST be R or RTmp
                            assert!(false);
                            TmpIdx::NONE
                        }
                    };
                    //t = i->to.val;
                    if bshas(&f.blk(bi).in_, ti.0) {
                        nlv[kbase(f.tmp(ti).cls) as usize] -= 1;
                    }
                    bsset(&mut f.blk_mut(bi).gen, ti.0);
                    bsclr(&mut f.blk_mut(bi).in_, ti.0);
                }
                for k in 0..2 {
                    let argk: Ref = [arg0, arg1][k];
                    match argk {
                        Ref::RMem(ma) => {
                            let (base, index) = {
                                let mem: &Mem = f.mem(ma);
                                (mem.base, mem.index)
                            };
                            bset(f, base, bi, &mut nlv);
                            bset(f, index, bi, &mut nlv);
                        }
                        _ => {
                            bset(f, argk, bi, &mut nlv);
                        }
                    }
                }
                for k in 0..2 {
                    if nlv[k] > f.blk(bi).nlive[k] {
                        f.blk_mut(bi).nlive[k] = nlv[k];
                    }
                }
            }
        }
        if chg {
            chg = false;
        } else {
            break;
        }
    }

    if true
    /*TODO debug['L']*/
    {
        /*e*/
        println!("\n> Liveness analysis:");
        let mut bi = f.start;
        while bi != BlkIdx::NONE {
            let b: &Blk = f.blk(bi);
            /*e*/
            print!("\t{:<10}in:   ", to_s(&b.name));
            dumpts(&b.in_, &f.tmps, &mut stdout() /*stderr*/);
            /*e*/
            print!("\t          out:  ");
            dumpts(&b.out, &f.tmps, &mut stdout() /*stderr*/);
            /*e*/
            print!("\t          gen:  ");
            dumpts(&b.gen, &f.tmps, &mut stdout() /*stderr*/);
            /*e*/
            print!("\t          live: ");
            /*e*/
            println!("{} {}", b.nlive[0], b.nlive[1]);

            bi = b.link;
        }
    }
}
