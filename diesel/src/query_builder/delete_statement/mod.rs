use super::from_clause::FromClause;
use crate::backend::{Backend, DieselReserveSpecialization};
use crate::dsl::{Filter, IntoBoxed, OrFilter};
use crate::expression::{AppearsOnTable, SelectableExpression};
use crate::query_builder::returning_clause::*;
use crate::query_builder::where_clause::*;
use crate::query_builder::*;
use crate::query_dsl::methods::{BoxedDsl, FilterDsl, OrFilterDsl};
use crate::query_dsl::RunQueryDsl;
use crate::query_source::{QuerySource, Table};
use crate::result::QueryResult;
#[must_use = "Queries are only executed when calling `load`, `get_result` or similar."]
pub struct DeleteStatement<T: QuerySource, U, Ret = NoReturningClause> {
    from_clause: FromClause<T>,
    where_clause: U,
    returning: Ret,
}
impl<T, U, Ret> Clone for DeleteStatement<T, U, Ret>
where
    T: QuerySource,
    FromClause<T>: Clone,
    U: Clone,
    Ret: Clone,
{
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<T, U, Ret> std::fmt::Debug for DeleteStatement<T, U, Ret>
where
    T: QuerySource,
    FromClause<T>: std::fmt::Debug,
    U: std::fmt::Debug,
    Ret: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
impl<T, U, Ret> QueryId for DeleteStatement<T, U, Ret>
where
    T: QuerySource + QueryId + 'static,
    U: QueryId,
    Ret: QueryId,
{
    type QueryId = DeleteStatement<T, U::QueryId, Ret::QueryId>;
    const HAS_STATIC_QUERY_ID: bool = T::HAS_STATIC_QUERY_ID && U::HAS_STATIC_QUERY_ID
        && Ret::HAS_STATIC_QUERY_ID;
}
pub type BoxedDeleteStatement<'a, DB, T, Ret = NoReturningClause> = DeleteStatement<
    T,
    BoxedWhereClause<'a, DB>,
    Ret,
>;
impl<T: QuerySource, U> DeleteStatement<T, U, NoReturningClause> {
    pub(crate) fn new(table: T, where_clause: U) -> Self {
        loop {}
    }
    pub fn filter<Predicate>(self, predicate: Predicate) -> Filter<Self, Predicate>
    where
        Self: FilterDsl<Predicate>,
    {
        loop {}
    }
    pub fn or_filter<Predicate>(self, predicate: Predicate) -> OrFilter<Self, Predicate>
    where
        Self: OrFilterDsl<Predicate>,
    {
        loop {}
    }
    pub fn into_boxed<'a, DB>(self) -> IntoBoxed<'a, Self, DB>
    where
        DB: Backend,
        Self: BoxedDsl<'a, DB>,
    {
        loop {}
    }
}
impl<T, U, Ret, Predicate> FilterDsl<Predicate> for DeleteStatement<T, U, Ret>
where
    U: WhereAnd<Predicate>,
    Predicate: AppearsOnTable<T>,
    T: QuerySource,
{
    type Output = DeleteStatement<T, U::Output, Ret>;
    fn filter(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
impl<T, U, Ret, Predicate> OrFilterDsl<Predicate> for DeleteStatement<T, U, Ret>
where
    T: QuerySource,
    U: WhereOr<Predicate>,
    Predicate: AppearsOnTable<T>,
{
    type Output = DeleteStatement<T, U::Output, Ret>;
    fn or_filter(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
impl<'a, T, U, Ret, DB> BoxedDsl<'a, DB> for DeleteStatement<T, U, Ret>
where
    U: Into<BoxedWhereClause<'a, DB>>,
    T: QuerySource,
{
    type Output = BoxedDeleteStatement<'a, DB, T, Ret>;
    fn internal_into_boxed(self) -> Self::Output {
        loop {}
    }
}
impl<T, U, Ret, DB> QueryFragment<DB> for DeleteStatement<T, U, Ret>
where
    DB: Backend + DieselReserveSpecialization,
    T: Table,
    FromClause<T>: QueryFragment<DB>,
    U: QueryFragment<DB>,
    Ret: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, U> AsQuery for DeleteStatement<T, U, NoReturningClause>
where
    T: Table,
    T::AllColumns: SelectableExpression<T>,
    DeleteStatement<T, U, ReturningClause<T::AllColumns>>: Query,
{
    type SqlType = <Self::Query as Query>::SqlType;
    type Query = DeleteStatement<T, U, ReturningClause<T::AllColumns>>;
    fn as_query(self) -> Self::Query {
        loop {}
    }
}
impl<T, U, Ret> Query for DeleteStatement<T, U, ReturningClause<Ret>>
where
    T: Table,
    Ret: SelectableExpression<T>,
{
    type SqlType = Ret::SqlType;
}
impl<T, U, Ret, Conn> RunQueryDsl<Conn> for DeleteStatement<T, U, Ret>
where
    T: QuerySource,
{}
impl<T: QuerySource, U> DeleteStatement<T, U, NoReturningClause> {
    pub fn returning<E>(self, returns: E) -> DeleteStatement<T, U, ReturningClause<E>>
    where
        E: SelectableExpression<T>,
        DeleteStatement<T, U, ReturningClause<E>>: Query,
    {
        loop {}
    }
}
