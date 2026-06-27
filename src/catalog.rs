//! A side data source the enrichment filter reads from. In production this might
//! be a database or another service; the filter only depends on `lookup`.

use std::collections::HashMap;

#[derive(Clone)]
pub struct Catalog {
    items: HashMap<String, (String, i64)>, // sku -> (name, price_cents)
}

impl Catalog {
    pub fn new(items: HashMap<String, (String, i64)>) -> Self {
        Self { items }
    }

    pub fn sample() -> Self {
        let mut items = HashMap::new();
        items.insert("SKU-KB".to_string(), ("Keyboard".to_string(), 9_900));
        items.insert("SKU-MS".to_string(), ("Mouse".to_string(), 4_500));
        items.insert("SKU-MON".to_string(), ("Monitor".to_string(), 129_900));
        Self::new(items)
    }

    pub fn lookup(&self, sku: &str) -> Option<(String, i64)> {
        self.items.get(sku).cloned()
    }
}
