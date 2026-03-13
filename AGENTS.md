# Agents

## Cursor Cloud specific instructions

### Rust frontend (primary development scope)

The Rust frontend lives under `rust/tilelang-rs/` and contains 4 crates: `tilelang-rs-ffi` (generated), `tilelang-rs-core`, `tilelang-rs-macros`, and `tilelang-rs`.

**Rust toolchain**: The `tvm-ffi` dependency requires Rust **nightly** (the `Option<T>` `TryFrom` coherence issue prevents building on stable 1.82–1.83). Use `rustup default nightly` before building.

**C++ runtime**: Rust runtime tests require `libtvm.so` and `libtilelang_module.so` in `build/lib/`. Build them with:
```bash
mkdir -p build && cd build
cmake .. -DUSE_CUDA=OFF -DCMAKE_C_COMPILER=gcc-13 -DCMAKE_CXX_COMPILER=g++-13 -G Ninja
ninja -j$(nproc)
```
The Clang default on this image looks for gcc-14 C++ headers (only gcc-13 is installed), so you must specify `gcc-13`/`g++-13` explicitly.

**`tvm-ffi-config`**: Needed by the `tvm-ffi` build script. Install via `pip install --break-system-packages apache-tvm-ffi`. Ensure `~/.local/bin` is on PATH.

**libstdc++ symlink**: Clang linker needs `/usr/lib/x86_64-linux-gnu/libstdc++.so` which may not exist; create it via:
```bash
sudo ln -sf /usr/lib/gcc/x86_64-linux-gnu/13/libstdc++.so /usr/lib/x86_64-linux-gnu/libstdc++.so
```

**Validation commands** (see also `rust/tilelang-rs/scripts/test_frontend.sh` and the SKILL at `.cursor/skills/tilelang-rs-frontend-dev/SKILL.md`):
```bash
cd rust/tilelang-rs
cargo fmt --all              # format
cargo test -p tilelang-rs-core -- --nocapture
cargo test -p tilelang-rs-macros -- --nocapture
cargo test -p tilelang-rs -- --nocapture
cargo run --example add_ir   # hello-world demo
```

**Known nightly caveats**:
- `cargo fmt --all --check` may show import-ordering diffs in generated `tilelang-rs-ffi` code; this is cosmetic due to nightly rustfmt sorting.
- `tilelang-rs-macros` trybuild tests may show stderr span-width mismatches on nightly vs the stable version used to snapshot `.stderr` files; runtime IR tests still pass.

### Python / C++ (secondary)

- See `CONTRIBUTING.md` for full Python dev setup.
- Pre-commit hooks are configured in `.pre-commit-config.yaml` (ruff, clang-format, codespell, etc.).
- `ruff check` and `cargo fmt` are the primary lint tools for Python and Rust respectively.

### Git submodules

Always ensure submodules are initialized: `git submodule update --init --recursive`. The Rust workspace depends on `tvm-ffi` from `3rdparty/tvm/3rdparty/tvm-ffi/rust/tvm-ffi`.
