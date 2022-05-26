use std::convert::From;
use std::error::Error as StdError;
use std::ffi::NulError;
use std::fmt::{self, Display};
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
#[non_exhaustive]
pub enum Error {
                InvalidCString(NulError),
                            DatabaseError(DatabaseErrorKind, Box<dyn DatabaseErrorInformation + Send + Sync>),
                                            NotFound,
                        QueryBuilderError(Box<dyn StdError + Send + Sync>),
                        DeserializationError(Box<dyn StdError + Send + Sync>),
                        SerializationError(Box<dyn StdError + Send + Sync>),
                    RollbackError(Box<Error>),
                            RollbackTransaction,
            AlreadyInTransaction,
            NotInTransaction,
        BrokenTransaction,
                        CommitTransactionFailed {
                commit_error: Box<Error>,
                rollback_result: Box<QueryResult<()>>,
    },
}
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum DatabaseErrorKind {
        UniqueViolation,
        ForeignKeyViolation,
                    UnableToSendCommand,
                                SerializationFailure,
                        ReadOnlyTransaction,
        NotNullViolation,
        CheckViolation,
                    ClosedConnection,
    #[doc(hidden)]
    Unknown,
}
pub trait DatabaseErrorInformation {
        fn message(&self) -> &str;
            fn details(&self) -> Option<&str>;
            fn hint(&self) -> Option<&str>;
                            fn table_name(&self) -> Option<&str>;
                            fn column_name(&self) -> Option<&str>;
                        fn constraint_name(&self) -> Option<&str>;
            fn statement_position(&self) -> Option<i32>;
}
impl fmt::Debug for dyn DatabaseErrorInformation + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl DatabaseErrorInformation for String {
    fn message(&self) -> &str {
        loop {}
    }
    fn details(&self) -> Option<&str> {
        loop {}
    }
    fn hint(&self) -> Option<&str> {
        loop {}
    }
    fn table_name(&self) -> Option<&str> {
        loop {}
    }
    fn column_name(&self) -> Option<&str> {
        loop {}
    }
    fn constraint_name(&self) -> Option<&str> {
        loop {}
    }
    fn statement_position(&self) -> Option<i32> {
        loop {}
    }
}
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum ConnectionError {
        InvalidCString(NulError),
        BadConnection(String),
        InvalidConnectionUrl(String),
                            CouldntSetupConfiguration(Error),
}
pub type QueryResult<T> = Result<T, Error>;
pub type ConnectionResult<T> = Result<T, ConnectionError>;
pub trait OptionalExtension<T> {
                                                                                    fn optional(self) -> Result<Option<T>, Error>;
}
impl<T> OptionalExtension<T> for QueryResult<T> {
    fn optional(self) -> Result<Option<T>, Error> {
        loop {}
    }
}
impl From<NulError> for ConnectionError {
    fn from(e: NulError) -> Self {
        loop {}
    }
}
impl From<NulError> for Error {
    fn from(e: NulError) -> Self {
        loop {}
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        loop {}
    }
}
impl Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for ConnectionError {
    fn cause(&self) -> Option<&dyn StdError> {
        loop {}
    }
}
impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        loop {}
    }
}
#[cfg(test)]
#[allow(warnings)]
fn error_impls_send() {
    loop {}
}
#[derive(Debug, Clone, Copy)]
pub struct UnexpectedNullError;
impl fmt::Display for UnexpectedNullError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for UnexpectedNullError {}
#[derive(Debug, Clone, Copy)]
pub struct UnexpectedEndOfRow;
impl fmt::Display for UnexpectedEndOfRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl StdError for UnexpectedEndOfRow {}
