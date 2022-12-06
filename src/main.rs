use {
    crate::{config::Config, error::KeyserverError, storage::Storage},
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
    std::{borrow::Cow, net::SocketAddr, sync::Arc, time::Duration},
    tower::{BoxError, ServiceBuilder},
    tower_http::{cors::CorsLayer, trace::TraceLayer},
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

mod config;
mod error;
mod handler;
mod state;
mod storage;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

type SharedState = Arc<State>;

#[tokio::main]
async fn main() -> Result<(), KeyserverError> {
    let config = Config::from_env()?;
    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port).parse()?;

    let concurrency_limit = config.server.concurrency_limit;
    let timeout = config.server.timeout;

    let storage = init_storage(&config).await?;
    let state = Arc::new(State { config, storage });

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
                .concurrency_limit(concurrency_limit)
                .timeout(Duration::from_secs(timeout))
                .layer(TraceLayer::new_for_http())
                .layer(Extension(state))
                .into_inner(),
        )
        .layer(
            CorsLayer::new()
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST]),
        );

    // Run our app with hyper
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn init_storage(config: &Config) -> Result<Box<dyn Storage>, KeyserverError> {
    if let Some(addr) = &config.mongo.addr {
        tracing::info!("initializing mongodb store at {}", addr);
        let st = storage::mongo::MongoStorage::new(addr, &config.mongo.database).await?;
        Ok(Box::new(st))
    } else {
        tracing::info!("initializing in-memory store");
        #[allow(clippy::box_default)]
        Ok(Box::new(storage::memory::MemoryStorage::default()))
    }
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
