use std::ascii::escape_default;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Bytes, Read, Write};
use std::path::Path;

use chomp1::ascii::{is_alpha, is_alphanumeric, is_digit};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr};

use crate::all::{
    bshas, jmp_for_cls, BSet, Blk, BlkIdx, Con, ConBits, ConIdx, ConT, Dat, DatT, DatU, Fn, Ins,
    KExt, Lnk, Mem, ORanges, Phi, PhiIdx, Ref, RubeResult, Sym, SymT, Target, Tmp, TmpIdx, Typ,
    TypFld, TypFldT, TypIdx, J, K0, KC, KD, KE, KL, KS, KSB, KSH, KUB, KUH, KW, KX, O, TMP0,
};
use crate::cfg::fillpreds;
use crate::optab::OPTAB;
use crate::util::{
    bsequal, bsinit, bsset, clsmerge, hash, intern, newcon, newtmp, str_, Bucket, InternId, IMASK,
};

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

fn to_s(raw: &[u8]) -> String {
    String::from_utf8_lossy(raw).to_string()
}

#[derive(PartialEq)]
enum PState {
    PLbl,
    PPhi,
    PIns,
    PEnd,
}

#[derive(Clone, Copy, Debug, EnumIter, FromRepr, PartialEq, PartialOrd)]
#[repr(u8)]
enum Token {
    Txxx = 0,

    // These have 1:1 correspondence with enum O public ops and MUST be the same values
    TOadd,
    TOsub,
    TOneg,
    TOdiv,
    TOrem,
    TOudiv,
    TOurem,
    TOmul,
    TOand,
    TOor,
    TOxor,
    TOsar,
    TOshr,
    TOshl,

    TOceqw,
    TOcnew,
    TOcsgew,
    TOcsgtw,
    TOcslew,
    TOcsltw,
    TOcugew,
    TOcugtw,
    TOculew,
    TOcultw,

    TOceql,
    TOcnel,
    TOcsgel,
    TOcsgtl,
    TOcslel,
    TOcsltl,
    TOcugel,
    TOcugtl,
    TOculel,
    TOcultl,

    TOceqs,
    TOcges,
    TOcgts,
    TOcles,
    TOclts,
    TOcnes,
    TOcos,
    TOcuos,

    TOceqd,
    TOcged,
    TOcgtd,
    TOcled,
    TOcltd,
    TOcned,
    TOcod,
    TOcuod,

    TOstoreb,
    TOstoreh,
    TOstorew,
    TOstorel,
    TOstores,
    TOstored,

    TOloadsb,
    TOloadub,
    TOloadsh,
    TOloaduh,
    TOloadsw,
    TOloaduw,
    TOload,

    TOextsb,
    TOextub,
    TOextsh,
    TOextuh,
    TOextsw,
    TOextuw,

    TOexts,
    TOtruncd,
    TOstosi,
    TOstoui,
    TOdtosi,
    TOdtoui,
    TOswtof,
    TOuwtof,
    TOsltof,
    TOultof,
    TOcast,

    TOalloc4,
    TOalloc8,
    TOalloc16,

    TOvaarg,
    TOvastart,

    TOcopy,

    TOdbgloc,

    // End op public ops

    /* aliases */
    Tloadw = ORanges::NPubOp as u8,
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

// This range must be numerically equal
const_assert_eq!(Token::TOadd as u8, O::Oadd as u8);
const_assert_eq!(Token::TOdbgloc as u8, O::Odbgloc as u8);

fn in_range_t(t: Token, l: Token, u: Token) -> bool {
    (l as u8) <= (t as u8) && (t as u8) <= (u as u8)
}

fn tok_to_pub_op(t: Token) -> Option<O> {
    if in_range_t(t, Token::TOadd, Token::TOdbgloc) {
        O::from_repr(t as u8)
    } else {
        None
    }
}

fn isstore(t: Token) -> bool {
    in_range_t(t, Token::TOstoreb, Token::TOstored)
}

lazy_static! {
    static ref KWMAP: [&'static [u8]; Token::Ntok as usize] = {
    // Crazy indent courtesy of emacs rustic.
    let mut kwmap0: [&'static [u8]; Token::Ntok as usize] = [b""; Token::Ntok as usize];

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

const TMASK: u32 = 16383; /* for temps hash */
const BMASK: u32 = 8191; /* for blocks hash */
const K: u32 = 9583425; /* found using tools/lexh.c */
const M: u32 = 23;

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

// Hrmmm, struggling with copyable vector
#[derive(Clone /*, Copy*/)]
enum TokVal2 {
    None,
    Eof,
    B(u8),
    S(f32),
    D(f64),
    I(i64),
    Str(Vec<u8>),
}

impl TokVal2 {
    fn as_b(&self) -> u8 {
        match self {
            TokVal2::B(b) => *b,
            _ => panic!("BUG - expecting single-char token value"),
        }
    }

    fn as_s(&self) -> f32 {
        match self {
            TokVal2::S(s) => *s,
            _ => panic!("BUG - expecting float token value"),
        }
    }

    fn as_d(&self) -> f64 {
        match self {
            TokVal2::D(d) => *d,
            _ => panic!("BUG - expecting double token value"),
        }
    }

    fn as_i(&self) -> i64 {
        match self {
            TokVal2::I(i) => *i,
            _ => panic!("BUG - expecting integer token value"),
        }
    }

    // TODO - return &[u8]
    fn as_str(&self) -> Vec<u8> {
        match self {
            TokVal2::Str(s) => s.clone(), // mmm, can we just return &[u8]?
            _ => panic!("BUG - expecting single-char token value"),
        }
    }
}

lazy_static! {
    static ref LEXH: [Token; (1 << (32 - M)) as usize] = {
        let mut lexh0: [Token; (1 << (32 - M)) as usize] = [Token::Txxx; (1 << (32 - M)) as usize];

        for t in Token::iter() {
            let i = t as usize;
            if t != Token::Ntok && !KWMAP[i].is_empty() {
                let h: u32 = hash(KWMAP[i]).wrapping_mul(K) >> M;
                assert!(lexh0[h as usize] == Token::Txxx);
                lexh0[h as usize] = t;
            }
        }

        lexh0
    };
}

// Ugh, pub for util::intern(), util::str_()
pub struct Parser<'a> {
    target: &'a Target,
    inf: Bytes<BufReader<&'a File>>,
    ungetc: Option<u8>,
    inpath: &'a Path,
    thead: (Token, TokVal2),
    tokval: TokVal,
    lnum: i32,
    tmph: [TmpIdx; (TMASK + 1) as usize],
    plink: PhiIdx,  // BlkIdx::INVALID before first phi of curb
    cur_bi: BlkIdx, // BlkIdx::INVALID before start parsing first blk
    blink: BlkIdx,  // BlkIdx::INVALID before finished parsing first blk, else prev blk
    blkh: [BlkIdx; (BMASK + 1) as usize],
    rcls: KExt,
    typ: Vec<Typ>,                            // from util.c
    insb: Vec<Ins>,                           // from util.c
    pub itbl: [Bucket; (IMASK + 1) as usize], // from util.c; string interning table; ugh pub for util::intern
}

impl Parser<'_> {
    fn new<'a>(target: &'a Target, f: &'a File, path: &'a Path) -> Parser<'a> {
        Parser {
            target,
            inf: BufReader::new(f).bytes(), // TODO use .peekable() instead of ungetc()
            ungetc: None,
            inpath: path,
            thead: (Token::Txxx, TokVal2::None),
            tokval: TokVal::new(),
            lnum: 1,
            tmph: [TmpIdx::INVALID; (TMASK + 1) as usize],
            plink: PhiIdx::INVALID,
            cur_bi: BlkIdx::INVALID,
            blink: BlkIdx::INVALID,
            blkh: [BlkIdx::INVALID; (BMASK + 1) as usize],
            rcls: K0,
            typ: vec![],
            insb: vec![],
            itbl: [(); (IMASK + 1) as usize].map(|_| Vec::new()),
        }
    }

