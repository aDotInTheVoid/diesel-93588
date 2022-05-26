use std::num::NonZeroU32;
use std::ops::Range;
/// Raw postgres value as received from the database
#[derive(Clone, Copy)]
#[allow(missing_debug_implementations)]
#[cfg(feature = "postgres_backend")]
pub struct PgValue<'a> {
    raw_value: &'a [u8],
    type_oid_lookup: &'a dyn TypeOidLookup,
}
/// This is a helper trait to defer a type oid
/// lookup to a later point in time
///
/// This is mainly used in the `PgConnection`
/// implementation so that we do not need to call
/// into libpq if we do not need the type oid.
///
/// Backend implementations based on pure rustc
/// database connection crates can likely reuse
/// the implementation for `NonZeroU32` here instead
/// of providing their own custom implementation
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
#[allow(unreachable_pub)]
pub trait TypeOidLookup {
    /// Lookup the type oid for the current value
    fn lookup(&self) -> NonZeroU32;
}
impl<F> TypeOidLookup for F
where
    F: Fn() -> NonZeroU32,
{
    fn lookup(&self) -> NonZeroU32 {
        loop {}
    }
}
impl<'a> TypeOidLookup for PgValue<'a> {
    fn lookup(&self) -> NonZeroU32 {
        loop {}
    }
}
impl TypeOidLookup for NonZeroU32 {
    fn lookup(&self) -> NonZeroU32 {
        loop {}
    }
}
impl<'a> PgValue<'a> {
    #[cfg(test)]
    pub(crate) fn for_test(raw_value: &'a [u8]) -> Self {
        loop {}
    }
    /// Create a new instance of `PgValue` based on a byte buffer
    /// and a way to receive information about the type of the value
    /// represented by the buffer
    #[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
    pub fn new(raw_value: &'a [u8], type_oid_lookup: &'a dyn TypeOidLookup) -> Self {
        loop {}
    }
    pub(in crate::pg) fn new_internal(
        raw_value: &'a [u8],
        type_oid_lookup: &'a dyn TypeOidLookup,
    ) -> Self {
        loop {}
    }
    /// Get the underlying raw byte representation
    pub fn as_bytes(&self) -> &[u8] {
        loop {}
    }
    /// Get the type oid of this value
    pub fn get_oid(&self) -> NonZeroU32 {
        loop {}
    }
    pub(crate) fn subslice(&self, range: Range<usize>) -> Self {
        loop {}
    }
}
