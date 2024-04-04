use crate::all::{Fn, KExt, Ref, Tmp, TmpIdx};
use crate::parse::Parser; // ugh for intern()

/*
#include "all.h"
#include <stdarg.h>

typedef struct Bitset Bitset;
typedef struct Vec Vec;
typedef struct Bucket Bucket;

struct Vec {
    ulong mag;
    Pool pool;
    size_t esz;
    ulong cap;
    union {
        long long ll;
        long double ld;
        void *ptr;
    } align[];
};

struct Bucket {
    uint nstr;
    char **str;
};
 */

pub type Bucket = Vec<Vec<u8>>;

/*

enum {
    VMin = 2,
    VMag = 0xcabba9e,
    NPtr = 256,
    IBits = 12,
    IMask = (1<<IBits) - 1,
};
 */

const IBits: usize = 12;
pub const IMask: usize = (1 << IBits) - 1;

/*
Typ *typ;
Ins insb[NIns], *curi;

static void *ptr[NPtr];
static void **pool = ptr;
static int nptr = 1;

static Bucket itbl[IMask+1]; /* string interning table */

uint32_t
hash(char *s)
{
    uint32_t h;

    for (h=0; *s; ++s)
        h = *s + 17*h;
    return h;
}
 */

pub fn hash(s: &[u8]) -> usize {
    let mut h: usize = 0;

    for c in s {
        h = (*c as usize) + 17 * h;
    }

    h
}

/*
void
die_(char *file, char *s, ...)
{
    va_list ap;

    fprintf(stderr, "%s: dying: ", file);
    va_start(ap, s);
    vfprintf(stderr, s, ap);
    va_end(ap);
    fputc('\n', stderr);
    abort();
}

void *
emalloc(size_t n)
{
    void *p;

    p = calloc(1, n);
    if (!p)
        die("emalloc, out of memory");
    return p;
}

void *
alloc(size_t n)
{
    void **pp;

    if (n == 0)
        return 0;
    if (nptr >= NPtr) {
        pp = emalloc(NPtr * sizeof(void *));
        pp[0] = pool;
        pool = pp;
        nptr = 1;
    }
    return pool[nptr++] = emalloc(n);
}

void
freeall()
{
    void **pp;

    for (;;) {
        for (pp = &pool[1]; pp < &pool[nptr]; pp++)
            free(*pp);
        pp = pool[0];
        if (!pp)
            break;
        free(pool);
        pool = pp;
        nptr = NPtr;
    }
    nptr = 1;
}

void *
vnew(ulong len, size_t esz, Pool pool)
{
    void *(*f)(size_t);
    ulong cap;
    Vec *v;

    for (cap=VMin; cap<len; cap*=2)
        ;
    f = pool == PHeap ? emalloc : alloc;
    v = f(cap * esz + sizeof(Vec));
    v->mag = VMag;
    v->cap = cap;
    v->esz = esz;
    v->pool = pool;
    return v + 1;
}

void
vfree(void *p)
{
    Vec *v;

    v = (Vec *)p - 1;
    assert(v->mag == VMag);
    if (v->pool == PHeap) {
        v->mag = 0;
        free(v);
    }
}

void
vgrow(void *vp, ulong len)
{
    Vec *v;
    void *v1;

    v = *(Vec **)vp - 1;
    assert(v+1 && v->mag == VMag);
    if (v->cap >= len)
        return;
    v1 = vnew(len, v->esz, v->pool);
    memcpy(v1, v+1, v->cap * v->esz);
    vfree(v+1);
    *(Vec **)vp = v1;
}

void
strf(char str[NString], char *s, ...)
{
    va_list ap;

    va_start(ap, s);
    vsnprintf(str, NString, s, ap);
    va_end(ap);
}

uint32_t
intern(char *s)
{
    Bucket *b;
    uint32_t h;
    uint i, n;

    h = hash(s) & IMask;
    b = &itbl[h];
    n = b->nstr;

    for (i=0; i<n; i++)
        if (strcmp(s, b->str[i]) == 0)
            return h + (i<<IBits);

    if (n == 1<<(32-IBits))
        die("interning table overflow");
    if (n == 0)
        b->str = vnew(1, sizeof b->str[0], PHeap);
    else if ((n & (n-1)) == 0)
        vgrow(&b->str, n+n);

    b->str[n] = emalloc(strlen(s)+1);
    b->nstr = n + 1;
    strcpy(b->str[n], s);
    return h + (n<<IBits);
}
 */

