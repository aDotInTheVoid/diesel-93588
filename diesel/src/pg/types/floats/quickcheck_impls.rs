extern crate quickcheck;
use self::quickcheck::{Arbitrary, Gen};
use super::PgNumeric;
const SCALE_MASK: u16 = 0x3FFF;
impl Arbitrary for PgNumeric {
    fn arbitrary(g: &mut Gen) -> Self {
        loop {}
    }
}
fn gen_vec_of_appropriate_length_valid_digits(
    g: &mut Gen,
    weight: u16,
    scale: u16,
) -> Vec<i16> {
    loop {}
}
#[derive(Debug, Clone, Copy)]
struct Digit(i16);
impl Arbitrary for Digit {
    fn arbitrary(g: &mut Gen) -> Self {
        loop {}
    }
}
