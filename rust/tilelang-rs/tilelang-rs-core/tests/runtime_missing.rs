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
fn runtime_falls_back_when_explicit_paths_are_missing() {
    let _tvm_guard = EnvGuard::set(
        "TILELANG_RS_TVM_PATH",
        "/definitely/missing/libtvm.so".to_string(),
    );
    let _tilelang_guard = EnvGuard::set(
        "TILELANG_RS_TILELANG_MODULE_PATH",
        "/definitely/missing/libtilelang.so".to_string(),
    );
    let _runtime_dir_guard = EnvGuard::set(
        "TILELANG_RS_RUNTIME_DIR",
        "/definitely/missing/runtime-dir".to_string(),
    );

    tilelang_rs_core::runtime::ensure_runtime_loaded()
        .expect("missing explicit paths should still fall back to in-tree runtime");
}
