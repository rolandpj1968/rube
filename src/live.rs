use std::io::stdout;

use crate::all::{
    bshas, kbase, to_s, BSet, Blk, BlkIdx, Fn, Ins, Mem, Phi, PhiIdx, Ref, Target, Tmp, TmpIdx, O,
};

use crate::util::{bsclr, bscopy, bscount, bsequal, bsinit, bsiter, bsset, bsunion, dumpts};

// Ugh, put phis on Blk
pub fn liveon(blks: &mut [Blk], phis: &[Phi], v: &mut BSet, bi: BlkIdx, si: BlkIdx) {
    bscopy(v, &blks[si].in_);
    let mut pi: PhiIdx = blks[si].phi;
    while pi != PhiIdx::NONE {
        let p: &Phi = &phis[pi];
        if let Ref::RTmp(ti) = p.to {
            bsclr(v, ti.usize());
        }
        pi = phis[pi].link;
    }
    let mut pi = blks[si].phi;
    while pi != PhiIdx::NONE {
        assert!(phis[pi].args.len() == phis[pi].blks.len());
        for a in 0..phis[pi].args.len() {
            if phis[pi].blks[a] == bi {
                if let Ref::RTmp(ati) = phis[pi].args[a] {
                    bsset(v, ati.usize());
                    bsset(&mut blks[bi].gen, ati.usize());
                }
            }
        }
        pi = phis[pi].link;
    }
}

fn bset(blks: &mut [Blk], tmps: &[Tmp], r: Ref, bi: BlkIdx, nlv: &mut [u32; 2]) {
    if let Ref::RTmp(ti) = r {
        bsset(&mut blks[bi].gen, ti.usize());
        if !bshas(&blks[bi].in_, ti.usize()) {
            nlv[kbase(tmps[ti].cls) as usize] += 1;
            bsset(&mut blks[bi].in_, ti.usize());
        }
    }
}

/* liveness analysis
 * requires rpo computation
 */
pub fn filllive(f: &mut Fn, targ: &Target) {
    let blks: &mut [Blk] = &mut f.blks;
    let phis: &[Phi] = &f.phis;
    let rpo: &[BlkIdx] = &f.rpo;
    let tmps: &[Tmp] = &f.tmps;
    let mems: &[Mem] = &f.mems;

    let ntmps: usize = tmps.len();
    let mut u: BSet = bsinit(ntmps);
    let mut v: BSet = bsinit(ntmps);

    // TODO - just iterate over live blks
    {
        let mut bi: BlkIdx = f.start;
        while bi != BlkIdx::NONE {
            let b: &mut Blk = &mut blks[bi];
            b.in_ = bsinit(ntmps);
            b.out = bsinit(ntmps);
            b.gen = bsinit(ntmps);
            bi = b.link;
        }
    }
    let mut chg: bool = true;

    loop {
        for n in (0..f.rpo.len()).rev() {
            let bi: BlkIdx = f.rpo[n];
            bscopy(&mut u, &blks[bi].out);
            let (s1, s2) = blks[bi].s1_s2();
            if s1 != BlkIdx::NONE {
                liveon(blks, phis, &mut v, bi, s1);
                bsunion(&mut blks[bi].out, &v);
            }
            if s2 != BlkIdx::NONE {
                liveon(blks, phis, &mut v, bi, s2);
                bsunion(&mut blks[bi].out, &v);
            }
            chg = chg || !bsequal(&blks[bi].out, &u);

            let mut nlv: [u32; 2] = [0; 2];
            {
                let b: &mut Blk = &mut blks[bi];
                b.out[0] |= targ.rglob;
                bscopy(&mut b.in_, &b.out);
            }

            {
                let mut ti: usize = 0;
                while bsiter(&blks[bi].in_, &mut ti) {
                    nlv[kbase(tmps[ti].cls) as usize] += 1;
                    ti += 1;
                }
            }

            {
                let jmp_arg: Ref = blks[bi].jmp.arg; // Copying...
                if let Ref::RCall(_) = jmp_arg {
                    let b: &mut Blk = &mut blks[bi];
                    assert!(bscount(&b.in_) == targ.nrglob && b.in_[0] == targ.rglob);
                    b.in_[0] |= (targ.retregs)(jmp_arg, &nlv); // TODO not implemented
                } else {
                    bset(blks, tmps, jmp_arg, bi, &mut nlv);
                }
            }

            blks[bi].nlive.copy_from_slice(&nlv);
            for ii in (0..blks[bi].ins.len()).rev() {
                let (op, to, arg0, arg1) = {
                    let i: &Ins = &blks[bi].ins[ii];
                    (i.op, i.to, i.args[0], i.args[1])
                };
                if op == O::Ocall {
                    if let Ref::RCall(_) = arg1 {
                        let mut m: [u32; 2] = [0; 2];
                        blks[bi].in_[0] &= (targ.retregs)(arg1, &mut m);
                        for k in 0..2 {
                            nlv[k] -= m[k];
                            /* caller-save registers are used
                             * by the callee, in that sense,
                             * right in the middle of the call,
                             * they are live: */
                            nlv[k] += targ.nrsave[k];
                            if nlv[k] > blks[bi].nlive[k] {
                                blks[bi].nlive[k] = nlv[k];
                            }
                        }
                        blks[bi].in_[0] |= (targ.argregs)(arg1, &mut m);
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
                    if bshas(&blks[bi].in_, ti.usize()) {
                        nlv[kbase(tmps[ti].cls) as usize] -= 1;
                    }
                    bsset(&mut blks[bi].gen, ti.usize());
                    bsclr(&mut blks[bi].in_, ti.usize());
                }
                for k in 0..2 {
                    let argk: Ref = [arg0, arg1][k];
                    match argk {
                        Ref::RMem(ma) => {
                            let (base, index) = {
                                let mem: &Mem = &mems[ma];
                                (mem.base, mem.index)
                            };
                            bset(blks, tmps, base, bi, &mut nlv);
                            bset(blks, tmps, index, bi, &mut nlv);
                        }
                        _ => {
                            bset(blks, tmps, argk, bi, &mut nlv);
                        }
                    }
                }
                for k in 0..2 {
                    if nlv[k] > blks[bi].nlive[k] {
                        blks[bi].nlive[k] = nlv[k];
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
            let b: &Blk = &blks[bi];
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
