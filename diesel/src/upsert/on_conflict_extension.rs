use crate::expression::Expression;
use crate::query_builder::upsert::into_conflict_clause::IntoConflictValueClause;
use crate::query_builder::upsert::on_conflict_actions::*;
use crate::query_builder::upsert::on_conflict_clause::*;
use crate::query_builder::upsert::on_conflict_target::*;
pub use crate::query_builder::upsert::on_conflict_target_decorations::DecoratableTarget;
use crate::query_builder::{AsChangeset, InsertStatement, UndecoratedInsertRecord};
use crate::query_source::QuerySource;
use crate::sql_types::BoolOrNullableBool;
impl<T, U, Op, Ret> InsertStatement<T, U, Op, Ret>
where
    T: QuerySource,
    U: UndecoratedInsertRecord<T> + IntoConflictValueClause,
{
                                                                                                                                                                                                                                                        pub fn on_conflict_do_nothing(
        self,
    ) -> InsertStatement<
            T,
            OnConflictValues<U::ValueClause, NoConflictTarget, DoNothing>,
            Op,
            Ret,
        > {
        loop {}
    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                    pub fn on_conflict<Target>(
        self,
        target: Target,
    ) -> IncompleteOnConflict<
            InsertStatement<T, U::ValueClause, Op, Ret>,
            ConflictTarget<Target>,
        >
    where
        ConflictTarget<Target>: OnConflictTarget<T>,
    {
        loop {}
    }
}
impl<Stmt, T, P> DecoratableTarget<P> for IncompleteOnConflict<Stmt, T>
where
    P: Expression,
    P::SqlType: BoolOrNullableBool,
    T: DecoratableTarget<P>,
{
    type FilterOutput = IncompleteOnConflict<
        Stmt,
        <T as DecoratableTarget<P>>::FilterOutput,
    >;
    fn filter_target(self, predicate: P) -> Self::FilterOutput {
        loop {}
    }
}
#[derive(Debug, Clone, Copy)]
pub struct IncompleteOnConflict<Stmt, Target> {
    stmt: Stmt,
    target: Target,
}
impl<
    T: QuerySource,
    U,
    Op,
    Ret,
    Target,
> IncompleteOnConflict<InsertStatement<T, U, Op, Ret>, Target> {
                                    pub fn do_nothing(
        self,
    ) -> InsertStatement<T, OnConflictValues<U, Target, DoNothing>, Op, Ret> {
        loop {}
    }
}
impl<Stmt, Target> IncompleteOnConflict<Stmt, Target> {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                pub fn do_update(self) -> IncompleteDoUpdate<Stmt, Target> {
        loop {}
    }
}
#[derive(Debug, Clone, Copy)]
pub struct IncompleteDoUpdate<Stmt, Target> {
    stmt: Stmt,
    target: Target,
}
impl<
    T: QuerySource,
    U,
    Op,
    Ret,
    Target,
> IncompleteDoUpdate<InsertStatement<T, U, Op, Ret>, Target> {
                pub fn set<Changes>(
        self,
        changes: Changes,
    ) -> InsertStatement<
            T,
            OnConflictValues<U, Target, DoUpdate<Changes::Changeset>>,
            Op,
            Ret,
        >
    where
        T: QuerySource,
        Changes: AsChangeset<Target = T>,
    {
        loop {}
    }
}
