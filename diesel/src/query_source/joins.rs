use super::{AppearsInFromClause, Plus, QuerySource};
use crate::backend::Backend;
use crate::backend::DieselReserveSpecialization;
use crate::expression::grouped::Grouped;
use crate::expression::nullable::Nullable;
use crate::expression::SelectableExpression;
use crate::prelude::*;
use crate::query_builder::*;
use crate::query_dsl::InternalJoinDsl;
use crate::result::QueryResult;
use crate::sql_types::BoolOrNullableBool;
use crate::util::TupleAppend;
pub struct Join<Left: QuerySource, Right: QuerySource, Kind> {
    left: FromClause<Left>,
    right: FromClause<Right>,
    kind: Kind,
}
impl<Left, Right, Kind> Clone for Join<Left, Right, Kind>
where
    Left: QuerySource,
    FromClause<Left>: Clone,
    Right: QuerySource,
    FromClause<Right>: Clone,
    Kind: Clone,
{
    fn clone(&self) -> Self {
        loop {}
    }
}
impl<Left, Right, Kind> Copy for Join<Left, Right, Kind>
where
    Left: QuerySource,
    FromClause<Left>: Copy,
    Right: QuerySource,
    FromClause<Right>: Copy,
    Kind: Copy,
{
}
impl<Left, Right, Kind> std::fmt::Debug for Join<Left, Right, Kind>
where
    Left: QuerySource,
    FromClause<Left>: std::fmt::Debug,
    Right: QuerySource,
    FromClause<Right>: std::fmt::Debug,
    Kind: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
impl<Left, Right, Kind> QueryId for Join<Left, Right, Kind>
where
    Left: QueryId + QuerySource + 'static,
    Right: QueryId + QuerySource + 'static,
    Kind: QueryId,
{
    type QueryId = Join<Left, Right, Kind::QueryId>;
    const HAS_STATIC_QUERY_ID: bool =
        Left::HAS_STATIC_QUERY_ID && Right::HAS_STATIC_QUERY_ID && Kind::HAS_STATIC_QUERY_ID;
}
#[derive(Debug, Clone, Copy, QueryId)]
#[doc(hidden)]
pub struct JoinOn<Join, On> {
    join: Join,
    on: On,
}
impl<Left, Right, Kind> Join<Left, Right, Kind>
where
    Left: QuerySource,
    Right: QuerySource,
{
    pub(crate) fn new(left: Left, right: Right, kind: Kind) -> Self {
        loop {}
    }
    pub(crate) fn on<On>(self, on: On) -> JoinOn<Self, On> {
        loop {}
    }
}
impl<Left, Right> QuerySource for Join<Left, Right, Inner>
where
    Left: QuerySource + AppendSelection<Right::DefaultSelection>,
    Right: QuerySource,
    Left::Output: SelectableExpression<Self>,
    Self: Clone,
{
    type FromClause = Self;
    type DefaultSelection = Left::Output;
    fn from_clause(&self) -> Self::FromClause {
        loop {}
    }
    fn default_selection(&self) -> Self::DefaultSelection {
        loop {}
    }
}
impl<Left, Right> QuerySource for Join<Left, Right, LeftOuter>
where
    Left: QuerySource + AppendSelection<Nullable<Right::DefaultSelection>>,
    Right: QuerySource,
    Left::Output: SelectableExpression<Self>,
    Self: Clone,
{
    type FromClause = Self;
    type DefaultSelection = Left::Output;
    fn from_clause(&self) -> Self::FromClause {
        loop {}
    }
    fn default_selection(&self) -> Self::DefaultSelection {
        loop {}
    }
}
#[derive(Debug, Clone, Copy)]
pub struct OnKeyword;
impl<DB: Backend> nodes::MiddleFragment<DB> for OnKeyword {
    fn push_sql(&self, mut pass: AstPass<'_, '_, DB>) {
        loop {}
    }
}
impl<Join, On> QuerySource for JoinOn<Join, On>
where
    Join: QuerySource,
    On: AppearsOnTable<Join::FromClause> + Clone,
    On::SqlType: BoolOrNullableBool,
    Join::DefaultSelection: SelectableExpression<Self>,
{
    type FromClause = Grouped<nodes::InfixNode<Join::FromClause, On, OnKeyword>>;
    type DefaultSelection = Join::DefaultSelection;
    fn from_clause(&self) -> Self::FromClause {
        loop {}
    }
    fn default_selection(&self) -> Self::DefaultSelection {
        loop {}
    }
}
impl<Left, Right, Kind, DB> QueryFragment<DB> for Join<Left, Right, Kind>
where
    DB: Backend + DieselReserveSpecialization,
    Left: QuerySource,
    Left::FromClause: QueryFragment<DB>,
    Right: QuerySource,
    Right::FromClause: QueryFragment<DB>,
    Kind: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
