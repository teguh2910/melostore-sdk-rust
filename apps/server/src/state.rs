use crate::config::Config;
use melostore_sdk_rust::{MelostoreClient, MelostoreConfig};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<MelostoreClient>,
    pub cfg: Arc<Config>,
}

impl AppState {
    pub fn new(cfg: Config) -> Result<Self, String> {
        let mut builder = MelostoreConfig::builder()
            .api_key(&cfg.api_key)
            .secret_key(&cfg.secret_key)
            .sandbox_mode(cfg.sandbox_mode);

        if let Some(wh) = &cfg.webhook_secret {
            builder = builder.webhook_secret(wh);
        }

        if let Some(url) = &cfg.base_url {
            builder = builder.base_url(url);
        }

        let melo_config = builder.build().map_err(|e| e.to_string())?;
        let client = MelostoreClient::with_config(melo_config).map_err(|e| e.to_string())?;

        Ok(Self {
            client: Arc::new(client),
            cfg: Arc::new(cfg),
        })
    }
}
