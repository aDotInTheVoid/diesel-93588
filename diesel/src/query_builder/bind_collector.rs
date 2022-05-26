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
pub trait BindCollector<'a, DB: TypeMetadata>: Sized {
        type Buffer;
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
#[non_exhaustive]
pub struct RawBytesBindCollector<DB: Backend + TypeMetadata> {
                #[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
    pub metadata: Vec<DB::TypeMetadata>,
    #[cfg(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
    )]
    pub(crate) metadata: Vec<DB::TypeMetadata>,
                #[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
    pub binds: Vec<Option<Vec<u8>>>,
    #[cfg(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
    )]
    pub(crate) binds: Vec<Option<Vec<u8>>>,
}
#[allow(clippy::new_without_default)]
impl<DB: Backend + TypeMetadata> RawBytesBindCollector<DB> {
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
        #[derive(Debug)]
    pub struct ByteWrapper<'a>(pub(crate) &'a mut Vec<u8>);
}
