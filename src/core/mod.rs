//! Core domain model for ExoZora.
//!
//! Defines the data types shared across all layers:
//! [`Task`], [`Plan`] (untrusted), and [`ApprovedPlan`] (trusted).

pub mod approved;
pub mod plan;
pub mod task;

// Re-export for ergonomic imports.
pub use approved::ApprovedPlan;
pub use plan::Plan;
pub use task::Task;
