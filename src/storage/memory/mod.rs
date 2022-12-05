use {
    crate::storage::{Storage, StorageError},
    async_trait::async_trait,
    std::{
        collections::HashMap,
        fmt::Debug,
        sync::RwLock,
    },
    tracing::error,
};

#[cfg(test)]
mod tests;

#[derive(Debug, Default)]
pub struct MemoryStorage {
    db: RwLock<HashMap<String, String>>,
}

#[async_trait]
impl Storage for MemoryStorage
{
    async fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        self.db.read()
            .map(|db| {
                db.get(key).cloned()
            })
            .map_err(|err| {
                error!(?err, "failed to acquire read lock for memory store");
                StorageError::Other(err.to_string())
            })
    }

    async fn set(&self, key: &str, value: &str) -> Result<Option<String>, StorageError> {
        self.db
            .write()
            .map(|mut db| db.insert(key.to_string(), value.to_string()))
            .map_err(|err| {
                error!(?err, "failed to acquire write lock for memory store");
                StorageError::Other(err.to_string())
            })
    }

    async fn remove(&self, key: &str) -> Result<Option<String>, StorageError> {
        self.db.write().map(|mut db| db.remove(key)).map_err(|err| {
            error!(?err, "failed to acquire write lock for memory store");
            StorageError::Other(err.to_string())
        })
    }

    async fn count(&self) -> Result<usize, StorageError> {
        self.db.read().map(|db| db.len()).map_err(|err| {
            error!(?err, "failed to acquire read lock for memory store");
            StorageError::Other(err.to_string())
        })
    }

    async fn clear(&self) -> Result<(), StorageError> {
        self.db
            .write()
            .map(|mut db| {
                db.clear();
            })
            .map_err(|err| {
                error!(?err, "failed to acquire write lock for memory store");
                StorageError::Other(err.to_string())
            })
    }
}
