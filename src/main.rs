use {
    http::{HeaderValue, Method},
    stores::keys::MongoPersistentStorage,
    tower_http::{
        cors::CorsLayer,
        trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    },
    tracing::Level,
};

mod auth;
mod config;
mod error;
mod handlers;
mod state;
mod stores;

use {
    crate::{
        config::Configuration,
        state::{AppState, Metrics},
    },
    axum::{
        routing::{get, post},
        Router,
    },
    dotenv::dotenv,
    opentelemetry::{
        sdk::{
            metrics::selectors,
            trace::{self, IdGenerator, Sampler},
            Resource,
        },
        KeyValue,
    },
    opentelemetry_otlp::{Protocol, WithExportConfig},
    std::{net::SocketAddr, sync::Arc, time::Duration},
    tower::ServiceBuilder,
    tracing_subscriber::fmt::format::FmtSpan,
};

build_info::build_info!(fn build_info);

#[tokio::main]
async fn main() -> crate::error::Result<()> {
    dotenv().ok();

    let config = Configuration::new().expect("Failed to load config!");

    let keys_persitent_storage = Arc::new(MongoPersistentStorage::new(&config).await?);

    let mut state = AppState::new(config, keys_persitent_storage)?;

    // Telemetry
    if state.config.telemetry_enabled.unwrap_or(false) {
        let grpc_url = state
            .config
            .telemetry_grpc_url
            .clone()
            .unwrap_or_else(|| "http://localhost:4317".to_string());

        let tracing_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(grpc_url.clone())
            .with_timeout(Duration::from_secs(5))
            .with_protocol(Protocol::Grpc);

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(tracing_exporter)
            .with_trace_config(
                trace::config()
                    .with_sampler(Sampler::AlwaysOn)
                    .with_id_generator(IdGenerator::default())
                    .with_max_events_per_span(64)
                    .with_max_attributes_per_span(16)
                    .with_max_events_per_span(16)
                    .with_resource(Resource::new(vec![
                        KeyValue::new("service.name", state.build_info.crate_info.name.clone()),
                        KeyValue::new(
                            "service.version",
                            state.build_info.crate_info.version.clone().to_string(),
                        ),
                    ])),
            )
            .install_batch(opentelemetry::runtime::Tokio)?;

        let metrics_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(grpc_url)
            .with_timeout(Duration::from_secs(5))
            .with_protocol(Protocol::Grpc);

        let meter_provider = opentelemetry_otlp::new_pipeline()
            .metrics(tokio::spawn, opentelemetry::util::tokio_interval_stream)
            .with_exporter(metrics_exporter)
            .with_period(Duration::from_secs(3))
            .with_timeout(Duration::from_secs(10))
            .with_aggregator_selector(selectors::simple::Selector::Exact)
            .build()?;

        opentelemetry::global::set_meter_provider(meter_provider.provider());

        let meter = opentelemetry::global::meter("rust-http-starter");
        let example_counter = meter
            .i64_up_down_counter("example")
            .with_description("This is an example counter")
            .init();

        state.set_telemetry(
            tracer,
            Metrics {
                example: example_counter,
            },
        )
    } else {
        // Only log to console if telemetry disabled
        tracing_subscriber::fmt()
            .with_max_level(state.config.log_level())
            .with_span_events(FmtSpan::CLOSE)
            .init();
    }

    let port = state.config.port;

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

    // note (Szymon): routes will propably be changed with proper Keyserver Specs
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
        .with_state(state_arc);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
