use {
    super::super::{validate_caip10_account, validate_identity_key, Response},
    crate::{
        auth::{
            did::{extract_did_data, DID_METHOD_KEY, DID_METHOD_PKH},
            jwt::{Jwt, JwtClaims, JwtVerifierByIssuer},
        },
        error,
        increment_counter,
        log::prelude::{info, warn},
        state::AppState,
    },
    axum::{extract::State, Json},
    serde::{Deserialize, Serialize},
    std::sync::Arc,
    validator::Validate,
};

#[derive(Deserialize)]
pub struct UnregisterIdentityPayload {
    #[serde(rename = "idAuth")]
    id_auth: String,
}

#[derive(Validate, Debug)]
pub struct UnregisterIdentityParams {
    #[validate(custom = "validate_caip10_account")]
    account: String,
    #[validate(custom = "validate_identity_key")]
    identity_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnregisterIdentityKeyClaims {
    aud: String, // keys server url used for registering
    exp: usize,  // timestamp when jwt must expire TODO: Should be 1 hour
    iat: usize,  // timestamp when jwt was issued
    iss: String, // public identity key in form of did:key, also used to verify jwt signature
    pkh: String, // corresponding blockchain account (did:pkh)
    act: String, // description of action intent. Must be equal to "unregister_identity"
}

impl JwtClaims for UnregisterIdentityKeyClaims {
    fn is_valid(&self) -> bool {
        // TODO: Add validation:
        // aud must be equal this dns?
        // exp must be in future
        // iat must be in past
        // iss must be valid did:key
        // pkh must be valid did:pkh
        self.act == "unregister_identity"
    }
}

impl JwtVerifierByIssuer for UnregisterIdentityKeyClaims {
    fn get_iss(&self) -> &str {
        &self.iss
    }
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UnregisterIdentityPayload>,
) -> error::Result<Response> {
    info!(
        "Handling - Unregister identity with jwt: {:?}",
        payload.id_auth
    );

    let jwt = Jwt::<UnregisterIdentityKeyClaims>::new(&payload.id_auth).map_err(|error| {
        increment_counter!(state.metrics, invalid_identity_unregister_jwt);
        info!(
            "Failure - Unregister identity with jwt: {:?}, error: {:?}",
            payload.id_auth, error
        );
        error
    })?;

    jwt.verify().map_err(|error| {
        increment_counter!(state.metrics, invalid_identity_unregister_jwt);
        info!(
            "Failure - Unregister identity with jwt: {:?}, error: {:?}",
            payload.id_auth, error
        );
        error
    })?;

    let claims: UnregisterIdentityKeyClaims = jwt.claims;
    let account = extract_did_data(&claims.pkh, DID_METHOD_PKH)?;
    let identity_key = extract_did_data(&claims.iss, DID_METHOD_KEY)?;

    let params = UnregisterIdentityParams {
        account: account.to_string(),
        identity_key: identity_key.to_string(),
    };

    params.validate().map_err(|error| {
        info!(
            "Failure - Unregister identity with jwt: {:?}, error: {:?}",
            payload.id_auth, error
        );
        error
    })?;

    state
        .keys_persitent_storage
        .remove_identity_key(&params.account, &params.identity_key)
        .await
        .map_err(|error| {
            warn!(
                "Failure - Unregister identity with jwt: {:?}, error: {:?}",
                payload.id_auth, error
            );
            error
        })?;

    info!(
        "Success - Unregister identity with jwt: {:?}",
        payload.id_auth
    );
    increment_counter!(state.metrics, identity_unregister);

    Ok(Response::default())
}
