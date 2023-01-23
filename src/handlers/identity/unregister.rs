use {
    super::super::{validate_caip10_account, validate_identity_key, Response},
    crate::{auth::cacao::Cacao, error, state::AppState},
    axum::{extract::State, Json},
    serde::Deserialize,
    std::sync::Arc,
    validator::Validate,
};

#[derive(Deserialize)]
pub struct UnregisterIdentityPayload {
    pub cacao: Cacao,
}

#[derive(Validate)]
pub struct UnregisterIdentityParams {
    #[validate(custom = "validate_caip10_account")]
    account: String,
    #[validate(custom = "validate_identity_key")]
    identity_key: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UnregisterIdentityPayload>,
) -> error::Result<Response> {
    let cacao = payload.cacao;
    cacao.verify()?;

    let identity_key = cacao.p.identity_key()?;
    let account = cacao.p.caip_10_address()?;

    let params = UnregisterIdentityParams {
        account,
        identity_key,
    };

    params.validate()?;

    state
        .keys_persitent_storage
        .remove_identity_key(&params.account, &params.identity_key)
        .await?;

    Ok(Response::default())
}
