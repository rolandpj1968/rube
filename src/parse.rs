// TODO remove eventually
#![allow(dead_code, unused_variables)]

use std::ascii::escape_default;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Bytes, Read};
use std::path::Path;

use chomp1::ascii::{is_alpha, is_digit, is_whitespace};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::all::{
    Blk, BlkIdx, Con, ConBits, ConT, Dat, DatT, DatU, Fn, KExt, Lnk, ORanges, RubeResult, Sym,
    SymT, Target, Tmp0, Typ, TypFld, TypFldT, TypIdx, J, K0, KC, KD, KL, KS, KSB, KSH, KUB, KUH,
    KW, O,
};
use crate::optab::OPTAB;
use crate::util::{hash, newtmp};

#[derive(Debug)]
struct ParseError {
    msg: String,
}

impl ParseError {
    fn new(msg: String) -> ParseError {
        ParseError { msg }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.msg
    }
}

/*

#include "all.h"
#include <ctype.h>
#include <stdarg.h>
 */

// Note KExt moved to all.rs

// Note optab moved to optab.rs
/*
Op optab[NOp] = {
#define O(op, t, cf) [O##op]={#op, t, cf},
    #include "ops.h"
};
 */

#[derive(PartialEq)]
enum PState {
    PXXX,
    PLbl,
    PPhi,
    PIns,
    PEnd,
}

#[derive(Clone, Copy, EnumIter, PartialEq)]
enum Token {
    Txxx = 0,

    /* aliases */
    Tloadw = ORanges::NPubOp as isize,
    Tloadl,
    Tloads,
    Tloadd,
    Talloc1,
    Talloc2,

    Tblit,
    Tcall,
    Tenv,
    Tphi,
    Tjmp,
    Tjnz,
    Tret,
    Thlt,
    Texport,
    Tthread,
    Tfunc,
    Ttype,
    Tdata,
    Tsection,
    Talign,
    Tdbgfile,
    Tl,
    Tw,
    Tsh,
    Tuh,
    Th,
    Tsb,
    Tub,
    Tb,
    Td,
    Ts,
    Tz,

    Tint,
    Tflts,
    Tfltd,
    Ttmp,
    Tlbl,
    Tglo,
    Ttyp,
    Tstr,

    Tplus,
    Teq,
    Tcomma,
    Tlparen,
    Trparen,
    Tlbrace,
    Trbrace,
    Tnl,
    Tdots,
    Teof,

    Ntok,
}

/*
static char *kwmap[Ntok] = {
    [Tloadw] = "loadw",
    [Tloadl] = "loadl",
    [Tloads] = "loads",
    [Tloadd] = "loadd",
    [Talloc1] = "alloc1",
    [Talloc2] = "alloc2",
    [Tblit] = "blit",
    [Tcall] = "call",
    [Tenv] = "env",
    [Tphi] = "phi",
    [Tjmp] = "jmp",
    [Tjnz] = "jnz",
    [Tret] = "ret",
    [Thlt] = "hlt",
    [Texport] = "export",
    [Tthread] = "thread",
    [Tfunc] = "function",
    [Ttype] = "type",
    [Tdata] = "data",
    [Tsection] = "section",
    [Talign] = "align",
    [Tdbgfile] = "dbgfile",
    [Tsb] = "sb",
    [Tub] = "ub",
    [Tsh] = "sh",
    [Tuh] = "uh",
    [Tb] = "b",
    [Th] = "h",
    [Tw] = "w",
    [Tl] = "l",
    [Ts] = "s",
    [Td] = "d",
    [Tz] = "z",
    [Tdots] = "...",
};
 */

lazy_static! {
    static ref KWMAP: [&'static [u8]; O::NOp as usize] = {
    // Crazy indent courtesy of emacs rustic.
    let mut kwmap0: [&'static [u8]; O::NOp as usize] = [b""; O::NOp as usize];

    kwmap0[Token::Tloadw as usize] = b"loadw";
    kwmap0[Token::Tloadl as usize] = b"loadl";
    kwmap0[Token::Tloads as usize] = b"loads";
    kwmap0[Token::Tloadd as usize] = b"loadd";
    kwmap0[Token::Talloc1 as usize] = b"alloc1";
    kwmap0[Token::Talloc2 as usize] = b"alloc2";
    kwmap0[Token::Tblit as usize] = b"blit";
    kwmap0[Token::Tcall as usize] = b"call";
    kwmap0[Token::Tenv as usize] = b"env";
    kwmap0[Token::Tphi as usize] = b"phi";
    kwmap0[Token::Tjmp as usize] = b"jmp";
    kwmap0[Token::Tjnz as usize] = b"jnz";
    kwmap0[Token::Tret as usize] = b"ret";
    kwmap0[Token::Thlt as usize] = b"hlt";
    kwmap0[Token::Texport as usize] = b"export";
    kwmap0[Token::Tthread as usize] = b"thread";
    kwmap0[Token::Tfunc as usize] = b"function";
    kwmap0[Token::Ttype as usize] = b"type";
    kwmap0[Token::Tdata as usize] = b"data";
    kwmap0[Token::Tsection as usize] = b"section";
    kwmap0[Token::Talign as usize] = b"align";
    kwmap0[Token::Tdbgfile as usize] = b"dbgfile";
    kwmap0[Token::Tsb as usize] = b"sb";
    kwmap0[Token::Tub as usize] = b"ub";
    kwmap0[Token::Tsh as usize] = b"sh";
    kwmap0[Token::Tuh as usize] = b"uh";
    kwmap0[Token::Tb as usize] = b"b";
    kwmap0[Token::Th as usize] = b"h";
    kwmap0[Token::Tw as usize] = b"w";
    kwmap0[Token::Tl as usize] = b"l";
    kwmap0[Token::Ts as usize] = b"s";
    kwmap0[Token::Td as usize] = b"d";
    kwmap0[Token::Tz as usize] = b"z";
    kwmap0[Token::Tdots as usize] = b"...";

    // formerly in lexinit()
    for i in 0..(ORanges::NPubOp as usize) {
            if !OPTAB[i].name.is_empty() {
        kwmap0[i] = OPTAB[i].name;
            }
    }

    kwmap0
    };
}

const NPRED: usize = 63;
const TMASK: usize = 16383; /* for temps hash */
const BMASK: usize = 8191; /* for blocks hash */
const K: usize = 9583425; /* found using tools/lexh.c */
const M: usize = 23;

/*
static uchar lexh[1 << (32-M)];
static FILE *inf;
static char *inpath;
static int thead;

static struct {
    char chr;
    double fltd;
    float flts;
    int64_t num;
    char *str;
} tokval;
 */

struct TokVal {
    chr: Option<u8>, // None on EOF (or uninit)
    fltd: f64,
    flts: f32,
    num: i64,
    str: Vec<u8>,
}

impl TokVal {
    fn new() -> TokVal {
        TokVal {
            chr: None,
            fltd: 0.0,
            flts: 0.0,
            num: 0,
            str: vec![],
        }
    }
}

/*
static int lnum;

static Fn *curf;
static int tmph[TMask+1];
static Phi **plink;
static Blk *curb;
static Blk **blink;
static Blk *blkh[BMask+1];
static int nblk;
static int rcls;
static uint ntyp;
 */

struct Parser<'a> {
    T: &'a Target,
    inf: Bytes<BufReader<&'a File>>,
    ungetc: Option<u8>,
    inpath: &'a Path,
    thead: Token,
    tokval: TokVal,
    lnum: i32,
    //curf: Option<Fn>,
    tmph: [i32; TMASK + 1],
    //static Phi **plink;
    curb: Option<BlkIdx>,
    //static Blk **blink;
    blkh: [BlkIdx; BMASK + 1],
    //nblk: i32,
    rcls: KExt,
    //ntyp: u32,
    typ: Vec<Typ>, // from util.c
}

/*
void
err(char *s, ...)
{
    va_list ap;

    va_start(ap, s);
    fprintf(stderr, "qbe:%s:%d: ", inpath, lnum);
    vfprintf(stderr, s, ap);
    fprintf(stderr, "\n");
    va_end(ap);
    exit(1);
}
 */

impl Parser<'_> {
    fn err(&self, s: &str) -> Box<ParseError> {
        Box::new(ParseError::new(format!(
            "qbe:{}:{}: {}",
            self.inpath.display(),
            self.lnum,
            s
        )))
    }
}

