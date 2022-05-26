#[cfg_attr(
    not(any(
        feature = "postgres_backend",
        feature = "mysql_backend",
        feature = "sqlite",
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )),
    allow(unused_imports)
)]
#[doc(inline)]
#[diesel_derives::__diesel_public_if(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
)]
pub(crate) use self::private::{
    DieselReserveSpecialization, HasBindCollector, HasRawValue, TrustedBackend,
};
use crate::query_builder::QueryBuilder;
use crate::sql_types::{self, HasSqlType, TypeMetadata};
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    doc = "[`HasBindCollector`]"
)]
#[cfg_attr(
    not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
    doc = "`HasBindCollector`"
)]
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    doc = "[`HasRawValue`]"
)]
#[cfg_attr(
    not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
    doc = "`HasRawValue`"
)]
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    doc = "[`DieselReserveSpecialization`]"
)]
#[cfg_attr(
    not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
    doc = "`DieselReserveSpecialization`"
)]
pub trait Backend
where
    Self: Sized + SqlDialect,
    Self: HasSqlType<sql_types::SmallInt>,
    Self: HasSqlType<sql_types::Integer>,
    Self: HasSqlType<sql_types::BigInt>,
    Self: HasSqlType<sql_types::Float>,
    Self: HasSqlType<sql_types::Double>,
    Self: HasSqlType<sql_types::Text>,
    Self: HasSqlType<sql_types::Binary>,
    Self: HasSqlType<sql_types::Date>,
    Self: HasSqlType<sql_types::Time>,
    Self: HasSqlType<sql_types::Timestamp>,
    Self: for<'a> HasRawValue<'a>,
    Self: for<'a> HasBindCollector<'a>,
{
    type QueryBuilder: QueryBuilder<Self>;
}
pub type RawValue<'a, DB> = <DB as HasRawValue<'a>>::RawValue;
pub type BindCollector<'a, DB> = <DB as HasBindCollector<'a>>::BindCollector;
pub trait SqlDialect: self::private::TrustedBackend {
    #[cfg_attr(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        doc = "implementation for [`ReturningClause`](crate::query_builder::ReturningClause)"
    )]
    #[cfg_attr(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
        doc = "implementation for `ReturningClause`"
    )]
    type ReturningClause;
    type OnConflictClause;
    type InsertWithDefaultKeyword;
    #[cfg_attr(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        doc = "implementation for [`BatchInsert`](crate::query_builder::BatchInsert)"
    )]
    #[cfg_attr(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
        doc = "implementation for `BatchInsert`"
    )]
    type BatchInsertSupport;
    #[cfg_attr(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        doc = "implementation for [`DefaultValues`](crate::query_builder::DefaultValues)"
    )]
    #[cfg_attr(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
        doc = "implementation for `DefaultValues`"
    )]
    type DefaultValueClauseForInsert;
    #[cfg_attr(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        doc = "implementation for [`NoFromClause`](crate::query_builder::NoFromClause)"
    )]
    #[cfg_attr(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
        doc = "implementation for `NoFromClause`"
    )]
    type EmptyFromClauseSyntax;
    #[cfg_attr(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        doc = "implementation for [`Exists`](crate::expression::exists::Exists)"
    )]
    #[cfg_attr(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
        doc = "implementation for `Exists`"
    )]
    type ExistsSyntax;
    #[cfg_attr(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        doc = "implementations for [`In`](crate::expression::array_comparison::In),
    [`NotIn`](crate::expression::array_comparison::NotIn) and
    [`Many`](crate::expression::array_comparison::Many)"
    )]
    #[cfg_attr(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
        doc = "implementations for `In`, `NotIn` and `Many`"
    )]
    type ArrayComparision;
}
pub mod sql_dialect {
    #[cfg(doc)]
    use super::SqlDialect;
    pub mod on_conflict_clause {
        pub trait SupportsOnConflictClause {}
        #[derive(Debug, Copy, Clone)]
        pub struct DoesNotSupportOnConflictClause;
    }
    pub mod returning_clause {
        pub trait SupportsReturningClause {}
        #[derive(Debug, Copy, Clone)]
        pub struct PgLikeReturningClause;
        #[derive(Debug, Copy, Clone)]
        pub struct DoesNotSupportReturningClause;
        impl SupportsReturningClause for PgLikeReturningClause {}
    }
    pub mod default_keyword_for_insert {
        pub trait SupportsDefaultKeyword {}
        #[derive(Debug, Copy, Clone)]
        pub struct IsoSqlDefaultKeyword;
        #[derive(Debug, Copy, Clone)]
        pub struct DoesNotSupportDefaultKeyword;
        impl SupportsDefaultKeyword for IsoSqlDefaultKeyword {}
    }
    pub mod batch_insert_support {
        pub trait SupportsBatchInsert {}
        #[derive(Debug, Copy, Clone)]
        pub struct DoesNotSupportBatchInsert;
        #[derive(Debug, Copy, Clone)]
        pub struct PostgresLikeBatchInsertSupport;
        impl SupportsBatchInsert for PostgresLikeBatchInsertSupport {}
    }
    pub mod default_value_clause {
        #[derive(Debug, Clone, Copy)]
        pub struct AnsiDefaultValueClause;
    }
    pub mod from_clause_syntax {
        #[derive(Debug, Copy, Clone)]
        pub struct AnsiSqlFromClauseSyntax;
    }
    pub mod exists_syntax {
        #[derive(Debug, Copy, Clone)]
        pub struct AnsiSqlExistsSyntax;
    }
    pub mod array_comparision {
        #[derive(Debug, Copy, Clone)]
        pub struct AnsiSqlArrayComparison;
    }
}
mod private {
    use super::TypeMetadata;
    #[cfg_attr(
        doc_cfg,
        doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
    )]
    pub trait HasRawValue<'a> {
        type RawValue;
    }
    #[cfg_attr(
        doc_cfg,
        doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
    )]
    pub trait DieselReserveSpecialization {}
    #[cfg_attr(
        doc_cfg,
        doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
    )]
    pub trait HasBindCollector<'a>: TypeMetadata + Sized {
        type BindCollector: crate::query_builder::bind_collector::BindCollector<'a, Self> + 'a;
    }
    #[cfg_attr(
        doc_cfg,
        doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
    )]
    pub trait TrustedBackend {}
}
