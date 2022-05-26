use crate::associations::HasTable;
#[cfg(any(feature = "sqlite", feature = "mysql"))]
use crate::associations::Identifiable;
use crate::connection::Connection;
#[cfg(any(feature = "sqlite", feature = "mysql"))]
use crate::dsl::Find;
#[cfg(any(feature = "sqlite", feature = "postgres", feature = "mysql"))]
use crate::dsl::Update;
#[cfg(any(feature = "sqlite", feature = "postgres", feature = "mysql"))]
use crate::expression::{is_aggregate, MixedAggregates, ValidGrouping};
use crate::query_builder::{AsChangeset, IntoUpdateTarget};
#[cfg(any(feature = "sqlite", feature = "mysql"))]
use crate::query_dsl::methods::{ExecuteDsl, FindDsl};
#[cfg(any(feature = "sqlite", feature = "postgres", feature = "mysql"))]
use crate::query_dsl::{LoadQuery, RunQueryDsl};
use crate::result::QueryResult;
#[cfg(any(feature = "sqlite", feature = "postgres", feature = "mysql"))]
use crate::Table;
pub trait UpdateAndFetchResults<Changes, Output>: Connection {
    fn update_and_fetch(&mut self, changeset: Changes) -> QueryResult<Output>;
}
#[cfg(feature = "postgres")]
use crate::pg::PgConnection;
#[cfg(feature = "postgres")]
impl<'b, Changes, Output> UpdateAndFetchResults<Changes, Output> for PgConnection
where
    Changes: Copy + AsChangeset<Target = <Changes as HasTable>::Table> + IntoUpdateTarget,
    Update<Changes, Changes>: LoadQuery<'b, PgConnection, Output>,
    <Changes::Table as Table>::AllColumns: ValidGrouping<()>,
    <<Changes::Table as Table>::AllColumns as ValidGrouping<()>>::IsAggregate:
        MixedAggregates<is_aggregate::No, Output = is_aggregate::No>,
{
    fn update_and_fetch(&mut self, changeset: Changes) -> QueryResult<Output> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
use crate::sqlite::SqliteConnection;
#[cfg(feature = "sqlite")]
impl<'b, Changes, Output> UpdateAndFetchResults<Changes, Output> for SqliteConnection
where
    Changes: Copy + Identifiable,
    Changes: AsChangeset<Target = <Changes as HasTable>::Table> + IntoUpdateTarget,
    Changes::Table: FindDsl<Changes::Id>,
    Update<Changes, Changes>: ExecuteDsl<SqliteConnection>,
    Find<Changes::Table, Changes::Id>: LoadQuery<'b, SqliteConnection, Output>,
    <Changes::Table as Table>::AllColumns: ValidGrouping<()>,
    <<Changes::Table as Table>::AllColumns as ValidGrouping<()>>::IsAggregate:
        MixedAggregates<is_aggregate::No, Output = is_aggregate::No>,
{
    fn update_and_fetch(&mut self, changeset: Changes) -> QueryResult<Output> {
        loop {}
    }
}
#[cfg(feature = "mysql")]
use crate::mysql::MysqlConnection;
#[cfg(feature = "mysql")]
impl<'b, Changes, Output> UpdateAndFetchResults<Changes, Output> for MysqlConnection
where
    Changes: Copy + Identifiable,
    Changes: AsChangeset<Target = <Changes as HasTable>::Table> + IntoUpdateTarget,
    Changes::Table: FindDsl<Changes::Id>,
    Update<Changes, Changes>: ExecuteDsl<MysqlConnection>,
    Find<Changes::Table, Changes::Id>: LoadQuery<'b, MysqlConnection, Output>,
    <Changes::Table as Table>::AllColumns: ValidGrouping<()>,
    <<Changes::Table as Table>::AllColumns as ValidGrouping<()>>::IsAggregate:
        MixedAggregates<is_aggregate::No, Output = is_aggregate::No>,
{
    fn update_and_fetch(&mut self, changeset: Changes) -> QueryResult<Output> {
        loop {}
    }
}
pub trait SaveChangesDsl<Conn> {
    fn save_changes<T>(self, connection: &mut Conn) -> QueryResult<T>
    where
        Self: Sized,
        Conn: UpdateAndFetchResults<Self, T>,
    {
        connection.update_and_fetch(self)
    }
}
impl<T, Conn> SaveChangesDsl<Conn> for T where
    T: Copy + AsChangeset<Target = <T as HasTable>::Table> + IntoUpdateTarget
{
}
