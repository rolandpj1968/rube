use crate::all::{KExt, Op, KD, KE, KL, KM, KS, KW, KX, O};

// TODO Op::new() instead
const fn mkop(name: &'static [u8], argcls: [[KExt; 4]; 2], canfold: bool) -> Op {
    Op {
        name,
        argcls,
        canfold,
    }
}

pub static OPTAB: [Op; O::NOp as usize] = {
    let nullop = mkop(b"", [[KE, KE, KE, KE], [KE, KE, KE, KE]], false);
    let mut optab0 = [nullop; O::NOp as usize];

    // Generated from QBE with gcc -E and then hand-munged
    optab0[O::Oadd as usize] = mkop(
        b"add",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
        ],
        true,
    );
    optab0[O::Osub as usize] = mkop(
        b"sub",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
        ],
        true,
    );
    optab0[O::Oneg as usize] = mkop(
        b"neg",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        true,
    );
    optab0[O::Odiv as usize] = mkop(
        b"div",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
        ],
        true,
    );
    optab0[O::Orem as usize] = mkop(
        b"rem",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oudiv as usize] = mkop(
        b"udiv",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ourem as usize] = mkop(
        b"urem",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Omul as usize] = mkop(
        b"mul",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
        ],
        true,
    );
    optab0[O::Oand as usize] = mkop(
        b"and",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oor as usize] = mkop(
        b"or",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oxor as usize] = mkop(
        b"xor",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Osar as usize] = mkop(
        b"sar",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oshr as usize] = mkop(
        b"shr",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oshl as usize] = mkop(
        b"shl",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );

    optab0[O::Oceqw as usize] = mkop(
        b"ceqw",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocnew as usize] = mkop(
        b"cnew",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocsgew as usize] = mkop(
        b"csgew",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocsgtw as usize] = mkop(
        b"csgtw",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocslew as usize] = mkop(
        b"cslew",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocsltw as usize] = mkop(
        b"csltw",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocugew as usize] = mkop(
        b"cugew",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocugtw as usize] = mkop(
        b"cugtw",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oculew as usize] = mkop(
        b"culew",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocultw as usize] = mkop(
        b"cultw",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );

    optab0[O::Oceql as usize] = mkop(
        b"ceql",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocnel as usize] = mkop(
        b"cnel",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocsgel as usize] = mkop(
        b"csgel",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocsgtl as usize] = mkop(
        b"csgtl",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocslel as usize] = mkop(
        b"cslel",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocsltl as usize] = mkop(
        b"csltl",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocugel as usize] = mkop(
        b"cugel",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocugtl as usize] = mkop(
        b"cugtl",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oculel as usize] = mkop(
        b"culel",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocultl as usize] = mkop(
        b"cultl",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );

    optab0[O::Oceqs as usize] = mkop(
        b"ceqs",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocges as usize] = mkop(
        b"cges",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocgts as usize] = mkop(
        b"cgts",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocles as usize] = mkop(
        b"cles",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oclts as usize] = mkop(
        b"clts",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocnes as usize] = mkop(
        b"cnes",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocos as usize] = mkop(
        b"cos",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocuos as usize] = mkop(
        b"cuos",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );

    optab0[O::Oceqd as usize] = mkop(
        b"ceqd",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocged as usize] = mkop(
        b"cged",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocgtd as usize] = mkop(
        b"cgtd",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocled as usize] = mkop(
        b"cled",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocltd as usize] = mkop(
        b"cltd",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocned as usize] = mkop(
        b"cned",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocod as usize] = mkop(
        b"cod",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ocuod as usize] = mkop(
        b"cuod",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );

    optab0[O::Ostoreb as usize] = mkop(
        b"storeb",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Ostoreh as usize] = mkop(
        b"storeh",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Ostorew as usize] = mkop(
        b"storew",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Ostorel as usize] = mkop(
        b"storel",
        [
            [
                /*[Kw]=*/ KL, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Ostores as usize] = mkop(
        b"stores",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Ostored as usize] = mkop(
        b"stored",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );

    optab0[O::Oloadsb as usize] = mkop(
        b"loadsb",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KM, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oloadub as usize] = mkop(
        b"loadub",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KM, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oloadsh as usize] = mkop(
        b"loadsh",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KM, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oloaduh as usize] = mkop(
        b"loaduh",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KM, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oloadsw as usize] = mkop(
        b"loadsw",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KM, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oloaduw as usize] = mkop(
        b"loaduw",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KM, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oload as usize] = mkop(
        b"load",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KM, /*[Ks]=*/ KM, /*[Kd]=*/ KM,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );

    optab0[O::Oextsb as usize] = mkop(
        b"extsb",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oextub as usize] = mkop(
        b"extub",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oextsh as usize] = mkop(
        b"extsh",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oextuh as usize] = mkop(
        b"extuh",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oextsw as usize] = mkop(
        b"extsw",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oextuw as usize] = mkop(
        b"extuw",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KW, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );

    optab0[O::Oexts as usize] = mkop(
        b"exts",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KS,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KX,
            ],
        ],
        true,
    );
    optab0[O::Otruncd as usize] = mkop(
        b"truncd",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KD, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KX, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ostosi as usize] = mkop(
        b"stosi",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Ostoui as usize] = mkop(
        b"stoui",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KS, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Odtosi as usize] = mkop(
        b"dtosi",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Odtoui as usize] = mkop(
        b"dtoui",
        [
            [
                /*[Kw]=*/ KD, /*[Kl]=*/ KD, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        true,
    );
    optab0[O::Oswtof as usize] = mkop(
        b"swtof",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KW, /*[Kd]=*/ KW,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        true,
    );
    optab0[O::Ouwtof as usize] = mkop(
        b"uwtof",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KW, /*[Kd]=*/ KW,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        true,
    );
    optab0[O::Osltof as usize] = mkop(
        b"sltof",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KL, /*[Kd]=*/ KL,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        true,
    );
    optab0[O::Oultof as usize] = mkop(
        b"ultof",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KL, /*[Kd]=*/ KL,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        true,
    );
    optab0[O::Ocast as usize] = mkop(
        b"cast",
        [
            [
                /*[Kw]=*/ KS, /*[Kl]=*/ KD, /*[Ks]=*/ KW, /*[Kd]=*/ KL,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        true,
    );

    optab0[O::Oalloc4 as usize] = mkop(
        b"alloc4",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oalloc8 as usize] = mkop(
        b"alloc8",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oalloc16 as usize] = mkop(
        b"alloc16",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );

    optab0[O::Ovaarg as usize] = mkop(
        b"vaarg",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KM, /*[Ks]=*/ KM, /*[Kd]=*/ KM,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Ovastart as usize] = mkop(
        b"vastart",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );

    optab0[O::Ocopy as usize] = mkop(
        b"copy",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );

    optab0[O::Odbgloc as usize] = mkop(
        b"dbgloc",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );

    optab0[O::Onop as usize] = mkop(
        b"nop",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oaddr as usize] = mkop(
        b"addr",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KM, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oblit0 as usize] = mkop(
        b"blit0",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oblit1 as usize] = mkop(
        b"blit1",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oswap as usize] = mkop(
        b"swap",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
        ],
        false,
    );
    optab0[O::Osign as usize] = mkop(
        b"sign",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Osalloc as usize] = mkop(
        b"salloc",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oxidiv as usize] = mkop(
        b"xidiv",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oxdiv as usize] = mkop(
        b"xdiv",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oxcmp as usize] = mkop(
        b"xcmp",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
        ],
        false,
    );
    optab0[O::Oxtest as usize] = mkop(
        b"xtest",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oacmp as usize] = mkop(
        b"acmp",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oacmn as usize] = mkop(
        b"acmn",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oafcmp as usize] = mkop(
        b"afcmp",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KE, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
        ],
        false,
    );
    optab0[O::Oreqz as usize] = mkop(
        b"reqz",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Ornez as usize] = mkop(
        b"rnez",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );

    optab0[O::Opar as usize] = mkop(
        b"par",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oparsb as usize] = mkop(
        b"parsb",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oparub as usize] = mkop(
        b"parub",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oparsh as usize] = mkop(
        b"parsh",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oparuh as usize] = mkop(
        b"paruh",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oparc as usize] = mkop(
        b"parc",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Opare as usize] = mkop(
        b"pare",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oarg as usize] = mkop(
        b"arg",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KL, /*[Ks]=*/ KS, /*[Kd]=*/ KD,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oargsb as usize] = mkop(
        b"argsb",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oargub as usize] = mkop(
        b"argub",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oargsh as usize] = mkop(
        b"argsh",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oarguh as usize] = mkop(
        b"arguh",
        [
            [
                /*[Kw]=*/ KW, /*[Kl]=*/ KE, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Oargc as usize] = mkop(
        b"argc",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oarge as usize] = mkop(
        b"arge",
        [
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KL, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KE, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oargv as usize] = mkop(
        b"argv",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );
    optab0[O::Ocall as usize] = mkop(
        b"call",
        [
            [
                /*[Kw]=*/ KM, /*[Kl]=*/ KM, /*[Ks]=*/ KM, /*[Kd]=*/ KM,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KX, /*[Kd]=*/ KX,
            ],
        ],
        false,
    );

    optab0[O::Oflagieq as usize] = mkop(
        b"flagieq",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagine as usize] = mkop(
        b"flagine",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagisge as usize] = mkop(
        b"flagisge",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagisgt as usize] = mkop(
        b"flagisgt",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagisle as usize] = mkop(
        b"flagisle",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagislt as usize] = mkop(
        b"flagislt",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagiuge as usize] = mkop(
        b"flagiuge",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagiugt as usize] = mkop(
        b"flagiugt",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagiule as usize] = mkop(
        b"flagiule",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagiult as usize] = mkop(
        b"flagiult",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagfeq as usize] = mkop(
        b"flagfeq",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagfge as usize] = mkop(
        b"flagfge",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagfgt as usize] = mkop(
        b"flagfgt",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagfle as usize] = mkop(
        b"flagfle",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagflt as usize] = mkop(
        b"flagflt",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagfne as usize] = mkop(
        b"flagfne",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagfo as usize] = mkop(
        b"flagfo",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );
    optab0[O::Oflagfuo as usize] = mkop(
        b"flagfuo",
        [
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
            [
                /*[Kw]=*/ KX, /*[Kl]=*/ KX, /*[Ks]=*/ KE, /*[Kd]=*/ KE,
            ],
        ],
        false,
    );

    optab0
};