    fn err(&self, s: &str) -> Box<ParseError> {
        Box::new(ParseError::new(format!(
            "qbe:{}:{}: {}",
            self.inpath.display(),
            self.lnum,
            s
        )))
    }

    fn getint(&mut self) -> RubeResult<i64> {
        let mut n: u64 = 0;
        let mut c = self.getc()?;
        let mut craw: u8;
        match c {
            None => return Err(self.err("end-of-file expecting integer constant")),
            Some(craw0) => craw = craw0,
        }
        let m = craw == b'-';
        if m {
            c = self.getc()?;
            match c {
                None => return Err(self.err("end-of-file after '-' in integer constant")),
                Some(craw0) => craw = craw0,
            }
            if !is_digit(craw) {
                return Err(self.err(&format!(
                    "invalid character '{}' ({:#02x?}) after '-' in integer constant",
                    escape_default(craw),
                    craw
                )));
            }
        }
        loop {
            n = n.wrapping_mul(10).wrapping_add((craw - b'0') as u64);
            c = self.getc()?;
            match c {
                None => break,
                Some(craw0) => {
                    craw = craw0;
                    if !is_digit(craw) {
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

    fn take_float_as_utf8(&mut self) -> RubeResult<String> {
        let mut bytes: Vec<u8> = vec![];
        let mut c: Option<u8>;

        loop {
            c = self.getc()?;
            match c {
                None => break, // EOF
                Some(craw) => {
                    if !(is_alphanumeric(craw)
                        || craw == b'.'
                        || (bytes.is_empty() && craw == b'-'))
                    {
                        break;
                    }
                }
            }
            bytes.push(c.unwrap());
        }
        self.ungetc(c);

        match String::from_utf8(bytes.clone()) {
            Ok(s) => Ok(s),
            Err(_) => Err(self.err(&format!(
                "invalid characters in floating point literal {:?}",
                to_s(&bytes)
            ))),
        }
    }

    fn get_float(&mut self) -> RubeResult<f32> {
        let s = self.take_float_as_utf8()?;
        let f: Result<f32, _> = s.parse();
        match f {
            Ok(v) => Ok(v),
            Err(_) => Err(self.err(&format!("invalid float literal {:?}", s))),
        }
    }

    fn get_double(&mut self) -> RubeResult<f64> {
        let s = self.take_float_as_utf8()?;
        let f: Result<f64, _> = s.parse();
        match f {
            Ok(v) => Ok(v),
            Err(_) => Err(self.err(&format!("invalid double literal {:?}", s))),
        }
    }

    // Todo use Rust peekable() iterator
    fn getc_real(&mut self) -> RubeResult<Option<u8>> {
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

    fn getc(&mut self) -> RubeResult<Option<u8>> {
        static DUMP_BYTES: bool = false;

        let r = self.getc_real();
        if DUMP_BYTES {
            if let Ok(Some(byte)) = r {
                println!(
                    "                      getc '{}' ({:#02x?})",
                    escape_default(byte),
                    byte
                );
            } else {
                println!("                      getc {:?}", r);
            }
        }
        r
    }

    fn ungetc(&mut self, c: Option<u8>) {
        assert!(self.ungetc.is_none());
        self.ungetc = c;
    }

    fn lex(&mut self) -> RubeResult<(Token, TokVal2)> {
        let mut c: Option<u8> = Some(b' ');

        let mut craw: u8;
        // Skip blanks
        loop {
            match c {
                None => return Ok((Token::Teof, TokVal2::Eof)),
                Some(craw0) => {
                    craw = craw0;
                    if craw != b' ' && craw != b'\t' {
                        break;
                    }
                }
            }
            c = self.getc()?;
        }

        let mut t: Token = Token::Txxx;
        let mut take_alpha = false;
        let mut take_quote = false;

        let tvc = TokVal2::B(craw);

        match craw {
            b',' => return Ok((Token::Tcomma, tvc)),
            b'(' => return Ok((Token::Tlparen, tvc)),
            b')' => return Ok((Token::Trparen, tvc)),
            b'{' => return Ok((Token::Tlbrace, tvc)),
            b'}' => return Ok((Token::Trbrace, tvc)),
            b'=' => return Ok((Token::Teq, tvc)),
            b'+' => return Ok((Token::Tplus, tvc)),
            b's' => {
                let c2 = self.getc()?;
                if c2 == Some(b'_') {
                    return Ok((Token::Tflts, TokVal2::S(self.get_float()?)));
                } else {
                    self.ungetc(c2);
                    take_alpha = true;
                }
            }
            b'd' => {
                let c2 = self.getc()?;
                if c2 == Some(b'_') {
                    return Ok((Token::Tfltd, TokVal2::D(self.get_double()?)));
                } else {
                    self.ungetc(c2);
                    take_alpha = true;
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
            }
            b'#' => {
                while c.is_some() && c.unwrap() != b'\n' {
                    c = self.getc()?;
                }
                self.lnum += 1;
                return Ok((Token::Tnl, tvc));
            }
            b'\n' => {
                self.lnum += 1;
                return Ok((Token::Tnl, tvc));
            }
            b'"' => {
                t = Token::Tstr;
                take_quote = true;
            }
            _ => {
                if !(is_digit(craw) || craw == b'-') {
                    take_alpha = true;
                }
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
                return Ok((t, TokVal2::Str(tok.clone())));
            }
            let h: u32 = hash(&tok).wrapping_mul(K) >> M;
            t = LEXH[h as usize];
            if t == Token::Txxx || KWMAP[t as usize] != tok {
                return Err(self.err(&format!("unknown keyword \"{:?}\"", tok)));
            }
            return Ok((t, TokVal2::Str(tok.clone())));
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
                    return Ok((t, TokVal2::Str(self.tokval.str.clone())));
                }
                esc = craw == b'\\' && !esc;
            }
        }

        if is_digit(craw) || craw == b'-' {
            self.ungetc(c);
            self.tokval.num = self.getint()?;
            return Ok((Token::Tint, TokVal2::I(self.tokval.num)));
        }

        Err(self.err(&format!(
            "unexpected character '{}' ({:#02x?})",
            escape_default(craw),
            craw
        )))
    }

    fn peek_with_val(&mut self) -> RubeResult<(Token, TokVal2)> {
        if self.thead.0 == Token::Txxx {
            self.thead = self.lex()?;
        }
        // Ugh, help - no clone()!
        Ok((self.thead.0, self.thead.1.clone()))
    }

    fn peek(&mut self) -> RubeResult<Token> {
        Ok(self.peek_with_val()?.0)
    }

    fn next(&mut self) -> RubeResult<(Token, TokVal2)> {
        let ttv = self.peek_with_val()?;
        self.thead = (Token::Txxx, TokVal2::None);
        Ok(ttv)
    }

    fn nextnl(&mut self) -> RubeResult<(Token, TokVal2)> {
        loop {
            let t = self.next()?;
            // println!("                                                        nextnl() - next() returned token {:?}", t);

            if t.0 != Token::Tnl {
                return Ok(t);
            }
        }
    }

    fn expect(&mut self, t: Token) -> RubeResult<TokVal2> {
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

        let (t1, tv1) = self.next()?;
        if t == t1 {
            return Ok(tv1);
        }

        Err(self.err(&format!(
            "{} expected, got {} instead",
            TTOA[t as usize], TTOA[t1 as usize]
        )))
    }

    fn tmpref(&mut self, v: &[u8], curf: &mut Fn) -> Ref {
        let tmp_idx: TmpIdx = self.tmph[(hash(v) & TMASK) as usize];
        if tmp_idx != TmpIdx::INVALID {
            if curf.tmps[tmp_idx.0].name == v {
                return Ref::RTmp(tmp_idx);
            }
            for t in (TMP0..curf.tmps.len()).rev() {
                if curf.tmps[t].name == v {
                    return Ref::RTmp(TmpIdx(t));
                }
            }
        }
        let t = curf.tmps.len();
        let r = newtmp(None, KX, curf);
        curf.tmps[t].name = v.to_vec(); // Ugh

        r
    }

    fn parseref(&mut self, curf: &mut Fn) -> RubeResult<Ref> {
        let (t, tv) = self.next()?;
        let c: Con = match t {
            Token::Ttmp => return Ok(self.tmpref(&tv.as_str(), curf)),
            Token::Tint => Con::new_bits(ConBits::I(tv.as_i())),
            Token::Tflts => Con::new_bits(ConBits::F(tv.as_s())), // c.flt = 1;
            Token::Tfltd => Con::new_bits(ConBits::D(tv.as_d())), // c.flt = 2;
            Token::Tthread => {
                let name = self.expect(Token::Tglo)?;
                Con::new_sym(Sym::new(SymT::SThr, intern(&name.as_str(), self)))
            }
            Token::Tglo => Con::new_sym(Sym::new(SymT::SGlo, intern(&tv.as_str(), self))), // Ugh
            _ => return Ok(Ref::R), // TODO, hrmmm - return Ok???
        };

        Ok(newcon(c, curf))
    }

    fn findtyp(&self, name: &[u8]) -> RubeResult<TypIdx> {
        for i in (0..self.typ.len()).rev() {
            if name == self.typ[i].name {
                return Ok(TypIdx(i));
            }
        }
        Err(self.err(&format!("undefined type :{}", to_s(name))))
    }

    fn parsecls(&mut self) -> RubeResult<(KExt, TypIdx)> {
        let (t, tv) = self.next()?;
        match t {
            Token::Ttyp => Ok((KC, self.findtyp(&tv.as_str())?)),
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

fn op_arg_bh(k: KExt) -> O {
    match k {
        KSB => O::Oargsb,
        KUB => O::Oargub,
        KSH => O::Oargsh,
        KUH => O::Oarguh,
        _ => panic!("BUG: expected byte/short type but got {:?}", k),
    }
}

fn op_par_bh(k: KExt) -> O {
    match k {
        KSB => O::Oparsb,
        KUB => O::Oparub,
        KSH => O::Oparsh,
        KUH => O::Oparuh,
        _ => panic!("BUG: expected byte/short type but got {:?}", k),
    }
}

impl Parser<'_> {
    fn parserefl(&mut self, arg: bool, curf: &mut Fn) -> RubeResult<bool> {
        let mut ty: TypIdx = TypIdx::INVALID;
        let mut k: KExt = KE; // KW???
        let mut env: bool = false;
        let mut hasenv: bool = false;
        let mut vararg: bool = false;

        self.expect(Token::Tlparen)?;

        while self.peek()? != Token::Trparen {
            if !arg && vararg {
                return Err(self.err("no parameters allowed after '...'"));
            }

            let mut goto_next: bool = false;
            match self.peek()? {
                Token::Tdots => {
                    if vararg {
                        return Err(self.err("only one '...' allowed"));
                    }
                    vararg = true;
                    if arg {
                        // TODO - Mmm, would actually like Ins's to be on Blk's
                        self.insb.push(Ins::new0(O::Oargv, KW, Ref::R)); // TODO - KW is 0 but seems wrong???
                    }
                    let _ = self.next()?;
                    goto_next = true;
                }
                Token::Tenv => {
                    if hasenv {
                        return Err(self.err("only one environment allowed"));
                    }
                    hasenv = true;
                    env = true;
                    let _ = self.next()?;
                    k = KL;
                }
                _ => {
                    env = false;
                    (k, ty) = self.parsecls()?;
                }
            }

            if !goto_next {
                let r: Ref = self.parseref(curf)?;
                match r {
                    Ref::R => return Err(self.err("invalid argument")),
                    Ref::RTmp(_) => (), // Ok
                    _ => {
                        if !arg {
                            return Err(self.err("invalid function parameter"));
                        }
                    }
                }
                let ins: Ins = {
                    if env {
                        if arg {
                            Ins::new1(O::Oarge, k, Ref::R, [r])
                        } else {
                            Ins::new1(O::Opare, k, r, [Ref::R])
                        }
                    } else if k == KC {
                        if arg {
                            Ins::new2(O::Oargc, KL, Ref::R, [Ref::RTyp(ty), r])
                        } else {
                            Ins::new1(O::Oparc, KL, r, [Ref::RTyp(ty)])
                        }
                    } else if k >= KSB {
                        if arg {
                            Ins::new1(op_arg_bh(k), KW, Ref::R, [r])
                        } else {
                            Ins::new1(op_par_bh(k), KW, r, [Ref::R])
                        }
                    } else {
                        if arg {
                            Ins::new1(O::Oarg, k, Ref::R, [r])
                        } else {
                            Ins::new1(O::Opar, k, r, [Ref::R])
                        }
                    }
                };
                self.insb.push(ins);
            }
            if self.peek()? == Token::Trparen {
                break;
            }
            self.expect(Token::Tcomma)?;
        }
        self.expect(Token::Trparen)?;

        Ok(vararg)
    }

    // Blk name is always in self.tokval.str
    // TODO - pass tv.as_str()
    fn findblk(&mut self, curf: &mut Fn, name: &[u8]) -> BlkIdx {
        let name: &[u8] = &self.tokval.str;
        let h: u32 = hash(name) & BMASK;
        let mut bi: BlkIdx = self.blkh[h as usize];
        while bi != BlkIdx::INVALID {
            let b: &Blk = curf.blk(bi);
            if b.name == name {
                return bi;
            }

            bi = b.dlink;
        }

        let id: usize = curf.blks.len();
        bi = curf.add_blk(Blk::new(name, id, self.blkh[h as usize]));
        self.blkh[h as usize] = bi;

        bi
    }

    fn closeblk(&mut self, curf: &mut Fn) {
        let curb: &mut Blk = curf.blk_mut(self.cur_bi);
        // TODO - this is silly, just use Blk::ins directly
        curb.ins = self.insb.clone();
        self.blink = self.cur_bi;
        self.insb.clear();
    }

    fn parseline(&mut self, ps: PState, curf: &mut Fn) -> RubeResult<PState> {
        // Instruction (or phi) arguments
        let mut arg: Vec<Ref> = vec![];
        // Phi targets
        let mut blk: Vec<BlkIdx> = vec![];
        let mut r: Ref = Ref::R;
        let mut k: KExt = KE; // KW???
        let mut ty: TypIdx = TypIdx::INVALID;

        let mut op_tok: Token = Token::Txxx;
        let mut op_tv: TokVal2 = TokVal2::None;

        let mut t: Token;
        let mut tv: TokVal2;

        (t, tv) = self.nextnl()?;

        if ps == PState::PLbl && t != Token::Tlbl && t != Token::Trbrace {
            return Err(self.err("label or } expected"));
        }

        let mut goto_close: bool = false;
        let mut goto_ins: bool = false;

        // In this block we produce an op_tok Token.
        // Below we will translate it to the corresponding O:: op.
        match t {
            // Instruction returning a value
            Token::Ttmp => {
                r = self.tmpref(&tv.as_str(), curf);
                self.expect(Token::Teq)?;
                (k, ty) = self.parsecls()?;
                (op_tok, op_tv) = self.next()?;
            }
            // Void instructions
            Token::Tblit | Token::Tcall | Token::TOvastart => {
                /* operations without result */
                r = Ref::R;
                k = KW; // why not T0?
                op_tok = t;
            }
            // End of function
            Token::Trbrace => return Ok(PState::PEnd),
            // New block
            Token::Tlbl => {
                let new_bi: BlkIdx = self.findblk(curf, &tv.as_str());
                if self.cur_bi != BlkIdx::INVALID && curf.blk(self.cur_bi).jmp.type_ == J::Jxxx {
                    self.closeblk(curf);
                    let curb: &mut Blk = curf.blk_mut(self.cur_bi);
                    curb.jmp.type_ = J::Jjmp;
                    curb.s1 = new_bi;
                }
                let new_b: &Blk = curf.blk(new_bi);
                if new_b.jmp.type_ != J::Jxxx {
                    return Err(self.err(&format!(
                        "multiple definitions of block @{}",
                        to_s(&new_b.name),
                    )));
                }
                if self.blink == BlkIdx::INVALID {
                    // First block
                    curf.start = new_bi;
                } else {
                    curf.blk_mut(self.cur_bi).link = new_bi;
                }
                self.cur_bi = new_bi;
                self.plink = PhiIdx::INVALID;
                self.expect(Token::Tnl)?;
                return Ok(PState::PPhi);
            }
            // Return instruction - ends block
            Token::Tret => {
                curf.blk_mut(self.cur_bi).jmp.type_ = match jmp_for_cls(self.rcls) {
                    None => {
                        return Err(self.err(&format!("BUG: invalid type {:?} for ret", self.rcls)))
                    }
                    Some(j) => j,
                };
                if self.peek()? == Token::Tnl {
                    curf.blk_mut(self.cur_bi).jmp.type_ = J::Jret0; // is this necessary?
                } else if self.rcls != K0 {
                    let r: Ref = self.parseref(curf)?;
                    if r == Ref::R {
                        return Err(self.err("invalid return value"));
                    }
                    curf.blk_mut(self.cur_bi).jmp.arg = r;
                }
                goto_close = true;
            }
            // Jump instruction - ends block
            Token::Tjmp | Token::Tjnz => {
                if t == Token::Tjmp {
                    curf.blk_mut(self.cur_bi).jmp.type_ = J::Jjmp;
                } else {
                    curf.blk_mut(self.cur_bi).jmp.type_ = J::Jjnz;
                    let r: Ref = self.parseref(curf)?;
                    if let Ref::R = r {
                        return Err(self.err("invalid argument for jnz jump"));
                    }
                    curf.blk_mut(self.cur_bi).jmp.arg = r;
                    self.expect(Token::Tcomma)?;
                }
                // Jump:
                let name = self.expect(Token::Tlbl)?.as_str();
                curf.blk_mut(self.cur_bi).s1 = self.findblk(curf, &name);
                if curf.blk(self.cur_bi).jmp.type_ != J::Jjmp {
                    self.expect(Token::Tcomma)?;
                    let name = self.expect(Token::Tlbl)?.as_str();
                    curf.blk_mut(self.cur_bi).s2 = self.findblk(curf, &name);
                }
                if curf.blk(self.cur_bi).s1 == curf.start || curf.blk(self.cur_bi).s2 == curf.start
                {
                    return Err(self.err("invalid jump to the start block"));
                }
                goto_close = true;
            }
            // Halt instruction - ends block
            Token::Thlt => {
                curf.blk_mut(self.cur_bi).jmp.type_ = J::Jhlt;
                goto_close = true;
            }
            // Debug line/column location tag
            Token::TOdbgloc => {
                op_tok = t;
                k = KW;
                r = Ref::R;
                let ln_i = self.expect(Token::Tint)?.as_i();
                //assert!(ln_i == self.tokval.num);
                let ln: i32 = /*self.tokval.num*/ln_i as i32;
                if ln < 0 || (ln as i64) != ln_i
                /*self.tokval.num*/
                {
                    return Err(self.err(&format!(
                        "line number {} negative or too big",
                        /*self.tokval.num*/ ln_i
                    )));
                }
                arg.push(Ref::RInt(ln));
                // TODO - didn't clean this up
                let cn: i32 = {
                    if self.peek()? == Token::Tcomma {
                        self.next()?;
                        let cn_i = self.expect(Token::Tint)?.as_i();
                        //assert!(cn_i == self.tokval.num);
                        let cn0: i32 = cn_i as i32;
                        if cn0 < 0 || (cn0 as i64) != cn_i
                        /*self.tokval.num*/
                        {
                            return Err(self.err(&format!(
                                "column number {} negative or too big",
                                /*self.tokval.num*/ cn_i
                            )));
                        }
                        cn0
                    } else {
                        0
                    }
                };
                arg.push(Ref::RInt(cn));
                goto_ins = true;
            }
            _ => {
                if isstore(t) {
                    /* operations without result */
                    r = Ref::R;
                    k = KW; // TODO why not K0?
                    op_tok = t;
                } else {
                    return Err(self.err("label, instruction or jump expected"));
                }
            }
        }

        assert!(!(goto_close && goto_ins));

        if goto_close {
            self.expect(Token::Tnl)?;
            self.closeblk(curf);
            return Ok(PState::PLbl);
        }

        let mut op: O = O::Oxxx;

        if !goto_ins {
            if op_tok == Token::Tcall {
                // Call instruction
                arg.push(self.parseref(curf)?);
                self.parserefl(true, curf)?;
                op = O::Ocall;
                self.expect(Token::Tnl)?;
                let arg1 = {
                    if k == KC {
                        k = KL;
                        Ref::RTyp(ty)
                    } else {
                        Ref::R
                    }
                };
                arg.push(arg1);
                if k >= KSB {
                    // else if ???
                    k = KW;
                }
            } else {
                // Alias instructions
                if op_tok == Token::Tloadw {
                    op_tok = Token::TOloadsw;
                } else if op_tok >= Token::Tloadl && op_tok <= Token::Tloadd {
                    op_tok = Token::TOload; // TODO - weird, weaking type?
                }
                if op_tok == Token::TOvastart && !curf.vararg {
                    return Err(self.err("cannot use vastart in non-variadic function"));
                } else if op_tok == Token::Talloc1 || op_tok == Token::Talloc2 {
                    op_tok = Token::TOalloc4; // Interesting, byte/short alloc promoted to word
                }

                if k >= KSB {
                    return Err(self.err("size class must be w, l, s, or d"));
                }
                // Instruction args
                if self.peek()? != Token::Tnl {
                    loop {
                        if op_tok == Token::Tphi {
                            let name = self.expect(Token::Tlbl)?.as_str();
                            blk.push(self.findblk(curf, &name));
                        }
                        let argi: Ref = self.parseref(curf)?;
                        if let Ref::R = argi {
                            return Err(self.err("invalid instruction argument"));
                        }
                        arg.push(argi);
                        t = self.peek()?;
                        if t == Token::Tnl {
                            break;
                        }
                        if t != Token::Tcomma {
                            return Err(self.err(", or end of line expected"));
                        }
                        self.next()?;
                    }
                }
                self.next()?;
                match op_tok {
                    Token::Tphi => {
                        if ps != PState::PPhi || self.cur_bi == curf.start {
                            return Err(self.err("unexpected phi instruction"));
                        }
                        let phii = PhiIdx(curf.phis.len());
                        curf.phis
                            .push(Phi::new(r, arg.clone(), blk.clone(), k, PhiIdx::INVALID));
                        if self.plink == PhiIdx::INVALID {
                            curf.blk_mut(self.cur_bi).phi = phii;
                        } else {
                            let prev_phi = &mut curf.phis[self.plink.0];
                            prev_phi.link = phii;
                        }
                        self.plink = phii;
                        return Ok(PState::PPhi);
                    }
                    Token::Tblit => {
                        if arg.len() < 3 {
                            return Err(self.err("insufficient args for blit"));
                        }
                        self.insb
                            .push(Ins::new2(O::Oblit0, K0, Ref::R, [arg[0], arg[1]]));
                        let coni: ConIdx;
                        if let Ref::RCon(coni0) = arg[2] {
                            coni = coni0;
                        } else {
                            return Err(self.err("blit size must be constant"));
                        }
                        let c: &Con = &curf.con[coni.0];
                        let sz: u32 = {
                            let mut sz_u32: u32 = 0;
                            let mut is_valid_size: bool = c.type_ == ConT::CBits;
                            if is_valid_size {
                                if let ConBits::I(sz_i64) = c.bits {
                                    sz_u32 = sz_i64 as u32;
                                    is_valid_size = sz_i64 >= 0 && sz_i64 == (sz_u32 as i64);
                                } else {
                                    return Err(self.err("blit size must be integer constant"));
                                }
                            }
                            if !is_valid_size {
                                return Err(self.err("blit size negative or too large"));
                            }
                            sz_u32
                        };
                        let r: Ref = Ref::RInt(sz as i32); /* Mmm */
                        self.insb.push(Ins::new1(O::Oblit1, K0, Ref::R, [r]));
                        return Ok(PState::PIns);
                    }
                    _ => {
                        op = match tok_to_pub_op(op_tok) {
                            None => return Err(self.err("invalid instruction")),
                            Some(op0) => op0,
                        };
                    }
                }
            }
        }
        while arg.len() < 2 {
            arg.push(Ref::R);
        }
        self.insb.push(Ins::new2(op, k, r, [arg[0], arg[1]]));

        return Ok(PState::PIns);
    }
}

/*
static int
usecheck(Ref r, int k, Fn *fn)
{
    return rtype(r) != RTmp || fn->tmp[r.val].cls == k
        || (fn->tmp[r.val].cls == Kl && k == Kw);
}
 */

fn usecheck(fn_: &Fn, r: &Ref, k: KExt) -> bool {
    match r {
        Ref::RTmp(ti) => {
            let cls: KExt = fn_.tmps[ti.0].cls;
            cls == k || (cls == KL && k == KW)
        }
        _ => false,
    }
}

/*
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

 */
impl Parser<'_> {
    fn typecheck(&self, fn_: &mut Fn) -> RubeResult<()> {
        fillpreds(fn_);
        let mut bi: BlkIdx = fn_.start;
        while bi != BlkIdx::INVALID {
            let mut pi: PhiIdx = fn_.blk(bi).phi;
            while pi != PhiIdx::INVALID {
                if let Ref::RTmp(ti) = fn_.phi(pi).to {
                    fn_.tmp_mut(ti).cls = fn_.phi(pi).cls;
                } else {
                    return Err(self.err(&format!(
                        "phi to val is not a tmp in block {}",
                        to_s(&fn_.blk(bi).name)
                    )));
                }
                pi = fn_.phi(pi).link;
            }
            for ii in 0..fn_.blk(bi).ins.len() {
                if let Ref::RTmp(ti) = fn_.blk(bi).ins[ii].to {
                    let ins_cls: KExt = fn_.blk(bi).ins[ii].cls;
                    let t: &mut Tmp = fn_.tmp_mut(ti);
                    if clsmerge(&mut t.cls, ins_cls) {
                        return Err(self.err(&format!(
                            "temporary %{} is assigned with multiple types",
                            to_s(&t.name)
                        )));
                    }
                }
            }
            bi = fn_.blk(bi).link;
        }

        bi = fn_.start;
        while bi != BlkIdx::INVALID {
            let b: &Blk = fn_.blk(bi);

            let mut pb: BSet = bsinit(fn_.blks.len());

            for pred_bi in &b.pred {
                bsset(&mut pb, pred_bi.0);
            }
            let mut pi = b.phi;
            while pi != PhiIdx::INVALID {
                let p: &Phi = fn_.phi(pi);
                if let Ref::RTmp(ti) = p.to {
                    let t: &Tmp = fn_.tmp(ti);
                    let k: KExt = t.cls;
                    let mut ppb: BSet = bsinit(fn_.blks.len());
                    for n in 0..fn_.phi(pi).arg.len() {
                        let pbi: BlkIdx = p.blk[n];
                        if bshas(&ppb, pbi.0) {
                            return Err(self.err(&format!(
                                "multiple entries for @{} in phi %{}",
                                to_s(&fn_.blk(pbi).name),
                                to_s(&t.name)
                            )));
                        }
                        if !usecheck(fn_, &p.arg[n], k) {
                            // TODO - notify QBE - this might not be a tmp - could be a constant
                            let argr: &Ref = &p.arg[n];
                            if let Ref::RTmp(ti) = argr {
                                return Err(self.err(&format!(
                                    "invalid type for operand %{} in phi %{}",
                                    to_s(&fn_.tmp(*ti).name),
                                    to_s(&t.name)
                                )));
                            } else {
                                return Err(self.err(&format!(
                                    "invalid type for operand {} in phi %{}",
                                    n,
                                    to_s(&t.name)
                                )));
                            }
                        }
                        bsset(&mut ppb, pbi.0);
                    }

                    if !bsequal(&pb, &ppb) {
                        return Err(self.err(&format!(
                            "predecessors not matched in phi %{}",
                            to_s(&t.name)
                        )));
                    }

                    pi = fn_.phi(pi).link;
                } else {
                    assert!(false); // Already checked above
                }
            }
            bi = fn_.blk(bi).link;
        }

        Ok(())
    }

    fn parsefn(&mut self, lnk: &Lnk) -> RubeResult<Fn> {
        self.cur_bi = BlkIdx::INVALID;
        self.insb.clear(); // TODO would prefer Ins's on Blk's...
        let mut curf = Fn::new(lnk.clone());
        for i in 0..(TMP0 as i32) {
            if self.target.fpr0 <= i && i < self.target.fpr0 + self.target.nfpr {
                let _ = newtmp(None, KD, &mut curf);
            } else {
                let _ = newtmp(None, KL, &mut curf);
            }
        }

        curf.con.push(Con::new(
            ConT::CBits,
            Sym::new(SymT::SGlo, InternId::INVALID),
            ConBits::I(0xdeaddead),
        )); /* UNDEF */
        // ??? what's this for?
        curf.con.push(Con::new(
            ConT::CBits,
            Sym::new(SymT::SGlo, InternId::INVALID),
            ConBits::I(0),
        ));
        curf.lnk = lnk.clone();

        self.blink = BlkIdx::INVALID;

        if self.peek()? != Token::Tglo {
            (self.rcls, curf.retty) = self.parsecls()?;
        }
        let (t, tv) = self.next()?;
        if t != Token::Tglo {
            return Err(self.err("function name expected"));
        }
        curf.name = tv.as_str();
        curf.vararg = self.parserefl(false, &mut curf)?;
        if self.nextnl()?.0 != Token::Tlbrace {
            return Err(self.err("function body must start with {"));
        }
        let mut ps: PState = PState::PLbl;
        loop {
            ps = self.parseline(ps, &mut curf)?;
            if ps == PState::PEnd {
                break;
            }
        }
        if self.cur_bi == BlkIdx::INVALID {
            return Err(self.err("empty function"));
        } else if curf.blk(self.cur_bi).jmp.type_ == J::Jxxx {
            return Err(self.err("last block misses jump"));
        }

        let mut bi = curf.start;
        while bi != BlkIdx::INVALID {
            let b: &mut Blk = curf.blk_mut(bi);
            b.dlink = BlkIdx::INVALID;
            bi = b.link;
        }
        for i in 0..BMASK + 1 {
            self.blkh[i as usize] = BlkIdx::INVALID;
        }
        for i in 0..TMASK + 1 {
            self.tmph[i as usize] = TmpIdx::INVALID;
        }

        println!("TODO - missing typecheck()");
        //self.typecheck(&mut curf)?;

        Ok(curf)
    }

    // TODO - this should just return a Vec<TypField>
    fn parsefields(&mut self, ty: &mut Typ, tparam: Token, tvparam: TokVal2) -> RubeResult<()> {
        let mut t: Token = tparam;
        let mut tv: TokVal2 = tvparam;
        let mut sz: u64 = 0;
        let mut al = ty.align;
        while t != Token::Trbrace {
            // let type_: TypFldT;
            // let mut s: u64;
            // let mut a: i32;
            let mut ftyp_idx: usize = 0;

            // TODO: Hrmmm, these are all unused, something missing
            let (type_, mut s, mut a) = match t {
                Token::Td => (TypFldT::Fd, 8u64, 3i32),
                Token::Tl => (TypFldT::Fl, 8u64, 3i32),
                Token::Ts => (TypFldT::Fs, 4u64, 2i32),
                Token::Tw => (TypFldT::Fw, 4u64, 2i32),
                Token::Th => (TypFldT::Fh, 2u64, 1i32),
                Token::Tb => (TypFldT::Fb, 1u64, 0i32),
                Token::Ttyp => {
                    let TypIdx(idx) = self.findtyp(&tv.as_str())?;
                    ftyp_idx = idx;
                    (TypFldT::FTyp, self.typ[idx].size, self.typ[idx].align)
                }
                _ => return Err(self.err("invalid type member specifier")),
            };
            if a > al {
                al = a;
            }
            a = (1 << a) - 1;
            a = (((sz as i32) + a) & !a) - (sz as i32); // TODO - this is fugly casting
            if a != 0 {
                ty.fields.push(TypFld::new(TypFldT::FPad, a as u32));
            }
            (t, tv) = self.nextnl()?;
            let mut c: i32 = 1;
            if t == Token::Tint {
                c = tv.as_i() as i32; // TODO - check cast range?
                (t, tv) = self.nextnl()?;
            }
            sz += (a as u64) + (c as u64) * s;
            if type_ == TypFldT::FTyp {
                s = ftyp_idx as u64;
            }
            while c > 0 {
                ty.fields.push(TypFld::new(type_, s as u32)); // ugh

                c -= 1;
            }
            if t != Token::Tcomma {
                break;
            }
            (t, tv) = self.nextnl()?;
        }
        if t != Token::Trbrace {
            return Err(self.err(", or } expected"));
        }
        let a: i32 = 1 << al;
        if sz < ty.size {
            sz = ty.size;
        }
        ty.size = (sz + (a as i64 as u64) - 1) & (-a as i64 as u64); // TODO ugh
        ty.align = al;

        Ok(())
    }

    // TODO - should return Typ???
    // TODO: need to pass tv
    fn parsetyp(&mut self) -> RubeResult<()> {
        /* be careful if extending the syntax
         * to handle nested types, any pointer
         * held to typ[] might be invalidated!
         */
        let mut ty = Typ::new(); // Ugh construct upwards

        if self.nextnl()?.0 != Token::Ttyp || self.nextnl()?.0 != Token::Teq {
            return Err(self.err("type name and then = expected"));
        }

        // TODO - we need to pass tvparam here and move this up...
        ty.name = self.tokval.str.clone();
        let (mut t, mut tv) = self.nextnl()?;
        if t == Token::Talign {
            (t, tv) = self.nextnl()?;
            if t != Token::Tint {
                return Err(self.err("alignment expected"));
            }
            let mut al_exp = tv.as_i();
            let mut al: i32 = 0;
            // TODO - there must be a better way of doing this - hi bit and check 2^N
            loop {
                al_exp /= 2;
                if al_exp == 0 {
                    break;
                }
                al += 1;
            }
            ty.align = al;
            (t, tv) = self.nextnl()?;
        }
        if t != Token::Tlbrace {
            return Err(self.err("type body must start with {"));
        }
        (t, tv) = self.nextnl()?;
        if t == Token::Tint {
            ty.isdark = true;
            ty.size = tv.as_i() as u64; // TODO: QBE notify? Mmm check negative value?
            if ty.align == -1 {
                return Err(self.err("dark types need alignment"));
            }
            if self.nextnl()?.0 != Token::Trbrace {
                return Err(self.err("} expected"));
            }
            self.typ.push(ty);
            return Ok(());
        }
        let mut n: u32 = 0;
        if t == Token::Tlbrace {
            ty.isunion = true;
            loop {
                if t != Token::Tlbrace {
                    return Err(self.err("invalid union member"));
                }
                (t, tv) = self.nextnl()?;
                self.parsefields(&mut ty, t, tv)?;
                n += 1;
                (t, tv) = self.nextnl()?;
                if t == Token::Trbrace {
                    break;
                }
            }
        } else {
            self.parsefields(&mut ty, t, tv)?;
            n += 1;
        }
        ty.nunion = n;
        self.typ.push(ty);
        Ok(())
    }

    // TODO - should just return Dat???
    // TODO - need to pass tv
    fn parsedatref(&mut self, d: &mut Dat) -> RubeResult<()> {
        d.isref = true;
        let name: Vec<u8> = self.tokval.str.clone();
        let mut off: i64 = 0;
        if self.peek()? == Token::Tplus {
            let _ = self.next()?;
            let (t, tv) = self.next()?;
            if t != Token::Tint {
                return Err(self.err("invalid token after offset in ref"));
            }
            off = tv.as_i();
        }
        d.u = DatU::Ref { name, off };

        Ok(())
    }

    // TODO - need to pass tv.as_str()
    fn parsedatstr(&self, d: &mut Dat) {
        d.isstr = true;
        d.u = DatU::Str(self.tokval.str.clone());
    }

    // TODO - need to pass tv.as_str()
    fn parsedat(&mut self, cb: fn(&Dat, &[Typ]) -> (), lnk: &mut Lnk) -> RubeResult<()> {
        if self.nextnl()?.0 != Token::Tglo || self.nextnl()?.0 != Token::Teq {
            return Err(self.err("data name, then = expected"));
        }

        let name: Vec<u8> = self.tokval.str.clone();
        let (mut t, mut tv) = self.nextnl()?;
        lnk.align = 8;
        if t == Token::Talign {
            (t, tv) = self.nextnl()?;
            if t != Token::Tint {
                return Err(self.err("alignment expected"));
            }
            lnk.align = tv.as_i() as i8; // TODO - check casting range?
            (t, tv) = self.nextnl()?;
        }

        let mut d: Dat = Dat::new(DatT::DStart, &name, lnk.clone());
        cb(&d, &self.typ);

        if t != Token::Tlbrace {
            return Err(self.err("expected data contents in { .. }"));
        }
        loop {
            (t, tv) = self.nextnl()?;
            d.type_ = match t {
                Token::Trbrace => break,
                Token::Tl => DatT::DL,
                Token::Tw => DatT::DW,
                Token::Th => DatT::DH,
                Token::Tb => DatT::DB,
                Token::Ts => DatT::DW,
                Token::Td => DatT::DL,
                Token::Tz => DatT::DZ,
                _ => {
                    let b = tv.as_b();
                    return Err(self.err(&format!(
                        "invalid size specifier '{}' ({:#02x?}) in data",
                        escape_default(b),
                        b
                    )));
                }
            };
            (t, tv) = self.nextnl()?;
            loop {
                d.isstr = false;
                d.isref = false;
                d.u = DatU::None;

                match t {
                    Token::Tflts => d.u = DatU::Flts(tv.as_s()),
                    Token::Tfltd => d.u = DatU::Fltd(tv.as_d()),
                    Token::Tint => d.u = DatU::Num(tv.as_i()),
                    Token::Tglo => self.parsedatref(&mut d)?,
                    Token::Tstr => self.parsedatstr(&mut d),
                    _ => {
                        return Err(self.err("constant literal or global ref expected in data"));
                    }
                }
                cb(&d, &self.typ);
                (t, tv) = self.nextnl()?;
                if !(t == Token::Tint
                    || t == Token::Tflts
                    || t == Token::Tfltd
                    || t == Token::Tstr
                    || t == Token::Tglo)
                {
                    break;
                }
            }
            if t == Token::Trbrace {
                break;
            }
            if t != Token::Tcomma {
                return Err(self.err(", or } expected in data"));
            }
        }
        d.type_ = DatT::DEnd;
        cb(&d, &self.typ);

        Ok(())
    }

    fn parselnk(&mut self, lnk: &mut Lnk) -> RubeResult<Token> {
        let mut haslnk: bool = false;

        loop {
            let (mut t, mut tv) = self.nextnl()?;

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
                    (t, tv) = self.next()?;
                    if t != Token::Tstr {
                        return Err(self.err("section \"name\" expected"));
                    }
                    lnk.sec = tv.as_str();
                    if self.peek()? == Token::Tstr {
                        (t, tv) = self.next()?;
                        lnk.secf = tv.as_str();
                    } else {
                        panic!("unhandled");
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

    pub fn parse(
        &mut self,
        dbgfile: fn(&[u8]) -> (),
        data: fn(&Dat, &[Typ]) -> (),
        func: fn(&Fn, &[Typ], &[Bucket]) -> (),
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
                    let s = self.expect(Token::Tstr)?;
                    dbgfile(&s.as_str());
                }

                Token::Tfunc => {
                    func(&self.parsefn(&lnk)?, &self.typ, &self.itbl);
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

pub fn parse(
    target: &Target,
    f: &File,
    path: &Path,
    dbgfile: fn(&[u8]) -> (),
    data: fn(&Dat, &[Typ]) -> (),
    func: fn(&Fn, &[Typ], &[Bucket]) -> (),
) -> RubeResult<()> {
    // Allocate on the heap cos it's laaarge; TODO do we need tmph? Revert to stack
    let mut parser = Box::new(Parser::new(target, f, path));

    parser.parse(dbgfile, data, func)
}

pub fn printcon(f: &mut dyn Write, itbl: &[Bucket], c: &Con) {
    match c.type_ {
        ConT::CUndef => {
            assert!(false); // nada
            let _ = write!(f, "");
        }
        ConT::CAddr => {
            if c.sym.type_ == SymT::SThr {
                let _ = write!(f, "thread ");
            }
            let _ = write!(f, "${}", to_s(str_(&c.sym.id, itbl)));
            if let ConBits::I(i) = c.bits {
                let _ = write!(f, "{}", i);
            }
        }
        ConT::CBits => match c.bits {
            ConBits::None => {
                assert!(false);
                let _ = write!(f, "");
            }
            ConBits::F(s) => {
                let _ = write!(f, "s_{}", s);
            }
            ConBits::D(d) => {
                let _ = write!(f, "d_{}", d);
            }
            ConBits::I(i) => {
                let _ = write!(f, "{}", i);
            }
        },
    }
}

fn printref(f: &mut dyn Write, fn_: &Fn, typ: &[Typ], itbl: &[Bucket], r: &Ref) {
    match r {
        Ref::R => assert!(false),
        Ref::RTmp(ti) => {
            if ti.0 < TMP0 {
                let _ = write!(f, "R{}", ti.0);
            } else {
                let _ = write!(f, "%{}", to_s(&fn_.tmps[ti.0].name));
            }
        }
        Ref::RCon(ci) => {
            if ci.0 == 0 {
                //(req(r, UNDEF)) { TODO - this seems missing
                let _ = write!(f, "UNDEF");
            } else {
                printcon(f, itbl, &fn_.con[ci.0]);
            }
        }
        Ref::RSlot(i) => {
            let _ = write!(f, "S{}", *i as i32);
        }
        Ref::RCall(n) => {
            let _ = write!(f, "{:04x}", n);
        }
        Ref::RTyp(ti) => {
            let _ = write!(f, ":{}", to_s(&typ[ti.0].name));
        }
        Ref::RMem(mi) => {
            let mut i: bool = false;
            let m: &Mem = &fn_.mem[mi.0];
            let _ = write!(f, "[");
            if m.offset.type_ != ConT::CUndef {
                printcon(f, itbl, &m.offset);
                i = true;
            }
            if m.base != Ref::R {
                if i {
                    let _ = write!(f, " + ");
                }
                printref(f, fn_, typ, itbl, &m.base);
                i = true;
            }
            if m.index != Ref::R {
                if i {
                    let _ = write!(f, " + ");
                }
                let _ = write!(f, "{} * ", m.scale);
                printref(f, fn_, typ, itbl, &m.index);
            }
            let _ = write!(f, "]");
        }
        Ref::RInt(i) => {
            let _ = write!(f, "{}", *i as i32);
        }
    }
}

pub fn printfn(f: &mut dyn Write, fn_: &Fn, typ: &[Typ], itbl: &[Bucket]) {
    static KTOC: [&str; 4] = ["w", "l", "s", "d"];
    // static char ktoc[] = "wlsd";
    // Generated from gcc -E and hand-munged
    static JTOA: [&str; J::NJmp as usize] = {
        let mut jtoa0: [&str; J::NJmp as usize] = [""; J::NJmp as usize];

        jtoa0[J::Jretw as usize] = "retw";
        jtoa0[J::Jretl as usize] = "retl";
        jtoa0[J::Jrets as usize] = "rets";
        jtoa0[J::Jretd as usize] = "retd";
        jtoa0[J::Jretsb as usize] = "retsb";
        jtoa0[J::Jretub as usize] = "retub";
        jtoa0[J::Jretsh as usize] = "retsh";
        jtoa0[J::Jretuh as usize] = "retuh";
        jtoa0[J::Jretc as usize] = "retc";
        jtoa0[J::Jret0 as usize] = "ret0";
        jtoa0[J::Jjmp as usize] = "jmp";
        jtoa0[J::Jjnz as usize] = "jnz";
        jtoa0[J::Jjfieq as usize] = "jfieq";
        jtoa0[J::Jjfine as usize] = "jfine";
        jtoa0[J::Jjfisge as usize] = "jfisge";
        jtoa0[J::Jjfisgt as usize] = "jfisgt";
        jtoa0[J::Jjfisle as usize] = "jfisle";
        jtoa0[J::Jjfislt as usize] = "jfislt";
        jtoa0[J::Jjfiuge as usize] = "jfiuge";
        jtoa0[J::Jjfiugt as usize] = "jfiugt";
        jtoa0[J::Jjfiule as usize] = "jfiule";
        jtoa0[J::Jjfiult as usize] = "jfiult";
        jtoa0[J::Jjffeq as usize] = "jffeq";
        jtoa0[J::Jjffge as usize] = "jffge";
        jtoa0[J::Jjffgt as usize] = "jffgt";
        jtoa0[J::Jjffle as usize] = "jffle";
        jtoa0[J::Jjfflt as usize] = "jfflt";
        jtoa0[J::Jjffne as usize] = "jffne";
        jtoa0[J::Jjffo as usize] = "jffo";
        jtoa0[J::Jjffuo as usize] = "jffuo";
        jtoa0[J::Jhlt as usize] = "hlt";

        jtoa0
    };
    let _ = writeln!(f, "function ${}() {{", to_s(&fn_.name));
    let mut bi: BlkIdx = fn_.start;
    while bi != BlkIdx::INVALID {
        let b: &Blk = fn_.blk(bi);
        let _ = writeln!(f, "@{}", to_s(&b.name));
        let mut pi: PhiIdx = b.phi;
        while pi != PhiIdx::INVALID {
            let p: &Phi = &fn_.phis[pi.0];
            let _ = write!(f, "\t");
            printref(f, fn_, typ, itbl, &p.to);
            let _ = write!(f, " ={} phi ", KTOC[p.cls as usize]);
            assert!(!p.arg.is_empty());
            assert!(p.arg.len() == p.blk.len());
            for n in 0..p.arg.len() {
                let bi: BlkIdx = p.blk[n];
                let pb = fn_.blk(bi);
                let _ = write!(f, "@{} ", to_s(&pb.name));
                printref(f, fn_, typ, itbl, &p.arg[n]);
                if n != p.arg.len() - 1 {
                    let _ = write!(f, ", ");
                }
            }
            let _ = writeln!(f);
            pi = p.link;
        }
        for i in &b.ins {
            let _ = write!(f, "\t");
            if i.to != Ref::R {
                printref(f, fn_, typ, itbl, &i.to);
                let _ = write!(f, " ={} ", KTOC[i.cls as usize]);
            }
            assert!(OPTAB[i.op as usize].name.len() != 0);
            let _ = write!(f, "{}", to_s(&OPTAB[i.op as usize].name));
            if i.to == Ref::R {
                match i.op {
                    O::Oarg
                    | O::Oswap
                    | O::Oxcmp
                    | O::Oacmp
                    | O::Oacmn
                    | O::Oafcmp
                    | O::Oxtest
                    | O::Oxdiv
                    | O::Oxidiv => {
                        let _ = write!(f, "{}", KTOC[i.cls as usize]);
                    }
                    _ => {} // nada
                }
            }
            if i.arg[0] != Ref::R {
                let _ = write!(f, " ");
                printref(f, fn_, typ, itbl, &i.arg[0]);
            }
            if i.arg[1] != Ref::R {
                let _ = write!(f, ", ");
                printref(f, fn_, typ, itbl, &i.arg[1]);
            }
            let _ = writeln!(f);
        }
        match b.jmp.type_ {
            J::Jret0
            | J::Jretsb
            | J::Jretub
            | J::Jretsh
            | J::Jretuh
            | J::Jretw
            | J::Jretl
            | J::Jrets
            | J::Jretd
            | J::Jretc => {
                let _ = write!(f, "\t{}", JTOA[b.jmp.type_ as usize]);
                if b.jmp.type_ != J::Jret0 || b.jmp.arg != Ref::R {
                    let _ = write!(f, " ");
                    printref(f, fn_, typ, itbl, &b.jmp.arg);
                }
                if b.jmp.type_ == J::Jretc {
                    let _ = write!(f, ", :{}", to_s(&typ[fn_.retty.0].name));
                }
            }
            J::Jhlt => {
                let _ = write!(f, "\thlt");
            }
            J::Jjmp => {
                if b.s1 != b.link {
                    let _ = write!(f, "\tjmp @{}", to_s(&fn_.blk(b.s1).name));
                }
            }
            _ => {
                let _ = write!(f, "\t{} ", JTOA[b.jmp.type_ as usize]);
                if b.jmp.type_ == J::Jjnz {
                    printref(f, fn_, typ, itbl, &b.jmp.arg);
                    let _ = write!(f, ", ");
                }
                assert!(b.s1 != BlkIdx::INVALID && b.s2 != BlkIdx::INVALID);
                let _ = writeln!(
                    f,
                    "@%{}, @%{}",
                    to_s(&fn_.blk(b.s1).name),
                    to_s(&fn_.blk(b.s2).name)
                );
            }
        }
        let _ = writeln!(f);
        bi = b.link;
    }
    let _ = writeln!(f, "}}");
}
