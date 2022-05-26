use super::{AstPass, QueryFragment, QueryId};
use crate::backend::Backend;
use crate::query_source::AppearsInFromClause;
use crate::{QueryResult, QuerySource};
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoFromClause;
impl<DB> QueryFragment<DB> for NoFromClause
where
    Self: QueryFragment<DB, DB::EmptyFromClauseSyntax>,
    DB: Backend,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<
    DB,
> QueryFragment<
    DB,
    crate::backend::sql_dialect::from_clause_syntax::AnsiSqlFromClauseSyntax,
> for NoFromClause
where
    DB: Backend<
        EmptyFromClauseSyntax = crate::backend::sql_dialect::from_clause_syntax::AnsiSqlFromClauseSyntax,
    >,
{
    fn walk_ast<'b>(&'b self, _pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
pub trait AsQuerySource {
    type QuerySource: QuerySource;
    fn as_query_source(&self) -> &Self::QuerySource;
}
impl<QS> AsQuerySource for QS
where
    QS: QuerySource,
{
    type QuerySource = Self;
    fn as_query_source(&self) -> &Self::QuerySource {
        loop {}
    }
}
#[doc(hidden)]
pub struct FromClause<F: QuerySource> {
    pub(crate) source: F,
    pub(crate) from_clause: F::FromClause,
}
impl<F> AsQuerySource for FromClause<F>
where
    F: QuerySource,
{
    type QuerySource = F;
    fn as_query_source(&self) -> &Self::QuerySource {
        loop {}
    }
}
impl<F> QueryId for FromClause<F>
where
    F: QuerySource + QueryId,
{
    type QueryId = F::QueryId;
    const HAS_STATIC_QUERY_ID: bool = F::HAS_STATIC_QUERY_ID;
}
impl<F> Copy for FromClause<F>
where
    F: QuerySource + Copy,
    F::FromClause: Copy,
{
}
impl<F> Clone for FromClause<F>
where
    F: QuerySource + Clone,
    F::FromClause: Clone,
{
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<F> std::fmt::Debug for FromClause<F>
where
    F: QuerySource,
    F::FromClause: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
impl<F: QuerySource> FromClause<F> {
    pub(crate) fn new(qs: F) -> Self {
        loop {}
    }
}
impl<DB, F> QueryFragment<DB> for FromClause<F>
where
    F: QuerySource,
    DB: Backend,
    F::FromClause: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<QS1, QS2> AppearsInFromClause<QS1> for FromClause<QS2>
where
    QS1: QuerySource,
    QS2: QuerySource,
    QS2: AppearsInFromClause<QS1>,
{
    type Count = <QS2 as AppearsInFromClause<QS1>>::Count;
}
