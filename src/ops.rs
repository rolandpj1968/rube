/*********************/
/* PUBLIC OPERATIONS */
/*********************/

/* Arithmetic and Bits */
op!((add,     T(w,l,s,d, w,l,s,d), 1) X(2, 1, 0) V(1));
op!((sub,     T(w,l,s,d, w,l,s,d), 1) X(2, 1, 0) V(0));
op!((neg,     T(w,l,s,d, x,x,x,x), 1) X(1, 1, 0) V(0));
op!((div,     T(w,l,s,d, w,l,s,d), 1) X(0, 0, 0) V(0));
op!((rem,     T(w,l,e,e, w,l,e,e), 1) X(0, 0, 0) V(0));
op!((udiv,    T(w,l,e,e, w,l,e,e), 1) X(0, 0, 0) V(0));
op!((urem,    T(w,l,e,e, w,l,e,e), 1) X(0, 0, 0) V(0));
op!((mul,     T(w,l,s,d, w,l,s,d), 1) X(2, 0, 0) V(0));
op!((and,     T(w,l,e,e, w,l,e,e), 1) X(2, 1, 0) V(1));
op!((or,      T(w,l,e,e, w,l,e,e), 1) X(2, 1, 0) V(1));
op!((xor,     T(w,l,e,e, w,l,e,e), 1) X(2, 1, 0) V(1));
op!((sar,     T(w,l,e,e, w,w,e,e), 1) X(1, 1, 0) V(1));
op!((shr,     T(w,l,e,e, w,w,e,e), 1) X(1, 1, 0) V(1));
op!((shl,     T(w,l,e,e, w,w,e,e), 1) X(1, 1, 0) V(1));

/* Comparisons */
op!((ceqw,    T(w,w,e,e, w,w,e,e), 1) X(0, 1, 0) V(0));
op!((cnew,    T(w,w,e,e, w,w,e,e), 1) X(0, 1, 0) V(0));
op!((csgew,   T(w,w,e,e, w,w,e,e), 1) X(0, 1, 0) V(0));
op!((csgtw,   T(w,w,e,e, w,w,e,e), 1) X(0, 1, 0) V(0));
op!((cslew,   T(w,w,e,e, w,w,e,e), 1) X(0, 1, 0) V(0));
op!((csltw,   T(w,w,e,e, w,w,e,e), 1) X(0, 1, 0) V(1));
op!((cugew,   T(w,w,e,e, w,w,e,e), 1) X(0, 1, 0) V(0));
op!((cugtw,   T(w,w,e,e, w,w,e,e), 1) X(0, 1, 0) V(0));
op!((culew,   T(w,w,e,e, w,w,e,e), 1) X(0, 1, 0) V(0));
op!((cultw,   T(w,w,e,e, w,w,e,e), 1) X(0, 1, 0) V(1));

op!((ceql,    T(l,l,e,e, l,l,e,e), 1) X(0, 1, 0) V(0));
op!((cnel,    T(l,l,e,e, l,l,e,e), 1) X(0, 1, 0) V(0));
op!((csgel,   T(l,l,e,e, l,l,e,e), 1) X(0, 1, 0) V(0));
op!((csgtl,   T(l,l,e,e, l,l,e,e), 1) X(0, 1, 0) V(0));
op!((cslel,   T(l,l,e,e, l,l,e,e), 1) X(0, 1, 0) V(0));
op!((csltl,   T(l,l,e,e, l,l,e,e), 1) X(0, 1, 0) V(1));
op!((cugel,   T(l,l,e,e, l,l,e,e), 1) X(0, 1, 0) V(0));
op!((cugtl,   T(l,l,e,e, l,l,e,e), 1) X(0, 1, 0) V(0));
op!((culel,   T(l,l,e,e, l,l,e,e), 1) X(0, 1, 0) V(0));
op!((cultl,   T(l,l,e,e, l,l,e,e), 1) X(0, 1, 0) V(1));

op!((ceqs,    T(s,s,e,e, s,s,e,e), 1) X(0, 1, 0) V(0));
op!((cges,    T(s,s,e,e, s,s,e,e), 1) X(0, 1, 0) V(0));
op!((cgts,    T(s,s,e,e, s,s,e,e), 1) X(0, 1, 0) V(0));
op!((cles,    T(s,s,e,e, s,s,e,e), 1) X(0, 1, 0) V(0));
op!((clts,    T(s,s,e,e, s,s,e,e), 1) X(0, 1, 0) V(0));
op!((cnes,    T(s,s,e,e, s,s,e,e), 1) X(0, 1, 0) V(0));
op!((cos,     T(s,s,e,e, s,s,e,e), 1) X(0, 1, 0) V(0));
op!((cuos,    T(s,s,e,e, s,s,e,e), 1) X(0, 1, 0) V(0));

