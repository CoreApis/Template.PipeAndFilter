//! The concrete filters for the order-ingestion pipeline. Each one implements
//! the same `Filter` trait and is independently testable and reorderable.

use uuid::Uuid;

use crate::catalog::Catalog;
use crate::domain::{EnrichedOrder, Line, RawOrder};
use crate::pipeline::Filter;

/// String -> RawOrder. Drops lines that are not valid JSON.
pub struct ParseFilter;

impl Filter for ParseFilter {
    type In = String;
    type Out = RawOrder;

    fn process(&self, line: String) -> Vec<RawOrder> {
        match serde_json::from_str::<RawOrder>(&line) {
            Ok(order) => vec![order],
            Err(err) => {
                tracing::warn!(%err, line, "parse: dropping malformed record");
                vec![]
            }
        }
    }
}

/// RawOrder -> RawOrder. Drops records that violate basic invariants.
pub struct ValidateFilter;

impl Filter for ValidateFilter {
    type In = RawOrder;
    type Out = RawOrder;

    fn process(&self, order: RawOrder) -> Vec<RawOrder> {
        let valid = !order.customer_id.trim().is_empty()
            && !order.items.is_empty()
            && order.items.iter().all(|i| i.quantity > 0);
        if valid {
            vec![order]
        } else {
            tracing::warn!(
                customer_id = order.customer_id,
                "validate: dropping invalid order"
            );
            vec![]
        }
    }
}

/// RawOrder -> EnrichedOrder. Enriches each line with name+price from the
/// catalog and computes the total. Drops orders referencing an unknown SKU.
pub struct EnrichFilter {
    catalog: Catalog,
}

impl EnrichFilter {
    pub fn new(catalog: Catalog) -> Self {
        Self { catalog }
    }
}

impl Filter for EnrichFilter {
    type In = RawOrder;
    type Out = EnrichedOrder;

    fn process(&self, order: RawOrder) -> Vec<EnrichedOrder> {
        let mut lines = Vec::with_capacity(order.items.len());
        for item in &order.items {
            let Some((name, unit_price_cents)) = self.catalog.lookup(&item.sku) else {
                tracing::warn!(sku = item.sku, "enrich: unknown SKU, dropping order");
                return vec![];
            };
            lines.push(Line {
                sku: item.sku.clone(),
                name,
                quantity: item.quantity,
                unit_price_cents,
            });
        }
        let total_cents = lines.iter().map(|l| l.unit_price_cents * l.quantity).sum();
        vec![EnrichedOrder {
            id: order.order_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            customer_id: order.customer_id,
            lines,
            total_cents,
        }]
    }
}
