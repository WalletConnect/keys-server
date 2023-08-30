use {
    crate::{config::Configuration, state::AppState},
    ::log::{info, warn},
    axum::{
        routing::{get, post},
        Router,
    },
    http::{HeaderValue, Method},
    opentelemetry::{sdk::Resource, KeyValue},
    std::{net::SocketAddr, sync::Arc},
    stores::keys::MongoPersistentStorage,
    tokio::{select, sync::broadcast},
    tower::ServiceBuilder,
    tower_http::{
        cors::CorsLayer,
        trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    },
    tracing::Level,
};

pub mod auth;
pub mod config;
pub mod error;
pub mod handlers;
pub mod log;
pub mod macros;
pub mod metrics;
pub mod state;
pub mod stores;

pub async fn bootstrap(
    mut shutdown: broadcast::Receiver<()>,
    config: Configuration,
) -> error::Result<()> {
    let keys_persistent_storage: Arc<MongoPersistentStorage> =
        Arc::new(MongoPersistentStorage::new(&config).await?);

    let mut state = AppState::new(config, keys_persistent_storage)?;

    if state.config.telemetry_prometheus_port.is_some() {
        state.set_metrics(metrics::Metrics::new(Resource::new(vec![
            KeyValue::new("service_name", state.build_info.crate_info.name.clone()),
            KeyValue::new(
                "service_version",
                state.build_info.crate_info.version.clone().to_string(),
            ),
        ]))?);
    } else {
        warn!("Telemetry is disabled")
    }

    let port = state.config.port;
    let private_port = state.config.telemetry_prometheus_port.unwrap_or(8081);

    let state_arc = Arc::new(state);

    let global_middleware = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .include_headers(true),
            ),
    );

    let cors_layer = CorsLayer::new()
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST]);

    let app = Router::new()
        .route("/health", get(handlers::health::handler))
        .route(
            "/identity",
            get(handlers::identity::resolve::handler)
                .post(handlers::identity::register::handler)
                .delete(handlers::identity::unregister::handler),
        )
        .route(
            "/invite",
            post(handlers::invite::register::handler)
                .delete(handlers::invite::unregister::handler)
                .get(handlers::invite::resolve::handler),
        )
        .layer(global_middleware)
        .layer(cors_layer)
        .with_state(state_arc.clone());

    let private_app = Router::new()
        .route("/metrics", get(handlers::metrics::handler))
        .with_state(state_arc);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let private_addr = SocketAddr::from(([0, 0, 0, 0], private_port));

    select! {
        _ = axum::Server::bind(&addr).serve(app.into_make_service()) => info!("Server terminating"),
        _ = axum::Server::bind(&private_addr).serve(private_app.into_make_service()) => info!("Internal Server terminating"),
        _ = shutdown.recv() => info!("Shutdown signal received, killing servers"),
    }

    Ok(())
}
