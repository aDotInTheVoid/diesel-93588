use std::marker::PhantomData;
use std::rc::Rc;
use super::result::PgResult;
use super::row::PgRow;
use super::PgConnection;
/// The type returned by various [`Connection`] methods.
/// Acts as an iterator over `T`.
#[allow(missing_debug_implementations)]
pub struct Cursor<'a> {
    current_row: usize,
    db_result: Rc<PgResult>,
    p: PhantomData<&'a mut PgConnection>,
}
impl Cursor<'_> {
    pub(super) fn new(db_result: PgResult) -> Self {
        loop {}
    }
}
impl ExactSizeIterator for Cursor<'_> {
    fn len(&self) -> usize {
        loop {}
    }
}
impl Iterator for Cursor<'_> {
    type Item = crate::QueryResult<PgRow>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {}
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        loop {}
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        loop {}
    }
    fn count(self) -> usize
    where
        Self: Sized,
    {
        loop {}
    }
}
#[test]
fn fun_with_row_iters() {
    loop {}
}
