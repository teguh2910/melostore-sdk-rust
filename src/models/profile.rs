use serde::{Deserialize, Serialize};

/// Partner account profile information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    /// Account UUID.
    pub id: String,
    /// Partner name.
    pub name: String,
    /// Registered email.
    pub email: String,
    /// Account tier configuration.
    pub tier: Tier,
    /// Current H2H balance in Melo Coin (MC / IDR).
    pub h2h_balance: f64,
    /// Whether default transaction mode is sandbox.
    pub is_sandbox_mode: bool,
    /// Total transaction volume in the last 30 days.
    pub total_volume_30_days: f64,
    /// Configured webhook URL for callback notifications.
    #[serde(default)]
    pub webhook_url: Option<String>,
    /// Account creation timestamp.
    pub created_at: String,
    /// Base USD/IDR exchange rate.
    pub usd_idr_rate: f64,
    /// Display USD/IDR rate estimate.
    #[serde(default)]
    pub display_usd_idr_rate: Option<f64>,
    /// Last update timestamp of the display rate.
    #[serde(default)]
    pub display_rate_updated_at: Option<String>,
}

/// Partner account tier limits and information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tier {
    /// Tier display name (e.g. "Basic", "Gold", "VIP").
    pub name: String,
    /// Internal tier code (e.g. "basic", "gold").
    pub code: String,
    /// Allowed requests per minute.
    pub rate_limit: u32,
    /// Maximum allowed IP whitelist entries.
    pub max_ips: u32,
}

/// Balance response information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Balance {
    /// Current H2H balance in Melo Coin (MC / IDR).
    pub h2h_balance: f64,
    /// Whether default transaction mode is sandbox.
    pub is_sandbox_mode: bool,
    /// Current USD/IDR conversion rate.
    pub usd_idr_rate: f64,
    /// Display USD/IDR rate estimate.
    #[serde(default)]
    pub display_usd_idr_rate: Option<f64>,
    /// Last update timestamp of the display rate.
    #[serde(default)]
    pub display_rate_updated_at: Option<String>,
    /// Approximate balance in USD.
    #[serde(default)]
    pub h2h_balance_usd: Option<f64>,
}
