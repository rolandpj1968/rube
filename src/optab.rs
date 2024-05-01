use std::ops::Index;

use crate::all::K::{Kd, Ke, Kl, Ks, Kw, Kx};
use crate::all::{Op, KM, O};

def_enum_index!(O, [Op], Op);
// Hrmm, doesn't work in the below cos mut refs not allowed in static
// def_enum_index_mut!(O, [Op], Op);

pub static OPTAB: [Op; O::NOp as usize] = {
    let nullop = Op::new(b"", [[Ke, Ke, Ke, Ke], [Ke, Ke, Ke, Ke]], false);
    let mut optab0 = [nullop; O::NOp as usize];

    // Generated from QBE with gcc -E and then hand-munged
    optab0[O::Oadd as usize] = Op::new(b"add", [[Kw, Kl, Ks, Kd], [Kw, Kl, Ks, Kd]], true);
    optab0[O::Osub as usize] = Op::new(b"sub", [[Kw, Kl, Ks, Kd], [Kw, Kl, Ks, Kd]], true);
    optab0[O::Oneg as usize] = Op::new(b"neg", [[Kw, Kl, Ks, Kd], [Kx, Kx, Kx, Kx]], true);
    optab0[O::Odiv as usize] = Op::new(b"div", [[Kw, Kl, Ks, Kd], [Kw, Kl, Ks, Kd]], true);
    optab0[O::Orem as usize] = Op::new(b"rem", [[Kw, Kl, Ke, Ke], [Kw, Kl, Ke, Ke]], true);
    optab0[O::Oudiv as usize] = Op::new(b"udiv", [[Kw, Kl, Ke, Ke], [Kw, Kl, Ke, Ke]], true);
    optab0[O::Ourem as usize] = Op::new(b"urem", [[Kw, Kl, Ke, Ke], [Kw, Kl, Ke, Ke]], true);
    optab0[O::Omul as usize] = Op::new(b"mul", [[Kw, Kl, Ks, Kd], [Kw, Kl, Ks, Kd]], true);
    optab0[O::Oand as usize] = Op::new(b"and", [[Kw, Kl, Ke, Ke], [Kw, Kl, Ke, Ke]], true);
    optab0[O::Oor as usize] = Op::new(b"or", [[Kw, Kl, Ke, Ke], [Kw, Kl, Ke, Ke]], true);
    optab0[O::Oxor as usize] = Op::new(b"xor", [[Kw, Kl, Ke, Ke], [Kw, Kl, Ke, Ke]], true);
    optab0[O::Osar as usize] = Op::new(b"sar", [[Kw, Kl, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Oshr as usize] = Op::new(b"shr", [[Kw, Kl, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Oshl as usize] = Op::new(b"shl", [[Kw, Kl, Ke, Ke], [Kw, Kw, Ke, Ke]], true);

    optab0[O::Oceqw as usize] = Op::new(b"ceqw", [[Kw, Kw, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Ocnew as usize] = Op::new(b"cnew", [[Kw, Kw, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Ocsgew as usize] = Op::new(b"csgew", [[Kw, Kw, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Ocsgtw as usize] = Op::new(b"csgtw", [[Kw, Kw, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Ocslew as usize] = Op::new(b"cslew", [[Kw, Kw, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Ocsltw as usize] = Op::new(b"csltw", [[Kw, Kw, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Ocugew as usize] = Op::new(b"cugew", [[Kw, Kw, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Ocugtw as usize] = Op::new(b"cugtw", [[Kw, Kw, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Oculew as usize] = Op::new(b"culew", [[Kw, Kw, Ke, Ke], [Kw, Kw, Ke, Ke]], true);
    optab0[O::Ocultw as usize] = Op::new(b"cultw", [[Kw, Kw, Ke, Ke], [Kw, Kw, Ke, Ke]], true);

    optab0[O::Oceql as usize] = Op::new(b"ceql", [[Kl, Kl, Ke, Ke], [Kl, Kl, Ke, Ke]], true);
    optab0[O::Ocnel as usize] = Op::new(b"cnel", [[Kl, Kl, Ke, Ke], [Kl, Kl, Ke, Ke]], true);
    optab0[O::Ocsgel as usize] = Op::new(b"csgel", [[Kl, Kl, Ke, Ke], [Kl, Kl, Ke, Ke]], true);
    optab0[O::Ocsgtl as usize] = Op::new(b"csgtl", [[Kl, Kl, Ke, Ke], [Kl, Kl, Ke, Ke]], true);
    optab0[O::Ocslel as usize] = Op::new(b"cslel", [[Kl, Kl, Ke, Ke], [Kl, Kl, Ke, Ke]], true);
    optab0[O::Ocsltl as usize] = Op::new(b"csltl", [[Kl, Kl, Ke, Ke], [Kl, Kl, Ke, Ke]], true);
    optab0[O::Ocugel as usize] = Op::new(b"cugel", [[Kl, Kl, Ke, Ke], [Kl, Kl, Ke, Ke]], true);
    optab0[O::Ocugtl as usize] = Op::new(b"cugtl", [[Kl, Kl, Ke, Ke], [Kl, Kl, Ke, Ke]], true);
    optab0[O::Oculel as usize] = Op::new(b"culel", [[Kl, Kl, Ke, Ke], [Kl, Kl, Ke, Ke]], true);
    optab0[O::Ocultl as usize] = Op::new(b"cultl", [[Kl, Kl, Ke, Ke], [Kl, Kl, Ke, Ke]], true);

    optab0[O::Oceqs as usize] = Op::new(b"ceqs", [[Ks, Ks, Ke, Ke], [Ks, Ks, Ke, Ke]], true);
    optab0[O::Ocges as usize] = Op::new(b"cges", [[Ks, Ks, Ke, Ke], [Ks, Ks, Ke, Ke]], true);
    optab0[O::Ocgts as usize] = Op::new(b"cgts", [[Ks, Ks, Ke, Ke], [Ks, Ks, Ke, Ke]], true);
    optab0[O::Ocles as usize] = Op::new(b"cles", [[Ks, Ks, Ke, Ke], [Ks, Ks, Ke, Ke]], true);
    optab0[O::Oclts as usize] = Op::new(b"clts", [[Ks, Ks, Ke, Ke], [Ks, Ks, Ke, Ke]], true);
    optab0[O::Ocnes as usize] = Op::new(b"cnes", [[Ks, Ks, Ke, Ke], [Ks, Ks, Ke, Ke]], true);
    optab0[O::Ocos as usize] = Op::new(b"cos", [[Ks, Ks, Ke, Ke], [Ks, Ks, Ke, Ke]], true);
    optab0[O::Ocuos as usize] = Op::new(b"cuos", [[Ks, Ks, Ke, Ke], [Ks, Ks, Ke, Ke]], true);

    optab0[O::Oceqd as usize] = Op::new(b"ceqd", [[Kd, Kd, Ke, Ke], [Kd, Kd, Ke, Ke]], true);
    optab0[O::Ocged as usize] = Op::new(b"cged", [[Kd, Kd, Ke, Ke], [Kd, Kd, Ke, Ke]], true);
    optab0[O::Ocgtd as usize] = Op::new(b"cgtd", [[Kd, Kd, Ke, Ke], [Kd, Kd, Ke, Ke]], true);
    optab0[O::Ocled as usize] = Op::new(b"cled", [[Kd, Kd, Ke, Ke], [Kd, Kd, Ke, Ke]], true);
    optab0[O::Ocltd as usize] = Op::new(b"cltd", [[Kd, Kd, Ke, Ke], [Kd, Kd, Ke, Ke]], true);
    optab0[O::Ocned as usize] = Op::new(b"cned", [[Kd, Kd, Ke, Ke], [Kd, Kd, Ke, Ke]], true);
    optab0[O::Ocod as usize] = Op::new(b"cod", [[Kd, Kd, Ke, Ke], [Kd, Kd, Ke, Ke]], true);
    optab0[O::Ocuod as usize] = Op::new(b"cuod", [[Kd, Kd, Ke, Ke], [Kd, Kd, Ke, Ke]], true);

    optab0[O::Ostoreb as usize] = Op::new(b"storeb", [[Kw, Ke, Ke, Ke], [KM, Ke, Ke, Ke]], false);
    optab0[O::Ostoreh as usize] = Op::new(b"storeh", [[Kw, Ke, Ke, Ke], [KM, Ke, Ke, Ke]], false);
    optab0[O::Ostorew as usize] = Op::new(b"storew", [[Kw, Ke, Ke, Ke], [KM, Ke, Ke, Ke]], false);
    optab0[O::Ostorel as usize] = Op::new(b"storel", [[Kl, Ke, Ke, Ke], [KM, Ke, Ke, Ke]], false);
    optab0[O::Ostores as usize] = Op::new(b"stores", [[Ks, Ke, Ke, Ke], [KM, Ke, Ke, Ke]], false);
    optab0[O::Ostored as usize] = Op::new(b"stored", [[Kd, Ke, Ke, Ke], [KM, Ke, Ke, Ke]], false);

    optab0[O::Oloadsb as usize] = Op::new(b"loadsb", [[KM, KM, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oloadub as usize] = Op::new(b"loadub", [[KM, KM, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oloadsh as usize] = Op::new(b"loadsh", [[KM, KM, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oloaduh as usize] = Op::new(b"loaduh", [[KM, KM, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oloadsw as usize] = Op::new(b"loadsw", [[KM, KM, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oloaduw as usize] = Op::new(b"loaduw", [[KM, KM, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oload as usize] = Op::new(b"load", [[KM, KM, KM, KM], [Kx, Kx, Kx, Kx]], false);

    optab0[O::Oextsb as usize] = Op::new(b"extsb", [[Kw, Kw, Ke, Ke], [Kx, Kx, Ke, Ke]], true);
    optab0[O::Oextub as usize] = Op::new(b"extub", [[Kw, Kw, Ke, Ke], [Kx, Kx, Ke, Ke]], true);
    optab0[O::Oextsh as usize] = Op::new(b"extsh", [[Kw, Kw, Ke, Ke], [Kx, Kx, Ke, Ke]], true);
    optab0[O::Oextuh as usize] = Op::new(b"extuh", [[Kw, Kw, Ke, Ke], [Kx, Kx, Ke, Ke]], true);
    optab0[O::Oextsw as usize] = Op::new(b"extsw", [[Ke, Kw, Ke, Ke], [Ke, Kx, Ke, Ke]], true);
    optab0[O::Oextuw as usize] = Op::new(b"extuw", [[Ke, Kw, Ke, Ke], [Ke, Kx, Ke, Ke]], true);

    optab0[O::Oexts as usize] = Op::new(b"exts", [[Ke, Ke, Ke, Ks], [Ke, Ke, Ke, Kx]], true);
    optab0[O::Otruncd as usize] = Op::new(b"truncd", [[Ke, Ke, Kd, Ke], [Ke, Ke, Kx, Ke]], true);
    optab0[O::Ostosi as usize] = Op::new(b"stosi", [[Ks, Ks, Ke, Ke], [Kx, Kx, Ke, Ke]], true);
    optab0[O::Ostoui as usize] = Op::new(b"stoui", [[Ks, Ks, Ke, Ke], [Kx, Kx, Ke, Ke]], true);
    optab0[O::Odtosi as usize] = Op::new(b"dtosi", [[Kd, Kd, Ke, Ke], [Kx, Kx, Ke, Ke]], true);
    optab0[O::Odtoui as usize] = Op::new(b"dtoui", [[Kd, Kd, Ke, Ke], [Kx, Kx, Ke, Ke]], true);
    optab0[O::Oswtof as usize] = Op::new(b"swtof", [[Ke, Ke, Kw, Kw], [Ke, Ke, Kx, Kx]], true);
    optab0[O::Ouwtof as usize] = Op::new(b"uwtof", [[Ke, Ke, Kw, Kw], [Ke, Ke, Kx, Kx]], true);
    optab0[O::Osltof as usize] = Op::new(b"sltof", [[Ke, Ke, Kl, Kl], [Ke, Ke, Kx, Kx]], true);
    optab0[O::Oultof as usize] = Op::new(b"ultof", [[Ke, Ke, Kl, Kl], [Ke, Ke, Kx, Kx]], true);
    optab0[O::Ocast as usize] = Op::new(b"cast", [[Ks, Kd, Kw, Kl], [Kx, Kx, Kx, Kx]], true);

    optab0[O::Oalloc4 as usize] = Op::new(b"alloc4", [[Ke, Kl, Ke, Ke], [Ke, Kx, Ke, Ke]], false);
    optab0[O::Oalloc8 as usize] = Op::new(b"alloc8", [[Ke, Kl, Ke, Ke], [Ke, Kx, Ke, Ke]], false);
    optab0[O::Oalloc16 as usize] = Op::new(b"alloc16", [[Ke, Kl, Ke, Ke], [Ke, Kx, Ke, Ke]], false);

    optab0[O::Ovaarg as usize] = Op::new(b"vaarg", [[KM, KM, KM, KM], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Ovastart as usize] = Op::new(b"vastart", [[KM, Ke, Ke, Ke], [Kx, Ke, Ke, Ke]], false);

    optab0[O::Ocopy as usize] = Op::new(b"copy", [[Kw, Kl, Ks, Kd], [Kx, Kx, Kx, Kx]], false);

    optab0[O::Odbgloc as usize] = Op::new(b"dbgloc", [[Kw, Ke, Ke, Ke], [Kw, Ke, Ke, Ke]], false);

    optab0[O::Onop as usize] = Op::new(b"nop", [[Kx, Kx, Kx, Kx], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oaddr as usize] = Op::new(b"addr", [[KM, KM, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oblit0 as usize] = Op::new(b"blit0", [[KM, Ke, Ke, Ke], [KM, Ke, Ke, Ke]], false);
    optab0[O::Oblit1 as usize] = Op::new(b"blit1", [[Kw, Ke, Ke, Ke], [Kx, Ke, Ke, Ke]], false);
    optab0[O::Oswap as usize] = Op::new(b"swap", [[Kw, Kl, Ks, Kd], [Kw, Kl, Ks, Kd]], false);
    optab0[O::Osign as usize] = Op::new(b"sign", [[Kw, Kl, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Osalloc as usize] = Op::new(b"salloc", [[Ke, Kl, Ke, Ke], [Ke, Kx, Ke, Ke]], false);
    optab0[O::Oxidiv as usize] = Op::new(b"xidiv", [[Kw, Kl, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oxdiv as usize] = Op::new(b"xdiv", [[Kw, Kl, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oxcmp as usize] = Op::new(b"xcmp", [[Kw, Kl, Ks, Kd], [Kw, Kl, Ks, Kd]], false);
    optab0[O::Oxtest as usize] = Op::new(b"xtest", [[Kw, Kl, Ke, Ke], [Kw, Kl, Ke, Ke]], false);
    optab0[O::Oacmp as usize] = Op::new(b"acmp", [[Kw, Kl, Ke, Ke], [Kw, Kl, Ke, Ke]], false);
    optab0[O::Oacmn as usize] = Op::new(b"acmn", [[Kw, Kl, Ke, Ke], [Kw, Kl, Ke, Ke]], false);
    optab0[O::Oafcmp as usize] = Op::new(b"afcmp", [[Ke, Ke, Ks, Kd], [Ke, Ke, Ks, Kd]], false);
    optab0[O::Oreqz as usize] = Op::new(b"reqz", [[Kw, Kl, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Ornez as usize] = Op::new(b"rnez", [[Kw, Kl, Ke, Ke], [Kx, Kx, Ke, Ke]], false);

    optab0[O::Opar as usize] = Op::new(b"par", [[Kx, Kx, Kx, Kx], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oparsb as usize] = Op::new(b"parsb", [[Kx, Kx, Kx, Kx], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oparub as usize] = Op::new(b"parub", [[Kx, Kx, Kx, Kx], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oparsh as usize] = Op::new(b"parsh", [[Kx, Kx, Kx, Kx], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oparuh as usize] = Op::new(b"paruh", [[Kx, Kx, Kx, Kx], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oparc as usize] = Op::new(b"parc", [[Ke, Kx, Ke, Ke], [Ke, Kx, Ke, Ke]], false);
    optab0[O::Opare as usize] = Op::new(b"pare", [[Ke, Kx, Ke, Ke], [Ke, Kx, Ke, Ke]], false);
    optab0[O::Oarg as usize] = Op::new(b"arg", [[Kw, Kl, Ks, Kd], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oargsb as usize] = Op::new(b"argsb", [[Kw, Ke, Ke, Ke], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oargub as usize] = Op::new(b"argub", [[Kw, Ke, Ke, Ke], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oargsh as usize] = Op::new(b"argsh", [[Kw, Ke, Ke, Ke], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oarguh as usize] = Op::new(b"arguh", [[Kw, Ke, Ke, Ke], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Oargc as usize] = Op::new(b"argc", [[Ke, Kx, Ke, Ke], [Ke, Kl, Ke, Ke]], false);
    optab0[O::Oarge as usize] = Op::new(b"arge", [[Ke, Kl, Ke, Ke], [Ke, Kx, Ke, Ke]], false);
    optab0[O::Oargv as usize] = Op::new(b"argv", [[Kx, Kx, Kx, Kx], [Kx, Kx, Kx, Kx]], false);
    optab0[O::Ocall as usize] = Op::new(b"call", [[KM, KM, KM, KM], [Kx, Kx, Kx, Kx]], false);

    optab0[O::Oflagieq as usize] = Op::new(b"flagieq", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagine as usize] = Op::new(b"flagine", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagisge as usize] =
        Op::new(b"flagisge", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagisgt as usize] =
        Op::new(b"flagisgt", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagisle as usize] =
        Op::new(b"flagisle", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagislt as usize] =
        Op::new(b"flagislt", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagiuge as usize] =
        Op::new(b"flagiuge", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagiugt as usize] =
        Op::new(b"flagiugt", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagiule as usize] =
        Op::new(b"flagiule", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagiult as usize] =
        Op::new(b"flagiult", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagfeq as usize] = Op::new(b"flagfeq", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagfge as usize] = Op::new(b"flagfge", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagfgt as usize] = Op::new(b"flagfgt", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagfle as usize] = Op::new(b"flagfle", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagflt as usize] = Op::new(b"flagflt", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagfne as usize] = Op::new(b"flagfne", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagfo as usize] = Op::new(b"flagfo", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);
    optab0[O::Oflagfuo as usize] = Op::new(b"flagfuo", [[Kx, Kx, Ke, Ke], [Kx, Kx, Ke, Ke]], false);

    optab0
};
