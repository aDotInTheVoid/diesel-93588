use crate::query_builder::upsert::on_conflict_actions::Excluded;
mod on_conflict_extension;
pub use self::on_conflict_extension::DecoratableTarget;
pub use self::on_conflict_extension::*;
#[cfg(feature = "postgres_backend")]
pub use crate::pg::query_builder::on_constraint::*;
pub fn excluded<T>(excluded: T) -> Excluded<T> {
    loop {}
}