/*
static void
lexinit()
{
    static int done;
    int i;
    long h;

    if (done)
        return;
    for (i=0; i<NPubOp; ++i)
        if (optab[i].name)
            kwmap[i] = optab[i].name;
    assert(Ntok <= UCHAR_MAX);
    for (i=0; i<Ntok; ++i)
        if (kwmap[i]) {
            h = hash(kwmap[i])*K >> M;
            assert(lexh[h] == Txxx);
            lexh[h] = i;
        }
    done = 1;
}
 */

lazy_static! {
    static ref LEXH: [Token; 1 << (32 - M)] = {
        let mut lexh0: [Token; 1 << (32 - M)] = [Token::Txxx; 1 << (32 - M)];

        for t in Token::iter() {
            let i = t as usize;
            if !KWMAP[i].is_empty() {
                let h = ((hash(KWMAP[i]) as usize) * K) >> M;
                assert!(lexh0[h] == Token::Txxx);
                lexh0[h] = t;
            }
        }

        lexh0
    };
}

/*
static int64_t
getint()
{
    uint64_t n;
    int c, m;

    n = 0;
    c = fgetc(inf);
    m = (c == '-');
    if (m || c == '+')
        c = fgetc(inf);
    do {
        n = 10*n + (c - '0');
        c = fgetc(inf);
    } while ('0' <= c && c <= '9');
    ungetc(c, inf);
    if (m)
        n = 1 + ~n;
    return *(int64_t *)&n;
}
 */

impl Parser<'_> {
    // Why return i64??? TODO ask QBE
    fn getint(&mut self) -> RubeResult<i64> {
        //uint64_t n;
        //int c, m;

        let mut n: u64 = 0;
        let mut c = self.getc()?;
        if c.is_none() {
            return Err(self.err("end-of-file expecting integer constant"));
        }
        let mut craw = c.unwrap();
        let m = craw == b'-';
        if m || craw == b'+' {
            c = self.getc()?;
            if c.is_none() {
                return Err(self.err("end-of-file in integer constant"));
            }
        }
        loop {
            //do {
            // ugh integer overflow TODO
            n = n.wrapping_mul(10).wrapping_add((craw - b'0') as u64);
            c = self.getc()?;
            match c {
                None => break,
                Some(craw0) => {
                    craw = craw0;
                    if craw < b'0' || craw > b'9' {
                        break;
                    }
                }
            }
        }
        self.ungetc(c);
        if m {
            n = 1u64.wrapping_add(!n);
        }

        Ok(n as i64)
    }
}

impl Parser<'_> {
    // TODO - this is wrong cos it will slurp the next comma - rather do alphanum and '.'
    fn take_non_ws_as_utf8(&mut self) -> RubeResult<String> {
        let mut bytes: Vec<u8> = vec![];

        loop {
            let c = self.getc()?;

            if c.is_none() || !is_whitespace(c.unwrap()) {
                self.ungetc(c);
                break;
            }

            bytes.push(c.unwrap());
        }

        match String::from_utf8(bytes.clone()) {
            Ok(s) => Ok(s),
            Err(_) => Err(self.err(&format!(
                "invalid characters in floating point literal {:?}",
                String::from_utf8_lossy(&bytes)
            ))),
        }
    }

    fn get_float(&mut self) -> RubeResult<f32> {
        let s = self.take_non_ws_as_utf8()?;
        let f: Result<f32, _> = s.parse();
        match f {
            Ok(v) => Ok(v),
            Err(_) => Err(self.err(&format!("invalid float literal {:?}", s))),
        }
    }

    fn get_double(&mut self) -> RubeResult<f64> {
        let s = self.take_non_ws_as_utf8()?;
        let f: Result<f64, _> = s.parse();
        match f {
            Ok(v) => Ok(v),
            Err(_) => Err(self.err(&format!("invalid double literal {:?}", s))),
        }
    }
}

/*
static int
lex()
{
    static char tok[NString];
    int c, i, esc;
    int t;

    do
        c = fgetc(inf);
    while (isblank(c));
    t = Txxx;
    tokval.chr = c;
    switch (c) {
    case EOF:
        return Teof;
    case ',':
        return Tcomma;
    case '(':
        return Tlparen;
    case ')':
        return Trparen;
    case '{':
        return Tlbrace;
    case '}':
        return Trbrace;
    case '=':
        return Teq;
    case '+':
        return Tplus;
    case 's':
        if (fscanf(inf, "_%f", &tokval.flts) != 1)
            break;
        return Tflts;
    case 'd':
        if (fscanf(inf, "_%lf", &tokval.fltd) != 1)
            break;
        return Tfltd;
    case '%':
        t = Ttmp;
        c = fgetc(inf);
        goto Alpha;
    case '@':
        t = Tlbl;
        c = fgetc(inf);
        goto Alpha;
    case '$':
        t = Tglo;
        if ((c = fgetc(inf)) == '"')
            goto Quoted;
        goto Alpha;
    case ':':
        t = Ttyp;
        c = fgetc(inf);
        goto Alpha;
    case '#':
        while ((c=fgetc(inf)) != '\n' && c != EOF)
            ;
        /* fall through */
	case '\n':
		lnum++;
		return Tnl;
	}
	if (isdigit(c) || c == '-' || c == '+') {
		ungetc(c, inf);
		tokval.num = getint();
		return Tint;
	}
	if (c == '"') {
		t = Tstr;
	Quoted:
		tokval.str = vnew(2, 1, PFn);
		tokval.str[0] = c;
		esc = 0;
		for (i=1;; i++) {
			c = fgetc(inf);
			if (c == EOF)
				err("unterminated string");
			vgrow(&tokval.str, i+2);
			tokval.str[i] = c;
			if (c == '"' && !esc) {
				tokval.str[i+1] = 0;
				return t;
			}
			esc = (c == '\\' && !esc);
		}
	}
Alpha:
	if (!isalpha(c) && c != '.' && c != '_')
		err("invalid character %c (%d)", c, c);
	i = 0;
	do {
		if (i >= NString-1)
			err("identifier too long");
		tok[i++] = c;
		c = fgetc(inf);
	} while (isalpha(c) || c == '$' || c == '.' || c == '_' || isdigit(c));
	tok[i] = 0;
	ungetc(c, inf);
	tokval.str = tok;
	if (t != Txxx) {
		return t;
	}
	t = lexh[hash(tok)*K >> M];
	if (t == Txxx || strcmp(kwmap[t], tok) != 0) {
		err("unknown keyword %s", tok);
		return Txxx;
	}
	return t;
}
 */

