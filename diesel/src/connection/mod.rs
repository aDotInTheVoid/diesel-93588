#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
pub mod commit_error_processor;
#[cfg(not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))]
pub(crate) mod commit_error_processor;
#[cfg(
    all(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
        any(feature = "sqlite", feature = "postgres", feature = "mysql")
    )
)]
pub(crate) mod statement_cache;
#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
pub mod statement_cache;
mod transaction_manager;
#[diesel_derives::__diesel_public_if(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
)]
pub(crate) use self::private::ConnectionGatWorkaround;
pub use self::transaction_manager::{
    AnsiTransactionManager, TransactionDepthChange, TransactionManager,
    TransactionManagerStatus, ValidTransactionManagerStatus,
};
use crate::backend::Backend;
use crate::expression::QueryMetadata;
use crate::query_builder::{Query, QueryFragment, QueryId};
use crate::result::*;
use std::fmt::Debug;
pub trait SimpleConnection {
    fn batch_execute(&mut self, query: &str) -> QueryResult<()>;
}
pub type LoadRowIter<'conn, 'query, C, DB> = <C as ConnectionGatWorkaround<
    'conn,
    'query,
    DB,
>>::Cursor;
#[cfg_attr(
    feature = "r2d2",
    doc = "it may be useful to also implement [`R2D2Connection`](crate::r2d2::R2D2Connection)"
)]
#[cfg_attr(
    not(feature = "r2d2"),
    doc = "it may be useful to also implement `R2D2Connection`"
)]
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    doc = "See [`StatementCache`](self::statement_cache::StatementCache)"
)]
#[cfg_attr(
    not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
    doc = "See `StatementCache`"
)]
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    doc = "The [statement_cache](self::statement_cache)"
)]
#[cfg_attr(
    not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
    doc = "The statement_cache"
)]
#[cfg_attr(
    feature = "r2d2",
    doc = "it may be useful to also implement [`R2D2Connection`](crate::r2d2::R2D2Connection)"
)]
#[cfg_attr(
    not(feature = "r2d2"),
    doc = "it may be useful to also implement `R2D2Connection`"
)]
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    doc = "likely contain a [`StatementCache`](self::statement_cache::StatementCache)"
)]
#[cfg_attr(
    not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
    doc = "likely contain a `StatementCache`"
)]
pub trait Connection: SimpleConnection + Sized + Send
where
    Self: for<'a, 'b> ConnectionGatWorkaround<'a, 'b, <Self as Connection>::Backend>,
{
    type Backend: Backend;
    #[diesel_derives::__diesel_public_if(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )]
    type TransactionManager: TransactionManager<Self>;
    fn establish(database_url: &str) -> ConnectionResult<Self>;
    fn transaction<T, E, F>(&mut self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
        E: From<Error>,
    {
        Self::TransactionManager::transaction(self, f)
    }
    fn begin_test_transaction(&mut self) -> QueryResult<()> {
        match Self::TransactionManager::transaction_manager_status_mut(self) {
            TransactionManagerStatus::Valid(valid_status) => {
                assert_eq!(None, valid_status.transaction_depth())
            }
            TransactionManagerStatus::InError => panic!("Transaction manager in error"),
        };
        Self::TransactionManager::begin_transaction(self)
    }
    fn test_transaction<T, E, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
        E: Debug,
    {
        let mut user_result = None;
        let _ = self
            .transaction::<
            (),
            _,
            _,
        >(|conn| {
                user_result = f(conn).ok();
                Err(Error::RollbackTransaction)
            });
        user_result.expect("Transaction did not succeed")
    }
    #[diesel_derives::__diesel_public_if(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )]
    fn load<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> QueryResult<LoadRowIter<'conn, 'query, Self, Self::Backend>>
    where
        T: Query + QueryFragment<Self::Backend> + QueryId + 'query,
        Self::Backend: QueryMetadata<T::SqlType>;
    #[diesel_derives::__diesel_public_if(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )]
    fn execute_returning_count<T>(&mut self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Self::Backend> + QueryId;
    #[diesel_derives::__diesel_public_if(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )]
    fn transaction_state(
        &mut self,
    ) -> &mut <Self::TransactionManager as TransactionManager<
        Self,
    >>::TransactionStateData;
}
pub trait BoxableConnection<DB: Backend>: SimpleConnection + std::any::Any {
    #[diesel_derives::__diesel_public_if(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )]
    fn as_any(&self) -> &dyn std::any::Any;
    #[doc(hidden)]
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
impl<C> BoxableConnection<C::Backend> for C
where
    C: Connection + std::any::Any,
{
    fn as_any(&self) -> &dyn std::any::Any {
        loop {}
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        loop {}
    }
}
impl<DB: Backend + 'static> dyn BoxableConnection<DB> {
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Connection<Backend = DB> + 'static,
    {
        loop {}
    }
    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Connection<Backend = DB> + 'static,
    {
        loop {}
    }
    pub fn is<T>(&self) -> bool
    where
        T: Connection<Backend = DB> + 'static,
    {
        loop {}
    }
}
mod private {
    use crate::backend::Backend;
    use crate::QueryResult;
    #[cfg_attr(
        doc_cfg,
        doc(
            cfg(
                feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
            )
        )
    )]
    pub trait ConnectionGatWorkaround<'conn, 'query, DB: Backend> {
        type Cursor: Iterator<Item = QueryResult<Self::Row>>;
        type Row: crate::row::Row<'conn, DB>;
    }
}
