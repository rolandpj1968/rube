// TODO remove eventually
#![allow(dead_code, unused_variables)]

use std::cell;
//use std::iter::Map;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

// TODO - use this more prevalently...
use derive_new::new;
use strum_macros::FromRepr;

use crate::mem::SlotIdx;
use crate::util::InternId;

use Ref::{RCon, R};
use K::{Kd, Kl, Ks, Kw, Kx, K0};

// Generic Result
pub type RubeError = Box<dyn std::error::Error>;
pub type RubeResult<T> = Result<T, RubeError>;

// Helper for displaying byte slice
pub fn to_s(raw: &[u8]) -> String {
    String::from_utf8_lossy(raw).to_string()
}

pub struct RefCellVec<T> {
    v: Vec<cell::RefCell<T>>,
}

impl<T> RefCellVec<T> {
    fn borrow_i(&self, i: usize) -> cell::Ref<T> {
        self.v[i].borrow()
    }
    pub fn with_i<R>(&self, i: usize, f: impl FnOnce(&T) -> R) -> R {
        f(&*self.borrow_i(i))
    }
    fn borrow_mut_i(&self, i: usize) -> cell::RefMut<T> {
        self.v[i].borrow_mut()
    }
    pub fn with_mut_i<R>(&self, i: usize, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut *self.borrow_mut_i(i))
    }
    pub fn len(&self) -> usize {
        self.v.len()
    }
    fn add_i(&mut self, elem: T) -> usize {
        self.v.push(cell::RefCell::new(elem));
        self.len() - 1
    }
}

pub type Blks = RefCellVec<Blk>;
// Hrmmm, wants lifetime specifier?
// pub type BlkRef = cell::Ref<Blk>;
// pub type BlkRefMut = cell::RefMut<Blk>;

// TODO - can do this on RefCellVec as generic on TagT
impl Blks {
    pub fn borrow(&self, bi: BlkIdx) -> cell::Ref<Blk> {
        assert!(bi != BlkIdx::NONE);
        self.borrow_i(bi.0 as usize)
    }
    pub fn with<R>(&self, bi: BlkIdx, f: impl FnOnce(&Blk) -> R) -> R {
        assert!(bi != BlkIdx::NONE);
        self.with_i(bi.0 as usize, f)
    }
    pub fn borrow_mut(&self, bi: BlkIdx) -> cell::RefMut<Blk> {
        assert!(bi != BlkIdx::NONE);
        self.borrow_mut_i(bi.0 as usize)
    }
    pub fn with_mut<R>(&self, bi: BlkIdx, f: impl FnOnce(&mut Blk) -> R) -> R {
        assert!(bi != BlkIdx::NONE);
        self.with_mut_i(bi.0 as usize, f)
    }
    pub fn add(&mut self, b: Blk) -> BlkIdx {
        BlkIdx::from(self.add_i(b))
    }
    pub fn play(&self) -> std::slice::Iter<cell::RefCell<Blk>> {
        self.v.iter()
    }
    // Hrmmmm, what type does map() return...
    // pub fn iter_mut<R, F: FnMut(cell::RefMut<Blk>) -> R>(
    //     &self,
    // ) -> Map<std::slice::Iter<cell::RefCell<Blk>>, F> {
    //     self.v
    //         .iter()
    //         .map::<std::slice::Iter<cell::RefCell<Blk>>, F>(|br: cell::RefCell<Blk>| {
    //             br.borrow_mut()
    //         })
    // }
    pub fn for_each_mut(&self, mut f: impl FnMut(&mut Blk)) {
        let mut bi = BlkIdx::START;
        while bi != BlkIdx::NONE {
            let mut b = self.borrow_mut(bi);
            f(&mut *b);
            bi = b.link;
        }
        // TODO - this generates blks in a different order from the link chain :(
        // Need to sort f.blks on link chain order to maintain behaviour parity with QBE
        // self.v.iter().for_each(|br| {
        //     let mut b = br.borrow_mut();
        //     assert!(b.is_defined);
        //     if !b.is_dead {
        //         f(&mut *b)
        //     }
        // });
    }

    pub fn for_each(&self, mut f: impl FnMut(&Blk)) {
        let mut bi = BlkIdx::START;
        while bi != BlkIdx::NONE {
            let b = self.borrow_mut(bi);
            f(&*b);
            bi = b.link;
        }
    }

