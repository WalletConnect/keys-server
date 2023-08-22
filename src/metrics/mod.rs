use {
    crate::error::{Error, Result},
    opentelemetry::{
        metrics::Counter,
        sdk::{
            self,
            export::metrics::aggregation,
            metrics::{processors, selectors},
            Resource,
        },
    },
    opentelemetry_prometheus::PrometheusExporter,
    prometheus_core::TextEncoder,
};

#[derive(Clone)]
pub struct Metrics {
    pub prometheus_exporter: PrometheusExporter,

    // Invite counters
    pub invite_register: Counter<u64>,
    pub invite_resolved: Counter<u64>,
    pub invite_unregister: Counter<u64>,

    // Identity counters
    pub identity_register: Counter<u64>,
    pub identity_resolved: Counter<u64>,
    pub identity_unregister: Counter<u64>,

    // Handlers params counters
    pub invalid_identity_register_cacao: Counter<u64>,
    pub invalid_identity_unregister_jwt: Counter<u64>,
    pub invalid_invite_register_jwt: Counter<u64>,
    pub invalid_invite_unregister_jwt: Counter<u64>,
}

impl Metrics {
    pub fn new(resource: Resource) -> Result<Self> {
        let controller = sdk::metrics::controllers::basic(
            processors::factory(
                selectors::simple::histogram(vec![]),
                aggregation::cumulative_temporality_selector(),
            )
            .with_memory(true),
        )
        .with_resource(resource)
        .build();

        let prometheus_exporter = opentelemetry_prometheus::exporter(controller).init();

        let meter = prometheus_exporter.meter_provider().unwrap();

        opentelemetry::global::set_meter_provider(meter);

        let meter = opentelemetry::global::meter("keyserver");

        let invite_register = meter
            .u64_counter("invite_register")
            .with_description("The number of invite keys registered")
            .init();

        let invite_resolved = meter
            .u64_counter("invite_resolved")
            .with_description("The number of invite keys resolved")
            .init();

        let invite_unregister = meter
            .u64_counter("invite_unregister")
            .with_description("The number of invite keys unregistered")
            .init();

        let identity_register = meter
            .u64_counter("identity_register")
            .with_description("The number of identity keys registered")
            .init();

        let identity_resolved = meter
            .u64_counter("identity_resolved")
            .with_description("The number of identity keys resolved")
            .init();

        let identity_unregister = meter
            .u64_counter("identity_unregister")
            .with_description("The number of identity keys unregistered")
            .init();

        let invalid_identity_register_cacao = meter
            .u64_counter("invalid_identity_register_cacao")
            .with_description("The number of invalid cacaos received for registering an identity")
            .init();

        let invalid_identity_unregister_jwt = meter
            .u64_counter("invalid_identity_unregister_jwt")
            .with_description("The number of invalid jwt received for unregistering an identity")
            .init();

        let invalid_invite_register_jwt = meter
            .u64_counter("invalid_invite_register_jwt")
            .with_description("The number of invalid jwt received for registering an invite")
            .init();

        let invalid_invite_unregister_jwt = meter
            .u64_counter("invalid_invite_unregister_jwt")
            .with_description("The number of invalid jwt received for unregistering an invite")
            .init();

        Ok(Metrics {
            prometheus_exporter,
            invite_register,
            invite_resolved,
            invite_unregister,
            identity_register,
            identity_resolved,
            identity_unregister,
            invalid_identity_register_cacao,
            invalid_identity_unregister_jwt,
            invalid_invite_register_jwt,
            invalid_invite_unregister_jwt,
        })
    }

    pub fn export(&self) -> Result<String> {
        let data = self.prometheus_exporter.registry().gather();
        TextEncoder::new()
            .encode_to_string(&data)
            .map_err(Error::Prometheus)
    }
}
