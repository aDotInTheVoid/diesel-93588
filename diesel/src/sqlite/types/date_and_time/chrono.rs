extern crate chrono;
use self::chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use crate::backend;
use crate::deserialize::{self, FromSql};
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types::{Date, Time, Timestamp, TimestamptzSqlite};
use crate::sqlite::Sqlite;
const SQLITE_DATE_FORMAT: &str = "%F";
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl FromSql<Date, Sqlite> for NaiveDate {
    fn from_sql(value: backend::RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl ToSql<Date, Sqlite> for NaiveDate {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl FromSql<Time, Sqlite> for NaiveTime {
    fn from_sql(value: backend::RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl ToSql<Time, Sqlite> for NaiveTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl FromSql<Timestamp, Sqlite> for NaiveDateTime {
    fn from_sql(value: backend::RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl ToSql<Timestamp, Sqlite> for NaiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl FromSql<TimestamptzSqlite, Sqlite> for NaiveDateTime {
    fn from_sql(value: backend::RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl ToSql<TimestamptzSqlite, Sqlite> for NaiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl FromSql<TimestamptzSqlite, Sqlite> for DateTime<Utc> {
    fn from_sql(value: backend::RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl FromSql<TimestamptzSqlite, Sqlite> for DateTime<Local> {
    fn from_sql(value: backend::RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "sqlite", feature = "chrono"))]
impl<TZ: TimeZone> ToSql<TimestamptzSqlite, Sqlite> for DateTime<TZ> {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    extern crate chrono;
    extern crate dotenvy;
    use self::chrono::{
        DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone,
        Timelike, Utc,
    };
    use crate::dsl::{now, sql};
    use crate::prelude::*;
    use crate::select;
    use crate::sql_types::{Text, Time, Timestamp, TimestamptzSqlite};
    use crate::test_helpers::connection;
    sql_function!(fn datetime(x : Text) -> Timestamp);
    sql_function!(fn time(x : Text) -> Time);
    sql_function!(fn date(x : Text) -> Date);
    #[test]
    fn unix_epoch_encodes_correctly() {
        loop {}
    }
    #[test]
    fn unix_epoch_decodes_correctly_in_all_possible_formats() {
        loop {}
    }
    #[test]
    fn times_relative_to_now_encode_correctly() {
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
    #[test]
    fn datetimes_decode_correctly() {
        loop {}
    }
    #[test]
    fn datetimes_encode_correctly() {
        loop {}
    }
    #[test]
    fn insert_timestamptz_into_table_as_text() {
        loop {}
    }
    #[test]
    fn can_query_timestamptz_column_with_between() {
        loop {}
    }
    #[test]
    fn unix_epoch_encodes_correctly_with_timezone() {
        loop {}
    }
    #[test]
    fn unix_epoch_encodes_correctly_with_utc_timezone() {
        loop {}
    }
    #[test]
    fn unix_epoch_decodes_correctly_with_utc_timezone_in_all_possible_formats() {
        loop {}
    }
}
