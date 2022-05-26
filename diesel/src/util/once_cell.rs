use std::cell::UnsafeCell;
pub(crate) struct OnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}
impl<T> Default for OnceCell<T> {
    fn default() -> Self {
        loop {}
    }
}
impl<T> OnceCell<T> {
    pub(crate) const fn new() -> OnceCell<T> {
        loop {}
    }
    pub(crate) fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        loop {}
    }
    pub(crate) fn get(&self) -> Option<&T> {
        loop {}
    }
    pub(crate) fn set(&self, value: T) -> Result<(), T> {
        loop {}
    }
}