pub trait JoinTo<T> {
    #[doc(hidden)]
    type FromClause;
    #[doc(hidden)]
    type OnClause;
    #[doc(hidden)]
    fn join_target(rhs: T) -> (Self::FromClause, Self::OnClause);
}
#[doc(hidden)]
pub trait AppendSelection<Selection> {
    type Output;
    fn append_selection(&self, selection: Selection) -> Self::Output;
}
impl<T: Table, Selection> AppendSelection<Selection> for T {
    type Output = (T::AllColumns, Selection);
    fn append_selection(&self, selection: Selection) -> Self::Output {
        loop {}
    }
}
impl<Left, Mid, Selection, Kind> AppendSelection<Selection> for Join<Left, Mid, Kind>
where
    Left: QuerySource,
    Mid: QuerySource,
    Self: QuerySource,
    <Self as QuerySource>::DefaultSelection: TupleAppend<Selection>,
{
    type Output = <<Self as QuerySource>::DefaultSelection as TupleAppend<Selection>>::Output;
    fn append_selection(&self, selection: Selection) -> Self::Output {
        loop {}
    }
}
impl<Join, On, Selection> AppendSelection<Selection> for JoinOn<Join, On>
where
    Join: AppendSelection<Selection>,
{
    type Output = Join::Output;
    fn append_selection(&self, selection: Selection) -> Self::Output {
        loop {}
    }
}
#[doc(hidden)]
#[derive(Debug, Clone, Copy, Default, QueryId)]
pub struct Inner;
impl<DB> QueryFragment<DB> for Inner
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[doc(hidden)]
#[derive(Debug, Clone, Copy, Default, QueryId)]
pub struct LeftOuter;
impl<DB> QueryFragment<DB> for LeftOuter
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<Left, Mid, Right, Kind> JoinTo<Right> for Join<Left, Mid, Kind>
where
    Left: JoinTo<Right> + QuerySource,
    Mid: QuerySource,
{
    type FromClause = <Left as JoinTo<Right>>::FromClause;
    type OnClause = Left::OnClause;
    fn join_target(rhs: Right) -> (Self::FromClause, Self::OnClause) {
        loop {}
    }
}
impl<Join, On, Right> JoinTo<Right> for JoinOn<Join, On>
where
    Join: JoinTo<Right>,
{
    type FromClause = Join::FromClause;
    type OnClause = Join::OnClause;
    fn join_target(rhs: Right) -> (Self::FromClause, Self::OnClause) {
        loop {}
    }
}
impl<T, Left, Right, Kind> AppearsInFromClause<T> for Join<Left, Right, Kind>
where
    Left: AppearsInFromClause<T> + QuerySource,
    Right: AppearsInFromClause<T> + QuerySource,
    Left::Count: Plus<Right::Count>,
{
    type Count = <Left::Count as Plus<Right::Count>>::Output;
}
impl<T, Join, On> AppearsInFromClause<T> for JoinOn<Join, On>
where
    Join: AppearsInFromClause<T>,
{
    type Count = Join::Count;
}
#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct OnClauseWrapper<Source, On> {
    pub(crate) source: Source,
    pub(crate) on: On,
}
impl<Source, On> OnClauseWrapper<Source, On> {
    pub fn new(source: Source, on: On) -> Self {
        loop {}
    }
}
impl<Lhs, Rhs, On> JoinTo<OnClauseWrapper<Rhs, On>> for Lhs
where
    Lhs: Table,
{
    type FromClause = Rhs;
    type OnClause = On;
    fn join_target(rhs: OnClauseWrapper<Rhs, On>) -> (Self::FromClause, Self::OnClause) {
        loop {}
    }
}
impl<Lhs, Rhs, On> JoinTo<Rhs> for OnClauseWrapper<Lhs, On>
where
    Lhs: JoinTo<Rhs>,
{
    type FromClause = <Lhs as JoinTo<Rhs>>::FromClause;
    type OnClause = <Lhs as JoinTo<Rhs>>::OnClause;
    fn join_target(rhs: Rhs) -> (Self::FromClause, Self::OnClause) {
        loop {}
    }
}
impl<Rhs, Kind, On1, On2, Lhs> InternalJoinDsl<Rhs, Kind, On1> for OnClauseWrapper<Lhs, On2>
where
    Lhs: InternalJoinDsl<Rhs, Kind, On1>,
{
    type Output = OnClauseWrapper<<Lhs as InternalJoinDsl<Rhs, Kind, On1>>::Output, On2>;
    fn join(self, rhs: Rhs, kind: Kind, on: On1) -> Self::Output {
        loop {}
    }
}
impl<Qs, On> QueryDsl for OnClauseWrapper<Qs, On> {}
#[doc(hidden)]
pub trait ToInnerJoin {
    type InnerJoin;
}
impl<Left, Right, Kind> ToInnerJoin for Join<Left, Right, Kind>
where
    Left: ToInnerJoin + QuerySource,
    Left::InnerJoin: QuerySource,
    Right: ToInnerJoin + QuerySource,
    Right::InnerJoin: QuerySource,
{
    type InnerJoin = Join<Left::InnerJoin, Right::InnerJoin, Inner>;
}
impl<Join, On> ToInnerJoin for JoinOn<Join, On>
where
    Join: ToInnerJoin,
{
    type InnerJoin = JoinOn<Join::InnerJoin, On>;
}
impl<From> ToInnerJoin for SelectStatement<FromClause<From>>
where
    From: ToInnerJoin + QuerySource,
    From::InnerJoin: QuerySource,
{
    type InnerJoin = SelectStatement<FromClause<From::InnerJoin>>;
}
impl<T: Table> ToInnerJoin for T {
    type InnerJoin = T;
}