    pub fn for_each_bi(&self, mut f: impl FnMut(BlkIdx)) {
        let mut bi = BlkIdx::START;
        while bi != BlkIdx::NONE {
            f(bi);
            bi = self.borrow(bi).link;
        }
        // TODO - this generates blks in a different order from the link chain :(
        // Need to sort f.blks on link chain order to maintain behaviour parity with QBE
        // let len = self.len();
        // for bii in 0..len {
        //     let is_dead = self.v[bii].borrow().is_dead;
        //     if !is_dead {
        //         f(BlkIdx::new(bii));
        //     }
        // }
    }

    pub fn id_of(&self, bi: BlkIdx) -> RpoIdx {
        self.borrow(bi).id
    }
    pub fn dom_of(&self, bi: BlkIdx) -> BlkIdx {
        self.borrow(bi).dom
    }
    pub fn idom_of(&self, bi: BlkIdx) -> BlkIdx {
        self.borrow(bi).idom
    }
    pub fn phi_of(&self, bi: BlkIdx) -> PhiIdx {
        self.borrow(bi).phi
    }
    pub fn succs_of(&self, bi: BlkIdx) -> [BlkIdx; 2] {
        self.borrow(bi).succs()
    }
    pub fn dlink_of(&self, bi: BlkIdx) -> BlkIdx {
        self.borrow(bi).dlink
    }
    pub fn visit_of(&self, bi: BlkIdx) -> RpoIdx {
        self.borrow(bi).visit
    }
    pub fn ivisit_of(&self, bi: BlkIdx) -> i32 {
        self.borrow(bi).ivisit
    }
}

// Hrmm, it's complaining about lifetime params - need more grokking, just use .borrow() for now
// impl Index<BlkIdx> for Blks {
//     type Output = cell::Ref<Blk>;
//     fn index(&self, index: BlkIdx) -> &Self::Output {
//         debug_assert!(index != BlkIdx::NONE);
//         &self.borrow(index)
//     }
// }

// Typed index into blks, tmps, etc for type safety
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Idx<T>(pub u32, PhantomData<T>);

impl<T> Idx<T> {
    pub const NONE: Idx<T> = Idx::<T>::from(u32::MAX as usize);
    pub const fn from(i: usize) -> Idx<T> {
        debug_assert!(i <= u32::MAX as usize);
        Idx::<T>(i as u32, PhantomData)
    }
    // Implement cast???
    pub fn usize(self) -> usize {
        self.0 as usize
    }
    pub fn next(self) -> Idx<T> {
        // Wrapping for RpoIdx in rporec et al
        Self(self.0.wrapping_add(1), PhantomData)
    }
    pub fn prev(self) -> Idx<T> {
        // Wrapping for RpoIdx in rporec et al
        Self(self.0.wrapping_sub(1), PhantomData)
    }
}

macro_rules! def_index {
    ($idxt:ty, $colt:ty, $valt:ty) => {
        impl Index<$idxt> for $colt {
            type Output = $valt;
            fn index(&self, index: $idxt) -> &Self::Output {
                debug_assert!(index != <$idxt>::NONE);
                self.index(index.0 as usize)
            }
        }
        impl Index<&$idxt> for $colt {
            type Output = $valt;
            fn index(&self, index: &$idxt) -> &Self::Output {
                debug_assert!(*index != <$idxt>::NONE);
                self.index(index.0 as usize)
            }
        }
    };
}

macro_rules! def_index_mut {
    ($idxt:ty, $colt:ty, $valt:ty) => {
        impl IndexMut<$idxt> for $colt {
            fn index_mut(&mut self, index: $idxt) -> &mut Self::Output {
                debug_assert!(index != <$idxt>::NONE);
                self.index_mut(index.0 as usize)
            }
        }
        impl IndexMut<&$idxt> for $colt {
            fn index_mut(&mut self, index: &$idxt) -> &mut Self::Output {
                debug_assert!(*index != <$idxt>::NONE);
                self.index_mut(index.0 as usize)
            }
        }
    };
}

pub type Bits = u64;

/*
enum {
    NString = 80,
    NIns    = 1 << 20,
    NAlign  = 3,
    NField  = 32,
    NBit    = CHAR_BIT * sizeof(bits),
};
 */

pub const NBIT: usize = 8 * std::mem::size_of::<Bits>();

pub struct Target {
    pub name: &'static [u8],
    pub apple: bool,
    pub gpr0: u32, // first general purpose reg
    pub ngpr: u32,
    pub fpr0: u32, // first floating point reg
    pub nfpr: u32,
    pub rglob: Bits, // globally live regs (e.g., sp, fp)
    pub nrglob: u32,
    pub rsave: &'static [u32], // caller-save
    pub nrsave: [u32; 2],
    pub retregs: fn(Ref, &[u32; 2]) -> Bits,
    pub argregs: fn(Ref, &[u32; 2]) -> Bits,
    pub memargs: fn(u32) -> u32,
    pub abi0: fn(&mut Fn),
    pub abi1: fn(&mut Fn),
    pub isel: fn(&mut Fn),
    pub emitfn: fn(&Fn /*, FILE **/), // TODO
    pub emitfin: fn(/*FILE **/),      // TODO
    pub asloc: &'static [u8],
    pub assym: &'static [u8],
}

