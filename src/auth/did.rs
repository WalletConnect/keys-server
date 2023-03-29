pub const DID_DELIMITER: &str = ":";
pub const DID_PREFIX: &str = "did";
pub const DID_METHOD_KEY: &str = "key";
pub const DID_METHOD_PKH: &str = "pkh";

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum DidError {
    #[error("Invalid issuer DID prefix")]
    Prefix,

    #[error("Invalid issuer DID method")]
    Method,

    #[error("Invalid issuer format")]
    Format,
}

pub fn extract_did_data<'a>(did: &'a str, method: &'a str) -> Result<&'a str, DidError> {
    let data = did
        .strip_prefix(DID_PREFIX)
        .ok_or(DidError::Prefix)?
        .strip_prefix(DID_DELIMITER)
        .ok_or(DidError::Format)?
        .strip_prefix(method)
        .ok_or(DidError::Method)?
        .strip_prefix(DID_DELIMITER)
        .ok_or(DidError::Format)?;
    Ok(data)
}

/// Checks whether the provided string is a valid `did` according to the
/// X25519[1] spec.
///
/// [1]: https://w3c-ccg.github.io/did-method-key/#x25519
pub fn validate_x25519(did: &str) -> bool {
    did.starts_with("did:key:z6LS")
}
