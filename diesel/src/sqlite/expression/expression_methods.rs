use super::operators::*;
use crate::dsl;
use crate::expression::grouped::Grouped;
use crate::expression::{AsExpression, Expression};
use crate::sql_types::SqlType;
#[cfg(feature = "sqlite")]
pub trait SqliteExpressionMethods: Expression + Sized {
    fn is<T>(self, other: T) -> dsl::Is<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    #[allow(clippy::wrong_self_convention)]
    fn is_not<T>(self, other: T) -> dsl::IsNot<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
}
impl<T: Expression> SqliteExpressionMethods for T {}
