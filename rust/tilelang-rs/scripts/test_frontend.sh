#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
REPO_ROOT="$(cd -- "${WORKSPACE_ROOT}/../.." && pwd)"

export RUSTUP_HOME="${RUSTUP_HOME:-${HOME}/.rustup}"
export CARGO_HOME="${CARGO_HOME:-${HOME}/.cargo}"
export PATH="${CARGO_HOME}/bin:${PATH}"

cd "${WORKSPACE_ROOT}"

"${SCRIPT_DIR}/check_stub_fresh.sh"
cargo fmt --all --check
cargo test -p tilelang-rs-core -- --nocapture
cargo test -p tilelang-rs-macros -- --nocapture
cargo test -p tilelang-rs -- --nocapture

echo "[tilelang-rs/test_frontend] all checks passed"
