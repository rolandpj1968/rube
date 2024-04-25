use crate::all::{
    astack, bit, isload, isstore, Alias, AliasLoc, AliasT, AliasU, Bits, Blk, BlkIdx, Blks,
    CanAlias, Con, ConBits, ConT, Fn, Ins, Phi, PhiIdx, Ref, Tmp, TmpIdx, J, NBIT, O, OALLOC,
    OALLOC1,
};

use crate::load::storesz;

pub fn getalias(tmps: &[Tmp], cons: &[Con], a_in: &Alias, r: Ref) -> Alias {
    let mut a_out: Alias = *a_in;
    assert!(matches!(r, Ref::RTmp(_)) || matches!(r, Ref::RCon(_)));
    match r {
        Ref::RTmp(ti) => {
            a_out = tmps[ti].alias;
            if astack(a_out.typ) != 0 {
                a_out.typ = tmps[a_out.slot].alias.typ;
            }
            assert!(a_out.typ != AliasT::ABot);
        }
        Ref::RCon(ci) => {
            let c: &Con = &cons[ci.0 as usize];
            match c.type_ {
                ConT::CAddr => {
                    a_out.typ = AliasT::ASym;
                    a_out.u = AliasU::ASym(c.sym);
                }
                _ => {
                    a_out.typ = AliasT::ACon;
                }
            }
            // TODO - get this right
            //assert!(matches!(c.bits, ConBits::I(_)));
            if let ConBits::I(i) = c.bits {
                a_out.offset = i;
            } else {
                // Needed for CAddr where c.bits is None; ropy!
                // Hrmm, changed CAddr to have I(0) by default now, check again...
                a_out.offset = 0;
                //assert!(false);
            }
            a_out.slot = TmpIdx::NONE;
        }
        _ => (),
    }

    a_out
}

pub fn alias(f: &Fn, p: Ref, op: i32, sp: i32, q: Ref, sq: i32) -> (CanAlias, i32) {
    let tmps: &[Tmp] = &f.tmps;
    let cons: &[Con] = &f.cons;

    let mut ap: Alias = getalias(tmps, cons, &Alias::default(), p);
    let aq: Alias = getalias(tmps, cons, &Alias::default(), q);
    ap.offset += op as i64;
    /* when delta is meaningful (ovlap == 1),
     * we do not overflow int because sp and
     * sq are bounded by 2^28 */
    let delta: i32 = (ap.offset - aq.offset) as i32; // Hrmm, this seems dodgy
    let ovlap: bool = ap.offset < aq.offset + (sq as i64) && aq.offset < ap.offset + (sp as i64);

    let can_alias: CanAlias = {
        if astack(ap.typ) != 0 && astack(aq.typ) != 0 {
            /* if both are offsets of the same
             * stack slot, they alias iif they
             * overlap */
            if ap.base == aq.base && ovlap {
                CanAlias::Must
            } else {
                CanAlias::No
            }
        } else if ap.typ == AliasT::ASym && aq.typ == AliasT::ASym {
            /* they conservatively alias if the
             * symbols are different, or they
             * alias for sure if they overlap */
            assert!(matches!(ap.u, AliasU::ASym(_)) && matches!(aq.u, AliasU::ASym(_)));
            if let (AliasU::ASym(ap_sym), AliasU::ASym(aq_sym)) = (ap.u, aq.u) {
                //if let AliasU::ASym(aq_sym) = aq.u {
                match () {
                    () if ap_sym != aq_sym => CanAlias::May,
                    () if ovlap => CanAlias::Must,
                    _ => CanAlias::No,
                }
            } else {
                // unreachable
                CanAlias::May
            }
        } else if (ap.typ == AliasT::ACon && aq.typ == AliasT::ACon)
            || (ap.typ == aq.typ && ap.base == aq.base)
        {
            assert!(ap.typ == AliasT::ACon || ap.typ == AliasT::AUnk);
            /* if they have the same base, we
             * can rely on the offsets only */
            if ovlap {
                CanAlias::Must
            } else {
                CanAlias::No
            }
        } else if (ap.typ == AliasT::AUnk && aq.typ != AliasT::ALoc)
            || (aq.typ == AliasT::AUnk && ap.typ != AliasT::ALoc)
        {
            /* if one of the two is unknown
             * there may be aliasing unless
             * the other is provably local */
            CanAlias::May
        } else {
            CanAlias::No
        }
    };
    (can_alias, delta)
}

