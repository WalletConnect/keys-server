use {
    super::super::Response,
    crate::{
        error,
        handlers::validate_identity_key,
        increment_counter,
        log::prelude::{info, warn},
        state::AppState,
    },
    axum::extract::{Query, State},
    http::StatusCode,
    relay_rpc::auth::cacao::Cacao,
    serde::{Deserialize, Serialize},
    serde_json::{json, Value},
    std::sync::Arc,
    tracing::instrument,
    validator::Validate,
};

#[derive(Deserialize, Debug, Validate)]
pub struct ResolveIdentityPayload {
    #[serde(rename = "publicKey")]
    #[validate(custom = "validate_identity_key")]
    public_key: String,
}

#[derive(Serialize)]
pub struct ResolveIdentityResponse {
    cacao: Cacao,
}

impl From<ResolveIdentityResponse> for Value {
    fn from(response: ResolveIdentityResponse) -> Self {
        json!(response)
    }
}

#[instrument(name = "resolve_handler", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ResolveIdentityPayload>,
) -> error::Result<Response> {
    info!("Handling - Resolve identity with params: {:?}", params);

    info!("Timing - Validating params - Start");
    params.validate().map_err(|error| {
        info!(
            "Failure - Resolve identity with params: {:?}, error: {:?}",
            params, error
        );
        error
    })?;
    info!("Timing - Validating params - End");

    info!("Timing - get_cacao_by_identity_key - Start");
    let cacao = state
        .keys_persitent_storage
        .get_cacao_by_identity_key(&params.public_key)
        .await
        .map_err(|error| {
            warn!(
                "Failure - Resolve identity with params: {:?}, error: {:?}",
                params, error
            );
            error
        })?;
    info!("Timing - get_cacao_by_identity_key - End");

    let response = ResolveIdentityResponse { cacao };

    info!("Success - Resolve identity with params: {:?}", params);
    increment_counter!(state.metrics, identity_resolved);
    info!("Incremented counter");

    Ok(Response::new_success_with_value(
        StatusCode::OK,
        response.into(),
    ))
}
