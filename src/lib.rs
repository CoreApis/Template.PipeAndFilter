//! Pipe-and-filter template: a concurrent order-ingestion pipeline.
//!
//! ```text
//! NDJSON line ──▶ Parse ──▶ Validate ──▶ Enrich ──▶ EnrichedOrder
//!               (drop bad   (drop      (price from
//!                JSON)       invalid)   catalog)
//! ```
//! Stages are independent tasks linked by bounded channels (pipes).

pub mod catalog;
pub mod domain;
pub mod filters;
pub mod pipeline;

use tokio::sync::mpsc::channel;

use crate::catalog::Catalog;
use crate::domain::EnrichedOrder;
use crate::filters::{EnrichFilter, ParseFilter, ValidateFilter};
use crate::pipeline::{spawn_stage, CHANNEL_CAPACITY};

/// Assemble and run the order pipeline over `lines`, collecting the enriched
/// output. The stages run concurrently; bounded channels apply backpressure.
pub async fn run_orders_pipeline(lines: Vec<String>, catalog: Catalog) -> Vec<EnrichedOrder> {
    let (src_tx, src_rx) = channel::<String>(CHANNEL_CAPACITY);
    let (parsed_tx, parsed_rx) = channel(CHANNEL_CAPACITY);
    let (valid_tx, valid_rx) = channel(CHANNEL_CAPACITY);
    let (out_tx, mut out_rx) = channel::<EnrichedOrder>(CHANNEL_CAPACITY);

    let stages = (
        spawn_stage(ParseFilter, src_rx, parsed_tx),
        spawn_stage(ValidateFilter, parsed_rx, valid_tx),
        spawn_stage(EnrichFilter::new(catalog), valid_rx, out_tx),
    );

    // Source: feed lines into the pipe, then close it.
    let feeder = tokio::spawn(async move {
        for line in lines {
            if src_tx.send(line).await.is_err() {
                break;
            }
        }
    });

    // Sink: collect everything the last stage emits.
    let mut collected = Vec::new();
    while let Some(order) = out_rx.recv().await {
        collected.push(order);
    }

    let _ = tokio::join!(feeder, stages.0, stages.1, stages.2);
    collected
}
