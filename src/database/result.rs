use std::ffi::NulError;

pub enum Error {
  InvalidCString (NulError),
  NotFound,
  RollbackErrorOnCommit {
    rollback_error: Box<Error>,
    commit_error: Box<Error>,
  },
  RollbackTransaction,
  AlreadyInTransaction,
  NotInTransaction,
  BrokenTransactionManager,
}

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