use crate::expression::Expression;
use crate::query_source::Table;
pub trait OrderDsl<Expr: Expression> {
    type Output;
    fn order(self, expr: Expr) -> Self::Output;
}
impl<T, Expr> OrderDsl<Expr> for T
where
    Expr: Expression,
    T: Table,
    T::Query: OrderDsl<Expr>,
{
    type Output = <T::Query as OrderDsl<Expr>>::Output;
    fn order(self, expr: Expr) -> Self::Output {
        loop {}
    }
}
pub trait ThenOrderDsl<Expr> {
    type Output;
    fn then_order_by(self, expr: Expr) -> Self::Output;
}
impl<T, Expr> ThenOrderDsl<Expr> for T
where
    Expr: Expression,
    T: Table,
    T::Query: ThenOrderDsl<Expr>,
{
    type Output = <T::Query as ThenOrderDsl<Expr>>::Output;
    fn then_order_by(self, expr: Expr) -> Self::Output {
        loop {}
    }
}
pub trait ValidOrderingForDistinct<D> {}
