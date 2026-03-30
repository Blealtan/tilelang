#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: gen_stub.sh [options]

Options:
  --out-dir <path>        Output crate directory. Default: rust/tilelang-rs/tilelang-rs-ffi
  --crate-name <name>     Generated crate package name. Default: tilelang-rs-ffi
  --dlls <a;b;...>        Semicolon-separated DLL list passed to tvm-ffi-stubgen
  --tilelang-dll <path>   Override TileLang shared library path
  --tvm-dll <path>        Override TVM shared library path
  --tvm-ffi-path <path>   Override tvm-ffi path recorded in generated Cargo.toml
  --check                 Generate into a temp dir and fail if output differs
  --help                  Show this help message

Environment overrides:
  TILELANG_RS_OUT_DIR
  TILELANG_RS_INIT_CRATE
  TILELANG_RS_DLLS
  TILELANG_RS_TILELANG_SO
  TILELANG_RS_TVM_SO
  TILELANG_RS_TVM_FFI_PATH
EOF
}

gen_log() {
  printf '[tilelang-rs/gen_stub] %s\n' "$*"
}

gen_die() {
  printf '[tilelang-rs/gen_stub] ERROR: %s\n' "$*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "${path}" ]] || gen_die "Missing required file: ${path}"
}

ensure_stubgen_env() {
  local setup_script="${REPO_ROOT}/tmp/setup_stubgen_env.sh"

  if command -v tvm-ffi-config >/dev/null 2>&1; then
    return 0
  fi
  [[ "${TILELANG_RS_SKIP_ENV_SETUP:-0}" == "1" ]] && \
    gen_die "tvm-ffi-config is unavailable and TILELANG_RS_SKIP_ENV_SETUP=1"
  require_file "${setup_script}"
  gen_log "Bootstrapping tvm-ffi environment via ${setup_script}"
  # shellcheck source=/dev/null
  source "${setup_script}"
  command -v tvm-ffi-config >/dev/null 2>&1 || \
    gen_die "tvm-ffi-config is still unavailable after sourcing ${setup_script}"
}

rewrite_generated_cargo_toml() {
  local cargo_toml="$1"
  local tvm_ffi_path="$2"

  python3 - "$cargo_toml" "$tvm_ffi_path" <<'PY'
from pathlib import Path
import os
import re
import sys

cargo_toml = Path(sys.argv[1])
tvm_ffi_path = Path(sys.argv[2]).resolve()
rel = os.path.relpath(tvm_ffi_path, cargo_toml.parent).replace(os.sep, "/")
text = cargo_toml.read_text()
updated = re.sub(
    r'(\[dependencies\.tvm-ffi\]\s*path\s*=\s*")[^"]+(")',
    rf'\1{rel}\2',
    text,
    count=1,
    flags=re.MULTILINE,
)
if text == updated:
    raise SystemExit("failed to rewrite generated tvm-ffi path")
cargo_toml.write_text(updated)
PY
}

sync_generated_dependency_path_from_target() {
  local target_cargo_toml="$1"
  local generated_cargo_toml="$2"

  python3 - "$target_cargo_toml" "$generated_cargo_toml" <<'PY'
from pathlib import Path
import re
import sys

target = Path(sys.argv[1]).read_text()
generated_path = Path(sys.argv[2])
generated = generated_path.read_text()

match = re.search(r'(\[dependencies\.tvm-ffi\]\s*path\s*=\s*")([^"]+)(")', target, re.MULTILINE)
if not match:
    raise SystemExit("failed to extract target tvm-ffi path")

replacement = match.group(2)
updated = re.sub(
    r'(\[dependencies\.tvm-ffi\]\s*path\s*=\s*")[^"]+(")',
    rf'\1{replacement}\2',
    generated,
    count=1,
    flags=re.MULTILINE,
)
generated_path.write_text(updated)
PY
}

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
REPO_ROOT="$(cd -- "${WORKSPACE_ROOT}/../.." && pwd)"
TVM_FFI_RUST_ROOT="${REPO_ROOT}/3rdparty/tvm/3rdparty/tvm-ffi/rust"
DEFAULT_OUT_DIR="${WORKSPACE_ROOT}/tilelang-rs-ffi"
DEFAULT_TILELANG_SO="${REPO_ROOT}/build/lib/libtilelang.so"
DEFAULT_TVM_SO="${REPO_ROOT}/build/lib/libtvm.so"
DEFAULT_TVM_FFI_PATH="${REPO_ROOT}/3rdparty/tvm/3rdparty/tvm-ffi/rust/tvm-ffi"

