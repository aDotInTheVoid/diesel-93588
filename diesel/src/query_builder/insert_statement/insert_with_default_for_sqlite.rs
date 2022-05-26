use super::{BatchInsert, InsertStatement};
use crate::connection::Connection;
use crate::insertable::InsertValues;
use crate::insertable::{CanInsertInSingleQuery, ColumnInsertValue, DefaultableColumnInsertValue};
use crate::prelude::*;
use crate::query_builder::{AstPass, QueryId, ValuesClause};
use crate::query_builder::{DebugQuery, QueryFragment};
use crate::query_dsl::methods::ExecuteDsl;
use crate::sqlite::Sqlite;
use crate::{QueryResult, Table};
use std::fmt::{self, Debug, Display};
pub trait DebugQueryHelper<ContainsDefaultableValue> {
    fn fmt_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    fn fmt_display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl<'a, T, V, QId, Op, Ret, const STATIC_QUERY_ID: bool> DebugQueryHelper<Yes>
    for DebugQuery<
        'a,
        InsertStatement<T, BatchInsert<Vec<ValuesClause<V, T>>, T, QId, STATIC_QUERY_ID>, Op, Ret>,
        Sqlite,
    >
where
    V: QueryFragment<Sqlite>,
    T: Copy + QuerySource,
    Op: Copy,
    Ret: Copy,
    for<'b> InsertStatement<T, &'b ValuesClause<V, T>, Op, Ret>: QueryFragment<Sqlite>,
{
    fn fmt_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
    fn fmt_display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<'a, T, V, QId, Op, const STATIC_QUERY_ID: bool> DebugQueryHelper<No>
    for DebugQuery<'a, InsertStatement<T, BatchInsert<V, T, QId, STATIC_QUERY_ID>, Op>, Sqlite>
where
    T: Copy + QuerySource,
    Op: Copy,
    DebugQuery<
        'a,
        InsertStatement<T, SqliteBatchInsertWrapper<V, T, QId, STATIC_QUERY_ID>, Op>,
        Sqlite,
    >: Debug + Display,
{
    fn fmt_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
    fn fmt_display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<'a, T, V, QId, Op, O, const STATIC_QUERY_ID: bool> Display
    for DebugQuery<
        'a,
        InsertStatement<T, BatchInsert<Vec<ValuesClause<V, T>>, T, QId, STATIC_QUERY_ID>, Op>,
        Sqlite,
    >
where
    T: QuerySource,
    V: ContainsDefaultableValue<Out = O>,
    Self: DebugQueryHelper<O>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<'a, T, V, QId, Op, O, const STATIC_QUERY_ID: bool> Debug
    for DebugQuery<
        'a,
        InsertStatement<T, BatchInsert<Vec<ValuesClause<V, T>>, T, QId, STATIC_QUERY_ID>, Op>,
        Sqlite,
    >
where
    T: QuerySource,
    V: ContainsDefaultableValue<Out = O>,
    Self: DebugQueryHelper<O>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Yes;
impl Default for Yes {
    fn default() -> Self {
        loop {}
    }
}
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct No;
impl Default for No {
    fn default() -> Self {
        loop {}
    }
}
pub trait Any<Rhs> {
    type Out: Any<Yes> + Any<No>;
}
impl Any<No> for No {
    type Out = No;
}
impl Any<Yes> for No {
    type Out = Yes;
}
impl Any<No> for Yes {
    type Out = Yes;
}
impl Any<Yes> for Yes {
    type Out = Yes;
}
pub trait ContainsDefaultableValue {
    type Out: Any<Yes> + Any<No>;
}
impl<C, B> ContainsDefaultableValue for ColumnInsertValue<C, B> {
    type Out = No;
}
impl<I> ContainsDefaultableValue for DefaultableColumnInsertValue<I> {
    type Out = Yes;
}
impl<I, const SIZE: usize> ContainsDefaultableValue for [I; SIZE]
where
    I: ContainsDefaultableValue,
{
    type Out = I::Out;
}
impl<I, T> ContainsDefaultableValue for ValuesClause<I, T>
where
    I: ContainsDefaultableValue,
{
    type Out = I::Out;
}
impl<'a, T> ContainsDefaultableValue for &'a T
where
    T: ContainsDefaultableValue,
{
    type Out = T::Out;
}
impl<V, T, QId, C, Op, O, const STATIC_QUERY_ID: bool> ExecuteDsl<C, Sqlite>
    for InsertStatement<T, BatchInsert<Vec<ValuesClause<V, T>>, T, QId, STATIC_QUERY_ID>, Op>
where
    T: QuerySource,
    C: Connection<Backend = Sqlite>,
    V: ContainsDefaultableValue<Out = O>,
    O: Default,
    (O, Self): ExecuteDsl<C, Sqlite>,
{
    fn execute(query: Self, conn: &mut C) -> QueryResult<usize> {
        loop {}
    }
}
impl<V, T, QId, C, Op, const STATIC_QUERY_ID: bool> ExecuteDsl<C, Sqlite>
    for (
        Yes,
        InsertStatement<T, BatchInsert<Vec<ValuesClause<V, T>>, T, QId, STATIC_QUERY_ID>, Op>,
    )
where
    C: Connection<Backend = Sqlite>,
    T: Table + Copy + QueryId + 'static,
    T::FromClause: QueryFragment<Sqlite>,
    Op: Copy + QueryId + QueryFragment<Sqlite>,
    V: InsertValues<T, Sqlite> + CanInsertInSingleQuery<Sqlite> + QueryId,
{
    fn execute((Yes, query): Self, conn: &mut C) -> QueryResult<usize> {
        loop {}
    }
}
#[allow(missing_debug_implementations, missing_copy_implementations)]
#[repr(transparent)]
pub struct SqliteBatchInsertWrapper<V, T, QId, const STATIC_QUERY_ID: bool>(
    BatchInsert<V, T, QId, STATIC_QUERY_ID>,
);
impl<V, Tab, QId, const STATIC_QUERY_ID: bool> QueryFragment<Sqlite>
    for SqliteBatchInsertWrapper<Vec<ValuesClause<V, Tab>>, Tab, QId, STATIC_QUERY_ID>
where
    ValuesClause<V, Tab>: QueryFragment<Sqlite>,
    V: QueryFragment<Sqlite>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Sqlite>) -> QueryResult<()> {
        loop {}
    }
}
#[allow(missing_copy_implementations, missing_debug_implementations)]
#[repr(transparent)]
pub struct SqliteCanInsertInSingleQueryHelper<T: ?Sized>(T);
impl<V, T, QId, const STATIC_QUERY_ID: bool> CanInsertInSingleQuery<Sqlite>
    for SqliteBatchInsertWrapper<Vec<ValuesClause<V, T>>, T, QId, STATIC_QUERY_ID>
where
    SqliteCanInsertInSingleQueryHelper<V>: CanInsertInSingleQuery<Sqlite>,
{
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<T> CanInsertInSingleQuery<Sqlite> for SqliteCanInsertInSingleQueryHelper<T>
where
    T: CanInsertInSingleQuery<Sqlite>,
{
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<V, T, QId, const STATIC_QUERY_ID: bool> QueryId
    for SqliteBatchInsertWrapper<V, T, QId, STATIC_QUERY_ID>
where
    BatchInsert<V, T, QId, STATIC_QUERY_ID>: QueryId,
{
    type QueryId = <BatchInsert<V, T, QId, STATIC_QUERY_ID> as QueryId>::QueryId;
    const HAS_STATIC_QUERY_ID: bool =
        <BatchInsert<V, T, QId, STATIC_QUERY_ID> as QueryId>::HAS_STATIC_QUERY_ID;
}
impl<V, T, QId, C, Op, const STATIC_QUERY_ID: bool> ExecuteDsl<C, Sqlite>
    for (
        No,
        InsertStatement<T, BatchInsert<V, T, QId, STATIC_QUERY_ID>, Op>,
    )
where
    C: Connection<Backend = Sqlite>,
    T: Table + QueryId + 'static,
    T::FromClause: QueryFragment<Sqlite>,
    Op: QueryFragment<Sqlite> + QueryId,
    SqliteBatchInsertWrapper<V, T, QId, STATIC_QUERY_ID>:
        QueryFragment<Sqlite> + QueryId + CanInsertInSingleQuery<Sqlite>,
{
    fn execute((No, query): Self, conn: &mut C) -> QueryResult<usize> {
        loop {}
    }
}
macro_rules! tuple_impls {
    ($($Tuple:tt { $(($idx:tt) -> $T:ident, $ST:ident, $TT:ident,)+ })+) => {
        $(impl_contains_defaultable_value!($($T,)*);)*
    };
}
macro_rules! impl_contains_defaultable_value {
    ($T1:ident, $($T:ident,)+) => {
        impl <$T1, $($T,)*> ContainsDefaultableValue for ($T1, $($T,)*) where $T1 :
        ContainsDefaultableValue, ($($T,)*) : ContainsDefaultableValue, $T1 ::Out : Any
        << ($($T,)*) as ContainsDefaultableValue >::Out > { type Out = <$T1 ::Out as Any
        << ($($T,)*) as ContainsDefaultableValue >::Out >>::Out; }
    };
    ($T1:ident,) => {
        impl <$T1 > ContainsDefaultableValue for ($T1,) where $T1 :
        ContainsDefaultableValue, { type Out = <$T1 as ContainsDefaultableValue >::Out; }
    };
}
diesel_derives::__diesel_for_each_tuple!(tuple_impls);
