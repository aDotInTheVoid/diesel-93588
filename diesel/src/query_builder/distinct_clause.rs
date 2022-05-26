use crate::backend::{Backend, DieselReserveSpecialization};
use crate::query_builder::*;
use crate::query_dsl::order_dsl::ValidOrderingForDistinct;
use crate::result::QueryResult;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoDistinctClause;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct DistinctClause;
impl<DB> QueryFragment<DB> for NoDistinctClause
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, _: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<DB> QueryFragment<DB> for DistinctClause
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<O> ValidOrderingForDistinct<NoDistinctClause> for O {}
impl<O> ValidOrderingForDistinct<DistinctClause> for O {}
#[allow(unreachable_pub)]
#[cfg(feature = "postgres_backend")]
pub use crate::pg::DistinctOnClause;
