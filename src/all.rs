// TODO remove eventually
#![allow(dead_code, unused_variables)]

// TODO - use this more prevalently...
use derive_new::new;
use strum_macros::FromRepr;

use crate::util::InternId;

// Generic Result
pub type RubeError = Box<dyn std::error::Error>;
pub type RubeResult<T> = Result<T, RubeError>;

/*

#include <assert.h>
#include <inttypes.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAKESURE(what, x) typedef char make_sure_##what[(x)?1:-1]
#define die(...) die_(__FILE__, __VA_ARGS__)

typedef unsigned char uchar;
typedef unsigned int uint;
typedef unsigned long ulong;
typedef unsigned long long bits;
 */

pub type Bits = u64;

/*
typedef struct BSet BSet;
typedef struct Ref Ref;
typedef struct Op Op;
typedef struct Ins Ins;
typedef struct Phi Phi;
typedef struct Blk Blk;
typedef struct Use Use;
typedef struct Sym Sym;
typedef struct Alias Alias;
typedef struct Tmp Tmp;
typedef struct Con Con;
typedef struct Addr Mem;
typedef struct Fn Fn;
typedef struct Typ Typ;
typedef struct Field Field;
typedef struct Dat Dat;
typedef struct Lnk Lnk;
typedef struct Target Target;

enum {
    NString = 80,
    NIns    = 1 << 20,
    NAlign  = 3,
    NField  = 32,
    NBit    = CHAR_BIT * sizeof(bits),
};
 */

pub const NBit: usize = 8 * std::mem::size_of::<Bits>();

/*
struct Target {
    char name[16];
    char apple;
    int gpr0;   /* first general purpose reg */
    int ngpr;
    int fpr0;   /* first floating point reg */
    int nfpr;
    bits rglob; /* globally live regs (e.g., sp, fp) */
    int nrglob;
    int *rsave; /* caller-save */
    int nrsave[2];
    bits (*retregs)(Ref, int[2]);
    bits (*argregs)(Ref, int[2]);
    int (*memargs)(int);
    void (*abi0)(Fn *);
    void (*abi1)(Fn *);
    void (*isel)(Fn *);
    void (*emitfn)(Fn *, FILE *);
    void (*emitfin)(FILE *);
    char asloc[4];
    char assym[4];
};
 */

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

/*
#define BIT(n) ((bits)1 << (n))

enum {
    RXX = 0,
    Tmp0 = NBit, /* first non-reg temporary */
};
 */

pub const RXX: usize = 0;
pub const Tmp0: usize = NBit;

/*
struct BSet {
    uint nt;
    bits *t;
};
 */

/*
struct Ref {
    uint type:3;
    uint val:29;
};
 */

// pub struct Ref {
//     pub type_: RefT,
//     pub val: u32,
// }

/*
enum {
    RTmp,
    RCon,
    RInt,
    RType, /* last kind to come out of the parser */
    RSlot,
    RCall,
    RMem,
};
 */

// TODO we can tighten up these types
#[derive(Clone, Copy)]
pub enum Ref {
    R,
    RTmp(TmpIdx),
    RCon(ConIdx),
    RInt(u32),
    RTyp(TypIdx), /* last kind to come out of the parser */
    RSlot(u32),
    RCall(u32),
    RMem(u32),
}

/*
#define R        (Ref){RTmp, 0}
#define UNDEF    (Ref){RCon, 0}  /* represents uninitialized data */
#define CON_Z    (Ref){RCon, 1}
#define TMP(x)   (Ref){RTmp, x}
#define CON(x)   (Ref){RCon, x}
#define SLOT(x)  (Ref){RSlot, (x)&0x1fffffff}
#define TYPE(x)  (Ref){RType, x}
#define CALL(x)  (Ref){RCall, x}
#define MEM(x)   (Ref){RMem, x}
#define INT(x)   (Ref){RInt, (x)&0x1fffffff}

static inline int req(Ref a, Ref b)
{
    return a.type == b.type && a.val == b.val;
}

static inline int rtype(Ref r)
{
    if (req(r, R))
        return -1;
    return r.type;
}

static inline int rsval(Ref r)
{
    return (int32_t)((int64_t)r.val << 3) >> 3;
}

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
#[derive(Clone, Copy, FromRepr, PartialEq)]
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

