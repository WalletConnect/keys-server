use {
    super::super::Response,
    crate::{auth::cacao::Cacao, error, handlers::validate_identity_key, state::AppState},
    axum::extract::{Query, State},
    http::StatusCode,
    serde::{Deserialize, Serialize},
    serde_json::{json, Value},
    std::sync::Arc,
    validator::Validate,
};

#[derive(Deserialize)]
pub struct ResolveIdentityPayload {
    #[serde(rename = "publicKey")]
    public_key: String,
}

#[derive(Validate)]
pub struct ResolveIdentityParams {
    #[validate(custom = "validate_identity_key")]
    identity_key: String,
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

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(payload): Query<ResolveIdentityPayload>,
) -> error::Result<Response> {
    let params = ResolveIdentityParams {
        identity_key: payload.public_key,
    };

    params.validate()?;

    let cacao = state
        .keys_persitent_storage
        .get_cacao_by_identity_key(&params.identity_key)
        .await?;

    let response = ResolveIdentityResponse { cacao };

    Ok(Response::new_success_with_value(
        StatusCode::OK,
        response.into(),
    ))
}
