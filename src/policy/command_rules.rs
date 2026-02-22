//! Command-level validation rules.
//!
//! Each function in this module inspects a single `RunCommand` task and
//! returns a [`PolicyError`] if it violates a constraint.

use crate::policy::PolicyError;

/// Known network utilities that must be blocked.
const NETWORK_COMMANDS: &[&str] = &[
    "curl",
    "wget",
    "nc",
    "netcat",
    "ncat",
    "ssh",
    "scp",
    "sftp",
    "ftp",
    "tftp",
    "nmap",
    "ping",
    "ping6",
    "traceroute",
    "dig",
    "nslookup",
    "host",
    "telnet",
];

/// Reject if the command is `sudo` or any argument is `sudo`.
pub fn check_sudo(command: &str, args: &[String]) -> Result<(), PolicyError> {
    let base = std::path::Path::new(command)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(command);

    if base.eq_ignore_ascii_case("sudo") {
        return Err(PolicyError::SudoDetected(command.to_owned()));
    }

    for arg in args {
        if arg.eq_ignore_ascii_case("sudo") {
            return Err(PolicyError::SudoDetected(format!(
                "{command} (sudo in args)"
            )));
        }
    }

    Ok(())
}

/// Reject if the command's base executable name matches a known network utility.
///
/// Comparison is **case-insensitive** and uses [`Path::file_name`] to strip
/// any leading directory components (e.g. `/usr/bin/curl` → `curl`).
pub fn check_network(command: &str) -> Result<(), PolicyError> {
    let base = std::path::Path::new(command)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(command);

    for &net_cmd in NETWORK_COMMANDS {
        if base.eq_ignore_ascii_case(net_cmd) {
            return Err(PolicyError::NetworkOperation(command.to_owned()));
        }
    }

    Ok(())
}
