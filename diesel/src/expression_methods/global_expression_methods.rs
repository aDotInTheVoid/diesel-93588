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
        Grouped(Eq::new(self, other.as_expression()))
    }
                                                            #[doc(alias = "<>")]
    fn ne<T>(self, other: T) -> dsl::NotEq<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        Grouped(NotEq::new(self, other.as_expression()))
    }
                                                                                                                                                        #[doc(alias = "in")]
    fn eq_any<T>(self, values: T) -> dsl::EqAny<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsInExpression<Self::SqlType>,
    {
        Grouped(In::new(self, values.as_in_expression()))
    }
                                                                                                                #[doc(alias = "in")]
    fn ne_all<T>(self, values: T) -> dsl::NeAny<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsInExpression<Self::SqlType>,
    {
        Grouped(NotIn::new(self, values.as_in_expression()))
    }
                                                                                            #[allow(clippy::wrong_self_convention)]
    fn is_null(self) -> dsl::IsNull<Self> {
        Grouped(IsNull::new(self))
    }
                                                                                            #[allow(clippy::wrong_self_convention)]
    fn is_not_null(self) -> dsl::IsNotNull<Self> {
        Grouped(IsNotNull::new(self))
    }
                                                                                            #[doc(alias = ">")]
    fn gt<T>(self, other: T) -> dsl::Gt<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        Grouped(Gt::new(self, other.as_expression()))
    }
                                                                                            #[doc(alias = ">=")]
    fn ge<T>(self, other: T) -> dsl::GtEq<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        Grouped(GtEq::new(self, other.as_expression()))
    }
                                                                                            #[doc(alias = "<")]
    fn lt<T>(self, other: T) -> dsl::Lt<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        Grouped(Lt::new(self, other.as_expression()))
    }
                                                                                        #[doc(alias = "<=")]
    fn le<T>(self, other: T) -> dsl::LtEq<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        Grouped(LtEq::new(self, other.as_expression()))
    }
                                                                                    fn between<T, U>(self, lower: T, upper: U) -> dsl::Between<Self, T, U>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
        U: AsExpression<Self::SqlType>,
    {
        Grouped(
            Between::new(self, And::new(lower.as_expression(), upper.as_expression())),
        )
    }
                                                                                                fn not_between<T, U>(self, lower: T, upper: U) -> dsl::NotBetween<Self, T, U>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
        U: AsExpression<Self::SqlType>,
    {
        Grouped(
            NotBetween::new(self, And::new(lower.as_expression(), upper.as_expression())),
        )
    }
                                                                                                    fn desc(self) -> dsl::Desc<Self> {
        Desc::new(self)
    }
                                                                                                    fn asc(self) -> dsl::Asc<Self> {
        Asc::new(self)
    }
}
impl<T> ExpressionMethods for T
where
    T: Expression,
    T::SqlType: SingleValue,
{}
pub trait NullableExpressionMethods: Expression + Sized {
                                                                                                                                            fn nullable(self) -> dsl::Nullable<Self> {
        nullable::Nullable::new(self)
    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            fn assume_not_null(self) -> dsl::AssumeNotNull<Self> {
        assume_not_null::AssumeNotNull::new(self)
    }
}
impl<T: Expression> NullableExpressionMethods for T {}
