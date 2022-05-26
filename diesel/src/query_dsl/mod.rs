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
        methods::DistinctDsl::distinct(self)
    }
    #[cfg(feature = "postgres")]
    fn distinct_on<Expr>(self, expr: Expr) -> DistinctOn<Self, Expr>
    where
        Self: methods::DistinctOnDsl<Expr>,
    {
        methods::DistinctOnDsl::distinct_on(self, expr)
    }
    fn select<Selection>(self, selection: Selection) -> Select<Self, Selection>
    where
        Selection: Expression,
        Self: methods::SelectDsl<Selection>,
    {
        methods::SelectDsl::select(self, selection)
    }
    fn count(self) -> Select<Self, CountStar>
    where
        Self: methods::SelectDsl<CountStar>,
    {
        use crate::dsl::count_star;
        QueryDsl::select(self, count_star())
    }
    fn inner_join<Rhs>(self, rhs: Rhs) -> InnerJoin<Self, Rhs>
    where
        Self: JoinWithImplicitOnClause<Rhs, joins::Inner>,
    {
        self.join_with_implicit_on_clause(rhs, joins::Inner)
    }
    fn left_outer_join<Rhs>(self, rhs: Rhs) -> LeftJoin<Self, Rhs>
    where
        Self: JoinWithImplicitOnClause<Rhs, joins::LeftOuter>,
    {
        self.join_with_implicit_on_clause(rhs, joins::LeftOuter)
    }
    fn left_join<Rhs>(self, rhs: Rhs) -> LeftJoin<Self, Rhs>
    where
        Self: JoinWithImplicitOnClause<Rhs, joins::LeftOuter>,
    {
        self.left_outer_join(rhs)
    }
    #[doc(alias = "where")]
    fn filter<Predicate>(self, predicate: Predicate) -> Filter<Self, Predicate>
    where
        Self: methods::FilterDsl<Predicate>,
    {
        methods::FilterDsl::filter(self, predicate)
    }
    #[doc(alias = "where")]
    fn or_filter<Predicate>(self, predicate: Predicate) -> OrFilter<Self, Predicate>
    where
        Self: methods::OrFilterDsl<Predicate>,
    {
        methods::OrFilterDsl::or_filter(self, predicate)
    }
    fn find<PK>(self, id: PK) -> Find<Self, PK>
    where
        Self: methods::FindDsl<PK>,
    {
        methods::FindDsl::find(self, id)
    }
    fn order<Expr>(self, expr: Expr) -> Order<Self, Expr>
    where
        Expr: Expression,
        Self: methods::OrderDsl<Expr>,
    {
        methods::OrderDsl::order(self, expr)
    }
    fn order_by<Expr>(self, expr: Expr) -> Order<Self, Expr>
    where
        Expr: Expression,
        Self: methods::OrderDsl<Expr>,
    {
        QueryDsl::order(self, expr)
    }
    fn then_order_by<Order>(self, order: Order) -> ThenOrderBy<Self, Order>
    where
        Self: methods::ThenOrderDsl<Order>,
    {
        methods::ThenOrderDsl::then_order_by(self, order)
    }
    fn limit(self, limit: i64) -> Limit<Self>
    where
        Self: methods::LimitDsl,
    {
        methods::LimitDsl::limit(self, limit)
    }
    fn offset(self, offset: i64) -> Offset<Self>
    where
        Self: methods::OffsetDsl,
    {
        methods::OffsetDsl::offset(self, offset)
    }
    fn group_by<GB>(self, group_by: GB) -> GroupBy<Self, GB>
    where
        GB: Expression,
        Self: methods::GroupByDsl<GB>,
    {
        methods::GroupByDsl::group_by(self, group_by)
    }
    fn having<Predicate>(self, predicate: Predicate) -> Having<Self, Predicate>
    where
        Self: methods::HavingDsl<Predicate>,
    {
        methods::HavingDsl::having(self, predicate)
    }
    fn for_update(self) -> ForUpdate<Self>
    where
        Self: methods::LockingDsl<lock::ForUpdate>,
    {
        methods::LockingDsl::with_lock(self, lock::ForUpdate)
    }
    fn for_no_key_update(self) -> ForNoKeyUpdate<Self>
    where
        Self: methods::LockingDsl<lock::ForNoKeyUpdate>,
    {
        methods::LockingDsl::with_lock(self, lock::ForNoKeyUpdate)
    }
    fn for_share(self) -> ForShare<Self>
    where
        Self: methods::LockingDsl<lock::ForShare>,
    {
        methods::LockingDsl::with_lock(self, lock::ForShare)
    }
    fn for_key_share(self) -> ForKeyShare<Self>
    where
        Self: methods::LockingDsl<lock::ForKeyShare>,
    {
        methods::LockingDsl::with_lock(self, lock::ForKeyShare)
    }
    fn skip_locked(self) -> SkipLocked<Self>
    where
        Self: methods::ModifyLockDsl<lock::SkipLocked>,
    {
        methods::ModifyLockDsl::modify_lock(self, lock::SkipLocked)
    }
    fn no_wait(self) -> NoWait<Self>
    where
        Self: methods::ModifyLockDsl<lock::NoWait>,
    {
        methods::ModifyLockDsl::modify_lock(self, lock::NoWait)
    }
    fn into_boxed<'a, DB>(self) -> IntoBoxed<'a, Self, DB>
    where
        DB: Backend,
        Self: methods::BoxedDsl<'a, DB>,
    {
        methods::BoxedDsl::internal_into_boxed(self)
    }
    fn single_value(self) -> SingleValue<Self>
    where
        Self: methods::SingleValueDsl,
    {
        methods::SingleValueDsl::single_value(self)
    }
    fn nullable(self) -> NullableSelect<Self>
    where
        Self: methods::SelectNullableDsl,
    {
        methods::SelectNullableDsl::nullable(self)
    }
}
impl<T: Table> QueryDsl for T {}
pub trait RunQueryDsl<Conn>: Sized {
    fn execute(self, conn: &mut Conn) -> QueryResult<usize>
    where
        Conn: Connection,
        Self: methods::ExecuteDsl<Conn>,
    {
        methods::ExecuteDsl::execute(self, conn)
    }
    fn load<'query, U>(self, conn: &mut Conn) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<'query, Conn, U>,
    {
        self.internal_load(conn)?.collect()
    }
    fn load_iter<'conn, 'query: 'conn, U>(
        self,
        conn: &'conn mut Conn,
    ) -> QueryResult<LoadIter<'conn, 'query, Self, Conn, U>>
    where
        U: 'conn,
        Self: LoadQuery<'query, Conn, U> + 'conn,
    {
        self.internal_load(conn)
    }
    fn get_result<'query, U>(self, conn: &mut Conn) -> QueryResult<U>
    where
        Self: LoadQuery<'query, Conn, U>,
    {
        match self.internal_load(conn)?.next() {
            Some(v) => v,
            None => Err(crate::result::Error::NotFound),
        }
    }
    fn get_results<'query, U>(self, conn: &mut Conn) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<'query, Conn, U>,
    {
        self.load(conn)
    }
    fn first<'query, U>(self, conn: &mut Conn) -> QueryResult<U>
    where
        Self: methods::LimitDsl,
        Limit<Self>: LoadQuery<'query, Conn, U>,
    {
        methods::LimitDsl::limit(self, 1).get_result(conn)
    }
}
impl<T, Conn> RunQueryDsl<Conn> for T where T: Table {}
