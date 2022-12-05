#![feature(assert_matches)]

use {
    crate::storage::memory::MemoryStorage,
    axum::{
        error_handling::HandleErrorLayer,
        extract::Extension,
        http::StatusCode,
        response::IntoResponse,
        routing::{delete, get, post},
        Router,
    },
    http::{HeaderValue, Method},
    state::State,
    std::{
        borrow::Cow,
        net::SocketAddr,
        sync::Arc,
        time::Duration,
    },
    tower::{BoxError, ServiceBuilder},
    tower_http::{cors::CorsLayer, trace::TraceLayer},
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

mod handler;
mod state;
mod storage;

type SharedState = Arc<State<MemoryStorage>>;

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
        .route("/health", get(handler::health))
        .route("/register", post(handler::register))
        .route("/resolve", get(handler::resolve))
        .route("/remove/:key", delete(handler::remove_key))
        .route(
            "/keys",
            get(handler::count_accounts).delete(handler::delete_all_keys),
        )
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
