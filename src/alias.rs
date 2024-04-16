// use std::error::Error;
// use std::fmt;

use crate::all::{
    astack, bit, isload, isstore, Alias, AliasIdx, AliasLoc, AliasT, AliasU, Bits, BlkIdx, Con, ConBits, ConT, Fn, Ins, PhiIdx, Ref, Tmp, TmpIdx, J, NBIT, O, OALLOC, OALLOC1,
};

use crate::load::storesz;

// TODO - can collapse all these error classes and just add a module field.
// #[derive(Debug)]
// struct AliasError {
//     msg: String,
// }

// impl AliasError {
//     fn new(msg: &str) -> AliasError {
//         AliasError {
//             msg: msg.to_string(),
//         }
//     }
// }

// impl fmt::Display for AliasError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.msg)
//     }
// }

// impl Error for AliasError {
//     fn description(&self) -> &str {
//         &self.msg
//     }
// }

/*
#include "all.h"
 */

fn getalias(f: &Fn, a_in: &Alias, r: Ref) -> Alias {
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
                assert!(false);
            }
            a_out.slot = AliasIdx::INVALID;
        }
        _ => assert!(false), /*unreachable*/
    }

    a_out
}

/*
int
alias(Ref p, int op, int sp, Ref q, int sq, int *delta, Fn *fn)
{
    Alias ap, aq;
    int ovlap;

    getalias(&ap, p, fn);
    getalias(&aq, q, fn);
    ap.offset += op;
    /* when delta is meaningful (ovlap == 1),
     * we do not overflow int because sp and
     * sq are bounded by 2^28 */
    *delta = ap.offset - aq.offset;
    ovlap = ap.offset < aq.offset + sq && aq.offset < ap.offset + sp;

    if (astack(ap.type) && astack(aq.type)) {
        /* if both are offsets of the same
         * stack slot, they alias iif they
         * overlap */
        if (ap.base == aq.base && ovlap)
            return MustAlias;
        return NoAlias;
    }

    if (ap.type == ASym && aq.type == ASym) {
        /* they conservatively alias if the
         * symbols are different, or they
         * alias for sure if they overlap */
        if (!symeq(ap.u.sym, aq.u.sym))
            return MayAlias;
        if (ovlap)
            return MustAlias;
        return NoAlias;
    }

    if ((ap.type == ACon && aq.type == ACon)
    || (ap.type == aq.type && ap.base == aq.base)) {
        assert(ap.type == ACon || ap.type == AUnk);
        /* if they have the same base, we
         * can rely on the offsets only */
        if (ovlap)
            return MustAlias;
        return NoAlias;
    }

    /* if one of the two is unknown
     * there may be aliasing unless
     * the other is provably local */
    if (ap.type == AUnk && aq.type != ALoc)
        return MayAlias;
    if (aq.type == AUnk && ap.type != ALoc)
        return MayAlias;

    return NoAlias;
}
 */

fn escapes(f: &Fn, r: Ref) -> bool {
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
	Ref::R => ()/*ok*/,
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
        if a_slot != AliasIdx::INVALID {
            assert!(astack(a_type) != 0);
            let m: Bits = {
		if (sz as u32) >= NBIT || (a_offset < 0 || a_offset >= NBIT as i64) {
                    u64::MAX
		} else {
                    (bit(sz as u32) - 1) << a_offset
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
    for ti in 0..f.tmps.len() {
	let ai = f.add_alias(Alias { type_: AliasT::ABot, base: TmpIdx::INVALID, offset: 0, u: AliasU::ALoc(AliasLoc { sz: 0, m: 0 }), slot: AliasIdx::INVALID });
        f.tmps[ti].alias = ai;
    }
    
    for n in 0..f.nblk {
        let bi: BlkIdx = f.rpo[n as usize];
	let mut pi: PhiIdx = f.blk(bi).phi;
	while pi != PhiIdx::INVALID {
	    if let Ref::RTmp(ti) = f.phi(pi).to {
		let ai = f.tmp(ti).alias;
		let a: &mut Alias = f.alias_mut(ai);
		assert!(a.type_ == AliasT::ABot);
		a.type_ = AliasT::AUnk;
		a.base = ti;
		a.offset = 0;
		a.slot = AliasIdx::INVALID;
	    } else {
		// Phi must define a Tmp
		assert!(false);
	    }
	    pi = f.phi(pi).link;
        }
	for ii in 0..f.blk(bi).ins.len() {
	    let (i_to, i_op, i_arg0, i_arg1) = {
		let i: &Ins = &f.blk(bi).ins[ii];
		(i.to, i.op, i.args[0], i.args[1])
	    };

	    if i_op == O::Oblit1 {
		// Already handled as part of preceding Oblit0
		continue;
	    }

	    let mut ai: AliasIdx = AliasIdx::INVALID;

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
                    maybe_a_u = Some(AliasU::ALoc(AliasLoc {sz, m: 0}));
                } else {
                    a_type = AliasT::AUnk;
                    a_slot = AliasIdx::INVALID;
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
                assert!(ai != AliasIdx::INVALID);
                let a0: Alias = getalias(f, f.alias(ai), i_arg0);
		*f.alias_mut(ai) = a0;
            }
            if i_op == O::Oadd {
		let a0_in: Alias = Alias { type_: AliasT::ABot, base: TmpIdx::INVALID, offset: 0, u: AliasU::ALoc(AliasLoc { sz: 0, m: 0 }), slot: AliasIdx::INVALID };
                let a0: Alias = getalias(f, &a0_in, i_arg0);
		let a1_in: Alias = Alias { type_: AliasT::ABot, base: TmpIdx::INVALID, offset: 0, u: AliasU::ALoc(AliasLoc { sz: 0, m: 0 }), slot: AliasIdx::INVALID };
                let a1: Alias = getalias(f, &a1_in, i_arg1);
                if a0.type_ == AliasT::ACon {
		    *f.alias_mut(ai) = a1;
                    f.alias_mut(ai).offset += a0.offset;
                }
                else if a1.type_ == AliasT::ACon {
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
		assert!(ii < f.blk(bi).ins.len()-1);
		let (blit1_op, blit1_arg0) = {
		    let blit1: &Ins = &f.blk(bi).ins[ii+1];
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
                store(f, i_arg1, storesz(&f.blk(bi).ins[ii]));
	    }
	}
        if f.blk(bi).jmp.type_ != J::Jretc {
            esc(f, f.blk(bi).jmp.arg);
	}
    }
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::INVALID {
	let mut pi: PhiIdx = f.blk(bi).phi;
	while pi != PhiIdx::INVALID {
	    for n in 0..f.phi(pi).args.len() {
		esc(f, f.phi(pi).args[n]);
	    }
	    pi = f.phi(pi).link;
	}
	bi = f.blk(bi).link;
    }
}

