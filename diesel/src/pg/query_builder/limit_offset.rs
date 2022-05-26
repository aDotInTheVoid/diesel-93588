use crate::pg::Pg;
use crate::query_builder::limit_offset_clause::{
    BoxedLimitOffsetClause, LimitOffsetClause,
};
use crate::query_builder::{AstPass, IntoBoxedClause, QueryFragment};
use crate::result::QueryResult;
impl<'a, L, O> IntoBoxedClause<'a, Pg> for LimitOffsetClause<L, O>
where
    L: QueryFragment<Pg> + Send + 'a,
    O: QueryFragment<Pg> + Send + 'a,
{
    type BoxedClause = BoxedLimitOffsetClause<'a, Pg>;
    fn into_boxed(self) -> Self::BoxedClause {
        loop {}
    }
}
impl<'a> QueryFragment<Pg> for BoxedLimitOffsetClause<'a, Pg> {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
impl<L, O> QueryFragment<Pg> for LimitOffsetClause<L, O>
where
    L: QueryFragment<Pg>,
    O: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
