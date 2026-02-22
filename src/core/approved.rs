use super::task::Task;

/// A policy-validated plan that is safe to hand to the executor.
///
/// `ApprovedPlan` can only be constructed by the **policy layer** — the
/// `pub(crate)` constructor prevents arbitrary creation from other modules,
/// and the private `tasks` field prevents external mutation.
///
/// The executor must accept **only** `ApprovedPlan`, never a raw [`super::Plan`].
#[derive(Debug)]
pub struct ApprovedPlan {
    tasks: Vec<Task>,
}

impl ApprovedPlan {
    /// Construct an `ApprovedPlan` from validated tasks.
    ///
    /// This is `pub(crate)` so only the policy validator can call it.
    pub(crate) fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }

    /// Borrow the approved task list (read-only).
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Consume the approved plan and return the inner task list.
    pub fn into_tasks(self) -> Vec<Task> {
        self.tasks
    }
}
