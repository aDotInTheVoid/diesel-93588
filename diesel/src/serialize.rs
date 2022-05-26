use crate::backend::{Backend, HasBindCollector};
#[doc(inline)]
#[cfg(feature = "postgres_backend")]
pub use crate::pg::serialize::WriteTuple;
use crate::query_builder::bind_collector::RawBytesBindCollector;
use crate::query_builder::BindCollector;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::result;
pub type Result = result::Result<IsNull, Box<dyn Error + Send + Sync>>;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IsNull {
    Yes,
    No,
}
pub struct Output<'a, 'b, DB>
where
    DB: Backend,
    DB::MetadataLookup: 'a,
{
    out: <crate::backend::BindCollector<'a, DB> as BindCollector<'a, DB>>::Buffer,
    metadata_lookup: Option<&'b mut DB::MetadataLookup>,
}
impl<'a, 'b, DB: Backend> Output<'a, 'b, DB> {
    pub fn new(
        out: <crate::backend::BindCollector<'a, DB> as BindCollector<'a, DB>>::Buffer,
        metadata_lookup: &'b mut DB::MetadataLookup,
    ) -> Self {
        loop {}
    }
    pub fn into_inner(
        self,
    ) -> <crate::backend::BindCollector<'a, DB> as BindCollector<'a, DB>>::Buffer {
        loop {}
    }
    pub fn metadata_lookup(&mut self) -> &mut DB::MetadataLookup {
        loop {}
    }
    pub fn set_value<V>(&mut self, value: V)
    where
        V: Into<<crate::backend::BindCollector<'a, DB> as BindCollector<'a, DB>>::Buffer>,
    {
        loop {}
    }
}
#[cfg(test)]
impl<'a, DB: Backend> Output<'a, 'static, DB> {
    pub fn test(
        buffer: <crate::backend::BindCollector<'a, DB> as BindCollector<'a, DB>>::Buffer,
    ) -> Self {
        loop {}
    }
}
impl<'a, 'b, DB: Backend<BindCollector = RawBytesBindCollector<DB>>> Write for Output<'a, 'b, DB> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        loop {}
    }
    fn flush(&mut self) -> io::Result<()> {
        loop {}
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        loop {}
    }
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        loop {}
    }
}
impl<'a, 'b, DB: Backend<BindCollector = RawBytesBindCollector<DB>>> Output<'a, 'b, DB> {
    pub fn reborrow<'c>(&'c mut self) -> Output<'c, 'c, DB>
    where
        'a: 'c,
    {
        loop {}
    }
}
impl<'a, 'b, DB> fmt::Debug for Output<'a, 'b, DB>
where
    <<DB as HasBindCollector<'a>>::BindCollector as BindCollector<'a, DB>>::Buffer: fmt::Debug,
    DB: Backend,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
pub trait ToSql<A, DB: Backend>: fmt::Debug {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> Result;
}
impl<'a, A, T, DB> ToSql<A, DB> for &'a T
where
    DB: Backend,
    T: ToSql<A, DB> + ?Sized,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> Result {
        loop {}
    }
}
