use {
    super::Response,
    crate::{error, state::AppState, stores::keys::Keys},
    axum::extract::{Json, Path, Query, State},
    http::StatusCode,
    serde::Deserialize,
    std::sync::Arc,
    validator::Validate,
};

#[derive(Validate)]
pub struct ExistsIdentityKeyParams {
    #[validate(length(min = 64))]
    identity_key: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    identity_key: Path<String>,
) -> error::Result<Response> {
    let params = ExistsIdentityKeyParams {
        identity_key: identity_key.to_string(),
    };

    params.validate()?;

    let keys = state
        .keys_persitent_storage
        .exists_identity_key(&params.identity_key)
        .await?;

    Ok(Response::new_success_with_value(
        StatusCode::OK,
        keys.into(),
    ))
}
