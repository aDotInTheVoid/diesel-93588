use crate::backend::Backend;
use crate::connection::{BoxableConnection, Connection};
use crate::deserialize::{FromSql, FromSqlRow};
use crate::expression::AsExpression;
use crate::result::QueryResult;
use crate::serialize::ToSql;
use crate::sql_types::Text;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;
pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub struct MigrationVersion<'a>(Cow<'a, str>);
impl<'a> MigrationVersion<'a> {
            pub fn as_owned(&self) -> MigrationVersion<'static> {
        loop {}
    }
}
impl<'a, DB> FromSql<Text, DB> for MigrationVersion<'a>
where
    String: FromSql<Text, DB>,
    DB: Backend,
{
    fn from_sql(
        bytes: crate::backend::RawValue<'_, DB>,
    ) -> crate::deserialize::Result<Self> {
        loop {}
    }
}
impl<'a, DB> ToSql<Text, DB> for MigrationVersion<'a>
where
    Cow<'a, str>: ToSql<Text, DB>,
    DB: Backend,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut crate::serialize::Output<'b, '_, DB>,
    ) -> crate::serialize::Result {
        loop {}
    }
}
impl<'a> From<String> for MigrationVersion<'a> {
    fn from(s: String) -> Self {
        loop {}
    }
}
impl<'a> From<&'a str> for MigrationVersion<'a> {
    fn from(s: &'a str) -> Self {
        loop {}
    }
}
impl<'a> From<&'a String> for MigrationVersion<'a> {
    fn from(s: &'a String) -> Self {
        loop {}
    }
}
impl<'a> Display for MigrationVersion<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
pub trait MigrationName: Display {
        fn version(&self) -> MigrationVersion<'_>;
}
pub trait Migration<DB: Backend> {
        fn run(&self, conn: &mut dyn BoxableConnection<DB>) -> Result<()>;
        fn revert(&self, conn: &mut dyn BoxableConnection<DB>) -> Result<()>;
        fn metadata(&self) -> &dyn MigrationMetadata;
                            fn name(&self) -> &dyn MigrationName;
}
pub trait MigrationMetadata {
                                fn run_in_transaction(&self) -> bool {
        true
    }
}
pub trait MigrationSource<DB: Backend> {
            fn migrations(&self) -> Result<Vec<Box<dyn Migration<DB>>>>;
}
impl<'a, DB: Backend> Migration<DB> for Box<dyn Migration<DB> + 'a> {
    fn run(&self, conn: &mut dyn BoxableConnection<DB>) -> Result<()> {
        loop {}
    }
    fn revert(&self, conn: &mut dyn BoxableConnection<DB>) -> Result<()> {
        loop {}
    }
    fn metadata(&self) -> &dyn MigrationMetadata {
        loop {}
    }
    fn name(&self) -> &dyn MigrationName {
        loop {}
    }
}
impl<'a, DB: Backend> Migration<DB> for &'a dyn Migration<DB> {
    fn run(&self, conn: &mut dyn BoxableConnection<DB>) -> Result<()> {
        loop {}
    }
    fn revert(&self, conn: &mut dyn BoxableConnection<DB>) -> Result<()> {
        loop {}
    }
    fn metadata(&self) -> &dyn MigrationMetadata {
        loop {}
    }
    fn name(&self) -> &dyn MigrationName {
        loop {}
    }
}
pub const CREATE_MIGRATIONS_TABLE: &str = include_str!("setup_migration_table.sql");
pub trait MigrationConnection: Connection {
                                                fn setup(&mut self) -> QueryResult<usize>;
}
#[cfg(feature = "postgres")]
impl MigrationConnection for crate::pg::PgConnection {
    fn setup(&mut self) -> QueryResult<usize> {
        loop {}
    }
}
#[cfg(feature = "mysql")]
impl MigrationConnection for crate::mysql::MysqlConnection {
    fn setup(&mut self) -> QueryResult<usize> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl MigrationConnection for crate::sqlite::SqliteConnection {
    fn setup(&mut self) -> QueryResult<usize> {
        loop {}
    }
}
