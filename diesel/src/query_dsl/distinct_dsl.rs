use crate::dsl;
#[cfg(feature = "postgres_backend")]
use crate::expression::SelectableExpression;
use crate::expression::TypedExpressionType;
use crate::expression::ValidGrouping;
use crate::query_builder::FromClause;
use crate::query_builder::{AsQuery, SelectStatement};
use crate::query_source::Table;
use crate::Expression;
pub trait DistinctDsl {
        type Output;
        fn distinct(self) -> dsl::Distinct<Self>;
}
impl<T> DistinctDsl for T
where
    T: Table + AsQuery<Query = SelectStatement<FromClause<T>>>,
    T::DefaultSelection: Expression<SqlType = T::SqlType> + ValidGrouping<()>,
    T::SqlType: TypedExpressionType,
{
    type Output = dsl::Distinct<SelectStatement<FromClause<T>>>;
    fn distinct(self) -> dsl::Distinct<SelectStatement<FromClause<T>>> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
pub trait DistinctOnDsl<Selection> {
        type Output;
        fn distinct_on(self, selection: Selection) -> dsl::DistinctOn<Self, Selection>;
}
#[cfg(feature = "postgres_backend")]
impl<T, Selection> DistinctOnDsl<Selection> for T
where
    Selection: SelectableExpression<T>,
    Selection::SqlType: crate::sql_types::SingleValue,
    T: Table + AsQuery<Query = SelectStatement<FromClause<T>>>,
    SelectStatement<FromClause<T>>: DistinctOnDsl<Selection>,
    T::DefaultSelection: Expression<SqlType = T::SqlType> + ValidGrouping<()>,
    T::SqlType: TypedExpressionType,
{
    type Output = dsl::DistinctOn<SelectStatement<FromClause<T>>, Selection>;
    fn distinct_on(self, selection: Selection) -> dsl::DistinctOn<Self, Selection> {
        loop {}
    }
}
