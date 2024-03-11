use {
    crate::{config::Configuration, error::Error, log::prelude::*, state::AppState},
    aws_config::meta::region::RegionProviderChain,
    aws_sdk_s3::{config::Region, Client as S3Client},
    axum::{
        routing::{get, post},
        Router,
    },
    blockchain_api::BlockchainApiProvider,
    http::{HeaderValue, Method},
    opentelemetry::{sdk::Resource, KeyValue},
    std::{net::SocketAddr, sync::Arc},
    stores::keys::MongoPersistentStorage,
    tokio::{select, sync::broadcast},
    tower::ServiceBuilder,
    tower_http::{
        cors::CorsLayer,
        request_id::MakeRequestUuid,
        trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
        ServiceBuilderExt,
    },
    tracing::Level,
    wc::geoip::{
        block::{middleware::GeoBlockLayer, BlockingPolicy},
        MaxMindResolver,
    },
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

    let s3_client = get_s3_client(&config).await;
    let geoip_resolver = get_geoip_resolver(&config, &s3_client).await;

    let provider = if let Some(blockchain_api_endpoint) = &config.blockchain_api_endpoint {
        Some(
            BlockchainApiProvider::new(
                config.project_id.clone(),
                blockchain_api_endpoint
                    .parse()
                    .expect("Error parsing blockchain_api_endpoint"),
            )
            .await
            .map_err(Error::BlockchainApi)?,
        )
    } else {
        None
    };
    let mut state = AppState::new(config, keys_persistent_storage, provider)?;

    if let Some(prometheus_port) = state.config.telemetry_prometheus_port {
        info!("Telemetry is enabled on port {}", prometheus_port);
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

    let global_middleware = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(true),
                )
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .include_headers(true),
                ),
        )
        .propagate_x_request_id();

    let cors_layer = CorsLayer::new()
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE]);

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
        .layer(cors_layer);
    let app = if let Some(resolver) = geoip_resolver {
        app.layer(GeoBlockLayer::new(
            resolver.clone(),
            state_arc.config.blocked_countries.clone(),
            BlockingPolicy::AllowAll,
        ))
    } else {
        app
    };
    let app = app.with_state(state_arc.clone());

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

async fn get_s3_client(config: &Configuration) -> S3Client {
    let region_provider = RegionProviderChain::first_try(Region::new("eu-central-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    let aws_config = match &config.s3_endpoint {
        Some(s3_endpoint) => {
            info!(%s3_endpoint, "initializing analytics with custom s3 endpoint");

            aws_sdk_s3::config::Builder::from(&shared_config)
                .endpoint_url(s3_endpoint)
                .build()
        }
        _ => aws_sdk_s3::config::Builder::from(&shared_config).build(),
    };

    S3Client::from_conf(aws_config)
}

async fn get_geoip_resolver(
    config: &Configuration,
    s3_client: &S3Client,
) -> Option<Arc<MaxMindResolver>> {
    match (&config.geoip_db_bucket, &config.geoip_db_key) {
        (Some(bucket), Some(key)) => {
            info!(%bucket, %key, "initializing geoip database from aws s3");

            Some(Arc::new(
                MaxMindResolver::from_aws_s3(s3_client, bucket, key)
                    .await
                    .expect("failed to load geoip resolver"),
            ))
        }
        _ => {
            info!("analytics geoip lookup is disabled");
            None
        }
    }
}
