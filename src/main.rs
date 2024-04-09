//#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate static_assertions;

mod all;
mod optab;
mod parse;
mod util;

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::stdout;
use std::path::Path;

use all::{Bits, Dat, Fn, Ref, Target, Typ};
use parse::{parse, printfn};

// Target T_amd64_sysv = {
// 	.name = "amd64_sysv",
// 	.emitfin = elf_emitfin,
// 	.asloc = ".L",
// 	.gpr0 = RAX, \
// 	.ngpr = NGPR, \
// 	.fpr0 = XMM0, \
// 	.nfpr = NFPR, \
// 	.rglob = BIT(RBP) | BIT(RSP), \
// 	.nrglob = 2, \
// 	.rsave = amd64_sysv_rsave, \
// 	.nrsave = {NGPS, NFPS}, \
// 	.retregs = amd64_sysv_retregs, \
// 	.argregs = amd64_sysv_argregs, \
// 	.memargs = amd64_memargs, \
// 	.abi0 = elimsb, \
// 	.abi1 = amd64_sysv_abi, \
// 	.isel = amd64_isel, \
// 	.emitfn = amd64_emitfn, \
// };

fn dummy_retregs(r: Ref, something: [i32; 2]) -> Bits { 0 }
fn dummy_argregs(r: Ref, something: [i32; 2]) -> Bits { 0 }
fn dummy_memargs(something: i32) -> i32 { 0 }
fn dummy_abi0(fn_: &mut Fn) {}
fn dummy_abi1(fn_: &mut Fn) {}
fn dummy_isel(fn_: &mut Fn) {}
fn dummy_emitfn(fn_: & Fn) {}
fn dummy_emitfin() {}

static amd64_sysv: Target = Target {
    name: b"amd64_sysv",
    apple: false,
    gpr0: 1, //i32, // first general purpose reg
    ngpr: 16, //i32,
    fpr0: 17, //i32, // first floating point reg
    nfpr: 15, //i32,
    rglob: 0, // not right but not needed for parser // Bits, // globally live regs (e.g., sp, fp)
    nrglob: 0, // not right but not needed for parser // i32,
    rsave: vec![], // not right but not needed for parser // Vec<i32>, // caller-save [Vec???]
    nrsave: [9, 15], // [i32; 2],
    retregs: dummy_retregs, // fn(Ref, [i32; 2]) -> Bits,
    argregs: dummy_argregs, // fn(Ref, [i32; 2]) -> Bits,
    memargs: dummy_memargs, // fn(i32) -> i32,
    abi0: dummy_abi0, // fn(&mut Fn),
    abi1: dummy_abi1, // fn(&mut Fn),
    isel: dummy_isel, // fn(&mut Fn),
    emitfn: dummy_emitfn, // fn(&Fn /*, FILE **/), // TODO
    emitfin: dummy_emitfin, // (/*FILE **/),      // TODO
    asloc: b".L", // &'static [u8],
    assym: b"", //&'static [u8],
};

// pub fn parse(
//     T: &Target,
//     f: &File,
//     path: &Path,
//     dbgfile: fn(&[u8]) -> (),
//     data: fn(&Dat) -> (),
//     func: fn(&Fn) -> (),
// )

fn dump_dbgfile(name: &[u8]) {
    println!("Got dbgfile {:?}", String::from_utf8_lossy(name));
}

fn dump_data(dat: &Dat, typ: &[Typ]) {
    println!("Got dat {:?} {:?}", String::from_utf8_lossy(&dat.name), dat.type_);
}

fn dump_func(fn_: &Fn, typ: &[Typ]) {
    println!("Got fn {:?}:", String::from_utf8_lossy(&fn_.name));
    println!();
    printfn(&mut stdout(), fn_, typ);
}

fn main() {
    println!("Hello, world!");
    let args: Vec<OsString> = env::args_os().collect();
    dbg!(args.clone());
    if args.len() != 2 {
	eprintln!("usage: {:?} <infile>", args[0]);
	std::process::exit(1);
    }
    let path_osstr = args[1].clone();
    let path: &Path = Path::new(&path_osstr);
    let infile = File::open(args[1].clone()).unwrap();

    match parse(&amd64_sysv, &infile, path, dump_dbgfile, dump_data, dump_func) {
	Ok(()) => println!("Finished parsing"),
	Err(e) => {
	    eprintln!("Error parsing {:?} - {:?}", path, e);
	    std::process::exit(1);
	}
    }
}
