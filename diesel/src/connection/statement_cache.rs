use crate::backend::Backend;
use crate::query_builder::*;
use crate::result::QueryResult;
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
#[allow(missing_debug_implementations, unreachable_pub)]
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
pub struct StatementCache<DB: Backend, Statement> {
    pub(crate) cache: HashMap<StatementCacheKey<DB>, Statement>,
}
#[derive(Debug, Clone, Copy)]
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
#[allow(unreachable_pub)]
pub enum PrepareForCache {
    Yes,
    No,
}
#[allow(
    clippy::len_without_is_empty,
    clippy::new_without_default,
    unreachable_pub
)]
impl<DB, Statement> StatementCache<DB, Statement>
where
    DB: Backend,
    DB::TypeMetadata: Clone,
    DB::QueryBuilder: Default,
    StatementCacheKey<DB>: Hash + Eq,
{
    #[allow(unreachable_pub)]
    pub fn new() -> Self {
        loop {}
    }
    #[allow(unreachable_pub)]
    #[cfg(any(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        feature = "postgres",
        all(feature = "sqlite", test)
    ))]
    #[cfg_attr(
        doc_cfg,
        doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
    )]
    pub fn len(&self) -> usize {
        loop {}
    }
    #[allow(unreachable_pub)]
    pub fn cached_statement<T, F>(
        &mut self,
        source: &T,
        backend: &DB,
        bind_types: &[DB::TypeMetadata],
        prepare_fn: F,
    ) -> QueryResult<MaybeCached<'_, Statement>>
    where
        T: QueryFragment<DB> + QueryId,
        F: FnOnce(&str, PrepareForCache) -> QueryResult<Statement>,
    {
        loop {}
    }
}
#[allow(missing_debug_implementations, unreachable_pub)]
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
#[non_exhaustive]
pub enum MaybeCached<'a, T: 'a> {
    CannotCache(T),
    Cached(&'a mut T),
}
impl<'a, T> Deref for MaybeCached<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        loop {}
    }
}
impl<'a, T> DerefMut for MaybeCached<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        loop {}
    }
}
#[allow(missing_debug_implementations, unreachable_pub)]
#[derive(Hash, PartialEq, Eq)]
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
pub enum StatementCacheKey<DB: Backend> {
    Type(TypeId),
    Sql {
        sql: String,
        bind_types: Vec<DB::TypeMetadata>,
    },
}
impl<DB> StatementCacheKey<DB>
where
    DB: Backend,
    DB::QueryBuilder: Default,
    DB::TypeMetadata: Clone,
{
    #[allow(unreachable_pub)]
    pub fn for_source<T>(
        source: &T,
        bind_types: &[DB::TypeMetadata],
        backend: &DB,
    ) -> QueryResult<Self>
    where
        T: QueryFragment<DB> + QueryId,
    {
        loop {}
    }
    #[allow(unreachable_pub)]
    pub fn sql<T: QueryFragment<DB>>(&self, source: &T, backend: &DB) -> QueryResult<Cow<'_, str>> {
        loop {}
    }
    fn construct_sql<T: QueryFragment<DB>>(source: &T, backend: &DB) -> QueryResult<String> {
        loop {}
    }
}
