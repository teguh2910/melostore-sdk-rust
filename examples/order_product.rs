//! Example demonstrating how to check catalog, verify nickname, and place regular & Smart orders.
//!
//! Run with:
//! ```bash
//! cargo run --example order_product
//! ```

use melostore_sdk_rust::models::account::CheckNicknameRequest;
use melostore_sdk_rust::models::transaction::{CreateSmartTransactionRequest, CreateTransactionRequest};
use melostore_sdk_rust::{MelostoreClient, Result};
use std::env;

#[allow(clippy::result_large_err)]
#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("MELO_API_KEY").unwrap_or_else(|_| "YOUR_API_KEY".to_string());
    let secret_key = env::var("MELO_SECRET_KEY").unwrap_or_else(|_| "YOUR_SECRET_KEY".to_string());

    let client = MelostoreClient::new(api_key, secret_key)?;

    // 1. Verify Player In-Game Nickname before ordering
    println!("Checking player nickname for MLBB...");
    let nickname_req = CheckNicknameRequest::new("mobile_legends", "12345678").with_zone("1234");
    match client.check_nickname(nickname_req).await {
        Ok(res) => {
            println!("Player Nickname: {}", res.data.username);
        }
        Err(e) => {
            println!("Nickname check failed: {}", e);
        }
    }

    // 2. Place a regular order with maximum price protection (Sandbox simulated)
    println!("\nPlacing regular order with price protection...");
    let mut order_req = CreateTransactionRequest::new("MLBB-86", "12345678", "partner-ref-001");
    order_req.customer_target_zone = Some("1234".into());
    order_req.sandbox_mode = Some(true); // Sandbox simulation
    order_req.max_price = Some(20000);   // Max MC 20,000

    match client.create_transaction(order_req).await {
        Ok(trx) => {
            println!("Order Created Successfully!");
            println!("Transaction ID: {}", trx.id);
            println!("Status: {}", trx.status);
            println!("Price Charged: MC {:.2}", trx.price);
            if let Some(sn) = trx.serial_number {
                println!("Serial Number / Voucher: {}", sn);
            }
        }
        Err(e) => {
            println!("Order placement failed: {}", e);
        }
    }

    // 3. Place a Smart Order with maximum bid protection
    println!("\nPlacing Smart Order with max bid protection...");
    let mut smart_req = CreateSmartTransactionRequest::new(
        "SMART-WDP",
        "12345678",
        "partner-smart-001",
        28000, // Max bid MC 28,000
    );
    smart_req.customer_target_zone = Some("1234".into());
    smart_req.sandbox_mode = Some(true);

    match client.create_smart_transaction(smart_req).await {
        Ok(ack) => {
            println!("Smart Order Queued!");
            println!("Transaction ID: {}", ack.id);
            println!("Status: {}", ack.status);
        }
        Err(e) => {
            println!("Smart order placement failed: {}", e);
        }
    }

    Ok(())
}
