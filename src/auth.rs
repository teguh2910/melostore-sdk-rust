use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::RequestBuilder;
use crate::error::{MelostoreError, Result};

pub const HEADER_API_KEY: &str = "X-API-Key";
pub const HEADER_SECRET_KEY: &str = "X-Secret-Key";

/// Authenticator that adds authentication headers to HTTP requests.
#[derive(Debug, Clone)]
pub struct Auth {
    api_key: String,
    secret_key: String,
}

impl Auth {
    /// Creates a new `Auth` instance with the given API key and Secret key.
    pub fn new(api_key: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
        }
    }

    /// Injects `X-API-Key` and `X-Secret-Key` headers into a `reqwest::RequestBuilder`.
    pub fn apply(&self, builder: RequestBuilder) -> Result<RequestBuilder> {
        let mut headers = HeaderMap::new();

        let api_key_val = HeaderValue::from_str(&self.api_key)
            .map_err(|e| MelostoreError::Authentication(format!("Invalid characters in API key: {}", e)))?;
        let secret_key_val = HeaderValue::from_str(&self.secret_key)
            .map_err(|e| MelostoreError::Authentication(format!("Invalid characters in Secret key: {}", e)))?;

        headers.insert(HeaderName::from_static("x-api-key"), api_key_val);
        headers.insert(HeaderName::from_static("x-secret-key"), secret_key_val);

        Ok(builder.headers(headers))
    }
}
