pub(crate) mod changeset;
pub(super) mod target;
use self::target::UpdateTarget;
use crate::backend::{Backend, DieselReserveSpecialization};
use crate::dsl::{Filter, IntoBoxed};
use crate::expression::{
    is_aggregate, AppearsOnTable, Expression, MixedAggregates, SelectableExpression, ValidGrouping,
};
use crate::query_builder::returning_clause::*;
use crate::query_builder::where_clause::*;
use crate::query_dsl::methods::{BoxedDsl, FilterDsl};
use crate::query_dsl::RunQueryDsl;
use crate::query_source::Table;
use crate::result::Error::QueryBuilderError;
use crate::result::QueryResult;
use crate::{query_builder::*, QuerySource};
impl<T: QuerySource, U> UpdateStatement<T, U, SetNotCalled> {
    pub(crate) fn new(target: UpdateTarget<T, U>) -> Self {
        loop {}
    }
    pub fn set<V>(self, values: V) -> UpdateStatement<T, U, V::Changeset>
    where
        T: Table,
        V: changeset::AsChangeset<Target = T>,
        UpdateStatement<T, U, V::Changeset>: AsQuery,
    {
        loop {}
    }
}
#[derive(Clone, Debug)]
#[must_use = "Queries are only executed when calling `load`, `get_result` or similar."]
pub struct UpdateStatement<T: QuerySource, U, V = SetNotCalled, Ret = NoReturningClause> {
    from_clause: T::FromClause,
    where_clause: U,
    values: V,
    returning: Ret,
}
pub type BoxedUpdateStatement<'a, DB, T, V = SetNotCalled, Ret = NoReturningClause> =
    UpdateStatement<T, BoxedWhereClause<'a, DB>, V, Ret>;
impl<T: QuerySource, U, V, Ret> UpdateStatement<T, U, V, Ret> {
    pub fn filter<Predicate>(self, predicate: Predicate) -> Filter<Self, Predicate>
    where
        Self: FilterDsl<Predicate>,
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
impl<T, U, V, Ret, Predicate> FilterDsl<Predicate> for UpdateStatement<T, U, V, Ret>
where
    T: QuerySource,
    U: WhereAnd<Predicate>,
    Predicate: AppearsOnTable<T>,
{
    type Output = UpdateStatement<T, U::Output, V, Ret>;
    fn filter(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
impl<'a, T, U, V, Ret, DB> BoxedDsl<'a, DB> for UpdateStatement<T, U, V, Ret>
where
    T: QuerySource,
    U: Into<BoxedWhereClause<'a, DB>>,
{
    type Output = BoxedUpdateStatement<'a, DB, T, V, Ret>;
    fn internal_into_boxed(self) -> Self::Output {
        loop {}
    }
}
impl<T, U, V, Ret, DB> QueryFragment<DB> for UpdateStatement<T, U, V, Ret>
where
    DB: Backend + DieselReserveSpecialization,
    T: Table,
    T::FromClause: QueryFragment<DB>,
    U: QueryFragment<DB>,
    V: QueryFragment<DB>,
    Ret: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, U, V, Ret> QueryId for UpdateStatement<T, U, V, Ret>
where
    T: QuerySource,
{
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}
impl<T, U, V> AsQuery for UpdateStatement<T, U, V, NoReturningClause>
where
    T: Table,
    UpdateStatement<T, U, V, ReturningClause<T::AllColumns>>: Query,
    T::AllColumns: ValidGrouping<()>,
    <T::AllColumns as ValidGrouping<()>>::IsAggregate:
        MixedAggregates<is_aggregate::No, Output = is_aggregate::No>,
{
    type SqlType = <Self::Query as Query>::SqlType;
    type Query = UpdateStatement<T, U, V, ReturningClause<T::AllColumns>>;
    fn as_query(self) -> Self::Query {
        loop {}
    }
}
impl<T, U, V, Ret> Query for UpdateStatement<T, U, V, ReturningClause<Ret>>
where
    T: Table,
    Ret: Expression + SelectableExpression<T> + ValidGrouping<()>,
    Ret::IsAggregate: MixedAggregates<is_aggregate::No, Output = is_aggregate::No>,
{
    type SqlType = Ret::SqlType;
}
impl<T: QuerySource, U, V, Ret, Conn> RunQueryDsl<Conn> for UpdateStatement<T, U, V, Ret> {}
impl<T: QuerySource, U, V> UpdateStatement<T, U, V, NoReturningClause> {
    pub fn returning<E>(self, returns: E) -> UpdateStatement<T, U, V, ReturningClause<E>>
    where
        T: Table,
        UpdateStatement<T, U, V, ReturningClause<E>>: Query,
    {
        loop {}
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SetNotCalled;