pub struct InternId(usize);

impl InternId {
    pub const INVALID: InternId = InternId(usize::MAX);
}

pub fn intern(s: &[u8], parser: &mut Parser) -> InternId {
    // Ugh, ownership
    // Bucket *b;
    // uint32_t h;
    // uint i, n;

    let h = hash(s) & IMask;
    let b = &mut parser.itbl[h];
    let n = b.len();

    for i in 0..n {
        if s == b[i] {
            return InternId(h + (i << IBits));
        }
    }

    if n == 1 << (32 - IBits) {
        panic!("interning table overflow");
    }

    // if (n == 0)
    //     b->str = vnew(1, sizeof b->str[0], PHeap);
    // else if ((n & (n-1)) == 0)
    //     vgrow(&b->str, n+n);

    // b->str[n] = emalloc(strlen(s)+1);
    // b->nstr = n + 1;
    // strcpy(b->str[n], s);

    b.push(s.to_vec());

    InternId(h + (n << IBits))
}

/*
char *
str(uint32_t id)
{
    assert(id>>IBits < itbl[id&IMask].nstr);
    return itbl[id&IMask].str[id>>IBits];
}

int
isreg(Ref r)
{
    return rtype(r) == RTmp && r.val < Tmp0;
}

int
iscmp(int op, int *pk, int *pc)
{
    if (Ocmpw <= op && op <= Ocmpw1) {
        *pc = op - Ocmpw;
        *pk = Kw;
    }
    else if (Ocmpl <= op && op <= Ocmpl1) {
        *pc = op - Ocmpl;
        *pk = Kl;
    }
    else if (Ocmps <= op && op <= Ocmps1) {
        *pc = NCmpI + op - Ocmps;
        *pk = Ks;
    }
    else if (Ocmpd <= op && op <= Ocmpd1) {
        *pc = NCmpI + op - Ocmpd;
        *pk = Kd;
    }
    else
        return 0;
    return 1;
}

int
argcls(Ins *i, int n)
{
    return optab[i->op].argcls[n][i->cls];
}

void
emit(int op, int k, Ref to, Ref arg0, Ref arg1)
{
    if (curi == insb)
        die("emit, too many instructions");
    *--curi = (Ins){
        .op = op, .cls = k,
        .to = to, .arg = {arg0, arg1}
    };
}

void
emiti(Ins i)
{
    emit(i.op, i.cls, i.to, i.arg[0], i.arg[1]);
}

void
idup(Ins **pd, Ins *s, ulong n)
{
    *pd = alloc(n * sizeof(Ins));
    if (n)
        memcpy(*pd, s, n * sizeof(Ins));
}

Ins *
icpy(Ins *d, Ins *s, ulong n)
{
    if (n)
        memcpy(d, s, n * sizeof(Ins));
    return d + n;
}

static int cmptab[][2] ={
                 /* negation    swap */
    [Ciule]      = {Ciugt,      Ciuge},
    [Ciult]      = {Ciuge,      Ciugt},
    [Ciugt]      = {Ciule,      Ciult},
    [Ciuge]      = {Ciult,      Ciule},
    [Cisle]      = {Cisgt,      Cisge},
    [Cislt]      = {Cisge,      Cisgt},
    [Cisgt]      = {Cisle,      Cislt},
    [Cisge]      = {Cislt,      Cisle},
    [Cieq]       = {Cine,       Cieq},
    [Cine]       = {Cieq,       Cine},
    [NCmpI+Cfle] = {NCmpI+Cfgt, NCmpI+Cfge},
    [NCmpI+Cflt] = {NCmpI+Cfge, NCmpI+Cfgt},
    [NCmpI+Cfgt] = {NCmpI+Cfle, NCmpI+Cflt},
    [NCmpI+Cfge] = {NCmpI+Cflt, NCmpI+Cfle},
    [NCmpI+Cfeq] = {NCmpI+Cfne, NCmpI+Cfeq},
    [NCmpI+Cfne] = {NCmpI+Cfeq, NCmpI+Cfne},
    [NCmpI+Cfo]  = {NCmpI+Cfuo, NCmpI+Cfo},
    [NCmpI+Cfuo] = {NCmpI+Cfo,  NCmpI+Cfuo},
};

int
cmpneg(int c)
{
    assert(0 <= c && c < NCmp);
    return cmptab[c][0];
}

