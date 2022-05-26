use crate::backend::Backend;
use crate::connection::Connection;
use crate::expression::count::CountStar;
use crate::expression::Expression;
use crate::helper_types::*;
use crate::query_builder::locking_clause as lock;
use crate::query_source::{joins, Table};
use crate::result::QueryResult;
mod belonging_to_dsl;
#[doc(hidden)]
pub mod boxed_dsl;
mod combine_dsl;
mod distinct_dsl;
#[doc(hidden)]
pub mod filter_dsl;
mod group_by_dsl;
mod having_dsl;
mod join_dsl;
#[doc(hidden)]
pub mod limit_dsl;
#[doc(hidden)]
pub mod load_dsl;
mod locking_dsl;
mod nullable_select_dsl;
mod offset_dsl;
pub(crate) mod order_dsl;
#[doc(hidden)]
pub mod positional_order_dsl;
mod save_changes_dsl;
#[doc(hidden)]
pub mod select_dsl;
mod single_value_dsl;
pub use self::belonging_to_dsl::BelongingToDsl;
pub use self::combine_dsl::CombineDsl;
pub use self::join_dsl::{InternalJoinDsl, JoinOnDsl, JoinWithImplicitOnClause};
#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
pub use self::load_dsl::CompatibleType;
#[doc(hidden)]
pub use self::load_dsl::{LoadQuery, LoadRet};
pub use self::save_changes_dsl::{SaveChangesDsl, UpdateAndFetchResults};
pub mod methods {
    pub use super::boxed_dsl::BoxedDsl;
    pub use super::distinct_dsl::*;
    #[doc(inline)]
    pub use super::filter_dsl::*;
    pub use super::group_by_dsl::GroupByDsl;
    pub use super::having_dsl::HavingDsl;
    pub use super::limit_dsl::LimitDsl;
    pub use super::load_dsl::{ExecuteDsl, LoadQuery, LoadRet};
    pub use super::locking_dsl::{LockingDsl, ModifyLockDsl};
    pub use super::nullable_select_dsl::SelectNullableDsl;
    pub use super::offset_dsl::OffsetDsl;
    pub use super::order_dsl::{OrderDsl, ThenOrderDsl};
    pub use super::select_dsl::SelectDsl;
    pub use super::single_value_dsl::SingleValueDsl;
}
pub trait QueryDsl: Sized {
    fn distinct(self) -> Distinct<Self>
    where
        Self: methods::DistinctDsl,
    {
        loop {}
    }
    #[cfg(feature = "postgres")]
    fn distinct_on<Expr>(self, expr: Expr) -> DistinctOn<Self, Expr>
    where
        Self: methods::DistinctOnDsl<Expr>,
    {
        loop {}
    }
    fn select<Selection>(self, selection: Selection) -> Select<Self, Selection>
    where
        Selection: Expression,
        Self: methods::SelectDsl<Selection>,
    {
        loop {}
    }
    fn count(self) -> Select<Self, CountStar>
    where
        Self: methods::SelectDsl<CountStar>,
    {
        loop {}
    }
    fn inner_join<Rhs>(self, rhs: Rhs) -> InnerJoin<Self, Rhs>
    where
        Self: JoinWithImplicitOnClause<Rhs, joins::Inner>,
    {
        loop {}
    }
    fn left_outer_join<Rhs>(self, rhs: Rhs) -> LeftJoin<Self, Rhs>
    where
        Self: JoinWithImplicitOnClause<Rhs, joins::LeftOuter>,
    {
        loop {}
    }
    fn left_join<Rhs>(self, rhs: Rhs) -> LeftJoin<Self, Rhs>
    where
        Self: JoinWithImplicitOnClause<Rhs, joins::LeftOuter>,
    {
        loop {}
    }
    #[doc(alias = "where")]
    fn filter<Predicate>(self, predicate: Predicate) -> Filter<Self, Predicate>
    where
        Self: methods::FilterDsl<Predicate>,
    {
        loop {}
    }
    #[doc(alias = "where")]
    fn or_filter<Predicate>(self, predicate: Predicate) -> OrFilter<Self, Predicate>
    where
        Self: methods::OrFilterDsl<Predicate>,
    {
        loop {}
    }
    fn find<PK>(self, id: PK) -> Find<Self, PK>
    where
        Self: methods::FindDsl<PK>,
    {
        loop {}
    }
    fn order<Expr>(self, expr: Expr) -> Order<Self, Expr>
    where
        Expr: Expression,
        Self: methods::OrderDsl<Expr>,
    {
        loop {}
    }
    fn order_by<Expr>(self, expr: Expr) -> Order<Self, Expr>
    where
        Expr: Expression,
        Self: methods::OrderDsl<Expr>,
    {
        loop {}
    }
    fn then_order_by<Order>(self, order: Order) -> ThenOrderBy<Self, Order>
    where
        Self: methods::ThenOrderDsl<Order>,
    {
        loop {}
    }
    fn limit(self, limit: i64) -> Limit<Self>
    where
        Self: methods::LimitDsl,
    {
        loop {}
    }
    fn offset(self, offset: i64) -> Offset<Self>
    where
        Self: methods::OffsetDsl,
    {
        loop {}
    }
    fn group_by<GB>(self, group_by: GB) -> GroupBy<Self, GB>
    where
        GB: Expression,
        Self: methods::GroupByDsl<GB>,
    {
        loop {}
    }
    fn having<Predicate>(self, predicate: Predicate) -> Having<Self, Predicate>
    where
        Self: methods::HavingDsl<Predicate>,
    {
        loop {}
    }
    fn for_update(self) -> ForUpdate<Self>
    where
        Self: methods::LockingDsl<lock::ForUpdate>,
    {
        loop {}
    }
    fn for_no_key_update(self) -> ForNoKeyUpdate<Self>
    where
        Self: methods::LockingDsl<lock::ForNoKeyUpdate>,
    {
        loop {}
    }
    fn for_share(self) -> ForShare<Self>
    where
        Self: methods::LockingDsl<lock::ForShare>,
    {
        loop {}
    }
    fn for_key_share(self) -> ForKeyShare<Self>
    where
        Self: methods::LockingDsl<lock::ForKeyShare>,
    {
        loop {}
    }
    fn skip_locked(self) -> SkipLocked<Self>
    where
        Self: methods::ModifyLockDsl<lock::SkipLocked>,
    {
        loop {}
    }
    fn no_wait(self) -> NoWait<Self>
    where
        Self: methods::ModifyLockDsl<lock::NoWait>,
    {
        loop {}
    }
    fn into_boxed<'a, DB>(self) -> IntoBoxed<'a, Self, DB>
    where
        DB: Backend,
        Self: methods::BoxedDsl<'a, DB>,
    {
        loop {}
    }
    fn single_value(self) -> SingleValue<Self>
    where
        Self: methods::SingleValueDsl,
    {
        loop {}
    }
    fn nullable(self) -> NullableSelect<Self>
    where
        Self: methods::SelectNullableDsl,
    {
        loop {}
    }
}
impl<T: Table> QueryDsl for T {}
pub trait RunQueryDsl<Conn>: Sized {
    fn execute(self, conn: &mut Conn) -> QueryResult<usize>
    where
        Conn: Connection,
        Self: methods::ExecuteDsl<Conn>,
    {
        loop {}
    }
    fn load<'query, U>(self, conn: &mut Conn) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<'query, Conn, U>,
    {
        loop {}
    }
    fn load_iter<'conn, 'query: 'conn, U>(
        self,
        conn: &'conn mut Conn,
    ) -> QueryResult<LoadIter<'conn, 'query, Self, Conn, U>>
    where
        U: 'conn,
        Self: LoadQuery<'query, Conn, U> + 'conn,
    {
        loop {}
    }
    fn get_result<'query, U>(self, conn: &mut Conn) -> QueryResult<U>
    where
        Self: LoadQuery<'query, Conn, U>,
    {
        loop {}
    }
    fn get_results<'query, U>(self, conn: &mut Conn) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<'query, Conn, U>,
    {
        loop {}
    }
    fn first<'query, U>(self, conn: &mut Conn) -> QueryResult<U>
    where
        Self: methods::LimitDsl,
        Limit<Self>: LoadQuery<'query, Conn, U>,
    {
        loop {}
    }
}
impl<T, Conn> RunQueryDsl<Conn> for T
where
    T: Table,
{}
