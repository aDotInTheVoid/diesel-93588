extern crate diesel_derives;
#[macro_use]
pub mod macros;
pub mod internal;
#[cfg(test)]
#[macro_use]
extern crate cfg_if;
pub mod associations;
pub mod backend;
pub mod connection;
pub mod data_types;
pub mod deserialize;
#[macro_use]
pub mod expression;
pub mod expression_methods;
pub mod insertable;
pub mod query_builder;
pub mod query_dsl;
pub mod query_source;
pub mod result;
pub mod serialize;
#[macro_use]
pub mod sql_types;
pub mod row;

mod type_impls;
mod util;

pub mod dsl {
    #[doc(inline)]
    pub use crate::expression::dsl::*;
    #[doc(inline)]
    pub use crate::helper_types::*;
    #[doc(inline)]
    pub use crate::query_builder::functions::{
        delete, insert_into, insert_or_ignore_into, replace_into, select, sql_query, update,
    };
}
pub mod helper_types {
    use super::query_builder::combination_clause::{self, CombinationClause};
    use super::query_builder::{locking_clause as lock, AsQuery};
    use super::query_dsl::methods::*;
    use super::query_dsl::*;
    use super::query_source::{aliasing, joins};
    #[doc(inline)]
    pub use crate::expression::helper_types::*;
    use crate::query_builder::select_clause::SelectClause;
    pub type Select<Source, Selection> = <Source as SelectDsl<Selection>>::Output;
    pub type BareSelect<Selection> = crate::query_builder::SelectStatement<
        crate::query_builder::NoFromClause,
        SelectClause<Selection>,
    >;
    pub type Filter<Source, Predicate> = <Source as FilterDsl<Predicate>>::Output;
    pub type FindBy<Source, Column, Value> = Filter<Source, Eq<Column, Value>>;
    pub type ForUpdate<Source> = <Source as LockingDsl<lock::ForUpdate>>::Output;
    pub type ForNoKeyUpdate<Source> = <Source as LockingDsl<lock::ForNoKeyUpdate>>::Output;
    pub type ForShare<Source> = <Source as LockingDsl<lock::ForShare>>::Output;
    pub type ForKeyShare<Source> = <Source as LockingDsl<lock::ForKeyShare>>::Output;
    pub type SkipLocked<Source> = <Source as ModifyLockDsl<lock::SkipLocked>>::Output;
    pub type NoWait<Source> = <Source as ModifyLockDsl<lock::NoWait>>::Output;
    pub type Find<Source, PK> = <Source as FindDsl<PK>>::Output;
    pub type OrFilter<Source, Predicate> = <Source as OrFilterDsl<Predicate>>::Output;
    pub type Order<Source, Ordering> = <Source as OrderDsl<Ordering>>::Output;
    pub type ThenOrderBy<Source, Ordering> = <Source as ThenOrderDsl<Ordering>>::Output;
    pub type Limit<Source> = <Source as LimitDsl>::Output;
    pub type Offset<Source> = <Source as OffsetDsl>::Output;
    pub type InnerJoin<Source, Rhs> =
        <Source as JoinWithImplicitOnClause<Rhs, joins::Inner>>::Output;
    pub type InnerJoinOn<Source, Rhs, On> =
        <Source as InternalJoinDsl<Rhs, joins::Inner, On>>::Output;
    pub type LeftJoin<Source, Rhs> =
        <Source as JoinWithImplicitOnClause<Rhs, joins::LeftOuter>>::Output;
    pub type LeftJoinOn<Source, Rhs, On> =
        <Source as InternalJoinDsl<Rhs, joins::LeftOuter, On>>::Output;
    pub type On<Source, On> = joins::OnClauseWrapper<Source, On>;
    use super::associations::HasTable;
    use super::query_builder::{AsChangeset, IntoUpdateTarget, UpdateStatement};
    pub type Update<Target, Changes> = UpdateStatement<
        <Target as HasTable>::Table,
        <Target as IntoUpdateTarget>::WhereClause,
        <Changes as AsChangeset>::Changeset,
    >;
    pub type IntoBoxed<'a, Source, DB> = <Source as BoxedDsl<'a, DB>>::Output;
    pub type Distinct<Source> = <Source as DistinctDsl>::Output;
    #[cfg(feature = "postgres_backend")]
    pub type DistinctOn<Source, Expr> = <Source as DistinctOnDsl<Expr>>::Output;
    pub type SingleValue<Source> = <Source as SingleValueDsl>::Output;
    pub type NullableSelect<Source> = <Source as SelectNullableDsl>::Output;
    pub type GroupBy<Source, Expr> = <Source as GroupByDsl<Expr>>::Output;
    pub type Having<Source, Predicate> = <Source as HavingDsl<Predicate>>::Output;
    pub type Union<Source, Rhs> = CombinationClause<
        combination_clause::Union,
        combination_clause::Distinct,
        <Source as CombineDsl>::Query,
        <Rhs as AsQuery>::Query,
    >;
    pub type UnionAll<Source, Rhs> = CombinationClause<
        combination_clause::Union,
        combination_clause::All,
        <Source as CombineDsl>::Query,
        <Rhs as AsQuery>::Query,
    >;
    pub type Intersect<Source, Rhs> = CombinationClause<
        combination_clause::Intersect,
        combination_clause::Distinct,
        <Source as CombineDsl>::Query,
        <Rhs as AsQuery>::Query,
    >;
    pub type IntersectAll<Source, Rhs> = CombinationClause<
        combination_clause::Intersect,
        combination_clause::All,
        <Source as CombineDsl>::Query,
        <Rhs as AsQuery>::Query,
    >;
    pub type Except<Source, Rhs> = CombinationClause<
        combination_clause::Except,
        combination_clause::Distinct,
        <Source as CombineDsl>::Query,
        <Rhs as AsQuery>::Query,
    >;
    pub type ExceptAll<Source, Rhs> = CombinationClause<
        combination_clause::Except,
        combination_clause::All,
        <Source as CombineDsl>::Query,
        <Rhs as AsQuery>::Query,
    >;
    type JoinQuerySource<Left, Right, Kind, On> = joins::JoinOn<joins::Join<Left, Right, Kind>, On>;
    pub type InnerJoinQuerySource<Left, Right, On = <Left as joins::JoinTo<Right>>::OnClause> =
        JoinQuerySource<Left, Right, joins::Inner, On>;
    pub type LeftJoinQuerySource<Left, Right, On = <Left as joins::JoinTo<Right>>::OnClause> =
        JoinQuerySource<Left, Right, joins::LeftOuter, On>;
    pub type LoadIter<'conn, 'query, Q, Conn, U> =
        <Q as load_dsl::LoadQueryGatWorkaround<'conn, 'query, Conn, U>>::Ret;
    pub type AliasedFields<S, F> = <F as aliasing::FieldAliasMapper<S>>::Out;
}
pub mod prelude {
    #[doc(inline)]
    pub use crate::associations::{Associations, GroupedBy, Identifiable};
    #[doc(inline)]
    pub use crate::connection::Connection;
    #[doc(inline)]
    pub use crate::deserialize::{Queryable, QueryableByName};
    #[doc(inline)]
    pub use crate::expression::functions::sql_function;
    #[doc(inline)]
    pub use crate::expression::SelectableHelper;
    #[doc(inline)]
    pub use crate::expression::{
        AppearsOnTable, BoxableExpression, Expression, IntoSql, Selectable, SelectableExpression,
    };
    #[doc(inline)]
    pub use crate::expression_methods::*;
    #[doc(inline)]
    pub use crate::insertable::Insertable;
    #[doc(inline)]
    pub use crate::macros::prelude::*;
    #[cfg(feature = "mysql")]
    #[doc(inline)]
    pub use crate::mysql::MysqlConnection;
    #[cfg(feature = "postgres")]
    #[doc(inline)]
    pub use crate::pg::PgConnection;
    #[doc(inline)]
    pub use crate::query_builder::AsChangeset;
    #[doc(inline)]
    pub use crate::query_builder::DecoratableTarget;
    #[doc(inline)]
    pub use crate::query_dsl::{
        BelongingToDsl, CombineDsl, JoinOnDsl, QueryDsl, RunQueryDsl, SaveChangesDsl,
    };
    #[doc(inline)]
    pub use crate::query_source::{Column, JoinTo, QuerySource, Table};
    #[doc(inline)]
    pub use crate::result::{ConnectionError, ConnectionResult, OptionalExtension, QueryResult};
    #[cfg(feature = "sqlite")]
    #[doc(inline)]
    pub use crate::sqlite::SqliteConnection;
}
pub use crate::prelude::*;
#[doc(inline)]
pub use crate::query_builder::debug_query;
#[doc(inline)]
pub use crate::query_builder::functions::{
    delete, insert_into, insert_or_ignore_into, replace_into, select, sql_query, update,
};
pub use crate::result::Error::NotFound;
pub(crate) mod diesel {
    pub(crate) use super::*;
}
#[doc(hidden)]
pub use __diesel_check_column_count_internal as __diesel_check_column_count;
