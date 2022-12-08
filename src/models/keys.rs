use {
    serde::{Deserialize, Serialize},
    serde_json::{json, Value},
    validator::Validate,
};

/// Represents entry of keys in the keystore.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Validate)]
pub struct Keys {
    /// The account in CAIP-10 account identifier associated and controlled with
    /// a blockchain private key.
    /// I.e: "eip155:1:0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826".
    pub account: String,

    /// The identity keys used by the client to authenticate payloads regarding
    /// the proposals and responses.
    #[serde(rename = "identityKeys")]
    pub identity_keys: Vec<IdentityKey>,

    /// The proposal encryption key used by a peer client to derive a shared DH
    /// symmetric key to encrypt proposals.
    #[serde(rename = "proposalEncryptionKey")]
    #[validate(length(min = 64))]
    pub proposal_encryption_key: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Validate)]
pub struct IdentityKey {
    /// The identity key used by the client to authenticate payloads regarding
    /// the proposals and responses.
    #[serde(rename = "identityKey")]
    #[validate(length(min = 64))]
    pub identity_key: String,
}

impl Into<IdentityKey> for String {
    fn into(self) -> IdentityKey {
        IdentityKey { identity_key: self }
    }
}

impl Into<Value> for Keys {
    fn into(self) -> Value {
        json!(self)
    }
}

impl Into<Value> for IdentityKey {
    fn into(self) -> Value {
        json!(self)
    }
}
