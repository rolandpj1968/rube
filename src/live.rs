use std::io::stdout;

use crate::all::{
    bshas, kbase, to_s, BSet, Blk, BlkIdx, Blks, Fn, Ins, Mem, Phi, PhiIdx, Ref, Target, Tmp, O,
};

use crate::util::{bsclr, bscopy, bscount, bsequal, bsinit, bsiter, bsset, bsunion, dumpts};

// Ugh, put phis on Blk
fn liveon(blks: &Blks, phis: &[Phi], v: &mut BSet, bi: BlkIdx, si: BlkIdx) {
    bscopy(v, &blks.borrow(si).in_);

    {
        let mut pi: PhiIdx = blks.phi_of(si);
        while pi != PhiIdx::NONE {
            let p: &Phi = &phis[pi];
            if let Ref::RTmp(ti) = p.to {
                bsclr(v, ti.usize());
            }
            pi = p.link;
        }
    }
    {
        let mut pi = blks.phi_of(si);
        while pi != PhiIdx::NONE {
            let p: &Phi = &phis[pi];
            assert!(p.args.len() == p.blks.len());
            for a in 0..p.args.len() {
                if p.blks[a] == bi {
                    if let Ref::RTmp(ati) = p.args[a] {
                        bsset(v, ati.usize());
                        blks.with_mut(bi, |b| bsset(&mut b.gen, ati.usize()));
                    }
                }
            }
            pi = p.link;
        }
    }
}

fn bset(tmps: &[Tmp], r: Ref, b: &mut Blk, nlv: &mut [u32; 2]) {
    if let Ref::RTmp(ti) = r {
        bsset(&mut b.gen, ti.usize());
        if !bshas(&b.in_, ti.usize()) {
            nlv[kbase(tmps[ti].cls) as usize] += 1;
            bsset(&mut b.in_, ti.usize());
        }
    }
}

/* liveness analysis
 * requires rpo computation
 */
pub fn filllive(f: &mut Fn, targ: &Target) {
    let blks: &Blks = &f.blks;
    let phis: &[Phi] = &f.phis;
    let rpo: &[BlkIdx] = &f.rpo;
    let tmps: &[Tmp] = &f.tmps;
    let mems: &[Mem] = &f.mems;

    let mut u: BSet = bsinit(tmps.len());
    let mut v: BSet = bsinit(tmps.len());

    blks.for_each_mut(|b| {
        b.in_ = bsinit(tmps.len());
        b.out = bsinit(tmps.len());
        b.gen = bsinit(tmps.len());
    });

    let mut chg: bool = true;
    loop {
        for n in (0..rpo.len()).rev() {
            let bi: BlkIdx = rpo[n];
            {
                let succs = blks.with_mut(bi, |b| {
                    bscopy(&mut u, &b.out);
                    b.succs()
                });
                succs.iter().for_each(|si| {
                    if *si != BlkIdx::NONE {
                        liveon(blks, phis, &mut v, bi, *si);
                        blks.with_mut(bi, |b| bsunion(&mut b.out, &v));
                    }
                });
                chg = chg || !bsequal(&blks.borrow(bi).out, &u);
            }

            let mut nlv: [u32; 2] = [0; 2];
            blks.with_mut(bi, |b| {
                b.out[0] |= targ.rglob;
                bscopy(&mut b.in_, &b.out);

                let mut ti: usize = 0;
                while bsiter(&b.in_, &mut ti) {
                    nlv[kbase(tmps[ti].cls) as usize] += 1;
                    ti += 1;
                }

                let jmp_arg = b.jmp().arg;
                if let Ref::RCall(_) = jmp_arg {
                    assert!(bscount(&b.in_) == targ.nrglob && b.in_[0] == targ.rglob);
                    b.in_[0] |= (targ.retregs)(jmp_arg, &nlv); // TODO not implemented
                } else {
                    bset(tmps, jmp_arg, b, &mut nlv);
                }

                b.nlive.copy_from_slice(&nlv);

                let ins_len = b.ins().len();
                for ii in (0..ins_len).rev() {
                    let i: Ins = b.ins()[ii]; // Note, copy
                    if i.op == O::Ocall {
                        if let Ref::RCall(_) = i.args[1] {
                            let mut m: [u32; 2] = [0; 2];
                            b.in_[0] &= (targ.retregs)(i.args[1], &mut m);
                            for k in 0..2 {
                                nlv[k] -= m[k];
                                /* caller-save registers are used
                                 * by the callee, in that sense,
                                 * right in the middle of the call,
                                 * they are live: */
                                nlv[k] += targ.nrsave[k];
                                if nlv[k] > b.nlive[k] {
                                    b.nlive[k] = nlv[k];
                                }
                            }
                            b.in_[0] |= (targ.argregs)(i.args[1], &mut m);
                            for k in 0..2 {
                                nlv[k] -= targ.nrsave[k];
                                nlv[k] += m[k];
                            }
                        }
                    }
                    assert!(i.to == Ref::R || matches!(i.to, Ref::RTmp(_)));
                    if let Ref::RTmp(ti) = i.to {
                        if bshas(&b.in_, ti.usize()) {
                            nlv[kbase(tmps[ti].cls) as usize] -= 1;
                        }
                        bsset(&mut b.gen, ti.usize());
                        bsclr(&mut b.in_, ti.usize());
                    }
                    for k in 0..2 {
                        match i.args[k] {
                            Ref::RMem(ma) => {
                                let (base, index) = {
                                    let mem: &Mem = &mems[ma];
                                    (mem.base, mem.index)
                                };
                                bset(tmps, base, b, &mut nlv);
                                bset(tmps, index, b, &mut nlv);
                            }
                            _ => {
                                bset(tmps, i.args[k], b, &mut nlv);
                            }
                        }
                    }
                    for k in 0..2 {
                        if nlv[k] > b.nlive[k] {
                            b.nlive[k] = nlv[k];
                        }
                    }
                }
            });
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
            let b: &Blk = &blks.borrow(bi);
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
