use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Write;
use crate::deserialize::{self, FromSql};
use crate::pg::{Pg, PgTypeMetadata, PgValue};
use crate::query_builder::bind_collector::ByteWrapper;
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types::{Array, HasSqlType, Nullable};
#[cfg(feature = "postgres_backend")]
impl<T> HasSqlType<Array<T>> for Pg
where
    Pg: HasSqlType<T>,
{
    fn metadata(lookup: &mut Self::MetadataLookup) -> PgTypeMetadata {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<T, ST> FromSql<Array<ST>, Pg> for Vec<T>
where
    T: FromSql<ST, Pg>,
{
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
use crate::expression::bound::Bound;
use crate::expression::AsExpression;
macro_rules! array_as_expression {
    ($ty:ty, $sql_type:ty) => {
        #[cfg(feature = "postgres_backend")] #[allow(clippy::extra_unused_lifetimes)]
        impl <'a, 'b, ST : 'static, T > AsExpression <$sql_type > for $ty { type
        Expression = Bound <$sql_type, Self >; fn as_expression(self) -> Self::Expression
        { Bound::new(self) } }
    };
}
array_as_expression!(&'a[T], Array < ST >);
array_as_expression!(&'a[T], Nullable < Array < ST >>);
array_as_expression!(&'a &'b[T], Array < ST >);
array_as_expression!(&'a &'b[T], Nullable < Array < ST >>);
array_as_expression!(Vec < T >, Array < ST >);
array_as_expression!(Vec < T >, Nullable < Array < ST >>);
array_as_expression!(&'a Vec < T >, Array < ST >);
array_as_expression!(&'a Vec < T >, Nullable < Array < ST >>);
array_as_expression!(&'a &'b Vec < T >, Array < ST >);
array_as_expression!(&'a &'b Vec < T >, Nullable < Array < ST >>);
#[cfg(feature = "postgres_backend")]
impl<ST, T> ToSql<Array<ST>, Pg> for [T]
where
    Pg: HasSqlType<ST>,
    T: ToSql<ST, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<ST, T> ToSql<Nullable<Array<ST>>, Pg> for [T]
where
    [T]: ToSql<Array<ST>, Pg>,
    ST: 'static,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<ST, T> ToSql<Array<ST>, Pg> for Vec<T>
where
    ST: 'static,
    [T]: ToSql<Array<ST>, Pg>,
    T: fmt::Debug,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<ST, T> ToSql<Nullable<Array<ST>>, Pg> for Vec<T>
where
    ST: 'static,
    Vec<T>: ToSql<Array<ST>, Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
