use super::{HasTable, Identifiable};
use crate::dsl::{Eq, EqAny, Filter, FindBy};
use crate::expression::array_comparison::AsInExpression;
use crate::expression::AsExpression;
use crate::prelude::*;
use crate::query_dsl::methods::FilterDsl;
use crate::sql_types::SqlType;
use std::borrow::Borrow;
use std::hash::Hash;
pub trait BelongsTo<Parent> {
    type ForeignKey: Hash + ::std::cmp::Eq;
    type ForeignKeyColumn: Column;
    fn foreign_key(&self) -> Option<&Self::ForeignKey>;
    fn foreign_key_column() -> Self::ForeignKeyColumn;
}
pub trait GroupedBy<'a, Parent>: IntoIterator + Sized {
    fn grouped_by(self, parents: &'a [Parent]) -> Vec<Vec<Self::Item>>;
}
type Id<T> = <T as Identifiable>::Id;
impl<'a, Parent: 'a, Child, Iter> GroupedBy<'a, Parent> for Iter
where
    Iter: IntoIterator<Item = Child>,
    Child: BelongsTo<Parent>,
    &'a Parent: Identifiable,
    Id<&'a Parent>: Borrow<Child::ForeignKey>,
{
    fn grouped_by(self, parents: &'a [Parent]) -> Vec<Vec<Child>> {
        loop {}
    }
}
impl<'a, Parent, Child> BelongingToDsl<&'a Parent> for Child
where
    &'a Parent: Identifiable,
    Child: HasTable + BelongsTo<Parent>,
    Id<&'a Parent>: AsExpression<<Child::ForeignKeyColumn as Expression>::SqlType>,
    Child::Table: FilterDsl<Eq<Child::ForeignKeyColumn, Id<&'a Parent>>>,
    Child::ForeignKeyColumn: ExpressionMethods,
    <Child::ForeignKeyColumn as Expression>::SqlType: SqlType,
{
    type Output = FindBy<Child::Table, Child::ForeignKeyColumn, Id<&'a Parent>>;
    fn belonging_to(parent: &'a Parent) -> Self::Output {
        loop {}
    }
}
impl<'a, Parent, Child> BelongingToDsl<&'a [Parent]> for Child
where
    &'a Parent: Identifiable,
    Child: HasTable + BelongsTo<Parent>,
    Vec<
        Id<&'a Parent>,
    >: AsInExpression<<Child::ForeignKeyColumn as Expression>::SqlType>,
    <Child as HasTable>::Table: FilterDsl<
        EqAny<Child::ForeignKeyColumn, Vec<Id<&'a Parent>>>,
    >,
    Child::ForeignKeyColumn: ExpressionMethods,
    <Child::ForeignKeyColumn as Expression>::SqlType: SqlType,
{
    type Output = Filter<
        Child::Table,
        EqAny<Child::ForeignKeyColumn, Vec<Id<&'a Parent>>>,
    >;
    fn belonging_to(parents: &'a [Parent]) -> Self::Output {
        loop {}
    }
}
impl<'a, Parent, Child> BelongingToDsl<&'a Vec<Parent>> for Child
where
    Child: BelongingToDsl<&'a [Parent]>,
{
    type Output = Child::Output;
    fn belonging_to(parents: &'a Vec<Parent>) -> Self::Output {
        loop {}
    }
}
