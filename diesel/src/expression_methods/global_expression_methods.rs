use crate::dsl;
use crate::expression::array_comparison::{AsInExpression, In, NotIn};
use crate::expression::grouped::Grouped;
use crate::expression::operators::*;
use crate::expression::{assume_not_null, nullable, AsExpression, Expression};
use crate::sql_types::{SingleValue, SqlType};
pub trait ExpressionMethods: Expression + Sized {
    #[doc(alias = "=")]
    fn eq<T>(self, other: T) -> dsl::Eq<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    #[doc(alias = "<>")]
    fn ne<T>(self, other: T) -> dsl::NotEq<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    #[doc(alias = "in")]
    fn eq_any<T>(self, values: T) -> dsl::EqAny<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsInExpression<Self::SqlType>,
    {
        loop {}
    }
    #[doc(alias = "in")]
    fn ne_all<T>(self, values: T) -> dsl::NeAny<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsInExpression<Self::SqlType>,
    {
        loop {}
    }
    #[allow(clippy::wrong_self_convention)]
    fn is_null(self) -> dsl::IsNull<Self> {
        loop {}
    }
    #[allow(clippy::wrong_self_convention)]
    fn is_not_null(self) -> dsl::IsNotNull<Self> {
        loop {}
    }
    #[doc(alias = ">")]
    fn gt<T>(self, other: T) -> dsl::Gt<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    #[doc(alias = ">=")]
    fn ge<T>(self, other: T) -> dsl::GtEq<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    #[doc(alias = "<")]
    fn lt<T>(self, other: T) -> dsl::Lt<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    #[doc(alias = "<=")]
    fn le<T>(self, other: T) -> dsl::LtEq<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    fn between<T, U>(self, lower: T, upper: U) -> dsl::Between<Self, T, U>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
        U: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    fn not_between<T, U>(self, lower: T, upper: U) -> dsl::NotBetween<Self, T, U>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
        U: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    fn desc(self) -> dsl::Desc<Self> {
        loop {}
    }
    fn asc(self) -> dsl::Asc<Self> {
        loop {}
    }
}
impl<T> ExpressionMethods for T
where
    T: Expression,
    T::SqlType: SingleValue,
{}
pub trait NullableExpressionMethods: Expression + Sized {
    fn nullable(self) -> dsl::Nullable<Self> {
        loop {}
    }
    fn assume_not_null(self) -> dsl::AssumeNotNull<Self> {
        loop {}
    }
}
impl<T: Expression> NullableExpressionMethods for T {}
