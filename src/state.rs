use {
    crate::{config::Configuration, metrics::Metrics, stores::keys::KeysPersistentStorageArc},
    build_info::BuildInfo,
    relay_rpc::auth::cacao::signature::eip1271::blockchain_api::BlockchainApiProvider,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Configuration,
    pub build_info: BuildInfo,
    pub metrics: Option<Metrics>,
    pub keys_persitent_storage: KeysPersistentStorageArc,
    pub provider: BlockchainApiProvider,
}

build_info::build_info!(fn build_info);

impl AppState {
    pub fn new(
        config: Configuration,
        keys_persitent_storage: KeysPersistentStorageArc,
        provider: BlockchainApiProvider,
    ) -> crate::error::Result<AppState> {
        let build_info: &BuildInfo = build_info();

        Ok(AppState {
            config,
            build_info: build_info.clone(),
            metrics: None,
            keys_persitent_storage,
            provider,
        })
    }

    pub fn set_metrics(&mut self, metrics: Metrics) {
        self.metrics = Some(metrics);
    }
}
