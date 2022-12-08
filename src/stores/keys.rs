pub use {
    super::StoreError,
    crate::models::keys::Keys,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    wither::{
        bson::{self, doc, oid::ObjectId},
        mongodb::{
            options::{ClientOptions, FindOptions},
            Client,
            Database,
        },
        Model,
    },
};
use {
    crate::{config::Configuration, models::keys::IdentityKey},
    std::str::FromStr,
    wither::mongodb::options::{FindOneAndUpdateOptions, UpdateOptions},
};

pub type KeysPersistentStorageArc = Arc<dyn KeysPersistentStorage + Send + Sync + 'static>;

#[async_trait]
pub trait KeysPersistentStorage: 'static + std::fmt::Debug + Send + Sync {
    async fn upsert_proposal_encryption_key(
        &self,
        account: &String,
        proposal_encryption_key: &String,
    ) -> Result<(), StoreError>;

    async fn create_account_if_not_exists_and_add_identity_key(
        &self,
        account: &String,
        identity_key: &IdentityKey,
    ) -> Result<(), StoreError>;

    async fn remove_identity_key(
        &self,
        account: &String,
        identity_key: &IdentityKey,
    ) -> Result<(), StoreError>;

    async fn exists_identity_key(&self, identity_key: &String) -> Result<bool, StoreError>;
    async fn retrieve(&self, account: &String) -> Result<Keys, StoreError>;
    async fn remove(&self, account: &String) -> Result<(), StoreError>;
}

#[derive(Debug, Model, Serialize, Deserialize, PartialEq, Eq)]
#[model(
    collection_name = "keys",
    index(keys = r#"doc!{"account": 1}"#, options = r#"doc!{"unique": true}"#),
    index(
        keys = r#"doc!{"identity_keys.identity_key": 1}"#,
        options = r#"doc!{"unique": true}"#
    )
)]
struct MongoKeys {
    /// Mongo's default `_id` field.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    /// The account in CAIP-10 account identifier associated and controlled with
    /// a blockchain private key. I.e. eip155:1:
    /// 0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826.
    pub account: String,

    /// The identity key used by the client to authenticate payloads regarding
    /// the proposals and responses.
    pub identity_keys: Vec<MongoIdentityKey>,

    /// The proposal encryption key used by a peer client to derive a shared DH
    /// symmetric key to encrypt proposals.
    pub proposal_encryption_key: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MongoIdentityKey {
    /// The identity key used by the client to authenticate payloads regarding
    /// the proposals and responses.
    pub identity_key: String,
}

impl From<MongoKeys> for Keys {
    fn from(value: MongoKeys) -> Self {
        Self {
            account: value.account,
            identity_keys: value.identity_keys.into_iter().map(Into::into).collect(),
            proposal_encryption_key: value.proposal_encryption_key,
        }
    }
}

impl From<IdentityKey> for MongoIdentityKey {
    fn from(value: IdentityKey) -> Self {
        Self {
            identity_key: value.identity_key,
        }
    }
}

impl From<MongoIdentityKey> for IdentityKey {
    fn from(value: MongoIdentityKey) -> Self {
        Self {
            identity_key: value.identity_key,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MongoPersistentStorage {
    db: Database,
}

impl MongoPersistentStorage {
    pub async fn new(config: &Configuration) -> anyhow::Result<Self> {
        let url = &config.database_url;

        let client_options = ClientOptions::parse(url).await?;
        let client = Client::with_options(client_options)?;
        let db = client.default_database().ok_or_else(|| {
            anyhow::anyhow!("no default database specified in the connection URL")
        })?;

        MongoKeys::sync(&db).await?;

        Ok(Self { db })
    }
}

#[async_trait]
impl KeysPersistentStorage for MongoPersistentStorage {
    async fn upsert_proposal_encryption_key(
        &self,
        account: &String,
        proposal_encryption_key: &String,
    ) -> Result<(), StoreError> {
        let filter = doc! {
            "account": &account,
        };

        let update = doc! {
            "$set": {
                "proposal_encryption_key": &proposal_encryption_key,
            }
        };

        match MongoKeys::find_one_and_update(&self.db, filter, update, None).await? {
            Some(_) => Ok(()),
            None => Err(StoreError::NotFound("Account".to_string(), account.clone())),
        }
    }

    async fn create_account_if_not_exists_and_add_identity_key(
        &self,
        account: &String,
        identity_key: &IdentityKey,
    ) -> Result<(), StoreError> {
        let filter = doc! {
            "account": &account,
        };

        let mongo_key = MongoIdentityKey::from(identity_key.clone());

        let update = doc! {
            "$addToSet": {
                "identity_keys": bson::to_bson(&mongo_key).unwrap()
            }
        };

        let option = FindOneAndUpdateOptions::builder().upsert(true).build();

        MongoKeys::find_one_and_update(&self.db, filter, update, option).await?;

        Ok(())
    }

    async fn remove_identity_key(
        &self,
        account: &String,
        identity_key: &IdentityKey,
    ) -> Result<(), StoreError> {
        let filter = doc! {
            "account": &account,
        };

        let mongo_key = MongoIdentityKey::from(identity_key.clone());

        let update = doc! {
            "$pull": {
                "identity_keys": bson::to_bson(&mongo_key).unwrap()
            }
        };

        let option = FindOneAndUpdateOptions::builder().upsert(true).build();

        match MongoKeys::find_one_and_update(&self.db, filter, update, option).await? {
            Some(_) => Ok(()),
            None => Err(StoreError::NotFound("Account".to_string(), account.clone())),
        }
    }

    async fn exists_identity_key(&self, identity_key: &String) -> Result<bool, StoreError> {
        let filter = doc! {
            "identity_keys.identity_key": identity_key,
        };

        match MongoKeys::find_one(&self.db, Some(filter), None).await? {
            Some(_) => Ok(true),
            // note(Szymon): A little conflicted if this should be Succes with false value or
            // NotFound error.
            None => Ok(false),
        }
    }

    async fn retrieve(&self, account: &String) -> Result<Keys, StoreError> {
        let filter = doc! {
            "account": account,
        };

        match MongoKeys::find_one(&self.db, Some(filter), None).await? {
            Some(keys) => Ok(keys.into()),
            None => Err(StoreError::NotFound("Account".to_string(), account.clone())),
        }
    }

    async fn remove(&self, account: &String) -> Result<(), StoreError> {
        let filter = doc! {
            "account": account,
        };

        match MongoKeys::find_one_and_delete(&self.db, filter, None).await? {
            Some(_) => Ok(()),
            None => Err(StoreError::NotFound("Account".to_string(), account.clone())),
        }
    }
}