/*
enum J {
    Jxxx,
#define JMPS(X)                                 \
    X(retw)   X(retl)   X(rets)   X(retd)   \
    X(retsb)  X(retub)  X(retsh)  X(retuh)  \
    X(retc)   X(ret0)   X(jmp)    X(jnz)    \
    X(jfieq)  X(jfine)  X(jfisge) X(jfisgt) \
    X(jfisle) X(jfislt) X(jfiuge) X(jfiugt) \
    X(jfiule) X(jfiult) X(jffeq)  X(jffge)  \
    X(jffgt)  X(jffle)  X(jfflt)  X(jffne)  \
    X(jffo)   X(jffuo)  X(hlt)
#define X(j) J##j,
    JMPS(X)
#undef X
    NJmp
};
 */

// Generated by hand from QBE C code
#[derive(Clone, Copy, FromRepr, PartialEq)]
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

pub fn jmp_for_cls(k: KExt) -> Option<J> {
    if KW <= k && k <= K0 {
        J::from_repr((J::Jretw) as u8 + (k as u8))
    } else {
        None
    }
}

#[repr(u8)]
pub enum ORanges {
    Ocmpw = O::Oceqw as u8,
    Ocmpw1 = O::Ocultw as u8,
    Ocmpl = O::Oceql as u8,
    Ocmpl1 = O::Ocultl as u8,
    Ocmps = O::Oceqs as u8,
    Ocmps1 = O::Ocuos as u8,
    Ocmpd = O::Oceqd as u8,
    Ocmpd1 = O::Ocuod as u8,
    Oalloc = O::Oalloc4 as u8,
    Oalloc1 = O::Oalloc16 as u8,
    Oflag = O::Oflagieq as u8,
    Oflag1 = O::Oflagfuo as u8,
    NPubOp = O::Onop as u8,
    Jjf = J::Jjfieq as u8,
    Jjf1 = J::Jjffuo as u8,
}

/*
#define INRANGE(x, l, u) ((unsigned)(x) - l <= u - l) /* linear in x */
#define isstore(o) INRANGE(o, Ostoreb, Ostored)
#define isload(o) INRANGE(o, Oloadsb, Oload)
#define isext(o) INRANGE(o, Oextsb, Oextuw)
#define ispar(o) INRANGE(o, Opar, Opare)
#define isarg(o) INRANGE(o, Oarg, Oargv)
#define isret(j) INRANGE(j, Jretw, Jret0)
#define isparbh(o) INRANGE(o, Oparsb, Oparuh)
#define isargbh(o) INRANGE(o, Oargsb, Oarguh)
#define isretbh(j) INRANGE(j, Jretsb, Jretuh)
 */

fn in_range_o(x: O, l: O, u: O) -> bool {
    // QBE code uses integer overflow
    // (x as usize) - (l as usize) <= (u as usize) - (l as usize) /* linear in x */
    (l as usize) <= (x as usize) && (x as usize) <= (u as usize)
}

// pub fn isstore(o: O) -> bool {
//     in_range_o(o, O::Ostoreb, O::Ostored)
// }

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
    // QBE code uses integer overflow
    // (x as usize) - (l as usize) <= (u as usize) - (l as usize) /* linear in x */
    (l as usize) <= (x as usize) && (x as usize) <= (u as usize)
}

pub fn isret(j: J) -> bool {
    in_range_j(j, J::Jretw, J::Jret0)
}

pub fn isretbh(j: J) -> bool {
    in_range_j(j, J::Jretsb, J::Jretuh)
}

// Jusgt using Kext as a super-set for now...
// pub enum KBase {
//     Kx = -1, /* "top" class (see usecheck() and clsmerge()) */
//     Kw,
//     Kl,
//     Ks,
//     Kd,
// }

// // Used as array indices in 'optab' etc.
// const_assert_eq!(KBase::Kw as usize, 0);
// const_assert_eq!(KBase::Kl as usize, 1);
// const_assert_eq!(KBase::Ks as usize, 2);
// const_assert_eq!(KBase::Kd as usize, 3);

/*
#define KWIDE(k) ((k)&1)
#define KBASE(k) ((k)>>1)
 */

// pub fn k_wide(k: KBase) -> usize {
//     (k as usize) & 1
// }

