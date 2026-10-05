# Template.PipeAndFilter — filtros uniformes ligados por canais limitados

Stack: Rust (edition 2021) · tokio · serde/serde_json · tracing · uuid.

## Arquitetura
`NDJSON line -> ParseFilter -> ValidateFilter -> EnrichFilter -> EnrichedOrder`
```
src/pipeline/   -> trait Filter + spawn_stage (runner com canais limitados)
src/filters/    -> Parse / Validate / Enrich
src/domain.rs   -> RawOrder, EnrichedOrder, Line
src/catalog.rs  -> fonte lateral do enrichment (sku -> nome/preço)
src/lib.rs      -> run_orders_pipeline (monta o pipeline)
src/main.rs     -> CLI (stdin/arquivo -> stdout)
```

## Invariantes
- `Filter::process(item) -> Vec<Out>`: 0 = descarta, 1 = mapeia, n = expande. Filtro novo implementa a mesma trait e é reordenável.
- Cada estágio é uma task própria com canal **limitado** (`CHANNEL_CAPACITY`) — canal ilimitado destrói o backpressure.
- Estágio não compartilha estado mutável fora dos canais. Fechar o canal de origem propaga o shutdown.
- Dinheiro em centavos.

## Comandos
`cargo test` · `cargo clippy -- -D warnings` · `cargo fmt` · `cat sample-orders.ndjson | cargo run` · `RUST_LOG=warn` mostra os descartes por filtro
