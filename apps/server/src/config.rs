use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub api_key: String,
    pub secret_key: String,
    pub webhook_secret: Option<String>,
    pub base_url: Option<String>,
    pub sandbox_mode: bool,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
        let api_key = env::var("MELOSTORE_API_KEY")
            .unwrap_or_else(|_| env::var("API_KEY").unwrap_or_default());
        let secret_key = env::var("MELOSTORE_API_SECRET")
            .unwrap_or_else(|_| env::var("API_SECRET").unwrap_or_default());
        let webhook_secret = env::var("MELOSTORE_WEBHOOK_SECRET")
            .ok()
            .or_else(|| env::var("WEBHOOK_SECRET").ok());
        let base_url = env::var("MELOSTORE_BASE_URL")
            .ok()
            .or_else(|| env::var("BASE_URL").ok());
        let sandbox_mode = env::var("MELOSTORE_SANDBOX")
            .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
            .unwrap_or(true);
        let cors_origin = env::var("CORS_ORIGIN").unwrap_or_else(|_| "*".to_string());

        Ok(Self {
            bind_addr,
            api_key,
            secret_key,
            webhook_secret,
            base_url,
            sandbox_mode,
            cors_origin,
        })
    }
}
