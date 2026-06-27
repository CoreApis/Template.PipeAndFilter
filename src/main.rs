//! CLI driver: reads NDJSON orders from a file argument (or stdin), runs them
//! through the pipeline, writes enriched orders as NDJSON to stdout, and prints
//! a summary to stderr.
//!
//!   cat sample-orders.ndjson | cargo run
//!   cargo run -- sample-orders.ndjson

use std::io::{self, Read};

use pipe_and_filter::{catalog::Catalog, run_orders_pipeline};

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".into()))
        .with_writer(io::stderr)
        .init();

    let input = match std::env::args().nth(1) {
        Some(path) => std::fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let lines: Vec<String> = input
        .lines()
        .map(str::to_string)
        .filter(|l| !l.trim().is_empty())
        .collect();
    let total_in = lines.len();

    let orders = run_orders_pipeline(lines, Catalog::sample()).await;

    for order in &orders {
        println!("{}", serde_json::to_string(order).expect("serialize"));
    }
    eprintln!(
        "pipeline: {total_in} input line(s) -> {} enriched order(s)",
        orders.len()
    );
    Ok(())
}
