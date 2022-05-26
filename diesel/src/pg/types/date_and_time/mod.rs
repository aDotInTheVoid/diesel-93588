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
pub struct PgTimestamp(pub i64);
#[cfg(feature = "postgres_backend")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
#[diesel(sql_type = Date)]
pub struct PgDate(pub i32);
#[cfg(feature = "postgres_backend")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
#[diesel(sql_type = Time)]
pub struct PgTime(pub i64);
#[cfg(feature = "postgres_backend")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Interval)]
pub struct PgInterval {
        pub microseconds: i64,
        pub days: i32,
        pub months: i32,
}
impl PgInterval {
                            pub fn new(microseconds: i64, days: i32, months: i32) -> Self {
        loop {}
    }
        pub fn from_microseconds(microseconds: i64) -> Self {
        loop {}
    }
        pub fn from_days(days: i32) -> Self {
        loop {}
    }
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
