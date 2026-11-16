use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Request body for placing a regular order (`POST /api/v1/h2h/transaction`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTransactionRequest {
    /// Exact sku_code from the regular catalogue.
    pub sku_code: String,
    /// Customer ID, phone number, or email according to the first catalogue form field.
    pub customer_target: String,
    /// Account zone/server if required by the product form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_target_zone: Option<String>,
    /// Additional product form fields keyed by their inquiry_form key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, serde_json::Value>>,
    /// Your unique order reference for idempotency.
    pub buyer_trx_id: String,
    /// Force sandbox mode: true for simulation, false for live debit, None to follow account setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_mode: Option<bool>,
    /// Optional price protection: maximum price in MC/IDR you are willing to pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<i64>,
}

impl CreateTransactionRequest {
    /// Creates a new regular transaction request with mandatory fields.
    pub fn new(
        sku_code: impl Into<String>,
        customer_target: impl Into<String>,
        buyer_trx_id: impl Into<String>,
    ) -> Self {
        Self {
            sku_code: sku_code.into(),
            customer_target: customer_target.into(),
            customer_target_zone: None,
            additional_data: None,
            buyer_trx_id: buyer_trx_id.into(),
            sandbox_mode: None,
            max_price: None,
        }
    }
}

/// Request body for placing a Smart Order (`POST /api/v1/h2h/smart-transaction`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSmartTransactionRequest {
    /// Smart SKU code from the Smart catalogue.
    pub sku_code: String,
    /// Customer ID, phone number, or email.
    pub customer_target: String,
    /// Account zone/server if required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_target_zone: Option<String>,
    /// Additional product form fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, serde_json::Value>>,
    /// Your unique order reference.
    pub buyer_trx_id: String,
    /// Maximum bid price in MC/IDR.
    pub max_bid: i64,
    /// Force sandbox mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_mode: Option<bool>,
}

impl CreateSmartTransactionRequest {
    /// Creates a new Smart transaction request with mandatory fields.
    pub fn new(
        sku_code: impl Into<String>,
        customer_target: impl Into<String>,
        buyer_trx_id: impl Into<String>,
        max_bid: i64,
    ) -> Self {
        Self {
            sku_code: sku_code.into(),
            customer_target: customer_target.into(),
            customer_target_zone: None,
            additional_data: None,
            buyer_trx_id: buyer_trx_id.into(),
            max_bid,
            sandbox_mode: None,
        }
    }
}

/// Request body for submitting a two-factor verification code.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubmitTwoFactorRequest {
    /// Order reference ID currently waiting for this 2FA code.
    pub buyer_trx_id: String,
    /// 2FA code received from customer (1 to 128 characters).
    pub code: String,
    /// Optional action ID from the order status or webhook event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    /// Optional action version to prevent stale submissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_version: Option<i64>,
}

impl SubmitTwoFactorRequest {
    /// Creates a new two-factor submission request.
    pub fn new(buyer_trx_id: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            buyer_trx_id: buyer_trx_id.into(),
            code: code.into(),
            action_id: None,
            action_version: None,
        }
    }
}

/// Customer action requirement on an order (e.g. 2FA verification).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomerAction {
    /// Action UUID.
    pub id: String,
    /// Action type (e.g. "two_factor").
    #[serde(rename = "type")]
    pub action_type: String,
    /// Action state: "required", "submitted", or "resolved".
    pub state: String,
    /// Monotonically increasing version number for idempotency.
    pub version: i64,
    /// User instructions.
    pub message: String,
    /// Whether the action is currently open for submission.
    pub can_submit: bool,
    /// Last update timestamp.
    pub updated_at: String,
}

/// Regular transaction detail.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    /// Transaction UUID.
    pub id: String,
    /// Partner order reference.
    #[serde(default)]
    pub buyer_trx_id: Option<String>,
    /// Product SKU code.
    #[serde(default)]
    pub sku_code: Option<String>,
    /// Product display name.
    #[serde(default)]
    pub product_name: Option<String>,
    /// Customer account identifier.
    pub customer_target: String,
    /// Customer account zone/server.
    #[serde(default)]
    pub customer_target_zone: Option<String>,
    /// Charged price in MC/IDR.
    pub price: f64,
    /// Transaction status ("pending", "processing", "success", "failed", "refunded").
    pub status: String,
    /// Serial number, voucher code, or supplier confirmation.
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Delivery status of the webhook callback.
    #[serde(default)]
    pub webhook_delivery_status: Option<String>,
    /// Whether this was processed in sandbox mode.
    pub is_sandbox: bool,
    /// Creation timestamp.
    pub created_at: String,
    /// Machine error code if failed.
    #[serde(default)]
    pub error_code: Option<String>,
    /// Failure reason message.
    #[serde(default)]
    pub message: Option<String>,
    /// Price ceiling set on the order.
    #[serde(default)]
    pub max_price: Option<i64>,
    /// Whether this transaction is currently waiting for customer action.
    #[serde(default)]
    pub need_action: Option<bool>,
    /// Details of required customer action.
    #[serde(default)]
    pub action: Option<CustomerAction>,
}

