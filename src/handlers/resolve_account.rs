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
pub struct RetrieveParams {
    // note(Szymon): Some validation might be usefull here.
    #[validate(length(min = 10))]
    account: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(account): Path<String>,
) -> error::Result<Response> {
    let params = RetrieveParams { account };
    params.validate()?;

    let keys = state
        .keys_persitent_storage
        .retrieve(&params.account)
        .await?;

    Ok(Response::new_success_with_value(
        StatusCode::OK,
        keys.into(),
    ))
}
