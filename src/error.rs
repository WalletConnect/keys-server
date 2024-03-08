use {
    crate::{auth, handlers::ResponseError, stores::StoreError},
    axum::response::{IntoResponse, Response},
    hyper::StatusCode,
    relay_rpc::auth::cacao::CacaoError,
    tracing::{error, info, warn},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Envy(#[from] envy::Error),

    #[error(transparent)]
    Trace(#[from] opentelemetry::trace::TraceError),

    #[error(transparent)]
    Metrics(#[from] opentelemetry::metrics::MetricsError),

    #[error(transparent)]
    Prometheus(#[from] prometheus_core::Error),

    #[error(transparent)]
    Database(#[from] wither::mongodb::error::Error),

    #[error(transparent)]
    Store(#[from] crate::stores::StoreError),

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    JwtVerification(#[from] auth::jwt::JwtError),

    #[error(transparent)]
    Did(#[from] auth::did::DidError),

    #[error(transparent)]
    Cacao(#[from] CacaoError),

    #[error("Blockchain API error: {0}")]
    BlockchainApi(blockchain_api::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        info!("Responding with error: {self:?}");
        let response = match &self {
            Error::Database(e) => crate::handlers::Response::new_failure(
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseError {
                    name: "mongodb".to_string(),
                    message: e.to_string(),
                },
            ),
            Error::Store(e) => match e {
                StoreError::Database(e) => crate::handlers::Response::new_failure(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseError {
                        name: "mongodb".to_string(),
                        message: e.to_string(),
                    },
                ),
                StoreError::NotFound(entity, id) => crate::handlers::Response::new_failure(
                    StatusCode::NOT_FOUND,
                    ResponseError {
                        name: format!("{} not found", &entity),
                        message: format!("Cannot find {} with specified identifier {}", entity, id),
                    },
                ),
            },
            Error::Validation(e) => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                ResponseError {
                    name: "validation".to_string(),
                    message: e.to_string(),
                }),
            Error::JwtVerification(e) => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                ResponseError {
                    name: "jwt_verification".to_string(),
                    message: e.to_string(),
                }),
            Error::Did(e) => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                ResponseError {
                    name: "did".to_string(),
                    message: e.to_string(),
                }),
            Error::Cacao(e) => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                ResponseError {
                    name: "cacao".to_string(),
                    message: e.to_string(),
                }),
            _ => crate::handlers::Response::new_failure(
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseError {
                    name: "unknown_error".to_string(),
                    message: "This error should not have occurred. Please file an issue at: https://github.com/walletconnect/keys-server".to_string(),
                }
            ),
        }.into_response();

        if response.status().is_client_error() {
            warn!("HTTP Client Error: {self:?}");
        }

        if response.status().is_server_error() {
            error!("HTTP Server Error: {self:?}");
        }

        response
    }
}
