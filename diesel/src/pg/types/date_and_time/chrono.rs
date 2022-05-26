extern crate chrono;
use self::chrono::naive::MAX_DATE;
use self::chrono::{
    DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
};
use super::{PgDate, PgTime, PgTimestamp};
use crate::deserialize::{self, FromSql};
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, Output, ToSql};
use crate::sql_types::{Date, Time, Timestamp, Timestamptz};
fn pg_epoch() -> NaiveDateTime {
    loop {}
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl FromSql<Timestamp, Pg> for NaiveDateTime {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl ToSql<Timestamp, Pg> for NaiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl FromSql<Timestamptz, Pg> for NaiveDateTime {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl ToSql<Timestamptz, Pg> for NaiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl FromSql<Timestamptz, Pg> for DateTime<Utc> {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl FromSql<Timestamptz, Pg> for DateTime<Local> {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl<TZ: TimeZone> ToSql<Timestamptz, Pg> for DateTime<TZ> {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
fn midnight() -> NaiveTime {
    loop {}
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl ToSql<Time, Pg> for NaiveTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl FromSql<Time, Pg> for NaiveTime {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
fn pg_epoch_date() -> NaiveDate {
    loop {}
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl ToSql<Date, Pg> for NaiveDate {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "postgres_backend"))]
impl FromSql<Date, Pg> for NaiveDate {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    extern crate chrono;
    extern crate dotenvy;
    use self::chrono::naive::MAX_DATE;
    use self::chrono::{Duration, FixedOffset, NaiveDate, NaiveTime, TimeZone, Utc};
    use crate::dsl::{now, sql};
    use crate::prelude::*;
    use crate::select;
    use crate::sql_types::{Date, Time, Timestamp, Timestamptz};
    use crate::test_helpers::connection;
    #[test]
    fn unix_epoch_encodes_correctly() {
        loop {}
    }
    #[test]
    fn unix_epoch_encodes_correctly_with_utc_timezone() {
        loop {}
    }
    #[test]
    fn unix_epoch_encodes_correctly_with_timezone() {
        loop {}
    }
    #[test]
    fn unix_epoch_decodes_correctly() {
        loop {}
    }
    #[test]
    fn unix_epoch_decodes_correctly_with_timezone() {
        loop {}
    }
    #[test]
    fn times_relative_to_now_encode_correctly() {
        loop {}
    }
    #[test]
    fn times_with_timezones_round_trip_after_conversion() {
        loop {}
    }
    #[test]
    fn times_of_day_encode_correctly() {
        loop {}
    }
    #[test]
    fn times_of_day_decode_correctly() {
        loop {}
    }
    #[test]
    fn dates_encode_correctly() {
        loop {}
    }
    #[test]
    fn dates_decode_correctly() {
        loop {}
    }
}
