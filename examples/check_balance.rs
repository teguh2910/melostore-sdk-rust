//! Example demonstrating how to initialize the client and check account profile and balance.
//!
//! Run with:
//! ```bash
//! cargo run --example check_balance
//! ```

use melostore_sdk_rust::{MelostoreClient, MelostoreConfig, Result};
use std::env;

#[allow(clippy::result_large_err)]
#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("MELO_API_KEY").unwrap_or_else(|_| "YOUR_API_KEY".to_string());
    let secret_key = env::var("MELO_SECRET_KEY").unwrap_or_else(|_| "YOUR_SECRET_KEY".to_string());

    let config = MelostoreConfig::builder()
        .api_key(api_key)
        .secret_key(secret_key)
        .build()?;

    let client = MelostoreClient::with_config(config)?;

    println!("Fetching account balance...");
    let balance = client.get_balance().await?;
    println!("--- H2H Wallet Balance ---");
    println!("Saldo H2H: MC {:.2}", balance.h2h_balance);
    println!("Sandbox Mode: {}", balance.is_sandbox_mode);
    println!("USD Rate: IDR {:.2}", balance.usd_idr_rate);
    if let Some(usd) = balance.h2h_balance_usd {
        println!("Estimasi USD: ${:.2}", usd);
    }

    println!("\nFetching account profile...");
    let profile = client.get_profile().await?;
    println!("--- Partner Profile ---");
    println!("Partner Name: {}", profile.name);
    println!("Email: {}", profile.email);
    println!("Tier: {} (Limit: {} req/min)", profile.tier.name, profile.tier.rate_limit);
    if let Some(url) = profile.webhook_url {
        println!("Webhook Callback URL: {}", url);
    }

    Ok(())
}
