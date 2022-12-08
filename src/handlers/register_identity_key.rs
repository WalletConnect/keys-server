use {
    super::Response,
    crate::{
        error::{self, Error},
        models::keys::IdentityKey,
        state::AppState,
        stores::keys::Keys,
    },
    axum::extract::{Json, Path, State},
    http::StatusCode,
    serde::Deserialize,
    std::sync::Arc,
    validator::Validate,
};

#[derive(Deserialize)]
pub struct AddIdentityKeyPayload {
    #[serde(rename = "identityKey")]
    identity_key: String,
}

#[derive(Validate)]
pub struct AddIdentityKeyParams {
    // note(Szymon): Some validation might be usefull here.
    #[validate(length(min = 10))]
    account: String,
    #[validate(length(min = 64))]
    identity_key: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(account): Path<String>,
    Json(payload): Json<AddIdentityKeyPayload>,
) -> error::Result<Response> {
    let params = AddIdentityKeyParams {
        account,
        identity_key: payload.identity_key,
    };

    params.validate()?;

    state
        .keys_persitent_storage
        .create_account_if_not_exists_and_add_identity_key(
            &params.account,
            &params.identity_key.into(),
        )
        .await?;

    Ok(Response::default())
}
