use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::collections::Bound;
use std::io::Write;
use crate::deserialize::{self, FromSql, Queryable};
use crate::expression::bound::Bound as SqlBound;
use crate::expression::AsExpression;
use crate::pg::{Pg, PgTypeMetadata, PgValue};
use crate::query_builder::bind_collector::ByteWrapper;
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types::*;
bitflags::bitflags! {
    struct RangeFlags : u8 { const EMPTY = 0x01; const LB_INC = 0x02; const UB_INC =
    0x04; const LB_INF = 0x08; const UB_INF = 0x10; const LB_NULL = 0x20; const UB_NULL =
    0x40; const CONTAIN_EMPTY = 0x80; }
}
#[cfg(feature = "postgres_backend")]
impl<ST: 'static, T> AsExpression<Range<ST>> for (Bound<T>, Bound<T>) {
    type Expression = SqlBound<Range<ST>, Self>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<'a, ST: 'static, T> AsExpression<Range<ST>> for &'a (Bound<T>, Bound<T>) {
    type Expression = SqlBound<Range<ST>, Self>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<ST: 'static, T> AsExpression<Nullable<Range<ST>>> for (Bound<T>, Bound<T>) {
    type Expression = SqlBound<Nullable<Range<ST>>, Self>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<'a, ST: 'static, T> AsExpression<Nullable<Range<ST>>> for &'a (Bound<T>, Bound<T>) {
    type Expression = SqlBound<Nullable<Range<ST>>, Self>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<T, ST> FromSql<Range<ST>, Pg> for (Bound<T>, Bound<T>)
where
    T: FromSql<ST, Pg>,
{
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<T, ST> Queryable<Range<ST>, Pg> for (Bound<T>, Bound<T>)
where
    T: FromSql<ST, Pg>,
{
    type Row = Self;
    fn build(row: Self) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<ST, T> ToSql<Range<ST>, Pg> for (Bound<T>, Bound<T>)
where
    T: ToSql<ST, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<ST, T> ToSql<Nullable<Range<ST>>, Pg> for (Bound<T>, Bound<T>)
where
    ST: 'static,
    (Bound<T>, Bound<T>): ToSql<Range<ST>, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl HasSqlType<Int4range> for Pg {
    fn metadata(_: &mut Self::MetadataLookup) -> PgTypeMetadata {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl HasSqlType<Numrange> for Pg {
    fn metadata(_: &mut Self::MetadataLookup) -> PgTypeMetadata {
        loop {}
    }
}
impl HasSqlType<Tsrange> for Pg {
    fn metadata(_: &mut Self::MetadataLookup) -> PgTypeMetadata {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl HasSqlType<Tstzrange> for Pg {
    fn metadata(_: &mut Self::MetadataLookup) -> PgTypeMetadata {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl HasSqlType<Daterange> for Pg {
    fn metadata(_: &mut Self::MetadataLookup) -> PgTypeMetadata {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl HasSqlType<Int8range> for Pg {
    fn metadata(_: &mut Self::MetadataLookup) -> PgTypeMetadata {
        loop {}
    }
}
