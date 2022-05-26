pub(super) mod batch_insert;
mod column_list;
mod insert_from_select;
pub(crate) use self::batch_insert::BatchInsert;
pub(crate) use self::column_list::ColumnList;
pub(crate) use self::insert_from_select::InsertFromSelect;
pub(crate) use self::private::{Insert, InsertOrIgnore, Replace};
use super::returning_clause::*;
use crate::backend::{sql_dialect, Backend, DieselReserveSpecialization, SqlDialect};
use crate::expression::grouped::Grouped;
use crate::expression::operators::Eq;
use crate::expression::{Expression, NonAggregate, SelectableExpression};
use crate::query_builder::*;
use crate::query_dsl::RunQueryDsl;
use crate::query_source::{Column, Table};
use crate::result::QueryResult;
use crate::{insertable::*, QuerySource};
use std::marker::PhantomData;
#[cfg(feature = "sqlite")]
mod insert_with_default_for_sqlite;
#[derive(Debug, Clone, Copy)]
#[must_use = "Queries are only executed when calling `load`, `get_result` or similar."]
pub struct IncompleteInsertStatement<T, Op = Insert> {
    target: T,
    operator: Op,
}
pub type IncompleteInsertOrIgnoreStatement<T> = IncompleteInsertStatement<
    T,
    InsertOrIgnore,
>;
pub type InsertOrIgnoreStatement<T, U, Ret = NoReturningClause> = InsertStatement<
    T,
    U,
    InsertOrIgnore,
    Ret,
>;
pub type IncompleteReplaceStatement<T> = IncompleteInsertStatement<T, Replace>;
pub type ReplaceStatement<T, U, Ret = NoReturningClause> = InsertStatement<
    T,
    U,
    Replace,
    Ret,
>;
impl<T, Op> IncompleteInsertStatement<T, Op>
where
    T: QuerySource,
{
    pub(crate) fn new(target: T, operator: Op) -> Self {
        loop {}
    }
                                                                                                                                                    pub fn default_values(self) -> InsertStatement<T, DefaultValues, Op> {
        loop {}
    }
                                                pub fn values<U>(self, records: U) -> InsertStatement<T, U::Values, Op>
    where
        U: Insertable<T>,
    {
        loop {}
    }
}
#[derive(Debug, Copy, Clone)]
#[must_use = "Queries are only executed when calling `load`, `get_result` or similar."]
pub struct InsertStatement<T: QuerySource, U, Op = Insert, Ret = NoReturningClause> {
    operator: Op,
    target: T,
    records: U,
    returning: Ret,
    into_clause: T::FromClause,
}
impl<T, U, Op, Ret> QueryId for InsertStatement<T, U, Op, Ret>
where
    T: QuerySource + QueryId + 'static,
    U: QueryId,
    Op: QueryId,
    Ret: QueryId,
{
    type QueryId = InsertStatement<T, U::QueryId, Op::QueryId, Ret::QueryId>;
    const HAS_STATIC_QUERY_ID: bool = T::HAS_STATIC_QUERY_ID && U::HAS_STATIC_QUERY_ID
        && Op::HAS_STATIC_QUERY_ID && Ret::HAS_STATIC_QUERY_ID;
}
impl<T: QuerySource, U, Op, Ret> InsertStatement<T, U, Op, Ret> {
    fn new(target: T, records: U, operator: Op, returning: Ret) -> Self {
        loop {}
    }
    pub(crate) fn replace_values<F, V>(self, f: F) -> InsertStatement<T, V, Op, Ret>
    where
        F: FnOnce(U) -> V,
    {
        loop {}
    }
}
impl<T: QuerySource, U, C, Op, Ret> InsertStatement<T, InsertFromSelect<U, C>, Op, Ret> {
                        pub fn into_columns<C2>(
        self,
        columns: C2,
    ) -> InsertStatement<T, InsertFromSelect<U, C2>, Op, Ret>
    where
        C2: ColumnList<Table = T> + Expression,
        U: Query<SqlType = C2::SqlType>,
    {
        loop {}
    }
}
impl<T, U, Op, Ret, DB> QueryFragment<DB> for InsertStatement<T, U, Op, Ret>
where
    DB: Backend + DieselReserveSpecialization,
    T: Table,
    T::FromClause: QueryFragment<DB>,
    U: QueryFragment<DB> + CanInsertInSingleQuery<DB>,
    Op: QueryFragment<DB>,
    Ret: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, U, Op> AsQuery for InsertStatement<T, U, Op, NoReturningClause>
