use crate::backend::{Backend, DieselReserveSpecialization};
use crate::expression::grouped::Grouped;
use crate::expression::operators::Eq;
use crate::expression::AppearsOnTable;
use crate::query_builder::*;
use crate::query_source::{Column, QuerySource};
use crate::result::QueryResult;
use crate::Table;
pub trait AsChangeset {
        type Target: QuerySource;
        type Changeset;
        #[allow(clippy::wrong_self_convention)]
    fn as_changeset(self) -> Self::Changeset;
}
#[allow(unreachable_pub)]
#[doc(inline)]
pub use diesel_derives::AsChangeset;
impl<T: AsChangeset> AsChangeset for Option<T> {
    type Target = T::Target;
    type Changeset = Option<T::Changeset>;
    fn as_changeset(self) -> Self::Changeset {
        loop {}
    }
}
impl<Left, Right> AsChangeset for Eq<Left, Right>
where
    Left: AssignmentTarget,
    Right: AppearsOnTable<Left::Table>,
{
    type Target = Left::Table;
    type Changeset = Assign<<Left as AssignmentTarget>::QueryAstNode, Right>;
    fn as_changeset(self) -> Self::Changeset {
        loop {}
    }
}
impl<Left, Right> AsChangeset for Grouped<Eq<Left, Right>>
where
    Eq<Left, Right>: AsChangeset,
{
    type Target = <Eq<Left, Right> as AsChangeset>::Target;
    type Changeset = <Eq<Left, Right> as AsChangeset>::Changeset;
    fn as_changeset(self) -> Self::Changeset {
        loop {}
    }
}
#[derive(Debug, Clone, Copy, QueryId)]
pub struct Assign<Target, Expr> {
    target: Target,
    expr: Expr,
}
impl<T, U, DB> QueryFragment<DB> for Assign<T, U>
where
    DB: Backend,
    T: QueryFragment<DB>,
    U: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
pub trait AssignmentTarget {
        type Table: Table;
            type QueryAstNode;
        fn into_target(self) -> Self::QueryAstNode;
}
#[derive(Debug, Clone, Copy)]
pub struct ColumnWrapperForUpdate<C>(pub C);
impl<DB, C> QueryFragment<DB> for ColumnWrapperForUpdate<C>
where
    DB: Backend + DieselReserveSpecialization,
    C: Column,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<C> AssignmentTarget for C
where
    C: Column,
{
    type Table = C::Table;
    type QueryAstNode = ColumnWrapperForUpdate<C>;
    fn into_target(self) -> Self::QueryAstNode {
        loop {}
    }
}
