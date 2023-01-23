use {
    super::{
        did::{extract_did_data, DID_METHOD_KEY},
        public_key::PublicKey,
    },
    serde::{de::DeserializeOwned, Deserialize, Serialize},
    std::str,
    thiserror::Error as ThisError,
};

pub const JWT_DELIMITER: &str = ".";
pub const JWT_HEADER_TYP: &str = "JWT";
pub const JWT_HEADER_ALG: &str = "EdDSA";

/// Errors that can occur during JWT verification
#[derive(Debug, ThisError)]
pub enum JwtError {
    #[error("Invalid format")]
    Format,

    #[error("Invalid encoding")]
    Encoding,

    #[error("Invalid JWT signing algorithm")]
    Header,

    #[error("Invalid claims")]
    Claims,

    #[error("Invalid signature")]
    Signature,

    #[error("Invalid JSON")]
    Serialization,

    #[error("Invalid issuer")]
    IssuerInvalid,
}

/// Takes the result of a rsplit and ensure we only get 3 parts
/// Errors if we don't
fn expect_three<'a, I>(vals: I) -> Result<(&'a str, &'a str, &'a str), JwtError>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut i = vals.into_iter();
    match (i.next(), i.next(), i.next(), i.next()) {
        (Some(first), Some(second), Some(third), None) => Ok((first, second, third)),
        _ => Err(JwtError::Format),
    }
}

fn expect_two<'a, I>(vals: I) -> Result<(&'a str, &'a str), JwtError>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut i = vals.into_iter();
    match (i.next(), i.next(), i.next()) {
        (Some(first), Some(second), None) => Ok((first, second)),
        _ => Err(JwtError::Format),
    }
}

pub trait JwtVerifierByIssuer {
    fn get_iss(&self) -> &str;

    fn verify(&self, token: &str) -> Result<(), JwtError> {
        let did_key = extract_did_data(self.get_iss(), DID_METHOD_KEY)
            .map_err(|_| JwtError::IssuerInvalid)?;

        let pub_key = did_key
            .parse::<PublicKey>()
            .map_err(|_| JwtError::IssuerInvalid)?;

        let key = jsonwebtoken::DecodingKey::from_ed_der(pub_key.as_ref());

        let parts = token.rsplitn(2, JWT_DELIMITER);

        let (signature, message) = expect_two(parts)?;

        // Verify signature with public key
        let result = jsonwebtoken::crypto::verify(
            signature,
            message.as_bytes(),
            &key,
            jsonwebtoken::Algorithm::EdDSA,
        );

        match result {
            Ok(true) => Ok(()),
            _ => Err(JwtError::Signature),
        }
    }
}

#[derive(Debug)]
pub struct Jwt<T: JwtClaims> {
    token: String,
    header: JwtHeader,
    pub claims: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtHeader {
    pub typ: String,
    pub alg: String,
}

impl JwtHeader {
    pub fn is_valid(&self) -> bool {
        self.typ == JWT_HEADER_TYP && self.alg == JWT_HEADER_ALG
    }
}

pub trait JwtClaims: DeserializeOwned + Serialize + JwtVerifierByIssuer {
    fn is_valid(&self) -> bool;
}

impl<T: JwtClaims> Jwt<T> {
    /// Create a new JWT from a string
    /// Errors if the JWT is invalid
    pub fn new(string: &str) -> Self {
        let (header, claims) = Self::decode(string).unwrap();
        Jwt {
            token: string.to_string(),
            header,
            claims,
        }
    }

    /// Verify the JWT
    /// Errors if the JWT is invalid
    pub fn verify(&self) -> Result<(), JwtError> {
        // // Header validity checks.
        if !self.header.is_valid() {
            return Err(JwtError::Header);
        }

        // Token validity checks.
        if !self.claims.is_valid() {
            return Err(JwtError::Claims);
        }

        // Signature verification.
        self.claims.verify(&self.token)
    }

    /// Decode a JWT string into a tuple of (header, claims, signature)
    /// Errors if the JWT is invalid
    fn decode(string: &str) -> Result<(JwtHeader, T), JwtError> {
        let parts = string.splitn(3, JWT_DELIMITER);

        let (encoded_header, encoded_claims, _) = expect_three(parts)?;

        let decoder = &data_encoding::BASE64URL_NOPAD;

        let header_len = decoder
            .decode_len(encoded_header.len())
            .map_err(|_| JwtError::Encoding)?;
        let claims_len = decoder
            .decode_len(encoded_claims.len())
            .map_err(|_| JwtError::Encoding)?;

        let mut header_output = vec![0u8; header_len];
        let mut claims_output = vec![0u8; claims_len];

        // Decode header.
        data_encoding::BASE64URL_NOPAD
            .decode_mut(encoded_header.as_bytes(), &mut header_output[..header_len])
            .map_err(|_| JwtError::Encoding)?;

        let header = serde_json::from_slice::<JwtHeader>(&header_output[..header_len])
            .map_err(|_| JwtError::Serialization)?;

        data_encoding::BASE64URL_NOPAD
            .decode_mut(encoded_claims.as_bytes(), &mut claims_output[..claims_len])
            .map_err(|_| JwtError::Encoding)?;

        let claims = serde_json::from_slice::<T>(&claims_output[..claims_len])
            .map_err(|_| JwtError::Serialization)?;

        Ok((header, claims))
    }
}

#[cfg(test)]
mod tests;
