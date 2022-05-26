use super::connection::{SqliteBindCollector, SqliteValue};
use super::query_builder::SqliteQueryBuilder;
use crate::backend::*;
use crate::sql_types::TypeMetadata;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Sqlite;
#[allow(missing_debug_implementations)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SqliteType {
        Binary,
        Text,
        Float,
        Double,
        SmallInt,
        Integer,
        Long,
}
impl Backend for Sqlite {
    type QueryBuilder = SqliteQueryBuilder;
}
impl<'a> HasBindCollector<'a> for Sqlite {
    type BindCollector = SqliteBindCollector<'a>;
}
impl<'a> HasRawValue<'a> for Sqlite {
    type RawValue = SqliteValue<'a, 'a, 'a>;
}
impl TypeMetadata for Sqlite {
    type TypeMetadata = SqliteType;
    type MetadataLookup = ();
}
impl SqlDialect for Sqlite {
    #[cfg(not(feature = "returning_clauses_for_sqlite_3_35"))]
    type ReturningClause = sql_dialect::returning_clause::DoesNotSupportReturningClause;
    #[cfg(feature = "returning_clauses_for_sqlite_3_35")]
    type ReturningClause = sql_dialect::returning_clause::PgLikeReturningClause;
    type OnConflictClause = SqliteOnConflictClaues;
    type InsertWithDefaultKeyword = sql_dialect::default_keyword_for_insert::DoesNotSupportDefaultKeyword;
    type BatchInsertSupport = SqliteBatchInsert;
    type DefaultValueClauseForInsert = sql_dialect::default_value_clause::AnsiDefaultValueClause;
    type EmptyFromClauseSyntax = sql_dialect::from_clause_syntax::AnsiSqlFromClauseSyntax;
    type ExistsSyntax = sql_dialect::exists_syntax::AnsiSqlExistsSyntax;
    type ArrayComparision = sql_dialect::array_comparision::AnsiSqlArrayComparison;
}
impl DieselReserveSpecialization for Sqlite {}
impl TrustedBackend for Sqlite {}
#[derive(Debug, Copy, Clone)]
pub struct SqliteOnConflictClaues;
impl sql_dialect::on_conflict_clause::SupportsOnConflictClause
for SqliteOnConflictClaues {}
#[derive(Debug, Copy, Clone)]
pub struct SqliteBatchInsert;