int
cmpop(int c)
{
    assert(0 <= c && c < NCmp);
    return cmptab[c][1];
}

int
clsmerge(short *pk, short k)
{
    short k1;

    k1 = *pk;
    if (k1 == Kx) {
        *pk = k;
        return 0;
    }
    if ((k1 == Kw && k == Kl) || (k1 == Kl && k == Kw)) {
        *pk = Kw;
        return 0;
    }
    return k1 != k;
}

int
phicls(int t, Tmp *tmp)
{
    int t1;

    t1 = tmp[t].phi;
    if (!t1)
        return t;
    t1 = phicls(t1, tmp);
    tmp[t].phi = t1;
    return t1;
}

Ref
newtmp(char *prfx, int k,  Fn *fn)
{
    static int n;
    int t;

    t = fn->ntmp++;
    vgrow(&fn->tmp, fn->ntmp);
    memset(&fn->tmp[t], 0, sizeof(Tmp));
    if (prfx)
        strf(fn->tmp[t].name, "%s.%d", prfx, ++n);
    fn->tmp[t].cls = k;
    fn->tmp[t].slot = -1;
    fn->tmp[t].nuse = +1;
    fn->tmp[t].ndef = +1;
    return TMP(t);
}
 */

pub fn newtmp(prfx: Option<&[u8]>, k: KExt, fn_: &mut Fn) -> Ref {
    // TODO why a globally unique name?
    static mut n: i32 = 0;
    // int t;

    // t = fn_->ntmp++;
    let t = fn_.tmp.len();
    // vgrow(&fn_->tmp, fn_->ntmp);
    // memset(&fn_->tmp[t], 0, sizeof(Tmp));
    let mut name: Vec<u8> = vec![];
    // Mmm, empty name if there's no prefix?
    if let Some(bytes) = prfx {
        // strf(fn->tmp[t].name, "%s.%d", prfx, ++n);
        name.extend_from_slice(bytes);
        name.push(b'.');
        unsafe {
            // TODO
            n += 1;
            name.extend_from_slice(&format!("{}", n).as_bytes());
        }
    }

    fn_.tmp.push(Tmp::new(
        name, /*ndef*/ 1, /*nuse*/ 1, /*slot*/ -1, /*cls*/ k,
    ));

    // fn->tmp[t].cls = k;
    // fn->tmp[t].slot = -1;
    // fn->tmp[t].nuse = +1;
    // fn->tmp[t].ndef = +1;

    // return TMP(t);
    Ref::RTmp(TmpIdx(t))
}

/*
void
chuse(Ref r, int du, Fn *fn)
{
    if (rtype(r) == RTmp)
        fn->tmp[r.val].nuse += du;
}

int
symeq(Sym s0, Sym s1)
{
    return s0.type == s1.type && s0.id == s1.id;
}

Ref
newcon(Con *c0, Fn *fn)
{
    Con *c1;
    int i;

    for (i=1; i<fn->ncon; i++) {
        c1 = &fn->con[i];
        if (c0->type == c1->type
        && symeq(c0->sym, c1->sym)
        && c0->bits.i == c1->bits.i)
            return CON(i);
    }
    vgrow(&fn->con, ++fn->ncon);
    fn->con[i] = *c0;
    return CON(i);
}

Ref
getcon(int64_t val, Fn *fn)
{
    int c;

    for (c=1; c<fn->ncon; c++)
        if (fn->con[c].type == CBits
        && fn->con[c].bits.i == val)
            return CON(c);
    vgrow(&fn->con, ++fn->ncon);
    fn->con[c] = (Con){.type = CBits, .bits.i = val};
    return CON(c);
}

int
addcon(Con *c0, Con *c1)
{
    if (c0->type == CUndef)
        *c0 = *c1;
    else {
        if (c1->type == CAddr) {
            if (c0->type == CAddr)
                return 0;
            c0->type = CAddr;
            c0->sym = c1->sym;
        }
        c0->bits.i += c1->bits.i;
    }
    return 1;
}

void
salloc(Ref rt, Ref rs, Fn *fn)
{
    Ref r0, r1;
    int64_t sz;

    /* we need to make sure
     * the stack remains aligned
     * (rsp = 0) mod 16
     */
    fn->dynalloc = 1;
    if (rtype(rs) == RCon) {
        sz = fn->con[rs.val].bits.i;
        if (sz < 0 || sz >= INT_MAX-15)
            err("invalid alloc size %"PRId64, sz);
        sz = (sz + 15)  & -16;
        emit(Osalloc, Kl, rt, getcon(sz, fn), R);
    } else {
        /* r0 = (r + 15) & -16 */
        r0 = newtmp("isel", Kl, fn);
        r1 = newtmp("isel", Kl, fn);
        emit(Osalloc, Kl, rt, r0, R);
        emit(Oand, Kl, r0, r1, getcon(-16, fn));
        emit(Oadd, Kl, r1, rs, getcon(15, fn));
        if (fn->tmp[rs.val].slot != -1)
            err("unlikely alloc argument %%%s for %%%s",
                fn->tmp[rs.val].name, fn->tmp[rt.val].name);
    }
}

