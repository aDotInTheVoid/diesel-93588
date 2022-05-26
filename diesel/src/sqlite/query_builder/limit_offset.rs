use crate::query_builder::limit_clause::{LimitClause, NoLimitClause};
use crate::query_builder::limit_offset_clause::{
    BoxedLimitOffsetClause, LimitOffsetClause,
};
use crate::query_builder::offset_clause::{NoOffsetClause, OffsetClause};
use crate::query_builder::{AstPass, IntoBoxedClause, QueryFragment};
use crate::result::QueryResult;
use crate::sqlite::Sqlite;
impl QueryFragment<Sqlite> for LimitOffsetClause<NoLimitClause, NoOffsetClause> {
    fn walk_ast<'b>(&'b self, _out: AstPass<'_, 'b, Sqlite>) -> QueryResult<()> {
        loop {}
    }
}
impl<L> QueryFragment<Sqlite> for LimitOffsetClause<LimitClause<L>, NoOffsetClause>
where
    LimitClause<L>: QueryFragment<Sqlite>,
{
    fn walk_ast<'b>(&'b self, out: AstPass<'_, 'b, Sqlite>) -> QueryResult<()> {
        loop {}
    }
}
impl<O> QueryFragment<Sqlite> for LimitOffsetClause<NoLimitClause, OffsetClause<O>>
where
    OffsetClause<O>: QueryFragment<Sqlite>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Sqlite>) -> QueryResult<()> {
        loop {}
    }
}
impl<L, O> QueryFragment<Sqlite> for LimitOffsetClause<LimitClause<L>, OffsetClause<O>>
where
    LimitClause<L>: QueryFragment<Sqlite>,
    OffsetClause<O>: QueryFragment<Sqlite>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Sqlite>) -> QueryResult<()> {
        loop {}
    }
}
impl<'a> QueryFragment<Sqlite> for BoxedLimitOffsetClause<'a, Sqlite> {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Sqlite>) -> QueryResult<()> {
        loop {}
    }
}
impl<'a> IntoBoxedClause<'a, Sqlite>
for LimitOffsetClause<NoLimitClause, NoOffsetClause> {
    type BoxedClause = BoxedLimitOffsetClause<'a, Sqlite>;
    fn into_boxed(self) -> Self::BoxedClause {
        loop {}
    }
}
impl<'a, L> IntoBoxedClause<'a, Sqlite>
for LimitOffsetClause<LimitClause<L>, NoOffsetClause>
where
    L: QueryFragment<Sqlite> + Send + 'a,
{
    type BoxedClause = BoxedLimitOffsetClause<'a, Sqlite>;
    fn into_boxed(self) -> Self::BoxedClause {
        loop {}
    }
}
impl<'a, O> IntoBoxedClause<'a, Sqlite>
for LimitOffsetClause<NoLimitClause, OffsetClause<O>>
where
    O: QueryFragment<Sqlite> + Send + 'a,
{
    type BoxedClause = BoxedLimitOffsetClause<'a, Sqlite>;
    fn into_boxed(self) -> Self::BoxedClause {
        loop {}
    }
}
impl<'a, L, O> IntoBoxedClause<'a, Sqlite>
for LimitOffsetClause<LimitClause<L>, OffsetClause<O>>
where
    L: QueryFragment<Sqlite> + Send + 'a,
    O: QueryFragment<Sqlite> + Send + 'a,
{
    type BoxedClause = BoxedLimitOffsetClause<'a, Sqlite>;
    fn into_boxed(self) -> Self::BoxedClause {
        loop {}
    }
}
