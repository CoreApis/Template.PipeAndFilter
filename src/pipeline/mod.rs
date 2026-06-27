//! The generic pipe-and-filter machinery.
//!
//! A `Filter` is the uniform interface every stage implements: it turns one
//! input into zero-or-more outputs (zero = filtered out, one = mapped, many =
//! expanded). `spawn_stage` connects filters with **bounded** channels, which
//! gives each stage its own task (concurrency) and natural **backpressure** —
//! a slow stage makes its predecessor block instead of exhausting memory.

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;

/// Uniform interface of a filter. Stateless or self-contained; reusable and
/// reorderable as long as the in/out types line up.
pub trait Filter: Send + Sync {
    type In: Send + 'static;
    type Out: Send + 'static;

    /// Returns 0..n outputs for one input. An empty result drops the item.
    fn process(&self, item: Self::In) -> Vec<Self::Out>;
}

/// Run one filter as its own task, reading from `rx` and writing to `tx`.
/// When the upstream channel closes, the task ends and drops `tx`, which closes
/// the channel for the next stage — the shutdown signal propagates down the pipe.
pub fn spawn_stage<F>(filter: F, mut rx: Receiver<F::In>, tx: Sender<F::Out>) -> JoinHandle<()>
where
    F: Filter + 'static,
{
    tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            for out in filter.process(item) {
                if tx.send(out).await.is_err() {
                    return; // downstream gone — stop early
                }
            }
        }
    })
}

/// Default channel capacity. Small enough that backpressure is observable.
pub const CHANNEL_CAPACITY: usize = 64;
