#[macro_use]
mod query_id;
#[macro_use]
mod clause_macro;
mod ast_pass;
pub mod bind_collector;
pub(crate) mod combination_clause;
mod debug_query;
mod delete_statement;
mod distinct_clause;
pub(crate) mod from_clause;
pub(crate) mod functions;
mod group_by_clause;
mod having_clause;
pub(crate) mod insert_statement;
pub(crate) mod limit_clause;
pub(crate) mod limit_offset_clause;
pub(crate) mod locking_clause;
pub(crate) mod nodes;
pub(crate) mod offset_clause;
pub(crate) mod order_clause;
mod returning_clause;
pub(crate) mod select_clause;
pub(crate) mod select_statement;
mod sql_query;
pub(crate) mod update_statement;
pub(crate) mod upsert;
mod where_clause;
#[doc(inline)]
pub use self::ast_pass::AstPass;
#[doc(inline)]
pub use self::bind_collector::BindCollector;
#[doc(inline)]
pub use self::debug_query::DebugQuery;
#[doc(inline)]
pub use self::delete_statement::{BoxedDeleteStatement, DeleteStatement};
#[doc(inline)]
#[diesel_derives::__diesel_public_if(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
)]
pub(crate) use self::from_clause::{FromClause, NoFromClause};
#[diesel_derives::__diesel_public_if(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
)]
#[doc(inline)]
pub(crate) use self::insert_statement::batch_insert::BatchInsert;
pub(crate) use self::insert_statement::ColumnList;
#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
#[doc(inline)]
pub use self::insert_statement::DefaultValues;
#[doc(inline)]
pub use self::insert_statement::{
    IncompleteInsertOrIgnoreStatement, IncompleteInsertStatement,
    IncompleteReplaceStatement, InsertOrIgnoreStatement, InsertStatement,
    ReplaceStatement,
};
#[diesel_derives::__diesel_public_if(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
)]
pub(crate) use self::insert_statement::{UndecoratedInsertRecord, ValuesClause};
#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
pub use self::limit_clause::{LimitClause, NoLimitClause};
#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
pub use self::limit_offset_clause::{BoxedLimitOffsetClause, LimitOffsetClause};
#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
pub use self::offset_clause::{NoOffsetClause, OffsetClause};
#[doc(inline)]
pub use self::query_id::QueryId;
#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
#[doc(inline)]
pub use self::returning_clause::ReturningClause;
#[doc(inline)]
#[diesel_derives::__diesel_public_if(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
)]
pub(crate) use self::select_clause::SelectClauseExpression;
pub(crate) use self::select_statement::BoxedSelectStatement;
pub(crate) use self::select_statement::SelectStatement;
#[doc(inline)]
pub use self::sql_query::{BoxedSqlQuery, SqlQuery};
#[doc(inline)]
pub use self::update_statement::changeset::AsChangeset;
#[doc(inline)]
pub use self::update_statement::target::{IntoUpdateTarget, UpdateTarget};
#[doc(inline)]
pub use self::update_statement::{BoxedUpdateStatement, UpdateStatement};
#[doc(inline)]
pub use self::upsert::on_conflict_target_decorations::DecoratableTarget;
use crate::backend::{Backend, HasBindCollector};
#[cfg(feature = "postgres_backend")]
pub use crate::pg::query_builder::only::Only;
use crate::result::QueryResult;
use std::error::Error;
#[doc(hidden)]
pub type Binds = Vec<Option<Vec<u8>>>;
pub type BuildQueryResult = Result<(), Box<dyn Error + Send + Sync>>;
pub trait QueryBuilder<DB: Backend> {
    fn push_sql(&mut self, sql: &str);
    fn push_identifier(&mut self, identifier: &str) -> QueryResult<()>;
    fn push_bind_param(&mut self);
    fn push_bind_param_value_only(&mut self) {}
    fn finish(self) -> String;
}
pub trait Query {
    type SqlType;
}
impl<'a, T: Query> Query for &'a T {
    type SqlType = T::SqlType;
}
pub trait SelectQuery {
    type SqlType;
}
pub trait QueryFragment<DB: Backend, SP = self::private::NotSpecialized> {
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()>;
    #[diesel_derives::__diesel_public_if(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )]
    fn to_sql(&self, out: &mut DB::QueryBuilder, backend: &DB) -> QueryResult<()> {
        self.walk_ast(AstPass::to_sql(out, backend))
    }
    #[diesel_derives::__diesel_public_if(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )]
    fn collect_binds<'b>(
        &'b self,
        out: &mut <DB as HasBindCollector<'b>>::BindCollector,
        metadata_lookup: &mut DB::MetadataLookup,
        backend: &'b DB,
    ) -> QueryResult<()> {
        self.walk_ast(AstPass::collect_binds(out, metadata_lookup, backend))
    }
    #[diesel_derives::__diesel_public_if(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )]
    fn is_safe_to_cache_prepared(&self, backend: &DB) -> QueryResult<bool> {
        let mut result = true;
        self.walk_ast(AstPass::is_safe_to_cache_prepared(&mut result, backend))?;
        Ok(result)
    }
    #[diesel_derives::__diesel_public_if(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )]
    fn is_noop(&self, backend: &DB) -> QueryResult<bool> {
        let mut result = true;
        self.walk_ast(AstPass::is_noop(&mut result, backend))?;
        Ok(result)
    }
}
impl<T: ?Sized, DB> QueryFragment<DB> for Box<T>
where
    DB: Backend,
    T: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<'a, T: ?Sized, DB> QueryFragment<DB> for &'a T
where
    DB: Backend,
    T: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<DB: Backend> QueryFragment<DB> for () {
    fn walk_ast<'b>(&'b self, _: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, DB> QueryFragment<DB> for Option<T>
where
    DB: Backend,
    T: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
pub trait IntoBoxedClause<'a, DB> {
    type BoxedClause;
    fn into_boxed(self) -> Self::BoxedClause;
}
pub trait AsQuery {
    type SqlType;
    type Query: Query<SqlType = Self::SqlType>;
    #[allow(clippy::wrong_self_convention)]
    fn as_query(self) -> Self::Query;
}
impl<T: Query> AsQuery for T {
    type SqlType = <Self as Query>::SqlType;
    type Query = Self;
    fn as_query(self) -> Self::Query {
        loop {}
    }
}
pub fn debug_query<DB, T>(query: &T) -> DebugQuery<'_, T, DB> {
    loop {}
}
mod private {
    #[allow(missing_debug_implementations, missing_copy_implementations)]
    pub struct NotSpecialized;
}
