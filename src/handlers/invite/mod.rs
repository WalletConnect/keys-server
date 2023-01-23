use {
    crate::auth::jwt::{JwtClaims, JwtVerifierByIssuer},
    serde::{Deserialize, Serialize},
};

pub mod register;
pub mod resolve;
pub mod unregister;

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteKeyClaims {
    aud: String, // keys server url used for registering
    exp: usize,  // timestamp when jwt must expire TODO: Should be 1 hour
    iat: usize,  // timestamp when jwt was issued
    iss: String, // public identity key in form of did:key, also used to verify jwt signature
    sub: String, // public key for chat invite key
    pkh: String, // corresponding blockchain account (did:pkh)
}

impl JwtClaims for InviteKeyClaims {
    fn is_valid(&self) -> bool {
        true
        // TODO: Add validation:
        // aud must be equal this dns?
        // exp must be in future
        // iat must be in past
        // iss must be valid did:key
        // sub must be valid public key
        // pkh must be valid did:pkh
    }
}

impl JwtVerifierByIssuer for InviteKeyClaims {
    fn get_iss(&self) -> &str {
        &self.iss
    }
}
