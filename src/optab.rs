use crate::all::{KExt, Kd, Ke, Kl, Km, Ks, Kw, Kx, Op, O};

const fn mkop(name: &'static [u8], argcls: [[KExt; 4]; 2], canfold: bool) -> Op {
    Op {
        name: name,
        argcls: argcls,
        canfold: canfold,
    }
}

pub static optab: [Op; O::NOp as usize] = {
    let nullop = mkop(b"", [[Ke, Ke, Ke, Ke], [Ke, Ke, Ke, Ke]], false);
    let mut optab0 = [nullop; O::NOp as usize];

    // Generated from QBE with gcc -E and then hand-munged
    optab0[O::Oadd as usize] = mkop(
        b"add",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
        ],
        true,
    );
    optab0[O::Osub as usize] = mkop(
        b"sub",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
        ],
        true,
    );
    optab0[O::Oneg as usize] = mkop(
        b"neg",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        true,
    );
    optab0[O::Odiv as usize] = mkop(
        b"div",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
        ],
        true,
    );
    optab0[O::Orem as usize] = mkop(
        b"rem",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oudiv as usize] = mkop(
        b"udiv",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ourem as usize] = mkop(
        b"urem",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Omul as usize] = mkop(
        b"mul",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
        ],
        true,
    );
    optab0[O::Oand as usize] = mkop(
        b"and",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oor as usize] = mkop(
        b"or",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oxor as usize] = mkop(
        b"xor",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Osar as usize] = mkop(
        b"sar",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oshr as usize] = mkop(
        b"shr",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oshl as usize] = mkop(
        b"shl",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );

    optab0[O::Oceqw as usize] = mkop(
        b"ceqw",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocnew as usize] = mkop(
        b"cnew",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocsgew as usize] = mkop(
        b"csgew",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocsgtw as usize] = mkop(
        b"csgtw",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocslew as usize] = mkop(
        b"cslew",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocsltw as usize] = mkop(
        b"csltw",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocugew as usize] = mkop(
        b"cugew",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocugtw as usize] = mkop(
        b"cugtw",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oculew as usize] = mkop(
        b"culew",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocultw as usize] = mkop(
        b"cultw",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );

    optab0[O::Oceql as usize] = mkop(
        b"ceql",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocnel as usize] = mkop(
        b"cnel",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocsgel as usize] = mkop(
        b"csgel",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocsgtl as usize] = mkop(
        b"csgtl",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocslel as usize] = mkop(
        b"cslel",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocsltl as usize] = mkop(
        b"csltl",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocugel as usize] = mkop(
        b"cugel",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocugtl as usize] = mkop(
        b"cugtl",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oculel as usize] = mkop(
        b"culel",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocultl as usize] = mkop(
        b"cultl",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );

    optab0[O::Oceqs as usize] = mkop(
        b"ceqs",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocges as usize] = mkop(
        b"cges",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocgts as usize] = mkop(
        b"cgts",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocles as usize] = mkop(
        b"cles",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oclts as usize] = mkop(
        b"clts",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocnes as usize] = mkop(
        b"cnes",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocos as usize] = mkop(
        b"cos",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocuos as usize] = mkop(
        b"cuos",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );

    optab0[O::Oceqd as usize] = mkop(
        b"ceqd",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocged as usize] = mkop(
        b"cged",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocgtd as usize] = mkop(
        b"cgtd",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocled as usize] = mkop(
        b"cled",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocltd as usize] = mkop(
        b"cltd",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocned as usize] = mkop(
        b"cned",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocod as usize] = mkop(
        b"cod",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ocuod as usize] = mkop(
        b"cuod",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );

    optab0[O::Ostoreb as usize] = mkop(
        b"storeb",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Ostoreh as usize] = mkop(
        b"storeh",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Ostorew as usize] = mkop(
        b"storew",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Ostorel as usize] = mkop(
        b"storel",
        [
            [
                /*[Kw]=*/ Kl, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Ostores as usize] = mkop(
        b"stores",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Ostored as usize] = mkop(
        b"stored",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );

    optab0[O::Oloadsb as usize] = mkop(
        b"loadsb",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Km, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oloadub as usize] = mkop(
        b"loadub",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Km, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oloadsh as usize] = mkop(
        b"loadsh",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Km, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oloaduh as usize] = mkop(
        b"loaduh",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Km, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oloadsw as usize] = mkop(
        b"loadsw",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Km, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oloaduw as usize] = mkop(
        b"loaduw",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Km, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oload as usize] = mkop(
        b"load",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Km, /*[Ks]=*/ Km, /*[Kd]=*/ Km,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );

    optab0[O::Oextsb as usize] = mkop(
        b"extsb",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oextub as usize] = mkop(
        b"extub",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oextsh as usize] = mkop(
        b"extsh",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oextuh as usize] = mkop(
        b"extuh",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oextsw as usize] = mkop(
        b"extsw",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oextuw as usize] = mkop(
        b"extuw",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kw, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );

    optab0[O::Oexts as usize] = mkop(
        b"exts",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ks,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Kx,
            ],
        ],
        true,
    );
    optab0[O::Otruncd as usize] = mkop(
        b"truncd",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Kd, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Kx, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ostosi as usize] = mkop(
        b"stosi",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Ostoui as usize] = mkop(
        b"stoui",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Ks, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Odtosi as usize] = mkop(
        b"dtosi",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Odtoui as usize] = mkop(
        b"dtoui",
        [
            [
                /*[Kw]=*/ Kd, /*[Kl]=*/ Kd, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        true,
    );
    optab0[O::Oswtof as usize] = mkop(
        b"swtof",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Kw, /*[Kd]=*/ Kw,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        true,
    );
    optab0[O::Ouwtof as usize] = mkop(
        b"uwtof",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Kw, /*[Kd]=*/ Kw,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        true,
    );
    optab0[O::Osltof as usize] = mkop(
        b"sltof",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Kl, /*[Kd]=*/ Kl,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        true,
    );
    optab0[O::Oultof as usize] = mkop(
        b"ultof",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Kl, /*[Kd]=*/ Kl,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        true,
    );
    optab0[O::Ocast as usize] = mkop(
        b"cast",
        [
            [
                /*[Kw]=*/ Ks, /*[Kl]=*/ Kd, /*[Ks]=*/ Kw, /*[Kd]=*/ Kl,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        true,
    );

    optab0[O::Oalloc4 as usize] = mkop(
        b"alloc4",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oalloc8 as usize] = mkop(
        b"alloc8",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oalloc16 as usize] = mkop(
        b"alloc16",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );

    optab0[O::Ovaarg as usize] = mkop(
        b"vaarg",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Km, /*[Ks]=*/ Km, /*[Kd]=*/ Km,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Ovastart as usize] = mkop(
        b"vastart",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );

    optab0[O::Ocopy as usize] = mkop(
        b"copy",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );

    optab0[O::Odbgloc as usize] = mkop(
        b"dbgloc",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );

    optab0[O::Onop as usize] = mkop(
        b"nop",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oaddr as usize] = mkop(
        b"addr",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Km, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oblit0 as usize] = mkop(
        b"blit0",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oblit1 as usize] = mkop(
        b"blit1",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oswap as usize] = mkop(
        b"swap",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
        ],
        false,
    );
    optab0[O::Osign as usize] = mkop(
        b"sign",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Osalloc as usize] = mkop(
        b"salloc",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oxidiv as usize] = mkop(
        b"xidiv",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oxdiv as usize] = mkop(
        b"xdiv",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oxcmp as usize] = mkop(
        b"xcmp",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
        ],
        false,
    );
    optab0[O::Oxtest as usize] = mkop(
        b"xtest",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oacmp as usize] = mkop(
        b"acmp",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oacmn as usize] = mkop(
        b"acmn",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oafcmp as usize] = mkop(
        b"afcmp",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Ke, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
        ],
        false,
    );
    optab0[O::Oreqz as usize] = mkop(
        b"reqz",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Ornez as usize] = mkop(
        b"rnez",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );

    optab0[O::Opar as usize] = mkop(
        b"par",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oparsb as usize] = mkop(
        b"parsb",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oparub as usize] = mkop(
        b"parub",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oparsh as usize] = mkop(
        b"parsh",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oparuh as usize] = mkop(
        b"paruh",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oparc as usize] = mkop(
        b"parc",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Opare as usize] = mkop(
        b"pare",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oarg as usize] = mkop(
        b"arg",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Kl, /*[Ks]=*/ Ks, /*[Kd]=*/ Kd,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oargsb as usize] = mkop(
        b"argsb",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oargub as usize] = mkop(
        b"argub",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oargsh as usize] = mkop(
        b"argsh",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oarguh as usize] = mkop(
        b"arguh",
        [
            [
                /*[Kw]=*/ Kw, /*[Kl]=*/ Ke, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Oargc as usize] = mkop(
        b"argc",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oarge as usize] = mkop(
        b"arge",
        [
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kl, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Ke, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oargv as usize] = mkop(
        b"argv",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );
    optab0[O::Ocall as usize] = mkop(
        b"call",
        [
            [
                /*[Kw]=*/ Km, /*[Kl]=*/ Km, /*[Ks]=*/ Km, /*[Kd]=*/ Km,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Kx, /*[Kd]=*/ Kx,
            ],
        ],
        false,
    );

    optab0[O::Oflagieq as usize] = mkop(
        b"flagieq",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagine as usize] = mkop(
        b"flagine",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagisge as usize] = mkop(
        b"flagisge",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagisgt as usize] = mkop(
        b"flagisgt",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagisle as usize] = mkop(
        b"flagisle",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagislt as usize] = mkop(
        b"flagislt",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagiuge as usize] = mkop(
        b"flagiuge",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagiugt as usize] = mkop(
        b"flagiugt",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagiule as usize] = mkop(
        b"flagiule",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagiult as usize] = mkop(
        b"flagiult",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagfeq as usize] = mkop(
        b"flagfeq",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagfge as usize] = mkop(
        b"flagfge",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagfgt as usize] = mkop(
        b"flagfgt",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagfle as usize] = mkop(
        b"flagfle",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagflt as usize] = mkop(
        b"flagflt",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagfne as usize] = mkop(
        b"flagfne",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagfo as usize] = mkop(
        b"flagfo",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );
    optab0[O::Oflagfuo as usize] = mkop(
        b"flagfuo",
        [
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
            [
                /*[Kw]=*/ Kx, /*[Kl]=*/ Kx, /*[Ks]=*/ Ke, /*[Kd]=*/ Ke,
            ],
        ],
        false,
    );

    optab0
};
