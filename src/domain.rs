//! The records that flow through the pipe. Each stage consumes one type and
//! produces the next, so the type system documents the shape of the dataflow.

use serde::{Deserialize, Serialize};

/// A line of raw input as parsed from NDJSON.
#[derive(Debug, Clone, Deserialize)]
pub struct RawItem {
    pub sku: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawOrder {
    #[serde(default)]
    pub order_id: Option<String>,
    pub customer_id: String,
    pub items: Vec<RawItem>,
}

/// A priced, named line after enrichment from the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub sku: String,
    pub name: String,
    pub quantity: i64,
    pub unit_price_cents: i64,
}

/// The pipeline's output record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedOrder {
    pub id: String,
    pub customer_id: String,
    pub lines: Vec<Line>,
    pub total_cents: i64,
}
