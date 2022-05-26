use std::num::NonZeroU32;
use std::ops::Range;
#[derive(Clone, Copy)]
#[allow(missing_debug_implementations)]
#[cfg(feature = "postgres_backend")]
pub struct PgValue<'a> {
    raw_value: &'a [u8],
    type_oid_lookup: &'a dyn TypeOidLookup,
}
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
#[allow(unreachable_pub)]
pub trait TypeOidLookup {
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
    pub fn as_bytes(&self) -> &[u8] {
        loop {}
    }
    pub fn get_oid(&self) -> NonZeroU32 {
        loop {}
    }
    pub(crate) fn subslice(&self, range: Range<usize>) -> Self {
        loop {}
    }
}
