use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::deserialize::{self, FromSql};
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, Output, ToSql};
use crate::sql_types;
fn pg_epoch() -> SystemTime {
    loop {}
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Timestamp, Pg> for SystemTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Timestamp, Pg> for SystemTime {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
const USEC_PER_SEC: u64 = 1_000_000;
const NANO_PER_USEC: u32 = 1_000;
fn usecs_to_duration(usecs_passed: u64) -> Duration {
    loop {}
}
fn duration_to_usecs(duration: Duration) -> u64 {
    loop {}
}
#[cfg(test)]
mod tests {
    extern crate dotenvy;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use crate::dsl::{now, sql};
    use crate::prelude::*;
    use crate::select;
    use crate::sql_types::Timestamp;
    use crate::test_helpers::pg_connection;
    #[test]
    fn unix_epoch_encodes_correctly() {
        loop {}
    }
    #[test]
    fn unix_epoch_decodes_correctly() {
        loop {}
    }
    #[test]
    fn times_relative_to_now_encode_correctly() {
        loop {}
    }
}
