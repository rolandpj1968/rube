use crate::all::{for_each_blk_mut, isargbh, isparbh, isretbh, Fn, J, O};

/* eliminate sub-word abi op
 * variants for targets that
 * treat char/short/... as
 * words with arbitrary high
 * bits
 */
// TODO - f now does now need to be mut here (interior mutability)
pub fn elimsb(f: &mut Fn) {
    for_each_blk_mut(&mut f.blks, |b| {
        for i in &mut b.ins {
            if isargbh(i.op) {
                i.op = O::Oarg;
            } else if isparbh(i.op) {
                i.op = O::Opar;
            }
        }
        if isretbh(b.jmp.typ) {
            b.jmp.typ = J::Jretw;
        }
    });
}
