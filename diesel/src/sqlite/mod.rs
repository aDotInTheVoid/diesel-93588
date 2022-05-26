pub(crate) mod backend;
mod connection;
pub(crate) mod expression;
pub mod query_builder;
mod types;
pub use self::backend::{Sqlite, SqliteType};
pub use self::connection::SqliteBindValue;
pub use self::connection::SqliteConnection;
pub use self::connection::SqliteValue;
pub use self::query_builder::SqliteQueryBuilder;
pub trait SqliteAggregateFunction<Args>: Default {
        type Output;
                        fn step(&mut self, args: Args);
                                fn finalize(aggregator: Option<Self>) -> Self::Output;
}
pub mod sql_types {
    #[doc(inline)]
    pub use super::types::Timestamptz;
}
