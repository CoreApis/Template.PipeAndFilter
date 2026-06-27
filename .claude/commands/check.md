Rode a verificação completa e reporte falhas por severidade:

1. `cargo fmt --check`
2. `cargo clippy -- -D warnings`
3. `cargo test`
4. `cargo build --release`

Demo end-to-end: `cat sample-orders.ndjson | cargo run` — confira o resumo no
stderr (linhas de entrada -> pedidos enriquecidos) e que os registros inválidos
foram descartados em cada estágio (`RUST_LOG=warn`).
