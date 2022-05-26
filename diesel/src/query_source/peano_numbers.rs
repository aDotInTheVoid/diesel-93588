#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Never;
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Once;
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct MoreThanOnce;
pub trait Plus<T> {
    type Output;
}
impl<T> Plus<T> for Never {
    type Output = T;
}
impl<T> Plus<T> for MoreThanOnce {
    type Output = Self;
}
impl Plus<Never> for Once {
    type Output = Self;
}
impl Plus<Once> for Once {
    type Output = MoreThanOnce;
}
impl Plus<MoreThanOnce> for Once {
    type Output = MoreThanOnce;
}
