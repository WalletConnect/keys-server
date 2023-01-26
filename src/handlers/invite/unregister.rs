use {
    super::{
        super::{validate_caip10_account, Response},
        InviteKeyClaims,
    },
    crate::{
        auth::{
            did::{extract_did_data, DID_METHOD_PKH},
            jwt::Jwt,
        },
        error::{self},
        state::AppState,
    },
    axum::extract::{Json, State},
    serde::Deserialize,
    std::sync::Arc,
    validator::Validate,
};

#[derive(Deserialize)]
pub struct UnregisterInviteKeyPayload {
    #[serde(rename = "idAuth")]
    id_auth: String,
}

#[derive(Validate)]
pub struct UnregisterInviteKeyParams {
    #[validate(custom = "validate_caip10_account")]
    account: String,
}

/// Unsets invite key for given account.
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UnregisterInviteKeyPayload>,
) -> error::Result<Response> {
    // Errors with invalid jwt claims
    let jwt = Jwt::<InviteKeyClaims>::new(&payload.id_auth)?;
    jwt.verify()?;

    let claims: InviteKeyClaims = jwt.claims;
    let account = extract_did_data(&claims.pkh, DID_METHOD_PKH)?;

    let params = UnregisterInviteKeyParams {
        account: account.to_string(),
    };
    params.validate()?;

    state
        .keys_persitent_storage
        .remove_invite_key(&params.account)
        .await?;

    Ok(Response::default())
}
