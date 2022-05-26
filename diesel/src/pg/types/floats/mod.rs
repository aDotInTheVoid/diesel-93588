use crate::deserialize::{self, FromSql, FromSqlRow};
use crate::expression::AsExpression;
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::error::Error;
#[cfg(feature = "quickcheck")]
mod quickcheck_impls;
#[derive(Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Numeric)]
pub enum PgNumeric {
    Positive { weight: i16, scale: u16, digits: Vec<i16> },
    Negative { weight: i16, scale: u16, digits: Vec<i16> },
    NaN,
}
#[derive(Debug, Clone, Copy)]
struct InvalidNumericSign(u16);
impl ::std::fmt::Display for InvalidNumericSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        loop {}
    }
}
impl Error for InvalidNumericSign {}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Numeric, Pg> for PgNumeric {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Numeric, Pg> for PgNumeric {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Float, Pg> for f32 {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Double, Pg> for f64 {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Float, Pg> for f32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Double, Pg> for f64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
