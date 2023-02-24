use {
    super::{
        super::{validate_caip10_account, validate_identity_key, Response},
        IdentityKeyClaims,
    },
    crate::{
        auth::{
            did::{extract_did_data, DID_METHOD_KEY, DID_METHOD_PKH},
            jwt::Jwt,
        },
        error,
        state::AppState,
    },
    axum::{extract::State, Json},
    serde::Deserialize,
    std::sync::Arc,
    validator::Validate,
};

#[derive(Deserialize)]
pub struct UnregisterIdentityPayload {
    #[serde(rename = "idAuth")]
    id_auth: String,
}

#[derive(Validate)]
pub struct UnregisterIdentityParams {
    #[validate(custom = "validate_caip10_account")]
    account: String,
    #[validate(custom = "validate_identity_key")]
    identity_key: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UnregisterIdentityPayload>,
) -> error::Result<Response> {
    let jwt = Jwt::<IdentityKeyClaims>::new(&payload.id_auth)?;
    jwt.verify()?;

    let claims: IdentityKeyClaims = jwt.claims;
    let account = extract_did_data(&claims.pkh, DID_METHOD_PKH)?;
    let identity_key = extract_did_data(&claims.iss, DID_METHOD_KEY)?;

    let params = UnregisterIdentityParams {
        account: account.to_string(),
        identity_key: identity_key.to_string(),
    };
    params.validate()?;

    state
        .keys_persitent_storage
        .remove_identity_key(&params.account, &params.identity_key)
        .await?;

    Ok(Response::default())
}
