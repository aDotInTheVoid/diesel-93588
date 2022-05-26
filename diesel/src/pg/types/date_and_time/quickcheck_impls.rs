extern crate quickcheck;
use self::quickcheck::{Arbitrary, Gen};
use super::{PgDate, PgInterval, PgTime, PgTimestamp};
impl Arbitrary for PgDate {
    fn arbitrary(g: &mut Gen) -> Self {
        loop {}
    }
}
impl Arbitrary for PgTime {
    fn arbitrary(g: &mut Gen) -> Self {
        loop {}
    }
}
impl Arbitrary for PgTimestamp {
    fn arbitrary(g: &mut Gen) -> Self {
        loop {}
    }
}
impl Arbitrary for PgInterval {
    fn arbitrary(g: &mut Gen) -> Self {
        loop {}
    }
}