pub const fn bit(n: usize) -> Bits {
    (1 as Bits) << n
}

pub const RXX: u32 = 0;
pub const TMP0: usize = NBIT as usize;
pub const TMP0IDX: TmpIdx = TmpIdx::from(TMP0);

// TODO - just use BitSet
pub type BSet = Vec<Bits>;

// TODO we can tighten up these types
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Ref {
    R,
    RTmp(TmpIdx),
    RCon(ConIdx),
    RInt(i32),
    RTyp(TypIdx),
    RSlot(i32),
    RCall(u32),
    RMem(MemIdx),
}

pub const UNDEF: Ref = RCon(ConIdx::UNDEF); /* represents uninitialized data */
pub const CON_Z: Ref = RCon(ConIdx::CON_Z); /* represents uninitialized data */

/*
enum CmpI {
    Cieq,
    Cine,
    Cisge,
    Cisgt,
    Cisle,
    Cislt,
    Ciuge,
    Ciugt,
    Ciule,
    Ciult,
    NCmpI,
};

enum CmpF {
    Cfeq,
    Cfge,
    Cfgt,
    Cfle,
    Cflt,
    Cfne,
    Cfo,
    Cfuo,
    NCmpF,
    NCmp = NCmpI + NCmpF,
};

enum O {
    Oxxx,
#define O(op, x, y) O##op,
    #include "ops.h"
    NOp,
};
 */

// Generated from 'gcc -E' on QBE
#[derive(Clone, Copy, Debug, FromRepr, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum O {
    Oxxx,

    Oadd,
    Osub,
    Oneg,
    Odiv,
    Orem,
    Oudiv,
    Ourem,
    Omul,
    Oand,
    Oor,
    Oxor,
    Osar,
    Oshr,
    Oshl,

    Oceqw,
    Ocnew,
    Ocsgew,
    Ocsgtw,
    Ocslew,
    Ocsltw,
    Ocugew,
    Ocugtw,
    Oculew,
    Ocultw,

    Oceql,
    Ocnel,
    Ocsgel,
    Ocsgtl,
    Ocslel,
    Ocsltl,
    Ocugel,
    Ocugtl,
    Oculel,
    Ocultl,

    Oceqs,
    Ocges,
    Ocgts,
    Ocles,
    Oclts,
    Ocnes,
    Ocos,
    Ocuos,

    Oceqd,
    Ocged,
    Ocgtd,
    Ocled,
    Ocltd,
    Ocned,
    Ocod,
    Ocuod,

    Ostoreb,
    Ostoreh,
    Ostorew,
    Ostorel,
    Ostores,
    Ostored,

    Oloadsb,
    Oloadub,
    Oloadsh,
    Oloaduh,
    Oloadsw,
    Oloaduw,
    Oload,

    Oextsb,
    Oextub,
    Oextsh,
    Oextuh,
    Oextsw,
    Oextuw,

    Oexts,
    Otruncd,
    Ostosi,
    Ostoui,
    Odtosi,
    Odtoui,
    Oswtof,
    Ouwtof,
    Osltof,
    Oultof,
    Ocast,

    Oalloc4,
    Oalloc8,
    Oalloc16,

    Ovaarg,
    Ovastart,

    Ocopy,

    Odbgloc,

    Onop,
    Oaddr,
    Oblit0,
    Oblit1,
    Oswap,
    Osign,
    Osalloc,
    Oxidiv,
    Oxdiv,
    Oxcmp,
    Oxtest,
    Oacmp,
    Oacmn,
    Oafcmp,
    Oreqz,
    Ornez,

    Opar,
    Oparsb,
    Oparub,
    Oparsh,
    Oparuh,
    Oparc,
    Opare,
    Oarg,
    Oargsb,
    Oargub,
    Oargsh,
    Oarguh,
    Oargc,
    Oarge,
    Oargv,
    Ocall,

    Oflagieq,
    Oflagine,
    Oflagisge,
    Oflagisgt,
    Oflagisle,
    Oflagislt,
    Oflagiuge,
    Oflagiugt,
    Oflagiule,
    Oflagiult,
    Oflagfeq,
    Oflagfge,
    Oflagfgt,
    Oflagfle,
    Oflagflt,
    Oflagfne,
    Oflagfo,
    Oflagfuo,

    NOp,
}

