use {
    crate::auth::{
        did,
        jwt::{JwtClaims, JwtVerifierByIssuer},
    },
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
        // TODO: Add validation:
        // aud must be equal this dns?
        // exp must be in future
        // iat must be in past
        // sub must be valid public key
        // pkh must be valid did:pkh

        did::validate_x25519(&self.iss)
    }
}

impl JwtVerifierByIssuer for InviteKeyClaims {
    fn get_iss(&self) -> &str {
        &self.iss
    }
}

#[cfg(test)]
mod test_claims_validation {
    use super::{InviteKeyClaims, JwtClaims as _};

    fn default() -> InviteKeyClaims {
        InviteKeyClaims {
            aud: String::new(),
            exp: 0,
            iat: 0,
            iss: String::new(),
            sub: String::new(),
            pkh: String::new(),
        }
    }

    #[test]
    fn fails_on_incorrect_claims() {
        let mut claims = default();
        assert!(!claims.is_valid());

        claims.iss = "ababagalamaga".to_string();
        assert!(!claims.is_valid());

        claims.iss = "did:key:zQ3shokFTS3brHcDQrn82RUDfCZESWL1ZdCEJwekUDPQiYBme".to_string();
        assert!(!claims.is_valid());

        claims.iss = "did:abc".to_string();
        assert!(!claims.is_valid());
    }

    #[test]
    fn succeeds_on_correct_claims() {
        let mut claims = default();

        claims.iss = "did:key:z6LSeu9HkTHSfLLeUs2nnzUSNedgDUevfNQgQjQC23ZCit6F".to_string();
        assert!(claims.is_valid());

        claims.iss = "did:key:z6LStiZsmxiK4odS4Sb6JmdRFuJ6e1SYP157gtiCyJKfrYha".to_string();
        assert!(claims.is_valid());

        claims.iss = "did:key:z6LSoMdmJz2Djah2P4L9taDmtqeJ6wwd2HhKZvNToBmvaczQ".to_string();
        assert!(claims.is_valid());
    }
}