/// Smart Order transaction detail.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SmartTransaction {
    /// Transaction UUID.
    pub id: String,
    /// Partner order reference.
    pub buyer_trx_id: String,
    /// Smart SKU code requested.
    pub sku_code: String,
    /// Product display name.
    #[serde(default)]
    pub product_name: Option<String>,
    /// Customer account identifier.
    pub customer_target: String,
    /// Customer account zone.
    #[serde(default)]
    pub customer_target_zone: Option<String>,
    /// Final price charged.
    pub price: f64,
    /// Transaction status.
    pub status: String,
    /// Serial number or supplier proof.
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Delivery status of webhook callback.
    #[serde(default)]
    pub webhook_delivery_status: Option<String>,
    /// Whether this is a sandbox transaction.
    pub is_sandbox: bool,
    /// Creation timestamp.
    pub created_at: String,
    /// Maximum bid specified by partner.
    #[serde(default)]
    pub max_bid: Option<i64>,
    /// The actual supplier SKU fulfilled.
    #[serde(default)]
    pub fulfilled_sku_code: Option<String>,
    /// Number of supplier attempts made.
    pub attempts_count: i64,
    /// Error code if failed.
    #[serde(default)]
    pub error_code: Option<String>,
    /// Failure message.
    #[serde(default)]
    pub message: Option<String>,
}

/// Acknowledgement returned when a Smart Order is queued (HTTP 201).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SmartCreateAcknowledgement {
    /// Transaction UUID.
    pub id: String,
    /// Partner order reference.
    pub buyer_trx_id: String,
    /// Smart SKU code requested.
    pub sku_code: String,
    /// Product display name.
    #[serde(default)]
    pub product_name: Option<String>,
    /// Customer account identifier.
    pub customer_target: String,
    /// Customer account zone.
    #[serde(default)]
    pub customer_target_zone: Option<String>,
    /// Initial price hold.
    pub price: f64,
    /// Status (typically "pending" or "processing").
    pub status: String,
    /// Serial number if immediately available.
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Webhook delivery status.
    #[serde(default)]
    pub webhook_delivery_status: Option<String>,
    /// Whether in sandbox mode.
    pub is_sandbox: bool,
    /// Creation timestamp.
    pub created_at: String,
    /// Partner max bid.
    #[serde(default)]
    pub max_bid: Option<i64>,
    /// Error code if immediately rejected.
    #[serde(default)]
    pub error_code: Option<String>,
    /// Message.
    #[serde(default)]
    pub message: Option<String>,
}

/// Polymorphic status response for `/api/v1/h2h/transaction/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TransactionStatusData {
    /// Smart transaction details with bid & attempt info.
    Smart(SmartTransaction),
    /// Regular transaction details.
    Regular(Transaction),
}

/// Webhook payload for regular transaction status update.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegularCallback {
    /// Transaction UUID.
    pub id: String,
    /// Partner order reference.
    pub buyer_trx_id: String,
    /// Product SKU code.
    #[serde(default)]
    pub sku_code: Option<String>,
    /// Customer target.
    pub customer_target: String,
    /// Customer target zone.
    #[serde(default)]
    pub customer_target_zone: Option<String>,
    /// Order status: "success", "failed", "pending", "processing".
    pub status: String,
    /// Serial number or delivery evidence.
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Sandbox mode indicator.
    pub is_sandbox: bool,
    /// Creation timestamp.
    pub created_at: String,
    /// Final price charged.
    pub price_charged: f64,
    /// Update timestamp.
    pub updated_at: String,
    /// Error code if failed.
    #[serde(default)]
    pub error_code: Option<String>,
    /// Error description if failed.
    #[serde(default)]
    pub message: Option<String>,
    /// Price ceiling set by buyer.
    #[serde(default)]
    pub max_price: Option<i64>,
    /// Whether user action is needed.
    #[serde(default)]
    pub need_action: Option<bool>,
    /// Action details if action is required.
    #[serde(default)]
    pub action: Option<CustomerAction>,
}

/// Webhook payload for Smart Order status update.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SmartCallback {
    /// Transaction UUID.
    pub id: String,
    /// Partner order reference.
    pub buyer_trx_id: String,
    /// Requested Smart SKU.
    pub sku_code: String,
    /// Customer target.
    pub customer_target: String,
    /// Customer target zone.
    #[serde(default)]
    pub customer_target_zone: Option<String>,
    /// Order status.
    pub status: String,
    /// Serial number or delivery evidence.
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Sandbox mode indicator.
    pub is_sandbox: bool,
    /// Creation timestamp.
    pub created_at: String,
    /// Final price charged.
    pub price_charged: f64,
    /// Update timestamp.
    pub updated_at: String,
    /// Maximum bid specified by partner.
    #[serde(default)]
    pub max_bid: Option<i64>,
    /// Supplier SKU fulfilled.
    #[serde(default)]
    pub fulfilled_sku_code: Option<String>,
    /// Supplier attempt count.
    pub attempts_count: i64,
    /// Error code if failed.
    #[serde(default)]
    pub error_code: Option<String>,
    /// Error message.
    #[serde(default)]
    pub message: Option<String>,
}

/// Webhook payload when an action state changes (e.g. 2FA requested or resolved).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionCallback {
    /// Event name (e.g. "transaction.action_change").
    pub event: String,
    /// Unique event delivery UUID.
    pub event_id: String,
    /// Transaction UUID.
    pub id: String,
    /// Partner order reference.
    #[serde(default)]
    pub buyer_trx_id: Option<String>,
    /// Current transaction status.
    pub status: String,
    /// Update timestamp.
    #[serde(default)]
    pub updated_at: Option<String>,
    /// True if customer action is currently needed.
    pub need_action: bool,
    /// Customer action snapshot.
    #[serde(default)]
    pub action: Option<CustomerAction>,
}
