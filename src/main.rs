use axum::{
    error_handling::HandleErrorLayer,
    extract::{Extension, Json, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use http::{HeaderValue, Method};
use serde::{Deserialize, Serialize};
use std::{
    assert_eq,
    borrow::Cow,
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_key_value_store=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/register", post(register))
        .route("/health", get(health))
        .route("/resolve", get(resolve))
        .route("/remove/:key", delete(remove_key))
        .route("/keys", get(count_accounts).delete(delete_all_keys)) //
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(Extension(SharedState::default()))
                .into_inner(),
        )
        .layer(
            CorsLayer::new()
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST]),
        );

    // Run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type SharedState = Arc<RwLock<State>>;

#[derive(Default)]
struct State {
    db: HashMap<String, String>,
}

#[derive(Deserialize)]
struct ResolveParams {
    account: String,
}

#[derive(Deserialize)]
struct DeleteParams {
    password: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Account {
    account: String,
    publicKey: String,
}

async fn delete_all_keys(Extension(state): Extension<SharedState>, params: Query<DeleteParams>) {
    let params = params.0;
    assert_eq!(
        &params.password,
        "f9132ad791031307dcc9723809c87ff734b485820ec5cae21059c3711765207a"
    );
    state.write().unwrap().db.clear();
}

async fn remove_key(Path(key): Path<String>, Extension(state): Extension<SharedState>) {
    state.write().unwrap().db.remove(&key);
}

async fn health() -> StatusCode {StatusCode::OK}

async fn resolve(
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

async fn register(Json(payload): Json<Account>, Extension(state): Extension<SharedState>) {
    state
        .write()
        .unwrap()
        .db
        .insert(payload.account, payload.publicKey);
}

async fn count_accounts(Extension(state): Extension<SharedState>) -> String {
    state.read().unwrap().db.len().to_string()
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}
