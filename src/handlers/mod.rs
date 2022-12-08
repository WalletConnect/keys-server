use {
    axum::{response::IntoResponse, Json},
    hyper::StatusCode,
    serde_json::{json, Value},
};

pub mod exists_identity_key;
pub mod health;
pub mod register_identity_key;
pub mod register_proposal_encryption_key;
pub mod resolve_account;
pub mod unregister_account;
pub mod unregister_identity_key;

#[derive(serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorLocation {
    Body,
    Header,
    Path,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(serde::Serialize)]
pub struct ErrorField {
    pub field: String,
    pub description: String,
    pub location: Vec<ErrorLocation>,
}

#[derive(serde::Serialize)]
pub struct ResponseError {
    pub name: String,
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct Response {
    pub status: ResponseStatus,
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub errors: Option<Vec<ResponseError>>,
    pub fields: Option<Vec<ErrorField>>,
    pub value: Option<Value>,
}

impl Response {
    pub fn new_success_with_value(status: StatusCode, value: Value) -> Self {
        Response {
            status: ResponseStatus::Success,
            status_code: status,
            errors: None,
            fields: None,
            value: Some(value),
        }
    }

    pub fn new_success(status: StatusCode) -> Self {
        Response {
            status: ResponseStatus::Success,
            status_code: status,
            errors: None,
            fields: None,
            value: None,
        }
    }

    pub fn new_failure(
        status: StatusCode,
        errors: Vec<ResponseError>,
        fields: Vec<ErrorField>,
    ) -> Self {
        Response {
            status: ResponseStatus::Failure,
            status_code: status,
            errors: Some(errors),
            fields: Some(fields),
            value: None,
        }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code.clone();
        let json: Json<Value> = self.into();

        (status, json).into_response()
    }
}

impl Into<Json<Value>> for Response {
    fn into(self) -> Json<Value> {
        Json(json!(self))
    }
}

impl Default for Response {
    fn default() -> Self {
        Response::new_success(StatusCode::OK)
    }
}
