use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::common::{deserialize_inquiry_forms, CursorPagination, InquiryForm};

/// Output format for the regular pricelist endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PricelistFormat {
    /// Default compact format with cursor pagination.
    #[default]
    Compact,
    /// Legacy format which embeds inquiry fields directly and returns all matching items.
    Legacy,
}

/// Query parameters for filtering regular catalog pricelists.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PricelistParams {
    /// Items per page (1 to 1000, recommended 500 for sync).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor from previous page's next_cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Filter by product catalog server_code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// Format: compact (default) or legacy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<PricelistFormat>,
    /// Filter by brand ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<u64>,
}

/// Query parameters for filtering Smart catalog pricelists.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SmartPricelistParams {
    /// Items per page (1 to 1000, default 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor token for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Legacy brand identifier or name. Prefer brand_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Legacy category identifier or name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Search by product name or Smart SKU code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter by brand ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<u64>,
}

/// Regular product catalogue response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Catalogue {
    /// Indicates success status.
    pub success: bool,
    /// List of catalogue products.
    pub data: Vec<CatalogueProduct>,
    /// Metadata including brand list, inquiry forms, and pagination.
    pub meta: CatalogueMeta,
    /// Category details when querying by category slug.
    #[serde(default)]
    pub category: Option<CatalogueCategory>,
}

/// Product item in the regular catalogue.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CatalogueProduct {
    /// Display name of the product.
    pub name: String,
    /// Unique SKU code used when creating orders.
    pub sku_code: String,
    /// Price in Melo Coin (MC / IDR).
    pub price: f64,
    /// Category display name.
    pub category_name: String,
    /// Brand ID for filtering.
    #[serde(default)]
    pub brand_id: Option<i64>,
    /// Product type name.
    #[serde(default)]
    pub type_name: Option<String>,
    /// Display sort order of category.
    #[serde(default)]
    pub category_order: Option<i64>,
    /// Display sort order of type.
    #[serde(default)]
    pub type_order: Option<i64>,
    /// Specific catalogue server code (if applicable).
    #[serde(default)]
    pub server_code: Option<String>,
    /// Display name of server.
    #[serde(default)]
    pub server_name: Option<String>,
    /// Availability status ("active" or "out_of_stock").
    pub status: String,
    /// Lookup key in meta.inquiry_forms.
    #[serde(default)]
    pub inquiry_form_key: Option<String>,
    /// Partner-specific note or instruction.
    #[serde(default)]
    pub h2h_description: Option<String>,
    /// General product description.
    #[serde(default)]
    pub description: Option<String>,
    /// Internal product numeric ID.
    #[serde(default)]
    pub id: Option<i64>,
}

/// Catalogue metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CatalogueMeta {
    /// Brands present on this catalogue page.
    #[serde(default)]
    pub brands: Vec<BrandMeta>,
    /// Input forms mapped by inquiry_form_key.
    #[serde(default, deserialize_with = "deserialize_inquiry_forms")]
    pub inquiry_forms: HashMap<String, InquiryForm>,
    /// Pagination details (when limit query is specified).
    #[serde(default)]
    pub pagination: Option<CursorPagination>,
}

/// Brand overview inside catalogue metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrandMeta {
    /// Brand ID.
    pub id: i64,
    /// Brand name.
    #[serde(default)]
    pub name: Option<String>,
    /// URL to brand thumbnail.
    #[serde(default)]
    pub thumbnail: Option<String>,
    /// Sort order.
    #[serde(default)]
    pub order: Option<i64>,
}

/// Category information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogueCategory {
    /// Category name.
    pub name: String,
    /// Category slug.
    pub slug: String,
}

/// Brand directory entry returned by `/pricelists/categories` and `/smart-pricelists/categories`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogueBrand {
    /// Unique brand ID for catalogue filtering.
    pub brand_id: i64,
    /// Brand name.
    pub name: String,
    /// Brand slug for path `/pricelists/{category_slug}`.
    pub slug: String,
    /// Parent category details.
    #[serde(default)]
    pub category: Option<CategoryRef>,
    /// Product type code.
    #[serde(default, rename = "type")]
    pub brand_type: Option<String>,
}

/// Parent category reference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CategoryRef {
    /// Category ID.
    pub id: i64,
    /// Category name.
    pub name: String,
    /// Category slug.
    pub slug: String,
}

/// Smart product catalogue response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SmartCatalogue {
    /// Indicates success status.
    pub success: bool,
    /// List of Smart catalogue products.
    pub data: Vec<SmartCatalogueProduct>,
    /// Metadata including brands and pagination.
    pub meta: CatalogueMeta,
}

/// Product item in the Smart catalogue.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SmartCatalogueProduct {
    /// Display name of the product.
    pub name: String,
    /// Category display name.
    pub category_name: String,
    /// Brand ID.
    #[serde(default)]
    pub brand_id: Option<i64>,
    /// Product type name.
    #[serde(default)]
    pub type_name: Option<String>,
    /// Display sort order of category.
    #[serde(default)]
    pub category_order: Option<i64>,
    /// Display sort order of type.
    #[serde(default)]
    pub type_order: Option<i64>,
    /// Availability status ("active" or "out_of_stock").
    pub status: String,
    /// Lookup key in meta.inquiry_forms.
    #[serde(default)]
    pub inquiry_form_key: Option<String>,
    /// Partner-specific note or instruction.
    #[serde(default)]
    pub h2h_description: Option<String>,
    /// General product description.
    #[serde(default)]
    pub description: Option<String>,
    /// Smart SKU code to use when placing Smart Orders.
    pub smart_sku_code: String,
    /// Minimum available price across all suppliers.
    #[serde(default)]
    pub min_price: Option<f64>,
    /// Maximum price across suppliers.
    #[serde(default)]
    pub max_price: Option<f64>,
    /// Count of supplier variants available.
    #[serde(default)]
    pub variant_count: Option<i64>,
    /// List of underlying supplier variants.
    #[serde(default)]
    pub variants: Vec<SmartProductVariant>,
}

/// Underlying variant in a Smart Catalogue product.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SmartProductVariant {
    /// Supplier SKU code.
    pub sku_code: String,
    /// Variant price.
    #[serde(default)]
    pub price: Option<f64>,
    /// Status of the variant.
    pub status: String,
}
