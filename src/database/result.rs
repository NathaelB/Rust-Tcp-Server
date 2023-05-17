use std::ffi::NulError;

pub enum Error {
  InvalidCString (NulError),
  NotFound,
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