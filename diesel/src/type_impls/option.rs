use crate::backend::{self, Backend};
use crate::deserialize::{self, FromSql, Queryable, QueryableByName};
use crate::expression::bound::Bound;
use crate::expression::*;
use crate::query_builder::QueryId;
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types::{is_nullable, HasSqlType, Nullable, SingleValue, SqlType};
use crate::NullableExpressionMethods;
impl<T, DB> HasSqlType<Nullable<T>> for DB
where
    DB: Backend + HasSqlType<T>,
    T: SqlType,
{
    fn metadata(lookup: &mut DB::MetadataLookup) -> DB::TypeMetadata {
        loop {}
    }
}
impl<T> QueryId for Nullable<T>
where
    T: QueryId + SqlType<IsNull = is_nullable::NotNull>,
{
    type QueryId = T::QueryId;
    const HAS_STATIC_QUERY_ID: bool = T::HAS_STATIC_QUERY_ID;
}
impl<T, ST, DB> FromSql<Nullable<ST>, DB> for Option<T>
where
    T: FromSql<ST, DB>,
    DB: Backend,
    ST: SqlType<IsNull = is_nullable::NotNull>,
{
    fn from_sql(bytes: backend::RawValue<'_, DB>) -> deserialize::Result<Self> {
        loop {}
    }
    fn from_nullable_sql(
        bytes: Option<backend::RawValue<'_, DB>>,
    ) -> deserialize::Result<Self> {
        loop {}
    }
}
impl<T, ST, DB> ToSql<Nullable<ST>, DB> for Option<T>
where
    T: ToSql<ST, DB>,
    DB: Backend,
    ST: SqlType<IsNull = is_nullable::NotNull>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        loop {}
    }
}
impl<T, ST> AsExpression<Nullable<ST>> for Option<T>
where
    ST: SqlType<IsNull = is_nullable::NotNull>,
    Nullable<ST>: TypedExpressionType,
{
    type Expression = Bound<Nullable<ST>, Self>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
impl<'a, T, ST> AsExpression<Nullable<ST>> for &'a Option<T>
where
    ST: SqlType<IsNull = is_nullable::NotNull>,
    Nullable<ST>: TypedExpressionType,
{
    type Expression = Bound<Nullable<ST>, Self>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
impl<T, DB> QueryableByName<DB> for Option<T>
where
    DB: Backend,
    T: QueryableByName<DB>,
{
    fn build<'a>(row: &impl crate::row::NamedRow<'a, DB>) -> deserialize::Result<Self> {
        loop {}
    }
}
impl<ST, T, DB> Queryable<ST, DB> for Option<T>
where
    ST: SingleValue<IsNull = is_nullable::IsNullable>,
    DB: Backend,
    Self: FromSql<ST, DB>,
{
    type Row = Self;
    fn build(row: Self::Row) -> deserialize::Result<Self> {
        loop {}
    }
}
impl<T, DB> Selectable<DB> for Option<T>
where
    DB: Backend,
    T: Selectable<DB>,
    crate::dsl::Nullable<T::SelectExpression>: Expression,
{
    type SelectExpression = crate::dsl::Nullable<T::SelectExpression>;
    fn construct_selection() -> Self::SelectExpression {
        loop {}
    }
}
#[test]
#[cfg(feature = "postgres")]
fn option_to_sql() {
    loop {}
}
