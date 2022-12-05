use {async_trait::async_trait, std::fmt::Debug, thiserror::Error as ThisError};

pub mod memory;

/// The error produced from most Storage functions
#[derive(Debug, ThisError)]
pub enum StorageError {
    /// An unexpected error occurred
    #[error("{0:?}")]
    Other(String),
}

#[async_trait]
pub trait Storage: Debug + Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, StorageError>;
    async fn set(&self, key: &str, value: &str) -> Result<Option<String>, StorageError>;
    async fn remove(&self, key: &str) -> Result<Option<String>, StorageError>;
    async fn count(&self) -> Result<usize, StorageError>;
    async fn clear(&self) -> Result<(), StorageError>;
}
