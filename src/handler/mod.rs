use {
    crate::SharedState,
    axum::{
        extract::{Path, Query},
        Extension,
        Json,
    },
    http::StatusCode,
    serde::{Deserialize, Serialize},
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

pub async fn register(Json(payload): Json<Account>, Extension(state): Extension<SharedState>) {
    state
        .write()
        .unwrap()
        .db
        .insert(payload.account, payload.publicKey);
}

pub async fn resolve(
    params: Query<ResolveParams>,
    Extension(state): Extension<SharedState>,
) -> Result<Json<Account>, StatusCode> {
    let db = &state.read().unwrap().db;
    let params = params.0;

    if let Some(value) = db.get(&params.account) {
        Ok(Json(Account {
            account: params.account,
            publicKey: value.clone(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn remove_key(Path(key): Path<String>, Extension(state): Extension<SharedState>) {
    state.write().unwrap().db.remove(&key);
}

pub async fn count_accounts(Extension(state): Extension<SharedState>) -> String {
    state.read().unwrap().db.len().to_string()
}

pub async fn delete_all_keys(
    Extension(state): Extension<SharedState>,
    params: Query<DeleteParams>,
) {
    let params = params.0;
    assert_eq!(
        &params.password,
        "f9132ad791031307dcc9723809c87ff734b485820ec5cae21059c3711765207a"
    );
    state.write().unwrap().db.clear();
}
