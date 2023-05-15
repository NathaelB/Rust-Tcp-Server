use std::ffi::NulError;

pub enum Error {
  InvalidCString (NulError),
  DatabaseError(
    DatabaseErrorKind,
    Box<dyn DatabaseErrorInformation + Send + Sync>
  ),
  NotFound,
  QueryBuilderError(
    Box<dyn StdError + Send + Sync>
  ),
  DeserializationError(Box<dyn StdError + Send + Sync>),
  SerializationError(Box<dyn StdError + Send + Sync>),
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