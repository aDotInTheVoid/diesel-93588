use super::functions::sql_function;
use super::{is_aggregate, AsExpression};
use super::{Expression, ValidGrouping};
use crate::backend::Backend;
use crate::query_builder::*;
use crate::result::QueryResult;
use crate::sql_types::{BigInt, DieselNumericOps, SingleValue, SqlType};
use crate::{AppearsOnTable, SelectableExpression};
use std::marker::PhantomData;
sql_function! {
    #[doc = " Creates a SQL `COUNT` expression"] #[doc = ""] #[doc =
    " As with most bare functions, this is not exported by default. You can import"]
    #[doc = " it specifically as `diesel::dsl::count`, or glob import"] #[doc =
    " `diesel::dsl::*`"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
    " ```rust"] #[doc = " # include!(\"../doctest_setup.rs\");"] #[doc =
    " # use diesel::dsl::*;"] #[doc = " #"] #[doc = " # fn main() {"] #[doc =
    " #     use schema::animals::dsl::*;"] #[doc =
    " #     let connection = &mut establish_connection();"] #[doc =
    " assert_eq!(Ok(1), animals.select(count(name)).first(connection));"] #[doc = " # }"]
    #[doc = " ```"] #[aggregate] fn count < T : SqlType + SingleValue > (expr : T) ->
    BigInt;
}
pub fn count_star() -> CountStar {
    loop {}
}
#[derive(Debug, Clone, Copy, QueryId, DieselNumericOps, ValidGrouping)]
#[diesel(aggregate)]
#[doc(hidden)]
pub struct CountStar;
impl Expression for CountStar {
    type SqlType = BigInt;
}
impl<DB: Backend> QueryFragment<DB> for CountStar {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl_selectable_expression!(CountStar);
pub fn count_distinct<T, E>(expr: E) -> CountDistinct<T, E::Expression>
where
    T: SqlType + SingleValue,
    E: AsExpression<T>,
{
    loop {}
}
#[derive(Debug, Clone, Copy, QueryId, DieselNumericOps)]
#[doc(hidden)]
pub struct CountDistinct<T, E> {
    expr: E,
    _marker: PhantomData<T>,
}
impl<T, E> Expression for CountDistinct<T, E>
where
    T: SqlType + SingleValue,
    E: Expression,
{
    type SqlType = BigInt;
}
impl<T, E, GB> ValidGrouping<GB> for CountDistinct<T, E>
where
    T: SqlType + SingleValue,
{
    type IsAggregate = is_aggregate::Yes;
}
impl<T, E, QS> SelectableExpression<QS> for CountDistinct<T, E>
where
    Self: AppearsOnTable<QS>,
    E: SelectableExpression<QS>,
{}
impl<T, E, QS> AppearsOnTable<QS> for CountDistinct<T, E>
where
    Self: Expression,
    E: AppearsOnTable<QS>,
{}
impl<T, E, DB> QueryFragment<DB> for CountDistinct<T, E>
where
    T: SqlType + SingleValue,
    DB: Backend,
    E: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
