pub use config::*;
use {
    crate::storage::{Storage, StorageError},
    async_trait::async_trait,
    mongodb::{
        bson::{doc, Document},
        options::UpdateOptions,
        Client,
        Collection,
    },
    std::fmt::Debug,
};

mod config;

#[cfg(test)]
mod tests;

const DB_COLLECTION_NAME: &str = "keys";
const DB_FIELD_KEY: &str = "account";
const DB_FIELD_VALUE: &str = "key";

#[derive(Debug)]
pub struct MongoStorage {
    pub collection: Collection<Document>,
}

impl MongoStorage {
    pub async fn new(addr: &str, database: &str) -> Result<Self, StorageError> {
        let db = Client::with_uri_str(addr).await?.database(database);

        Ok(MongoStorage {
            collection: db.collection::<Document>(DB_COLLECTION_NAME),
        })
    }
}

#[async_trait]
impl Storage for MongoStorage {
    async fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        Ok(
            match self
                .collection
                .find_one(doc! { DB_FIELD_KEY: key }, None)
                .await?
            {
                Some(doc) => Some(doc.get_str(DB_FIELD_VALUE)?.to_string()),
                None => None,
            },
        )
    }

    async fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        self.collection
            .update_one(
                doc! { DB_FIELD_KEY: key },
                doc! {
                    "$set": { DB_FIELD_VALUE: value }
                },
                Some(UpdateOptions::builder().upsert(Some(true)).build()),
            )
            .await?;

        Ok(())
    }

    async fn remove(&self, key: &str) -> Result<(), StorageError> {
        self.collection
            .find_one_and_delete(doc! { DB_FIELD_KEY: key }, None)
            .await?;

        Ok(())
    }

    async fn count(&self) -> Result<usize, StorageError> {
        Ok(self.collection.count_documents(None, None).await? as usize)
    }

    async fn clear(&self) -> Result<(), StorageError> {
        self.collection.delete_many(doc! {}, None).await?;

        Ok(())
    }
}
