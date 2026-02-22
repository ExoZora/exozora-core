use super::task::Task;

/// A sequence of tasks produced by the planner, awaiting policy validation.
///
/// `Plan` represents **untrusted intent**. The inner `tasks` field is private
/// to prevent external mutation — access is controlled via [`Plan::tasks`]
/// (borrowed) and [`Plan::into_tasks`] (consuming).
#[derive(Debug, Clone)]
pub struct Plan {
    tasks: Vec<Task>,
}

impl Plan {
    /// Create a new plan from a list of tasks.
    pub fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }

    /// Borrow the task list (read-only).
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Consume the plan and return the inner task list.
    pub fn into_tasks(self) -> Vec<Task> {
        self.tasks
    }
}
