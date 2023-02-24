use {
    crate::auth::jwt::{JwtClaims, JwtVerifierByIssuer},
    serde::{Deserialize, Serialize},
};

pub mod register;
pub mod resolve;
pub mod unregister;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityKeyClaims {
    aud: String, // keys server url used for registering
    exp: usize,  // timestamp when jwt must expire TODO: Should be 1 hour
    iat: usize,  // timestamp when jwt was issued
    iss: String, // public identity key in form of did:key, also used to verify jwt signature
    pkh: String, // corresponding blockchain account (did:pkh)
}

impl JwtClaims for IdentityKeyClaims {
    fn is_valid(&self) -> bool {
        true
        // TODO: Add validation:
        // aud must be equal this dns?
        // exp must be in future
        // iat must be in past
        // iss must be valid did:key
        // pkh must be valid did:pkh
    }
}

impl JwtVerifierByIssuer for IdentityKeyClaims {
    fn get_iss(&self) -> &str {
        &self.iss
    }
}
