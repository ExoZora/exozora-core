use std::path::PathBuf;

/// The atomic operations ExoZora can perform.
///
/// Each variant represents a single side-effect the executor may carry out.
/// No validation logic lives here — that belongs in the policy layer.
#[derive(Debug, Clone)]
pub enum Task {
    /// Create a directory at the given path.
    CreateDir { path: PathBuf },

    /// Write `contents` to a file at the given path.
    WriteFile { path: PathBuf, contents: String },

    /// Run an OS command with optional arguments.
    RunCommand { command: String, args: Vec<String> },
}
