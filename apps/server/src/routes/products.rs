use crate::{error::AppError, AppState};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use melostore_sdk_rust::models::catalog::PricelistParams;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Default)]
pub struct ProductQuery {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub brand_id: Option<u64>,
    pub server: Option<String>,
}

pub async fn get_products(
    State(state): State<AppState>,
    Query(query): Query<ProductQuery>,
) -> Result<Json<Value>, AppError> {
    if state.cfg.mock_mode {
        return Ok(Json(crate::mock::get_mock_catalogue(None)));
    }

    let params = PricelistParams {
        cursor: query.cursor,
        limit: query.limit.or(Some(500)),
        brand_id: query.brand_id,
        server: query.server,
        format: None,
    };

    let catalogue = state.client.get_pricelist(Some(params)).await?;
    let val = serde_json::to_value(catalogue).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(val))
}

pub async fn get_products_by_category(
    State(state): State<AppState>,
    Path(brand_slug): Path<String>,
    Query(query): Query<ProductQuery>,
) -> Result<Json<Value>, AppError> {
    if state.cfg.mock_mode {
        return Ok(Json(crate::mock::get_mock_catalogue(Some(&brand_slug))));
    }

    let params = PricelistParams {
        cursor: query.cursor,
        limit: query.limit.or(Some(500)),
        brand_id: query.brand_id,
        server: query.server,
        format: None,
    };

    let catalogue = state
        .client
        .get_pricelist_by_brand(&brand_slug, Some(params))
        .await?;
    let val = serde_json::to_value(catalogue).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(val))
}
