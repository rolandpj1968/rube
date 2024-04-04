use crate::all::{Op, KD, KE, KL, KM, KS, KW, KX, O};

pub static OPTAB: [Op; O::NOp as usize] = {
    let nullop = Op::new(b"", [[KE, KE, KE, KE], [KE, KE, KE, KE]], false);
    let mut optab0 = [nullop; O::NOp as usize];

    // Generated from QBE with gcc -E and then hand-munged
    optab0[O::Oadd as usize] = Op::new(b"add", [[KW, KL, KS, KD], [KW, KL, KS, KD]], true);
    optab0[O::Osub as usize] = Op::new(b"sub", [[KW, KL, KS, KD], [KW, KL, KS, KD]], true);
    optab0[O::Oneg as usize] = Op::new(b"neg", [[KW, KL, KS, KD], [KX, KX, KX, KX]], true);
    optab0[O::Odiv as usize] = Op::new(b"div", [[KW, KL, KS, KD], [KW, KL, KS, KD]], true);
    optab0[O::Orem as usize] = Op::new(b"rem", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Oudiv as usize] = Op::new(b"udiv", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Ourem as usize] = Op::new(b"urem", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Omul as usize] = Op::new(b"mul", [[KW, KL, KS, KD], [KW, KL, KS, KD]], true);
    optab0[O::Oand as usize] = Op::new(b"and", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Oor as usize] = Op::new(b"or", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Oxor as usize] = Op::new(b"xor", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Osar as usize] = Op::new(b"sar", [[KW, KL, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Oshr as usize] = Op::new(b"shr", [[KW, KL, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Oshl as usize] = Op::new(b"shl", [[KW, KL, KE, KE], [KW, KW, KE, KE]], true);

    optab0[O::Oceqw as usize] = Op::new(b"ceqw", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocnew as usize] = Op::new(b"cnew", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocsgew as usize] = Op::new(b"csgew", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocsgtw as usize] = Op::new(b"csgtw", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocslew as usize] = Op::new(b"cslew", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocsltw as usize] = Op::new(b"csltw", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocugew as usize] = Op::new(b"cugew", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocugtw as usize] = Op::new(b"cugtw", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Oculew as usize] = Op::new(b"culew", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocultw as usize] = Op::new(b"cultw", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);

    optab0[O::Oceql as usize] = Op::new(b"ceql", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocnel as usize] = Op::new(b"cnel", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocsgel as usize] = Op::new(b"csgel", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocsgtl as usize] = Op::new(b"csgtl", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocslel as usize] = Op::new(b"cslel", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocsltl as usize] = Op::new(b"csltl", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocugel as usize] = Op::new(b"cugel", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocugtl as usize] = Op::new(b"cugtl", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Oculel as usize] = Op::new(b"culel", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocultl as usize] = Op::new(b"cultl", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);

    optab0[O::Oceqs as usize] = Op::new(b"ceqs", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocges as usize] = Op::new(b"cges", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocgts as usize] = Op::new(b"cgts", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocles as usize] = Op::new(b"cles", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Oclts as usize] = Op::new(b"clts", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocnes as usize] = Op::new(b"cnes", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocos as usize] = Op::new(b"cos", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocuos as usize] = Op::new(b"cuos", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);

    optab0[O::Oceqd as usize] = Op::new(b"ceqd", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocged as usize] = Op::new(b"cged", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocgtd as usize] = Op::new(b"cgtd", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocled as usize] = Op::new(b"cled", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocltd as usize] = Op::new(b"cltd", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocned as usize] = Op::new(b"cned", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocod as usize] = Op::new(b"cod", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocuod as usize] = Op::new(b"cuod", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);

    optab0[O::Ostoreb as usize] = Op::new(b"storeb", [[KW, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Ostoreh as usize] = Op::new(b"storeh", [[KW, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Ostorew as usize] = Op::new(b"storew", [[KW, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Ostorel as usize] = Op::new(b"storel", [[KL, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Ostores as usize] = Op::new(b"stores", [[KS, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Ostored as usize] = Op::new(b"stored", [[KD, KE, KE, KE], [KM, KE, KE, KE]], false);

    optab0[O::Oloadsb as usize] = Op::new(b"loadsb", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oloadub as usize] = Op::new(b"loadub", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oloadsh as usize] = Op::new(b"loadsh", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oloaduh as usize] = Op::new(b"loaduh", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oloadsw as usize] = Op::new(b"loadsw", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oloaduw as usize] = Op::new(b"loaduw", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oload as usize] = Op::new(b"load", [[KM, KM, KM, KM], [KX, KX, KX, KX]], false);

    optab0[O::Oextsb as usize] = Op::new(b"extsb", [[KW, KW, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Oextub as usize] = Op::new(b"extub", [[KW, KW, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Oextsh as usize] = Op::new(b"extsh", [[KW, KW, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Oextuh as usize] = Op::new(b"extuh", [[KW, KW, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Oextsw as usize] = Op::new(b"extsw", [[KE, KW, KE, KE], [KE, KX, KE, KE]], true);
    optab0[O::Oextuw as usize] = Op::new(b"extuw", [[KE, KW, KE, KE], [KE, KX, KE, KE]], true);

    optab0[O::Oexts as usize] = Op::new(b"exts", [[KE, KE, KE, KS], [KE, KE, KE, KX]], true);
    optab0[O::Otruncd as usize] = Op::new(b"truncd", [[KE, KE, KD, KE], [KE, KE, KX, KE]], true);
    optab0[O::Ostosi as usize] = Op::new(b"stosi", [[KS, KS, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Ostoui as usize] = Op::new(b"stoui", [[KS, KS, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Odtosi as usize] = Op::new(b"dtosi", [[KD, KD, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Odtoui as usize] = Op::new(b"dtoui", [[KD, KD, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Oswtof as usize] = Op::new(b"swtof", [[KE, KE, KW, KW], [KE, KE, KX, KX]], true);
    optab0[O::Ouwtof as usize] = Op::new(b"uwtof", [[KE, KE, KW, KW], [KE, KE, KX, KX]], true);
    optab0[O::Osltof as usize] = Op::new(b"sltof", [[KE, KE, KL, KL], [KE, KE, KX, KX]], true);
    optab0[O::Oultof as usize] = Op::new(b"ultof", [[KE, KE, KL, KL], [KE, KE, KX, KX]], true);
    optab0[O::Ocast as usize] = Op::new(b"cast", [[KS, KD, KW, KL], [KX, KX, KX, KX]], true);

    optab0[O::Oalloc4 as usize] = Op::new(b"alloc4", [[KE, KL, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Oalloc8 as usize] = Op::new(b"alloc8", [[KE, KL, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Oalloc16 as usize] = Op::new(b"alloc16", [[KE, KL, KE, KE], [KE, KX, KE, KE]], false);

    optab0[O::Ovaarg as usize] = Op::new(b"vaarg", [[KM, KM, KM, KM], [KX, KX, KX, KX]], false);
    optab0[O::Ovastart as usize] = Op::new(b"vastart", [[KM, KE, KE, KE], [KX, KE, KE, KE]], false);

    optab0[O::Ocopy as usize] = Op::new(b"copy", [[KW, KL, KS, KD], [KX, KX, KX, KX]], false);

    optab0[O::Odbgloc as usize] = Op::new(b"dbgloc", [[KW, KE, KE, KE], [KW, KE, KE, KE]], false);

    optab0[O::Onop as usize] = Op::new(b"nop", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oaddr as usize] = Op::new(b"addr", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oblit0 as usize] = Op::new(b"blit0", [[KM, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Oblit1 as usize] = Op::new(b"blit1", [[KW, KE, KE, KE], [KX, KE, KE, KE]], false);
    optab0[O::Oswap as usize] = Op::new(b"swap", [[KW, KL, KS, KD], [KW, KL, KS, KD]], false);
    optab0[O::Osign as usize] = Op::new(b"sign", [[KW, KL, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Osalloc as usize] = Op::new(b"salloc", [[KE, KL, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Oxidiv as usize] = Op::new(b"xidiv", [[KW, KL, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oxdiv as usize] = Op::new(b"xdiv", [[KW, KL, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oxcmp as usize] = Op::new(b"xcmp", [[KW, KL, KS, KD], [KW, KL, KS, KD]], false);
    optab0[O::Oxtest as usize] = Op::new(b"xtest", [[KW, KL, KE, KE], [KW, KL, KE, KE]], false);
    optab0[O::Oacmp as usize] = Op::new(b"acmp", [[KW, KL, KE, KE], [KW, KL, KE, KE]], false);
    optab0[O::Oacmn as usize] = Op::new(b"acmn", [[KW, KL, KE, KE], [KW, KL, KE, KE]], false);
    optab0[O::Oafcmp as usize] = Op::new(b"afcmp", [[KE, KE, KS, KD], [KE, KE, KS, KD]], false);
    optab0[O::Oreqz as usize] = Op::new(b"reqz", [[KW, KL, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Ornez as usize] = Op::new(b"rnez", [[KW, KL, KE, KE], [KX, KX, KE, KE]], false);

    optab0[O::Opar as usize] = Op::new(b"par", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oparsb as usize] = Op::new(b"parsb", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oparub as usize] = Op::new(b"parub", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oparsh as usize] = Op::new(b"parsh", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oparuh as usize] = Op::new(b"paruh", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oparc as usize] = Op::new(b"parc", [[KE, KX, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Opare as usize] = Op::new(b"pare", [[KE, KX, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Oarg as usize] = Op::new(b"arg", [[KW, KL, KS, KD], [KX, KX, KX, KX]], false);
    optab0[O::Oargsb as usize] = Op::new(b"argsb", [[KW, KE, KE, KE], [KX, KX, KX, KX]], false);
    optab0[O::Oargub as usize] = Op::new(b"argub", [[KW, KE, KE, KE], [KX, KX, KX, KX]], false);
    optab0[O::Oargsh as usize] = Op::new(b"argsh", [[KW, KE, KE, KE], [KX, KX, KX, KX]], false);
    optab0[O::Oarguh as usize] = Op::new(b"arguh", [[KW, KE, KE, KE], [KX, KX, KX, KX]], false);
    optab0[O::Oargc as usize] = Op::new(b"argc", [[KE, KX, KE, KE], [KE, KL, KE, KE]], false);
    optab0[O::Oarge as usize] = Op::new(b"arge", [[KE, KL, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Oargv as usize] = Op::new(b"argv", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Ocall as usize] = Op::new(b"call", [[KM, KM, KM, KM], [KX, KX, KX, KX]], false);

    optab0[O::Oflagieq as usize] = Op::new(b"flagieq", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagine as usize] = Op::new(b"flagine", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagisge as usize] =
        Op::new(b"flagisge", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagisgt as usize] =
        Op::new(b"flagisgt", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagisle as usize] =
        Op::new(b"flagisle", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagislt as usize] =
        Op::new(b"flagislt", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagiuge as usize] =
        Op::new(b"flagiuge", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagiugt as usize] =
        Op::new(b"flagiugt", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagiule as usize] =
        Op::new(b"flagiule", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagiult as usize] =
        Op::new(b"flagiult", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfeq as usize] = Op::new(b"flagfeq", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfge as usize] = Op::new(b"flagfge", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfgt as usize] = Op::new(b"flagfgt", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfle as usize] = Op::new(b"flagfle", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagflt as usize] = Op::new(b"flagflt", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfne as usize] = Op::new(b"flagfne", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfo as usize] = Op::new(b"flagfo", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfuo as usize] = Op::new(b"flagfuo", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);

    optab0
};
