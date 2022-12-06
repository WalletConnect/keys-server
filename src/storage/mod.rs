use {async_trait::async_trait, std::fmt::Debug, thiserror::Error as ThisError};

pub mod memory;
pub mod mongo;

/// The error produced from most Storage functions
#[derive(Clone, Debug, ThisError)]
pub enum StorageError {
    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("MongoDB Access error: {0}")]
    MongoAccessError(#[from] mongodb::bson::document::ValueAccessError),

    /// An unexpected error occurred
    #[error("{0:?}")]
    Other(String),
}

#[async_trait]
pub trait Storage: Debug + Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, StorageError>;
    async fn set(&self, key: &str, value: &str) -> Result<(), StorageError>;
    async fn remove(&self, key: &str) -> Result<(), StorageError>;
    async fn count(&self) -> Result<usize, StorageError>;
    async fn clear(&self) -> Result<(), StorageError>;
}
