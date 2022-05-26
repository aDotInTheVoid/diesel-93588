use crate::backend::{Backend, DieselReserveSpecialization};
use crate::expression::NonAggregate;
use crate::query_builder::insert_statement::InsertFromSelect;
use crate::query_builder::{AsQuery, AstPass, Query, QueryFragment, QueryId};
use crate::{CombineDsl, Insertable, QueryResult, RunQueryDsl, Table};
#[derive(Debug, Clone, Copy, QueryId)]
pub(crate) struct NoCombinationClause;
impl<DB> QueryFragment<DB> for NoCombinationClause
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, _: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Copy, Clone, QueryId)]
#[must_use = "Queries are only executed when calling `load`, `get_result` or similar."]
pub struct CombinationClause<Combinator, Rule, Source, Rhs> {
    combinator: Combinator,
    duplicate_rule: Rule,
    source: ParenthesisWrapper<Source>,
    rhs: ParenthesisWrapper<Rhs>,
}
impl<Combinator, Rule, Source, Rhs> CombinationClause<Combinator, Rule, Source, Rhs> {
    pub(crate) fn new(
        combinator: Combinator,
        duplicate_rule: Rule,
        source: Source,
        rhs: Rhs,
    ) -> Self {
        loop {}
    }
}
impl<Combinator, Rule, Source, Rhs> Query for CombinationClause<Combinator, Rule, Source, Rhs>
where
    Source: Query,
    Rhs: Query<SqlType = Source::SqlType>,
{
    type SqlType = Source::SqlType;
}
impl<Combinator, Rule, Source, Rhs, Conn> RunQueryDsl<Conn>
    for CombinationClause<Combinator, Rule, Source, Rhs>
{
}
impl<Combinator, Rule, Source, Rhs, T> Insertable<T>
    for CombinationClause<Combinator, Rule, Source, Rhs>
where
    T: Table,
    T::AllColumns: NonAggregate,
    Self: Query,
{
    type Values = InsertFromSelect<Self, T::AllColumns>;
    fn values(self) -> Self::Values {
        loop {}
    }
}
impl<Combinator, Rule, Source, OriginRhs> CombineDsl
    for CombinationClause<Combinator, Rule, Source, OriginRhs>
where
    Self: Query,
{
    type Query = Self;
    fn union<Rhs>(self, rhs: Rhs) -> crate::dsl::Union<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn union_all<Rhs>(self, rhs: Rhs) -> crate::dsl::UnionAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn intersect<Rhs>(self, rhs: Rhs) -> crate::dsl::Intersect<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn intersect_all<Rhs>(self, rhs: Rhs) -> crate::dsl::IntersectAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn except<Rhs>(self, rhs: Rhs) -> crate::dsl::Except<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
    fn except_all<Rhs>(self, rhs: Rhs) -> crate::dsl::ExceptAll<Self, Rhs>
    where
        Rhs: AsQuery<SqlType = <Self::Query as Query>::SqlType>,
    {
        loop {}
    }
}
impl<Combinator, Rule, Source, Rhs, DB: Backend> QueryFragment<DB>
    for CombinationClause<Combinator, Rule, Source, Rhs>
where
    Combinator: QueryFragment<DB>,
    Rule: QueryFragment<DB>,
    ParenthesisWrapper<Source>: QueryFragment<DB>,
    ParenthesisWrapper<Rhs>: QueryFragment<DB>,
    DB: Backend + SupportsCombinationClause<Combinator, Rule> + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Copy, Clone, QueryId)]
pub struct Union;
impl<DB> QueryFragment<DB> for Union
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Copy, Clone, QueryId)]
pub struct Intersect;
impl<DB> QueryFragment<DB> for Intersect
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Copy, Clone, QueryId)]
pub struct Except;
impl<DB> QueryFragment<DB> for Except
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Copy, Clone, QueryId)]
pub struct Distinct;
impl<DB> QueryFragment<DB> for Distinct
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, _: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Copy, Clone, QueryId)]
pub struct All;
impl<DB> QueryFragment<DB> for All
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
pub trait SupportsCombinationClause<Combinator, Rule> {}
#[derive(Debug, Copy, Clone, QueryId)]
pub struct ParenthesisWrapper<T>(T);
#[cfg(feature = "postgres")]
mod postgres {
    use super::*;
    use crate::pg::Pg;
    use crate::query_builder::{AstPass, QueryFragment};
    use crate::QueryResult;
    impl<T: QueryFragment<Pg>> QueryFragment<Pg> for ParenthesisWrapper<T> {
        fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
            loop {}
        }
    }
    impl SupportsCombinationClause<Union, Distinct> for Pg {}
    impl SupportsCombinationClause<Union, All> for Pg {}
    impl SupportsCombinationClause<Intersect, Distinct> for Pg {}
    impl SupportsCombinationClause<Intersect, All> for Pg {}
    impl SupportsCombinationClause<Except, Distinct> for Pg {}
    impl SupportsCombinationClause<Except, All> for Pg {}
}
#[cfg(feature = "mysql")]
mod mysql {
    use super::*;
    use crate::mysql::Mysql;
    use crate::query_builder::{AstPass, QueryFragment};
    use crate::QueryResult;
    impl<T: QueryFragment<Mysql>> QueryFragment<Mysql> for ParenthesisWrapper<T> {
        fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
            loop {}
        }
    }
    impl SupportsCombinationClause<Union, Distinct> for Mysql {}
    impl SupportsCombinationClause<Union, All> for Mysql {}
}
#[cfg(feature = "sqlite")]
mod sqlite {
    use super::*;
    use crate::query_builder::{AstPass, QueryFragment};
    use crate::sqlite::Sqlite;
    use crate::QueryResult;
    impl<T: QueryFragment<Sqlite>> QueryFragment<Sqlite> for ParenthesisWrapper<T> {
        fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Sqlite>) -> QueryResult<()> {
            loop {}
        }
    }
    impl SupportsCombinationClause<Union, Distinct> for Sqlite {}
    impl SupportsCombinationClause<Union, All> for Sqlite {}
    impl SupportsCombinationClause<Intersect, Distinct> for Sqlite {}
    impl SupportsCombinationClause<Except, Distinct> for Sqlite {}
}