pub const NPUBOP: u8 = O::Onop as u8;

// Generated by hand from QBE C code
#[derive(Clone, Copy, Debug, FromRepr, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum J {
    Jxxx,
    Jretw,
    Jretl,
    Jrets,
    Jretd,
    Jretsb,
    Jretub,
    Jretsh,
    Jretuh,
    Jretc,
    Jret0,
    Jjmp,
    Jjnz,
    Jjfieq,
    Jjfine,
    Jjfisge,
    Jjfisgt,
    Jjfisle,
    Jjfislt,
    Jjfiuge,
    Jjfiugt,
    Jjfiule,
    Jjfiult,
    Jjffeq,
    Jjffge,
    Jjffgt,
    Jjffle,
    Jjfflt,
    Jjffne,
    Jjffo,
    Jjffuo,
    Jhlt,
    NJmp,
}

pub fn ret_for_cls(k: K) -> Option<J> {
    if Kw <= k && k <= K0 {
        J::from_repr((J::Jretw) as u8 + (k as u8))
    } else {
        None
    }
}

pub fn cls_for_ret(j: J) -> Option<K> {
    if j == J::Jretc {
        Some(Kl)
    } else if in_range_j(j, J::Jretsb, J::Jretuh) {
        Some(Kw)
    } else if in_range_j(j, J::Jretw, J::Jretd) {
        K::from_repr((j as i8) - (J::Jretw as i8))
    } else {
        None
    }
}

// #[repr(u8)]
// pub enum ORanges {
//     Ocmpw = O::Oceqw as u8,
//     Ocmpw1 = O::Ocultw as u8,
//     Ocmpl = O::Oceql as u8,
//     Ocmpl1 = O::Ocultl as u8,
//     Ocmps = O::Oceqs as u8,
//     Ocmps1 = O::Ocuos as u8,
//     Ocmpd = O::Oceqd as u8,
//     Ocmpd1 = O::Ocuod as u8,
//     Oalloc = O::Oalloc4 as u8,
//     Oalloc1 = O::Oalloc16 as u8,
//     Oflag = O::Oflagieq as u8,
//     Oflag1 = O::Oflagfuo as u8,
//     NPubOp = O::Onop as u8,
//     Jjf = J::Jjfieq as u8,
//     Jjf1 = J::Jjffuo as u8,
// }

pub const OALLOC: O = O::Oalloc4;
pub const OALLOC1: O = O::Oalloc16;

fn in_range_o(x: O, l: O, u: O) -> bool {
    l <= x && x <= u
}

pub fn isstore(o: O) -> bool {
    in_range_o(o, O::Ostoreb, O::Ostored)
}

pub fn isload(o: O) -> bool {
    in_range_o(o, O::Oloadsb, O::Oload)
}

pub fn isext(o: O) -> bool {
    in_range_o(o, O::Oextsb, O::Oextuw)
}

pub fn ispar(o: O) -> bool {
    in_range_o(o, O::Opar, O::Opare)
}

pub fn isarg(o: O) -> bool {
    in_range_o(o, O::Oarg, O::Oargv)
}

pub fn isparbh(o: O) -> bool {
    in_range_o(o, O::Oparsb, O::Oparuh)
}

pub fn isargbh(o: O) -> bool {
    in_range_o(o, O::Oargsb, O::Oarguh)
}

fn in_range_j(x: J, l: J, u: J) -> bool {
    l <= x && x <= u
}

pub fn isret(j: J) -> bool {
    in_range_j(j, J::Jretw, J::Jret0)
}

pub fn isretbh(j: J) -> bool {
    in_range_j(j, J::Jretsb, J::Jretuh)
}

#[derive(Clone, Copy, Debug, FromRepr, PartialEq, PartialOrd)]
#[repr(i8)]
pub enum K {
    // Duplicated here from Kbase cos optab etc. uses the everythings.
    // This is going to cause grief?
    // Really want to extend KBase
    Kx = -1, /* "top" class (see usecheck() and clsmerge()) */
    Kw = 0,
    Kl,
    Ks,
    Kd,

    Ksb = 4, /* matches Oarg/Opar/Jret */
    Kub,
    Ksh,
    Kuh,
    Kc,
    K0,

    Ke = -2, /* erroneous mode */
             //Km = KBase::Kl as isize, /* memory pointer */
}

pub fn kwide(k: K) -> i32 {
    (k as i32) & 1
}

pub fn kbase(k: K) -> i32 {
    (k as i32) >> 1
}

// Alias
pub const KM: K = K::Kl;

