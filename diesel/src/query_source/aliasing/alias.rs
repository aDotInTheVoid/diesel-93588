use super::field_alias_mapper::FieldAliasMapper;
use super::{AliasSource, AliasedField};
use crate::backend::Backend;
use crate::expression::{Expression, SelectableExpression, ValidGrouping};
use crate::helper_types::AliasedFields;
use crate::query_builder::{
    AsQuery, AstPass, FromClause, QueryFragment, QueryId, SelectStatement,
};
use crate::query_source::{
    AppearsInFromClause, Column, Never, QuerySource, Table, TableNotEqual,
};
use crate::result::QueryResult;
use std::marker::PhantomData;
#[derive(Debug, Clone, Copy, Default)]
pub struct Alias<S> {
    pub(crate) source: S,
}
impl<S: AliasSource> Alias<S> {
    pub fn field<F>(&self, field: F) -> AliasedField<S, F>
    where
        F: Column<Table = S::Target>,
    {
        loop {}
    }
    pub fn fields<Fields>(&self, fields: Fields) -> AliasedFields<S, Fields>
    where
        Fields: FieldAliasMapper<S>,
    {
        loop {}
    }
}
impl<S> Alias<S> {
    #[doc(hidden)]
    pub const fn new(source: S) -> Self {
        loop {}
    }
}
impl<S> QueryId for Alias<S>
where
    Self: 'static,
    S: AliasSource,
    S::Target: QueryId,
{
    type QueryId = Self;
    const HAS_STATIC_QUERY_ID: bool = <S::Target as QueryId>::HAS_STATIC_QUERY_ID;
}
impl<S> QuerySource for Alias<S>
where
    Self: Clone,
    S: AliasSource,
    S::Target: QuerySource,
    <S::Target as QuerySource>::DefaultSelection: FieldAliasMapper<S>,
    <<S::Target as QuerySource>::DefaultSelection as FieldAliasMapper<
        S,
    >>::Out: SelectableExpression<Self>,
{
    type FromClause = Self;
    type DefaultSelection = <<S::Target as QuerySource>::DefaultSelection as FieldAliasMapper<
        S,
    >>::Out;
    fn from_clause(&self) -> Self::FromClause {
        loop {}
    }
    fn default_selection(&self) -> Self::DefaultSelection {
        loop {}
    }
}
impl<S, DB> QueryFragment<DB> for Alias<S>
where
    S: AliasSource,
    DB: Backend,
    S::Target: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<S> AsQuery for Alias<S>
where
    S: AliasSource,
    S::Target: AsQuery,
    Self: QuerySource,
    <Self as QuerySource>::DefaultSelection: ValidGrouping<()>,
{
    type SqlType = <<Self as QuerySource>::DefaultSelection as Expression>::SqlType;
    type Query = SelectStatement<FromClause<Self>>;
    fn as_query(self) -> Self::Query {
        loop {}
    }
}
pub trait AliasAppearsInFromClause<S, QS> {
    type Count;
}
impl<S, QS> AppearsInFromClause<QS> for Alias<S>
where
    S: AliasSource,
    S::Target: AliasAppearsInFromClause<S, QS>,
{
    type Count = <S::Target as AliasAppearsInFromClause<S, QS>>::Count;
}
pub trait AliasAliasAppearsInFromClause<T2, S1, S2> {
    type Count;
}
impl<T1, S1, S2> AliasAppearsInFromClause<S1, Alias<S2>> for T1
where
    S2: AliasSource,
    T1: AliasAliasAppearsInFromClause<S2::Target, S1, S2>,
{
    type Count = <T1 as AliasAliasAppearsInFromClause<S2::Target, S1, S2>>::Count;
}
pub trait AliasAliasAppearsInFromClauseSameTable<S2, T> {
    type Count;
}
impl<T1, T2, S> AliasAppearsInFromClause<S, T2> for T1
where
    T1: TableNotEqual<T2> + Table,
    T2: Table,
    S: AliasSource<Target = T1>,
{
    type Count = Never;
}
impl<T1, T2, S1, S2> AliasAliasAppearsInFromClause<T1, S2, S1> for T2
where
    T1: TableNotEqual<T2> + Table,
    T2: Table,
    S1: AliasSource<Target = T1>,
    S2: AliasSource<Target = T2>,
{
    type Count = Never;
}
