#![allow(unused_parens)]
#![cfg_attr(any(feature = "huge-tables", feature = "large-tables"), allow(deprecated))]
use super::backend::{FailedToLookupTypeError, InnerPgTypeMetadata};
use super::{Pg, PgTypeMetadata};
use crate::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
#[cfg(feature = "postgres_backend")]
pub trait PgMetadataLookup {
                        fn lookup_type(&mut self, type_name: &str, schema: Option<&str>) -> PgTypeMetadata;
}
impl<T> PgMetadataLookup for T
where
    T: Connection<Backend = Pg> + GetPgMetadataCache,
{
    fn lookup_type(&mut self, type_name: &str, schema: Option<&str>) -> PgTypeMetadata {
        loop {}
    }
}
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
)]
pub trait GetPgMetadataCache {
        fn get_metadata_cache(&mut self) -> &mut PgMetadataCache;
}
fn lookup_type<T: Connection<Backend = Pg>>(
    cache_key: &PgMetadataCacheKey<'_>,
    conn: &mut T,
) -> QueryResult<InnerPgTypeMetadata> {
    loop {}
}
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
)]
pub struct PgMetadataCacheKey<'a> {
    pub(in crate::pg) schema: Option<Cow<'a, str>>,
    pub(in crate::pg) type_name: Cow<'a, str>,
}
impl<'a> PgMetadataCacheKey<'a> {
            pub fn new(schema: Option<Cow<'a, str>>, type_name: Cow<'a, str>) -> Self {
        loop {}
    }
            pub fn into_owned(self) -> PgMetadataCacheKey<'static> {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
#[derive(Default)]
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
)]
pub struct PgMetadataCache {
    cache: HashMap<PgMetadataCacheKey<'static>, InnerPgTypeMetadata>,
}
impl PgMetadataCache {
        pub fn new() -> Self {
        loop {}
    }
        pub fn lookup_type(
        &self,
        type_name: &PgMetadataCacheKey<'_>,
    ) -> Option<PgTypeMetadata> {
        loop {}
    }
        pub fn store_type(
        &mut self,
        type_name: PgMetadataCacheKey<'_>,
        type_metadata: impl Into<InnerPgTypeMetadata>,
    ) {
        loop {}
    }
}
table! {
    pg_type(oid) { oid -> Oid, typname -> Text, typarray -> Oid, typnamespace -> Oid, }
}
table! {
    pg_namespace(oid) { oid -> Oid, nspname -> Text, }
}
joinable!(pg_type -> pg_namespace(typnamespace));
allow_tables_to_appear_in_same_query!(pg_type, pg_namespace);
sql_function! {
    fn pg_my_temp_schema() -> Oid;
}
