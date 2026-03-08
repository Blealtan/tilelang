use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use tilelang_rs_core::{runtime, BuilderContext, Result};

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

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(prefix: &str) -> std::io::Result<Self> {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{unique}"));
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(unix)]
fn link_or_copy(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    std::os::unix::fs::symlink(src, dst)
}

#[cfg(not(unix))]
fn link_or_copy(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    fs::copy(src, dst).map(|_| ())
}

#[test]
fn runtime_loads_from_runtime_dir_override() -> Result<()> {
    let lib_dir = repo_root().join("build/lib");
    let temp_dir = TempDir::new("tilelang-rs-runtime-dir").expect("temp dir should be creatable");
    let tvm_src = lib_dir.join("libtvm.so");
    let tilelang_src = lib_dir.join("libtilelang_module.so");
    let tvm_dst = temp_dir.path.join("libtvm.so");
    let tilelang_dst = temp_dir.path.join("libtilelang_module.so");

    link_or_copy(&tvm_src, &tvm_dst).expect("tvm runtime should be linked");
    link_or_copy(&tilelang_src, &tilelang_dst).expect("tilelang runtime should be linked");

    let _runtime_dir_guard = EnvGuard::set(
        "TILELANG_RS_RUNTIME_DIR",
        temp_dir.path.display().to_string(),
    );

    runtime::ensure_runtime_loaded()?;
    let ctx = BuilderContext::new("runtime_dir_override")?;
    let _module = ctx.finish_ir_module();
    Ok(())
}
