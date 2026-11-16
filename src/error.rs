use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur when using the Melostore SDK.
#[derive(Debug, Error)]
#[allow(clippy::result_large_err)]
pub enum MelostoreError {
    /// An error returned by the underlying HTTP transport (reqwest).
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// An API-level error returned by Melo H2H API.
    #[error("API error (HTTP {status_code}): {message}")]
    Api {
        /// HTTP status code (e.g. 400, 401, 403, 404, 409, 422, 500)
        status_code: u16,
        /// Error message description returned by Melo API
        message: String,
        /// Specific machine-readable error code (e.g. "MAX_PRICE_TOO_LOW", "SUPPLIER_ERROR")
        error_code: Option<String>,
        /// Validation field errors (e.g. { "buyer_trx_id": ["has already been taken"] })
        errors: Option<HashMap<String, Vec<String>>>,
        /// Raw response body if available
        raw_body: Option<String>,
    },

    /// Request was rate limited (HTTP 429).
    #[error("Rate limited: {message} (Retry after: {retry_after:?} seconds)")]
    RateLimited {
        /// Number of seconds to wait before retrying, if specified by Retry-After header
        retry_after: Option<u64>,
        /// Message from the server
        message: String,
    },

    /// An error during authentication or credential validation.
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// JSON serialization or deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid parameter supplied to a request or configuration.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Missing required header or configuration field.
    #[error("Missing required field: {0}")]
    MissingRequiredField(String),

    /// Webhook verification error (e.g. signature mismatch).
    #[error("Webhook verification failed: {0}")]
    WebhookVerificationFailed(String),
}

impl MelostoreError {
    /// Creates a new API error.
    pub fn api(
        status_code: u16,
        message: impl Into<String>,
        error_code: Option<String>,
        errors: Option<HashMap<String, Vec<String>>>,
        raw_body: Option<String>,
    ) -> Self {
        Self::Api {
            status_code,
            message: message.into(),
            error_code,
            errors,
            raw_body,
        }
    }
}

/// Convenience alias for SDK operations returning a `Result`.
pub type Result<T> = std::result::Result<T, MelostoreError>;
