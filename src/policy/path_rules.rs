//! Path-level validation rules.
//!
//! Ensures that all filesystem paths in a plan resolve within the
//! designated working directory, using **lexical normalisation** only
//! (no `canonicalize`, no requirement for the path to exist on disk).

use std::path::{Component, Path, PathBuf};

use crate::policy::PolicyError;

/// Reject if `path` resolves outside `working_dir` after lexical normalisation.
///
/// # Algorithm
/// 1. If `path` is relative, join it with `working_dir`.
/// 2. Collapse `.` and `..` components lexically.
/// 3. Check that the result `starts_with` `working_dir`.
pub fn check_confinement(path: &Path, working_dir: &Path) -> Result<(), PolicyError> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        working_dir.join(path)
    };

    let normalised = normalise(&absolute);

    if !normalised.starts_with(working_dir) {
        return Err(PolicyError::PathEscape(path.to_path_buf()));
    }

    Ok(())
}

/// Lexically resolve a path — collapse `.` and `..` without touching the
/// filesystem.
fn normalise(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {} // skip "."
            Component::ParentDir => {
                out.pop(); // go up one level
            }
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn relative_inside_is_ok() {
        let wd = Path::new("/home/user/project");
        assert!(check_confinement(Path::new("src/main.rs"), wd).is_ok());
    }

    #[test]
    fn parent_escape_is_rejected() {
        let wd = Path::new("/home/user/project");
        assert!(check_confinement(Path::new("../../etc/passwd"), wd).is_err());
    }

    #[test]
    fn absolute_inside_is_ok() {
        let wd = Path::new("/home/user/project");
        assert!(check_confinement(Path::new("/home/user/project/foo"), wd).is_ok());
    }

    #[test]
    fn absolute_outside_is_rejected() {
        let wd = Path::new("/home/user/project");
        assert!(check_confinement(Path::new("/etc/shadow"), wd).is_err());
    }
}
