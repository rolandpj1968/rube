//#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate static_assertions;

mod abi;
mod alias;
mod all;
mod amd64;
mod cfg;
mod live;
mod load;
mod mem;
mod optab;
mod parse;
mod ssa;
mod util;

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::stdout;
use std::path::Path;

use abi::elimsb;
use alias::fillalias;
use all::{Dat, Fn, Target, Typ};
use amd64::targ::T_AMD64_SYSV;
use cfg::fillrpo;
use load::loadopt;
use mem::promote;
use parse::{parse, printfn};
use ssa::{filluse, ssa};
use util::Bucket;

use crate::ssa::ssacheck;

fn dump_dbgfile(name: &[u8]) {
    println!("Got dbgfile {:?}", String::from_utf8_lossy(name));
}

fn dump_data(dat: &Dat, _typ: &[Typ]) {
    println!(
        "Got dat {:?} {:?}",
        String::from_utf8_lossy(&dat.name),
        dat.type_
    );
}

fn dump_func(f: &mut Fn, targ: &Target, typ: &[Typ], itbl: &[Bucket]) {
    println!("Got fn {:?}:", String::from_utf8_lossy(&f.name));
    println!();
    elimsb(f); // TODO targ.abi0()
    fillrpo(f);
    filluse(f);
    promote(f).unwrap();
    filluse(f);
    ssa(f, targ, typ, itbl).unwrap();
    filluse(f);
    ssacheck(f).unwrap();
    fillalias(f);
    loadopt(f);

    printfn(&mut stdout(), f, typ, itbl);
}

fn main() {
    let args: Vec<OsString> = env::args_os().collect();
    if args.len() != 2 {
        eprintln!("usage: {:?} <infile>", args[0]);
        std::process::exit(1);
    }
    let path_osstr = args[1].clone();
    let path: &Path = Path::new(&path_osstr);
    let infile = File::open(args[1].clone()).unwrap();

    match parse(
        &T_AMD64_SYSV,
        &infile,
        path,
        dump_dbgfile,
        dump_data,
        dump_func,
    ) {
        Ok(()) => println!("Finished parsing"),
        Err(e) => {
            eprintln!("Error parsing {:?} - {:?}", path, e);
            std::process::exit(1);
        }
    }
}
