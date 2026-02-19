#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[exozora] Bootstrapping development environment..."

need_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "[exozora] Missing required command: $cmd" >&2
    return 1
  fi
}

need_cmd git
need_cmd rustup
need_cmd cargo
need_cmd rustc
need_cmd pre-commit

TOOLCHAIN="$(awk -F'"' '/channel/ {print $2}' rust-toolchain.toml)"

if [[ -z "$TOOLCHAIN" ]]; then
  echo "[exozora] Could not read toolchain channel from rust-toolchain.toml" >&2
  exit 1
fi

echo "[exozora] Ensuring Rust toolchain '$TOOLCHAIN' is installed..."
rustup toolchain install "$TOOLCHAIN"
rustup component add rustfmt clippy --toolchain "$TOOLCHAIN"

echo "[exozora] Installing pre-commit hooks..."
pre-commit install

echo "[exozora] Rust version: $(rustc --version)"
echo "[exozora] Cargo version: $(cargo --version)"
echo "[exozora] pre-commit version: $(pre-commit --version)"

echo "[exozora] Running formatter check..."
cargo fmt -- --check

echo "[exozora] Bootstrap complete. Next steps:"
echo "  1) git checkout -b feat/<short-description>"
echo "  2) cargo clippy --all-targets --all-features -- -D warnings"
echo "  3) cargo test"
