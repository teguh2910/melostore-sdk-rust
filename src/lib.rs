//! # Melo Store SDK for Rust
//!
//! An asynchronous, type-safe Rust SDK for integrating with the [Melo H2H Public API (v1.4.0)](https://api.melostore.id).
//!
//! ## Key Capabilities
//! - **Account & Wallet Management**: Check partner profile, tier, request limits, and live balance (`/profile`, `/profile/balance`).
//! - **Product Catalog Synchronization**: Query regular and Smart catalogues with cursor pagination and multi-variant pricing (`/pricelists`, `/smart-pricelists`).
//! - **Transactional Ordering**: Place regular orders with price protection (`max_price`), execute Smart Orders with automated supplier routing (`max_bid`), and track order status (`/transaction`, `/smart-transaction`).
//! - **Two-Factor Authentication**: Seamlessly submit customer 2FA verification codes for pending orders (`/transactions/2fa`).
//! - **Bulk Orders & Invoices**: Manage repeated bulk batch orders and download official PDF invoices (`/bulk-orders`, `/bulk-orders/{id}/invoice`).
//! - **Player Nickname & Limit Inquiries**: Verify in-game usernames and check Mobile Legends Weekly Diamond Pass purchase limits (`/check-nickname`, `/mobile-legends/purchase-limit`).
//! - **Webhook Utilities**: Verify HMAC-SHA256 signatures with constant-time security and deserialize polymorphic callbacks (`RegularCallback`, `SmartCallback`, `BulkCallback`, `ActionCallback`).
//!
//! ## Quickstart
//!
//! ```no_run
//! use melostore_sdk_rust::{MelostoreClient, MelostoreConfig, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Initialize the client with your credentials
//!     let client = MelostoreClient::new(
//!         "YOUR_API_KEY",
//!         "YOUR_SECRET_KEY",
//!     )?;
//!
//!     // Check wallet balance
//!     let balance = client.get_balance().await?;
//!     println!("H2H Balance: MC {}", balance.h2h_balance);
//!
//!     Ok(())
//! }
//! ```

#![allow(clippy::result_large_err)]

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod webhook;

// Re-exports
pub use auth::Auth;
pub use client::MelostoreClient;
pub use config::{Environment, MelostoreConfig, MelostoreConfigBuilder};
pub use error::{MelostoreError, Result};
pub use models::*;
pub use webhook::{parse_and_verify_webhook, parse_webhook, verify_webhook_signature, WebhookPayload};
