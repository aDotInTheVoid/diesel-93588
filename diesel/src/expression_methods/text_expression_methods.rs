use self::private::TextOrNullableText;
use crate::dsl;
use crate::expression::grouped::Grouped;
use crate::expression::operators::{Concat, Like, NotLike};
use crate::expression::{AsExpression, Expression};
use crate::sql_types::SqlType;
pub trait TextExpressionMethods: Expression + Sized {
    fn concat<T>(self, other: T) -> dsl::Concat<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    fn like<T>(self, other: T) -> dsl::Like<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
    fn not_like<T>(self, other: T) -> dsl::NotLike<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        loop {}
    }
}
impl<T> TextExpressionMethods for T
where
    T: Expression,
    T::SqlType: TextOrNullableText,
{}
mod private {
    use crate::sql_types::{Nullable, Text};
    pub trait TextOrNullableText {}
    impl TextOrNullableText for Text {}
    impl TextOrNullableText for Nullable<Text> {}
}
