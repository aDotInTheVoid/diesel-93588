use super::on_conflict_actions::*;
use super::on_conflict_target::*;
use crate::backend::sql_dialect;
use crate::backend::Backend;
use crate::insertable::*;
use crate::query_builder::*;
use crate::result::QueryResult;
#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct OnConflictValues<Values, Target, Action> {
    values: Values,
    target: Target,
    action: Action,
}
impl<Values, Target, Action> QueryId for OnConflictValues<Values, Target, Action> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}
impl<Values> OnConflictValues<Values, NoConflictTarget, DoNothing> {
    pub(crate) fn do_nothing(values: Values) -> Self {
        loop {}
    }
}
impl<Values, Target, Action> OnConflictValues<Values, Target, Action> {
    pub(crate) fn new(values: Values, target: Target, action: Action) -> Self {
        loop {}
    }
}
impl<DB, Values, Target, Action> CanInsertInSingleQuery<DB>
for OnConflictValues<Values, Target, Action>
where
    DB: Backend,
    DB::OnConflictClause: sql_dialect::on_conflict_clause::SupportsOnConflictClause,
    Values: CanInsertInSingleQuery<DB>,
{
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<DB, Values, Target, Action> QueryFragment<DB>
for OnConflictValues<Values, Target, Action>
where
    DB: Backend,
    DB::OnConflictClause: sql_dialect::on_conflict_clause::SupportsOnConflictClause,
    Values: QueryFragment<DB>,
    Target: QueryFragment<DB>,
    Action: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
