use crate::all::RXX;

/*
#include "../all.h"

typedef struct Amd64Op Amd64Op;

*/
#[repr(u8)]
pub enum Amd64Reg {
    RAX = (RXX as u8) + 1, /* caller-save */
    RCX,
    RDX,
    RSI,
    RDI,
    R8,
    R9,
    R10,
    R11,

    RBX, /* callee-save */
    R12,
    R13,
    R14,
    R15,

    RBP, /* globally live */
    RSP,

    XMM0, /* sse */
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
    XMM8,
    XMM9,
    XMM10,
    XMM11,
    XMM12,
    XMM13,
    XMM14,
    XMM15,
}
//MAKESURE(reg_not_tmp, XMM15 < (int)Tmp0);

pub const NFPR: u32 = (Amd64Reg::XMM14 as u32) - (Amd64Reg::XMM0 as u32) + 1; /* reserve XMM15 */
pub const NGPR: u32 = (Amd64Reg::RSP as u32) - (Amd64Reg::RAX as u32) + 1;
pub const NGPS: u32 = (Amd64Reg::R11 as u32) - (Amd64Reg::RAX as u32) + 1;
pub const NFPS: u32 = NFPR;
pub const NCLR: u32 = (Amd64Reg::R15 as u32) - (Amd64Reg::RBX as u32) + 1;

/*

struct Amd64Op {
    char nmem;
    char zflag;
    char lflag;
};

/* targ.c */
extern Amd64Op amd64_op[];

/* sysv.c (abi) */
extern int amd64_sysv_rsave[];
extern int amd64_sysv_rclob[];
bits amd64_sysv_retregs(Ref, int[2]);
bits amd64_sysv_argregs(Ref, int[2]);
void amd64_sysv_abi(Fn *);

/* isel.c */
void amd64_isel(Fn *);

/* emit.c */
void amd64_emitfn(Fn *, FILE *);
 */