impl Parser<'_> {
    fn getc(&mut self) -> RubeResult<Option<u8>> {
        match self.ungetc {
            None => match self.inf.next() {
                None => Ok(None),
                Some(r) => Ok(Some(r?)),
            },

            Some(c) => {
                self.ungetc = None;
                Ok(Some(c))
            }
        }
    }

    fn ungetc(&mut self, c: Option<u8>) {
        assert!(self.ungetc.is_none());
        self.ungetc = c;
    }

    fn lex(&mut self) -> RubeResult<Token> {
        let mut c: Option<u8> = Some(b' ');

        while c.is_some() && is_whitespace(c.unwrap()) {
            c = self.getc()?;
        }

        self.tokval.chr = c;

        if c.is_none() {
            return Ok(Token::Teof);
        }
        let mut craw = c.unwrap();

        let mut t: Token = Token::Txxx;
        let mut take_alpha = false;
        let mut take_quote = false;

        match craw {
            b',' => return Ok(Token::Tcomma),
            b'(' => return Ok(Token::Tlparen),
            b')' => return Ok(Token::Trparen),
            b'{' => return Ok(Token::Tlbrace),
            b'}' => return Ok(Token::Trbrace),
            b'=' => return Ok(Token::Teq),
            b'+' => return Ok(Token::Tplus),
            b's' => {
                let c2 = self.getc()?;
                if c2 == Some(b'_') {
                    self.tokval.flts = self.get_float()?;
                    return Ok(Token::Tflts);
                } else {
                    self.ungetc(c2);
                }
            }
            b'd' => {
                let c2 = self.getc()?;
                if c2 == Some(b'_') {
                    self.tokval.fltd = self.get_double()?;
                    return Ok(Token::Tfltd);
                } else {
                    self.ungetc(c2);
                }
            }
            b'%' => {
                t = Token::Ttmp;
                take_alpha = true;
            }
            b'@' => {
                t = Token::Tlbl;
                take_alpha = true;
            }
            b'$' => {
                t = Token::Tglo;
                let c2 = self.getc()?;
                if c2 == Some(b'"') {
                    take_quote = true;
                    c = c2;
                    craw = c.unwrap();
                } else {
                    self.ungetc(c2); // TODO - hopefully EOF is idempotent
                    take_alpha = true;
                }
            }
            b':' => {
                t = Token::Ttyp;
                take_alpha = true;
                // c = fgetc(inf);
                // goto Alpha;
            }
            b'#' => {
                while c.is_some() && c.unwrap() != b'\n' {
		    c = self.getc()?;
		}
		//while ((c=fgetc(inf)) != '\n' && c != EOF)
		    ;
                self.lnum += 1;
                return Ok(Token::Tnl);
            }
            b'\n' => {
                self.lnum += 1;
                return Ok(Token::Tnl);
            }
            b'"' => {
                t = Token::Tstr;
                take_quote = true;
            }
            _ => {
                // Expect a keyword
                take_alpha = true;
            }
        }

        assert!(!(take_alpha && take_quote));

        if take_alpha {
            if t != Token::Txxx {
                let prev_craw = craw;
                c = self.getc()?;
                if c.is_none() {
                    return Err(self.err(&format!(
                        "end of file after '{}' ({:#02x?})",
                        escape_default(prev_craw),
                        prev_craw
                    )));
                }
                craw = c.unwrap();
            }

            if !is_alpha(craw) && craw != b'.' && craw != b'_' {
                return Err(self.err(&format!(
                    "invalid character '{}' ({:#02x?})",
                    escape_default(craw),
                    craw
                )));
            }

            let mut tok: Vec<u8> = vec![];
            loop {
                tok.push(craw);
                c = self.getc()?;
                if c.is_none() {
                    break;
                }
                craw = c.unwrap();
                if !is_alpha(craw)
                    && craw != b'$'
                    && craw != b'.'
                    && craw != b'_'
                    && !is_digit(craw)
                {
                    break;
                }
            }
            self.ungetc(c); // Hope EOF is idempotent
            self.tokval.str = tok.clone();
            if t != Token::Txxx {
                return Ok(t);
            }
            t = LEXH[(hash(&tok) as usize) * K >> M];
            if t == Token::Txxx || KWMAP[t as usize] != tok {
                return Err(self.err(&format!("unknown keyword \"{:?}\"", tok)));
            }
        } else if take_quote {
            assert!(t != Token::Txxx);
            self.tokval.str = vec![];
            self.tokval.str.push(craw);
            let mut esc = false;
            loop {
                c = self.getc()?;
                if c.is_none() {
                    return Err(self.err("unterminated string"));
                }
                craw = c.unwrap();
                self.tokval.str.push(craw);
                if craw == b'"' && !esc {
                    return Ok(t);
                }
                esc = craw == b'\\' && !esc;
            }
        }

        if is_digit(craw) || craw == b'-' || craw == b'+' {
            self.ungetc(c);
            self.tokval.num = self.getint()?;
            return Ok(Token::Tint);
        }

        Err(self.err(&format!(
            "unexpected character '{}' ({:#02x?})",
            escape_default(craw),
            craw
        )))
    }
}

/*
static int
peek()
{
    if (thead == Txxx)
        thead = lex();
    return thead;
}
 */

impl Parser<'_> {
    fn peek(&mut self) -> RubeResult<Token> {
        if self.thead == Token::Txxx {
            self.thead = self.lex()?;
        }
        Ok(self.thead)
    }
}

/*
static int
next()
{
    int t;

    t = peek();
    thead = Txxx;
    return t;
}
 */

impl Parser<'_> {
    fn next(&mut self) -> RubeResult<Token> {
        let t = self.peek()?;
        self.thead = Token::Txxx;
        Ok(t)
    }
}

/*
static int
nextnl()
{
    int t;

    while ((t = next()) == Tnl)
        ;
    return t;
}
 */

impl Parser<'_> {
    fn nextnl(&mut self) -> RubeResult<Token> {
        loop {
            let t = self.next()?;

            if t != Token::Tnl {
                return Ok(t);
            }
        }
    }
}

/*
static void
expect(int t)
{
    static char *ttoa[] = {
        [Tlbl] = "label",
        [Tcomma] = ",",
        [Teq] = "=",
        [Tnl] = "newline",
        [Tlparen] = "(",
        [Trparen] = ")",
        [Tlbrace] = "{",
        [Trbrace] = "}",
        [Teof] = 0,
    };
    char buf[128], *s1, *s2;
    int t1;

    t1 = next();
    if (t == t1)
        return;
    s1 = ttoa[t] ? ttoa[t] : "??";
    s2 = ttoa[t1] ? ttoa[t1] : "??";
    sprintf(buf, "%s expected, got %s instead", s1, s2);
    err(buf);
}
 */

impl Parser<'_> {
    fn expect(&mut self, t: Token) -> RubeResult<()> {
        static TTOA: [&'static str; Token::Ntok as usize] = {
            let mut ttoa0: [&'static str; Token::Ntok as usize] =
                ["<unknown>"; Token::Ntok as usize];

            ttoa0[Token::Tlbl as usize] = "label";
            ttoa0[Token::Tcomma as usize] = ",";
            ttoa0[Token::Teq as usize] = "=";
            ttoa0[Token::Tnl as usize] = "newline";
            ttoa0[Token::Tlparen as usize] = "(";
            ttoa0[Token::Trparen as usize] = ")";
            ttoa0[Token::Tlbrace as usize] = "{";
            ttoa0[Token::Trbrace as usize] = "}";

            ttoa0
        };

        let t1 = self.next()?;
        if t == t1 {
            return Ok(());
        }

        Err(self.err(&format!(
            "{} expected, got {} instead",
            TTOA[t as usize], TTOA[t1 as usize]
        )))
    }
}
/*

static Ref
tmpref(char *v)
{
    int t, *h;

    h = &tmph[hash(v) & TMask];
    t = *h;
    if (t) {
        if (strcmp(curf->tmp[t].name, v) == 0)
            return TMP(t);
        for (t=curf->ntmp-1; t>=Tmp0; t--)
            if (strcmp(curf->tmp[t].name, v) == 0)
                return TMP(t);
    }
    t = curf->ntmp;
    *h = t;
    newtmp(0, Kx, curf);
    strcpy(curf->tmp[t].name, v);
    return TMP(t);
}

static Ref
parseref()
{
    Con c;

    memset(&c, 0, sizeof c);
    switch (next()) {
    default:
        return R;
    case Ttmp:
        return tmpref(tokval.str);
    case Tint:
        c.type = CBits;
        c.bits.i = tokval.num;
        break;
    case Tflts:
        c.type = CBits;
        c.bits.s = tokval.flts;
        c.flt = 1;
        break;
    case Tfltd:
        c.type = CBits;
        c.bits.d = tokval.fltd;
        c.flt = 2;
        break;
    case Tthread:
        c.sym.type = SThr;
        expect(Tglo);
/* fall through */
    case Tglo:
        c.type = CAddr;
        c.sym.id = intern(tokval.str);
        break;
    }
    return newcon(&c, curf);
}

static int
findtyp(int i)
{
    while (--i >= 0)
        if (strcmp(tokval.str, typ[i].name) == 0)
            return i;
    err("undefined type :%s", tokval.str);
}
 */

impl Parser<'_> {
    //static int
    fn findtyp(&self /*, int i*/) -> RubeResult<TypIdx> {
        for i in (0..self.typ.len()).rev() {
            //while (--i >= 0)
            // if (strcmp(tokval.str, typ[i].name) == 0)
            // 	return i;
            if self.tokval.str == self.typ[i].name {
                return Ok(TypIdx(i));
            }
        }
        Err(self.err(&format!(
            "undefined type :{}",
            String::from_utf8_lossy(&self.tokval.str)
        )))
    }
}

/*
static int
parsecls(int *tyn)
{
    switch (next()) {
    default:
        err("invalid class specifier");
    case Ttyp:
        *tyn = findtyp(ntyp);
        return Kc;
    case Tsb:
        return Ksb;
    case Tub:
        return Kub;
    case Tsh:
        return Ksh;
    case Tuh:
        return Kuh;
    case Tw:
        return Kw;
    case Tl:
        return Kl;
    case Ts:
        return Ks;
    case Td:
        return Kd;
    }
}
 */