// pub fn k_base(k: KBase) -> usize {
//     (k as usize) >> 1
// }

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

/*
struct Op {
    char *name;
    short argcls[2][4];
    int canfold;
};
 */

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

/*
struct Ins {
    uint op:30;
    uint cls:2;
    Ref to;
    Ref arg[2];
};
 */
#[derive(Clone, new)]
pub struct Ins {
    op: O,
    cls: KExt, // Must be one of Kw, Kl, Ks, Kd
    to: Ref,
    arg: [Ref; 2],
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

/*
struct Phi {
    Ref to;
    Ref *arg;
    Blk **blk;
    uint narg;
    int cls;
    Phi *link;
};
 */

#[derive(new)]
pub struct Phi {
    pub to: Ref,
    pub arg: Vec<Ref>,
    pub blk: Vec<BlkIdx>,
    //uint narg;
    pub cls: KExt,
    pub link: PhiIdx,
}

#[derive(Clone, Copy, PartialEq)]
pub struct PhiIdx(pub usize); // Index into Fn::phi

impl PhiIdx {
    pub const INVALID: PhiIdx = PhiIdx(usize::MAX);
}

/*
struct Blk {
    Phi *phi;
    Ins *ins;
    uint nins;
    struct {
        short type;
        Ref arg;
    } jmp;
    Blk *s1;
    Blk *s2;
    Blk *link;

    uint id;
    uint visit;

    Blk *idom;
    Blk *dom, *dlink;
    Blk **fron;
    uint nfron;

    Blk **pred;
    uint npred;
    BSet in[1], out[1], gen[1];
    int nlive[2];
    int loop;
    char name[NString];
};
 */

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
    pub ins: Vec<Ins>, // not gonna work! Maybe InsIdx?
    //pub uint nins;
    pub jmp: BlkJmp,
    pub s1: BlkIdx,
    pub s2: BlkIdx,
    pub link: BlkIdx,

    pub id: usize, // Same as BlkIdx for this block
    pub visit: u32,

    pub idom: BlkIdx, // maybe Vec<BlkIdx>?
    pub dom: BlkIdx,  // maybe Vec<BlkIdx>?
    pub dlink: BlkIdx,
    pub fron: Vec<BlkIdx>,
    //pub uint nfron;
    pub pred: Vec<BlkIdx>,
    //pub uint npred;
    //pub BSet in[1], out[1], gen[1]; // TODO
    pub nlive: [u32; 2],
    pub loop_: bool, // i32?
    pub name: Vec<u8>,
}

impl Blk {
    pub fn new(name: &[u8], id: usize, dlink: BlkIdx) -> Blk {
        Blk {
            phi: PhiIdx::INVALID,
            ins: vec![], // not gonna work! Maybe InsIdx?
            //pub uint nins;
            jmp: BlkJmp::new(),
            s1: BlkIdx::INVALID,
            s2: BlkIdx::INVALID,
            link: BlkIdx::INVALID,

            id, // Same as BlkIdx for this block
            visit: 0,

            idom: BlkIdx::INVALID, // maybe Vec<BlkIdx>?
            dom: BlkIdx::INVALID,  // maybe Vec<BlkIdx>?
            dlink,
            fron: vec![],
            //pub uint nfron;
            pred: vec![],
            //pub uint npred;
            //pub BSet in[1], out[1], gen[1]; // TODO
            nlive: [0u32; 2],
            loop_: false, // i32?
            name: name.to_vec(),
        }
    }
}

// Index into Fn::blks
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlkIdx(pub usize);

