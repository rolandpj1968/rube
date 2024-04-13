// TODO remove eventually
#![allow(dead_code, unused_variables)]

// TODO - use this more prevalently...
use derive_new::new;
use strum_macros::FromRepr;

use crate::util::InternId;

// Generic Result
pub type RubeError = Box<dyn std::error::Error>;
pub type RubeResult<T> = Result<T, RubeError>;

// Helper for displaying byte slice
pub fn to_s(raw: &[u8]) -> String {
    String::from_utf8_lossy(raw).to_string()
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

pub const NBIT: u32 = 8 * std::mem::size_of::<Bits>() as u32;

pub struct Target {
    pub name: &'static [u8],
    pub apple: bool,
    pub gpr0: i32, // first general purpose reg
    pub ngpr: i32,
    pub fpr0: i32, // first floating point reg
    pub nfpr: i32,
    pub rglob: Bits, // globally live regs (e.g., sp, fp)
    pub nrglob: i32,
    pub rsave: Vec<i32>, // caller-save [Vec???]
    pub nrsave: [i32; 2],
    pub retregs: fn(Ref, [i32; 2]) -> Bits,
    pub argregs: fn(Ref, [i32; 2]) -> Bits,
    pub memargs: fn(i32) -> i32,
    pub abi0: fn(&mut Fn),
    pub abi1: fn(&mut Fn),
    pub isel: fn(&mut Fn),
    pub emitfn: fn(&Fn /*, FILE **/), // TODO
    pub emitfin: fn(/*FILE **/),      // TODO
    pub asloc: &'static [u8],
    pub assym: &'static [u8],
}

pub fn bit(n: u32) -> Bits {
    (1 as Bits) << n
}

pub const RXX: usize = 0;
pub const TMP0: u32 = NBIT;

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
#[derive(Clone, Copy, FromRepr, PartialEq, PartialOrd)]
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

pub fn ret_for_cls(k: KExt) -> Option<J> {
    if KW <= k && k <= K0 {
        J::from_repr((J::Jretw) as u8 + (k as u8))
    } else {
        None
    }
}

pub fn cls_for_ret(j: J) -> Option<KExt> {
    if j == J::Jretc {
        Some(KL)
    } else if in_range_j(j, J::Jretsb, J::Jretuh) {
        Some(KW)
    } else if in_range_j(j, J::Jretw, J::Jretd) {
        KExt::from_repr((j as i8) - (J::Jretw as i8))
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

fn in_range_o(x: O, l: O, u: O) -> bool {
    l <= x && x <= u
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
pub enum KExt {
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

pub const KX: KExt = KExt::Kx;
pub const KW: KExt = KExt::Kw;
pub const KL: KExt = KExt::Kl;
pub const KS: KExt = KExt::Ks;
pub const KD: KExt = KExt::Kd;

// Alias
pub const KM: KExt = KExt::Kl;

pub const KSB: KExt = KExt::Ksb;
pub const KUB: KExt = KExt::Kub;
pub const KSH: KExt = KExt::Ksh;
pub const KUH: KExt = KExt::Kuh;

pub const KC: KExt = KExt::Kc;
pub const K0: KExt = KExt::K0;

pub const KE: KExt = KExt::Ke;

// Used as array indices in OPTAB init
const_assert_eq!(KW as usize, 0);
const_assert_eq!(KL as usize, 1);
const_assert_eq!(KS as usize, 2);
const_assert_eq!(KD as usize, 3);

#[derive(Clone, Copy)]
pub struct Op {
    pub name: &'static [u8],
    pub argcls: [[KExt; 4]; 2],
    pub canfold: bool,
}

impl Op {
    pub const fn new(name: &'static [u8], argcls: [[KExt; 4]; 2], canfold: bool) -> Op {
        Op {
            name,
            argcls,
            canfold,
        }
    }
}

#[derive(Clone, new)]
pub struct Ins {
    pub op: O,
    pub cls: KExt, // Must be one of Kw, Kl, Ks, Kd
    pub to: Ref,
    pub args: [Ref; 2],
}

impl Ins {
    pub fn new0(op: O, cls: KExt, to: Ref) -> Ins {
        Ins::new(op, cls, to, [Ref::R, Ref::R])
    }

    pub fn new1(op: O, cls: KExt, to: Ref, args1: [Ref; 1]) -> Ins {
        Ins::new(op, cls, to, [args1[0], Ref::R])
    }

    pub fn new2(op: O, cls: KExt, to: Ref, args2: [Ref; 2]) -> Ins {
        Ins::new(op, cls, to, args2)
    }
}

#[derive(new)]
pub struct Phi {
    pub to: Ref,
    pub args: Vec<Ref>, // TODO would be cool to just have one Vec<(Ref, BlkIdx)>
    pub blks: Vec<BlkIdx>,
    pub cls: KExt,
    pub link: PhiIdx,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhiIdx(pub u32); // Index into Fn::phis

impl PhiIdx {
    pub const INVALID: PhiIdx = PhiIdx(u32::MAX);
}

#[derive(Clone, Copy)]
pub struct BlkJmp {
    pub type_: J,
    pub arg: Ref,
}

impl BlkJmp {
    pub fn new() -> BlkJmp {
        BlkJmp {
            type_: J::Jxxx,
            arg: Ref::R,
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

    pub id: u32,
    pub visit: u32,

    pub idom: BlkIdx, // maybe Vec<BlkIdx>?
    pub dom: BlkIdx,  // maybe Vec<BlkIdx>?
    pub dlink: BlkIdx,
    pub frons: Vec<BlkIdx>,
    pub preds: Vec<BlkIdx>,
    //pub BSet in[1], out[1], gen[1]; // TODO
    pub nlive: [u32; 2],
    pub loop_: u32, // was i32 in QBE
    pub name: Vec<u8>,
}

impl Blk {
    pub fn new(name: &[u8], id: u32, dlink: BlkIdx) -> Blk {
        Blk {
            phi: PhiIdx::INVALID,
            ins: vec![],
            jmp: BlkJmp::new(),
            s1: BlkIdx::INVALID,
            s2: BlkIdx::INVALID,
            link: BlkIdx::INVALID,

            id, // Same as BlkIdx for this block
            visit: 0,

            idom: BlkIdx::INVALID, // maybe Vec<BlkIdx>?
            dom: BlkIdx::INVALID,  // maybe Vec<BlkIdx>?
            dlink,
            frons: vec![],
            preds: vec![],
            //pub BSet in[1], out[1], gen[1]; // TODO
            nlive: [0u32; 2],
            loop_: 0,
            name: name.to_vec(),
        }
    }

    pub fn s1_s2(&self) -> (BlkIdx, BlkIdx) {
        (self.s1, self.s2)
    }
}

// Index into Fn::blks
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlkIdx(pub u32);

impl BlkIdx {
    pub const INVALID: BlkIdx = BlkIdx(u32::MAX);
}

/*
struct Use {
    enum {
        UXXX,
        UPhi,
        UIns,
        UJmp,
    } type;
    uint bid;
    union {
        Ins *ins;
        Phi *phi;
    } u;
};

 */
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum SymT {
    SGlo,
    SThr,
}

#[derive(new, Debug, PartialEq)]
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

/*
enum {
    NoAlias,
    MayAlias,
    MustAlias
};

struct Alias {
    enum {
        ABot = 0,
        ALoc = 1, /* stack local */
        ACon = 2,
        AEsc = 3, /* stack escaping */
        ASym = 4,
        AUnk = 6,
    #define astack(t) ((t) & 1)
    } type;
    int base;
    int64_t offset;
    union {
        Sym sym;
        struct {
            int sz; /* -1 if > NBit */
            bits m;
        } loc;
    } u;
    Alias *slot;
};
 */

#[derive(Debug)]
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

// TODO derive new?
#[derive(Debug)]
pub struct Tmp {
    pub name: Vec<u8>,
    // Ins *def;
    // Use *use;
    pub ndef: u32,
    pub nuse: u32,
    // uint bid; /* id of a defining block */
    // uint cost;
    pub slot: i32, /* -1 for unset */
    pub cls: KExt,
    // struct {
    //     int r;  /* register or -1 */
    //     int w;  /* weight */
    //     bits m; /* avoid these registers */
    // } hint;
    // int phi;
    // Alias alias;
    pub width: TmpWdth,
    // int visit;
}

impl Tmp {
    pub fn new(name: Vec<u8>, ndef: u32, nuse: u32, slot: i32, cls: KExt) -> Tmp {
        Tmp {
            name,
            ndef,
            nuse,
            slot,
            cls,
            width: TmpWdth::WFull,
        }
    }
}

// Index in Fn::tmps
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TmpIdx(pub u32);

impl TmpIdx {
    pub const INVALID: TmpIdx = TmpIdx(u32::MAX);
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ConT {
    CUndef,
    CBits,
    CAddr,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ConBits {
    None,
    I(i64),
    D(f64),
    F(f32),
}

#[derive(new, Debug, PartialEq)]
pub struct Con {
    pub type_: ConT,
    pub sym: Sym,
    pub bits: ConBits,
}

impl Con {
    // TODO - merge bits and sym into same enum, unless sym actual const is imported later...
    pub fn new_sym(sym: Sym) -> Con {
        Con::new(ConT::CAddr, sym, ConBits::None)
    }

    pub fn new_bits(bits: ConBits) -> Con {
        Con::new(ConT::CBits, Sym::UNDEF, bits)
    }
}

// Index in Fn::cons
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConIdx(pub u32);

impl ConIdx {
    pub const INVALID: ConIdx = ConIdx(u32::MAX);
}

#[derive(Debug)]
pub struct Addr {
    // amd64 addressing
    pub offset: Con,
    pub base: Ref,
    pub index: Ref,
    pub scale: i32,
}

pub type Mem = Addr;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MemIdx(pub u32); // Index into Fn::mem

impl MemIdx {
    pub const INVALID: MemIdx = MemIdx(u32::MAX);
}

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
            blks: vec![],
            phis: vec![],
            start: BlkIdx::INVALID,
            tmps: vec![],
            cons: vec![],
            mems: vec![],
            nblk: 0,
            retty: TypIdx::INVALID,
            retr: Ref::R,
            rpo: vec![],
            //bits reg,
            slot: -1, // ???
            vararg: false,
            dynalloc: false,
            name: vec![],
            lnk,
        }
    }

    pub fn blk(&self, bi: BlkIdx) -> &Blk {
        assert!(bi != BlkIdx::INVALID);
        &self.blks[bi.0 as usize]
    }

    pub fn blk_mut(&mut self, bi: BlkIdx) -> &mut Blk {
        assert!(bi != BlkIdx::INVALID);
        &mut self.blks[bi.0 as usize]
    }

    pub fn add_blk(&mut self, b: Blk) -> BlkIdx {
        let bi: BlkIdx = BlkIdx(self.blks.len() as u32);
        self.blks.push(b);
        bi
    }

    pub fn set_blk_link(&mut self, from_bi: BlkIdx, to_bi: BlkIdx) {
        if from_bi == BlkIdx::INVALID {
            self.start = to_bi;
        } else {
            self.blk_mut(from_bi).link = to_bi;
        }
    }

    pub fn phi(&self, pi: PhiIdx) -> &Phi {
        assert!(pi != PhiIdx::INVALID);
        &self.phis[pi.0 as usize]
    }

    pub fn phi_mut(&mut self, pi: PhiIdx) -> &mut Phi {
        assert!(pi != PhiIdx::INVALID);
        &mut self.phis[pi.0 as usize]
    }

    pub fn add_phi(&mut self, p: Phi) -> PhiIdx {
        let pi: PhiIdx = PhiIdx(self.phis.len() as u32);
        self.phis.push(p);
        pi
    }

    pub fn tmp(&self, ti: TmpIdx) -> &Tmp {
        assert!(ti != TmpIdx::INVALID);
        &self.tmps[ti.0 as usize]
    }

    pub fn tmp_mut(&mut self, ti: TmpIdx) -> &mut Tmp {
        assert!(ti != TmpIdx::INVALID);
        &mut self.tmps[ti.0 as usize]
    }

    pub fn add_tmp(&mut self, t: Tmp) -> TmpIdx {
        let ti: TmpIdx = TmpIdx(self.tmps.len() as u32);
        self.tmps.push(t);
        ti
    }

    pub fn con(&self, ci: ConIdx) -> &Con {
        assert!(ci != ConIdx::INVALID);
        &self.cons[ci.0 as usize]
    }

    pub fn con_mut(&mut self, ci: ConIdx) -> &mut Con {
        assert!(ci != ConIdx::INVALID);
        &mut self.cons[ci.0 as usize]
    }

    pub fn add_con(&mut self, c: Con) -> ConIdx {
        let ci: ConIdx = ConIdx(self.cons.len() as u32);
        self.cons.push(c);
        ci
    }

    pub fn mem(&self, mi: MemIdx) -> &Mem {
        assert!(mi != MemIdx::INVALID);
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
pub struct TypIdx(pub u32);

impl TypIdx {
    pub const INVALID: TypIdx = TypIdx(u32::MAX);
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

pub fn bshas(bs: &BSet, elt: u32) -> bool {
    assert!(elt < (bs.len() as u32) * NBIT);
    (bs[(elt / NBIT) as usize] & bit(elt % NBIT)) != 0
}