impl Parser<'_> {
    fn parsecls(&mut self) -> RubeResult<(KExt, TypIdx)> {
        match self.next()? {
            Token::Ttyp => Ok((KC, self.findtyp()?)),
            Token::Tsb => Ok((KSB, TypIdx::INVALID)),
            Token::Tub => Ok((KUB, TypIdx::INVALID)),
            Token::Tsh => Ok((KSH, TypIdx::INVALID)),
            Token::Tuh => Ok((KUH, TypIdx::INVALID)),
            Token::Tw => Ok((KW, TypIdx::INVALID)),
            Token::Tl => Ok((KL, TypIdx::INVALID)),
            Token::Ts => Ok((KS, TypIdx::INVALID)),
            Token::Td => Ok((KD, TypIdx::INVALID)),
            _ => Err(self.err("invalid class specifier")),
        }
    }
}

/*
static int
parserefl(int arg)
{
    int k, ty, env, hasenv, vararg;
    Ref r;

    hasenv = 0;
    vararg = 0;
    expect(Tlparen);
    while (peek() != Trparen) {
        if (curi - insb >= NIns)
            err("too many instructions");
        if (!arg && vararg)
            err("no parameters allowed after '...'");
        switch (peek()) {
        case Tdots:
            if (vararg)
                err("only one '...' allowed");
            vararg = 1;
            if (arg) {
                *curi = (Ins){.op = Oargv};
                curi++;
            }
            next();
            goto Next;
        case Tenv:
            if (hasenv)
                err("only one environment allowed");
            hasenv = 1;
            env = 1;
            next();
            k = Kl;
            break;
        default:
            env = 0;
            k = parsecls(&ty);
            break;
        }
        r = parseref();
        if (req(r, R))
            err("invalid argument");
        if (!arg && rtype(r) != RTmp)
            err("invalid function parameter");
        if (env)
            if (arg)
                *curi = (Ins){Oarge, k, R, {r}};
            else
                *curi = (Ins){Opare, k, r, {R}};
        else if (k == Kc)
            if (arg)
                *curi = (Ins){Oargc, Kl, R, {TYPE(ty), r}};
            else
                *curi = (Ins){Oparc, Kl, r, {TYPE(ty)}};
        else if (k >= Ksb)
            if (arg)
                *curi = (Ins){Oargsb+(k-Ksb), Kw, R, {r}};
            else
                *curi = (Ins){Oparsb+(k-Ksb), Kw, r, {R}};
        else
            if (arg)
                *curi = (Ins){Oarg, k, R, {r}};
            else
                *curi = (Ins){Opar, k, r, {R}};
        curi++;
    Next:
        if (peek() == Trparen)
            break;
        expect(Tcomma);
    }
    expect(Trparen);
    return vararg;
}

static Blk *
findblk(char *name)
{
    Blk *b;
    uint32_t h;

    h = hash(name) & BMask;
    for (b=blkh[h]; b; b=b->dlink)
        if (strcmp(b->name, name) == 0)
            return b;
    b = newblk();
    b->id = nblk++;
    strcpy(b->name, name);
    b->dlink = blkh[h];
    blkh[h] = b;
    return b;
}

static void
closeblk()
{
    curb->nins = curi - insb;
    idup(&curb->ins, insb, curb->nins);
    blink = &curb->link;
    curi = insb;
}

static PState
parseline(PState ps)
{
    Ref arg[NPred] = {R};
    Blk *blk[NPred];
    Phi *phi;
    Ref r;
    Blk *b;
    Con *c;
    int t, op, i, k, ty;

    t = nextnl();
    if (ps == PLbl && t != Tlbl && t != Trbrace)
        err("label or } expected");
    switch (t) {
    case Ttmp:
        r = tmpref(tokval.str);
        expect(Teq);
        k = parsecls(&ty);
        op = next();
        break;
    default:
        if (isstore(t)) {
        case Tblit:
        case Tcall:
        case Ovastart:
/* operations without result */
            r = R;
            k = Kw;
            op = t;
            break;
        }
        err("label, instruction or jump expected");
    case Trbrace:
        return PEnd;
    case Tlbl:
        b = findblk(tokval.str);
        if (curb && curb->jmp.type == Jxxx) {
            closeblk();
            curb->jmp.type = Jjmp;
            curb->s1 = b;
        }
        if (b->jmp.type != Jxxx)
            err("multiple definitions of block @%s", b->name);
        *blink = b;
        curb = b;
        plink = &curb->phi;
        expect(Tnl);
        return PPhi;
    case Tret:
        curb->jmp.type = Jretw + rcls;
        if (peek() == Tnl)
            curb->jmp.type = Jret0;
        else if (rcls != K0) {
            r = parseref();
            if (req(r, R))
                err("invalid return value");
            curb->jmp.arg = r;
        }
        goto Close;
    case Tjmp:
        curb->jmp.type = Jjmp;
        goto Jump;
    case Tjnz:
        curb->jmp.type = Jjnz;
        r = parseref();
        if (req(r, R))
            err("invalid argument for jnz jump");
        curb->jmp.arg = r;
        expect(Tcomma);
    Jump:
        expect(Tlbl);
        curb->s1 = findblk(tokval.str);
        if (curb->jmp.type != Jjmp) {
            expect(Tcomma);
            expect(Tlbl);
            curb->s2 = findblk(tokval.str);
        }
        if (curb->s1 == curf->start || curb->s2 == curf->start)
            err("invalid jump to the start block");
        goto Close;
    case Thlt:
        curb->jmp.type = Jhlt;
    Close:
        expect(Tnl);
        closeblk();
        return PLbl;
    case Odbgloc:
        op = t;
        k = Kw;
        r = R;
        expect(Tint);
        arg[0] = INT(tokval.num);
        if (arg[0].val != tokval.num)
            err("line number too big");
        if (peek() == Tcomma) {
            next();
            expect(Tint);
            arg[1] = INT(tokval.num);
            if (arg[1].val != tokval.num)
                err("column number too big");
        } else
            arg[1] = INT(0);
        goto Ins;
    }
    if (op == Tcall) {
        arg[0] = parseref();
        parserefl(1);
        op = Ocall;
        expect(Tnl);
        if (k == Kc) {
            k = Kl;
            arg[1] = TYPE(ty);
        }
        if (k >= Ksb)
            k = Kw;
        goto Ins;
    }
    if (op == Tloadw)
        op = Oloadsw;
    if (op >= Tloadl && op <= Tloadd)
        op = Oload;
    if (op == Talloc1 || op == Talloc2)
        op = Oalloc;
    if (op == Ovastart && !curf->vararg)
        err("cannot use vastart in non-variadic function");
    if (k >= Ksb)
        err("size class must be w, l, s, or d");
    i = 0;
    if (peek() != Tnl)
        for (;;) {
            if (i == NPred)
                err("too many arguments");
            if (op == Tphi) {
                expect(Tlbl);
                blk[i] = findblk(tokval.str);
            }
            arg[i] = parseref();
            if (req(arg[i], R))
                err("invalid instruction argument");
            i++;
            t = peek();
            if (t == Tnl)
                break;
            if (t != Tcomma)
                err(", or end of line expected");
            next();
        }
    next();
    switch (op) {
    case Tphi:
        if (ps != PPhi || curb == curf->start)
            err("unexpected phi instruction");
        phi = alloc(sizeof *phi);
        phi->to = r;
        phi->cls = k;
        phi->arg = vnew(i, sizeof arg[0], PFn);
        memcpy(phi->arg, arg, i * sizeof arg[0]);
        phi->blk = vnew(i, sizeof blk[0], PFn);
        memcpy(phi->blk, blk, i * sizeof blk[0]);
        phi->narg = i;
        *plink = phi;
        plink = &phi->link;
        return PPhi;
    case Tblit:
        if (curi - insb >= NIns-1)
            err("too many instructions");
        memset(curi, 0, 2 * sizeof(Ins));
        curi->op = Oblit0;
        curi->arg[0] = arg[0];
        curi->arg[1] = arg[1];
        curi++;
        if (rtype(arg[2]) != RCon)
            err("blit size must be constant");
        c = &curf->con[arg[2].val];
        r = INT(c->bits.i);
        if (c->type != CBits
        || rsval(r) < 0
        || rsval(r) != c->bits.i)
            err("invalid blit size");
        curi->op = Oblit1;
        curi->arg[0] = r;
        curi++;
        return PIns;
    default:
        if (op >= NPubOp)
            err("invalid instruction");
    Ins:
        if (curi - insb >= NIns)
            err("too many instructions");
        curi->op = op;
        curi->cls = k;
        curi->to = r;
        curi->arg[0] = arg[0];
        curi->arg[1] = arg[1];
        curi++;
        return PIns;
    }
}

static int
usecheck(Ref r, int k, Fn *fn)
{
    return rtype(r) != RTmp || fn->tmp[r.val].cls == k
        || (fn->tmp[r.val].cls == Kl && k == Kw);
}

