pub(crate) mod backend;
#[cfg(feature = "mysql")]
mod connection;
mod value;
mod query_builder;
mod types;
pub use self::backend::{Mysql, MysqlType};
#[cfg(feature = "mysql")]
pub use self::connection::MysqlConnection;
pub use self::query_builder::MysqlQueryBuilder;
pub use self::value::{MysqlValue, NumericRepresentation};
pub mod data_types {
    #[doc(inline)]
    pub use super::types::date_and_time::{MysqlTime, MysqlTimestampType};
}
pub mod sql_types {
    #[doc(inline)]
    pub use super::types::{Datetime, Unsigned};
}
