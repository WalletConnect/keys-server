pub use {
    super::StoreError,
    crate::log::prelude::info,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    wither::{
        bson::{self, doc, oid::ObjectId},
        mongodb::{
            options::{ClientOptions, FindOptions},
            Client, Database,
        },
        Model,
    },
};
use {
    crate::config::Configuration, relay_rpc::auth::cacao::Cacao,
    wither::mongodb::options::FindOneAndUpdateOptions,
};

pub type KeysPersistentStorageArc = Arc<dyn KeysPersistentStorage + Send + Sync + 'static>;

#[async_trait]
pub trait KeysPersistentStorage: 'static + std::fmt::Debug + Send + Sync {
    async fn upsert_invite_key(&self, account: &str, invite_key: &str) -> Result<(), StoreError>;

    async fn create_account_if_not_exists_and_add_identity_key(
        &self,
        account: &str,
        identity_key: &str,
        cacao: &Cacao,
    ) -> Result<(), StoreError>;

    async fn remove_identity_key(
        &self,
        account: &str,
        identity_key: &str,
    ) -> Result<(), StoreError>;

    async fn get_cacao_by_identity_key(&self, identity_key: &str) -> Result<Cacao, StoreError>;
    async fn remove_invite_key(&self, account: &str) -> Result<(), StoreError>;
    async fn retrieve_invite_key(&self, account: &str) -> Result<String, StoreError>;
    async fn remove(&self, account: &str) -> Result<(), StoreError>;
}

#[derive(Debug, Model, Serialize, Deserialize, PartialEq, Eq)]
#[model(
    collection_name = "keys",
    index(keys = r#"doc!{"account": 1}"#, options = r#"doc!{"unique": true}"#),
    index(
        keys = r#"doc!{"identities.identity_key": 1}"#,
        options = r#"doc!{"unique": true, "sparse": true}"#
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

    /// TODO describe identities
    pub identities: Vec<MongoIdentity>,

    /// The proposal encryption key used by a peer client to derive a shared DH
    /// symmetric key to encrypt proposals.
    pub invite_key: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MongoIdentity {
    pub identity_key: String,
    pub cacao: Cacao,
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
    async fn upsert_invite_key(&self, account: &str, invite_key: &str) -> Result<(), StoreError> {
        let filter = doc! {
            "account": &account,
        };

        let update = doc! {
            "$set": {
                "invite_key": &invite_key,
            }
        };

        match MongoKeys::find_one_and_update(&self.db, filter, update, None).await? {
            Some(_) => Ok(()),
            None => Err(StoreError::NotFound(
                "Account".to_string(),
                account.to_string(),
            )),
        }
    }

    async fn create_account_if_not_exists_and_add_identity_key(
        &self,
        account: &str,
        identity_key: &str,
        cacao: &Cacao,
    ) -> Result<(), StoreError> {
        let filter = doc! {
            "account": &account,
            "identities.identity_key": {"$ne": &identity_key},
        };

        let mongo_identity = MongoIdentity {
            identity_key: identity_key.to_string(),
            cacao: cacao.clone(),
        };

        let update = doc! {
            "$push": {
                "identities": bson::to_bson(&mongo_identity).unwrap()
            }
        };

        let option = FindOneAndUpdateOptions::builder().upsert(true).build();

        match MongoKeys::find_one_and_update(&self.db, filter, update, option).await {
            Ok(Some(_)) => Ok(()),
            Ok(None) => Ok(()),
            Err(e) => {
                if e.to_string().starts_with(
                    "Command failed (DuplicateKey): E11000 duplicate key error collection: \
                     keyserver.keys index: account_1",
                )
                // Todo add better error matching
                {
                    Ok(())
                } else {
                    Err(StoreError::Database(e))
                }
            }
        }
    }

    async fn remove_identity_key(
        &self,
        account: &str,
        identity_key: &str,
    ) -> Result<(), StoreError> {
        let filter = doc! {
            "account": &account,
        };

        let update = doc! {
            "$pull": {
                "identities" : {
                    "identity_key": &identity_key,
                }
            }
        };

        match MongoKeys::find_one_and_update(&self.db, filter, update, None).await? {
            Some(_) => Ok(()),
            None => Err(StoreError::NotFound(
                "Account".to_string(),
                account.to_string(),
            )),
        }
    }

    async fn remove_invite_key(&self, account: &str) -> Result<(), StoreError> {
        let filter = doc! {
            "account": &account,
        };

        let update = doc! {
            "$unset": {
                "invite_key": 1,
            }
        };

        match MongoKeys::find_one_and_update(&self.db, filter, update, None).await? {
            Some(_) => Ok(()),
            None => Err(StoreError::NotFound(
                "Account".to_string(),
                account.to_string(),
            )),
        }
    }

    async fn get_cacao_by_identity_key(&self, identity_key: &str) -> Result<Cacao, StoreError> {
        info!("get_cacao_by_identity_key");
        let filter = doc! {
            "identities.identity_key": identity_key,
            "identities.identity_key": {
                "$exists": true, // https://docs.aws.amazon.com/documentdb/latest/developerguide/functional-differences.html#functional-differences.sparse-index
                "$eq": identity_key,
            },
        };

        info!("constructing not_found");
        let not_found = StoreError::NotFound("Identity key".to_string(), identity_key.to_string());

        info!("find_one");
        match MongoKeys::find_one(&self.db, Some(filter), None).await? {
            Some(mongo_keys) => {
                info!(
                    "Account:{:?} has {:?} identities. Returned entity: {:?}",
                    mongo_keys.account,
                    mongo_keys.identities.len(),
                    mongo_keys
                );

                info!("Timing - find - Start");
                let mongo_identity = mongo_keys
                    .identities
                    .into_iter()
                    .find(|i| i.identity_key == *identity_key)
                    .ok_or(not_found)?;
                info!("Timing - find - End");

                Ok(mongo_identity.cacao)
            }
            None => Err(not_found),
        }
    }

    async fn retrieve_invite_key(&self, account: &str) -> Result<String, StoreError> {
        let filter = doc! {
            "account": account,
        };

        match MongoKeys::find_one(&self.db, Some(filter), None).await? {
            Some(keys) => Ok(keys.invite_key.ok_or_else(|| {
                StoreError::NotFound("Invite key".to_string(), account.to_string())
            })?),
            None => Err(StoreError::NotFound(
                "Account".to_string(),
                account.to_string(),
            )),
        }
    }

    async fn remove(&self, account: &str) -> Result<(), StoreError> {
        let filter = doc! {
            "account": account,
        };

        match MongoKeys::find_one_and_delete(&self.db, filter, None).await? {
            Some(_) => Ok(()),
            None => Err(StoreError::NotFound(
                "Account".to_string(),
                account.to_string(),
            )),
        }
    }
}
