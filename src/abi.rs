use crate::all::{isargbh, isparbh, isretbh, Blk, BlkIdx, Fn, J, O};

/* eliminate sub-word abi op
 * variants for targets that
 * treat char/short/... as
 * words with arbitrary high
 * bits
 */
pub fn elimsb(f: &mut Fn) {
    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        let b: &mut Blk = &mut f.blks[bi];
        for i in &mut b.ins {
            if isargbh(i.op) {
                i.op = O::Oarg;
            } else if isparbh(i.op) {
                i.op = O::Opar;
            }
        }
        if isretbh(b.jmp.type_) {
            b.jmp.type_ = J::Jretw;
        }

        bi = b.link;
    }
}
