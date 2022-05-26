use super::{AstPass, QueryFragment};
use crate::backend::{Backend, DieselReserveSpecialization};
use crate::query_builder::QueryId;
use crate::result::QueryResult;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoReturningClause;
impl<DB> QueryFragment<DB> for NoReturningClause
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, _: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct ReturningClause<Expr>(pub Expr);
impl<Expr, DB> QueryFragment<DB> for ReturningClause<Expr>
where
    DB: Backend,
    Self: QueryFragment<DB, DB::ReturningClause>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<Expr, DB>
    QueryFragment<DB, crate::backend::sql_dialect::returning_clause::PgLikeReturningClause>
    for ReturningClause<Expr>
where
    DB: Backend<
        ReturningClause = crate::backend::sql_dialect::returning_clause::PgLikeReturningClause,
    >,
    Expr: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
