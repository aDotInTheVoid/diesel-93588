use crate::dsl;
use crate::query_builder::combination_clause::{
    All, CombinationClause, Distinct, Except, Intersect, Union,
};
use crate::query_builder::{AsQuery, Query};
use crate::Table;
pub trait CombineDsl {
    type Query: Query;
    fn union<Rhs>(self, rhs: Rhs) -> dsl::Union<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>;
    fn union_all<Rhs>(self, rhs: Rhs) -> dsl::UnionAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>;
    fn intersect<Rhs>(self, rhs: Rhs) -> dsl::Intersect<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>;
    fn intersect_all<Rhs>(self, rhs: Rhs) -> dsl::IntersectAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>;
    fn except<Rhs>(self, rhs: Rhs) -> dsl::Except<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>;
    fn except_all<Rhs>(self, rhs: Rhs) -> dsl::ExceptAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>;
}
impl<T: Table> CombineDsl for T {
    type Query = T::Query;
    fn union<Rhs>(self, rhs: Rhs) -> dsl::Union<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn union_all<Rhs>(self, rhs: Rhs) -> dsl::UnionAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn intersect<Rhs>(self, rhs: Rhs) -> dsl::Intersect<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn intersect_all<Rhs>(self, rhs: Rhs) -> dsl::IntersectAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn except<Rhs>(self, rhs: Rhs) -> dsl::Except<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn except_all<Rhs>(self, rhs: Rhs) -> dsl::ExceptAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
}
