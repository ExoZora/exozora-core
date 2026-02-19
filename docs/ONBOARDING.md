# ExoZora Onboarding Guide

This document is the single source of truth for setting up a contributor workstation for `exozora-core`.

> If onboarding takes more than 30 minutes, the setup is considered too complex and should be simplified.

## 1) Required Tools

Install the following before contributing:

- **Git** (latest stable)
- **Rust toolchain** pinned by `rust-toolchain.toml`
- **Cargo** (installed with Rust)
- **pre-commit**
- **A POSIX shell** (`bash` recommended)

Recommended verification commands:

```bash
git --version
rustc --version
cargo --version
pre-commit --version
```

## 2) Clone Repository

```bash
git clone <your-repo-url> exozora-core
cd exozora-core
```

## 3) Install Project Dependencies (Standardized Setup)

Run the bootstrap script from repository root:

```bash
./scripts/bootstrap.sh
```

This script will:
- Verify required tools are available.
- Ensure Rust toolchain from `rust-toolchain.toml` is installed.
- Install `rustfmt` and `clippy` components.
- Install Git pre-commit hooks.

## 4) Install pre-commit (if missing)

If `pre-commit` is not installed, use one of:

### pipx (recommended)
```bash
pipx install pre-commit
```

### pip
```bash
python3 -m pip install --user pre-commit
```

### Homebrew (macOS/Linux)
```bash
brew install pre-commit
```

Then run:

```bash
pre-commit --version
```

## 5) Run CI Locally (Before Opening PR)

Run the same baseline checks locally:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo check
```

## 6) Create a Branch

Never work directly on `main`.

```bash
git checkout -b feat/<short-description>
```

Examples:
- `feat/onboarding-docs`
- `fix/planner-validation-typo`

## 7) Commit and Push

```bash
git add -A
git commit -m "docs: add onboarding and bootstrap workflow"
git push -u origin feat/<short-description>
```

## 8) Pull Request Workflow Rules

- No direct commits to `main`.
- PR is required for all changes.
- Minimum **1 approval** before merge.
- Keep PRs small and focused.
- Architecture changes require an RFC under `docs/rfc/`.

## 9) Coding Standards

- Follow Rust idioms and repository structure.
- Keep modules focused and composable.
- Add/update documentation with each structural change.
- Run local CI checks before opening PR.
- Avoid mixing refactors with feature work in the same PR.

## 10) Definition of Done (Contributor Flow)

Every teammate should be able to:

1. Clone repository.
2. Run `./scripts/bootstrap.sh`.
3. Create a feature branch.
4. Make and commit changes.
5. Open a PR.
6. Pass local CI checks.
