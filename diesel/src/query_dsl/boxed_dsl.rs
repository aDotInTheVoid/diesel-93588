use crate::dsl;
use crate::expression::TypedExpressionType;
use crate::expression::ValidGrouping;
use crate::query_builder::AsQuery;
use crate::query_builder::FromClause;
use crate::query_builder::SelectStatement;
use crate::query_source::Table;
use crate::Expression;
pub trait BoxedDsl<'a, DB> {
        type Output;
        fn internal_into_boxed(self) -> dsl::IntoBoxed<'a, Self, DB>;
}
impl<'a, T, DB> BoxedDsl<'a, DB> for T
where
    T: Table + AsQuery<Query = SelectStatement<FromClause<T>>>,
    SelectStatement<FromClause<T>>: BoxedDsl<'a, DB>,
    T::DefaultSelection: Expression<SqlType = T::SqlType> + ValidGrouping<()>,
    T::SqlType: TypedExpressionType,
{
    type Output = dsl::IntoBoxed<'a, SelectStatement<FromClause<T>>, DB>;
    fn internal_into_boxed(self) -> Self::Output {
        loop {}
    }
}
