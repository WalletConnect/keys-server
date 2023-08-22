use {
    crate::{error::Result, state::AppState},
    axum::{extract::State, http::StatusCode},
    std::sync::Arc,
};

pub async fn handler(State(state): State<Arc<AppState>>) -> Result<(StatusCode, String)> {
    if let Some(metrics) = &state.metrics {
        let exported = metrics.export()?;

        Ok((StatusCode::OK, exported))
    } else {
        // No Metrics!
        Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Metrics not enabled.".to_string(),
        ))
    }
}
