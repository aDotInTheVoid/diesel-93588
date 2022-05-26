use super::field_alias_mapper::FieldAliasMapper;
use super::{Alias, AliasSource};
use crate::dsl;
#[cfg(feature = "postgres_backend")]
use crate::expression::SelectableExpression;
use crate::expression::{Expression, TypedExpressionType, ValidGrouping};
use crate::expression_methods::EqAll;
use crate::query_builder::{
    combination_clause, AsQuery, FromClause, Query, SelectStatement,
};
use crate::query_dsl::methods::*;
use crate::query_dsl::{CombineDsl, QueryDsl, RunQueryDsl};
use crate::query_source::{QuerySource, Table};
impl<S: AliasSource> QueryDsl for Alias<S> {}
impl<S, Predicate> FilterDsl<Predicate> for Alias<S>
where
    Self: AsQuery,
    <Self as AsQuery>::Query: FilterDsl<Predicate>,
{
    type Output = dsl::Filter<<Self as AsQuery>::Query, Predicate>;
    fn filter(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
impl<S, Selection> SelectDsl<Selection> for Alias<S>
where
    Selection: Expression,
    Self: AsQuery,
    <Self as AsQuery>::Query: SelectDsl<Selection>,
{
    type Output = dsl::Select<<Self as AsQuery>::Query, Selection>;
    fn select(self, selection: Selection) -> Self::Output {
        loop {}
    }
}
impl<S, PK> FindDsl<PK> for Alias<S>
where
    S: AliasSource,
    S::Target: Table,
    <S::Target as Table>::PrimaryKey: FieldAliasMapper<S>,
    <<S::Target as Table>::PrimaryKey as FieldAliasMapper<S>>::Out: EqAll<PK>,
    Self: FilterDsl<
        <<<S::Target as Table>::PrimaryKey as FieldAliasMapper<
            S,
        >>::Out as EqAll<PK>>::Output,
    >,
{
    type Output = dsl::Filter<
        Self,
        <<<S::Target as Table>::PrimaryKey as FieldAliasMapper<
            S,
        >>::Out as EqAll<PK>>::Output,
    >;
    fn find(self, id: PK) -> Self::Output {
        loop {}
    }
}
impl<'a, S, DB> BoxedDsl<'a, DB> for Alias<S>
where
    Alias<S>: QuerySource + AsQuery<Query = SelectStatement<FromClause<Alias<S>>>>,
    SelectStatement<FromClause<Alias<S>>>: BoxedDsl<'a, DB>,
    <Alias<
        S,
    > as QuerySource>::DefaultSelection: Expression<
            SqlType = <Alias<S> as AsQuery>::SqlType,
        > + ValidGrouping<()>,
    <Alias<S> as AsQuery>::SqlType: TypedExpressionType,
{
    type Output = dsl::IntoBoxed<'a, SelectStatement<FromClause<Alias<S>>>, DB>;
    fn internal_into_boxed(self) -> Self::Output {
        loop {}
    }
}
impl<S> CombineDsl for Alias<S>
where
    S: AliasSource,
    S::Target: Table,
    Self: AsQuery,
{
    type Query = <Self as AsQuery>::Query;
    fn union<Rhs>(self, rhs: Rhs) -> dsl::Union<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <<Self as AsQuery>::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn union_all<Rhs>(self, rhs: Rhs) -> dsl::UnionAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <<Self as AsQuery>::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn intersect<Rhs>(self, rhs: Rhs) -> dsl::Intersect<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <<Self as AsQuery>::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn intersect_all<Rhs>(self, rhs: Rhs) -> dsl::IntersectAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <<Self as AsQuery>::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn except<Rhs>(self, rhs: Rhs) -> dsl::Except<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <<Self as AsQuery>::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn except_all<Rhs>(self, rhs: Rhs) -> dsl::ExceptAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <<Self as AsQuery>::Query as Query>::SqlType>,
    {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl<S, Selection> DistinctOnDsl<Selection> for Alias<S>
where
    S: AliasSource,
    Selection: SelectableExpression<Self>,
    Selection::SqlType: crate::sql_types::SingleValue,
    Self: QuerySource + AsQuery<Query = SelectStatement<FromClause<Self>>>,
    SelectStatement<FromClause<Self>>: DistinctOnDsl<Selection>,
    <Self as QuerySource>::DefaultSelection: Expression<
            SqlType = <Self as AsQuery>::SqlType,
        > + ValidGrouping<()>,
    <Self as AsQuery>::SqlType: TypedExpressionType,
{
    type Output = dsl::DistinctOn<SelectStatement<FromClause<Self>>, Selection>;
    fn distinct_on(self, selection: Selection) -> dsl::DistinctOn<Self, Selection> {
        loop {}
    }
}
impl<S, Predicate> OrFilterDsl<Predicate> for Alias<S>
where
    Self: AsQuery,
    <Self as AsQuery>::Query: OrFilterDsl<Predicate>,
{
    type Output = dsl::OrFilter<<Self as AsQuery>::Query, Predicate>;
    fn or_filter(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
impl<S, Expr> GroupByDsl<Expr> for Alias<S>
where
    Expr: Expression,
    Self: QuerySource + AsQuery<Query = SelectStatement<FromClause<Self>>>,
    <Self as QuerySource>::DefaultSelection: Expression<
            SqlType = <Self as AsQuery>::SqlType,
        > + ValidGrouping<()>,
    <Self as AsQuery>::SqlType: TypedExpressionType,
{
    type Output = dsl::GroupBy<SelectStatement<FromClause<Self>>, Expr>;
    fn group_by(self, expr: Expr) -> dsl::GroupBy<Self, Expr> {
        loop {}
    }
}
impl<S> LimitDsl for Alias<S>
where
    Self: AsQuery,
    <Self as AsQuery>::Query: LimitDsl,
{
    type Output = <<Self as AsQuery>::Query as LimitDsl>::Output;
    fn limit(self, limit: i64) -> Self::Output {
        loop {}
    }
}
impl<S, Lock> LockingDsl<Lock> for Alias<S>
where
    Self: QuerySource + AsQuery<Query = SelectStatement<FromClause<Self>>>,
    <Self as QuerySource>::DefaultSelection: Expression<
            SqlType = <Self as AsQuery>::SqlType,
        > + ValidGrouping<()>,
    <Self as AsQuery>::SqlType: TypedExpressionType,
{
    type Output = <SelectStatement<FromClause<Self>> as LockingDsl<Lock>>::Output;
    fn with_lock(self, lock: Lock) -> Self::Output {
        loop {}
    }
}
impl<S: AliasSource, Conn> RunQueryDsl<Conn> for Alias<S> {}
impl<S> OffsetDsl for Alias<S>
where
    Self: AsQuery,
    <Self as AsQuery>::Query: OffsetDsl,
{
    type Output = <<Self as AsQuery>::Query as OffsetDsl>::Output;
    fn offset(self, offset: i64) -> Self::Output {
        loop {}
    }
}
impl<S, Expr> OrderDsl<Expr> for Alias<S>
where
    Expr: Expression,
    Self: AsQuery,
    <Self as AsQuery>::Query: OrderDsl<Expr>,
{
    type Output = <<Self as AsQuery>::Query as OrderDsl<Expr>>::Output;
    fn order(self, expr: Expr) -> Self::Output {
        loop {}
    }
}
impl<S, Expr> ThenOrderDsl<Expr> for Alias<S>
where
    Expr: Expression,
    Self: AsQuery,
    <Self as AsQuery>::Query: ThenOrderDsl<Expr>,
{
    type Output = <<Self as AsQuery>::Query as ThenOrderDsl<Expr>>::Output;
    fn then_order_by(self, expr: Expr) -> Self::Output {
        loop {}
    }
}
