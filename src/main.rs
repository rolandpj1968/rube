//#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate static_assertions;

#[macro_use]
mod macros;

mod abi;
mod alias;
mod all;
mod amd64;
mod cfg;
mod copy;
mod fold;
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

use alias::fillalias;
use all::{Dat, Fn, Target, Typ};
use amd64::targ::T_AMD64_SYSV;
use cfg::fillrpo;
use copy::copy;
use fold::fold;
use load::loadopt;
use mem::{coalesce, promote};
use parse::{parse, printfn};
use ssa::{filluse, ssa};
use util::Bucket;

use crate::ssa::ssacheck;

fn dump_dbgfile(name: &[u8]) {
    println!("Got dbgfile {:?}", String::from_utf8_lossy(name));
}

fn dump_data(dat: &Dat, _typ: &[Typ]) {
    println!(
        "Got dat \"{}\" {:?}",
        String::from_utf8_lossy(&dat.name),
        dat.typ
    );
}

fn dump_func(f: &mut Fn, targ: &Target, typ: &[Typ], itbl: &[Bucket]) {
    println!("Got fn \"{}\":", String::from_utf8_lossy(&f.name));
    println!();
    (targ.abi0)(f);
    fillrpo(f);
    filluse(f);
    promote(f, typ, itbl).unwrap();
    filluse(f);
    ssa(f, targ, typ, itbl).unwrap();
    filluse(f);
    ssacheck(f).unwrap();
    fillalias(f);
    loadopt(f, typ, itbl);
    filluse(f);
    fillalias(f);
    coalesce(f, typ, itbl);
    filluse(f);
    ssacheck(f).unwrap();
    copy(f, typ, itbl);
    filluse(f);
    fold(f, typ, itbl);

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

// use std::thread;

// fn main_to_thread() {
//     const STACK_SIZE: usize = 32 * 1024 * 1024;

//     // Spawn thread with explicit stack size
//     let child = thread::Builder::new()
//         .stack_size(STACK_SIZE)
//         .spawn(main_real)
//         .unwrap();

//     // Wait for thread to join
//     child.join().unwrap();
// }
