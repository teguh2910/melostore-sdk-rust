use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use melostore_sdk_rust::MelostoreError;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Upstream error ({0}): {1}")]
    Upstream(StatusCode, String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<MelostoreError> for AppError {
    fn from(err: MelostoreError) -> Self {
        match err {
            MelostoreError::Api {
                status_code,
                message,
                ..
            } => {
                let code = StatusCode::from_u16(status_code).unwrap_or(StatusCode::BAD_GATEWAY);
                AppError::Upstream(code, message)
            }
            MelostoreError::RateLimited { message, .. } => {
                AppError::Upstream(StatusCode::TOO_MANY_REQUESTS, message)
            }
            MelostoreError::WebhookVerificationFailed(e) => AppError::BadRequest(e),
            MelostoreError::MissingRequiredField(e) => AppError::BadRequest(e),
            MelostoreError::InvalidParameter(e) => AppError::BadRequest(e),
            MelostoreError::Authentication(e) => {
                AppError::Upstream(StatusCode::UNAUTHORIZED, format!("Authentication error: {e}"))
            }
            MelostoreError::Http(e) => AppError::Provider(format!("Provider connection error: {e}")),
            MelostoreError::Json(e) => AppError::Internal(format!("Serialization error: {e}")),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Upstream(code, msg) => (code, msg),
            AppError::Provider(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "result": false,
            "message": message,
        }));

        (status, body).into_response()
    }
}
