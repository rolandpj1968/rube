use crate::all::{isargbh, isparbh, isretbh, BlkIdx, Blks, Fn, J, O};

/* eliminate sub-word abi op
 * variants for targets that
 * treat char/short/... as
 * words with arbitrary high
 * bits
 */
// TODO - f now does now need to be mut here (interior mutability)
pub fn elimsb(f: &mut Fn) {
    let blks: &Blks = &f.blks;

    let mut bi: BlkIdx = f.start;
    while bi != BlkIdx::NONE {
        blks.with_mut(bi, |b| {
            for i in b.ins_mut().iter_mut() {
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