pub fn escapes(tmps: &[Tmp], r: Ref) -> bool {
    if let Ref::RTmp(ti) = r {
        let a: &Alias = &tmps[ti].alias;
        astack(a.typ) == 0 || tmps[a.slot].alias.typ == AliasT::AEsc
    } else {
        true
    }
}

fn esc(tmps: &mut [Tmp], r: Ref) {
    match r {
        Ref::RTmp(ti) => {
            let a: Alias = tmps[ti].alias; // Note, copy
            if astack(a.typ) != 0 {
                tmps[a.slot].alias.typ = AliasT::AEsc;
            }
        }
        Ref::R | Ref::RCon(_) | Ref::RInt(_) | Ref::RTyp(_) => (), /*ok*/
        _ => assert!(false),
    }
}

fn store(tmps: &mut [Tmp], r: Ref, sz: i32) {
    if let Ref::RTmp(ti) = r {
        let a: Alias = tmps[ti].alias; // Note, copy
        if a.slot != TmpIdx::NONE {
            assert!(astack(a.typ) != 0);
            let m: Bits = {
                if sz >= (NBIT as i32) || (a.offset < 0 || a.offset >= (NBIT as i64)) {
                    u64::MAX
                } else {
                    (bit(sz as usize) - 1) << a.offset
                }
            };
            let aslot: &mut Alias = &mut tmps[a.slot].alias;
            assert!(matches!(aslot.u, AliasU::ALoc(_)));
            if let AliasU::ALoc(loc) = &mut aslot.u {
                loc.m |= m;
            }
        }
    }
}

