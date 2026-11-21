use crate::{error::AppError, AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use melostore_sdk_rust::models::transaction::CreateTransactionRequest;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CreateOrderPayload {
    pub sku_code: Option<String>,
    pub sku: Option<String>,
    pub customer_target: Option<String>,
    pub target_uid: Option<String>,
    pub customer_target_zone: Option<String>,
    pub target_zone: Option<String>,
    pub buyer_trx_id: Option<String>,
    pub ref_id: Option<String>,
    pub sandbox_mode: Option<bool>,
    pub max_price: Option<i64>,
    pub additional_data: Option<HashMap<String, Value>>,
}

pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderPayload>,
) -> Result<Json<Value>, AppError> {
    let sku_code = payload
        .sku_code
        .or(payload.sku)
        .ok_or_else(|| AppError::BadRequest("Field 'sku_code' is required".into()))?;

    let customer_target = payload
        .customer_target
        .or(payload.target_uid)
        .ok_or_else(|| AppError::BadRequest("Field 'customer_target' is required".into()))?;

    let buyer_trx_id = payload
        .buyer_trx_id
        .or(payload.ref_id)
        .ok_or_else(|| AppError::BadRequest("Field 'buyer_trx_id' is required".into()))?;

    let customer_target_zone = payload.customer_target_zone.or(payload.target_zone);

    let mut req = CreateTransactionRequest::new(sku_code, customer_target, buyer_trx_id);
    req.customer_target_zone = customer_target_zone;
    req.sandbox_mode = payload.sandbox_mode.or(Some(state.cfg.sandbox_mode));
    req.max_price = payload.max_price;
    req.additional_data = payload.additional_data;

    let trx = state.client.create_transaction(req).await?;
    let val = serde_json::to_value(trx).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(val))
}

pub async fn get_order_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let trx = state.client.get_transaction(&id).await?;
    let val = serde_json::to_value(trx).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(val))
}
