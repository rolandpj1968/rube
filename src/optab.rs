use crate::all::{Op, KD, KE, KL, KM, KS, KW, KX, O};

pub static OPTAB: [Op; O::NOp as usize] = {
    let nullop = Op::new("", [[KE, KE, KE, KE], [KE, KE, KE, KE]], false);
    let mut optab0 = [nullop; O::NOp as usize];

    // Generated from QBE with gcc -E and then hand-munged
    optab0[O::Oadd as usize] = Op::new("add", [[KW, KL, KS, KD], [KW, KL, KS, KD]], true);
    optab0[O::Osub as usize] = Op::new("sub", [[KW, KL, KS, KD], [KW, KL, KS, KD]], true);
    optab0[O::Oneg as usize] = Op::new("neg", [[KW, KL, KS, KD], [KX, KX, KX, KX]], true);
    optab0[O::Odiv as usize] = Op::new("div", [[KW, KL, KS, KD], [KW, KL, KS, KD]], true);
    optab0[O::Orem as usize] = Op::new("rem", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Oudiv as usize] = Op::new("udiv", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Ourem as usize] = Op::new("urem", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Omul as usize] = Op::new("mul", [[KW, KL, KS, KD], [KW, KL, KS, KD]], true);
    optab0[O::Oand as usize] = Op::new("and", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Oor as usize] = Op::new("or", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Oxor as usize] = Op::new("xor", [[KW, KL, KE, KE], [KW, KL, KE, KE]], true);
    optab0[O::Osar as usize] = Op::new("sar", [[KW, KL, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Oshr as usize] = Op::new("shr", [[KW, KL, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Oshl as usize] = Op::new("shl", [[KW, KL, KE, KE], [KW, KW, KE, KE]], true);

    optab0[O::Oceqw as usize] = Op::new("ceqw", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocnew as usize] = Op::new("cnew", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocsgew as usize] = Op::new("csgew", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocsgtw as usize] = Op::new("csgtw", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocslew as usize] = Op::new("cslew", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocsltw as usize] = Op::new("csltw", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocugew as usize] = Op::new("cugew", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocugtw as usize] = Op::new("cugtw", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Oculew as usize] = Op::new("culew", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);
    optab0[O::Ocultw as usize] = Op::new("cultw", [[KW, KW, KE, KE], [KW, KW, KE, KE]], true);

    optab0[O::Oceql as usize] = Op::new("ceql", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocnel as usize] = Op::new("cnel", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocsgel as usize] = Op::new("csgel", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocsgtl as usize] = Op::new("csgtl", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocslel as usize] = Op::new("cslel", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocsltl as usize] = Op::new("csltl", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocugel as usize] = Op::new("cugel", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocugtl as usize] = Op::new("cugtl", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Oculel as usize] = Op::new("culel", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);
    optab0[O::Ocultl as usize] = Op::new("cultl", [[KL, KL, KE, KE], [KL, KL, KE, KE]], true);

    optab0[O::Oceqs as usize] = Op::new("ceqs", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocges as usize] = Op::new("cges", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocgts as usize] = Op::new("cgts", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocles as usize] = Op::new("cles", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Oclts as usize] = Op::new("clts", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocnes as usize] = Op::new("cnes", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocos as usize] = Op::new("cos", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);
    optab0[O::Ocuos as usize] = Op::new("cuos", [[KS, KS, KE, KE], [KS, KS, KE, KE]], true);

    optab0[O::Oceqd as usize] = Op::new("ceqd", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocged as usize] = Op::new("cged", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocgtd as usize] = Op::new("cgtd", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocled as usize] = Op::new("cled", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocltd as usize] = Op::new("cltd", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocned as usize] = Op::new("cned", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocod as usize] = Op::new("cod", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);
    optab0[O::Ocuod as usize] = Op::new("cuod", [[KD, KD, KE, KE], [KD, KD, KE, KE]], true);

    optab0[O::Ostoreb as usize] = Op::new("storeb", [[KW, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Ostoreh as usize] = Op::new("storeh", [[KW, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Ostorew as usize] = Op::new("storew", [[KW, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Ostorel as usize] = Op::new("storel", [[KL, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Ostores as usize] = Op::new("stores", [[KS, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Ostored as usize] = Op::new("stored", [[KD, KE, KE, KE], [KM, KE, KE, KE]], false);

    optab0[O::Oloadsb as usize] = Op::new("loadsb", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oloadub as usize] = Op::new("loadub", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oloadsh as usize] = Op::new("loadsh", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oloaduh as usize] = Op::new("loaduh", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oloadsw as usize] = Op::new("loadsw", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oloaduw as usize] = Op::new("loaduw", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oload as usize] = Op::new("load", [[KM, KM, KM, KM], [KX, KX, KX, KX]], false);

    optab0[O::Oextsb as usize] = Op::new("extsb", [[KW, KW, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Oextub as usize] = Op::new("extub", [[KW, KW, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Oextsh as usize] = Op::new("extsh", [[KW, KW, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Oextuh as usize] = Op::new("extuh", [[KW, KW, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Oextsw as usize] = Op::new("extsw", [[KE, KW, KE, KE], [KE, KX, KE, KE]], true);
    optab0[O::Oextuw as usize] = Op::new("extuw", [[KE, KW, KE, KE], [KE, KX, KE, KE]], true);

    optab0[O::Oexts as usize] = Op::new("exts", [[KE, KE, KE, KS], [KE, KE, KE, KX]], true);
    optab0[O::Otruncd as usize] = Op::new("truncd", [[KE, KE, KD, KE], [KE, KE, KX, KE]], true);
    optab0[O::Ostosi as usize] = Op::new("stosi", [[KS, KS, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Ostoui as usize] = Op::new("stoui", [[KS, KS, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Odtosi as usize] = Op::new("dtosi", [[KD, KD, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Odtoui as usize] = Op::new("dtoui", [[KD, KD, KE, KE], [KX, KX, KE, KE]], true);
    optab0[O::Oswtof as usize] = Op::new("swtof", [[KE, KE, KW, KW], [KE, KE, KX, KX]], true);
    optab0[O::Ouwtof as usize] = Op::new("uwtof", [[KE, KE, KW, KW], [KE, KE, KX, KX]], true);
    optab0[O::Osltof as usize] = Op::new("sltof", [[KE, KE, KL, KL], [KE, KE, KX, KX]], true);
    optab0[O::Oultof as usize] = Op::new("ultof", [[KE, KE, KL, KL], [KE, KE, KX, KX]], true);
    optab0[O::Ocast as usize] = Op::new("cast", [[KS, KD, KW, KL], [KX, KX, KX, KX]], true);

    optab0[O::Oalloc4 as usize] = Op::new("alloc4", [[KE, KL, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Oalloc8 as usize] = Op::new("alloc8", [[KE, KL, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Oalloc16 as usize] = Op::new("alloc16", [[KE, KL, KE, KE], [KE, KX, KE, KE]], false);

    optab0[O::Ovaarg as usize] = Op::new("vaarg", [[KM, KM, KM, KM], [KX, KX, KX, KX]], false);
    optab0[O::Ovastart as usize] = Op::new("vastart", [[KM, KE, KE, KE], [KX, KE, KE, KE]], false);

    optab0[O::Ocopy as usize] = Op::new("copy", [[KW, KL, KS, KD], [KX, KX, KX, KX]], false);

    optab0[O::Odbgloc as usize] = Op::new("dbgloc", [[KW, KE, KE, KE], [KW, KE, KE, KE]], false);

    optab0[O::Onop as usize] = Op::new("nop", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oaddr as usize] = Op::new("addr", [[KM, KM, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oblit0 as usize] = Op::new("blit0", [[KM, KE, KE, KE], [KM, KE, KE, KE]], false);
    optab0[O::Oblit1 as usize] = Op::new("blit1", [[KW, KE, KE, KE], [KX, KE, KE, KE]], false);
    optab0[O::Oswap as usize] = Op::new("swap", [[KW, KL, KS, KD], [KW, KL, KS, KD]], false);
    optab0[O::Osign as usize] = Op::new("sign", [[KW, KL, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Osalloc as usize] = Op::new("salloc", [[KE, KL, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Oxidiv as usize] = Op::new("xidiv", [[KW, KL, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oxdiv as usize] = Op::new("xdiv", [[KW, KL, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oxcmp as usize] = Op::new("xcmp", [[KW, KL, KS, KD], [KW, KL, KS, KD]], false);
    optab0[O::Oxtest as usize] = Op::new("xtest", [[KW, KL, KE, KE], [KW, KL, KE, KE]], false);
    optab0[O::Oacmp as usize] = Op::new("acmp", [[KW, KL, KE, KE], [KW, KL, KE, KE]], false);
    optab0[O::Oacmn as usize] = Op::new("acmn", [[KW, KL, KE, KE], [KW, KL, KE, KE]], false);
    optab0[O::Oafcmp as usize] = Op::new("afcmp", [[KE, KE, KS, KD], [KE, KE, KS, KD]], false);
    optab0[O::Oreqz as usize] = Op::new("reqz", [[KW, KL, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Ornez as usize] = Op::new("rnez", [[KW, KL, KE, KE], [KX, KX, KE, KE]], false);

    optab0[O::Opar as usize] = Op::new("par", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oparsb as usize] = Op::new("parsb", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oparub as usize] = Op::new("parub", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oparsh as usize] = Op::new("parsh", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oparuh as usize] = Op::new("paruh", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Oparc as usize] = Op::new("parc", [[KE, KX, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Opare as usize] = Op::new("pare", [[KE, KX, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Oarg as usize] = Op::new("arg", [[KW, KL, KS, KD], [KX, KX, KX, KX]], false);
    optab0[O::Oargsb as usize] = Op::new("argsb", [[KW, KE, KE, KE], [KX, KX, KX, KX]], false);
    optab0[O::Oargub as usize] = Op::new("argub", [[KW, KE, KE, KE], [KX, KX, KX, KX]], false);
    optab0[O::Oargsh as usize] = Op::new("argsh", [[KW, KE, KE, KE], [KX, KX, KX, KX]], false);
    optab0[O::Oarguh as usize] = Op::new("arguh", [[KW, KE, KE, KE], [KX, KX, KX, KX]], false);
    optab0[O::Oargc as usize] = Op::new("argc", [[KE, KX, KE, KE], [KE, KL, KE, KE]], false);
    optab0[O::Oarge as usize] = Op::new("arge", [[KE, KL, KE, KE], [KE, KX, KE, KE]], false);
    optab0[O::Oargv as usize] = Op::new("argv", [[KX, KX, KX, KX], [KX, KX, KX, KX]], false);
    optab0[O::Ocall as usize] = Op::new("call", [[KM, KM, KM, KM], [KX, KX, KX, KX]], false);

    optab0[O::Oflagieq as usize] = Op::new("flagieq", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagine as usize] = Op::new("flagine", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagisge as usize] =
        Op::new("flagisge", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagisgt as usize] =
        Op::new("flagisgt", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagisle as usize] =
        Op::new("flagisle", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagislt as usize] =
        Op::new("flagislt", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagiuge as usize] =
        Op::new("flagiuge", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagiugt as usize] =
        Op::new("flagiugt", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagiule as usize] =
        Op::new("flagiule", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagiult as usize] =
        Op::new("flagiult", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfeq as usize] = Op::new("flagfeq", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfge as usize] = Op::new("flagfge", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfgt as usize] = Op::new("flagfgt", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfle as usize] = Op::new("flagfle", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagflt as usize] = Op::new("flagflt", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfne as usize] = Op::new("flagfne", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfo as usize] = Op::new("flagfo", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);
    optab0[O::Oflagfuo as usize] = Op::new("flagfuo", [[KX, KX, KE, KE], [KX, KX, KE, KE]], false);

    optab0
};
