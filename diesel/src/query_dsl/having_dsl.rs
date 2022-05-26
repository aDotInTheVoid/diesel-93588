use crate::dsl;
pub trait HavingDsl<Predicate> {
        type Output;
        fn having(self, predicate: Predicate) -> dsl::Having<Self, Predicate>;
}