op!((ceqd,    T(d,d,e,e, d,d,e,e), 1) X(0, 1, 0) V(0));
op!((cged,    T(d,d,e,e, d,d,e,e), 1) X(0, 1, 0) V(0));
op!((cgtd,    T(d,d,e,e, d,d,e,e), 1) X(0, 1, 0) V(0));
op!((cled,    T(d,d,e,e, d,d,e,e), 1) X(0, 1, 0) V(0));
op!((cltd,    T(d,d,e,e, d,d,e,e), 1) X(0, 1, 0) V(0));
op!((cned,    T(d,d,e,e, d,d,e,e), 1) X(0, 1, 0) V(0));
op!((cod,     T(d,d,e,e, d,d,e,e), 1) X(0, 1, 0) V(0));
op!((cuod,    T(d,d,e,e, d,d,e,e), 1) X(0, 1, 0) V(0));

/* Memory */
op!((storeb,  T(w,e,e,e, m,e,e,e), 0) X(0, 0, 1) V(0));
op!((storeh,  T(w,e,e,e, m,e,e,e), 0) X(0, 0, 1) V(0));
op!((storew,  T(w,e,e,e, m,e,e,e), 0) X(0, 0, 1) V(0));
op!((storel,  T(l,e,e,e, m,e,e,e), 0) X(0, 0, 1) V(0));
op!((stores,  T(s,e,e,e, m,e,e,e), 0) X(0, 0, 1) V(0));
op!((stored,  T(d,e,e,e, m,e,e,e), 0) X(0, 0, 1) V(0));

op!((loadsb,  T(m,m,e,e, x,x,e,e), 0) X(0, 0, 1) V(0));
op!((loadub,  T(m,m,e,e, x,x,e,e), 0) X(0, 0, 1) V(0));
op!((loadsh,  T(m,m,e,e, x,x,e,e), 0) X(0, 0, 1) V(0));
op!((loaduh,  T(m,m,e,e, x,x,e,e), 0) X(0, 0, 1) V(0));
op!((loadsw,  T(m,m,e,e, x,x,e,e), 0) X(0, 0, 1) V(0));
op!((loaduw,  T(m,m,e,e, x,x,e,e), 0) X(0, 0, 1) V(0));
op!((load,    T(m,m,m,m, x,x,x,x), 0) X(0, 0, 1) V(0));

/* Extensions and Truncations */
op!((extsb,   T(w,w,e,e, x,x,e,e), 1) X(0, 0, 1) V(0));
op!((extub,   T(w,w,e,e, x,x,e,e), 1) X(0, 0, 1) V(0));
op!((extsh,   T(w,w,e,e, x,x,e,e), 1) X(0, 0, 1) V(0));
op!((extuh,   T(w,w,e,e, x,x,e,e), 1) X(0, 0, 1) V(0));
op!((extsw,   T(e,w,e,e, e,x,e,e), 1) X(0, 0, 1) V(0));
op!((extuw,   T(e,w,e,e, e,x,e,e), 1) X(0, 0, 1) V(0));

op!((exts,    T(e,e,e,s, e,e,e,x), 1) X(0, 0, 1) V(0));
op!((truncd,  T(e,e,d,e, e,e,x,e), 1) X(0, 0, 1) V(0));
op!((stosi,   T(s,s,e,e, x,x,e,e), 1) X(0, 0, 1) V(0));
op!((stoui,   T(s,s,e,e, x,x,e,e), 1) X(0, 0, 1) V(0));
op!((dtosi,   T(d,d,e,e, x,x,e,e), 1) X(0, 0, 1) V(0));
op!((dtoui,   T(d,d,e,e, x,x,e,e), 1) X(0, 0, 1) V(0));
op!((swtof,   T(e,e,w,w, e,e,x,x), 1) X(0, 0, 1) V(0));
op!((uwtof,   T(e,e,w,w, e,e,x,x), 1) X(0, 0, 1) V(0));
op!((sltof,   T(e,e,l,l, e,e,x,x), 1) X(0, 0, 1) V(0));
op!((ultof,   T(e,e,l,l, e,e,x,x), 1) X(0, 0, 1) V(0));
op!((cast,    T(s,d,w,l, x,x,x,x), 1) X(0, 0, 1) V(0));

