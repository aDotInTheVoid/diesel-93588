#[macro_use]
pub(crate) mod ops;
#[cfg(not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))]
pub(crate) mod array_comparison;
#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
pub mod array_comparison;
pub(crate) mod assume_not_null;
pub(crate) mod bound;
mod coerce;
pub(crate) mod count;
#[cfg(not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))]
mod exists;
#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
pub mod exists;
pub mod functions;
pub(crate) mod grouped;
pub(crate) mod helper_types;
mod not;
pub(crate) mod nullable;
#[macro_use]
pub(crate) mod operators;
pub(crate) mod select_by;
mod sql_literal;
pub(crate) mod subselect;
#[allow(non_camel_case_types, unreachable_pub)]
pub(crate) mod dsl {
    #[doc(inline)]
    pub use super::count::*;
    #[doc(inline)]
    pub use super::exists::exists;
    #[doc(inline)]
    pub use super::functions::aggregate_folding::*;
    #[doc(inline)]
    pub use super::functions::aggregate_ordering::*;
    #[doc(inline)]
    pub use super::functions::date_and_time::*;
    #[doc(inline)]
    pub use super::not::not;
    #[doc(inline)]
    pub use super::sql_literal::sql;
    use crate::dsl::SqlTypeOf;
    #[cfg(feature = "postgres_backend")]
    pub use crate::pg::expression::dsl::*;
    pub type count<Expr> = super::count::count::HelperType<SqlTypeOf<Expr>, Expr>;
    pub type count_star = super::count::CountStar;
    pub type count_distinct<Expr> = super::count::CountDistinct<SqlTypeOf<Expr>, Expr>;
    pub type date<Expr> = super::functions::date_and_time::date::HelperType<Expr>;
}
#[doc(inline)]
pub use self::sql_literal::{SqlLiteral, UncheckedBind};
use crate::backend::Backend;
use crate::dsl::{AsExprOf, AsSelect};
use crate::sql_types::{HasSqlType, SingleValue, SqlType};
pub trait Expression {
    type SqlType: TypedExpressionType;
}
pub trait TypedExpressionType {}
pub mod expression_types {
    use super::{QueryMetadata, TypedExpressionType};
    use crate::backend::Backend;
    use crate::sql_types::SingleValue;
    #[derive(Clone, Copy, Debug)]
    pub struct Untyped;
    #[derive(Debug, Clone, Copy)]
    pub struct NotSelectable;
    impl TypedExpressionType for Untyped {}
    impl TypedExpressionType for NotSelectable {}
    impl<ST> TypedExpressionType for ST where ST: SingleValue {}
    impl<DB: Backend> QueryMetadata<Untyped> for DB {
        fn row_metadata(_: &mut DB::MetadataLookup, row: &mut Vec<Option<DB::TypeMetadata>>) {
            loop {}
        }
    }
}
impl<T: Expression + ?Sized> Expression for Box<T> {
    type SqlType = T::SqlType;
}
impl<'a, T: Expression + ?Sized> Expression for &'a T {
    type SqlType = T::SqlType;
}
pub trait QueryMetadata<T>: Backend {
    fn row_metadata(lookup: &mut Self::MetadataLookup, out: &mut Vec<Option<Self::TypeMetadata>>);
}
impl<T, DB> QueryMetadata<T> for DB
where
    DB: Backend + HasSqlType<T>,
    T: SingleValue,
{
    fn row_metadata(lookup: &mut Self::MetadataLookup, out: &mut Vec<Option<Self::TypeMetadata>>) {
        loop {}
    }
}
pub trait AsExpression<T>
where
    T: SqlType + TypedExpressionType,
{
    type Expression: Expression<SqlType = T>;
    #[allow(clippy::wrong_self_convention)]
    fn as_expression(self) -> Self::Expression;
}
#[doc(inline)]
pub use diesel_derives::AsExpression;
impl<T, ST> AsExpression<ST> for T
where
    T: Expression<SqlType = ST>,
    ST: SqlType + TypedExpressionType,
{
    type Expression = Self;
    fn as_expression(self) -> Self {
        loop {}
    }
}
pub trait IntoSql {
    fn into_sql<T>(self) -> AsExprOf<Self, T>
    where
        Self: AsExpression<T> + Sized,
        T: SqlType + TypedExpressionType,
    {
        self.as_expression()
    }
    fn as_sql<'a, T>(&'a self) -> AsExprOf<&'a Self, T>
    where
        &'a Self: AsExpression<T>,
        T: SqlType + TypedExpressionType,
    {
        <&'a Self as AsExpression<T>>::as_expression(self)
    }
}
impl<T> IntoSql for T {}
pub trait AppearsOnTable<QS: ?Sized>: Expression {}
impl<T: ?Sized, QS> AppearsOnTable<QS> for Box<T>
where
    T: AppearsOnTable<QS>,
    Box<T>: Expression,
{
}
impl<'a, T: ?Sized, QS> AppearsOnTable<QS> for &'a T
where
    T: AppearsOnTable<QS>,
    &'a T: Expression,
{
}
pub trait SelectableExpression<QS: ?Sized>: AppearsOnTable<QS> {}
impl<T: ?Sized, QS> SelectableExpression<QS> for Box<T>
where
    T: SelectableExpression<QS>,
    Box<T>: AppearsOnTable<QS>,
{
}
impl<'a, T: ?Sized, QS> SelectableExpression<QS> for &'a T
where
    T: SelectableExpression<QS>,
    &'a T: AppearsOnTable<QS>,
{
}
pub trait Selectable<DB: Backend> {
    type SelectExpression: Expression;
    fn construct_selection() -> Self::SelectExpression;
}
#[doc(inline)]
pub use diesel_derives::Selectable;
pub trait SelectableHelper<DB: Backend>: Selectable<DB> + Sized {
    fn as_select() -> AsSelect<Self, DB>;
    fn as_returning() -> AsSelect<Self, DB> {
        Self::as_select()
    }
}
impl<T, DB> SelectableHelper<DB> for T
where
    T: Selectable<DB>,
    DB: Backend,
{
    fn as_select() -> AsSelect<Self, DB> {
        loop {}
    }
}
pub trait ValidGrouping<GroupByClause> {
    type IsAggregate;
}
impl<T: ValidGrouping<GB> + ?Sized, GB> ValidGrouping<GB> for Box<T> {
    type IsAggregate = T::IsAggregate;
}
impl<'a, T: ValidGrouping<GB> + ?Sized, GB> ValidGrouping<GB> for &'a T {
    type IsAggregate = T::IsAggregate;
}
#[doc(inline)]
pub use diesel_derives::ValidGrouping;
#[doc(hidden)]
pub trait IsContainedInGroupBy<T> {
    type Output;
}
#[doc(hidden)]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub mod is_contained_in_group_by {
    pub struct Yes;
    pub struct No;
    pub trait IsAny<O> {
        type Output;
    }
    impl<T> IsAny<T> for Yes {
        type Output = Yes;
    }
    impl IsAny<Yes> for No {
        type Output = Yes;
    }
    impl IsAny<No> for No {
        type Output = No;
    }
}
pub trait MixedAggregates<Other> {
    type Output;
}
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub mod is_aggregate {
    use super::MixedAggregates;
    pub struct Yes;
    pub struct No;
    pub struct Never;
    impl MixedAggregates<Yes> for Yes {
        type Output = Yes;
    }
    impl MixedAggregates<Never> for Yes {
        type Output = Yes;
    }
    impl MixedAggregates<No> for No {
        type Output = No;
    }
    impl MixedAggregates<Never> for No {
        type Output = No;
    }
    impl<T> MixedAggregates<T> for Never {
        type Output = T;
    }
}
#[cfg(feature = "unstable")]
pub trait NonAggregate = ValidGrouping<()>
where
    <Self as ValidGrouping<()>>::IsAggregate:
        MixedAggregates<is_aggregate::No, Output = is_aggregate::No>;
