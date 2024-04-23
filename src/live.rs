use std::io::stdout;

use crate::all::{
    bshas, kbase, to_s, BSet, Blk, BlkIdx, Fn, Ins, Mem, Phi, PhiIdx, Ref, Target, Tmp, TmpIdx, O,
};

use crate::util::{bsclr, bscopy, bscount, bsequal, bsinit, bsiter, bsset, bsunion, dumpts};

// Ugh, put phis on Blk
fn liveon(blks: &mut [Blk], phis: &[Phi], v: &mut BSet, bi: BlkIdx, si: BlkIdx) {
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

fn bset(tmps: &[Tmp], r: Ref, b: &mut Blk, nlv: &mut [u32; 2]) {
    if let Ref::RTmp(ti) = r {
        bsset(&mut b.gen, ti.usize());
        if !bshas(&b.in_, ti.usize()) {
            nlv[kbase(tmps[ti].cls) as usize] += 1;
            bsset(&mut b.in_, ti.usize());
        }
    }
}

fn add_liveon_succ_out(blks: &mut [Blk], phis: &[Phi], v: &mut BSet, bi: BlkIdx, si: BlkIdx) {
    if si != BlkIdx::NONE {
        liveon(blks, phis, v, bi, si);
        bsunion(&mut blks[bi].out, v);
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

    let mut u: BSet = bsinit(tmps.len());
    let mut v: BSet = bsinit(tmps.len());

    // TODO - just iterate over live blks with for_each()
    {
        let mut bi: BlkIdx = f.start;
        while bi != BlkIdx::NONE {
            let b: &mut Blk = &mut blks[bi];
            b.in_ = bsinit(tmps.len());
            b.out = bsinit(tmps.len());
            b.gen = bsinit(tmps.len());
            bi = b.link;
        }
    }
    let mut chg: bool = true;

    loop {
        for n in (0..rpo.len()).rev() {
            let bi: BlkIdx = rpo[n];
            bscopy(&mut u, &blks[bi].out);
            // Ugh, crying for succs iter
            add_liveon_succ_out(blks, phis, &mut v, bi, blks[bi].s1);
            add_liveon_succ_out(blks, phis, &mut v, bi, blks[bi].s2);
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
                    bset(tmps, jmp_arg, &mut blks[bi], &mut nlv);
                }
            }

            blks[bi].nlive.copy_from_slice(&nlv);
            for ii in (0..blks[bi].ins.len()).rev() {
                let i: Ins = blks[bi].ins[ii]; // Note, copy
                if i.op == O::Ocall {
                    if let Ref::RCall(_) = i.args[1] {
                        let mut m: [u32; 2] = [0; 2];
                        blks[bi].in_[0] &= (targ.retregs)(i.args[1], &mut m);
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
                        blks[bi].in_[0] |= (targ.argregs)(i.args[1], &mut m);
                        for k in 0..2 {
                            nlv[k] -= targ.nrsave[k];
                            nlv[k] += m[k];
                        }
                    }
                }
                match i.to {
                    Ref::R => (),
                    Ref::RTmp(ti) => {
                        if bshas(&blks[bi].in_, ti.usize()) {
                            nlv[kbase(tmps[ti].cls) as usize] -= 1;
                        }
                        bsset(&mut blks[bi].gen, ti.usize());
                        bsclr(&mut blks[bi].in_, ti.usize());
                    }
                    _ => {
                        // i.to MUST be R or RTmp
                        assert!(false);
                    }
                }
                for k in 0..2 {
                    match i.args[k] {
                        Ref::RMem(ma) => {
                            let (base, index) = {
                                let mem: &Mem = &mems[ma];
                                (mem.base, mem.index)
                            };
                            bset(tmps, base, &mut blks[bi], &mut nlv);
                            bset(tmps, index, &mut blks[bi], &mut nlv);
                        }
                        _ => {
                            bset(tmps, i.args[k], &mut blks[bi], &mut nlv);
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
