use pipe_and_filter::{catalog::Catalog, run_orders_pipeline};

#[tokio::test]
async fn drops_bad_records_and_enriches_valid_ones() {
    let lines = vec![
        "{ this is not json".to_string(),                                          // parse drops
        r#"{"customerId":"c-1","items":[]}"#.to_string(),                          // validate drops (empty)
        r#"{"customerId":"c-1","items":[{"sku":"NOPE","quantity":1}]}"#.to_string(), // enrich drops (unknown sku)
        r#"{"customerId":"c-1","items":[{"sku":"SKU-KB","quantity":2}]}"#.to_string(), // ok -> 19800
        r#"{"customerId":"c-2","items":[{"sku":"SKU-MS","quantity":1},{"sku":"SKU-MON","quantity":1}]}"#.to_string(), // ok -> 134400
    ];

    let mut out = run_orders_pipeline(lines, Catalog::sample()).await;
    out.sort_by_key(|o| o.total_cents);

    assert_eq!(out.len(), 2);
    assert_eq!(out[0].total_cents, 19_800);
    assert_eq!(out[1].total_cents, 4_500 + 129_900);
    assert_eq!(out[0].lines[0].name, "Keyboard");
}

#[tokio::test]
async fn empty_input_yields_no_output() {
    let out = run_orders_pipeline(vec![], Catalog::sample()).await;
    assert!(out.is_empty());
}
