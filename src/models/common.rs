use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize};

/// Generic standard response wrapper used by Melo H2H API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse<T> {
    /// Indicates whether the request succeeded.
    pub success: bool,
    /// Message returned by the API (often present on create or update operations).
    #[serde(default)]
    pub message: Option<String>,
    /// Payload data.
    pub data: T,
}

/// Structured error response returned by Melo H2H API on failure (4xx/5xx).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiErrorBody {
    /// Always false for error responses.
    #[serde(default)]
    pub success: Option<bool>,
    /// Human-readable error message.
    pub message: String,
    /// Detailed validation error messages grouped by input field.
    #[serde(default)]
    pub errors: Option<HashMap<String, Vec<String>>>,
    /// Specific machine-readable error code (e.g. "MAX_PRICE_TOO_LOW").
    #[serde(default)]
    pub error_code: Option<String>,
    /// Nested error details.
    #[serde(default)]
    pub error: Option<NestedErrorDetails>,
    /// Billing information snapshot if the error occurred after a billing attempt.
    #[serde(default)]
    pub billing: Option<BillingDetails>,
}

/// Nested error details within an error response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedErrorDetails {
    /// Numeric error code.
    pub code: i64,
    /// Error category name.
    pub category: String,
    /// Optional nested error message.
    #[serde(default)]
    pub message: Option<String>,
}

/// Billing information returned with inquiries or error responses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BillingDetails {
    /// Amount charged in Melo Coin (MC).
    #[serde(default)]
    pub charged_mc: Option<f64>,
    /// Remaining free check quota for today.
    #[serde(default)]
    pub free_remaining_today: Option<i64>,
    /// Transaction ID for the billing deduction.
    #[serde(default)]
    pub transaction_id: Option<String>,
}

/// Pagination metadata for cursor-based catalog queries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CursorPagination {
    /// Current cursor token.
    #[serde(default)]
    pub cursor: Option<String>,
    /// Next cursor token for fetching the subsequent page.
    #[serde(default)]
    pub next_cursor: Option<String>,
    /// Limit of items requested.
    pub limit: u32,
    /// Total count of items matching the query.
    #[serde(default)]
    pub total: Option<u64>,
    /// Whether there are more items to paginate.
    pub has_more: bool,
}

/// Product input form instructions and fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InquiryForm {
    /// List of input fields required to purchase the product.
    #[serde(default)]
    pub fields: Vec<InquiryFormField>,
    /// Multilingual instructions and notices.
    #[serde(default)]
    pub notice: Option<InquiryNotice>,
}

/// Individual input field definition for a product order form.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InquiryFormField {
    /// Field identifier key (e.g. "user_id", "zone_id", "server").
    #[serde(default)]
    pub key: String,
    /// Display name of the field.
    #[serde(default)]
    pub name: String,
    /// Input type (e.g. "text", "number", "select").
    #[serde(default, rename = "type")]
    pub field_type: String,
    /// Whether this field is mandatory.
    #[serde(default)]
    pub required: bool,
    /// Available options for dropdown/selection fields.
    #[serde(default)]
    pub options: Option<Vec<InquiryFormOption>>,
    /// Placeholder text for the input element.
    #[serde(default)]
    pub placeholder: Option<String>,
}

/// Option entry for selectable form fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InquiryFormOption {
    /// Display label.
    #[serde(default)]
    pub label: String,
    /// Value to submit in the request.
    #[serde(default)]
    pub value: serde_json::Value,
}

/// Multilingual instructions for an inquiry form.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct InquiryNotice {
    /// Indonesian instructions.
    #[serde(default)]
    pub id: Option<String>,
    /// English instructions.
    #[serde(default)]
    pub en: Option<String>,
}

/// Deserializer helper to handle Laravel/PHP empty assoc arrays serializing as `[]` instead of `{}`.
pub fn deserialize_inquiry_forms<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<String, InquiryForm>, D::Error>
where
    D: Deserializer<'de>,
{
    let val = serde_json::Value::deserialize(deserializer)?;
    match val {
        serde_json::Value::Object(map) => {
            let mut result = HashMap::new();
            for (k, v) in map {
                if let Ok(form) = serde_json::from_value::<InquiryForm>(v) {
                    result.insert(k, form);
                }
            }
            Ok(result)
        }
        serde_json::Value::Array(_) => Ok(HashMap::new()),
        _ => Ok(HashMap::new()),
    }
}
