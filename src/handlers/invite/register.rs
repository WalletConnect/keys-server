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
pub struct RegisterInviteKeyPayload {
    #[serde(rename = "idAuth")]
    id_auth: String,
}

#[derive(Validate)]
pub struct RegisterInviteKeyParams {
    #[validate(custom = "validate_caip10_account")]
    account: String,
    invite_key: String,
}

/// Registers invite key for given account.
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterInviteKeyPayload>,
) -> error::Result<Response> {
    // Errors with invalid jwt claims
    let jwt = Jwt::<InviteKeyClaims>::new(&payload.id_auth);
    jwt.verify()?;

    let claims: InviteKeyClaims = jwt.claims;
    let account = extract_did_data(&claims.pkh, DID_METHOD_PKH)?;

    let params = RegisterInviteKeyParams {
        account: account.to_string(),
        invite_key: claims.sub,
    };
    params.validate()?;

    state
        .keys_persitent_storage
        .upsert_invite_key(&params.account, &params.invite_key)
        .await?;

    Ok(Response::default())
}
