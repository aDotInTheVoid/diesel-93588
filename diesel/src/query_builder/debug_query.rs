use super::{AstPass, QueryBuilder, QueryFragment};
use crate::backend::Backend;
use std::fmt::{self, Debug, Display};
use std::marker::PhantomData;
pub struct DebugQuery<'a, T: 'a, DB> {
    pub(crate) query: &'a T,
    _marker: PhantomData<DB>,
}
impl<'a, T, DB> DebugQuery<'a, T, DB> {
    pub(crate) fn new(query: &'a T) -> Self {
        loop {}
    }
}
impl<'a, T, DB> Display for DebugQuery<'a, T, DB>
where
    DB: Backend + Default,
    DB::QueryBuilder: Default,
    T: QueryFragment<DB>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl<'a, T, DB> Debug for DebugQuery<'a, T, DB>
where
    DB: Backend + Default,
    DB::QueryBuilder: Default,
    T: QueryFragment<DB>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
pub(crate) struct DebugBinds<'a, T: 'a, DB> {
    query: &'a T,
    _marker: PhantomData<DB>,
}
impl<'a, T, DB> DebugBinds<'a, T, DB> {
    fn new(query: &'a T) -> Self {
        loop {}
    }
}
impl<'a, T, DB> Debug for DebugBinds<'a, T, DB>
where
    DB: Backend + Default,
    T: QueryFragment<DB>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
