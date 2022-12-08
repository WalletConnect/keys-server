use {
    super::Response,
    crate::{error, state::AppState},
    axum::extract::{Path, State},
    http::StatusCode,
    std::sync::Arc,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(account): Path<String>,
) -> error::Result<Response> {
    let keys = state.keys_persitent_storage.remove(&account).await?;

    Ok(Response::new_success_with_value(
        StatusCode::OK,
        keys.into(),
    ))
}
