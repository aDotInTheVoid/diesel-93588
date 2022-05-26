use crate::backend::{Backend, DieselReserveSpecialization};
use crate::query_builder::*;
use crate::result::QueryResult;
use std::marker::PhantomData;
#[doc(hidden)]
pub trait StaticQueryFragment {
    type Component: 'static;
    const STATIC_COMPONENT: &'static Self::Component;
}
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct StaticQueryFragmentInstance<T>(PhantomData<T>);
impl<T> StaticQueryFragmentInstance<T> {
    #[doc(hidden)]
    pub const fn new() -> Self {
        loop {}
    }
}
impl<T, DB> QueryFragment<DB> for StaticQueryFragmentInstance<T>
where
    DB: Backend + DieselReserveSpecialization,
    T: StaticQueryFragment,
    T::Component: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct Identifier<'a>(pub &'a str);
impl<'a, DB: Backend> QueryFragment<DB> for Identifier<'a> {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
pub trait MiddleFragment<DB: Backend> {
    fn push_sql(&self, pass: AstPass<'_, '_, DB>);
}
impl<'a, DB: Backend> MiddleFragment<DB> for &'a str {
    fn push_sql(&self, mut pass: AstPass<'_, '_, DB>) {
        loop {}
    }
}
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct InfixNode<T, U, M> {
    lhs: T,
    rhs: U,
    middle: M,
}
impl<T, U, M> InfixNode<T, U, M> {
    #[doc(hidden)]
    pub const fn new(lhs: T, rhs: U, middle: M) -> Self {
        loop {}
    }
}
impl<T, U, DB, M> QueryFragment<DB> for InfixNode<T, U, M>
where
    DB: Backend + DieselReserveSpecialization,
    T: QueryFragment<DB>,
    U: QueryFragment<DB>,
    M: MiddleFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
