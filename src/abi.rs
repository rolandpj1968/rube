use crate::all::{isargbh, isparbh, isretbh, Blk, BlkIdx, Blks, Fn, J, O};

/* eliminate sub-word abi op
 * variants for targets that
 * treat char/short/... as
 * words with arbitrary high
 * bits
 */
pub fn elimsb(f: &Fn) {
    let blks: &Blks = &f.blks;

    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        blks.with_mut(bi, |b| {
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
        });
    }
}
