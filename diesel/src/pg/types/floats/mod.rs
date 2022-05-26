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
/// Represents a NUMERIC value, closely mirroring the PG wire protocol
/// representation
pub enum PgNumeric {
    /// A positive number
    Positive {
        /// How many digits come before the decimal point?
        weight: i16,
        /// How many significant digits are there?
        scale: u16,
        /// The digits in this number, stored in base 10000
        digits: Vec<i16>,
    },
    /// A negative number
    Negative {
        /// How many digits come before the decimal point?
        weight: i16,
        /// How many significant digits are there?
        scale: u16,
        /// The digits in this number, stored in base 10000
        digits: Vec<i16>,
    },
    /// Not a number
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
