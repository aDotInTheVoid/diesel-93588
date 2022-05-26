use crate::backend::{Backend, DieselReserveSpecialization};
use crate::expression::TypedExpressionType;
use crate::expression::*;
use crate::query_builder::*;
use crate::query_source::joins::ToInnerJoin;
use crate::result::QueryResult;
use crate::sql_types::{DieselNumericOps, IntoNullable};
#[doc(hidden)]
#[derive(Debug, Copy, Clone, DieselNumericOps, ValidGrouping)]
pub struct Nullable<T>(pub(crate) T);
impl<T> Nullable<T> {
    pub(crate) fn new(expr: T) -> Self {
        loop {}
    }
}
impl<T> Expression for Nullable<T>
where
    T: Expression,
    T::SqlType: IntoNullable,
    <T::SqlType as IntoNullable>::Nullable: TypedExpressionType,
{
    type SqlType = <T::SqlType as IntoNullable>::Nullable;
}
impl<T, DB> QueryFragment<DB> for Nullable<T>
where
    DB: Backend + DieselReserveSpecialization,
    T: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, QS> AppearsOnTable<QS> for Nullable<T>
where
    T: AppearsOnTable<QS>,
    Nullable<T>: Expression,
{}
impl<T: QueryId> QueryId for Nullable<T> {
    type QueryId = T::QueryId;
    const HAS_STATIC_QUERY_ID: bool = T::HAS_STATIC_QUERY_ID;
}
impl<T, QS> SelectableExpression<QS> for Nullable<T>
where
    Self: AppearsOnTable<QS>,
    QS: ToInnerJoin,
    T: SelectableExpression<QS::InnerJoin>,
{}
impl<T> SelectableExpression<NoFromClause> for Nullable<T>
where
    Self: AppearsOnTable<NoFromClause>,
{}