static void
typecheck(Fn *fn)
{
    Blk *b;
    Phi *p;
    Ins *i;
    uint n;
    int k;
    Tmp *t;
    Ref r;
    BSet pb[1], ppb[1];

    fillpreds(fn);
    bsinit(pb, fn->nblk);
    bsinit(ppb, fn->nblk);
    for (b=fn->start; b; b=b->link) {
        for (p=b->phi; p; p=p->link)
            fn->tmp[p->to.val].cls = p->cls;
        for (i=b->ins; i<&b->ins[b->nins]; i++)
            if (rtype(i->to) == RTmp) {
                t = &fn->tmp[i->to.val];
                if (clsmerge(&t->cls, i->cls))
                    err("temporary %%%s is assigned with"
                        " multiple types", t->name);
            }
    }
    for (b=fn->start; b; b=b->link) {
        bszero(pb);
        for (n=0; n<b->npred; n++)
            bsset(pb, b->pred[n]->id);
        for (p=b->phi; p; p=p->link) {
            bszero(ppb);
            t = &fn->tmp[p->to.val];
            for (n=0; n<p->narg; n++) {
                k = t->cls;
                if (bshas(ppb, p->blk[n]->id))
                    err("multiple entries for @%s in phi %%%s",
                        p->blk[n]->name, t->name);
                if (!usecheck(p->arg[n], k, fn))
                    err("invalid type for operand %%%s in phi %%%s",
                        fn->tmp[p->arg[n].val].name, t->name);
                bsset(ppb, p->blk[n]->id);
            }
            if (!bsequal(pb, ppb))
                err("predecessors not matched in phi %%%s", t->name);
        }
        for (i=b->ins; i<&b->ins[b->nins]; i++)
            for (n=0; n<2; n++) {
                k = optab[i->op].argcls[n][i->cls];
                r = i->arg[n];
                t = &fn->tmp[r.val];
                if (k == Ke)
                    err("invalid instruction type in %s",
                        optab[i->op].name);
                if (rtype(r) == RType)
                    continue;
                if (rtype(r) != -1 && k == Kx)
                    err("no %s operand expected in %s",
                        n == 1 ? "second" : "first",
                        optab[i->op].name);
                if (rtype(r) == -1 && k != Kx)
                    err("missing %s operand in %s",
                        n == 1 ? "second" : "first",
                        optab[i->op].name);
                if (!usecheck(r, k, fn))
                    err("invalid type for %s operand %%%s in %s",
                        n == 1 ? "second" : "first",
                        t->name, optab[i->op].name);
            }
        r = b->jmp.arg;
        if (isret(b->jmp.type)) {
            if (b->jmp.type == Jretc)
                k = Kl;
            else if (b->jmp.type >= Jretsb)
                k = Kw;
            else
                k = b->jmp.type - Jretw;
            if (!usecheck(r, k, fn))
                goto JErr;
        }
        if (b->jmp.type == Jjnz && !usecheck(r, Kw, fn))
        JErr:
            err("invalid type for jump argument %%%s in block @%s",
                fn->tmp[r.val].name, b->name);
        if (b->s1 && b->s1->jmp.type == Jxxx)
            err("block @%s is used undefined", b->s1->name);
        if (b->s2 && b->s2->jmp.type == Jxxx)
            err("block @%s is used undefined", b->s2->name);
    }
}

static Fn *
parsefn(Lnk *lnk)
{
    Blk *b;
    int i;
    PState ps;

    curb = 0;
    nblk = 0;
    curi = insb;
    curf = alloc(sizeof *curf);
    curf->ntmp = 0;
    curf->ncon = 2;
    curf->tmp = vnew(curf->ntmp, sizeof curf->tmp[0], PFn);
    curf->con = vnew(curf->ncon, sizeof curf->con[0], PFn);
    for (i=0; i<Tmp0; ++i)
        if (T.fpr0 <= i && i < T.fpr0 + T.nfpr)
            newtmp(0, Kd, curf);
        else
            newtmp(0, Kl, curf);
    curf->con[0].type = CBits;
curf->con[0].bits.i = 0xdeaddead; /* UNDEF */
    curf->con[1].type = CBits;
    curf->lnk = *lnk;
    blink = &curf->start;
    curf->retty = Kx;
    if (peek() != Tglo)
        rcls = parsecls(&curf->retty);
    else
        rcls = K0;
    if (next() != Tglo)
        err("function name expected");
    strncpy(curf->name, tokval.str, NString-1);
    curf->vararg = parserefl(0);
    if (nextnl() != Tlbrace)
        err("function body must start with {");
    ps = PLbl;
    do
        ps = parseline(ps);
    while (ps != PEnd);
    if (!curb)
        err("empty function");
    if (curb->jmp.type == Jxxx)
        err("last block misses jump");
    curf->mem = vnew(0, sizeof curf->mem[0], PFn);
    curf->nmem = 0;
    curf->nblk = nblk;
    curf->rpo = 0;
    for (b=0; b; b=b->link)
b->dlink = 0; /* was trashed by findblk() */
    for (i=0; i<BMask+1; ++i)
        blkh[i] = 0;
    memset(tmph, 0, sizeof tmph);
    typecheck(curf);
    return curf;
}
 */
impl Parser<'_> {
    fn parsefn(&mut self, lnk: &Lnk) -> RubeResult<Fn> {
        // Blk *b;
        // int i;
        // PState ps;

        // self.curb = None;
        // nblk = 0;
        // curi = insb;
        // curf = alloc(sizeof *curf);
        let mut curf = Fn::new(lnk.clone());
        //let curf: &mut Fn = &mut self.curf.unwrap();
        // curf->ntmp = 0;
        // curf->ncon = 2;
        // curf->tmp = vnew(curf->ntmp, sizeof curf->tmp[0], PFn);
        // curf->con = vnew(curf->ncon, sizeof curf->con[0], PFn);
        for i in 0..(Tmp0 as i32) {
            if self.T.fpr0 <= i && i < self.T.fpr0 + self.T.nfpr {
                let _ = newtmp(None, KD, &mut curf);
            } else {
                let _ = newtmp(None, KL, &mut curf);
            }
        }

        // curf->con[0].type = CBits;
        // curf->con[0].bits.i = 0xdeaddead; /* UNDEF */
        curf.con.push(Con::new(
            ConT::CBits,
            Sym::new(SymT::SGlo, 0),
            ConBits::I(0xdeaddead),
        )); /* UNDEF */
        // curf->con[1].type = CBits;
        curf.con.push(Con::new(
            ConT::CBits,
            Sym::new(SymT::SGlo, 0),
            ConBits::I(0),
        )); // ??? what's this for?
            // curf->lnk = *lnk;
            // blink = &curf->start; // TODO - ???
            // curf.retty = Some(KX); // Handled in Fn::new() as TypIdx::INVALID
        if self.peek()? != Token::Tglo {
            (self.rcls, curf.retty) = self.parsecls()?;
        } /*else {
              self.rcls = K0; // Default in Fn::new()
          }*/

        if self.next()? != Token::Tglo {
            return Err(self.err("function name expected"));
        }
        // strncpy(curf->name, tokval.str, NString-1);
        curf.name = self.tokval.str.clone();
        curf.vararg = self.parserefl(0)?;
        if self.nextnl()? != Token::Tlbrace {
            return Err(self.err("function body must start with {"));
        }
        let mut ps: PState = PState::PLbl;
        loop {
            ps = self.parseline(ps)?;
            if ps == PState::PEnd {
                break;
            }
        }
        match self.curb {
            None => {
                return Err(self.err("empty function"));
            }
            Some(blki) => {
                let b: &Blk = &curf.blks[blki.0]; // TODO accessor fn rather?
                if b.jmp.type_ == J::Jxxx {
                    return Err(self.err("last block misses jump"));
                }
            }
        }
        // curf->mem = vnew(0, sizeof curf->mem[0], PFn);
        // curf->nmem = 0;
        // curf->nblk = nblk;
        // curf->rpo = 0;

        // WTF is this for loop doing? It starts at 0????? TODO TODO TODO - Notify QBE?
        // for (b=0; b; b=b->link)
        //     b->dlink = 0; /* was trashed by findblk() */
        for i in 0..BMASK + 1 {
            self.blkh[i] = BlkIdx::INVALID;
        }
        // memset(tmph, 0, sizeof tmph);
        for i in 0..TMASK + 1 {
            self.tmph[i] = 0;
        }
        self.typecheck(&curf)?;
        //return curf;

        Ok(curf)
    }
}
/*
static void
parsefields(Field *fld, Typ *ty, int t)
{
    Typ *ty1;
    int n, c, a, al, type;
    uint64_t sz, s;

    n = 0;
    sz = 0;
    al = ty->align;
    while (t != Trbrace) {
        ty1 = 0;
        switch (t) {
        default: err("invalid type member specifier");
        case Td: type = Fd; s = 8; a = 3; break;
        case Tl: type = Fl; s = 8; a = 3; break;
        case Ts: type = Fs; s = 4; a = 2; break;
        case Tw: type = Fw; s = 4; a = 2; break;
        case Th: type = Fh; s = 2; a = 1; break;
        case Tb: type = Fb; s = 1; a = 0; break;
        case Ttyp:
            type = FTyp;
            ty1 = &typ[findtyp(ntyp-1)];
            s = ty1->size;
            a = ty1->align;
            break;
        }
        if (a > al)
            al = a;
        a = (1 << a) - 1;
        a = ((sz + a) & ~a) - sz;
        if (a) {
            if (n < NField) {
/* padding */
                fld[n].type = FPad;
                fld[n].len = a;
                n++;
            }
        }
        t = nextnl();
        if (t == Tint) {
            c = tokval.num;
            t = nextnl();
        } else
            c = 1;
        sz += a + c*s;
        if (type == FTyp)
            s = ty1 - typ;
        for (; c>0 && n<NField; c--, n++) {
            fld[n].type = type;
            fld[n].len = s;
        }
        if (t != Tcomma)
            break;
        t = nextnl();
    }
    if (t != Trbrace)
        err(", or } expected");
    fld[n].type = FEnd;
    a = 1 << al;
    if (sz < ty->size)
        sz = ty->size;
    ty->size = (sz + a - 1) & -a;
    ty->align = al;
}
 */

