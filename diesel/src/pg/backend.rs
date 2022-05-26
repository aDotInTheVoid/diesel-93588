use super::query_builder::PgQueryBuilder;
use super::{PgMetadataLookup, PgValue};
use crate::backend::*;
use crate::deserialize::Queryable;
use crate::pg::metadata_lookup::PgMetadataCacheKey;
use crate::query_builder::bind_collector::RawBytesBindCollector;
use crate::sql_types::TypeMetadata;
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct Pg;
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Queryable)]
pub struct InnerPgTypeMetadata {
    pub(in crate::pg) oid: u32,
    pub(in crate::pg) array_oid: u32,
}
impl From<(u32, u32)> for InnerPgTypeMetadata {
    fn from((oid, array_oid): (u32, u32)) -> Self {
        loop {}
    }
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
)]
#[allow(unreachable_pub)]
pub struct FailedToLookupTypeError(Box<PgMetadataCacheKey<'static>>);
impl FailedToLookupTypeError {
    #[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
    pub fn new(cache_key: PgMetadataCacheKey<'static>) -> Self {
        loop {}
    }
    pub(in crate::pg) fn new_internal(cache_key: PgMetadataCacheKey<'static>) -> Self {
        loop {}
    }
}
impl std::error::Error for FailedToLookupTypeError {}
impl std::fmt::Display for FailedToLookupTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct PgTypeMetadata(
    pub(in crate::pg) Result<InnerPgTypeMetadata, FailedToLookupTypeError>,
);
impl PgTypeMetadata {
    pub fn new(type_oid: u32, array_oid: u32) -> Self {
        loop {}
    }
    #[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
    pub fn from_result(r: Result<(u32, u32), FailedToLookupTypeError>) -> Self {
        loop {}
    }
    pub fn oid(&self) -> Result<u32, impl std::error::Error + Send + Sync> {
        loop {}
    }
    pub fn array_oid(&self) -> Result<u32, impl std::error::Error + Send + Sync> {
        loop {}
    }
}
impl Backend for Pg {
    type QueryBuilder = PgQueryBuilder;
}
impl<'a> HasBindCollector<'a> for Pg {
    type BindCollector = RawBytesBindCollector<Pg>;
}
impl<'a> HasRawValue<'a> for Pg {
    type RawValue = PgValue<'a>;
}
impl TypeMetadata for Pg {
    type TypeMetadata = PgTypeMetadata;
    type MetadataLookup = dyn PgMetadataLookup;
}
impl SqlDialect for Pg {
    type ReturningClause = sql_dialect::returning_clause::PgLikeReturningClause;
    type OnConflictClause = PgOnConflictClaues;
    type InsertWithDefaultKeyword = sql_dialect::default_keyword_for_insert::IsoSqlDefaultKeyword;
    type BatchInsertSupport = sql_dialect::batch_insert_support::PostgresLikeBatchInsertSupport;
    type DefaultValueClauseForInsert = sql_dialect::default_value_clause::AnsiDefaultValueClause;
    type EmptyFromClauseSyntax = sql_dialect::from_clause_syntax::AnsiSqlFromClauseSyntax;
    type ExistsSyntax = sql_dialect::exists_syntax::AnsiSqlExistsSyntax;
    type ArrayComparision = PgStyleArrayComparision;
}
impl DieselReserveSpecialization for Pg {}
impl TrustedBackend for Pg {}
#[derive(Debug, Copy, Clone)]
pub struct PgOnConflictClaues;
impl sql_dialect::on_conflict_clause::SupportsOnConflictClause for PgOnConflictClaues {}
#[derive(Debug, Copy, Clone)]
pub struct PgStyleArrayComparision;