/* Stack Allocation */
op!((alloc4,  T(e,l,e,e, e,x,e,e), 0) X(0, 0, 0) V(0));
op!((alloc8,  T(e,l,e,e, e,x,e,e), 0) X(0, 0, 0) V(0));
op!((alloc16, T(e,l,e,e, e,x,e,e), 0) X(0, 0, 0) V(0));

/* Variadic Function Helpers */
op!((vaarg,   T(m,m,m,m, x,x,x,x), 0) X(0, 0, 0) V(0));
op!((vastart, T(m,e,e,e, x,e,e,e), 0) X(0, 0, 0) V(0));

op!((copy,    T(w,l,s,d, x,x,x,x), 0) X(0, 0, 1) V(0));

/* Debug */
op!((dbgloc,  T(w,e,e,e, w,e,e,e), 0) X(0, 0, 1) V(0));

/****************************************/
/* INTERNAL OPERATIONS (keep nop first) */
/****************************************/

/* Miscellaneous and Architecture-Specific Operations */
op!((nop, T(x, x, x, x, x, x, x, x), 0), X(0, 0, 1), V(0));
op!((addr, T(m, m, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((blit0, T(m, e, e, e, m, e, e, e), 0), X(0, 1, 0), V(0));
op!((blit1, T(w, e, e, e, x, e, e, e), 0), X(0, 1, 0), V(0));
op!((swap, T(w, l, s, d, w, l, s, d), 0), X(1, 0, 0), V(0));
op!((sign, T(w, l, e, e, x, x, e, e), 0), X(0, 0, 0), V(0));
op!((salloc, T(e, l, e, e, e, x, e, e), 0), X(0, 0, 0), V(0));
op!((xidiv, T(w, l, e, e, x, x, e, e), 0), X(1, 0, 0), V(0));
op!((xdiv, T(w, l, e, e, x, x, e, e), 0), X(1, 0, 0), V(0));
op!((xcmp, T(w, l, s, d, w, l, s, d), 0), X(1, 1, 0), V(0));
op!((xtest, T(w, l, e, e, w, l, e, e), 0), X(1, 1, 0), V(0));
op!((acmp, T(w, l, e, e, w, l, e, e), 0), X(0, 0, 0), V(0));
op!((acmn, T(w, l, e, e, w, l, e, e), 0), X(0, 0, 0), V(0));
op!((afcmp, T(e, e, s, d, e, e, s, d), 0), X(0, 0, 0), V(0));
op!((reqz, T(w, l, e, e, x, x, e, e), 0), X(0, 0, 0), V(0));
op!((rnez, T(w, l, e, e, x, x, e, e), 0), X(0, 0, 0), V(0));

/* Arguments, Parameters, and Calls */
op!((par, T(x, x, x, x, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((parsb, T(x, x, x, x, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((parub, T(x, x, x, x, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((parsh, T(x, x, x, x, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((paruh, T(x, x, x, x, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((parc, T(e, x, e, e, e, x, e, e), 0), X(0, 0, 0), V(0));
op!((pare, T(e, x, e, e, e, x, e, e), 0), X(0, 0, 0), V(0));
op!((arg, T(w, l, s, d, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((argsb, T(w, e, e, e, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((argub, T(w, e, e, e, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((argsh, T(w, e, e, e, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((arguh, T(w, e, e, e, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((argc, T(e, x, e, e, e, l, e, e), 0), X(0, 0, 0), V(0));
op!((arge, T(e, l, e, e, e, x, e, e), 0), X(0, 0, 0), V(0));
op!((argv, T(x, x, x, x, x, x, x, x), 0), X(0, 0, 0), V(0));
op!((call, T(m, m, m, m, x, x, x, x), 0), X(0, 0, 0), V(0));

/* Flags Setting */
op!((flagieq, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagine, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagisge, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagisgt, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagisle, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagislt, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagiuge, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagiugt, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagiule, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagiult, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagfeq, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagfge, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagfgt, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagfle, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagflt, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagfne, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagfo, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
op!((flagfuo, T(x, x, e, e, x, x, e, e), 0), X(0, 0, 1), V(0));
