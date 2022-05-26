use super::query_builder::MysqlQueryBuilder;
use super::MysqlValue;
use crate::backend::*;
use crate::query_builder::bind_collector::RawBytesBindCollector;
use crate::sql_types::TypeMetadata;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Mysql;
#[allow(missing_debug_implementations)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum MysqlType {
        Tiny,
        UnsignedTiny,
        Short,
        UnsignedShort,
        Long,
        UnsignedLong,
        LongLong,
        UnsignedLongLong,
        Float,
        Double,
        Numeric,
        Time,
        Date,
            DateTime,
            Timestamp,
        String,
        Blob,
        Bit,
        Set,
        Enum,
}
impl Backend for Mysql {
    type QueryBuilder = MysqlQueryBuilder;
}
impl<'a> HasBindCollector<'a> for Mysql {
    type BindCollector = RawBytesBindCollector<Self>;
}
impl<'a> HasRawValue<'a> for Mysql {
    type RawValue = MysqlValue<'a>;
}
impl TypeMetadata for Mysql {
    type TypeMetadata = MysqlType;
    type MetadataLookup = ();
}
impl SqlDialect for Mysql {
    type ReturningClause = sql_dialect::returning_clause::DoesNotSupportReturningClause;
    type OnConflictClause = sql_dialect::on_conflict_clause::DoesNotSupportOnConflictClause;
    type InsertWithDefaultKeyword = sql_dialect::default_keyword_for_insert::IsoSqlDefaultKeyword;
    type BatchInsertSupport = sql_dialect::batch_insert_support::PostgresLikeBatchInsertSupport;
    type DefaultValueClauseForInsert = MysqlStyleDefaultValueClause;
    type EmptyFromClauseSyntax = sql_dialect::from_clause_syntax::AnsiSqlFromClauseSyntax;
    type ExistsSyntax = sql_dialect::exists_syntax::AnsiSqlExistsSyntax;
    type ArrayComparision = sql_dialect::array_comparision::AnsiSqlArrayComparison;
}
impl DieselReserveSpecialization for Mysql {}
impl TrustedBackend for Mysql {}
#[derive(Debug, Clone, Copy)]
pub struct MysqlStyleDefaultValueClause;
