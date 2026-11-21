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

    let cache_key = format!("all:{:?}:{:?}:{:?}:{:?}", query.cursor, query.limit, query.brand_id, query.server);
    let ttl = std::time::Duration::from_secs(300);

    {
        let cache = state.cache.read().await;
        if let Some(entry) = cache.get(&cache_key) {
            if entry.fetched_at.elapsed() < ttl {
                return Ok(Json(entry.value.clone()));
            }
        }
    }

    let params = PricelistParams {
        cursor: query.cursor,
        limit: query.limit.or(Some(500)),
        brand_id: query.brand_id,
        server: query.server,
        format: None,
    };

    let catalogue = match state.client.get_pricelist(Some(params)).await {
        Ok(c) => c,
        Err(e) => {
            let cache = state.cache.read().await;
            if let Some(entry) = cache.get(&cache_key) {
                tracing::warn!("serving stale full catalogue cache due to upstream error: {}", e);
                return Ok(Json(entry.value.clone()));
            }
            return Err(e.into());
        }
    };
    let val = serde_json::to_value(catalogue).map_err(|e| AppError::Internal(e.to_string()))?;

    {
        let mut cache = state.cache.write().await;
        cache.insert(cache_key, crate::state::CacheEntry {
            value: val.clone(),
            fetched_at: std::time::Instant::now(),
        });
    }

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

    let cache_key = format!("{brand_slug}:{:?}:{:?}:{:?}:{:?}", query.cursor, query.limit, query.brand_id, query.server);
    let ttl = std::time::Duration::from_secs(300);

    {
        let cache = state.cache.read().await;
        if let Some(entry) = cache.get(&cache_key) {
            if entry.fetched_at.elapsed() < ttl {
                return Ok(Json(entry.value.clone()));
            }
        }
    }

    let params = PricelistParams {
        cursor: query.cursor,
        limit: query.limit.or(Some(500)),
        brand_id: query.brand_id,
        server: query.server,
        format: None,
    };

    let catalogue = match state.client.get_pricelist_by_brand(&brand_slug, Some(params)).await {
        Ok(c) => c,
        Err(e) => {
            let cache = state.cache.read().await;
            if let Some(entry) = cache.get(&cache_key) {
                tracing::warn!("serving stale category catalogue cache for {} due to upstream error: {}", brand_slug, e);
                return Ok(Json(entry.value.clone()));
            }
            return Err(e.into());
        }
    };
    let val = serde_json::to_value(catalogue).map_err(|e| AppError::Internal(e.to_string()))?;

    {
        let mut cache = state.cache.write().await;
        cache.insert(cache_key, crate::state::CacheEntry {
            value: val.clone(),
            fetched_at: std::time::Instant::now(),
        });
    }

    Ok(Json(val))
}