impl BlkIdx {
    pub const INVALID: BlkIdx = BlkIdx(usize::MAX);
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

struct Sym {
    enum {
        SGlo,
        SThr,
    } type;
    uint32_t id;
};
 */
#[derive(PartialEq)]
pub enum SymT {
    SGlo,
    SThr,
}

#[derive(new, PartialEq)]
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

// impl Sym {
//     pub fn new(type_: SymT, id: u32) -> Sym {
//         Sym { type_, id }
//     }
// }

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

struct Tmp {
    char name[NString];
    Ins *def;
    Use *use;
    uint ndef, nuse;
    uint bid; /* id of a defining block */
    uint cost;
    int slot; /* -1 for unset */
    short cls;
    struct {
        int r;  /* register or -1 */
        int w;  /* weight */
        bits m; /* avoid these registers */
    } hint;
    int phi;
    Alias alias;
    enum {
        WFull,
        Wsb, /* must match Oload/Oext order */
        Wub,
        Wsh,
        Wuh,
        Wsw,
        Wuw
    } width;
    int visit;
};
 */

enum TmpWdth {
    WFull,
    Wsb, /* must match Oload/Oext order */
    Wub,
    Wsh,
    Wuh,
    Wsw,
    Wuw,
}

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

// Index in Fn::tmp
#[derive(Clone, Copy, PartialEq)]
pub struct TmpIdx(pub usize);

impl TmpIdx {
    pub const INVALID: TmpIdx = TmpIdx(usize::MAX);
}

/*
struct Con {
    enum {
        CUndef,
        CBits,
        CAddr,
    } type;
    Sym sym;
    union {
        int64_t i;
        double d;
        float s;
    } bits;
    char flt; /* 1 to print as s, 2 to print as d */
};
 */

#[derive(PartialEq)]
pub enum ConT {
    CUndef,
    CBits,
    CAddr,
}

#[derive(PartialEq)]
pub enum ConBits {
    None,
    I(i64),
    D(f64),
    F(f32),
}

#[derive(new, PartialEq)]
pub struct Con {
    pub type_: ConT,
    pub sym: Sym,
    pub bits: ConBits,
    // char flt; /* 1 to print as s, 2 to print as d */
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

// Index in Fn::con
#[derive(Clone, Copy)]
pub struct ConIdx(pub usize);

/*
typedef struct Addr Addr;

struct Addr { /* amd64 addressing */
    Con offset;
    Ref base;
    Ref index;
    int scale;
};
 */

pub struct Addr {
    // amd64 addressing
    offset: Con,
    base: Ref,
    index: Ref,
    scale: i32,
}

pub type Mem = Addr;
#[derive(Clone, Copy)]
pub struct MemIdx(pub usize); // Index into Fn::mem

/*
struct Lnk {
    char export;
    char thread;
    char align;
    char *sec;
    char *secf;
};
 */

#[derive(Clone)]
pub struct Lnk {
    pub export: bool,
    pub thread: bool,
    pub align: i8,
    pub sec: Vec<u8>,
    pub secf: Vec<u8>,
}

/*
struct Fn {
    Blk *start;
    Tmp *tmp;
    Con *con;
    Mem *mem;
    int ntmp;
    int ncon;
    int nmem;
    uint nblk;
    int retty; /* index in typ[], -1 if no aggregate return */
    Ref retr;
    Blk **rpo;
    bits reg;
    int slot;
    char vararg;
    char dynalloc;
    char name[NString];
    Lnk lnk;
};
 */

pub struct Fn {
    pub blks: Vec<Blk>,
    pub phis: Vec<Phi>,
    pub start: BlkIdx,
    pub tmp: Vec<Tmp>,
    pub con: Vec<Con>,
    pub mem: Vec<Mem>,
    //pub int ntmp,
    //pub int ncon,
    //pub int nmem,
    //pub uint nblk,
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
            tmp: vec![],
            con: vec![],
            mem: vec![],
            //int ntmp,
            //int ncon,
            //int nmem,
            //uint nblk,
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
}

/*
struct Typ {
char name[NString];
char isdark;
char isunion;
int align;
uint64_t size;
uint nunion;
struct Field {
enum {
FEnd,
Fb,
            Fh,
            Fw,
            Fl,
            Fs,
            Fd,
            FPad,
            FTyp,
        } type;
        uint len; /* or index in typ[] for FTyp */
    } (*fields)[NField+1];
};
 */
#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy)]
pub struct TypIdx(pub usize);

impl TypIdx {
    pub const INVALID: TypIdx = TypIdx(usize::MAX);
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

/*
struct Dat {
    enum {
        DStart,
        DEnd,
        DB,
        DH,
        DW,
        DL,
        DZ
    } type;
    char *name;
    Lnk *lnk;
    union {
        int64_t num;
        double fltd;
        float flts;
        char *str;
        struct {
            char *name;
            int64_t off;
        } ref;
    } u;
    char isref;
    char isstr;
};
 */

#[derive(Debug)]
pub enum DatT {
    DStart,
    DEnd,
    DB,
    DH,
    DW,
    DL,
    DZ,
}

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
    pub fn new(type_: DatT, name: &Vec<u8>, lnk: Lnk) -> Dat {
        Dat {
            type_,
            name: name.clone(),
            lnk: lnk,
            u: DatU::None,
            isref: false,
            isstr: false,
        }
    }
}

