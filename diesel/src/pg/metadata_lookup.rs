#![allow(unused_parens)]
#![cfg_attr(any(feature = "huge-tables", feature = "large-tables"), allow(deprecated))]
use super::backend::{FailedToLookupTypeError, InnerPgTypeMetadata};
use super::{Pg, PgTypeMetadata};
use crate::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
/// Determines the OID of types at runtime
///
/// Custom implementations of `Connection<Backend = Pg>` should not implement this trait directly.
/// Instead `GetPgMetadataCache` should be implemented, afterwards the generic implementation will provide
/// the necessary functions to perform the type lookup.
#[cfg(feature = "postgres_backend")]
pub trait PgMetadataLookup {
    /// Determine the type metadata for the given `type_name`
    ///
    /// This function should only be used for user defined types, or types which
    /// come from an extension. This function may perform a SQL query to look
    /// up the type. For built-in types, a static OID should be preferred.
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
/// Gets the `PgMetadataCache` for a `Connection<Backend=Pg>`
/// so that the lookup of user defined types, or types which come from an extension can be cached.
///
/// Implementing this trait for a `Connection<Backend=Pg>` will cause `PgMetadataLookup` to be auto implemented.
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
)]
pub trait GetPgMetadataCache {
    /// Get the `PgMetadataCache`
    fn get_metadata_cache(&mut self) -> &mut PgMetadataCache;
}
fn lookup_type<T: Connection<Backend = Pg>>(
    cache_key: &PgMetadataCacheKey<'_>,
    conn: &mut T,
) -> QueryResult<InnerPgTypeMetadata> {
    loop {}
}
/// The key used to lookup cached type oid's inside of
/// a [PgMetadataCache].
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
    /// Construct a new cache key from an optional schema name and
    /// a type name
    pub fn new(schema: Option<Cow<'a, str>>, type_name: Cow<'a, str>) -> Self {
        loop {}
    }
    /// Convert the possibly borrowed version of this metadata cache key
    /// into a lifetime independ owned version
    pub fn into_owned(self) -> PgMetadataCacheKey<'static> {
        loop {}
    }
}
/// Cache for the [OIDs] of custom Postgres types
///
/// [OIDs]: https://www.postgresql.org/docs/current/static/datatype-oid.html
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
    /// Construct a new `PgMetadataCache`
    pub fn new() -> Self {
        loop {}
    }
    /// Lookup the OID of a custom type
    pub fn lookup_type(
        &self,
        type_name: &PgMetadataCacheKey<'_>,
    ) -> Option<PgTypeMetadata> {
        loop {}
    }
    /// Store the OID of a custom type
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
