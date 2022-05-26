use crate::pg::Pg;
use crate::serialize::{self, Output};
#[cfg(feature = "postgres_backend")]
pub trait WriteTuple<ST> {
        fn write_tuple(&self, out: &mut Output<'_, '_, Pg>) -> serialize::Result;
}