pub fn fillalias(f: &mut Fn) {
    let blks: &Blks = &f.blks;
    let rpo: &[BlkIdx] = &f.rpo;
    let tmps: &mut [Tmp] = &mut f.tmps;
    let cons: &[Con] = &f.cons;
    let phis: &[Phi] = &f.phis;

    tmps.iter_mut().for_each(|t| t.alias = Alias::default());

    for n in 0..f.nblk {
        let bi: BlkIdx = rpo[n as usize];
        let b = blks.borrow(bi);
        let mut pi: PhiIdx = b.phi; //blks.phi_of(bi);
        while pi != PhiIdx::NONE {
            let p: &Phi = &phis[pi];
            assert!(matches!(p.to, Ref::RTmp(_)));
            if let Ref::RTmp(ti) = p.to {
                let a: &mut Alias = &mut tmps[ti].alias;
                assert!(a.typ == AliasT::ABot);
                a.typ = AliasT::AUnk;
                a.base = ti;
                a.offset = 0;
                a.slot = TmpIdx::NONE;
            }
            pi = p.link;
        }
        let ins = b.ins();
        for ii in 0..ins.len() {
            let i: &Ins = &ins[ii];

            if i.op == O::Oblit1 {
                // Already handled as part of preceding Oblit0
                continue;
            }

            let mut ai: TmpIdx = TmpIdx::NONE;

            assert!(i.to == Ref::R || matches!(i.to, Ref::RTmp(_)));
            if let Ref::RTmp(ti) = i.to {
                ai = ti;
                let a: &mut Alias = &mut tmps[ti].alias;
                // TODO isalloc()
                assert!(a.typ == AliasT::ABot);
                if OALLOC <= i.op && i.op <= OALLOC1 {
                    a.typ = AliasT::ALoc;
                    a.slot = ti;
                    let mut sz: i32 = -1;
                    if let Ref::RCon(ci) = i.args[0] {
                        let c: &Con = &cons[ci.0 as usize];
                        assert!(matches!(c.bits, ConBits::I(_)));
                        if let ConBits::I(sz0) = c.bits {
                            if c.type_ == ConT::CBits && (0 <= sz0 && sz0 <= NBIT as i64) {
                                sz = sz0 as i32;
                            }
                        }
                    }
                    a.u = AliasU::ALoc(AliasLoc { sz, m: 0 });
                } else {
                    a.typ = AliasT::AUnk;
                    a.slot = TmpIdx::NONE;
                    a.u = AliasU::None;
                }
                a.base = ti;
                a.offset = 0;
            }
            if i.op == O::Ocopy {
                assert!(ai != TmpIdx::NONE);
                let a0: Alias = getalias(tmps, cons, &tmps[ai].alias, i.args[0]);
                tmps[ai].alias = a0;
            }
            // TODO - why not Osub too? QBE question
            if i.op == O::Oadd {
                assert!(ai != TmpIdx::NONE);
                let a0: Alias = getalias(tmps, cons, &Alias::default(), i.args[0]);
                let a1: Alias = getalias(tmps, cons, &Alias::default(), i.args[1]);
                if a0.typ == AliasT::ACon {
                    tmps[ai].alias = a1;
                    tmps[ai].alias.offset += a0.offset;
                } else if a1.typ == AliasT::ACon {
                    tmps[ai].alias = a0;
                    tmps[ai].alias.offset += a1.offset;
                }
            }
            if (i.to == Ref::R || tmps[ai].alias.typ == AliasT::AUnk) && i.op != O::Oblit0 {
                if !isload(i.op) {
                    esc(tmps, i.args[0]);
                }
                if !isstore(i.op) && i.op != O::Oargc {
                    esc(tmps, i.args[1]);
                }
            }
            if i.op == O::Oblit0 {
                assert!(ii < ins.len() - 1);
                let blit1 = &ins[ii + 1];
                assert!(blit1.op == O::Oblit1);
                assert!(matches!(blit1.args[0], Ref::RInt(_)));
                if let Ref::RInt(blit1_i) = blit1.args[0] {
                    store(tmps, i.args[1], blit1_i.abs());
                }
            }
            if isstore(i.op) {
                store(tmps, i.args[1], storesz(&i));
            }
        }
        if b.jmp().type_ != J::Jretc {
            let jmp_arg = b.jmp().arg;
            esc(tmps, jmp_arg);
        }
    }
    blks.for_each_bi(|bi| {
        let mut pi: PhiIdx = blks.phi_of(bi);
        while pi != PhiIdx::NONE {
            let p: &Phi = &phis[pi];
            p.args.iter().for_each(|arg| esc(tmps, *arg));
            pi = p.link;
        }
    });

    // println!("\nAfter fillalias:\n");
    // const TYPENAMES: [&str; 7] = ["ABot", "ALoc", "ACon", "AEsc", "ASym", "<5>", "AUnk"];
    // for i in TMP0..tmps.len() {
    //     print!(
    //         "    tmp {} {} - alias: type {} ",
    //         i,
    //         to_s(&tmps[i].name),
    //         TYPENAMES[tmps[i].alias.typ as usize]
    //     );
    //     if tmps[i].alias.base == TmpIdx::NONE {
    //         print!("base nil ");
    //     } else {
    //         print!(
    //             "base {} {} ",
    //             tmps[i].alias.base.0,
    //             to_s(&tmps[tmps[i].alias.base].name)
    //         );
    //     }
    //     print!("offset {} ", tmps[i].alias.offset);
    //     if tmps[i].alias.slot == TmpIdx::NONE {
    //         print!("slot nil");
    //     } else {
    //         print!(
    //             "slot {} {}",
    //             tmps[i].alias.slot.0,
    //             to_s(&tmps[tmps[i].alias.slot].name)
    //         );
    //     }
    //     println!();
    // }
    // println!("\n------------------------------\n");
}
