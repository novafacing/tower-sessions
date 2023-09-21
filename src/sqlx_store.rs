#[cfg(feature = "postgres-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres-store")))]
pub use self::postgres_store::PostgresStore;
#[cfg(feature = "sqlite-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite-store")))]
pub use self::sqlite_store::SqliteStore;
use crate::session::SessionError;

#[cfg(feature = "sqlite-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite-store")))]
mod sqlite_store;

#[cfg(feature = "postgres-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres-store")))]
mod postgres_store;

/// An error type for SQLx stores.
#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum SqlxStoreError {
    /// A variant to map session errors.
    #[error(transparent)]
    SessionError(#[from] SessionError),

    /// A variant to map `sqlx` errors.
    #[error("SQLx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    /// A variant to map `serde_json` errors.
    #[error("JSON serialization/deserialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}