// Used as array indices in OPTAB init
const_assert_eq!(Kw as usize, 0);
const_assert_eq!(Kl as usize, 1);
const_assert_eq!(Ks as usize, 2);
const_assert_eq!(Kd as usize, 3);

#[derive(Clone, Copy)]
pub struct Op {
    pub name: &'static [u8],
    pub argcls: [[K; 4]; 2],
    pub canfold: bool,
}

impl Op {
    pub const fn new(name: &'static [u8], argcls: [[K; 4]; 2], canfold: bool) -> Op {
        Op {
            name,
            argcls,
            canfold,
        }
    }
}

#[derive(Clone, Copy, Debug, new)]
pub struct Ins {
    pub op: O,
    pub cls: K, // Must be one of Kw, Kl, Ks, Kd
    pub to: Ref,
    pub args: [Ref; 2],
}

impl Ins {
    pub const NOP: Ins = Ins {
        op: O::Onop,
        cls: Kx,
        to: R,
        args: [R, R],
    };
    pub fn new0(op: O, cls: K, to: Ref) -> Ins {
        Ins::new(op, cls, to, [R, R])
    }

    pub fn new1(op: O, cls: K, to: Ref, args1: [Ref; 1]) -> Ins {
        Ins::new(op, cls, to, [args1[0], R])
    }

