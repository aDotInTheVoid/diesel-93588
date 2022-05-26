use crate::dsl;
use crate::expression::grouped::Grouped;
use crate::expression::operators::{And, Or};
use crate::expression::{AsExpression, Expression, TypedExpressionType};
use crate::sql_types::{BoolOrNullableBool, SqlType};
pub trait BoolExpressionMethods: Expression + Sized {
    fn and<T, ST>(self, other: T) -> dsl::And<Self, T, ST>
    where
        Self::SqlType: SqlType,
        ST: SqlType + TypedExpressionType,
        T: AsExpression<ST>,
        And<Self, T::Expression>: Expression,
    {
        loop {}
    }
    fn or<T, ST>(self, other: T) -> dsl::Or<Self, T, ST>
    where
        Self::SqlType: SqlType,
        ST: SqlType + TypedExpressionType,
        T: AsExpression<ST>,
        Or<Self, T::Expression>: Expression,
    {
        loop {}
    }
}
impl<T> BoolExpressionMethods for T
where
    T: Expression,
    T::SqlType: BoolOrNullableBool,
{}
