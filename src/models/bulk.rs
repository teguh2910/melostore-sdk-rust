use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Request body for creating a bulk order (`POST /api/v1/h2h/bulk-orders`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBulkOrderRequest {
    /// Exact sku_code from regular catalogue.
    pub sku_code: String,
    /// Customer account identifier.
    pub customer_target: String,
    /// Zone/server if required by product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_target_zone: Option<String>,
    /// Additional product inquiry form values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, serde_json::Value>>,
    /// Force sandbox mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_mode: Option<bool>,
    /// Your unique batch identifier.
    pub buyer_bulk_id: String,
    /// Total number of repeat orders to execute (min: 1).
    pub order_count: u32,
}

impl CreateBulkOrderRequest {
    /// Creates a new bulk order request.
    pub fn new(
        sku_code: impl Into<String>,
        customer_target: impl Into<String>,
        buyer_bulk_id: impl Into<String>,
        order_count: u32,
    ) -> Self {
        Self {
            sku_code: sku_code.into(),
            customer_target: customer_target.into(),
            customer_target_zone: None,
            additional_data: None,
            sandbox_mode: None,
            buyer_bulk_id: buyer_bulk_id.into(),
            order_count,
        }
    }
}

/// Query parameters for listing bulk orders history.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BulkHistoryParams {
    /// Page number (1-indexed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Filter by bulk status ("pending", "processing", "completed", "partially_failed", "failed").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Search by buyer_bulk_id or SKU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter by start date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Filter by end date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// Bulk order summary and progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkOrder {
    /// Bulk order UUID.
    pub id: String,
    /// Partner bulk identifier.
    pub buyer_bulk_id: String,
    /// Product SKU code.
    pub sku_code: String,
    /// Product display name.
    pub product_name: String,
    /// Customer account identifier.
    pub customer_target: String,
    /// Customer zone/server.
    #[serde(default)]
    pub customer_target_zone: Option<String>,
    /// Total child orders requested.
    pub order_count: i64,
    /// Price per unit.
    pub unit_price: f64,
    /// Total amount debited.
    pub total_debit: f64,
    /// Batch status ("pending", "processing", "completed", "failed", "partially_failed").
    pub status: String,
    /// Origin of bulk order (e.g. "api").
    #[serde(default)]
    pub via: Option<String>,
    /// Whether in sandbox mode.
    pub is_sandbox: bool,
    /// Whether PDF invoice is ready for download.
    pub invoice_available: bool,
    /// Creation timestamp.
    pub created_at: String,
    /// Completion timestamp.
    #[serde(default)]
    pub completed_at: Option<String>,
    /// Total child items count.
    pub total_count: i64,
    /// Succeeded child items count.
    pub success_count: i64,
    /// Failed child items count.
    pub failed_count: i64,
    /// Processed items count.
    pub processed_count: i64,
    /// Remaining items count.
    pub remaining_count: i64,
    /// Currently processing count.
    pub processing_count: i64,
    /// Queued items count.
    pub queued_count: i64,
}

/// Navigation link in Laravel pagination.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageLink {
    /// URL of the page.
    #[serde(default)]
    pub url: Option<String>,
    /// Label of the link (e.g. "&laquo; Previous", "1", "Next &raquo;").
    pub label: String,
    /// Whether this is the active current page.
    pub active: bool,
}

/// Paginated page of bulk orders.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkPage {
    /// Current page number.
    pub current_page: u32,
    /// Bulk orders in the current page.
    pub data: Vec<BulkOrder>,
    /// First page URL.
    #[serde(default)]
    pub first_page_url: Option<String>,
    /// Starting item index on this page.
    #[serde(default)]
    pub from: Option<u64>,
    /// Last page number.
    pub last_page: u32,
    /// Last page URL.
    #[serde(default)]
    pub last_page_url: Option<String>,
    /// Pagination links.
    #[serde(default)]
    pub links: Option<Vec<PageLink>>,
    /// Next page URL.
    #[serde(default)]
    pub next_page_url: Option<String>,
    /// Base endpoint path.
    #[serde(default)]
    pub path: Option<String>,
    /// Items per page.
    pub per_page: u32,
    /// Previous page URL.
    #[serde(default)]
    pub prev_page_url: Option<String>,
    /// Ending item index on this page.
    #[serde(default)]
    pub to: Option<u64>,
    /// Total count of matching bulk orders.
    pub total: u64,
}

/// Individual child transaction within a bulk order.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkChild {
    /// Child transaction UUID.
    pub id: String,
    /// Auto-generated child order reference.
    pub buyer_trx_id: String,
    /// Parent bulk order UUID.
    pub bulk_order_id: String,
    /// Parent bulk order partner reference.
    pub buyer_bulk_id: String,
    /// Sequence index (1-based) within the bulk batch.
    pub bulk_sequence: i64,
    /// SKU code.
    #[serde(default)]
    pub sku_code: Option<String>,
    /// Customer account identifier.
    pub customer_target: String,
    /// Customer zone/server.
    #[serde(default)]
    pub customer_target_zone: Option<String>,
    /// Price charged for this child order.
    pub price: f64,
    /// Status: "pending", "processing", "success", "failed", "refunded".
    pub status: String,
    /// Serial number or supplier confirmation.
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Error code if failed.
    #[serde(default)]
    pub error_code: Option<String>,
    /// Error message.
    #[serde(default)]
    pub message: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

/// Paginated page of child transactions for a bulk order.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChildPage {
    /// Current page number.
    pub current_page: u32,
    /// Child transactions on this page.
    pub data: Vec<BulkChild>,
    /// First page URL.
    #[serde(default)]
    pub first_page_url: Option<String>,
    /// Starting item index on this page.
    #[serde(default)]
    pub from: Option<u64>,
    /// Last page number.
    pub last_page: u32,
    /// Last page URL.
    #[serde(default)]
    pub last_page_url: Option<String>,
    /// Pagination links.
    #[serde(default)]
    pub links: Option<Vec<PageLink>>,
    /// Next page URL.
    #[serde(default)]
    pub next_page_url: Option<String>,
    /// Base endpoint path.
    #[serde(default)]
    pub path: Option<String>,
    /// Items per page.
    pub per_page: u32,
    /// Previous page URL.
    #[serde(default)]
    pub prev_page_url: Option<String>,
    /// Ending item index on this page.
    #[serde(default)]
    pub to: Option<u64>,
    /// Total count of child transactions.
    pub total: u64,
}

/// Webhook payload when a child item in a bulk order completes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkCallback {
    /// Child transaction UUID.
    pub id: String,
    /// Partner child order reference.
    pub buyer_trx_id: String,
    /// Product SKU code.
    #[serde(default)]
    pub sku_code: Option<String>,
    /// Customer target.
    pub customer_target: String,
    /// Customer target zone.
    #[serde(default)]
    pub customer_target_zone: Option<String>,
    /// Status: "success", "failed".
    pub status: String,
    /// Serial number or delivery evidence.
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Sandbox indicator.
    pub is_sandbox: bool,
    /// Creation timestamp.
    pub created_at: String,
    /// Final price charged.
    pub price_charged: f64,
    /// Update timestamp.
    pub updated_at: String,
    /// Parent bulk order UUID.
    pub bulk_order_id: String,
    /// Parent bulk order partner reference.
    pub buyer_bulk_id: String,
    /// Sequence index within the batch.
    pub bulk_sequence: i64,
    /// Error code if failed.
    #[serde(default)]
    pub error_code: Option<String>,
    /// Error message.
    #[serde(default)]
    pub message: Option<String>,
}
