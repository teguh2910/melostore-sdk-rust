//! Example demonstrating how to securely verify and handle Melo H2H webhook callbacks.
//!
//! Run with:
//! ```bash
//! cargo run --example handle_webhook
//! ```

use melostore_sdk_rust::webhook::{hmac_sha256, parse_and_verify_webhook, WebhookPayload};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook_secret =
        env::var("MELO_WEBHOOK_SECRET").unwrap_or_else(|_| "sample_webhook_secret_12345".to_string());

    // Simulated incoming HTTP request payload
    let raw_payload = br#"{
        "id": "11111111-1111-4111-8111-111111111111",
        "buyer_trx_id": "partner-ref-001",
        "sku_code": "MLBB-86",
        "customer_target": "12345678",
        "customer_target_zone": "1234",
        "status": "success",
        "serial_number": "SN-DELIVERED-998877",
        "is_sandbox": true,
        "created_at": "2026-10-02T03:00:00+00:00",
        "price_charged": 19500.0,
        "updated_at": "2026-10-02T03:00:15+00:00"
    }"#;

    // Simulate Melo H2H header signature: X-H2H-Signature
    let signature = hex::encode(hmac_sha256(webhook_secret.as_bytes(), raw_payload));
    println!("Simulated Incoming Signature: {}", signature);

    // Verify signature & parse polymorphic payload
    match parse_and_verify_webhook(&webhook_secret, raw_payload, &signature) {
        Ok(event) => match event {
            WebhookPayload::Regular(cb) => {
                println!("[Regular Order Callback]");
                println!("Order ID: {}", cb.id);
                println!("Partner Ref: {}", cb.buyer_trx_id);
                println!("Status: {}", cb.status);
                println!("Charged: MC {:.2}", cb.price_charged);
                if let Some(sn) = cb.serial_number {
                    println!("Serial Number: {}", sn);
                }
            }
            WebhookPayload::Smart(cb) => {
                println!("[Smart Order Callback]");
                println!("Order ID: {}", cb.id);
                println!("Status: {}", cb.status);
                println!("Fulfilled SKU: {:?}", cb.fulfilled_sku_code);
                println!("Supplier Attempts: {}", cb.attempts_count);
            }
            WebhookPayload::Bulk(cb) => {
                println!("[Bulk Child Callback]");
                println!("Bulk Batch: {}", cb.buyer_bulk_id);
                println!("Sequence #: {}", cb.bulk_sequence);
                println!("Status: {}", cb.status);
            }
            WebhookPayload::Action(cb) => {
                println!("[Action Change Notification]");
                println!("Event: {}", cb.event);
                println!("Need Action: {}", cb.need_action);
                if let Some(action) = cb.action {
                    println!("Action Type: {}", action.action_type);
                    println!("Message: {}", action.message);
                    println!("Can Submit: {}", action.can_submit);
                }
            }
        },
        Err(err) => {
            eprintln!("Webhook signature validation failed: {}", err);
        }
    }

    Ok(())
}
