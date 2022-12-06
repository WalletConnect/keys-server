use crate::storage::StorageError;

#[derive(Debug, thiserror::Error)]
pub enum KeyserverError {
    #[error(transparent)]
    EnvyError(#[from] envy::Error),

    #[error("Invalid socket address: {0}")]
    InvalidSocketAddress(#[from] std::net::AddrParseError),

    #[error("Storage error")]
    StorageError(#[from] StorageError),
}
