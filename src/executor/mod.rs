//! Executor stub — not implemented in v0.01.
//!
//! A real executor would iterate tasks and carry them out (create directories,
//! write files, spawn processes). This stub logs start and end only.

use tracing::info;

use crate::core::ApprovedPlan;

/// Execute an [`ApprovedPlan`].
///
/// **Stub** — logs start/end, performs no actual work.
pub async fn execute(plan: ApprovedPlan) {
    let task_count = plan.tasks().len();
    info!(task_count, "Execution start");

    // TODO: dispatch each Task variant here.

    info!(task_count, "Execution end");
}
