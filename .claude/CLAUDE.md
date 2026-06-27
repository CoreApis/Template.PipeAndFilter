# Template.PipeAndFilter — Contexto Claude

## Stack
Rust (edition 2021) · tokio · serde/serde_json · tracing · uuid.

## Arquitetura
Pipe-and-Filter: dados fluem por **filtros independentes** ligados por **pipes**
(canais tokio mpsc, **limitados** → backpressure). Cada filtro implementa a
mesma trait `Filter` (interface uniforme) e é reordenável/reutilizável.

```
NDJSON line -> ParseFilter -> ValidateFilter -> EnrichFilter -> EnrichedOrder
src/pipeline/  -> trait Filter + spawn_stage (runner com canais limitados)
src/filters/   -> Parse / Validate / Enrich
src/domain.rs  -> RawOrder, EnrichedOrder, Line
src/catalog.rs -> fonte lateral para enrichment (sku -> nome/preço)
src/lib.rs     -> run_orders_pipeline (monta o pipeline)
src/main.rs    -> CLI (stdin/arquivo -> stdout)
```

## Convenções
- `Filter::process(item) -> Vec<Out>`: 0 = descarta (filtra), 1 = mapeia, n = expande.
- Cada estágio é uma task própria; canais limitados (`CHANNEL_CAPACITY`) dão backpressure.
- Fechar o canal de origem propaga o shutdown estágio a estágio.
- Dinheiro em centavos. Commits: Conventional Commits.

## Evitar
- Estágios compartilharem estado mutável fora dos canais.
- Canais ilimitados (perde a propriedade de backpressure).

## Comandos frequentes
- `cargo test` · `cargo clippy -- -D warnings` · `cargo fmt`
- `cat sample-orders.ndjson | cargo run` ou `cargo run -- sample-orders.ndjson`
- `RUST_LOG=warn` mostra os descartes de cada filtro.
