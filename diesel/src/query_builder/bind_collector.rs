//! Types related to managing bind parameters during query construction.
use crate::backend::Backend;
use crate::result::Error::SerializationError;
use crate::result::QueryResult;
use crate::serialize::{IsNull, Output, ToSql};
use crate::sql_types::{HasSqlType, TypeMetadata};
#[doc(inline)]
#[diesel_derives::__diesel_public_if(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
)]
pub(crate) use self::private::ByteWrapper;
/// A type which manages serializing bind parameters during query construction.
///
/// The only reason you would ever need to interact with this trait is if you
/// are adding support for a new backend to Diesel. Plugins which are extending
/// the query builder will use [`AstPass::push_bind_param`] instead.
///
/// [`AstPass::push_bind_param`]: crate::query_builder::AstPass::push_bind_param()
pub trait BindCollector<'a, DB: TypeMetadata>: Sized {
    /// The internal buffer type used by this bind collector
    type Buffer;
    /// Serializes the given bind value, and collects the result.
    fn push_bound_value<T, U>(
        &mut self,
        bind: &'a U,
        metadata_lookup: &mut DB::MetadataLookup,
    ) -> QueryResult<()>
    where
        DB: Backend + HasSqlType<T>,
        U: ToSql<T, DB> + 'a;
}
#[derive(Debug)]
/// A bind collector used by backends which transmit bind parameters as an
/// opaque blob of bytes.
///
/// For most backends, this is the concrete implementation of `BindCollector`
/// that should be used.
#[non_exhaustive]
pub struct RawBytesBindCollector<DB: Backend + TypeMetadata> {
    /// The metadata associated with each bind parameter.
    ///
    /// This vec is guaranteed to be the same length as `binds`.
    #[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
    pub metadata: Vec<DB::TypeMetadata>,
    #[cfg(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
    )]
    pub(crate) metadata: Vec<DB::TypeMetadata>,
    /// The serialized bytes for each bind parameter.
    ///
    /// This vec is guaranteed to be the same length as `metadata`.
    #[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
    pub binds: Vec<Option<Vec<u8>>>,
    #[cfg(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
    )]
    pub(crate) binds: Vec<Option<Vec<u8>>>,
}
#[allow(clippy::new_without_default)]
impl<DB: Backend + TypeMetadata> RawBytesBindCollector<DB> {
    /// Construct an empty `RawBytesBindCollector`
    pub fn new() -> Self {
        loop {}
    }
    pub(crate) fn reborrow_buffer<'a: 'b, 'b>(
        b: &'b mut ByteWrapper<'a>,
    ) -> ByteWrapper<'b> {
        loop {}
    }
}
impl<'a, DB> BindCollector<'a, DB> for RawBytesBindCollector<DB>
where
    DB: Backend<BindCollector = Self> + TypeMetadata,
{
    type Buffer = ByteWrapper<'a>;
    fn push_bound_value<T, U>(
        &mut self,
        bind: &U,
        metadata_lookup: &mut DB::MetadataLookup,
    ) -> QueryResult<()>
    where
        DB: HasSqlType<T>,
        U: ToSql<T, DB>,
    {
        loop {}
    }
}
mod private {
    /// A type wrapper for raw bytes
    #[derive(Debug)]
    pub struct ByteWrapper<'a>(pub(crate) &'a mut Vec<u8>);
}
