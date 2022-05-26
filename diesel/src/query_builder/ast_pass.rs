use std::fmt;
use crate::backend::{Backend, HasBindCollector};
use crate::query_builder::{BindCollector, QueryBuilder};
use crate::result::QueryResult;
use crate::serialize::ToSql;
use crate::sql_types::HasSqlType;
#[allow(missing_debug_implementations)]
pub struct AstPass<'a, 'b, DB>
where
    DB: Backend,
    DB::QueryBuilder: 'a,
    <DB as HasBindCollector<'a>>::BindCollector: 'a,
    DB::MetadataLookup: 'a,
    'b: 'a,
{
    internals: AstPassInternals<'a, 'b, DB>,
    backend: &'b DB,
}
impl<'a, 'b, DB> AstPass<'a, 'b, DB>
where
    DB: Backend,
    'b: 'a,
{
    pub(crate) fn to_sql(
        query_builder: &'a mut DB::QueryBuilder,
        backend: &'b DB,
    ) -> Self {
        loop {}
    }
    pub(crate) fn collect_binds(
        collector: &'a mut <DB as HasBindCollector<'b>>::BindCollector,
        metadata_lookup: &'a mut DB::MetadataLookup,
        backend: &'b DB,
    ) -> Self {
        loop {}
    }
    pub(crate) fn is_safe_to_cache_prepared(
        result: &'a mut bool,
        backend: &'b DB,
    ) -> Self {
        loop {}
    }
    pub(crate) fn debug_binds(
        formatter: &'a mut Vec<&'b dyn fmt::Debug>,
        backend: &'b DB,
    ) -> Self {
        loop {}
    }
                    pub(crate) fn is_noop(result: &'a mut bool, backend: &'b DB) -> Self {
        loop {}
    }
                                        pub fn reborrow(&'_ mut self) -> AstPass<'_, 'b, DB> {
        loop {}
    }
                                                                                    pub fn unsafe_to_cache_prepared(&mut self) {
        loop {}
    }
                                                                                                    pub fn push_sql(&mut self, sql: &str) {
        loop {}
    }
                    pub fn push_identifier(&mut self, identifier: &str) -> QueryResult<()> {
        loop {}
    }
                        pub fn push_bind_param<T, U>(&mut self, bind: &'b U) -> QueryResult<()>
    where
        DB: HasSqlType<T>,
        U: ToSql<T, DB>,
    {
        loop {}
    }
    pub(crate) fn push_bind_param_value_only<T, U>(
        &mut self,
        bind: &'b U,
    ) -> QueryResult<()>
    where
        DB: HasSqlType<T>,
        U: ToSql<T, DB>,
    {
        loop {}
    }
        #[cfg_attr(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
        doc(hidden)
    )]
    #[cfg_attr(
        doc_cfg,
        doc(
            cfg(
                feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
            )
        )
    )]
    pub fn backend(&self) -> &DB {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
enum AstPassInternals<'a, 'b, DB>
where
    DB: Backend,
    DB::QueryBuilder: 'a,
    <DB as HasBindCollector<'a>>::BindCollector: 'a,
    DB::MetadataLookup: 'a,
    'b: 'a,
{
    ToSql(&'a mut DB::QueryBuilder),
    CollectBinds {
        collector: &'a mut <DB as HasBindCollector<'b>>::BindCollector,
        metadata_lookup: &'a mut DB::MetadataLookup,
    },
    IsSafeToCachePrepared(&'a mut bool),
    DebugBinds(&'a mut Vec<&'b dyn fmt::Debug>),
    IsNoop(&'a mut bool),
}
