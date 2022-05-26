use crate::backend::{Backend, DieselReserveSpecialization};
use crate::expression::helper_types::{Asc, Desc};
use crate::query_builder::combination_clause::CombinationClause;
use crate::query_builder::{AstPass, Query, QueryFragment, QueryId};
use crate::{QueryResult, RunQueryDsl};
pub trait PositionalOrderDsl<Expr: Order>: Sized {
    fn positional_order_by(
        self,
        expr: Expr,
    ) -> PositionalOrderClause<Self, Expr::Fragment> {
        PositionalOrderClause {
            source: self,
            expr: expr.into_fragment(),
        }
    }
}
#[derive(Debug, Clone, Copy, QueryId)]
pub struct PositionalOrderClause<Source, Expr> {
    source: Source,
    expr: Expr,
}
impl<Combinator, Rule, Source, Rhs, Expr: Order> PositionalOrderDsl<Expr>
for CombinationClause<Combinator, Rule, Source, Rhs> {}
impl<Source, Expr> Query for PositionalOrderClause<Source, Expr>
where
    Source: Query,
{
    type SqlType = Source::SqlType;
}
impl<Source, Expr, Conn> RunQueryDsl<Conn> for PositionalOrderClause<Source, Expr> {}
impl<Source, Expr, DB> QueryFragment<DB> for PositionalOrderClause<Source, Expr>
where
    DB: Backend + DieselReserveSpecialization,
    Source: QueryFragment<DB>,
    Expr: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Clone, Copy, QueryId)]
pub struct OrderColumn(u32);
impl<DB: Backend> QueryFragment<DB> for OrderColumn {
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl From<u32> for OrderColumn {
    fn from(order: u32) -> Self {
        loop {}
    }
}
pub trait IntoOrderColumn: Into<OrderColumn> {
    fn asc(self) -> Asc<OrderColumn> {
        Asc { expr: self.into() }
    }
    fn desc(self) -> Desc<OrderColumn> {
        Desc { expr: self.into() }
    }
}
impl<T> IntoOrderColumn for T
where
    T: Into<OrderColumn>,
{}
pub trait Order: Copy {
    type Fragment;
    fn into_fragment(self) -> Self::Fragment;
}
impl<T: Into<OrderColumn> + Copy> Order for T {
    type Fragment = OrderColumn;
    fn into_fragment(self) -> Self::Fragment {
        loop {}
    }
}
impl Order for Asc<OrderColumn> {
    type Fragment = Asc<OrderColumn>;
    fn into_fragment(self) -> Self::Fragment {
        loop {}
    }
}
impl Order for Desc<OrderColumn> {
    type Fragment = Desc<OrderColumn>;
    fn into_fragment(self) -> Self::Fragment {
        loop {}
    }
}
macro_rules! impl_order_for_all_tuples {
    ($($unused1:tt { $(($idx:tt) -> $T:ident, $unused2:ident, $unused3:tt,)+ })+) => {
        $(impl <$($T : Order),+> Order for ($($T,)+) { type Fragment = ($(<$T as Order
        >::Fragment,)+); fn into_fragment(self) -> Self::Fragment { ($(self.$idx
        .into_fragment(),)+) } })+
    };
}
diesel_derives::__diesel_for_each_tuple!(impl_order_for_all_tuples);
