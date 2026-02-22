//! Planner stub — not implemented in v0.01.
//!
//! A real planner would parse `input` and emit a sequence of [`crate::core::Task`]s.
//! This stub returns an empty [`Plan`] so the rest of the pipeline can be exercised.

use tracing::info;

use crate::core::Plan;

/// Produce a [`Plan`] from raw user input.
///
/// **Stub** — always returns an empty plan.
pub fn plan(input: &str) -> Plan {
    info!(input = %input, "Planner received input (stub — returning empty plan)");
    Plan::new(vec![])
}
