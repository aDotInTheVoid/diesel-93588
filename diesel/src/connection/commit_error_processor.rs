//! A module to evaluate what to do when a commit triggers an error.
use crate::result::Error;
/// Transaction status returned upon error on commit.
#[derive(Debug)]
#[non_exhaustive]
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
pub enum CommitErrorOutcome {
    /// Necessitates a rollback to return to a valid transaction
    RollbackAndThrow(Error),
    /// Broken transaction. Returned if an error has occurred earlier in a Postgres transaction.
    Throw(Error),
    /// Broken transaction. Similar to `Throw`, but marks the manager as broken. It should switch
    /// to `TransactionManagerStatus::InError` and refuse to run additional operations.
    ThrowAndMarkManagerAsBroken(Error),
}
/// Trait needed for the transaction manager.
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
pub trait CommitErrorProcessor {
    /// Returns the status of the transaction following an error upon commit.
    /// When any of these kinds of error happen on `COMMIT`, it is expected
    /// that a `ROLLBACK` would succeed, leaving the transaction in a non-broken state.
    /// If there are other such errors, it is fine to add them here.
    fn process_commit_error(&self, error: Error) -> CommitErrorOutcome;
}
/// Default implementation of CommitErrorProcessor::process_commit_error(), used for MySql and
/// Sqlite connections. Returns `CommitErrorOutcome::RollbackAndThrow` if the transaction depth is
/// greater than 1, the error is a `DatabaseError` and the error kind is either
/// `DatabaseErrorKind::SerializationFailure` or `DatabaseErrorKind::ReadOnlyTransaction`
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
#[cfg(
    any(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        feature = "mysql",
        feature = "sqlite"
    )
)]
#[diesel_derives::__diesel_public_if(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
)]
pub(crate) fn default_process_commit_error(
    transaction_state: &super::ValidTransactionManagerStatus,
    error: Error,
) -> CommitErrorOutcome {
    loop {}
}
#[cfg(test)]
#[cfg(
    any(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        feature = "mysql",
        feature = "sqlite"
    )
)]
mod tests {
    use super::CommitErrorOutcome;
    use crate::connection::ValidTransactionManagerStatus;
    use crate::result::{DatabaseErrorKind, Error};
    use std::num::NonZeroU32;
    #[test]
    fn check_default_process_commit_error_implementation() {
        loop {}
    }
    #[test]
    #[should_panic]
    fn check_invalid_transaction_state_rejected() {
        loop {}
    }
}
