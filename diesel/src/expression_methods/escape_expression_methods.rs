use crate::dsl;
use crate::expression::grouped::Grouped;
use crate::expression::operators::{Escape, Like, NotLike};
use crate::expression::IntoSql;
use crate::sql_types::VarChar;
pub trait EscapeExpressionMethods: Sized {
    #[doc(hidden)]
    type TextExpression;
    fn escape(self, _character: char) -> dsl::Escape<Self>;
}
impl<T, U> EscapeExpressionMethods for Grouped<Like<T, U>> {
    type TextExpression = Like<T, U>;
    fn escape(self, character: char) -> dsl::Escape<Self> {
        loop {}
    }
}
impl<T, U> EscapeExpressionMethods for Grouped<NotLike<T, U>> {
    type TextExpression = NotLike<T, U>;
    fn escape(self, character: char) -> dsl::Escape<Self> {
        loop {}
    }
}
