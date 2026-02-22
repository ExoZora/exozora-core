//! Policy layer — deterministic, stateless, side-effect-free validation.
//!
//! The public interface is [`validate`] (re-exported from [`validator`])
//! and [`PolicyError`].

pub mod command_rules;
pub mod path_rules;
pub mod validator;

use std::path::PathBuf;
use thiserror::Error;

// Re-export the validation entry-point for ergonomic use.
pub use validator::validate;

/// All reasons a plan can be rejected during policy validation.
#[derive(Debug, Error)]
pub enum PolicyError {
    /// A command or its arguments contain `sudo`.
    #[error("sudo detected in command: {0}")]
    SudoDetected(String),

    /// A command is a known network utility.
    #[error("network operation detected: {0}")]
    NetworkOperation(String),

    /// A filesystem path escapes the designated working directory.
    #[error("path escapes working directory: {}", .0.display())]
    PathEscape(PathBuf),
}
