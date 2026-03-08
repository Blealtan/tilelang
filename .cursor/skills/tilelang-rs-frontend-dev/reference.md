# Reference

## Key Files

- `rust/tilelang-rs/tilelang-rs-core/src/lib.rs`
- `rust/tilelang-rs/tilelang-rs-core/src/runtime.rs`
- `rust/tilelang-rs/tilelang-rs-macros/src/lib.rs`
- `rust/tilelang-rs/tilelang-rs/src/lib.rs`
- `rust/tilelang-rs/scripts/gen_stub.sh`
- `rust/tilelang-rs/scripts/check_stub_fresh.sh`
- `rust/tilelang-rs/scripts/test_frontend.sh`

## Important Test Files

- `rust/tilelang-rs/tilelang-rs-core/tests/manual_ir_smoke.rs`
- `rust/tilelang-rs/tilelang-rs-core/tests/loop_dsl_smoke.rs`
- `rust/tilelang-rs/tilelang-rs-core/tests/pred_if_smoke.rs`
- `rust/tilelang-rs/tilelang-rs-core/tests/alloc_var_smoke.rs`
- `rust/tilelang-rs/tilelang-rs-core/tests/runtime_env_paths.rs`
- `rust/tilelang-rs/tilelang-rs-core/tests/runtime_dir_override.rs`
- `rust/tilelang-rs/tilelang-rs-core/tests/runtime_missing.rs`
- `rust/tilelang-rs/tilelang-rs-macros/tests/precheck.rs`
- `rust/tilelang-rs/tilelang-rs-macros/tests/transform.rs`
- `rust/tilelang-rs/tilelang-rs-macros/tests/runtime_ir.rs`
- `rust/tilelang-rs/tilelang-rs/tests/macro_loops_if.rs`
- `rust/tilelang-rs/tilelang-rs/tests/stubgen_check.rs`

## Runtime Autoload Contract

- Public entry: `tilelang_rs_core::runtime::ensure_runtime_loaded()`
- Automatic call site: `BuilderContext::new(...)`
- Environment overrides:
  - `TILELANG_RS_TILELANG_MODULE_PATH`
  - `TILELANG_RS_TVM_PATH`
  - `TILELANG_RS_RUNTIME_DIR`
- Fallback order:
  - explicit paths
  - runtime dir
  - in-tree `build/lib`
  - executable-adjacent dirs
  - soname fallback

## Macro Lowering Contract

- `#[tl_ir]` first runs mutable-state precheck.
- It wraps the function to return `Result<ffi::ir::IRModule>`.
- It enters `BuilderContext::with_tir_prim_func(...)`.
- `for` lowers to `for_each(...)`.
- `if` lowers to `if_stmt(...)`.
- logical predicates lower through `pred::*`.

## Common Commands

```bash
cargo fmt --all
cargo test -p tilelang-rs-core -- --nocapture
cargo test -p tilelang-rs-macros -- --nocapture
cargo test -p tilelang-rs -- --nocapture
./scripts/test_frontend.sh
./scripts/check_stub_fresh.sh
```
