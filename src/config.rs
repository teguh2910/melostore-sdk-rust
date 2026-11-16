use std::time::Duration;
use crate::error::{MelostoreError, Result};

/// API Environment for Melo H2H service.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Environment {
    /// Live production environment (https://api.melostore.id)
    #[default]
    Production,
    /// Custom base URL (e.g. for mock testing or staging)
    Custom(String),
}

impl Environment {
    /// Returns the base URL string for this environment.
    pub fn base_url(&self) -> &str {
        match self {
            Environment::Production => "https://api.melostore.id",
            Environment::Custom(url) => url.as_str(),
        }
    }
}

/// Configuration settings for the Melo H2H client.
#[derive(Debug, Clone)]
pub struct MelostoreConfig {
    /// Partner API Key (sent via X-API-Key header)
    pub api_key: String,
    /// Partner API Secret Key (sent via X-Secret-Key header)
    pub secret_key: String,
    /// Webhook Secret used to verify incoming webhook callbacks HMAC-SHA256 signature
    pub webhook_secret: Option<String>,
    /// Base URL for API requests
    pub base_url: String,
    /// Request timeout duration (defaults to 30 seconds)
    pub timeout: Duration,
    /// Default sandbox mode for transactions (None uses account dashboard default)
    pub sandbox_mode: Option<bool>,
}

impl MelostoreConfig {
    /// Creates a new configuration builder.
    pub fn builder() -> MelostoreConfigBuilder {
        MelostoreConfigBuilder::default()
    }

    /// Creates a new configuration with required credentials and default production environment.
    pub fn new(
        api_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            webhook_secret: None,
            base_url: Environment::Production.base_url().to_string(),
            timeout: Duration::from_secs(30),
            sandbox_mode: None,
        }
    }
}

/// Builder for constructing a [`MelostoreConfig`].
#[derive(Debug, Default)]
pub struct MelostoreConfigBuilder {
    api_key: Option<String>,
    secret_key: Option<String>,
    webhook_secret: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    sandbox_mode: Option<bool>,
}

impl MelostoreConfigBuilder {
    /// Sets the API Key (`X-API-Key`).
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the API Secret Key (`X-Secret-Key`).
    pub fn secret_key(mut self, secret_key: impl Into<String>) -> Self {
        self.secret_key = Some(secret_key.into());
        self
    }

    /// Sets the Webhook Secret for signature verification.
    pub fn webhook_secret(mut self, webhook_secret: impl Into<String>) -> Self {
        self.webhook_secret = Some(webhook_secret.into());
        self
    }

    /// Sets the environment.
    pub fn environment(mut self, env: Environment) -> Self {
        self.base_url = Some(env.base_url().to_string());
        self
    }

    /// Sets custom base URL directly.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        let mut u = url.into();
        if u.ends_with('/') {
            u.pop();
        }
        self.base_url = Some(u);
        self
    }

    /// Sets request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets default sandbox mode flag.
    pub fn sandbox_mode(mut self, sandbox: bool) -> Self {
        self.sandbox_mode = Some(sandbox);
        self
    }

    /// Builds and validates the [`MelostoreConfig`].
    pub fn build(self) -> Result<MelostoreConfig> {
        let api_key = self
            .api_key
            .ok_or_else(|| MelostoreError::MissingRequiredField("api_key is required".into()))?;
        let secret_key = self
            .secret_key
            .ok_or_else(|| MelostoreError::MissingRequiredField("secret_key is required".into()))?;

        if api_key.trim().is_empty() {
            return Err(MelostoreError::InvalidParameter("api_key cannot be empty".into()));
        }
        if secret_key.trim().is_empty() {
            return Err(MelostoreError::InvalidParameter("secret_key cannot be empty".into()));
        }

        Ok(MelostoreConfig {
            api_key,
            secret_key,
            webhook_secret: self.webhook_secret,
            base_url: self
                .base_url
                .unwrap_or_else(|| Environment::Production.base_url().to_string()),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            sandbox_mode: self.sandbox_mode,
        })
    }
}
