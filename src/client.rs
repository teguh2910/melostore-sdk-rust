use reqwest::{Client as HttpClient, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::auth::Auth;
use crate::config::MelostoreConfig;
use crate::error::{MelostoreError, Result};
use crate::models::account::{
    CheckNicknameRequest, CheckPurchaseLimitRequest, GamesResponse, NicknameGame, NicknamePlan,
    NicknameResult, PurchaseLimitResult,
};
use crate::models::bulk::{
    BulkHistoryParams, BulkOrder, BulkPage, ChildPage, CreateBulkOrderRequest,
};
use crate::models::catalog::{
    Catalogue, CatalogueBrand, PricelistParams, SmartCatalogue, SmartPricelistParams,
};
use crate::models::common::{ApiErrorBody, ApiResponse};
use crate::models::profile::{Balance, Profile};
use crate::models::transaction::{
    CreateSmartTransactionRequest, CreateTransactionRequest, CustomerAction,
    SmartCreateAcknowledgement, SubmitTwoFactorRequest, Transaction, TransactionStatusData,
};
use crate::webhook::{parse_and_verify_webhook, WebhookPayload};

/// Asynchronous client for interacting with the Melo H2H Public API.
#[derive(Debug, Clone)]
pub struct MelostoreClient {
    http: HttpClient,
    config: MelostoreConfig,
    auth: Auth,
}

impl MelostoreClient {
    /// Creates a new `MelostoreClient` with the given API key and Secret key using default production settings.
    pub fn new(api_key: impl Into<String>, secret_key: impl Into<String>) -> Result<Self> {
        let config = MelostoreConfig::new(api_key, secret_key);
        Self::with_config(config)
    }

    /// Creates a new `MelostoreClient` with a custom [`MelostoreConfig`].
    pub fn with_config(config: MelostoreConfig) -> Result<Self> {
        let http = HttpClient::builder()
            .timeout(config.timeout)
            .build()
            .map_err(MelostoreError::Http)?;

        let auth = Auth::new(&config.api_key, &config.secret_key);

        Ok(Self { http, config, auth })
    }

    /// Returns a reference to the active client configuration.
    pub fn config(&self) -> &MelostoreConfig {
        &self.config
    }

    // --- Helper Request Utilities ---

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url, path)
    }

    fn apply_auth(&self, builder: RequestBuilder) -> Result<RequestBuilder> {
        self.auth.apply(builder)
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();
        let headers = response.headers().clone();

        if status.is_success() {
            let body_bytes = response.bytes().await.map_err(MelostoreError::Http)?;
            let data: T = serde_json::from_slice(&body_bytes).map_err(|e| {
                let raw_body = String::from_utf8_lossy(&body_bytes).into_owned();
                tracing::error!("Failed to parse response JSON: {e}. Raw body: {raw_body}");
                MelostoreError::Json(e)
            })?;
            return Ok(data);
        }

        let raw_bytes = response.bytes().await.unwrap_or_default();
        let raw_text = String::from_utf8_lossy(&raw_bytes).into_owned();

        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = headers
                .get("Retry-After")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());

            let message = if let Ok(err) = serde_json::from_str::<ApiErrorBody>(&raw_text) {
                err.message
            } else {
                "Rate limit exceeded".to_string()
            };

            return Err(MelostoreError::RateLimited {
                retry_after,
                message,
            });
        }

        if let Ok(api_err) = serde_json::from_str::<ApiErrorBody>(&raw_text) {
            return Err(MelostoreError::Api {
                status_code: status.as_u16(),
                message: api_err.message,
                error_code: api_err.error_code,
                errors: api_err.errors,
                raw_body: Some(raw_text),
            });
        }

        Err(MelostoreError::Api {
            status_code: status.as_u16(),
            message: format!("HTTP error: {}", status),
            error_code: None,
            errors: None,
            raw_body: Some(raw_text),
        })
    }

    // ==========================================
    // Profile API Endpoints
    // ==========================================

    /// Retrieves partner profile and tier details (`GET /api/v1/h2h/profile`).
    pub async fn get_profile(&self) -> Result<Profile> {
        let req = self.http.get(self.url("/api/v1/h2h/profile"));
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<Profile> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    /// Retrieves current H2H wallet balance and USD rate estimate (`GET /api/v1/h2h/profile/balance`).
    pub async fn get_balance(&self) -> Result<Balance> {
        let req = self.http.get(self.url("/api/v1/h2h/profile/balance"));
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<Balance> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    // ==========================================
    // Catalog API Endpoints
    // ==========================================

    /// Retrieves the regular product catalogue (`GET /api/v1/h2h/pricelists`).
    pub async fn get_pricelist(&self, params: Option<PricelistParams>) -> Result<Catalogue> {
        let mut req = self.http.get(self.url("/api/v1/h2h/pricelists"));
        if let Some(p) = params {
            req = req.query(&p);
        }
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        self.handle_response(res).await
    }

    /// Retrieves regular product prices filtered by brand slug (`GET /api/v1/h2h/pricelists/{category_slug}`).
    pub async fn get_pricelist_by_brand(
        &self,
        category_slug: &str,
        params: Option<PricelistParams>,
    ) -> Result<Catalogue> {
        let path = format!("/api/v1/h2h/pricelists/{}", category_slug);
        let mut req = self.http.get(self.url(&path));
        if let Some(p) = params {
            req = req.query(&p);
        }
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        self.handle_response(res).await
    }

    /// Retrieves brand directory for the regular catalogue (`GET /api/v1/h2h/pricelists/categories`).
    pub async fn get_catalogue_brands(&self) -> Result<Vec<CatalogueBrand>> {
        let req = self.http.get(self.url("/api/v1/h2h/pricelists/categories"));
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<Vec<CatalogueBrand>> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    /// Retrieves the Smart Order product catalogue (`GET /api/v1/h2h/smart-pricelists`).
    pub async fn get_smart_pricelist(
        &self,
        params: Option<SmartPricelistParams>,
    ) -> Result<SmartCatalogue> {
        let mut req = self.http.get(self.url("/api/v1/h2h/smart-pricelists"));
        if let Some(p) = params {
            req = req.query(&p);
        }
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        self.handle_response(res).await
    }

    /// Retrieves Smart Order prices filtered by brand slug (`GET /api/v1/h2h/smart-pricelists/{category_slug}`).
    pub async fn get_smart_pricelist_by_brand(
        &self,
        category_slug: &str,
        params: Option<SmartPricelistParams>,
    ) -> Result<SmartCatalogue> {
        let path = format!("/api/v1/h2h/smart-pricelists/{}", category_slug);
        let mut req = self.http.get(self.url(&path));
        if let Some(p) = params {
            req = req.query(&p);
        }
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        self.handle_response(res).await
    }

    /// Retrieves brand directory for the Smart catalogue (`GET /api/v1/h2h/smart-pricelists/categories`).
    pub async fn get_smart_catalogue_brands(&self) -> Result<Vec<CatalogueBrand>> {
        let req = self
            .http
            .get(self.url("/api/v1/h2h/smart-pricelists/categories"));
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<Vec<CatalogueBrand>> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    // ==========================================
    // Transactions API Endpoints
    // ==========================================

    /// Places a regular single order (`POST /api/v1/h2h/transaction`).
    pub async fn create_transaction(
        &self,
        mut request: CreateTransactionRequest,
    ) -> Result<Transaction> {
        if request.sandbox_mode.is_none() {
            request.sandbox_mode = self.config.sandbox_mode;
        }

        let req = self.http.post(self.url("/api/v1/h2h/transaction"));
        let req = self.apply_auth(req)?;
        let res = req.json(&request).send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<Transaction> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    /// Places a Smart Order with maximum bid protection (`POST /api/v1/h2h/smart-transaction`).
    pub async fn create_smart_transaction(
        &self,
        mut request: CreateSmartTransactionRequest,
    ) -> Result<SmartCreateAcknowledgement> {
        if request.sandbox_mode.is_none() {
            request.sandbox_mode = self.config.sandbox_mode;
        }

        let req = self.http.post(self.url("/api/v1/h2h/smart-transaction"));
        let req = self.apply_auth(req)?;
        let res = req.json(&request).send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<SmartCreateAcknowledgement> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    /// Queries order status by transaction UUID or `buyer_trx_id` (`GET /api/v1/h2h/transaction/{id}`).
    pub async fn get_transaction(&self, id_or_buyer_trx_id: &str) -> Result<TransactionStatusData> {
        let path = format!("/api/v1/h2h/transaction/{}", id_or_buyer_trx_id);
        let req = self.http.get(self.url(&path));
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<TransactionStatusData> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    /// Submits customer two-factor verification code (`POST /api/v1/h2h/transactions/2fa`).
    pub async fn submit_two_factor(
        &self,
        request: SubmitTwoFactorRequest,
    ) -> Result<CustomerAction> {
        let req = self.http.post(self.url("/api/v1/h2h/transactions/2fa"));
        let req = self.apply_auth(req)?;
        let res = req.json(&request).send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<CustomerAction> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    // ==========================================
    // Bulk Orders API Endpoints
    // ==========================================

    /// Places a batch of repeated orders (`POST /api/v1/h2h/bulk-orders`).
    pub async fn create_bulk_order(
        &self,
        mut request: CreateBulkOrderRequest,
    ) -> Result<BulkOrder> {
        if request.sandbox_mode.is_none() {
            request.sandbox_mode = self.config.sandbox_mode;
        }

        let req = self.http.post(self.url("/api/v1/h2h/bulk-orders"));
        let req = self.apply_auth(req)?;
        let res = req.json(&request).send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<BulkOrder> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    /// Lists bulk order history with pagination and status filters (`GET /api/v1/h2h/bulk-orders`).
    pub async fn get_bulk_orders(&self, params: Option<BulkHistoryParams>) -> Result<BulkPage> {
        let mut req = self.http.get(self.url("/api/v1/h2h/bulk-orders"));
        if let Some(p) = params {
            req = req.query(&p);
        }
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<BulkPage> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    /// Retrieves detailed status and counters for a bulk order (`GET /api/v1/h2h/bulk-orders/{id}`).
    pub async fn get_bulk_order(&self, bulk_order_id: &str) -> Result<BulkOrder> {
        let path = format!("/api/v1/h2h/bulk-orders/{}", bulk_order_id);
        let req = self.http.get(self.url(&path));
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<BulkOrder> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    /// Lists individual child transactions belonging to a bulk order (`GET /api/v1/h2h/bulk-orders/{id}/transactions`).
    pub async fn get_bulk_transactions(
        &self,
        bulk_order_id: &str,
        page: Option<u32>,
    ) -> Result<ChildPage> {
        let path = format!("/api/v1/h2h/bulk-orders/{}/transactions", bulk_order_id);
        let mut req = self.http.get(self.url(&path));
        if let Some(pg) = page {
            let mut query = HashMap::new();
            query.insert("page", pg);
            req = req.query(&query);
        }
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<ChildPage> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    /// Downloads the official PDF receipt/invoice for a bulk order (`GET /api/v1/h2h/bulk-orders/{id}/invoice`).
    pub async fn get_bulk_invoice_pdf(&self, bulk_order_id: &str) -> Result<Vec<u8>> {
        let path = format!("/api/v1/h2h/bulk-orders/{}/invoice", bulk_order_id);
        let req = self.http.get(self.url(&path));
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;

        let status = res.status();
        if !status.is_success() {
            let raw_bytes = res.bytes().await.unwrap_or_default();
            let raw_text = String::from_utf8_lossy(&raw_bytes).into_owned();

            if let Ok(api_err) = serde_json::from_str::<ApiErrorBody>(&raw_text) {
                return Err(MelostoreError::Api {
                    status_code: status.as_u16(),
                    message: api_err.message,
                    error_code: api_err.error_code,
                    errors: api_err.errors,
                    raw_body: Some(raw_text),
                });
            }

            return Err(MelostoreError::Api {
                status_code: status.as_u16(),
                message: format!("Failed to download invoice: HTTP {}", status),
                error_code: None,
                errors: None,
                raw_body: Some(raw_text),
            });
        }

        let bytes = res.bytes().await.map_err(MelostoreError::Http)?;
        Ok(bytes.to_vec())
    }

    // ==========================================
    // Accounts (Nickname & Limits) API Endpoints
    // ==========================================

    /// Lists supported games and form specifications for nickname checking (`GET /api/v1/h2h/check-nickname/games`).
    pub async fn get_nickname_games(&self) -> Result<Vec<NicknameGame>> {
        let req = self.http.get(self.url("/api/v1/h2h/check-nickname/games"));
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        let data: GamesResponse = self.handle_response(res).await?;
        Ok(data.games)
    }

    /// Retrieves daily free quota and paid rates for nickname checks (`GET /api/v1/h2h/check-nickname/plan`).
    pub async fn get_nickname_plan(&self) -> Result<NicknamePlan> {
        let req = self.http.get(self.url("/api/v1/h2h/check-nickname/plan"));
        let req = self.apply_auth(req)?;
        let res = req.send().await.map_err(MelostoreError::Http)?;
        let wrapper: ApiResponse<NicknamePlan> = self.handle_response(res).await?;
        Ok(wrapper.data)
    }

    /// Verifies customer player username/nickname (`POST /api/v1/h2h/check-nickname`).
    pub async fn check_nickname(&self, request: CheckNicknameRequest) -> Result<NicknameResult> {
        let req = self.http.post(self.url("/api/v1/h2h/check-nickname"));
        let req = self.apply_auth(req)?;
        let res = req.json(&request).send().await.map_err(MelostoreError::Http)?;
        self.handle_response(res).await
    }

    /// Verifies Mobile Legends Weekly Diamond Pass purchase limits (`POST /api/v1/h2h/mobile-legends/purchase-limit`).
    pub async fn check_mlbb_purchase_limit(
        &self,
        request: CheckPurchaseLimitRequest,
    ) -> Result<PurchaseLimitResult> {
        let req = self
            .http
            .post(self.url("/api/v1/h2h/mobile-legends/purchase-limit"));
        let req = self.apply_auth(req)?;
        let res = req.json(&request).send().await.map_err(MelostoreError::Http)?;
        self.handle_response(res).await
    }

    // ==========================================
    // Webhook Verification Utility
    // ==========================================

    /// Verifies the HMAC-SHA256 signature using the client's configured `webhook_secret` and parses the event.
    ///
    /// Returns `Err(MelostoreError::MissingRequiredField)` if `webhook_secret` was not configured on this client.
    pub fn verify_webhook(&self, raw_body: &[u8], signature_hex: &str) -> Result<WebhookPayload> {
        let secret = self
            .config
            .webhook_secret
            .as_deref()
            .ok_or_else(|| MelostoreError::MissingRequiredField("webhook_secret is not configured on client".into()))?;

        parse_and_verify_webhook(secret, raw_body, signature_hex)
    }
}
