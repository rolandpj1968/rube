use crate::all::{
    astack, bit, isload, isstore, Alias, AliasIdx, AliasLoc, AliasT, AliasU, Bits, BlkIdx,
    CanAlias, Con, ConBits, ConT, Fn, Ins, PhiIdx, Ref, Tmp, J, NBIT, O, OALLOC, OALLOC1,
};

use crate::load::storesz;

pub fn getalias(f: &Fn, a_in: &Alias, r: Ref) -> Alias {
    let mut a_out: Alias = a_in.clone();
    match r {
        Ref::RTmp(ti) => {
            let t: &Tmp = f.tmp(ti);
            let a1: &Alias = f.alias(t.alias);
            a_out = a1.clone();
            if astack(a_in.type_) != 0 {
                a_out.type_ = f.alias(a_out.slot).type_;
            }
            assert!(a_out.type_ != AliasT::ABot);
        }
        Ref::RCon(ci) => {
            let c: &Con = f.con(ci);
            // println!(
            //     "                                           getalias() RCon {:?}",
            //     c
            // );
            match c.type_ {
                ConT::CAddr => {
                    a_out.type_ = AliasT::ASym;
                    a_out.u = AliasU::ASym(c.sym);
                }
                _ => {
                    a_out.type_ = AliasT::ACon;
                }
            }
            if let ConBits::I(i) = c.bits {
                a_out.offset = i;
            } else {
                // Needed for CAddr where c.bits is None; ropy!
                // Hrmm, changed CAddr to have I(0) by default now, check again...
                a_out.offset = 0;
                //assert!(false);
            }
            a_out.slot = AliasIdx::NONE;
        }
        _ => assert!(false), /*unreachable*/
    }

    a_out
}

pub fn alias(f: &Fn, p: Ref, op: i32, sp: i32, q: Ref, sq: i32) -> (CanAlias, i32) {
    let mut ap: Alias = getalias(f, &Alias::default(), p);
    let aq: Alias = getalias(f, &Alias::default(), q);
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
        if astack(ap.type_) != 0 && astack(aq.type_) != 0 {
            /* if both are offsets of the same
             * stack slot, they alias iif they
             * overlap */
            if ap.base == aq.base && ovlap {
                CanAlias::Must
            } else {
                CanAlias::No
            }
        } else if ap.type_ == AliasT::ASym && aq.type_ == AliasT::ASym {
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
        } else if (ap.type_ == AliasT::ACon && aq.type_ == AliasT::ACon)
            || (ap.type_ == aq.type_ && ap.base == aq.base)
        {
            assert!(ap.type_ == AliasT::ACon || ap.type_ == AliasT::AUnk);
            /* if they have the same base, we
             * can rely on the offsets only */
            if ovlap {
                CanAlias::Must
            } else {
                CanAlias::No
            }
        } else if (ap.type_ == AliasT::AUnk && aq.type_ != AliasT::ALoc)
            || (aq.type_ == AliasT::AUnk && ap.type_ != AliasT::ALoc)
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
        let ai: AliasIdx = f.tmp(ti).alias;
        let a: &Alias = f.alias(ai);
        astack(a.type_) == 0 || f.alias(a.slot).type_ == AliasT::AEsc
    } else {
        true
    }
}

fn esc(f: &mut Fn, r: Ref) {
    match r {
        Ref::RTmp(ti) => {
            let ai: AliasIdx = f.tmp(ti).alias;
            let (a_type, a_slot) = {
                let a: &Alias = f.alias(ai);
                (a.type_, a.slot)
            };
            if astack(a_type) != 0 {
                f.alias_mut(a_slot).type_ = AliasT::AEsc;
            }
        }
        Ref::R | Ref::RCon(_) | Ref::RInt(_) | Ref::RTyp(_) => (), /*ok*/
        _ => assert!(false),
    }
}

fn store(f: &mut Fn, r: Ref, sz: i32) {
    if let Ref::RTmp(ti) = r {
        let ai: AliasIdx = f.tmp(ti).alias;
        let (a_type, a_offset, a_slot) = {
            let a: &Alias = f.alias(ai);
            (a.type_, a.offset, a.slot)
        };
        if a_slot != AliasIdx::NONE {
            assert!(astack(a_type) != 0);
            let m: Bits = {
                if sz >= (NBIT as i32) || (a_offset < 0 || a_offset >= (NBIT as i64)) {
                    u64::MAX
                } else {
                    (bit(sz as usize) - 1) << a_offset
                }
            };
            let aslot: &mut Alias = f.alias_mut(a_slot);
            if let AliasU::ALoc(loc) = &mut aslot.u {
                loc.m |= m;
            } else {
                assert!(false);
            }
        }
    }
}