    pub fn new2(op: O, cls: K, to: Ref, args2: [Ref; 2]) -> Ins {
        Ins::new(op, cls, to, args2)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InsTag();
// Index into Blk::ins
pub type InsIdx = Idx<InsTag>;

def_index!(InsIdx, [Ins], Ins);
def_index_mut!(InsIdx, [Ins], Ins);
def_index!(InsIdx, Vec<Ins>, Ins);
def_index_mut!(InsIdx, Vec<Ins>, Ins);

#[derive(new)]
pub struct Phi {
    pub to: Ref,
    pub args: Vec<Ref>, // TODO would be cool to just have one Vec<(Ref, BlkIdx)>
    pub blks: Vec<BlkIdx>,
    pub cls: K,
    pub link: PhiIdx,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhiTag();
// Index into Fn::phis
pub type PhiIdx = Idx<PhiTag>;

def_index!(PhiIdx, [Phi], Phi);
def_index_mut!(PhiIdx, [Phi], Phi);
def_index!(PhiIdx, Vec<Phi>, Phi);
def_index_mut!(PhiIdx, Vec<Phi>, Phi);

#[derive(Clone, Copy)]
pub struct BlkJmp {
    pub typ: J,
    pub arg: Ref,
}

impl BlkJmp {
    pub fn new() -> BlkJmp {
        BlkJmp {
            typ: J::Jxxx,
            arg: R,
        }
    }
}

pub struct Blk {
    pub phi: PhiIdx,
    // Behind a RefCell to get access mutably together with other fields.
    pub ins: cell::RefCell<Vec<Ins>>,
    pub jmp: cell::RefCell<BlkJmp>,
    pub s1: BlkIdx,
    pub s2: BlkIdx,
    pub link: BlkIdx,

    pub is_defined: bool,
    pub is_dead: bool,
    pub id: RpoIdx,
    pub visit: RpoIdx, // TODO - this is probs not always...
    pub ivisit: i32,   // TODO - for ssa.rs, fold.rs ... fixme

    pub idom: BlkIdx,
    pub dom: BlkIdx,
    pub dlink: BlkIdx,
    pub frons: Vec<BlkIdx>,
    pub preds: Vec<BlkIdx>,
    pub in_: BSet,
    pub out: BSet,
    pub gen: BSet,
    pub nlive: [u32; 2],
    pub loop_: i32,
    pub name: Vec<u8>,
}

impl Blk {
    pub fn new(name: &[u8], id: RpoIdx, dlink: BlkIdx) -> Blk {
        Blk {
            phi: PhiIdx::NONE,
            ins: cell::RefCell::new(vec![]),
            jmp: cell::RefCell::new(BlkJmp::new()),
            s1: BlkIdx::NONE,
            s2: BlkIdx::NONE,
            link: BlkIdx::NONE,
            is_defined: false,
            is_dead: false,

            id,
            visit: RpoIdx::NONE,
            ivisit: -1, // TODO ... fixme

            idom: BlkIdx::NONE,
            dom: BlkIdx::NONE,
            dlink,
            frons: vec![],
            preds: vec![],
            in_: vec![],
            out: vec![],
            gen: vec![],
            nlive: [0u32; 2],
            loop_: 0,
            name: name.to_vec(),
        }
    }

    pub fn with_ins<R>(&self, f: impl FnOnce(&[Ins]) -> R) -> R {
        f(&*self.ins.borrow())
    }

    pub fn ins(&self) -> cell::Ref<Vec<Ins>> {
        self.ins.borrow()
    }

    pub fn ins_mut(&self) -> cell::RefMut<Vec<Ins>> {
        self.ins.borrow_mut()
    }

    pub fn jmp(&self) -> cell::Ref<BlkJmp> {
        self.jmp.borrow()
    }

    pub fn jmp_mut(&self) -> cell::RefMut<BlkJmp> {
        self.jmp.borrow_mut()
    }

    pub fn succs(&self) -> [BlkIdx; 2] {
        [
            self.s1,
            if self.s1 == self.s2 {
                BlkIdx::NONE
            } else {
                self.s2
            },
        ]
    }

    pub fn s1_s2(&self) -> (BlkIdx, BlkIdx) {
        (self.s1, self.s2)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlkTag();
// Index into Fn::blks
pub type BlkIdx = Idx<BlkTag>;

impl BlkIdx {
    pub const START: BlkIdx = BlkIdx::from(0);
}

def_index!(BlkIdx, [Blk], Blk);
def_index_mut!(BlkIdx, [Blk], Blk);
def_index!(BlkIdx, Vec<Blk>, Blk);
def_index_mut!(BlkIdx, Vec<Blk>, Blk);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RpoTag();
// Index into Fn::rpj
pub type RpoIdx = Idx<RpoTag>;

def_index!(RpoIdx, [BlkIdx], BlkIdx);
def_index_mut!(RpoIdx, [BlkIdx], BlkIdx);
def_index!(RpoIdx, Vec<BlkIdx>, BlkIdx);
def_index_mut!(RpoIdx, Vec<BlkIdx>, BlkIdx);

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum UseT {
    UXXX,
    UPhi(PhiIdx),
    UIns(InsIdx),
    UJmp,
}

#[derive(new, Clone, Copy, Debug)]
pub struct Use {
    pub typ: UseT,
    pub bi: BlkIdx, // TODO - need this to access type_ PhiIdx or InsIdx, but now bid is redundant
    pub bid: RpoIdx,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SymT {
    SGlo,
    SThr,
}

#[derive(new, Clone, Copy, Debug, PartialEq)]
pub struct Sym {
    pub type_: SymT,
    pub id: InternId,
}

impl Sym {
    const UNDEF: Sym = Sym {
        type_: SymT::SGlo,
        id: InternId::INVALID,
    }; // Ugh, sort out Con
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CanAlias {
    No,
    May,
    Must,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AliasT {
    ABot = 0,
    ALoc = 1, /* stack local */
    ACon = 2,
    AEsc = 3, /* stack escaping */
    ASym = 4,
    AUnk = 6,
}

pub fn astack(t: AliasT) -> u8 {
    (t as u8) & 1
}

#[derive(Clone, Copy, Debug)]
pub struct AliasLoc {
    pub sz: i32, /* -1 if > NBit */
    pub m: Bits,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
// TODO - this is partially redundant with AliasT
pub enum AliasU {
    None,
    ASym(Sym),
    ALoc(AliasLoc),
}

#[derive(Clone, Copy, Debug)]
pub struct Alias {
    pub typ: AliasT,
    pub base: TmpIdx,
    pub offset: i64,
    pub u: AliasU,
    pub slot: TmpIdx,
}

impl Alias {
    pub fn default() -> Alias {
        Alias {
            typ: AliasT::ABot,
            base: TmpIdx::NONE,
            offset: 0,
            u: AliasU::None,
            slot: TmpIdx::NONE,
        }
    }
}

#[derive(Clone, Copy, Debug, FromRepr, PartialEq)]
#[repr(u8)]
pub enum TmpWdth {
    WFull,
    Wsb, /* must match Oload/Oext order */
    Wub,
    Wsh,
    Wuh,
    Wsw,
    Wuw,
}

impl TmpWdth {
    fn from_opbh(opbh: O, opsb: O) -> TmpWdth {
        TmpWdth::from_repr((TmpWdth::Wsb as u8) + ((opbh as u8) - (opsb as u8))).unwrap()
    }

    pub fn from_parbh(op: O) -> TmpWdth {
        assert!(isparbh(op));
        TmpWdth::from_opbh(op, O::Oparsb)
    }

    pub fn from_loadbh(op: O) -> TmpWdth {
        assert!(isload(op) && op != O::Oload);
        TmpWdth::from_opbh(op, O::Oloadsb)
    }

    pub fn from_ext(op: O) -> TmpWdth {
        assert!(isext(op));
        TmpWdth::from_opbh(op, O::Oextsb)
    }
}

// TODO derive new?
#[derive(Debug)]
pub struct Tmp {
    pub name: Vec<u8>,
    pub def: InsIdx,
    pub uses: Vec<Use>,
    pub ndef: u32,
    // pub nuse: u32,
    pub bid: RpoIdx,
    // uint cost;
    pub slot: i32, /* -1 for unset */
    pub cls: K,
    // struct {
    //     int r;  /* register or -1 */
    //     int w;  /* weight */
    //     bits m; /* avoid these registers */
    // } hint;
    pub phi: TmpIdx,
    pub alias: Alias,
    pub width: TmpWdth,
    pub tvisit: TmpIdx, /*u32*/
    // bool??? TmpIdx?? It's a slot index in mem::coalesce :(
    pub svisit: SlotIdx,
}

impl Tmp {
    pub fn new(name: Vec<u8>, /*ndef: u32, nuse: u32,*/ slot: i32, cls: K) -> Tmp {
        Tmp {
            name,
            def: InsIdx::NONE, // ??? QBE sets ndef to 1 initially in parse.c
            uses: vec![Use::new(UseT::UXXX, BlkIdx::NONE, RpoIdx::NONE)], // QBE sets nuse to 1 initially in parse.c - probs not necessary
            ndef: 1, // TODO??? QBE sets ndef to 1 initially in parse.c
            bid: RpoIdx::NONE,

            slot,
            cls,
            phi: TmpIdx::NONE, // QBE inits to 0 in newtmp()
            alias: Alias::default(),
            width: TmpWdth::WFull,
            tvisit: TmpIdx::NONE,
            svisit: SlotIdx::NONE,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TmpTag();
// Index into Fn::tmps
pub type TmpIdx = Idx<TmpTag>;

def_index!(TmpIdx, [Tmp], Tmp);
def_index_mut!(TmpIdx, [Tmp], Tmp);
def_index!(TmpIdx, Vec<Tmp>, Tmp);
def_index_mut!(TmpIdx, Vec<Tmp>, Tmp);

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ConPP {
    I,
    S,
    D,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Con {
    CUndef,
    CBits(i64, ConPP),
    CAddr(Sym, i64),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConTag();
// Index in Fn::cons
pub type ConIdx = Idx<ConTag>;

impl ConIdx {
    pub const UNDEF: ConIdx = ConIdx::from(0); /* represents uninitialized data */
    pub const CON_Z: ConIdx = ConIdx::from(1);
}

def_index!(ConIdx, [Con], Con);
def_index_mut!(ConIdx, [Con], Con);
def_index!(ConIdx, Vec<Con>, Con);
def_index_mut!(ConIdx, Vec<Con>, Con);

#[derive(Debug)]
pub struct Addr {
    // amd64 addressing
    pub offset: Con,
    pub base: Ref,
    pub index: Ref,
    pub scale: i32,
}

pub type Mem = Addr;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemTag();
// Index into Fn::mems
pub type MemIdx = Idx<MemTag>;

def_index!(MemIdx, [Mem], Mem);
def_index_mut!(MemIdx, [Mem], Mem);
def_index!(MemIdx, Vec<Mem>, Mem);
def_index_mut!(MemIdx, Vec<Mem>, Mem);

#[derive(Clone)]
pub struct Lnk {
    pub export: bool,
    pub thread: bool,
    pub align: i8,
    pub sec: Vec<u8>,
    pub secf: Vec<u8>,
}

pub struct Fn {
    pub blks: Blks,
    pub phis: Vec<Phi>,
    pub start: BlkIdx,
    pub tmps: Vec<Tmp>,
    pub cons: Vec<Con>,
    pub mems: Vec<Mem>,
    pub nblk: u32,     // Becomes number of reachable Blk's
    pub retty: TypIdx, // index in Parser::typ, TypIdx::INVALID if no aggregate return
    pub retr: Ref,
    pub rpo: Vec<BlkIdx>,
    //pub bits reg,
    pub slot: i32, // ???
    pub vararg: bool,
    pub dynalloc: bool,
    pub name: Vec<u8>,
    pub lnk: Lnk,
}

impl Fn {
    pub fn new(lnk: Lnk) -> Fn {
        Fn {
            blks: Blks { v: vec![] },
            phis: vec![], // TODO - should be on Blk
            start: BlkIdx::NONE,
            tmps: vec![],
            cons: vec![],
            mems: vec![],
            nblk: 0,
            retty: TypIdx::NONE,
            retr: R,
            rpo: vec![],
            //bits reg,
            slot: -1, // ???
            vararg: false,
            dynalloc: false,
            name: vec![],
            lnk,
        }
    }

    pub fn blk(&self, bi: BlkIdx) -> cell::Ref<Blk> {
        self.blks.borrow(bi)
    }

    pub fn blk_mut(&self, bi: BlkIdx) -> cell::RefMut<Blk> {
        self.blks.borrow_mut(bi)
    }

    pub fn add_blk(&mut self, b: Blk) -> BlkIdx {
        self.blks.add(b)
    }

    pub fn set_blk_link(&mut self, from_bi: BlkIdx, to_bi: BlkIdx) {
        if from_bi == BlkIdx::NONE {
            self.start = to_bi;
        } else {
            self.blk_mut(from_bi).link = to_bi;
        }
    }

    pub fn phi(&self, pi: PhiIdx) -> &Phi {
        assert!(pi != PhiIdx::NONE);
        &self.phis[pi.0 as usize]
    }

    pub fn phi_mut(&mut self, pi: PhiIdx) -> &mut Phi {
        assert!(pi != PhiIdx::NONE);
        &mut self.phis[pi.0 as usize]
    }

    pub fn add_phi(&mut self, p: Phi) -> PhiIdx {
        let pi: PhiIdx = PhiIdx::from(self.phis.len());
        self.phis.push(p);
        pi
    }

    pub fn tmp(&self, ti: TmpIdx) -> &Tmp {
        assert!(ti != TmpIdx::NONE);
        &self.tmps[ti.0 as usize]
    }

    pub fn tmp_mut(&mut self, ti: TmpIdx) -> &mut Tmp {
        assert!(ti != TmpIdx::NONE);
        &mut self.tmps[ti.0 as usize]
    }

    pub fn add_tmp(&mut self, t: Tmp) -> TmpIdx {
        let ti: TmpIdx = TmpIdx::from(self.tmps.len());
        self.tmps.push(t);
        ti
    }

    pub fn con(&self, ci: ConIdx) -> &Con {
        assert!(ci != ConIdx::NONE);
        &self.cons[ci.0 as usize]
    }

    pub fn con_mut(&mut self, ci: ConIdx) -> &mut Con {
        assert!(ci != ConIdx::NONE);
        &mut self.cons[ci.0 as usize]
    }

    pub fn add_con(&mut self, c: Con) -> ConIdx {
        let ci: ConIdx = ConIdx::from(self.cons.len());
        self.cons.push(c);
        ci
    }

    pub fn mem(&self, mi: MemIdx) -> &Mem {
        assert!(mi != MemIdx::NONE);
        &self.mems[mi.0 as usize]
    }
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum TypFldT {
    FEnd,
    Fb,
    Fh,
    Fw,
    Fl,
    Fs,
    Fd,
    FPad,
    FTyp,
}

pub struct TypFld {
    pub type_: TypFldT,
    pub len: u32, // or index in typ[] for FTyp
}

impl TypFld {
    pub fn new(type_: TypFldT, len: u32) -> TypFld {
        TypFld { type_, len }
    }
}

pub struct Typ {
    pub name: Vec<u8>,
    pub isdark: bool,
    pub isunion: bool,
    pub align: i32,
    pub size: u64,
    pub nunion: u32,
    pub fields: Vec<TypFld>, // TODO need indirection???
}

#[derive(Clone, Copy, Debug, PartialEq)]
// TODO - Idx<T>
pub struct TypIdx(pub u32);

impl TypIdx {
    pub const NONE: TypIdx = TypIdx(u32::MAX);
}

impl Typ {
    pub fn new() -> Typ {
        Typ {
            name: vec![],
            isdark: false,
            isunion: false,
            align: -1,
            size: 0,
            nunion: 0,
            fields: vec![],
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum DatT {
    DStart,
    DEnd,
    DB,
    DH,
    DW,
    DL,
    DZ,
}

#[repr(u8)]
pub enum DatU {
    None,
    Num(i64),
    Fltd(f64),
    Flts(f32),
    Str(Vec<u8>),
    Ref { name: Vec<u8>, off: i64 },
}

pub struct Dat {
    pub type_: DatT,
    pub name: Vec<u8>,
    pub lnk: Lnk,
    pub u: DatU,
    pub isref: bool,
    pub isstr: bool,
}

impl Dat {
    pub fn new(type_: DatT, name: &[u8], lnk: Lnk) -> Dat {
        Dat {
            type_,
            name: name.to_vec(),
            lnk,
            u: DatU::None,
            isref: false,
            isstr: false,
        }
    }
}

pub fn bshas(bs: &BSet, elt: usize) -> bool {
    assert!(elt < bs.len() * NBIT);
    bs[elt / NBIT] & bit(elt % NBIT) != 0
}
