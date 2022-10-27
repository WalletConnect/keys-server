use axum::{
    error_handling::HandleErrorLayer,
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use http::{HeaderValue, Method};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions, PgQueryResult};
use std::{
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

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");

    let app = Router::with_state(pool)
        .route("/register", post(register))
        .route("/resolve", get(resolve))
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
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

#[derive(Deserialize)]
struct ResolveParams {
    account: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
#[allow(non_snake_case)]
struct Account {
    account: String,
    publicKey: String,
}
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
    }
}

async fn resolve(
    params: Query<ResolveParams>,
    State(pool): State<PgPool>,
) -> Result<Account, Error> {
    Ok(sqlx::query_as::<sqlx::postgres::Postgres, Account>(
        "SELECT (1) FROM registry WHERE address = $1;",
    )
    .bind(&params.account)
    .fetch_one(&pool)
    .await?)
}

async fn register(State(pool): State<PgPool>, Json(payload): Json<Account>) -> Result<(), Error> {
    sqlx::query_as::<sqlx::postgres::Postgres, Account>("INSERT INTO registry VALUES ($1, $2) ON CONFLICT (address) DO UPDATE SET public_key = EXCLUDED.public_key RETURNING *;")
    .bind(payload.account)
    .bind(payload.publicKey)
    .fetch_one(&pool)
    .await?;

    Ok(())
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
