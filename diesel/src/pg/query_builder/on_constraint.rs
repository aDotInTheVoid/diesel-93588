use crate::pg::backend::PgOnConflictClaues;
use crate::pg::Pg;
use crate::query_builder::upsert::on_conflict_target::{ConflictTarget, OnConflictTarget};
use crate::query_builder::*;
use crate::result::QueryResult;
pub fn on_constraint(constraint_name: &str) -> OnConstraint<'_> {
    loop {}
}
#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct OnConstraint<'a> {
    constraint_name: &'a str,
}
impl<'a> QueryId for OnConstraint<'a> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}
impl<'a> QueryFragment<Pg, PgOnConflictClaues> for ConflictTarget<OnConstraint<'a>> {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
impl<'a, Table> OnConflictTarget<Table> for ConflictTarget<OnConstraint<'a>> {}
