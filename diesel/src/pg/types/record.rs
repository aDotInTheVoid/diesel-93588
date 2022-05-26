use byteorder::*;
use std::io::Write;
use std::num::NonZeroU32;
use crate::deserialize::{self, FromSql, Queryable};
use crate::expression::{
    AppearsOnTable, AsExpression, Expression, SelectableExpression, TypedExpressionType,
    ValidGrouping,
};
use crate::pg::{Pg, PgValue};
use crate::query_builder::bind_collector::ByteWrapper;
use crate::query_builder::{AstPass, QueryFragment, QueryId};
use crate::result::QueryResult;
use crate::serialize::{self, IsNull, Output, ToSql, WriteTuple};
use crate::sql_types::{HasSqlType, Record, SqlType};
macro_rules! tuple_impls {
    ($($Tuple:tt { $(($idx:tt) -> $T:ident, $ST:ident, $TT:ident,)+ })+) => {
        $(#[cfg(feature = "postgres_backend")] impl <$($T,)+ $($ST,)+> FromSql < Record <
        ($($ST,)+) >, Pg > for ($($T,)+) where $($T : FromSql <$ST, Pg >,)+ {
        #[allow(clippy::eval_order_dependence)] fn from_sql(value : PgValue <'_ >) ->
        deserialize::Result < Self > { let mut bytes = value.as_bytes(); let num_elements
        = bytes.read_i32::< NetworkEndian > () ?; if num_elements != $Tuple { return
        Err(format!("Expected a tuple of {} elements, got {}", $Tuple, num_elements,)
        .into()); } let result = ($({ let oid = NonZeroU32::new(bytes.read_u32::<
        NetworkEndian > () ?).expect("Oid's aren't zero"); let num_bytes = bytes
        .read_i32::< NetworkEndian > () ?; if num_bytes == - 1 { $T
        ::from_nullable_sql(None) ? } else { let (elem_bytes, new_bytes) = bytes
        .split_at(num_bytes as usize); bytes = new_bytes; $T
        ::from_sql(PgValue::new_internal(elem_bytes, & oid,)) ? } },)+); if bytes
        .is_empty() { Ok(result) } else {
        Err("Received too many bytes. This tuple likely contains \
                        an element of the wrong SQL type."
        .into()) } } } #[cfg(feature = "postgres_backend")] impl <$($T,)+ $($ST,)+>
        Queryable < Record < ($($ST,)+) >, Pg > for ($($T,)+) where Self : FromSql <
        Record < ($($ST,)+) >, Pg > { type Row = Self; fn build(row : Self::Row) ->
        deserialize::Result < Self > { Ok(row) } } #[cfg(feature = "postgres_backend")]
        impl <$($T,)+ $($ST,)+> AsExpression < Record < ($($ST,)+) >> for ($($T,)+) where
        $($ST : SqlType + TypedExpressionType,)+ $($T : AsExpression <$ST >,)+ PgTuple <
        ($($T ::Expression,)+) >: Expression < SqlType = Record < ($($ST,)+) >>, { type
        Expression = PgTuple < ($($T ::Expression,)+) >; fn as_expression(self) ->
        Self::Expression { PgTuple(($(self.$idx .as_expression(),)+)) } } #[cfg(feature =
        "postgres_backend")] impl <$($T,)+ $($ST,)+> WriteTuple < ($($ST,)+) > for
        ($($T,)+) where $($T : ToSql <$ST, Pg >,)+ $(Pg : HasSqlType <$ST >),+ { fn
        write_tuple(& self, out : & mut Output <'_, '_, Pg >) -> serialize::Result { let
        mut buffer = Vec::new(); out.write_i32::< NetworkEndian > ($Tuple) ?; $(let oid =
        < Pg as HasSqlType <$ST >>::metadata(out.metadata_lookup()).oid() ?; out
        .write_u32::< NetworkEndian > (oid) ?; let is_null = { let mut temp_buffer =
        Output::new(ByteWrapper(& mut buffer), out.metadata_lookup()); let is_null = self
        .$idx .to_sql(& mut temp_buffer) ?; is_null }; if let IsNull::No = is_null { out
        .write_i32::< NetworkEndian > (buffer.len() as i32) ?; out.write_all(& buffer) ?;
        buffer.clear(); } else { out.write_i32::< NetworkEndian > (- 1) ?; })+
        Ok(IsNull::No) } })+
    };
}
diesel_derives::__diesel_for_each_tuple!(tuple_impls);
#[derive(Debug, Clone, Copy, QueryId, ValidGrouping)]
pub struct PgTuple<T>(T);
impl<T> QueryFragment<Pg> for PgTuple<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
impl<T> Expression for PgTuple<T>
where
    T: Expression,
    T::SqlType: 'static,
{
    type SqlType = Record<T::SqlType>;
}
impl<T, QS> SelectableExpression<QS> for PgTuple<T>
where
    T: SelectableExpression<QS>,
    Self: AppearsOnTable<QS>,
{}
impl<T, QS> AppearsOnTable<QS> for PgTuple<T>
where
    T: AppearsOnTable<QS>,
    Self: Expression,
{}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::sql;
    use crate::prelude::*;
    use crate::sql_types::*;
    use crate::test_helpers::*;
    #[test]
    fn record_deserializes_correctly() {
        loop {}
    }
    #[test]
    fn record_kinda_sorta_not_really_serializes_correctly() {
        loop {}
    }
    #[test]
    fn serializing_named_composite_types() {
        loop {}
    }
}
