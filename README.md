# ExoZora Core

## Project Overview
**ExoZora v0.01 is a sandboxed, policy-gated task execution runtime baseline** intended to lock architectural expectations before full implementation lands.

v0.01 enforces (as the design contract for this baseline):
- Working directory confinement
- No `sudo` usage
- No known network CLI tools
- Deterministic task execution

This is **NOT**:
- A syscall-level sandbox
- A container runtime
- A capability-based system (yet)

If scope is not stated precisely, readers can incorrectly assume stronger guarantees than v0.01 provides.

## Architecture

### Runtime Flow Diagram

```text
Input → Planner → Plan → Policy → ApprovedPlan → Executor
```

### Trust and Ownership Rules
- **Planner produces untrusted intent** (`Plan`).
- **Policy promotes intent to trusted output** by validating and transforming `Plan` into `ApprovedPlan`.
- **Executor only accepts `ApprovedPlan`** and must never execute raw planner intent.
- **`ApprovedPlan` is policy-owned** and acts as the single executable boundary artifact.

This separation is mandatory for contributors so trust boundaries remain explicit in code and review.

## v0.01 Command Grammar (Formal)

```text
COMMAND := statement (";" statement)*

statement :=
    "create dir" PATH
  | "write file" PATH STRING
  | "run" COMMAND [ARGS...]
```

### Parsing Notes
- Quoted strings are supported.
- `;` separates statements and preserves deterministic left-to-right execution order.
- Nested `write file` paths trigger implicit parent directory injection when required.

This is documented explicitly to avoid future ambiguity around automatic directory creation.

## Security Guarantees

### v0.01 Guarantees
- No path escape outside `working_dir`
- No direct or indirect `sudo` invocation (substring hardened)
- No known network utilities
- Deterministic sequential execution
- Fail-fast on first error

### Known Limitations
- No syscall filtering
- No environment variable sanitization
- No container isolation
- Network is still possible via interpreters (`python`, `node`, etc.)

Security claims must not exceed code-enforced boundaries.

## Testing
Run:

```bash
cargo test
cargo test --test stress_tests
```

Test coverage focus for v0.01 should include:
- Unicode paths
- Multi-command parsing
- Deep nesting
- Hardened `sudo` detection

> Note: this repository is architecture-first and currently scaffolded; command references above define the expected validation surface for the v0.01 baseline.

## Dependencies

### Linux (WSL or Native)
Required:

```bash
sudo apt update
sudo apt install build-essential
```

Why this is required:
- `gcc`
- `g++`
- `ld`
- `make`

Rust relies on a working linker and C/C++ build toolchain.

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Confirm:

```bash
rustc -vV
```

Expected target includes:

```text
x86_64-unknown-linux-gnu
```

### Windows (MSVC Target)
If using native Windows (not WSL), install:
- Visual Studio Build Tools 2019+ (or 2022)
- Desktop development with C++
- Windows SDK

Verify linker availability:

```powershell
where link
```

If missing, the MSVC linker is not installed correctly.

### Alternative: Windows GNU Target
If using the GNU target:

```powershell
rustup default stable-x86_64-pc-windows-gnu
```

Then install MinGW. MSVC remains the recommended target for stability.

## Cargo Dependencies (v0.01)
Dependencies to document explicitly:
- `clap` → CLI parsing
- `tokio` → async runtime/executor orchestration
- `tracing` → structured logging instrumentation
- `tracing-subscriber` → log formatting/filter configuration
- `thiserror` → structured error type definitions

No mystery dependencies should be introduced without documentation.

## Versioning Policy
**Version: v0.01 (Locked Baseline)**

All behavioral changes must occur in `v0.02+` branches. v0.01 is intended to remain a stable architectural and policy reference point.

