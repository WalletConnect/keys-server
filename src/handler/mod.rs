use {
    crate::SharedState,
    axum::{
        extract::{Path, Query},
        Extension,
        Json,
    },
    http::StatusCode,
    serde::{Deserialize, Serialize},
    tracing::error,
};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Account {
    pub account: String,
    pub publicKey: String,
}

#[derive(Deserialize)]
pub struct DeleteParams {
    pub password: String,
}

#[derive(Deserialize)]
pub struct ResolveParams {
    pub account: String,
}

pub async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn register(
    Json(payload): Json<Account>,
    Extension(state): Extension<SharedState>,
) -> StatusCode {
    state
        .storage
        .set(&payload.account, &payload.publicKey)
        .await
        .map(|_| StatusCode::CREATED)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn resolve(
    params: Query<ResolveParams>,
    Extension(state): Extension<SharedState>,
) -> Result<Json<Account>, StatusCode> {
    let params = params.0;
    match state.storage.get(&params.account).await {
        Ok(Some(value)) => Ok(Json(Account {
            account: params.account,
            publicKey: value,
        })),

        Ok(None) => Err(StatusCode::NOT_FOUND),

        Err(err) => {
            error!(?err, "failed to query data store");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn remove_key(
    Path(key): Path<String>,
    Extension(state): Extension<SharedState>,
) -> StatusCode {
    state
        .storage
        .remove(&key)
        .await
        .map(|_| StatusCode::OK)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn count_accounts(
    Extension(state): Extension<SharedState>,
) -> Result<String, StatusCode> {
    state
        .storage
        .count()
        .await
        .map(|len| len.to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_all_keys(
    Extension(state): Extension<SharedState>,
    params: Query<DeleteParams>,
) -> StatusCode {
    let params = params.0;
    assert_eq!(
        &params.password,
        "f9132ad791031307dcc9723809c87ff734b485820ec5cae21059c3711765207a"
    );

    state
        .storage
        .clear()
        .await
        .map(|_| StatusCode::OK)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
