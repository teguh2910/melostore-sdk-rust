# melostore-sdk-rust

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![Documentation](https://docs.rs/melostore-sdk-rust/badge.svg)](https://docs.rs/melostore-sdk-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

An asynchronous, type-safe Rust SDK for the **[Melo H2H Public API (v1.4.0)](https://api.melostore.id)**.

---

## Features

- **Asynchronous & Fast**: Built on top of `reqwest` and `tokio` with `rustls` TLS support.
- **Dual-Key Authentication**: Automatic injection of `X-API-Key` and `X-Secret-Key` headers.
- **Complete Endpoint Coverage**: 100% compliant with the Melo H2H v1.4.0 OpenAPI specification.
  - **Account & Profile**: `/profile`, `/profile/balance`
  - **Regular & Smart Catalog**: `/pricelists`, `/pricelists/{category_slug}`, `/pricelists/categories`, `/smart-pricelists`, `/smart-pricelists/{category_slug}`, `/smart-pricelists/categories`
  - **Transactions**: `/transaction`, `/smart-transaction`, `/transaction/{id}`, `/transactions/2fa`
  - **Bulk Orders**: `/bulk-orders`, `/bulk-orders/{id}`, `/bulk-orders/{id}/transactions`, `/bulk-orders/{id}/invoice`
  - **Nickname & Limits**: `/check-nickname/games`, `/check-nickname/plan`, `/check-nickname`, `/mobile-legends/purchase-limit`
- **Security-First Webhook Utilities**:
  - RFC 2104 compliant HMAC-SHA256 signature verification (`X-H2H-Signature`).
  - Constant-time comparison to protect against timing side-channel attacks.
  - Polymorphic parsing for all callback events: `RegularCallback`, `SmartCallback`, `BulkCallback`, and `ActionCallback`.
- **Robust Error Handling**: Structured `MelostoreError` with full API error code mapping, validation details, and rate-limiting (`Retry-After`).

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
melostore-sdk-rust = "0.1.0"
tokio = { version = "1.40", features = ["full"] }
```

---

## Quickstart

```rust
use melostore_sdk_rust::{MelostoreClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize with your API Key and Secret Key
    let client = MelostoreClient::new(
        "YOUR_API_KEY",
        "YOUR_SECRET_KEY",
    )?;

    // Check your H2H balance
    let balance = client.get_balance().await?;
    println!("Balance: MC {:.2} (Sandbox: {})", balance.h2h_balance, balance.is_sandbox_mode);

    Ok(())
}
```

---

## Configuration & Builder

You can customize timeout, custom base URLs (for staging/mocking), default sandbox behavior, and the webhook verification secret:

```rust
use melostore_sdk_rust::{Environment, MelostoreClient, MelostoreConfig};
use std::time::Duration;

let config = MelostoreConfig::builder()
    .api_key("YOUR_API_KEY")
    .secret_key("YOUR_SECRET_KEY")
    .webhook_secret("YOUR_WEBHOOK_SECRET")
    .environment(Environment::Production)
    .timeout(Duration::from_secs(45))
    .sandbox_mode(true) // Force simulation mode for all transactions
    .build()?;

let client = MelostoreClient::with_config(config)?;
```

---

## Usage Guide

### 1. Catalog Synchronization

Query regular products or Smart catalogue items with cursor pagination:

```rust
use melostore_sdk_rust::models::catalog::PricelistParams;

// Regular pricelist
let params = PricelistParams {
    limit: Some(500),
    server: Some("asia".into()),
    ..Default::default()
};
let catalogue = client.get_pricelist(Some(params)).await?;
for product in catalogue.data {
    println!("{} ({}): MC {:.2}", product.name, product.sku_code, product.price);
}

// Smart catalogue with multi-supplier routing
let smart = client.get_smart_pricelist(None).await?;
for sp in smart.data {
    println!("Smart SKU: {} (Variants: {})", sp.smart_sku_code, sp.variants.len());
}
```

### 2. Player Nickname & Purchase Limit Inquiries

```rust
use melostore_sdk_rust::models::account::{CheckNicknameRequest, CheckPurchaseLimitRequest};

// Verify player username
let req = CheckNicknameRequest::new("mobile_legends", "12345678").with_zone("1234");
let nick = client.check_nickname(req).await?;
println!("Username: {}", nick.data.username);

// Check Mobile Legends Weekly Diamond Pass purchase limits
let limit_req = CheckPurchaseLimitRequest::new("12345678", "1234");
let limit = client.check_mlbb_purchase_limit(limit_req).await?;
if let Some(pass) = limit.data.weekly_pass {
    println!("Weekly pass limit reached: {}", pass.limit_reached);
}
```

### 3. Placing Orders (Regular & Smart)

```rust
use melostore_sdk_rust::models::transaction::{
    CreateSmartTransactionRequest, CreateTransactionRequest, TransactionStatusData,
};

// 1. Regular Order with Price Protection
let mut req = CreateTransactionRequest::new("MLBB-86", "12345678", "unique-ref-001");
req.customer_target_zone = Some("1234".into());
req.max_price = Some(20000); // Price ceiling
let trx = client.create_transaction(req).await?;
println!("Order Placed: {} (Status: {})", trx.id, trx.status);

// 2. Smart Order with Max Bid
let smart_req = CreateSmartTransactionRequest::new(
    "SMART-WDP",
    "12345678",
    "unique-smart-001",
    28000, // Max bid
);
let ack = client.create_smart_transaction(smart_req).await?;
println!("Smart Order Queued: {}", ack.id);

// 3. Track Status
let status = client.get_transaction("unique-ref-001").await?;
match status {
    TransactionStatusData::Regular(r) => println!("Status: {}", r.status),
    TransactionStatusData::Smart(s) => println!("Smart fulfilled: {:?}", s.fulfilled_sku_code),
}
```

### 4. Two-Factor Authentication (2FA)

When an order enters `need_action: true` (e.g., OTP/2FA requested):

```rust
use melostore_sdk_rust::models::transaction::SubmitTwoFactorRequest;

let req = SubmitTwoFactorRequest::new("unique-ref-001", "654321");
let action = client.submit_two_factor(req).await?;
println!("Action state: {}", action.state);
```

### 5. Bulk Orders & PDF Invoice

```rust
use melostore_sdk_rust::models::bulk::CreateBulkOrderRequest;

// Create batch order
let req = CreateBulkOrderRequest::new("MLBB-86", "12345678", "batch-100", 10);
let bulk = client.create_bulk_order(req).await?;
println!("Bulk Batch Created: {} (Total Debit: MC {:.2})", bulk.id, bulk.total_debit);

// Download PDF Invoice
let pdf_bytes = client.get_bulk_invoice_pdf(&bulk.id).await?;
std::fs::write("invoice.pdf", &pdf_bytes)?;
println!("Saved invoice.pdf ({} bytes)", pdf_bytes.len());
```

### 6. Webhook Signature Verification & Handling

Melo H2H sends callbacks with an `X-H2H-Signature` header containing the raw-body HMAC-SHA256:

```rust
use melostore_sdk_rust::webhook::{parse_and_verify_webhook, WebhookPayload};

// In your Axum, Actix, or Rocket webhook handler:
fn handle_melo_callback(secret: &str, raw_body: &[u8], signature_hex: &str) {
    match parse_and_verify_webhook(secret, raw_body, signature_hex) {
        Ok(event) => match event {
            WebhookPayload::Regular(cb) => {
                println!("Order {} finished with status: {}", cb.buyer_trx_id, cb.status);
            }
            WebhookPayload::Smart(cb) => {
                println!("Smart Order {} finished. Fulfilled: {:?}", cb.buyer_trx_id, cb.fulfilled_sku_code);
            }
            WebhookPayload::Bulk(cb) => {
                println!("Bulk item {} in batch {} completed", cb.bulk_sequence, cb.buyer_bulk_id);
            }
            WebhookPayload::Action(cb) => {
                println!("Action update for order {}: need_action = {}", cb.id, cb.need_action);
            }
        },
        Err(e) => {
            eprintln!("Invalid webhook signature or payload: {}", e);
        }
    }
}
```

---

## Running Examples & Tests

### Run Unit & Integration Tests
```bash
cargo test
```

### Run Examples
```bash
# Check profile and balance
MELO_API_KEY=your_key MELO_SECRET_KEY=your_secret cargo run --example check_balance

# Place orders
MELO_API_KEY=your_key MELO_SECRET_KEY=your_secret cargo run --example order_product

# Webhook verification
cargo run --example handle_webhook
```

---

## License

This project is licensed under the [MIT License](LICENSE).
