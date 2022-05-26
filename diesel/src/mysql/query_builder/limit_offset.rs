use crate::mysql::Mysql;
use crate::query_builder::limit_clause::{LimitClause, NoLimitClause};
use crate::query_builder::limit_offset_clause::{
    BoxedLimitOffsetClause, LimitOffsetClause,
};
use crate::query_builder::offset_clause::{NoOffsetClause, OffsetClause};
use crate::query_builder::{AstPass, IntoBoxedClause, QueryFragment};
use crate::result::QueryResult;
impl QueryFragment<Mysql> for LimitOffsetClause<NoLimitClause, NoOffsetClause> {
    fn walk_ast<'b>(&'b self, _out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
        loop {}
    }
}
impl<L> QueryFragment<Mysql> for LimitOffsetClause<LimitClause<L>, NoOffsetClause>
where
    LimitClause<L>: QueryFragment<Mysql>,
{
    fn walk_ast<'b>(&'b self, out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
        loop {}
    }
}
impl<L, O> QueryFragment<Mysql> for LimitOffsetClause<LimitClause<L>, OffsetClause<O>>
where
    LimitClause<L>: QueryFragment<Mysql>,
    OffsetClause<O>: QueryFragment<Mysql>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
        loop {}
    }
}
impl<'a> QueryFragment<Mysql> for BoxedLimitOffsetClause<'a, Mysql> {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
        loop {}
    }
}
impl<'a> IntoBoxedClause<'a, Mysql>
for LimitOffsetClause<NoLimitClause, NoOffsetClause> {
    type BoxedClause = BoxedLimitOffsetClause<'a, Mysql>;
    fn into_boxed(self) -> Self::BoxedClause {
        loop {}
    }
}
impl<'a, L> IntoBoxedClause<'a, Mysql>
for LimitOffsetClause<LimitClause<L>, NoOffsetClause>
where
    L: QueryFragment<Mysql> + Send + 'a,
{
    type BoxedClause = BoxedLimitOffsetClause<'a, Mysql>;
    fn into_boxed(self) -> Self::BoxedClause {
        loop {}
    }
}
impl<'a, L, O> IntoBoxedClause<'a, Mysql>
for LimitOffsetClause<LimitClause<L>, OffsetClause<O>>
where
    L: QueryFragment<Mysql> + Send + 'a,
    O: QueryFragment<Mysql> + Send + 'a,
{
    type BoxedClause = BoxedLimitOffsetClause<'a, Mysql>;
    fn into_boxed(self) -> Self::BoxedClause {
        loop {}
    }
}