where
    T: Table,
    InsertStatement<T, U, Op, ReturningClause<T::AllColumns>>: Query,
{
    type SqlType = <Self::Query as Query>::SqlType;
    type Query = InsertStatement<T, U, Op, ReturningClause<T::AllColumns>>;
    fn as_query(self) -> Self::Query {
        loop {}
    }
}
impl<T, U, Op, Ret> Query for InsertStatement<T, U, Op, ReturningClause<Ret>>
where
    T: QuerySource,
    Ret: Expression + SelectableExpression<T> + NonAggregate,
{
    type SqlType = Ret::SqlType;
}
impl<T: QuerySource, U, Op, Ret, Conn> RunQueryDsl<Conn>
for InsertStatement<T, U, Op, Ret> {}
impl<T: QuerySource, U, Op> InsertStatement<T, U, Op> {
                                                                                        pub fn returning<E>(
        self,
        returns: E,
    ) -> InsertStatement<T, U, Op, ReturningClause<E>>
    where
        InsertStatement<T, U, Op, ReturningClause<E>>: Query,
    {
        loop {}
    }
}
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
)]
pub trait UndecoratedInsertRecord<Table> {}
impl<'a, T, Tab> UndecoratedInsertRecord<Tab> for &'a T
where
    T: ?Sized + UndecoratedInsertRecord<Tab>,
{}
impl<T, U> UndecoratedInsertRecord<T::Table> for ColumnInsertValue<T, U>
where
    T: Column,
{}
impl<T, U> UndecoratedInsertRecord<T::Table>
for DefaultableColumnInsertValue<ColumnInsertValue<T, U>>
where
    T: Column,
{}
impl<T, Table> UndecoratedInsertRecord<Table> for [T]
where
    T: UndecoratedInsertRecord<Table>,
{}
impl<T, Table, QId, const STATIC_QUERY_ID: bool> UndecoratedInsertRecord<Table>
for BatchInsert<T, Table, QId, STATIC_QUERY_ID>
where
    T: UndecoratedInsertRecord<Table>,
{}
impl<T, Table> UndecoratedInsertRecord<Table> for Vec<T>
where
    [T]: UndecoratedInsertRecord<Table>,
{}
impl<Lhs, Rhs> UndecoratedInsertRecord<Lhs::Table> for Eq<Lhs, Rhs>
where
    Lhs: Column,
{}
impl<Lhs, Rhs, Tab> UndecoratedInsertRecord<Tab> for Option<Eq<Lhs, Rhs>>
where
    Eq<Lhs, Rhs>: UndecoratedInsertRecord<Tab>,
{}
impl<Lhs, Rhs> UndecoratedInsertRecord<Lhs::Table> for Grouped<Eq<Lhs, Rhs>>
where
    Lhs: Column,
{}
impl<Lhs, Rhs, Tab> UndecoratedInsertRecord<Tab> for Option<Grouped<Eq<Lhs, Rhs>>>
where
    Eq<Lhs, Rhs>: UndecoratedInsertRecord<Tab>,
{}
impl<T, Table> UndecoratedInsertRecord<Table> for ValuesClause<T, Table>
where
    T: UndecoratedInsertRecord<Table>,
{}
#[derive(Debug, Clone, Copy, QueryId)]
#[doc(hidden)]
pub struct DefaultValues;
impl<DB: Backend> CanInsertInSingleQuery<DB> for DefaultValues {
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<Tab> Insertable<Tab> for DefaultValues {
    type Values = DefaultValues;
    fn values(self) -> Self::Values {
        loop {}
    }
}
impl<'a, Tab> Insertable<Tab> for &'a DefaultValues {
    type Values = DefaultValues;
    fn values(self) -> Self::Values {
        loop {}
    }
}
impl<DB> QueryFragment<DB> for DefaultValues
where
    DB: Backend,
    Self: QueryFragment<DB, DB::DefaultValueClauseForInsert>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<DB> QueryFragment<DB, sql_dialect::default_value_clause::AnsiDefaultValueClause>
for DefaultValues
where
    DB: Backend
        + SqlDialect<
            DefaultValueClauseForInsert = sql_dialect::default_value_clause::AnsiDefaultValueClause,
        >,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct ValuesClause<T, Tab> {
        pub values: T,
    _marker: PhantomData<Tab>,
}
impl<T: Default, Tab> Default for ValuesClause<T, Tab> {
    fn default() -> Self {
        loop {}
    }
}
impl<T, Tab> ValuesClause<T, Tab> {
    pub(crate) fn new(values: T) -> Self {
        loop {}
    }
}
impl<T, Tab, DB> CanInsertInSingleQuery<DB> for ValuesClause<T, Tab>
where
    DB: Backend,
    T: CanInsertInSingleQuery<DB>,
{
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<T, Tab, DB> QueryFragment<DB> for ValuesClause<T, Tab>
where
    DB: Backend,
    Tab: Table,
    T: InsertValues<Tab, DB>,
    DefaultValues: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
mod private {
    use crate::backend::{Backend, DieselReserveSpecialization};
    use crate::query_builder::{AstPass, QueryFragment, QueryId};
    use crate::QueryResult;
    #[derive(Debug, Copy, Clone, QueryId)]
    pub struct Insert;
    impl<DB> QueryFragment<DB> for Insert
    where
        DB: Backend + DieselReserveSpecialization,
    {
        fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
            loop {}
        }
    }
    #[derive(Debug, Copy, Clone, QueryId)]
    pub struct InsertOrIgnore;
    #[cfg(feature = "sqlite")]
    impl QueryFragment<crate::sqlite::Sqlite> for InsertOrIgnore {
        fn walk_ast<'b>(
            &'b self,
            mut out: AstPass<'_, 'b, crate::sqlite::Sqlite>,
        ) -> QueryResult<()> {
            loop {}
        }
    }
    #[cfg(feature = "mysql")]
    impl QueryFragment<crate::mysql::Mysql> for InsertOrIgnore {
        fn walk_ast<'b>(
            &'b self,
            mut out: AstPass<'_, 'b, crate::mysql::Mysql>,
        ) -> QueryResult<()> {
            loop {}
        }
    }
    #[derive(Debug, Copy, Clone, QueryId)]
    pub struct Replace;
    #[cfg(feature = "sqlite")]
    impl QueryFragment<crate::sqlite::Sqlite> for Replace {
        fn walk_ast<'b>(
            &'b self,
            mut out: AstPass<'_, 'b, crate::sqlite::Sqlite>,
        ) -> QueryResult<()> {
            loop {}
        }
    }
    #[cfg(feature = "mysql")]
    impl QueryFragment<crate::mysql::Mysql> for Replace {
        fn walk_ast<'b>(
            &'b self,
            mut out: AstPass<'_, 'b, crate::mysql::Mysql>,
        ) -> QueryResult<()> {
            loop {}
        }
    }
}
