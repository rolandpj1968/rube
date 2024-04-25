use crate::all::{
    astack, bit, isload, isstore, Alias, AliasLoc, AliasT, AliasU, Bits, BlkIdx, Blks, CanAlias,
    Con, ConBits, ConT, Fn, Ins, Phi, PhiIdx, Ref, Tmp, TmpIdx, J, NBIT, O, OALLOC, OALLOC1,
};

use crate::load::storesz;

//pub fn getalias2(tmps:
pub fn getalias(tmps: &[Tmp], cons: &[Con], a_in: &Alias, r: Ref) -> Alias {
    let mut a_out: Alias = a_in.clone();
    assert!(matches!(r, Ref::RTmp(_)) || matches!(r, Ref::RCon(_)));
    match r {
        Ref::RTmp(ti) => {
            //let t: &Tmp = f.tmp(ti);
            let a1: &Alias = &tmps[ti].alias; //f.alias(t.alias);
            a_out = a1.clone();
            if astack(a_in.typ) != 0 {
                a_out.typ = tmps[a_out.slot].alias.typ;
            }
            assert!(a_out.typ != AliasT::ABot);
        }
        Ref::RCon(ci) => {
            let c: &Con = &cons[ci.0 as usize]; //f.con(ci);
                                                // println!(
                                                //     "                                           getalias() RCon {:?}",
                                                //     c
                                                // );
            match c.type_ {
                ConT::CAddr => {
                    a_out.typ = AliasT::ASym;
                    a_out.u = AliasU::ASym(c.sym);
                }
                _ => {
                    a_out.typ = AliasT::ACon;
                }
            }
            assert!(matches!(c.bits, ConBits::I(_)));
            if let ConBits::I(i) = c.bits {
                a_out.offset = i;
            }
            // else {
            //     // Needed for CAddr where c.bits is None; ropy!
            //     // Hrmm, changed CAddr to have I(0) by default now, check again...
            //     a_out.offset = 0;
            //     //assert!(false);
            // }
            a_out.slot = TmpIdx::NONE; //AliasIdx::NONE;
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
    // println!(
    //     "                        alias: p {:?} ap {:?} : q {:?} aq {:?}",
    //     p, ap, q, aq
    // );
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
            // println!(
            //     "                                both ASym - ovlap is {}",
            //     ovlap
            // );
            /* they conservatively alias if the
             * symbols are different, or they
             * alias for sure if they overlap */
            if let AliasU::ASym(ap_sym) = ap.u {
                if let AliasU::ASym(aq_sym) = aq.u {
                    if ap_sym != aq_sym {
                        CanAlias::May
                    } else if ovlap {
                        CanAlias::Must
                    } else {
                        CanAlias::No
                    }
                } else {
                    assert!(false);
                    CanAlias::May
                }
            } else {
                assert!(false);
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

pub fn escapes(f: &Fn, r: Ref) -> bool {
    if let Ref::RTmp(ti) = r {
        //let ai: AliasIdx = f.tmp(ti).alias;
        let a: &Alias = &f.tmps[ti].alias;
        astack(a.typ) == 0 || f.tmps[a.slot].alias.typ == AliasT::AEsc
    } else {
        true
    }
}

fn esc(tmps: &mut [Tmp], r: Ref) {
    match r {
        Ref::RTmp(ti) => {
            //let ai: AliasIdx = f.tmp(ti).alias;
            let (a_type, a_slot) = {
                let a: &Alias = &tmps[ti].alias;
                (a.typ, a.slot)
            };
            if astack(a_type) != 0 {
                tmps[a_slot].alias.typ = AliasT::AEsc;
            }
        }
        Ref::R | Ref::RCon(_) | Ref::RInt(_) | Ref::RTyp(_) => (), /*ok*/
        _ => assert!(false),
    }
}

fn store(tmps: &mut [Tmp], r: Ref, sz: i32) {
    if let Ref::RTmp(ti) = r {
        //let ai: AliasIdx = f.tmp(ti).alias;
        let (a_type, a_offset, a_slot) = {
            let a: &Alias = &tmps[ti].alias;
            (a.typ, a.offset, a.slot)
        };
        if a_slot != TmpIdx::NONE
        /*AliasIdx::NONE*/
        {
            assert!(astack(a_type) != 0);
            let m: Bits = {
                if sz >= (NBIT as i32) || (a_offset < 0 || a_offset >= (NBIT as i64)) {
                    u64::MAX
                } else {
                    (bit(sz as usize) - 1) << a_offset
                }
            };
            let aslot: &mut Alias = &mut tmps[a_slot].alias;
            assert!(matches!(aslot.u, AliasU::ALoc(_)));
            if let AliasU::ALoc(loc) = &mut aslot.u {
                loc.m |= m;
            }
        }
    }
}

pub fn fillalias(f: &mut Fn) {
    let blks: &Blks = &f.blks;
    let tmps: &mut [Tmp] = &mut f.tmps;
    let cons: &[Con] = &f.cons;
    let phis: &[Phi] = &f.phis;
    // println!("        fillalias:      function ${}", to_s(&f.name));
    //let aliases = &mut f.aliases;

    {
        //f.aliases.clear();
        for ti in 0..tmps.len() {
            //let ai = f.add_alias(Alias::default());
            tmps[ti].alias = Alias::default(); // ai
        }
    }

    //let blks = &f.blks;

    for n in 0..f.nblk {
        let bi: BlkIdx = f.rpo[n as usize];
        let mut pi: PhiIdx = f.blks.phi_of(bi);
        while pi != PhiIdx::NONE {
            assert!(matches!(phis[pi].to, Ref::RTmp(_)));
            if let Ref::RTmp(ti) = phis[pi].to {
                //let ai = f.tmp(ti).alias;
                let a: &mut Alias = &mut tmps[ti].alias; //f.alias_mut(ai);
                assert!(a.typ == AliasT::ABot);
                a.typ = AliasT::AUnk;
                a.base = ti;
                a.offset = 0;
                a.slot = TmpIdx::NONE; // AliasIdx::NONE;
            }
            pi = phis[pi].link;
        }
        let ins_len = f.blks.borrow(bi).ins().len();
        for ii in 0..ins_len
        /*f.blks.borrow(bi).ins.len()*/
        {
            let (i_to, i_op, i_arg0, i_arg1) = {
                let b = f.blks.borrow(bi);
                let i: &Ins = &b /*f.blks.borrow(bi)*/
                    .ins()[ii];
                // println!("        fillalias:          ins ${:?}", i);
                (i.to, i.op, i.args[0], i.args[1])
            };
            let i: Ins = f.blks.borrow(bi).ins()[ii]; // Note copy

            if i_op == O::Oblit1 {
                // Already handled as part of preceding Oblit0
                continue;
            }

            //let mut ai: AliasIdx = AliasIdx::NONE;
            let mut ai: TmpIdx = TmpIdx::NONE;

            assert!(i.to == Ref::R || matches!(i.to, Ref::RTmp(_)));
            if let Ref::RTmp(ti) = i_to {
                ai = ti;
                //ai = f.tmp(ti).alias;
                let a: &mut Alias = &mut tmps[ti].alias;
                // let a_type: AliasT;
                // let a_slot: AliasIdx;
                // let maybe_a_u: Option<AliasU>;
                // TODO isalloc()
                assert!(a.typ == AliasT::ABot);
                if OALLOC <= i_op && i_op <= OALLOC1 {
                    a.typ = AliasT::ALoc;
                    a.slot = ti;
                    let mut sz: i32 = -1;
                    if let Ref::RCon(ci) = i_arg0 {
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
                    a.slot = TmpIdx::NONE; //AliasIdx::NONE;
                    a.u = AliasU::None;
                }
                // let a: &mut Alias = f.alias_mut(ai);
                // assert!(a.typ == AliasT::ABot);
                // a.typ = a_type;
                // a.slot = a_slot;
                // a.base = ti;
                // a.offset = 0;
                // if let Some(a_u) = maybe_a_u {
                //     a.u = a_u;
                // }
            }
            if i_op == O::Ocopy {
                assert!(ai != TmpIdx::NONE /*AliasIdx::NONE*/);
                let a0: Alias = getalias(tmps, cons, &tmps[ai].alias, i_arg0);
                tmps[ai].alias = a0;
                //*f.alias_mut(ai) = a0;
            }
            // TODO - why not Osub too? QBE question
            if i_op == O::Oadd {
                assert!(ai != TmpIdx::NONE /*AliasIdx::NONE*/); // not in QBE - too aggressive?
                let a0: Alias = getalias(tmps, cons, &Alias::default(), i_arg0);
                let a1: Alias = getalias(tmps, cons, &Alias::default(), i_arg1);
                if a0.typ == AliasT::ACon {
                    tmps[ai].alias = a1;
                    tmps[ai].alias.offset += a0.offset;
                } else if a1.typ == AliasT::ACon {
                    //println!("                               arg1 is a constant");
                    tmps[ai].alias = a0;
                    tmps[ai].alias.offset += a1.offset;
                }
            }
            if (i_to == Ref::R || tmps[ai].alias.typ == AliasT::AUnk) && i_op != O::Oblit0 {
                if !isload(i_op) {
                    esc(tmps, i_arg0);
                }
                if !isstore(i_op) && i_op != O::Oargc {
                    esc(tmps, i_arg1);
                }
            }
            if i_op == O::Oblit0 {
                assert!(ii < f.blks.borrow(bi).ins().len() - 1);
                let (blit1_op, blit1_arg0) = {
                    let b = f.blks.borrow(bi);
                    let blit1: &Ins = &b /*f.blks.borrow(bi)*/
                        .ins()[ii + 1];
                    (blit1.op, blit1.args[0])
                };
                assert!(blit1_op == O::Oblit1);
                if let Ref::RInt(blit1_i) = blit1_arg0 {
                    store(tmps, i_arg1, blit1_i.abs());
                } else {
                    // Oblit1 arg MUST be RInt
                    assert!(false);
                }
            }
            if isstore(i_op) {
                store(tmps, i_arg1, storesz(&i /*f.blks.borrow(bi).ins()[ii]*/));
            }
        }
        if blks.borrow(bi).jmp().type_ != J::Jretc {
            let jmp_arg = blks.borrow(bi).jmp().arg;
            esc(tmps, jmp_arg /*f.blks.borrow(bi).jmp().arg*/);
        }
    }
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let mut pi: PhiIdx = f.blks.phi_of(bi);
        while pi != PhiIdx::NONE {
            for n in 0..phis[pi].args.len() {
                esc(tmps, phis[pi].args[n]);
            }
            pi = phis[pi].link;
        }
        bi = blks.borrow(bi).link;
    }
}
