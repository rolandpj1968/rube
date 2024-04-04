use crate::all::{KExt, Op, KD, KE, KL, KM, KS, KW, KX, O};

pub static OPTAB: [Op; O::NOp as usize] = {
    let nullop = Op::new(b"", [[KE, KE, KE, KE], [KE, KE, KE, KE]], false);
    let mut optab0 = [nullop; O::NOp as usize];

    // Generated from QBE with gcc -E and then hand-munged
    optab0[O::Oadd as usize] = Op::new(
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
    optab0[O::Osub as usize] = Op::new(
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
    optab0[O::Oneg as usize] = Op::new(
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
    optab0[O::Odiv as usize] = Op::new(
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
    optab0[O::Orem as usize] = Op::new(
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
    optab0[O::Oudiv as usize] = Op::new(
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
    optab0[O::Ourem as usize] = Op::new(
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
    optab0[O::Omul as usize] = Op::new(
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
    optab0[O::Oand as usize] = Op::new(
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
    optab0[O::Oor as usize] = Op::new(
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
    optab0[O::Oxor as usize] = Op::new(
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
    optab0[O::Osar as usize] = Op::new(
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
    optab0[O::Oshr as usize] = Op::new(
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
    optab0[O::Oshl as usize] = Op::new(
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

    optab0[O::Oceqw as usize] = Op::new(
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
    optab0[O::Ocnew as usize] = Op::new(
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
    optab0[O::Ocsgew as usize] = Op::new(
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
    optab0[O::Ocsgtw as usize] = Op::new(
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
    optab0[O::Ocslew as usize] = Op::new(
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
    optab0[O::Ocsltw as usize] = Op::new(
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
    optab0[O::Ocugew as usize] = Op::new(
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
    optab0[O::Ocugtw as usize] = Op::new(
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
    optab0[O::Oculew as usize] = Op::new(
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
    optab0[O::Ocultw as usize] = Op::new(
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

    optab0[O::Oceql as usize] = Op::new(
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
    optab0[O::Ocnel as usize] = Op::new(
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
    optab0[O::Ocsgel as usize] = Op::new(
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
    optab0[O::Ocsgtl as usize] = Op::new(
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
    optab0[O::Ocslel as usize] = Op::new(
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
    optab0[O::Ocsltl as usize] = Op::new(
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
    optab0[O::Ocugel as usize] = Op::new(
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
    optab0[O::Ocugtl as usize] = Op::new(
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
    optab0[O::Oculel as usize] = Op::new(
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
    optab0[O::Ocultl as usize] = Op::new(
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

    optab0[O::Oceqs as usize] = Op::new(
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
    optab0[O::Ocges as usize] = Op::new(
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
    optab0[O::Ocgts as usize] = Op::new(
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
    optab0[O::Ocles as usize] = Op::new(
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
    optab0[O::Oclts as usize] = Op::new(
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
    optab0[O::Ocnes as usize] = Op::new(
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
    optab0[O::Ocos as usize] = Op::new(
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
    optab0[O::Ocuos as usize] = Op::new(
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

    optab0[O::Oceqd as usize] = Op::new(
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
    optab0[O::Ocged as usize] = Op::new(
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
    optab0[O::Ocgtd as usize] = Op::new(
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
    optab0[O::Ocled as usize] = Op::new(
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
    optab0[O::Ocltd as usize] = Op::new(
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
    optab0[O::Ocned as usize] = Op::new(
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
    optab0[O::Ocod as usize] = Op::new(
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
    optab0[O::Ocuod as usize] = Op::new(
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

    optab0[O::Ostoreb as usize] = Op::new(
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
    optab0[O::Ostoreh as usize] = Op::new(
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
    optab0[O::Ostorew as usize] = Op::new(
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
    optab0[O::Ostorel as usize] = Op::new(
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
    optab0[O::Ostores as usize] = Op::new(
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
    optab0[O::Ostored as usize] = Op::new(
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

    optab0[O::Oloadsb as usize] = Op::new(
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
    optab0[O::Oloadub as usize] = Op::new(
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
    optab0[O::Oloadsh as usize] = Op::new(
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
    optab0[O::Oloaduh as usize] = Op::new(
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
    optab0[O::Oloadsw as usize] = Op::new(
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
    optab0[O::Oloaduw as usize] = Op::new(
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
    optab0[O::Oload as usize] = Op::new(
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

    optab0[O::Oextsb as usize] = Op::new(
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
    optab0[O::Oextub as usize] = Op::new(
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
    optab0[O::Oextsh as usize] = Op::new(
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
    optab0[O::Oextuh as usize] = Op::new(
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
    optab0[O::Oextsw as usize] = Op::new(
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
    optab0[O::Oextuw as usize] = Op::new(
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

    optab0[O::Oexts as usize] = Op::new(
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
    optab0[O::Otruncd as usize] = Op::new(
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
    optab0[O::Ostosi as usize] = Op::new(
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
    optab0[O::Ostoui as usize] = Op::new(
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
    optab0[O::Odtosi as usize] = Op::new(
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
    optab0[O::Odtoui as usize] = Op::new(
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
    optab0[O::Oswtof as usize] = Op::new(
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
    optab0[O::Ouwtof as usize] = Op::new(
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
    optab0[O::Osltof as usize] = Op::new(
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
    optab0[O::Oultof as usize] = Op::new(
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
    optab0[O::Ocast as usize] = Op::new(
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

    optab0[O::Oalloc4 as usize] = Op::new(
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
    optab0[O::Oalloc8 as usize] = Op::new(
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
    optab0[O::Oalloc16 as usize] = Op::new(
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

    optab0[O::Ovaarg as usize] = Op::new(
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
    optab0[O::Ovastart as usize] = Op::new(
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

    optab0[O::Ocopy as usize] = Op::new(
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

    optab0[O::Odbgloc as usize] = Op::new(
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

    optab0[O::Onop as usize] = Op::new(
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
    optab0[O::Oaddr as usize] = Op::new(
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
    optab0[O::Oblit0 as usize] = Op::new(
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
    optab0[O::Oblit1 as usize] = Op::new(
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
    optab0[O::Oswap as usize] = Op::new(
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
    optab0[O::Osign as usize] = Op::new(
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
    optab0[O::Osalloc as usize] = Op::new(
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
    optab0[O::Oxidiv as usize] = Op::new(
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
    optab0[O::Oxdiv as usize] = Op::new(
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
    optab0[O::Oxcmp as usize] = Op::new(
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
    optab0[O::Oxtest as usize] = Op::new(
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
    optab0[O::Oacmp as usize] = Op::new(
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
    optab0[O::Oacmn as usize] = Op::new(
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
    optab0[O::Oafcmp as usize] = Op::new(
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
    optab0[O::Oreqz as usize] = Op::new(
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
    optab0[O::Ornez as usize] = Op::new(
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

    optab0[O::Opar as usize] = Op::new(
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
    optab0[O::Oparsb as usize] = Op::new(
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
    optab0[O::Oparub as usize] = Op::new(
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
    optab0[O::Oparsh as usize] = Op::new(
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
    optab0[O::Oparuh as usize] = Op::new(
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
    optab0[O::Oparc as usize] = Op::new(
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
    optab0[O::Opare as usize] = Op::new(
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
    optab0[O::Oarg as usize] = Op::new(
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
    optab0[O::Oargsb as usize] = Op::new(
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
    optab0[O::Oargub as usize] = Op::new(
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
    optab0[O::Oargsh as usize] = Op::new(
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
    optab0[O::Oarguh as usize] = Op::new(
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
    optab0[O::Oargc as usize] = Op::new(
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
    optab0[O::Oarge as usize] = Op::new(
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
    optab0[O::Oargv as usize] = Op::new(
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
    optab0[O::Ocall as usize] = Op::new(
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

    optab0[O::Oflagieq as usize] = Op::new(
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
    optab0[O::Oflagine as usize] = Op::new(
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
    optab0[O::Oflagisge as usize] = Op::new(
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
    optab0[O::Oflagisgt as usize] = Op::new(
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
    optab0[O::Oflagisle as usize] = Op::new(
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
    optab0[O::Oflagislt as usize] = Op::new(
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
    optab0[O::Oflagiuge as usize] = Op::new(
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
    optab0[O::Oflagiugt as usize] = Op::new(
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
    optab0[O::Oflagiule as usize] = Op::new(
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
    optab0[O::Oflagiult as usize] = Op::new(
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
    optab0[O::Oflagfeq as usize] = Op::new(
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
    optab0[O::Oflagfge as usize] = Op::new(
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
    optab0[O::Oflagfgt as usize] = Op::new(
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
    optab0[O::Oflagfle as usize] = Op::new(
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
    optab0[O::Oflagflt as usize] = Op::new(
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
    optab0[O::Oflagfne as usize] = Op::new(
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
    optab0[O::Oflagfo as usize] = Op::new(
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
    optab0[O::Oflagfuo as usize] = Op::new(
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
