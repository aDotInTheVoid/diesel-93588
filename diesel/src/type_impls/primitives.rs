use crate::backend::{self, Backend};
use crate::deserialize::{self, FromSql, Queryable};
use crate::query_builder::bind_collector::RawBytesBindCollector;
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types::{
    self, BigInt, Binary, Bool, Double, Float, Integer, SingleValue, SmallInt, Text,
};
use std::error::Error;
use std::io::Write;
#[allow(dead_code)]
mod foreign_impls {
    use super::*;
    use crate::deserialize::FromSqlRow;
    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Bool)]
    struct BoolProxy(bool);
    #[derive(FromSqlRow)]
    #[cfg_attr(feature = "mysql_backend", derive(AsExpression))]
    #[diesel(foreign_derive)]
    #[cfg_attr(feature = "mysql_backend", diesel(sql_type = crate::sql_types::TinyInt))]
    struct I8Proxy(i8);
    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = SmallInt)]
    struct I16Proxy(i16);
    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Integer)]
    struct I32Proxy(i32);
    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = BigInt)]
    struct I64Proxy(i64);
    #[derive(FromSqlRow)]
    #[cfg_attr(feature = "mysql_backend", derive(AsExpression))]
    #[diesel(foreign_derive)]
    #[cfg_attr(
        feature = "mysql_backend",
        diesel(sql_type = crate::sql_types::Unsigned<crate::sql_types::TinyInt>)
    )]
    struct U8Proxy(u8);
    #[derive(FromSqlRow)]
    #[cfg_attr(feature = "mysql_backend", derive(AsExpression))]
    #[diesel(foreign_derive)]
    #[cfg_attr(
        feature = "mysql_backend",
        diesel(sql_type = crate::sql_types::Unsigned<SmallInt>)
    )]
    struct U16Proxy(u16);
    #[derive(FromSqlRow)]
    #[cfg_attr(
        any(feature = "mysql_backend", feature = "postgres_backend"),
        derive(AsExpression)
    )]
    #[diesel(foreign_derive)]
    #[cfg_attr(
        feature = "mysql_backend",
        diesel(sql_type = crate::sql_types::Unsigned<Integer>)
    )]
    #[cfg_attr(feature = "postgres_backend", diesel(sql_type = crate::sql_types::Oid))]
    struct U32Proxy(u32);
    #[derive(FromSqlRow)]
    #[cfg_attr(feature = "mysql_backend", derive(AsExpression))]
    #[diesel(foreign_derive)]
    #[cfg_attr(
        feature = "mysql_backend",
        diesel(sql_type = crate::sql_types::Unsigned<BigInt>)
    )]
    struct U64Proxy(u64);
    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Float)]
    struct F32Proxy(f32);
    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Double)]
    struct F64Proxy(f64);
    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Text)]
    #[cfg_attr(feature = "sqlite", diesel(sql_type = crate::sql_types::Date))]
    #[cfg_attr(feature = "sqlite", diesel(sql_type = crate::sql_types::Time))]
    #[cfg_attr(feature = "sqlite", diesel(sql_type = crate::sql_types::Timestamp))]
    struct StringProxy(String);
    #[derive(AsExpression)]
    #[diesel(foreign_derive, not_sized)]
    #[diesel(sql_type = Text)]
    #[cfg_attr(feature = "sqlite", diesel(sql_type = crate::sql_types::Date))]
    #[cfg_attr(feature = "sqlite", diesel(sql_type = crate::sql_types::Time))]
    #[cfg_attr(feature = "sqlite", diesel(sql_type = crate::sql_types::Timestamp))]
    struct StrProxy(str);
    #[derive(FromSqlRow)]
    #[diesel(foreign_derive)]
    struct VecProxy<T>(Vec<T>);
    #[derive(AsExpression)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Binary)]
    struct BinaryVecProxy(Vec<u8>);
    #[derive(AsExpression)]
    #[diesel(foreign_derive, not_sized)]
    #[diesel(sql_type = Binary)]
    struct BinarySliceProxy([u8]);
}
impl<ST, DB> FromSql<ST, DB> for String
where
    DB: Backend,
    *const str: FromSql<ST, DB>,
{
    fn from_sql(bytes: backend::RawValue<'_, DB>) -> deserialize::Result<Self> {
        loop {}
    }
}
impl<DB> ToSql<sql_types::Text, DB> for str
where
    DB: Backend<BindCollector = RawBytesBindCollector<DB>>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        loop {}
    }
}
impl<DB> ToSql<sql_types::Text, DB> for String
where
    DB: Backend,
    str: ToSql<sql_types::Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        loop {}
    }
}
impl<ST, DB> FromSql<ST, DB> for Vec<u8>
where
    DB: Backend,
    *const [u8]: FromSql<ST, DB>,
{
    fn from_sql(bytes: backend::RawValue<'_, DB>) -> deserialize::Result<Self> {
        loop {}
    }
}
impl<DB> ToSql<sql_types::Binary, DB> for Vec<u8>
where
    DB: Backend,
    [u8]: ToSql<sql_types::Binary, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        loop {}
    }
}
impl<DB> ToSql<sql_types::Binary, DB> for [u8]
where
    DB: Backend<BindCollector = RawBytesBindCollector<DB>>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        loop {}
    }
}
use std::borrow::{Cow, ToOwned};
use std::fmt;
impl<'a, T: ?Sized, ST, DB> ToSql<ST, DB> for Cow<'a, T>
where
    T: 'a + ToOwned + ToSql<ST, DB>,
    DB: Backend,
    Self: fmt::Debug,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        loop {}
    }
}
impl<'a, T: ?Sized, ST, DB> FromSql<ST, DB> for Cow<'a, T>
where
    T: 'a + ToOwned,
    DB: Backend,
    T::Owned: FromSql<ST, DB>,
{
    fn from_sql(bytes: backend::RawValue<'_, DB>) -> deserialize::Result<Self> {
        loop {}
    }
}
impl<'a, T: ?Sized, ST, DB> Queryable<ST, DB> for Cow<'a, T>
where
    T: 'a + ToOwned,
    ST: SingleValue,
    DB: Backend,
    Self: FromSql<ST, DB>,
{
    type Row = Self;
    fn build(row: Self::Row) -> deserialize::Result<Self> {
        loop {}
    }
}
use crate::expression::bound::Bound;
use crate::expression::{AsExpression, Expression, TypedExpressionType};
use sql_types::SqlType;
impl<'a, T: ?Sized, ST> AsExpression<ST> for Cow<'a, T>
where
    T: 'a + ToOwned,
    Bound<ST, Cow<'a, T>>: Expression<SqlType = ST>,
    ST: SqlType + TypedExpressionType,
{
    type Expression = Bound<ST, Self>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
impl<'a, 'b, T: ?Sized, ST> AsExpression<ST> for &'b Cow<'a, T>
where
    T: 'a + ToOwned,
    Bound<ST, &'b T>: Expression<SqlType = ST>,
    ST: SqlType + TypedExpressionType,
{
    type Expression = Bound<ST, &'b T>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
