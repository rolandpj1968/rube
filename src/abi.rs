use crate::all::{isargbh, isparbh, isretbh, Fn, J, O};

/* eliminate sub-word abi op
 * variants for targets that
 * treat char/short/... as
 * words with arbitrary high
 * bits
 */
// TODO - f now does now need to be mut here (interior mutability)
pub fn elimsb(f: &mut Fn) {
    f.blks.for_each_mut(|b| {
        for i in b.ins_mut().iter_mut() {
            if isargbh(i.op) {
                i.op = O::Oarg;
            } else if isparbh(i.op) {
                i.op = O::Opar;
            }
        }
        if isretbh(b.jmp().typ) {
            b.jmp_mut().typ = J::Jretw;
        }
    });
}
