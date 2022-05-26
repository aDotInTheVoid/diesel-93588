use std::cell::UnsafeCell;
/// A cell which can be written to only once.
///
/// Unlike `RefCell`, a `OnceCell` only provides shared `&T` references to its value.
/// Unlike `Cell`, a `OnceCell` doesn't require copying or replacing the value to access it.
///
/// # Examples
///
/// ```ignore
///
/// use crate::util::OnceCell;
///
/// let cell = OnceCell::new();
/// assert!(cell.get().is_none());
///
/// let value: &String = cell.get_or_init(|| {
///     "Hello, World!".to_string()
/// });
/// assert_eq!(value, "Hello, World!");
/// assert!(cell.get().is_some());
/// ```
pub(crate) struct OnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}
impl<T> Default for OnceCell<T> {
    fn default() -> Self {
        loop {}
    }
}
impl<T> OnceCell<T> {
    /// Creates a new empty cell.
    pub(crate) const fn new() -> OnceCell<T> {
        loop {}
    }
    /// Gets the contents of the cell, initializing it with `f` if
    /// the cell was empty. If the cell was empty and `f` failed, an
    /// error is returned.
    ///
    /// # Panics
    ///
    /// If `f` panics, the panic is propagated to the caller, and the cell
    /// remains uninitialized.
    ///
    /// It is an error to reentrantly initialize the cell from `f`. Doing
    /// so results in a panic.
    ///
    /// # Examples
    ///
    /// ```ignore
    ///
    /// use crate::util::OnceCell;
    ///
    /// let cell = OnceCell::new();
    /// assert_eq!(cell.get_or_try_init(|| Err(())), Err(()));
    /// assert!(cell.get().is_none());
    /// let value = cell.get_or_try_init(|| -> Result<i32, ()> {
    ///     Ok(92)
    /// });
    /// assert_eq!(value, Ok(&92));
    /// assert_eq!(cell.get(), Some(&92))
    /// ```
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
