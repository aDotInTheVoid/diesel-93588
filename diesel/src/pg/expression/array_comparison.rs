use crate::expression::subselect::Subselect;
use crate::expression::{AsExpression, Expression, TypedExpressionType, ValidGrouping};
use crate::pg::Pg;
use crate::query_builder::*;
use crate::result::QueryResult;
use crate::sql_types::{Array, SqlType};
#[deprecated(since = "2.0.0", note = "Use `ExpressionMethods::eq_any` instead")]
pub fn any<ST, T>(vals: T) -> Any<T::Expression>
where
    T: AsArrayExpression<ST>,
{
    loop {}
}
#[deprecated(since = "2.0.0", note = "Use `ExpressionMethods::ne_all` instead")]
pub fn all<ST, T>(vals: T) -> All<T::Expression>
where
    T: AsArrayExpression<ST>,
{
    loop {}
}
#[doc(hidden)]
#[derive(Debug, Copy, Clone, QueryId, ValidGrouping)]
pub struct Any<Expr> {
    expr: Expr,
}
impl<Expr> Any<Expr> {
    fn new(expr: Expr) -> Self {
        loop {}
    }
}
impl<Expr, ST> Expression for Any<Expr>
where
    Expr: Expression<SqlType = Array<ST>>,
    ST: SqlType + TypedExpressionType,
{
    type SqlType = ST;
}
impl<Expr> QueryFragment<Pg> for Any<Expr>
where
    Expr: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
impl_selectable_expression!(Any < Expr >);
#[doc(hidden)]
#[derive(Debug, Copy, Clone, QueryId, ValidGrouping)]
pub struct All<Expr> {
    expr: Expr,
}
impl<Expr> All<Expr> {
    fn new(expr: Expr) -> Self {
        loop {}
    }
}
impl<Expr, ST> Expression for All<Expr>
where
    Expr: Expression<SqlType = Array<ST>>,
    ST: SqlType + TypedExpressionType,
{
    type SqlType = ST;
}
impl<Expr> QueryFragment<Pg> for All<Expr>
where
    Expr: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
impl_selectable_expression!(All < Expr >);
pub trait AsArrayExpression<ST: 'static> {
    type Expression: Expression<SqlType = Array<ST>>;
    #[allow(clippy::wrong_self_convention)]
    fn as_expression(self) -> Self::Expression;
}
impl<ST, T> AsArrayExpression<ST> for T
where
    ST: 'static,
    T: AsExpression<Array<ST>>,
{
    type Expression = <T as AsExpression<Array<ST>>>::Expression;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
impl<ST, F, S, D, W, O, LOf, G, H, LC> AsArrayExpression<ST>
for SelectStatement<F, S, D, W, O, LOf, G, H, LC>
where
    ST: 'static,
    Self: SelectQuery<SqlType = ST>,
{
    type Expression = Subselect<Self, Array<ST>>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
impl<'a, ST, QS, DB, GB> AsArrayExpression<ST>
for BoxedSelectStatement<'a, ST, QS, DB, GB>
where
    ST: 'static,
    Self: SelectQuery<SqlType = ST>,
{
    type Expression = Subselect<Self, Array<ST>>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
