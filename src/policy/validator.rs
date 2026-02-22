//! Validation pipeline — the single entry-point for policy enforcement.
//!
//! Consumes a [`Plan`] and either returns an [`ApprovedPlan`] (all tasks
//! passed) or a [`PolicyError`] (first violation found).

use std::path::Path;

use crate::core::{ApprovedPlan, Plan, Task};
use crate::policy::{command_rules, path_rules, PolicyError};

/// Validate every task in `plan` against ExoZora's safety policy.
///
/// # Contract
/// - The `Plan` is **consumed** to prevent re-use of unvalidated intent.
/// - `working_dir` should be pre-canonicalised by the caller.
/// - Returns on **first** violation (fail-fast).
/// - Has **zero side effects** — no I/O, no logging, no mutation.
///
/// # Future-proofing
/// The current signature `(Plan, &Path)` is designed to evolve into
/// `(Plan, &PolicyContext)` where `PolicyContext` may carry user identity,
/// capability tokens, execution mode, and security profiles.
pub fn validate(plan: Plan, working_dir: &Path) -> Result<ApprovedPlan, PolicyError> {
    for task in plan.tasks() {
        match task {
            Task::RunCommand { command, args } => {
                command_rules::check_sudo(command, args)?;
                command_rules::check_network(command)?;
            }

            Task::CreateDir { path } => {
                path_rules::check_confinement(path, working_dir)?;
            }

            Task::WriteFile { path, .. } => {
                path_rules::check_confinement(path, working_dir)?;
            }
        }
    }

    // All tasks passed — promote to trusted contract.
    Ok(ApprovedPlan::new(plan.into_tasks()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Plan, Task};
    use std::path::{Path, PathBuf};

    fn wd() -> &'static Path {
        Path::new("/home/user/project")
    }

    #[test]
    fn empty_plan_is_approved() {
        let plan = Plan::new(vec![]);
        assert!(validate(plan, wd()).is_ok());
    }

    #[test]
    fn safe_tasks_are_approved() {
        let plan = Plan::new(vec![
            Task::CreateDir {
                path: PathBuf::from("build"),
            },
            Task::WriteFile {
                path: PathBuf::from("build/output.txt"),
                contents: "hello".into(),
            },
            Task::RunCommand {
                command: "echo".into(),
                args: vec!["hello".into()],
            },
        ]);
        assert!(validate(plan, wd()).is_ok());
    }

    #[test]
    fn sudo_is_rejected() {
        let plan = Plan::new(vec![Task::RunCommand {
            command: "sudo".into(),
            args: vec!["rm".into(), "-rf".into(), "/".into()],
        }]);
        assert!(matches!(
            validate(plan, wd()),
            Err(PolicyError::SudoDetected(_))
        ));
    }

    #[test]
    fn network_command_is_rejected() {
        let plan = Plan::new(vec![Task::RunCommand {
            command: "curl".into(),
            args: vec!["https://evil.com".into()],
        }]);
        assert!(matches!(
            validate(plan, wd()),
            Err(PolicyError::NetworkOperation(_))
        ));
    }

    #[test]
    fn path_escape_is_rejected() {
        let plan = Plan::new(vec![Task::WriteFile {
            path: PathBuf::from("../../etc/passwd"),
            contents: "pwned".into(),
        }]);
        assert!(matches!(
            validate(plan, wd()),
            Err(PolicyError::PathEscape(_))
        ));
    }
}
