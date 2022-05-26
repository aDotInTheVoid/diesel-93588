use crate::expression::SelectableExpression;
use crate::pg::Pg;
use crate::query_builder::order_clause::{NoOrderClause, OrderClause};
use crate::query_builder::{
    AstPass, FromClause, QueryFragment, QueryId, SelectQuery, SelectStatement,
};
use crate::query_dsl::methods::DistinctOnDsl;
use crate::query_dsl::order_dsl::ValidOrderingForDistinct;
use crate::result::QueryResult;
use crate::sql_types::SingleValue;
use crate::{Expression, QuerySource};
#[derive(Debug, Clone, Copy, QueryId)]
#[cfg(feature = "postgres_backend")]
pub struct DistinctOnClause<T>(pub(crate) T);
impl<T> ValidOrderingForDistinct<DistinctOnClause<T>> for NoOrderClause {}
impl<T> ValidOrderingForDistinct<DistinctOnClause<T>> for OrderClause<(T,)> {}
impl<T> ValidOrderingForDistinct<DistinctOnClause<T>> for OrderClause<T>
where
    T: Expression,
    T::SqlType: SingleValue,
{}
impl<T> ValidOrderingForDistinct<DistinctOnClause<T>>
for OrderClause<crate::helper_types::Desc<T>>
where
    T: Expression,
    T::SqlType: SingleValue,
{}
impl<T> ValidOrderingForDistinct<DistinctOnClause<T>>
for OrderClause<crate::helper_types::Asc<T>>
where
    T: Expression,
    T::SqlType: SingleValue,
{}
impl<T> ValidOrderingForDistinct<DistinctOnClause<T>>
for OrderClause<(crate::helper_types::Desc<T>,)>
where
    T: Expression,
    T::SqlType: SingleValue,
{}
impl<T> ValidOrderingForDistinct<DistinctOnClause<T>>
for OrderClause<(crate::helper_types::Asc<T>,)>
where
    T: Expression,
    T::SqlType: SingleValue,
{}
impl<T> QueryFragment<Pg> for DistinctOnClause<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
impl<ST, F, S, D, W, O, LOf, G, H, Selection> DistinctOnDsl<Selection>
for SelectStatement<FromClause<F>, S, D, W, O, LOf, G, H>
where
    F: QuerySource,
    Selection: SelectableExpression<F>,
    Selection::SqlType: SingleValue,
    Self: SelectQuery<SqlType = ST>,
    O: ValidOrderingForDistinct<DistinctOnClause<Selection>>,
    SelectStatement<
        FromClause<F>,
        S,
        DistinctOnClause<Selection>,
        W,
        O,
        LOf,
        G,
        H,
    >: SelectQuery<SqlType = ST>,
{
    type Output = SelectStatement<
        FromClause<F>,
        S,
        DistinctOnClause<Selection>,
        W,
        O,
        LOf,
        G,
        H,
    >;
    fn distinct_on(self, selection: Selection) -> Self::Output {
        loop {}
    }
}
