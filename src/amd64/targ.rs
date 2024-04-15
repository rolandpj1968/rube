// TODO remove eventually
#![allow(dead_code, unused_variables)]

use crate::abi::elimsb;
use crate::all::{bit, Bits, Fn, Ref, Target};
use crate::amd64::all::{Amd64Reg, NFPR, NFPS, NGPR, NGPS};
use crate::amd64::sysv::AMD64_SYSV_RSAVE;

/*
#include "all.h"

Amd64Op amd64_op[NOp] = {
#define O(op, t, x) [O##op] =
#define X(nm, zf, lf) { nm, zf, lf, },
    #include "../ops.h"
};

static int
amd64_memargs(int op)
{
    return amd64_op[op].nmem;
}
 */

fn abi1_dummy(_: &mut Fn) {
    panic!("Implement me");
}

fn retregs_dummy(_: Ref, _: &[u32; 2]) -> Bits {
    panic!("Implement me");
}

fn argregs_dummy(_: Ref, _: &[u32; 2]) -> Bits {
    panic!("Implement me");
}

fn memargs_dummy(_: u32) -> u32 {
    panic!("Implement me");
}

fn isel_dummy(_: &mut Fn) {
    panic!("Implement me");
}

fn emitfn_dummy(_: &Fn /*, FILE **/) {
    panic!("Implement me");
}

fn emitfin_dummy(/*FILE **/) {
    panic!("Implement me");
}

pub static T_AMD64_SYSV: Target = Target {
    name: b"amd64_sysv",
    apple: false,
    emitfin: /*elf_emitfin*/emitfin_dummy, // TODO
    asloc: b".L",
    assym: b"",
    gpr0: Amd64Reg::RAX as u32,
    ngpr: NGPR,
    fpr0: Amd64Reg::XMM0 as u32,
    nfpr: NFPR,
    rglob: bit(Amd64Reg::RBP as u32) | bit(Amd64Reg::RSP as u32),
    nrglob: 2,
    rsave: AMD64_SYSV_RSAVE,
    nrsave: [NGPS, NFPS],
    retregs: /*amd64_sysv_retregs*/retregs_dummy, // TODO
    argregs: /*amd64_sysv_argregs*/ argregs_dummy, // TODO
    memargs: /*amd64_memargs*/memargs_dummy, // TODO
    abi0: elimsb,
    abi1: /*amd64_sysv_abi*/abi1_dummy, // TODO
    isel: /*amd64_isel*/isel_dummy, // TODO
    emitfn: /*amd64_emitfn*/emitfn_dummy, // TODO
};

pub static T_AMD64_APPLE: Target = Target {
    name: b"amd64_apple",
    apple: true,
    emitfin: /*macho_emitfin*/emitfin_dummy, // TODO
    asloc: b"L",
    assym: b"_",
    gpr0: Amd64Reg::RAX as u32,
    ngpr: NGPR,
    fpr0: Amd64Reg::XMM0 as u32,
    nfpr: NFPR,
    rglob: bit(Amd64Reg::RBP as u32) | bit(Amd64Reg::RSP as u32),
    nrglob: 2,
    rsave: AMD64_SYSV_RSAVE,
    nrsave: [NGPS, NFPS],
    retregs: /*amd64_sysv_retregs*/retregs_dummy, // TODO
    argregs: /*amd64_sysv_argregs*/argregs_dummy, // TODO
    memargs: /*amd64_memargs*/memargs_dummy, // TODO
    abi0: elimsb,
    abi1: /*amd64_sysv_abi*/abi1_dummy, // TODO
    isel: /*amd64_isel*/isel_dummy, // TODO
    emitfn: /*amd64_emitfn*/emitfn_dummy, // TODO
};
