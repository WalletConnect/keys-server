use {
    crate::{config::Configuration, stores::keys::KeysPersistentStorageArc},
    build_info::BuildInfo,
    opentelemetry::{metrics::UpDownCounter, sdk::trace::Tracer},
    tracing_subscriber::prelude::*,
};

#[derive(Clone)]
pub struct Metrics {
    pub example: UpDownCounter<i64>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Configuration,
    pub build_info: BuildInfo,
    pub metrics: Option<Metrics>,
    pub keys_persitent_storage: KeysPersistentStorageArc,
}

build_info::build_info!(fn build_info);

impl AppState {
    pub fn new(
        config: Configuration,
        keys_persitent_storage: KeysPersistentStorageArc,
    ) -> crate::error::Result<AppState> {
        let build_info: &BuildInfo = build_info();

        Ok(AppState {
            config,
            build_info: build_info.clone(),
            metrics: None,
            keys_persitent_storage,
        })
    }

    pub fn set_telemetry(&mut self, tracer: Tracer, metrics: Metrics) {
        let otel_tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(otel_tracing_layer)
            .init();

        self.metrics = Some(metrics);
    }
}
