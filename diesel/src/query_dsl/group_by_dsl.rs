use crate::dsl;
use crate::expression::Expression;
use crate::expression::TypedExpressionType;
use crate::expression::ValidGrouping;
use crate::query_builder::FromClause;
use crate::query_builder::{AsQuery, SelectStatement};
use crate::query_source::Table;
pub trait GroupByDsl<Expr: Expression> {
        type Output;
        fn group_by(self, expr: Expr) -> dsl::GroupBy<Self, Expr>;
}
impl<T, Expr> GroupByDsl<Expr> for T
where
    Expr: Expression,
    T: Table + AsQuery<Query = SelectStatement<FromClause<T>>>,
    T::DefaultSelection: Expression<SqlType = T::SqlType> + ValidGrouping<()>,
    T::SqlType: TypedExpressionType,
{
    type Output = dsl::GroupBy<SelectStatement<FromClause<T>>, Expr>;
    fn group_by(self, expr: Expr) -> dsl::GroupBy<Self, Expr> {
        loop {}
    }
}