pub fn fillalias(f: &mut Fn) {
    // println!("        fillalias:      function ${}", to_s(&f.name));
    //let aliases = &mut f.aliases;

    {
        f.aliases.clear();
        for ti in 0..f.tmps.len() {
            let ai = f.add_alias(Alias::default());
            f.tmps[ti].alias = ai;
        }
    }

    //let blks = &f.blks;

    for n in 0..f.nblk {
        let bi: BlkIdx = f.rpo[n as usize];
        let mut pi: PhiIdx = f.blks.borrow(bi).phi;
        while pi != PhiIdx::NONE {
            if let Ref::RTmp(ti) = f.phi(pi).to {
                let ai = f.tmp(ti).alias;
                let a: &mut Alias = f.alias_mut(ai);
                assert!(a.type_ == AliasT::ABot);
                a.type_ = AliasT::AUnk;
                a.base = ti;
                a.offset = 0;
                a.slot = AliasIdx::NONE;
            } else {
                // Phi must define a Tmp
                assert!(false);
            }
            pi = f.phi(pi).link;
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

            let mut ai: AliasIdx = AliasIdx::NONE;

            if let Ref::RTmp(ti) = i_to {
                ai = f.tmp(ti).alias;
                let a_type: AliasT;
                let a_slot: AliasIdx;
                let maybe_a_u: Option<AliasU>;
                if OALLOC <= i_op && i_op <= OALLOC1 {
                    a_type = AliasT::ALoc;
                    a_slot = ai;
                    let mut sz: i32 = -1;
                    if let Ref::RCon(ci) = i_arg0 {
                        let c: &Con = f.con(ci);
                        if let ConBits::I(sz0) = c.bits {
                            if c.type_ == ConT::CBits && (0 <= sz0 && sz0 <= NBIT as i64) {
                                sz = sz0 as i32;
                            }
                        } else {
                            // MUST be an integer constant
                            assert!(false);
                        }
                    }
                    maybe_a_u = Some(AliasU::ALoc(AliasLoc { sz, m: 0 }));
                } else {
                    a_type = AliasT::AUnk;
                    a_slot = AliasIdx::NONE;
                    maybe_a_u = None;
                }
                let a: &mut Alias = f.alias_mut(ai);
                assert!(a.type_ == AliasT::ABot);
                a.type_ = a_type;
                a.slot = a_slot;
                a.base = ti;
                a.offset = 0;
                if let Some(a_u) = maybe_a_u {
                    a.u = a_u;
                }
            } else {
                // Ins must define a Tmp or be void
                assert!(i_to == Ref::R);
            }
            if i_op == O::Ocopy {
                assert!(ai != AliasIdx::NONE);
                let a0: Alias = getalias(f, f.alias(ai), i_arg0);
                *f.alias_mut(ai) = a0;
            }
            if i_op == O::Oadd {
                let a0: Alias = getalias(f, &Alias::default(), i_arg0);
                let a1: Alias = getalias(f, &Alias::default(), i_arg1);
                if a0.type_ == AliasT::ACon {
                    *f.alias_mut(ai) = a1;
                    f.alias_mut(ai).offset += a0.offset;
                } else if a1.type_ == AliasT::ACon {
                    //println!("                               arg1 is a constant");
                    *f.alias_mut(ai) = a0;
                    f.alias_mut(ai).offset += a1.offset;
                }
            }
            if (i_to == Ref::R || f.alias(ai).type_ == AliasT::AUnk) && i_op != O::Oblit0 {
                if !isload(i_op) {
                    esc(f, i_arg0);
                }
                if !isstore(i_op) && i_op != O::Oargc {
                    esc(f, i_arg1);
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
                    store(f, i_arg1, blit1_i.abs());
                } else {
                    // Oblit1 arg MUST be RInt
                    assert!(false);
                }
            }
            if isstore(i_op) {
                store(f, i_arg1, storesz(&i /*f.blks.borrow(bi).ins()[ii]*/));
            }
        }
        if f.blks.borrow(bi).jmp.type_ != J::Jretc {
            let jmp_arg = f.blks.borrow(bi).jmp.arg;
            esc(f, jmp_arg /*f.blks.borrow(bi).jmp.arg*/);
        }
    }
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let mut pi: PhiIdx = f.blks.borrow(bi).phi;
        while pi != PhiIdx::NONE {
            for n in 0..f.phi(pi).args.len() {
                esc(f, f.phi(pi).args[n]);
            }
            pi = f.phi(pi).link;
        }
        bi = f.blks.borrow(bi).link;
    }
}
