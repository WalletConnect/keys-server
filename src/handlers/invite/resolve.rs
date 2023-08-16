use {
    super::super::{validate_caip10_account, Response},
    crate::{
        error, increment_counter,
        log::prelude::{info, warn},
        state::AppState,
    },
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
    info!(
        "Handling - Resolve invite of account: {:?}",
        payload.account
    );

    let params = ResolveInviteParams {
        account: payload.account.clone(),
    };
    params.validate().map_err(|error| {
        info!(
            "Failure - Resolve invite of account: {:?}, error: {:?}",
            payload.account, error
        );
        error
    })?;

    let invite_key = state
        .keys_persitent_storage
        .retrieve_invite_key(&params.account)
        .await
        .map_err(|error| {
            warn!(
                "Failure - Resolve invite of account: {:?}, error: {:?}",
                payload.account, error
            );
            error
        })?;

    let response = ResolveInviteResponse { invite_key };

    info!("Success - Resolve invite of account: {:?}", payload.account);
    increment_counter!(state.metrics, invite_resolved);
    Ok(Response::new_success_with_value(
        StatusCode::OK,
        response.into(),
    ))
}
