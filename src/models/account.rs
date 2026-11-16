use serde::{Deserialize, Serialize};
use super::common::BillingDetails;

/// List of supported games for nickname inquiries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GamesResponse {
    /// Indicates success status.
    pub success: bool,
    /// List of games.
    pub games: Vec<NicknameGame>,
}

/// Game supported for nickname check (`GET /api/v1/h2h/check-nickname/games`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NicknameGame {
    /// Unique game identifier to use in `check_nickname`.
    pub game_code: String,
    /// Game display title.
    pub name: String,
    /// Game type classification.
    #[serde(rename = "type")]
    pub game_type: String,
    /// Whether zone ID is required.
    pub zone_required: bool,
    /// Help instructions or valid format for zone values.
    #[serde(default)]
    pub zone_values: Option<String>,
    /// Additional parameter definitions.
    #[serde(default)]
    pub params: Vec<NicknameParam>,
    /// Game availability status.
    pub status: String,
}

/// Parameter requirement for nickname check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NicknameParam {
    /// Parameter name (e.g. "user_id", "zone_id").
    pub name: String,
}

/// Nickname quota and pricing plan (`GET /api/v1/h2h/check-nickname/plan`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NicknamePlan {
    /// Number of free checks granted per day.
    pub free_daily_limit: i64,
    /// Allowed free checks per minute.
    pub free_minute_limit: i64,
    /// Fee in Melo Coin (MC) per paid check after exhausting free quota.
    pub paid_price_mc: i64,
    /// Allowed paid checks per minute.
    pub paid_minute_limit: i64,
    /// Timestamp when daily quota resets.
    pub resets_at: String,
}

/// Request body for verifying a player account username (`POST /api/v1/h2h/check-nickname`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckNicknameRequest {
    /// Game code from `get_nickname_games`.
    pub game_code: String,
    /// Customer user ID or account number.
    pub customer_target: String,
    /// Account zone/server if required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_target_zone: Option<String>,
}

impl CheckNicknameRequest {
    /// Creates a new nickname check request.
    pub fn new(game_code: impl Into<String>, customer_target: impl Into<String>) -> Self {
        Self {
            game_code: game_code.into(),
            customer_target: customer_target.into(),
            customer_target_zone: None,
        }
    }

    /// Sets the customer zone/server.
    pub fn with_zone(mut self, zone: impl Into<String>) -> Self {
        self.customer_target_zone = Some(zone.into());
        self
    }
}

/// Result of a nickname check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NicknameResult {
    /// Indicates success status.
    pub success: bool,
    /// Account username details.
    pub data: NicknameData,
    /// Billing deduction information.
    pub billing: BillingDetails,
}

/// Verified username data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NicknameData {
    /// Game code checked.
    pub game_code: String,
    /// Customer target submitted.
    pub customer_target: String,
    /// Customer zone submitted.
    #[serde(default)]
    pub customer_target_zone: Option<String>,
    /// Resolved player in-game nickname / username.
    pub username: String,
    /// Region name if available.
    #[serde(default)]
    pub region: Option<String>,
    /// Region classification code.
    #[serde(default)]
    pub region_type: Option<String>,
}

/// Request body for Mobile Legends purchase limit check (`POST /api/v1/h2h/mobile-legends/purchase-limit`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckPurchaseLimitRequest {
    /// Mobile Legends User ID.
    pub customer_target: String,
    /// Mobile Legends Zone ID.
    pub customer_target_zone: String,
}

impl CheckPurchaseLimitRequest {
    /// Creates a new Mobile Legends purchase limit check request.
    pub fn new(customer_target: impl Into<String>, customer_target_zone: impl Into<String>) -> Self {
        Self {
            customer_target: customer_target.into(),
            customer_target_zone: customer_target_zone.into(),
        }
    }
}

/// Result of Mobile Legends purchase limit check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PurchaseLimitResult {
    /// Indicates success status.
    pub success: bool,
    /// Purchase limit quota details.
    pub data: PurchaseLimitData,
    /// Billing deduction information if charged.
    #[serde(default)]
    pub billing: Option<BillingDetails>,
}

/// Purchase limit status for Mobile Legends account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PurchaseLimitData {
    /// Game code ("mobile_legends").
    #[serde(default)]
    pub game_code: Option<String>,
    /// Customer User ID.
    pub customer_target: String,
    /// Customer Zone ID.
    pub customer_target_zone: String,
    /// Resolved player username.
    pub username: String,
    /// Weekly Diamond Pass limit status.
    #[serde(default)]
    pub weekly_pass: Option<WeeklyPassLimit>,
    /// Double Diamond bonus eligibility status.
    #[serde(default)]
    pub double_diamonds: Option<DoubleDiamondsLimit>,
}

/// Weekly Diamond Pass eligibility details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WeeklyPassLimit {
    /// Whether the user has reached their maximum weekly limit.
    pub limit_reached: bool,
    /// Package breakdown.
    #[serde(default)]
    pub items: Vec<PurchaseLimitItem>,
}

/// Double Diamond package eligibility details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoubleDiamondsLimit {
    /// Whether user is eligible for double diamond bonus.
    pub eligible: bool,
    /// Package breakdown.
    #[serde(default)]
    pub items: Vec<PurchaseLimitItem>,
}

/// Individual item status within a purchase limit check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PurchaseLimitItem {
    /// SKU or package code.
    pub package_code: String,
    /// Whether purchase limit has been reached for this specific package.
    pub limit_reached: bool,
}
