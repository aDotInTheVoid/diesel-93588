use crate::result::Error;
#[derive(Debug)]
#[non_exhaustive]
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
pub enum CommitErrorOutcome {
    RollbackAndThrow(Error),
    Throw(Error),
    ThrowAndMarkManagerAsBroken(Error),
}
#[cfg_attr(
    doc_cfg,
    doc(cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))
)]
pub trait CommitErrorProcessor {
    fn process_commit_error(&self, error: Error) -> CommitErrorOutcome;
}
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
