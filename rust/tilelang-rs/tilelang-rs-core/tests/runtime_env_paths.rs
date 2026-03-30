use std::path::PathBuf;

use tilelang_rs_core::{BuilderContext, Result, runtime};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tilelang-rs-core should live under rust/tilelang-rs")
        .parent()
        .expect("tilelang-rs workspace should live under rust/")
        .parent()
        .expect("rust/ should live under repository root")
        .to_path_buf()
}

struct EnvGuard {
    key: &'static str,
    previous: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: String) -> Self {
        let previous = std::env::var(key).ok();
        unsafe {
            std::env::set_var(key, value);
        }
        Self { key, previous }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.previous {
            Some(value) => unsafe {
                std::env::set_var(self.key, value);
            },
            None => unsafe {
                std::env::remove_var(self.key);
            },
        }
    }
}

#[test]
fn runtime_loads_from_explicit_env_paths_and_is_idempotent() -> Result<()> {
    let lib_dir = repo_root().join("build/lib");
    let _tvm_guard = EnvGuard::set(
        "TILELANG_RS_TVM_PATH",
        lib_dir.join("libtvm.so").display().to_string(),
    );
    let _tilelang_guard = EnvGuard::set(
        "TILELANG_RS_TILELANG_MODULE_PATH",
        lib_dir.join("libtilelang.so").display().to_string(),
    );

    runtime::ensure_runtime_loaded()?;
    runtime::ensure_runtime_loaded()?;

    let ctx = BuilderContext::new("runtime_env_paths")?;
    let _module = ctx.finish_ir_module();
    Ok(())
}
