use {
    crate::{
        handlers::{ErrorField, ErrorLocation, ResponseError},
        stores::StoreError,
    },
    axum::response::{IntoResponse, Response},
    hyper::StatusCode,
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
    Database(#[from] wither::mongodb::error::Error),

    #[error(transparent)]
    Store(#[from] crate::stores::StoreError),

    #[error("the `{0}` field must not be empty")]
    EmptyField(String),

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Database(e) => crate::handlers::Response::new_failure(
                StatusCode::INTERNAL_SERVER_ERROR,
                vec![ResponseError {
                    name: "mongodb".to_string(),
                    message: e.to_string(),
                }],
                vec![],
            ),
            Error::Store(e) => match e {
                StoreError::Database(e) => crate::handlers::Response::new_failure(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    vec![ResponseError {
                        name: "mongodb".to_string(),
                        message: e.to_string(),
                    }],
                    vec![],
                ),
                StoreError::NotFound(entity, id) => crate::handlers::Response::new_failure(
                    StatusCode::NOT_FOUND,
                    vec![ResponseError {
                        name: format!("{} not found", &entity),
                        message: format!("Cannot find {} with specified identifier {}", entity, id),
                    }],
                    vec![],
                ),
            },
            Error::Validation(e) => crate::handlers::Response::new_failure(
                StatusCode::BAD_REQUEST,
                vec![ResponseError {
                    name: "validation".to_string(),
                    message: e.to_string(),
                }],
                vec![]),
            _ => crate::handlers::Response::new_failure(StatusCode::INTERNAL_SERVER_ERROR, vec![
                ResponseError {
                    name: "unknown_error".to_string(),
                    message: "This error should not have occurred. Please file an issue at: https://github.com/walletconnect/keyserver".to_string(),
                }
            ], vec![])
        }.into_response()
    }
}
