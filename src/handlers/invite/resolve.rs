use {
    super::super::{validate_caip10_account, Response},
    crate::{error, state::AppState},
    axum::extract::{Query, State},
    http::StatusCode,
    serde::{Deserialize, Serialize},
    serde_json::{json, Value},
    std::sync::Arc,
    validator::Validate,
};

#[derive(Deserialize)]
pub struct ResolveInvitePayload {
    account: String,
}

#[derive(Validate)]
pub struct ResolveInviteParams {
    #[validate(custom = "validate_caip10_account")]
    account: String,
}

#[derive(Serialize)]
pub struct ResolveInviteResponse {
    #[serde(rename = "inviteKey")]
    invite_key: String,
}

impl From<ResolveInviteResponse> for Value {
    fn from(response: ResolveInviteResponse) -> Self {
        json!(response)
    }
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(payload): Query<ResolveInvitePayload>,
) -> error::Result<Response> {
    let params = ResolveInviteParams {
        account: payload.account,
    };
    params.validate()?;

    let invite_key = state
        .keys_persitent_storage
        .retrieve_invite_key(&params.account)
        .await?;

    let response = ResolveInviteResponse { invite_key };

    Ok(Response::new_success_with_value(
        StatusCode::OK,
        response.into(),
    ))
}