impl Parser<'_> {
    // TODO - this should just return a Vec<TypField>
    fn parsefields(&mut self, /*Field *fld,*/ ty: &mut Typ, tparam: Token) -> RubeResult<()> {
        let mut t: Token = tparam;
        // Typ *ty1;
        // int n, c, a, al, type;
        // uint64_t sz, s;

        //let mut n: i32 = 0;
        let mut sz: u64 = 0;
        let mut al = ty.align;
        while t != Token::Trbrace {
            let type_: TypFldT;
            let mut s: u64;
            let mut a: i32;
            let mut ftyp_idx: usize = 0;
            match t {
                Token::Td => {
                    type_ = TypFldT::Fd;
                    s = 8;
                    a = 3;
                }
                Token::Tl => {
                    type_ = TypFldT::Fl;
                    s = 8;
                    a = 3;
                }
                Token::Ts => {
                    type_ = TypFldT::Fs;
                    s = 4;
                    a = 2;
                }
                Token::Tw => {
                    type_ = TypFldT::Fw;
                    s = 4;
                    a = 2;
                }
                Token::Th => {
                    type_ = TypFldT::Fh;
                    s = 2;
                    a = 1;
                }
                Token::Tb => {
                    type_ = TypFldT::Fb;
                    s = 1;
                    a = 0;
                }
                Token::Ttyp => {
                    //let mut ty1: &Typ;
                    type_ = TypFldT::FTyp;
                    let TypIdx(idx) = self.findtyp()?;
                    ftyp_idx = idx;
                    //ty1 = &typ[findtyp(ntyp - 1)];
                    s = self.typ[idx].size;
                    a = self.typ[idx].align;
                }
                _ => return Err(self.err("invalid type member specifier")),
            }
            if a > al {
                al = a;
            }
            a = (1 << a) - 1;
            a = (((sz as i32) + a) & !a) - (sz as i32); // TODO - this is fugly
            if a != 0 {
                // TODO WTF?
                if true
                /*n < NField*/
                {
                    // TODO we don't need this check? Seems broken in QBE - just dropping fields
                    /* padding */
                    // fld[n].type = FPad;
                    // fld[n].len = a;
                    ty.fields.push(TypFld::new(TypFldT::FPad, a as u32));
                    //n += 1;
                }
            }
            t = self.nextnl()?;
            let mut c: i32 = 1;
            if t == Token::Tint {
                c = self.tokval.num as i32;
                t = self.nextnl()?;
            }
            sz += (a as u64) + (c as u64) * s;
            if type_ == TypFldT::FTyp {
                //s = ty1 - typ; // TODO WTF? ah, it's the index!
                s = ftyp_idx as u64;
            }
            //for (; c>0 && n<NField; c--, n++) {
            while c > 0
            /*&& n < NField*/
            {
                // fld[n].type_ = type_; // TODO WTF?
                // fld[n].len = s;
                ty.fields.push(TypFld::new(type_, s as u32)); // ugh

                c -= 1;
                //n += 1;
            }
            if t != Token::Tcomma {
                break;
            }
            t = self.nextnl()?;
        }
        if t != Token::Trbrace {
            return Err(self.err(", or } expected"));
        }
        // TODO sentinal value marking end of fields - we don't need this in rust
        //fld[n].type_ = FEnd;
        let a: i32 = 1 << al;
        if sz < ty.size {
            sz = ty.size;
        }
        ty.size = (sz + (a as i64 as u64) - 1) & (-a as i64 as u64); // TODO ugh
        ty.align = al;

        Ok(())
    }
}

/*
static void
parsetyp()
{
    Typ *ty;
    int t, al;
    uint n;

/* be careful if extending the syntax
 * to handle nested types, any pointer
 * held to typ[] might be invalidated!
 */
    vgrow(&typ, ntyp+1);
    ty = &typ[ntyp++];
    ty->isdark = 0;
    ty->isunion = 0;
    ty->align = -1;
    ty->size = 0;
    if (nextnl() != Ttyp ||  nextnl() != Teq)
        err("type name and then = expected");
    strcpy(ty->name, tokval.str);
    t = nextnl();
    if (t == Talign) {
        if (nextnl() != Tint)
            err("alignment expected");
        for (al=0; tokval.num /= 2; al++)
            ;
        ty->align = al;
        t = nextnl();
    }
    if (t != Tlbrace)
        err("type body must start with {");
    t = nextnl();
    if (t == Tint) {
        ty->isdark = 1;
        ty->size = tokval.num;
        if (ty->align == -1)
            err("dark types need alignment");
        if (nextnl() != Trbrace)
            err("} expected");
        return;
    }
    n = 0;
    ty->fields = vnew(1, sizeof ty->fields[0], PHeap);
    if (t == Tlbrace) {
        ty->isunion = 1;
        do {
            if (t != Tlbrace)
                err("invalid union member");
            vgrow(&ty->fields, n+1);
            parsefields(ty->fields[n++], ty, nextnl());
            t = nextnl();
        } while (t != Trbrace);
    } else
        parsefields(ty->fields[n++], ty, t);
    ty->nunion = n;
}
 */

impl Parser<'_> {
    fn parsetyp(&mut self) -> RubeResult<()> {
        // Typ *ty;
        // int t, al;
        // uint n;

        /* be careful if extending the syntax
         * to handle nested types, any pointer
         * held to typ[] might be invalidated!
         */
        // vgrow(&typ, ntyp+1);
        // ty = &typ[ntyp++];
        // ty->isdark = 0;
        // ty->isunion = 0;
        // ty->align = -1;
        // ty->size = 0;

        let mut ty = Typ::new();

        if self.nextnl()? != Token::Ttyp || self.nextnl()? != Token::Teq {
            return Err(self.err("type name and then = expected"));
        }

        ty.name = self.tokval.str.clone();
        let mut t = self.nextnl()?;
        if t == Token::Talign {
            if self.nextnl()? != Token::Tint {
                return Err(self.err("alignment expected"));
            }
            let mut al: i32 = 0;
            // for (al=0; tokval.num /= 2; al++)
            // 	;
            loop {
                self.tokval.num /= 2;
                if self.tokval.num == 0 {
                    break;
                }
                al += 1;
            }
            ty.align = al;
            t = self.nextnl()?;
        }
        if t != Token::Tlbrace {
            return Err(self.err("type body must start with {"));
        }
        t = self.nextnl()?;
        if t == Token::Tint {
            ty.isdark = true;
            ty.size = self.tokval.num as u64; // TODO: QBE notify? Mmm check negative value?
            if ty.align == -1 {
                return Err(self.err("dark types need alignment"));
            }
            if self.nextnl()? != Token::Trbrace {
                return Err(self.err("} expected"));
            }
            self.typ.push(ty);
            return Ok(());
        }
        let mut n: u32 = 0;
        //ty->fields = vnew(1, sizeof ty->fields[0], PHeap);
        if t == Token::Tlbrace {
            ty.isunion = true;
            //do {
            loop {
                if t != Token::Tlbrace {
                    return Err(self.err("invalid union member"));
                }
                //vgrow(&ty->fields, n+1);
                t = self.nextnl()?;
                self.parsefields(/*ty->fields[n++],*/ &mut ty, t)?;
                n += 1;
                t = self.nextnl()?;
                if t == Token::Trbrace {
                    break;
                }
            } //while (t != Trbrace);
        } else {
            self.parsefields(/*ty->fields[n++],*/ &mut ty, t)?;
            n += 1;
        }
        ty.nunion = n;
        self.typ.push(ty);
        Ok(())
    }
}