#[cfg(not(feature = "unstable"))]
pub trait NonAggregate: ValidGrouping<()> {}
#[cfg(not(feature = "unstable"))]
impl<T> NonAggregate for T
where
    T: ValidGrouping<()>,
    T::IsAggregate: MixedAggregates<is_aggregate::No, Output = is_aggregate::No>,
{
}
use crate::query_builder::{QueryFragment, QueryId};
pub trait BoxableExpression<QS, DB, GB = (), IsAggregate = is_aggregate::No>
where
    DB: Backend,
    Self: Expression,
    Self: SelectableExpression<QS>,
    Self: QueryFragment<DB>,
    Self: Send,
{
}
impl<QS, T, DB, GB, IsAggregate> BoxableExpression<QS, DB, GB, IsAggregate> for T
where
    DB: Backend,
    T: Expression,
    T: SelectableExpression<QS>,
    T: ValidGrouping<GB>,
    T: QueryFragment<DB>,
    T: Send,
    T::IsAggregate: MixedAggregates<IsAggregate, Output = IsAggregate>,
{
}
impl<'a, QS, ST, DB, GB, IsAggregate> QueryId
    for dyn BoxableExpression<QS, DB, GB, IsAggregate, SqlType = ST> + 'a
{
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}
impl<'a, QS, ST, DB, GB, IsAggregate> ValidGrouping<GB>
    for dyn BoxableExpression<QS, DB, GB, IsAggregate, SqlType = ST> + 'a
{
    type IsAggregate = IsAggregate;
}
pub trait AsExpressionList<ST> {
    type Expression;
    #[allow(clippy::wrong_self_convention)]
    fn as_expression_list(self) -> Self::Expression;
}
