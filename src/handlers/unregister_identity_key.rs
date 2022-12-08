use {
    super::Response,
    crate::{error, state::AppState},
    axum::{
        extract::{Path, State},
        Json,
    },
    http::StatusCode,
    serde::Deserialize,
    std::sync::Arc,
    validator::Validate,
};

#[derive(Validate)]
pub struct RemoveIdentityKeyParams {
    // note(Szymon): Some validation might be usefull here.
    #[validate(length(min = 10))]
    account: String,
    #[validate(length(min = 64))]
    identity_key: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path((account, identity_key)): Path<(String, String)>,
) -> error::Result<Response> {
    let params = RemoveIdentityKeyParams {
        account,
        identity_key,
    };

    params.validate()?;

    state
        .keys_persitent_storage
        .remove_identity_key(&params.account, &params.identity_key.into())
        .await?;

    Ok(Response::default())
}