OUT_DIR="${TILELANG_RS_OUT_DIR:-${DEFAULT_OUT_DIR}}"
INIT_CRATE="${TILELANG_RS_INIT_CRATE:-tilelang-rs-ffi}"
TILELANG_SO="${TILELANG_RS_TILELANG_SO:-${DEFAULT_TILELANG_SO}}"
TVM_SO="${TILELANG_RS_TVM_SO:-${DEFAULT_TVM_SO}}"
TVM_FFI_PATH="${TILELANG_RS_TVM_FFI_PATH:-${DEFAULT_TVM_FFI_PATH}}"
DLLS="${TILELANG_RS_DLLS:-}"
CHECK_MODE=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir)
      OUT_DIR="$2"
      shift 2
      ;;
    --crate-name)
      INIT_CRATE="$2"
      shift 2
      ;;
    --dlls)
      DLLS="$2"
      shift 2
      ;;
    --tilelang-dll)
      TILELANG_SO="$2"
      shift 2
      ;;
    --tvm-dll)
      TVM_SO="$2"
      shift 2
      ;;
    --tvm-ffi-path)
      TVM_FFI_PATH="$2"
      shift 2
      ;;
    --check)
      CHECK_MODE=1
      shift
      ;;
    --help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      gen_die "Unknown argument: $1"
      ;;
  esac
done

export RUSTUP_HOME="${RUSTUP_HOME:-${HOME}/.rustup}"
export CARGO_HOME="${CARGO_HOME:-${HOME}/.cargo}"
export PATH="${CARGO_HOME}/bin:${PATH}"

command -v cargo >/dev/null 2>&1 || gen_die "cargo is unavailable on PATH"
ensure_stubgen_env

require_file "${TVM_FFI_PATH}/Cargo.toml"
if [[ -z "${DLLS}" ]]; then
  require_file "${TILELANG_SO}"
  require_file "${TVM_SO}"
  DLLS="${TILELANG_SO};${TVM_SO}"
fi

TARGET_OUT_DIR="${OUT_DIR}"
WORK_OUT_DIR="${OUT_DIR}"
TMP_DIR=""
cleanup() {
  if [[ -n "${TMP_DIR}" && -d "${TMP_DIR}" ]]; then
    rm -rf "${TMP_DIR}"
  fi
}
trap cleanup EXIT

if [[ "${CHECK_MODE}" == "1" ]]; then
  [[ -d "${TARGET_OUT_DIR}" ]] || gen_die "Cannot check freshness because output dir does not exist: ${TARGET_OUT_DIR}"
  TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/tilelang-rs-stubgen-check.XXXXXX")"
  WORK_OUT_DIR="${TMP_DIR}/tilelang-rs-ffi"
fi

gen_log "Generating stub crate into ${WORK_OUT_DIR}"
(cd "${TVM_FFI_RUST_ROOT}" && cargo run -p tvm-ffi-stubgen -- \
  "${WORK_OUT_DIR}" \
  --dlls "${DLLS}" \
  --init-prefix tl \
  --init-prefix ir \
  --init-prefix tir \
  --init-prefix script \
  --init-prefix relax \
  --init-prefix transform \
  --init-prefix target \
  --init-prefix te \
  --init-crate "${INIT_CRATE}" \
  --tvm-ffi-path "${TVM_FFI_PATH}" \
  --overwrite)

rewrite_generated_cargo_toml "${WORK_OUT_DIR}/Cargo.toml" "${TVM_FFI_PATH}"

if [[ "${CHECK_MODE}" == "1" ]]; then
  sync_generated_dependency_path_from_target "${TARGET_OUT_DIR}/Cargo.toml" "${WORK_OUT_DIR}/Cargo.toml"
  if ! diff -ruN "${TARGET_OUT_DIR}" "${WORK_OUT_DIR}" >/dev/null; then
    gen_log "Generated output is stale; diff follows"
    diff -ruN "${TARGET_OUT_DIR}" "${WORK_OUT_DIR}" || true
    exit 1
  fi
  gen_log "Stubgen output is fresh"
  exit 0
fi

gen_log "Stub crate updated at ${WORK_OUT_DIR}"
