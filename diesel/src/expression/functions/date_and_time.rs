use crate::backend::Backend;
use crate::expression::coerce::Coerce;
use crate::expression::functions::sql_function;
use crate::expression::{AsExpression, Expression, ValidGrouping};
use crate::query_builder::*;
use crate::result::QueryResult;
use crate::sql_types::*;
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, QueryId, ValidGrouping)]
pub struct now;
impl Expression for now {
    type SqlType = Timestamp;
}
impl<DB: Backend> QueryFragment<DB> for now {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl_selectable_expression!(now);
operator_allowed!(now, Add, add);
operator_allowed!(now, Sub, sub);
sql_function! {
    #[doc = " Represents the SQL `DATE` function. The argument should be a Timestamp"]
    #[doc = " expression, and the return value will be an expression of type Date."]
    #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
    " # include!(\"../../doctest_setup.rs\");"] #[doc =
    " # use diesel::dsl::{now, date};"] #[doc = " # use diesel::deserialize::Queryable;"]
    #[doc = " #"] #[doc =
    " # fn test<R: Queryable<diesel::sql_types::Date, DB> + 'static>() -> QueryResult<R> {"]
    #[doc = " #     let connection = &mut establish_connection();"] #[doc =
    " let today = diesel::select(date(now)).first(connection)?;"] #[doc =
    " #     Ok(today)"] #[doc = " # }"] #[doc = " # fn main() {"] #[doc = " #"] #[doc =
    " # }"] #[doc = " ```"] fn date(expr : Timestamp) -> Date;
}
impl AsExpression<Nullable<Timestamp>> for now {
    type Expression = Coerce<now, Nullable<Timestamp>>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
#[cfg(feature = "postgres")]
impl AsExpression<Timestamptz> for now {
    type Expression = Coerce<now, Timestamptz>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
#[cfg(feature = "postgres")]
impl AsExpression<Nullable<Timestamptz>> for now {
    type Expression = Coerce<now, Nullable<Timestamptz>>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl AsExpression<TimestamptzSqlite> for now {
    type Expression = Coerce<now, TimestamptzSqlite>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl AsExpression<Nullable<TimestamptzSqlite>> for now {
    type Expression = Coerce<now, Nullable<TimestamptzSqlite>>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, QueryId, ValidGrouping)]
pub struct today;
impl Expression for today {
    type SqlType = Date;
}
impl<DB: Backend> QueryFragment<DB> for today {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl_selectable_expression!(today);
operator_allowed!(today, Add, add);
operator_allowed!(today, Sub, sub);
impl AsExpression<Nullable<Date>> for today {
    type Expression = Coerce<today, Nullable<Date>>;
    fn as_expression(self) -> Self::Expression {
        loop {}
    }
}
