use {
    super::Response,
    crate::{
        error::{self, Error},
        state::AppState,
        stores::keys::Keys,
    },
    axum::extract::{Json, Path, State},
    http::StatusCode,
    serde::Deserialize,
    std::sync::Arc,
    validator::Validate,
};

#[derive(Deserialize)]
pub struct RegisterProposalKeyPayload {
    #[serde(rename = "proposalEncryptionKey")]
    proposal_encryption_key: String,
}

#[derive(Validate)]
pub struct RegisterProposalKeyParams {
    #[validate(length(min = 1))]
    account: String,
    #[validate(length(min = 64))]
    proposal_encryption_key: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(account): Path<String>,
    Json(payload): Json<RegisterProposalKeyPayload>,
) -> error::Result<Response> {
    let params = RegisterProposalKeyParams {
        account,
        proposal_encryption_key: payload.proposal_encryption_key,
    };
    params.validate()?;

    state
        .keys_persitent_storage
        .upsert_proposal_encryption_key(&params.account, &params.proposal_encryption_key)
        .await?;

    Ok(Response::default())
}
