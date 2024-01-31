use {
    super::super::{validate_caip10_account, validate_identity_key, Response},
    crate::{
        error::{self},
        increment_counter,
        log::prelude::{info, warn},
        state::AppState,
    },
    axum::extract::{Json, State},
    relay_rpc::auth::cacao::Cacao,
    serde::Deserialize,
    std::sync::Arc,
    validator::Validate,
};

#[derive(Deserialize)]
pub struct RegisterIdentityPayload {
    pub cacao: Cacao,
}

#[derive(Validate, Debug)]
pub struct RegisterIdentityParams {
    #[validate(custom = "validate_caip10_account")]
    account: String,
    #[validate(custom = "validate_identity_key")]
    identity_key: String,
    cacao: Cacao,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterIdentityPayload>,
) -> error::Result<Response> {
    let cacao = payload.cacao.clone();

    info!(
        "Handling - Register identity with cacao: {:?}",
        payload.cacao
    );

    cacao.verify(&state.provider).await.map_err(|error| {
        increment_counter!(state.metrics, invalid_identity_register_cacao);
        info!(
            "Failure - Register identity with cacao: {:?}, error: {:?}",
            payload.cacao, error
        );
        error
    })?;

    let identity_key = cacao.p.identity_key()?;
    let account = cacao.p.caip_10_address()?;
    let params = RegisterIdentityParams {
        account,
        identity_key,
        cacao,
    };

    params.validate().map_err(|error| {
        info!(
            "Failure - Register identity with cacao: {:?}, error: {:?}",
            payload.cacao, error
        );
        error
    })?;

    state
        .keys_persitent_storage
        .create_account_if_not_exists_and_add_identity_key(
            &params.account,
            &params.identity_key,
            &params.cacao,
        )
        .await
        .map_err(|error| {
            warn!(
                "Failure - Register identity with cacao: {:?}, error: {:?}",
                payload.cacao, error
            );
            error
        })?;

    info!(
        "Success - Register identity with cacao: {:?}",
        payload.cacao
    );
    increment_counter!(state.metrics, identity_register);

    Ok(Response::default())
}