void
bsinit(BSet *bs, uint n)
{
    n = (n + NBit-1) / NBit;
    bs->nt = n;
    bs->t = alloc(n * sizeof bs->t[0]);
}

MAKESURE(NBit_is_64, NBit == 64);
inline static uint
popcnt(bits b)
{
    b = (b & 0x5555555555555555) + ((b>>1) & 0x5555555555555555);
    b = (b & 0x3333333333333333) + ((b>>2) & 0x3333333333333333);
    b = (b & 0x0f0f0f0f0f0f0f0f) + ((b>>4) & 0x0f0f0f0f0f0f0f0f);
    b += (b>>8);
    b += (b>>16);
    b += (b>>32);
    return b & 0xff;
}

inline static int
firstbit(bits b)
{
    int n;

    n = 0;
    if (!(b & 0xffffffff)) {
        n += 32;
        b >>= 32;
    }
    if (!(b & 0xffff)) {
        n += 16;
        b >>= 16;
    }
    if (!(b & 0xff)) {
        n += 8;
        b >>= 8;
    }
    if (!(b & 0xf)) {
        n += 4;
        b >>= 4;
    }
    n += (char[16]){4,0,1,0,2,0,1,0,3,0,1,0,2,0,1,0}[b & 0xf];
    return n;
}

uint
bscount(BSet *bs)
{
    uint i, n;

    n = 0;
    for (i=0; i<bs->nt; i++)
        n += popcnt(bs->t[i]);
    return n;
}

static inline uint
bsmax(BSet *bs)
{
    return bs->nt * NBit;
}

void
bsset(BSet *bs, uint elt)
{
    assert(elt < bsmax(bs));
    bs->t[elt/NBit] |= BIT(elt%NBit);
}

void
bsclr(BSet *bs, uint elt)
{
    assert(elt < bsmax(bs));
    bs->t[elt/NBit] &= ~BIT(elt%NBit);
}

#define BSOP(f, op)                           \
    void                                  \
    f(BSet *a, BSet *b)                   \
    {                                     \
        uint i;                       \
                                      \
        assert(a->nt == b->nt);       \
        for (i=0; i<a->nt; i++)       \
            a->t[i] op b->t[i];   \
    }

BSOP(bscopy, =)
BSOP(bsunion, |=)
BSOP(bsinter, &=)
BSOP(bsdiff, &= ~)

int
bsequal(BSet *a, BSet *b)
{
    uint i;

    assert(a->nt == b->nt);
    for (i=0; i<a->nt; i++)
        if (a->t[i] != b->t[i])
            return 0;
    return 1;
}

void
bszero(BSet *bs)
{
    memset(bs->t, 0, bs->nt * sizeof bs->t[0]);
}

/* iterates on a bitset, use as follows
 *
 * 	for (i=0; bsiter(set, &i); i++)
 * 		use(i);
 *
 */
int
bsiter(BSet *bs, int *elt)
{
    bits b;
    uint t, i;

    i = *elt;
    t = i/NBit;
    if (t >= bs->nt)
        return 0;
    b = bs->t[t];
    b &= ~(BIT(i%NBit) - 1);
    while (!b) {
        ++t;
        if (t >= bs->nt)
            return 0;
        b = bs->t[t];
    }
    *elt = NBit*t + firstbit(b);
    return 1;
}

void
dumpts(BSet *bs, Tmp *tmp, FILE *f)
{
    int t;

    fprintf(f, "[");
    for (t=Tmp0; bsiter(bs, &t); t++)
        fprintf(f, " %s", tmp[t].name);
    fprintf(f, " ]\n");
}
 */