/*
static void
parsedatref(Dat *d)
{
    int t;

    d->isref = 1;
    d->u.ref.name = tokval.str;
    d->u.ref.off = 0;
    t = peek();
    if (t == Tplus) {
        next();
        if (next() != Tint)
            err("invalid token after offset in ref");
        d->u.ref.off = tokval.num;
    }
}
 */

impl Parser<'_> {
    fn parsedatref(&mut self, d: &mut Dat) -> RubeResult<()> {
        //int t;

        d.isref = true;
        let name: Vec<u8> = self.tokval.str.clone();
        // d->u.ref.name = tokval.str;
        // d->u.ref.off = 0;
        let mut off: i64 = 0;
        // t = peek();
        if self.peek()? == Token::Tplus {
            let _ = self.next()?;
            if self.next()? != Token::Tint {
                return Err(self.err("invalid token after offset in ref"));
            }
            off = self.tokval.num;
        }
        d.u = DatU::Ref { name, off };

        Ok(())
    }
}

/*
static void
parsedatstr(Dat *d)
{
    d->isstr = 1;
    d->u.str = tokval.str;
}
 */

impl Parser<'_> {
    fn parsedatstr(&self, d: &mut Dat) {
        d.isstr = true;
        d.u = DatU::Str(self.tokval.str.clone());
    }
}

/*
static void
parsedat(void cb(Dat *), Lnk *lnk)
{
    char name[NString] = {0};
    int t;
    Dat d;

    if (nextnl() != Tglo || nextnl() != Teq)
        err("data name, then = expected");
    strncpy(name, tokval.str, NString-1);
    t = nextnl();
    lnk->align = 8;
    if (t == Talign) {
        if (nextnl() != Tint)
            err("alignment expected");
        lnk->align = tokval.num;
        t = nextnl();
    }
    d.type = DStart;
    d.name = name;
    d.lnk = lnk;
    cb(&d);

    if (t != Tlbrace)
        err("expected data contents in { .. }");
    for (;;) {
        switch (nextnl()) {
        default: err("invalid size specifier %c in data", tokval.chr);
        case Trbrace: goto Done;
        case Tl: d.type = DL; break;
        case Tw: d.type = DW; break;
        case Th: d.type = DH; break;
        case Tb: d.type = DB; break;
        case Ts: d.type = DW; break;
        case Td: d.type = DL; break;
        case Tz: d.type = DZ; break;
        }
        t = nextnl();
        do {
            d.isstr = 0;
            d.isref = 0;
            memset(&d.u, 0, sizeof d.u);
            if (t == Tflts)
                d.u.flts = tokval.flts;
            else if (t == Tfltd)
                d.u.fltd = tokval.fltd;
            else if (t == Tint)
                d.u.num = tokval.num;
            else if (t == Tglo)
                parsedatref(&d);
            else if (t == Tstr)
                parsedatstr(&d);
            else
                err("constant literal expected");
            cb(&d);
            t = nextnl();
        } while (t == Tint || t == Tflts || t == Tfltd || t == Tstr || t == Tglo);
        if (t == Trbrace)
            break;
        if (t != Tcomma)
            err(", or } expected");
    }
Done:
    d.type = DEnd;
    cb(&d);
}
 */

impl Parser<'_> {
    fn parsedat(&mut self, cb: fn(&Dat) -> (), lnk: &mut Lnk) -> RubeResult<()> {
        // char name[NString] = {0};
        // int t;
        // Dat d;

        if self.nextnl()? != Token::Tglo || self.nextnl()? != Token::Teq {
            return Err(self.err("data name, then = expected"));
        }

        //strncpy(name, tokval.str, NString-1);
        let name: Vec<u8> = self.tokval.str.clone();
        let mut t: Token = self.nextnl()?;
        lnk.align = 8;
        if t == Token::Talign {
            if self.nextnl()? != Token::Tint {
                return Err(self.err("alignment expected"));
            }
            lnk.align = self.tokval.num as i8;
            t = self.nextnl()?;
        }

        // d.type = DStart;
        // d.name = name;
        // d.lnk = lnk;
        let mut d: Dat = Dat::new(DatT::DStart, &name, lnk.clone());
        cb(&d);

        if t != Token::Tlbrace {
            return Err(self.err("expected data contents in { .. }"));
        }
        //for (;;) {
        loop {
            match self.nextnl()? {
                Token::Trbrace => break,
                Token::Tl => d.type_ = DatT::DL,
                Token::Tw => d.type_ = DatT::DW,
                Token::Th => d.type_ = DatT::DH,
                Token::Tb => d.type_ = DatT::DB,
                Token::Ts => d.type_ = DatT::DW,
                Token::Td => d.type_ = DatT::DL,
                Token::Tz => d.type_ = DatT::DZ,
                _ => {
                    return Err(self.err(&format!(
                        "invalid size specifier '{}' ({:#02x?}) in data",
                        escape_default(self.tokval.chr.unwrap_or(b'?')),
                        self.tokval.chr
                    )));
                }
            }
            t = self.nextnl()?;
            loop {
                d.isstr = false;
                d.isref = false;
                d.u = DatU::None;

                match t {
                    Token::Tflts => d.u = DatU::Flts(self.tokval.flts),
                    Token::Tfltd => d.u = DatU::Fltd(self.tokval.fltd),
                    Token::Tint => d.u = DatU::Num(self.tokval.num),
                    Token::Tglo => self.parsedatref(&mut d)?,
                    Token::Tstr => self.parsedatstr(&mut d),
                    _ => {
                        return Err(self.err("constant literal or global ref expected in data"));
                    }
                }
                cb(&d);
                t = self.nextnl()?;
                if !(t == Token::Tint
                    || t == Token::Tflts
                    || t == Token::Tfltd
                    || t == Token::Tstr
                    || t == Token::Tglo)
                {
                    break;
                }
            } //while (t == Tint || t == Tflts || t == Tfltd || t == Tstr || t == Tglo);
            if t == Token::Trbrace {
                break;
            }
            if t != Token::Tcomma {
                return Err(self.err(", or } expected in data"));
            }
        }
        //Done:
        d.type_ = DatT::DEnd;
        cb(&d);

        Ok(())
    }
}

/*
static int
parselnk(Lnk *lnk)
{
    int t, haslnk;

    for (haslnk=0;; haslnk=1)
        switch ((t=nextnl())) {
        case Texport:
            lnk->export = 1;
            break;
        case Tthread:
            lnk->thread = 1;
            break;
        case Tsection:
            if (lnk->sec)
                err("only one section allowed");
            if (next() != Tstr)
                err("section \"name\" expected");
            lnk->sec = tokval.str;
            if (peek() == Tstr) {
                next();
                lnk->secf = tokval.str;
            }
            break;
        default:
            if (t == Tfunc && lnk->thread)
                err("only data may have thread linkage");
            if (haslnk && t != Tdata && t != Tfunc)
                err("only data and function have linkage");
            return t;
        }
}
 */

