use std::ops::Add;
use crate::deserialize::{self, FromSql, FromSqlRow};
use crate::expression::AsExpression;
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types::{self, Date, Interval, Time, Timestamp, Timestamptz};
#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "quickcheck")]
mod quickcheck_impls;
mod std_time;
#[cfg(feature = "postgres_backend")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
#[diesel(sql_type = Timestamp)]
#[diesel(sql_type = Timestamptz)]
/// Timestamps are represented in Postgres as a 64 bit signed integer representing the number of
/// microseconds since January 1st 2000. This struct is a dumb wrapper type, meant only to indicate
/// the integer's meaning.
pub struct PgTimestamp(pub i64);
#[cfg(feature = "postgres_backend")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
#[diesel(sql_type = Date)]
/// Dates are represented in Postgres as a 32 bit signed integer representing the number of julian
/// days since January 1st 2000. This struct is a dumb wrapper type, meant only to indicate the
/// integer's meaning.
pub struct PgDate(pub i32);
/// Time is represented in Postgres as a 64 bit signed integer representing the number of
/// microseconds since midnight. This struct is a dumb wrapper type, meant only to indicate the
/// integer's meaning.
#[cfg(feature = "postgres_backend")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
#[diesel(sql_type = Time)]
pub struct PgTime(pub i64);
/// Intervals in Postgres are separated into 3 parts. A 64 bit integer representing time in
/// microseconds, a 32 bit integer representing number of days, and a 32 bit integer
/// representing number of months. This struct is a dumb wrapper type, meant only to indicate the
/// meaning of these parts.
#[cfg(feature = "postgres_backend")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Interval)]
pub struct PgInterval {
    /// The number of whole microseconds
    pub microseconds: i64,
    /// The number of whole days
    pub days: i32,
    /// The number of whole months
    pub months: i32,
}
impl PgInterval {
    /// Constructs a new `PgInterval`
    ///
    /// No conversion occurs on the arguments. It is valid to provide a number
    /// of microseconds greater than the longest possible day, or a number of
    /// days greater than the longest possible month, as it is impossible to say
    /// how many months are in "40 days" without knowing a precise date.
    pub fn new(microseconds: i64, days: i32, months: i32) -> Self {
        loop {}
    }
    /// Equivalent to `new(microseconds, 0, 0)`
    pub fn from_microseconds(microseconds: i64) -> Self {
        loop {}
    }
    /// Equivalent to `new(0, days, 0)`
    pub fn from_days(days: i32) -> Self {
        loop {}
    }
    /// Equivalent to `new(0, 0, months)`
    pub fn from_months(months: i32) -> Self {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Timestamp, Pg> for PgTimestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Timestamp, Pg> for PgTimestamp {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Timestamptz, Pg> for PgTimestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Timestamptz, Pg> for PgTimestamp {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Date, Pg> for PgDate {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Date, Pg> for PgDate {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Time, Pg> for PgTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Time, Pg> for PgTime {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Interval, Pg> for PgInterval {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Interval, Pg> for PgInterval {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
impl Add<PgInterval> for PgInterval {
    type Output = PgInterval;
    fn add(self, other: PgInterval) -> Self::Output {
        loop {}
    }
}
