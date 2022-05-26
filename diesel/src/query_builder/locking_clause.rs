use crate::backend::{Backend, DieselReserveSpecialization};
use crate::query_builder::{AstPass, QueryFragment, QueryId};
use crate::result::QueryResult;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoLockingClause;
impl<DB> QueryFragment<DB> for NoLockingClause
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, _: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Clone, Copy, QueryId)]
pub struct LockingClause<LockMode = ForUpdate, Modifier = NoModifier> {
    pub(crate) lock_mode: LockMode,
    modifier: Modifier,
}
impl<LockMode, Modifier> LockingClause<LockMode, Modifier> {
    pub(crate) fn new(lock_mode: LockMode, modifier: Modifier) -> Self {
        loop {}
    }
}
impl<DB, L, M> QueryFragment<DB> for LockingClause<L, M>
where
    DB: Backend + DieselReserveSpecialization,
    L: QueryFragment<DB>,
    M: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Clone, Copy, QueryId)]
pub struct ForUpdate;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct ForNoKeyUpdate;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct ForShare;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct ForKeyShare;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoModifier;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct SkipLocked;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoWait;