impl Parser<'_> {
    fn parselnk(&mut self, lnk: &mut Lnk) -> RubeResult<Token> {
        let mut haslnk: bool = false;

        loop {
            let t = self.nextnl()?;

            match t {
                Token::Texport => {
                    lnk.export = true;
                }

                Token::Tthread => {
                    lnk.thread = true;
                }

                Token::Tsection => {
                    if lnk.sec.is_empty() {
                        return Err(self.err("only one section allowed"));
                    }
                    if self.next()? != Token::Tstr {
                        return Err(self.err("section \"name\" expected"));
                    }
                    lnk.sec = self.tokval.str.clone();
                    if self.peek()? == Token::Tstr {
                        self.next()?;
                        lnk.secf = self.tokval.str.clone();
                    }
                }

                _ => {
                    if t == Token::Tfunc && lnk.thread {
                        return Err(self.err("only data may have thread linkage"));
                    }
                    if haslnk && t != Token::Tdata && t != Token::Tfunc {
                        return Err(self.err("only data and function have linkage"));
                    }
                    return Ok(t);
                }
            }

            haslnk = true;
        }
    }
}
/*
void
parse(FILE *f, char *path, void dbgfile(char *), void data(Dat *), void func(Fn *))
{
    Lnk lnk;
    uint n;

    lexinit();
    inf = f;
    inpath = path;
    lnum = 1;
    thead = Txxx;
    ntyp = 0;
    typ = vnew(0, sizeof typ[0], PHeap);
    for (;;) {
        lnk = (Lnk){0};
        switch (parselnk(&lnk)) {
        default:
            err("top-level definition expected");
        case Tdbgfile:
            expect(Tstr);
            dbgfile(tokval.str);
            break;
        case Tfunc:
            func(parsefn(&lnk));
            break;
        case Tdata:
            parsedat(data, &lnk);
            break;
        case Ttype:
            parsetyp();
            break;
        case Teof:
            for (n=0; n<ntyp; n++)
                if (typ[n].nunion)
                    vfree(typ[n].fields);
            vfree(typ);
            return;
        }
    }
}
 */

impl Parser<'_> {
    fn new<'a>(
        T: &'a Target,
        f: &'a File,
        path: &'a Path,
        dbgfile: fn(&[u8]) -> (),
        data: fn(&Dat) -> (),
        func: fn(&Fn) -> (),
    ) -> Parser<'a> {
        Parser {
            T,
            inf: BufReader::new(f).bytes(),
            ungetc: None,
            inpath: path,
            thead: Token::Txxx,
            tokval: TokVal::new(),
            lnum: 0,
            //curf: None,
            tmph: [0; TMASK + 1],
            curb: None,
            blkh: [BlkIdx::INVALID; BMASK + 1],
            //nblk: 0,
            rcls: K0,
            //ntyp: 0,
            typ: vec![],
        }
    }
}

pub fn parse(
    T: &Target,
    f: &File,
    path: &Path,
    dbgfile: fn(&[u8]) -> (),
    data: fn(&Dat) -> (),
    func: fn(&Fn) -> (),
) -> RubeResult<()> {
    // Allocate on the heap cos it's laaarge; TODO do we need tmph? Revert to stack
    let mut parser = Box::new(Parser::new(T, f, path, dbgfile, data, func));

    parser.parse(dbgfile, data, func)
}

impl Parser<'_> {
    pub fn parse(
        &mut self,
        dbgfile: fn(&[u8]) -> (),
        data: fn(&Dat) -> (),
        func: fn(&Fn) -> (),
    ) -> RubeResult<()> {
        loop {
            let mut lnk = Lnk {
                export: false,
                thread: false,
                align: 0,
                sec: vec![],
                secf: vec![],
            };

            match self.parselnk(&mut lnk)? {
                Token::Tdbgfile => {
                    self.expect(Token::Tstr)?;
                    dbgfile(&self.tokval.str);
                }

                Token::Tfunc => {
                    //func(parsefn(&lnk));
                }

                Token::Tdata => {
                    self.parsedat(data, &mut lnk)?;
                }

                Token::Ttype => {
                    self.parsetyp()?;
                }

                Token::Teof => {
                    break;
                }

                _ => {
                    return Err(self.err("top-level definition expected"));
                }
            }
        }

        Ok(())
    }
}

/*
static void
printcon(Con *c, FILE *f)
{
    switch (c->type) {
    case CUndef:
        break;
    case CAddr:
        if (c->sym.type == SThr)
            fprintf(f, "thread ");
        fprintf(f, "$%s", str(c->sym.id));
        if (c->bits.i)
            fprintf(f, "%+"PRIi64, c->bits.i);
        break;
    case CBits:
        if (c->flt == 1)
            fprintf(f, "s_%f", c->bits.s);
        else if (c->flt == 2)
            fprintf(f, "d_%lf", c->bits.d);
        else
            fprintf(f, "%"PRIi64, c->bits.i);
        break;
    }
}

void
printref(Ref r, Fn *fn, FILE *f)
{
    int i;
    Mem *m;

    switch (rtype(r)) {
    case RTmp:
        if (r.val < Tmp0)
            fprintf(f, "R%d", r.val);
        else
            fprintf(f, "%%%s", fn->tmp[r.val].name);
        break;
    case RCon:
        if (req(r, UNDEF))
            fprintf(f, "UNDEF");
        else
            printcon(&fn->con[r.val], f);
        break;
    case RSlot:
        fprintf(f, "S%d", rsval(r));
        break;
    case RCall:
        fprintf(f, "%04x", r.val);
        break;
    case RType:
        fprintf(f, ":%s", typ[r.val].name);
        break;
    case RMem:
        i = 0;
        m = &fn->mem[r.val];
        fputc('[', f);
        if (m->offset.type != CUndef) {
            printcon(&m->offset, f);
            i = 1;
        }
        if (!req(m->base, R)) {
            if (i)
                fprintf(f, " + ");
            printref(m->base, fn, f);
            i = 1;
        }
        if (!req(m->index, R)) {
            if (i)
                fprintf(f, " + ");
            fprintf(f, "%d * ", m->scale);
            printref(m->index, fn, f);
        }
        fputc(']', f);
        break;
    case RInt:
        fprintf(f, "%d", rsval(r));
        break;
    }
}

void
printfn(Fn *fn, FILE *f)
{
    static char ktoc[] = "wlsd";
    static char *jtoa[NJmp] = {
    #define X(j) [J##j] = #j,
        JMPS(X)
    #undef X
    };
    Blk *b;
    Phi *p;
    Ins *i;
    uint n;

    fprintf(f, "function $%s() {\n", fn->name);
    for (b=fn->start; b; b=b->link) {
        fprintf(f, "@%s\n", b->name);
        for (p=b->phi; p; p=p->link) {
            fprintf(f, "\t");
            printref(p->to, fn, f);
            fprintf(f, " =%c phi ", ktoc[p->cls]);
            assert(p->narg);
            for (n=0;; n++) {
                fprintf(f, "@%s ", p->blk[n]->name);
                printref(p->arg[n], fn, f);
                if (n == p->narg-1) {
                    fprintf(f, "\n");
                    break;
                } else
                    fprintf(f, ", ");
            }
        }
        for (i=b->ins; i<&b->ins[b->nins]; i++) {
            fprintf(f, "\t");
            if (!req(i->to, R)) {
                printref(i->to, fn, f);
                fprintf(f, " =%c ", ktoc[i->cls]);
            }
            assert(optab[i->op].name);
            fprintf(f, "%s", optab[i->op].name);
            if (req(i->to, R))
                switch (i->op) {
                case Oarg:
                case Oswap:
                case Oxcmp:
                case Oacmp:
                case Oacmn:
                case Oafcmp:
                case Oxtest:
                case Oxdiv:
                case Oxidiv:
                    fputc(ktoc[i->cls], f);
                }
            if (!req(i->arg[0], R)) {
                fprintf(f, " ");
                printref(i->arg[0], fn, f);
            }
            if (!req(i->arg[1], R)) {
                fprintf(f, ", ");
                printref(i->arg[1], fn, f);
            }
            fprintf(f, "\n");
        }
        switch (b->jmp.type) {
        case Jret0:
        case Jretsb:
        case Jretub:
        case Jretsh:
        case Jretuh:
        case Jretw:
        case Jretl:
        case Jrets:
        case Jretd:
        case Jretc:
            fprintf(f, "\t%s", jtoa[b->jmp.type]);
            if (b->jmp.type != Jret0 || !req(b->jmp.arg, R)) {
                fprintf(f, " ");
                printref(b->jmp.arg, fn, f);
            }
            if (b->jmp.type == Jretc)
                fprintf(f, ", :%s", typ[fn->retty].name);
            fprintf(f, "\n");
            break;
        case Jhlt:
            fprintf(f, "\thlt\n");
            break;
        case Jjmp:
            if (b->s1 != b->link)
                fprintf(f, "\tjmp @%s\n", b->s1->name);
            break;
        default:
            fprintf(f, "\t%s ", jtoa[b->jmp.type]);
            if (b->jmp.type == Jjnz) {
                printref(b->jmp.arg, fn, f);
                fprintf(f, ", ");
            }
            assert(b->s1 && b->s2);
            fprintf(f, "@%s, @%s\n", b->s1->name, b->s2->name);
            break;
        }
    }
    fprintf(f, "}\n");
}

 */
