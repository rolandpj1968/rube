// TODO remove eventually
#![allow(dead_code, unused_variables)]

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

// Typed index into blks, tmps, etc for type safety
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Idx<TagT: Eq + Copy>(pub u32, PhantomData<TagT>);

impl<TagT: Eq + Copy> Idx<TagT> {
    pub const NONE: Idx<TagT> = Idx::<TagT>::from(u32::MAX as usize);
    pub const fn from(i: usize) -> Idx<TagT> {
        debug_assert!(i <= u32::MAX as usize);
        Idx::<TagT>(i as u32, PhantomData)
    }
    // Implement cast???
    pub fn usize(self) -> usize {
        self.0 as usize
    }
    pub fn next(self) -> Idx<TagT> {
        // Wrapping for RpoIdx in rporec et al
        Self(self.0.wrapping_add(1), PhantomData)
    }
    pub fn prev(self) -> Idx<TagT> {
        // Wrapping for RpoIdx in rporec et al
        Self(self.0.wrapping_sub(1), PhantomData)
    }
    pub fn is_none(self) -> bool {
        self == Self::NONE
    }
    pub fn is_some(self) -> bool {
        !self.is_none()
    }
}

impl<TagT: Eq + Copy> Default for Idx<TagT> {
    fn default() -> Self {
        Self::NONE
    }
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
pub const TMP0: usize = NBIT;

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

#[derive(Clone, Copy, Debug, FromRepr, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum CmpI {
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
}

impl CmpI {
    pub fn from_op_w(op: O) -> CmpI {
        assert!(OCMPW <= op && op <= OCMPW1);
        return CmpI::from_repr((op as u8) - (OCMPW as u8)).unwrap();
    }

    pub fn from_op_l(op: O) -> CmpI {
        assert!(OCMPL <= op && op <= OCMPL1);
        return CmpI::from_repr((op as u8) - (OCMPL as u8)).unwrap();
    }
}

#[derive(Clone, Copy, Debug, FromRepr, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum CmpF {
    Cfeq,
    Cfge,
    Cfgt,
    Cfle,
    Cflt,
    Cfne,
    Cfo,
    Cfuo,
    NCmpF,
    //NCmp = NCmpI + NCmpF,
}

impl CmpF {
    pub fn from_op_s(op: O) -> CmpF {
        assert!(OCMPS <= op && op <= OCMPS1);
        return CmpF::from_repr((op as u8) - (OCMPS as u8)).unwrap();
    }

    pub fn from_op_d(op: O) -> CmpF {
        assert!(OCMPD <= op && op <= OCMPD1);
        return CmpF::from_repr((op as u8) - (OCMPD as u8)).unwrap();
    }
}

/*
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

pub const OCMPW: O = O::Oceqw;
pub const OCMPW1: O = O::Ocultw;
pub const OCMPL: O = O::Oceql;
pub const OCMPL1: O = O::Ocultl;
pub const OCMPS: O = O::Oceqs;
pub const OCMPS1: O = O::Ocuos;
pub const OCMPD: O = O::Oceqd;
pub const OCMPD1: O = O::Ocuod;
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
}

pub fn kwide(k: K) -> i32 {
    (k as i32) & 1
}

pub fn kbase(k: K) -> i32 {
    (k as i32) >> 1
}

// Alias
pub const KM: K = K::Kl; /* memory pointer */

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
    pub ins: Vec<Ins>,
    pub jmp: BlkJmp,
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
            ins: vec![],
            jmp: BlkJmp::new(),
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
// Index into Fn::rpo
pub type RpoIdx = Idx<RpoTag>;

def_index!(RpoIdx, [BlkIdx], BlkIdx);
def_index_mut!(RpoIdx, [BlkIdx], BlkIdx);
def_index!(RpoIdx, Vec<BlkIdx>, BlkIdx);
def_index_mut!(RpoIdx, Vec<BlkIdx>, BlkIdx);

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum UseT {
    Uxxx,
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
    pub typ: SymT,
    pub id: InternId,
}

impl Sym {
    const UNDEF: Sym = Sym {
        typ: SymT::SGlo,
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
    pub tvisit: TmpIdx,
    pub svisit: SlotIdx,
}

impl Tmp {
    pub fn new(name: Vec<u8>, cls: K) -> Tmp {
        Tmp {
            name,
            def: InsIdx::NONE, // ??? QBE sets ndef to 1 initially in parse.c
            uses: vec![Use::new(UseT::Uxxx, BlkIdx::NONE, RpoIdx::NONE)], // QBE sets nuse to 1 initially in parse.c - probs not necessary
            ndef: 1, // TODO??? QBE sets ndef to 1 initially in parse.c
            bid: RpoIdx::NONE,

            slot: -1,
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

impl TmpIdx {
    pub const TMP0: TmpIdx = TmpIdx::from(TMP0);
}

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
    pub blks: Vec<Blk>,
    pub phis: Vec<Phi>,
    pub start: BlkIdx, // Always 0
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
            blks: vec![],
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

    pub fn add_blk(&mut self, b: Blk) -> BlkIdx {
        let bi: BlkIdx = BlkIdx::from(self.blks.len());
        self.blks.push(b);
        bi
    }

    pub fn add_phi(&mut self, p: Phi) -> PhiIdx {
        let pi: PhiIdx = PhiIdx::from(self.phis.len());
        self.phis.push(p);
        pi
    }

    pub fn add_tmp(&mut self, t: Tmp) -> TmpIdx {
        let ti: TmpIdx = TmpIdx::from(self.tmps.len());
        self.tmps.push(t);
        ti
    }

    pub fn add_con(&mut self, c: Con) -> ConIdx {
        let ci: ConIdx = ConIdx::from(self.cons.len());
        self.cons.push(c);
        ci
    }
}

// Helpers for iterating through Blk::link chain
pub fn for_each_bi(blks: &[Blk], mut f: impl FnMut(BlkIdx)) {
    loop_bi!(blks, bi, {
        f(bi);
    });
}

pub fn for_each_blk_mut(blks: &mut [Blk], mut f: impl FnMut(&mut Blk)) {
    loop_bi!(blks, bi, {
        f(&mut blks[bi]);
    });
}

pub fn for_each_blk(blks: &[Blk], mut f: impl FnMut(&Blk)) {
    loop_bi!(blks, bi, {
        f(&blks[bi]);
    });
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Debug, new)]
pub struct TypFld {
    pub typ: TypFldT,
    pub len: u32, // or index in typ[] for FTyp
}

#[derive(Debug)]
pub struct Typ {
    pub name: Vec<u8>,
    pub isdark: bool,
    pub isunion: bool,
    pub align: i32,
    pub size: u64,
    pub nunion: u32,
    pub fields: Vec<TypFld>,
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypTag();
// Index into Fn::tmps
pub type TypIdx = Idx<TypTag>;

def_index!(TypIdx, [Typ], Typ);
def_index_mut!(TypIdx, [Typ], Typ);
def_index!(TypIdx, Vec<Typ>, Typ);
def_index_mut!(TypIdx, Vec<Typ>, Typ);

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
    pub typ: DatT,
    pub name: Vec<u8>,
    pub lnk: Lnk,
    pub u: DatU,
    pub isref: bool,
    pub isstr: bool,
}

impl Dat {
    pub fn new(typ: DatT, name: &[u8], lnk: Lnk) -> Dat {
        Dat {
            typ,
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