/*
/* main.c */
extern Target T;
extern char debug['Z'+1];

/* util.c */
typedef enum {
    PHeap, /* free() necessary */
    PFn, /* discarded after processing the function */
} Pool;

extern Typ *typ;
extern Ins insb[NIns], *curi;
uint32_t hash(char *);
void die_(char *, char *, ...) __attribute__((noreturn));
void *emalloc(size_t);
void *alloc(size_t);
void freeall(void);
void *vnew(ulong, size_t, Pool);
void vfree(void *);
void vgrow(void *, ulong);
void strf(char[NString], char *, ...);
uint32_t intern(char *);
char *str(uint32_t);
int argcls(Ins *, int);
int isreg(Ref);
int iscmp(int, int *, int *);
void emit(int, int, Ref, Ref, Ref);
void emiti(Ins);
void idup(Ins **, Ins *, ulong);
Ins *icpy(Ins *, Ins *, ulong);
int cmpop(int);
int cmpneg(int);
int clsmerge(short *, short);
int phicls(int, Tmp *);
Ref newtmp(char *, int, Fn *);
void chuse(Ref, int, Fn *);
int symeq(Sym, Sym);
Ref newcon(Con *, Fn *);
Ref getcon(int64_t, Fn *);
int addcon(Con *, Con *);
void salloc(Ref, Ref, Fn *);
void dumpts(BSet *, Tmp *, FILE *);

void bsinit(BSet *, uint);
void bszero(BSet *);
uint bscount(BSet *);
void bsset(BSet *, uint);
void bsclr(BSet *, uint);
void bscopy(BSet *, BSet *);
void bsunion(BSet *, BSet *);
void bsinter(BSet *, BSet *);
void bsdiff(BSet *, BSet *);
int bsequal(BSet *, BSet *);
int bsiter(BSet *, int *);

static inline int
bshas(BSet *bs, uint elt)
{
    assert(elt < bs->nt * NBit);
    return (bs->t[elt/NBit] & BIT(elt%NBit)) != 0;
}

/* parse.c */
extern Op optab[NOp];
void parse(FILE *, char *, void (char *), void (Dat *), void (Fn *));
void printfn(Fn *, FILE *);
void printref(Ref, Fn *, FILE *);
void err(char *, ...) __attribute__((noreturn));

/* abi.c */
void elimsb(Fn *);

/* cfg.c */
Blk *newblk(void);
void edgedel(Blk *, Blk **);
void fillpreds(Fn *);
void fillrpo(Fn *);
void filldom(Fn *);
int sdom(Blk *, Blk *);
int dom(Blk *, Blk *);
void fillfron(Fn *);
void loopiter(Fn *, void (*)(Blk *, Blk *));
void fillloop(Fn *);
void simpljmp(Fn *);

/* mem.c */
void promote(Fn *);
void coalesce(Fn *);

/* alias.c */
void fillalias(Fn *);
void getalias(Alias *, Ref, Fn *);
int alias(Ref, int, int, Ref, int, int *, Fn *);
int escapes(Ref, Fn *);

/* load.c */
int loadsz(Ins *);
int storesz(Ins *);
void loadopt(Fn *);

/* ssa.c */
void filluse(Fn *);
void ssa(Fn *);
void ssacheck(Fn *);

/* copy.c */
void copy(Fn *);

/* fold.c */
void fold(Fn *);

/* simpl.c */
void simpl(Fn *);

/* live.c */
void liveon(BSet *, Blk *, Blk *);
void filllive(Fn *);

/* spill.c */
void fillcost(Fn *);
void spill(Fn *);

/* rega.c */
void rega(Fn *);

/* emit.c */
void emitfnlnk(char *, Lnk *, FILE *);
void emitdat(Dat *, FILE *);
void emitdbgfile(char *, FILE *);
void emitdbgloc(uint, uint, FILE *);
int stashbits(void *, int);
void elf_emitfnfin(char *, FILE *);
void elf_emitfin(FILE *);
void macho_emitfin(FILE *);

 */
