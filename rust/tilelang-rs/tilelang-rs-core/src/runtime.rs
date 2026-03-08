use std::env;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::{ffi, Result};

const TILELANG_MODULE_ENV: &str = "TILELANG_RS_TILELANG_MODULE_PATH";
const TVM_ENV: &str = "TILELANG_RS_TVM_PATH";
const RUNTIME_DIR_ENV: &str = "TILELANG_RS_RUNTIME_DIR";

static RUNTIME_STATE: OnceLock<RuntimeInitState> = OnceLock::new();

enum RuntimeInitState {
    Loaded(LoadedRuntime),
    Failed(RuntimeLoadError),
}

struct LoadedRuntime {
    _tvm: tvm_ffi::Module,
    _tilelang: tvm_ffi::Module,
    #[allow(dead_code)]
    tvm_path: String,
    #[allow(dead_code)]
    tilelang_path: String,
}

#[derive(Debug)]
pub struct RuntimeLoadError {
    missing_library: &'static str,
    attempted_paths: Vec<String>,
    env_snapshot: Vec<(&'static str, Option<String>)>,
    suggestions: Vec<String>,
    source_message: String,
}

impl RuntimeLoadError {
    fn to_tvm_error(&self) -> tvm_ffi::Error {
        tvm_ffi::Error::new(tvm_ffi::error::VALUE_ERROR, &self.format_message(), "")
    }

    fn format_message(&self) -> String {
        let mut message = String::new();
        let _ = writeln!(
            message,
            "failed to load runtime library `{}`: {}",
            self.missing_library, self.source_message
        );

        if !self.attempted_paths.is_empty() {
            let _ = writeln!(message, "attempted paths:");
            for path in &self.attempted_paths {
                let _ = writeln!(message, "  - {}", path);
            }
        }

        if !self.env_snapshot.is_empty() {
            let _ = writeln!(message, "environment:");
            for (key, value) in &self.env_snapshot {
                match value {
                    Some(value) => {
                        let _ = writeln!(message, "  - {}={}", key, value);
                    }
                    None => {
                        let _ = writeln!(message, "  - {}=<unset>", key);
                    }
                }
            }
        }

        if !self.suggestions.is_empty() {
            let _ = writeln!(message, "suggestions:");
            for suggestion in &self.suggestions {
                let _ = writeln!(message, "  - {}", suggestion);
            }
        }

        message
    }
}

pub fn ensure_runtime_loaded() -> Result<()> {
    let state = RUNTIME_STATE.get_or_init(initialize_runtime);
    match state {
        RuntimeInitState::Loaded(runtime) => {
            let _ = &runtime.tvm_path;
            let _ = &runtime.tilelang_path;
            Ok(())
        }
        RuntimeInitState::Failed(error) => Err(error.to_tvm_error()),
    }
}

fn initialize_runtime() -> RuntimeInitState {
    match try_load_runtime() {
        Ok(runtime) => RuntimeInitState::Loaded(runtime),
        Err(error) => RuntimeInitState::Failed(error),
    }
}

fn try_load_runtime() -> std::result::Result<LoadedRuntime, RuntimeLoadError> {
    let env_snapshot = env_snapshot();
    let (tvm_name, tilelang_name) = platform_library_names();
    let tvm_candidates = tvm_candidates();
    let tilelang_candidates = tilelang_candidates();

    let tvm = load_first_available(tvm_name, &tvm_candidates, &env_snapshot)?;
    let tilelang = load_first_available(tilelang_name, &tilelang_candidates, &env_snapshot)?;

    Ok(LoadedRuntime {
        _tvm: tvm.0,
        tvm_path: tvm.1,
        _tilelang: tilelang.0,
        tilelang_path: tilelang.1,
    })
}

fn load_first_available(
    missing_library: &'static str,
    candidates: &[String],
    env_snapshot: &Vec<(&'static str, Option<String>)>,
) -> std::result::Result<(tvm_ffi::Module, String), RuntimeLoadError> {
    let mut attempts = Vec::new();
    let mut last_error = None;

    for candidate in candidates {
        attempts.push(candidate.clone());
        match ffi::load_library(candidate) {
            Ok(module) => return Ok((module, candidate.clone())),
            Err(error) => last_error = Some(error.to_string()),
        }
    }

    Err(RuntimeLoadError {
        missing_library,
        attempted_paths: attempts,
        env_snapshot: env_snapshot.clone(),
        suggestions: runtime_suggestions(),
        source_message: last_error.unwrap_or_else(|| "no candidate path succeeded".to_string()),
    })
}

fn tvm_candidates() -> Vec<String> {
    let (tvm_name, _) = platform_library_names();
    let mut candidates = Vec::new();
    if let Ok(path) = env::var(TVM_ENV) {
        push_unique(&mut candidates, path);
    }
    for dir in runtime_dirs() {
        push_unique(&mut candidates, dir.join(tvm_name).display().to_string());
    }
    push_unique(&mut candidates, tvm_name.to_string());
    candidates
}

fn tilelang_candidates() -> Vec<String> {
    let (_, tilelang_name) = platform_library_names();
    let mut candidates = Vec::new();
    if let Ok(path) = env::var(TILELANG_MODULE_ENV) {
        push_unique(&mut candidates, path);
    }
    for dir in runtime_dirs() {
        push_unique(
            &mut candidates,
            dir.join(tilelang_name).display().to_string(),
        );
    }
    push_unique(&mut candidates, tilelang_name.to_string());
    candidates
}

#[cfg(test)]
fn tvm_candidates_for_test() -> Vec<String> {
    tvm_candidates()
}

fn runtime_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Ok(runtime_dir) = env::var(RUNTIME_DIR_ENV) {
        push_unique_path(&mut dirs, PathBuf::from(runtime_dir));
    }

    for dir in in_tree_dirs() {
        push_unique_path(&mut dirs, dir);
    }

    if let Ok(exe) = env::current_exe() {
        if let Some(parent) = exe.parent() {
            push_unique_path(&mut dirs, parent.to_path_buf());
            push_unique_path(&mut dirs, parent.join("lib"));
            if let Some(grand_parent) = parent.parent() {
                push_unique_path(&mut dirs, grand_parent.join("lib"));
            }
        }
    }

    if let Ok(cwd) = env::current_dir() {
        push_unique_path(&mut dirs, cwd.join("lib"));
    }

    dirs
}

fn in_tree_dirs() -> Vec<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut dirs = Vec::new();
    if let Some(repo_root) = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
    {
        dirs.push(repo_root.join("build/lib"));
    }
    dirs
}

fn env_snapshot() -> Vec<(&'static str, Option<String>)> {
    vec![
        (TILELANG_MODULE_ENV, env::var(TILELANG_MODULE_ENV).ok()),
        (TVM_ENV, env::var(TVM_ENV).ok()),
        (RUNTIME_DIR_ENV, env::var(RUNTIME_DIR_ENV).ok()),
    ]
}

fn runtime_suggestions() -> Vec<String> {
    vec![
        format!(
            "set {} and {} to explicit library paths",
            TILELANG_MODULE_ENV, TVM_ENV
        ),
        format!(
            "or set {} to a directory containing both runtime libraries",
            RUNTIME_DIR_ENV
        ),
        "for in-tree development, build the runtime libraries into `build/lib` first".to_string(),
    ]
}

fn platform_library_names() -> (&'static str, &'static str) {
    match env::consts::OS {
        "macos" => ("libtvm.dylib", "libtilelang_module.dylib"),
        "windows" => ("tvm.dll", "tilelang_module.dll"),
        _ => ("libtvm.so", "libtilelang_module.so"),
    }
}

fn push_unique(items: &mut Vec<String>, item: String) {
    if !items.iter().any(|existing| existing == &item) {
        items.push(item);
    }
}

fn push_unique_path(items: &mut Vec<PathBuf>, item: PathBuf) {
    if !items.iter().any(|existing| existing == &item) {
        items.push(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_include_library_and_attempts() {
        let error = RuntimeLoadError {
            missing_library: "libtilelang_module.so",
            attempted_paths: vec![
                "/tmp/libtilelang_module.so".into(),
                "libtilelang_module.so".into(),
            ],
            env_snapshot: vec![
                (TILELANG_MODULE_ENV, None),
                (TVM_ENV, Some("/tmp/libtvm.so".into())),
            ],
            suggestions: vec!["set TILELANG_RS_RUNTIME_DIR".into()],
            source_message: "No such file or directory".into(),
        };

        let message = error.format_message();
        assert!(message.contains("libtilelang_module.so"));
        assert!(message.contains("/tmp/libtilelang_module.so"));
        assert!(message.contains("TILELANG_RS_TVM_PATH=/tmp/libtvm.so"));
        assert!(message.contains("set TILELANG_RS_RUNTIME_DIR"));
    }

    #[test]
    fn in_tree_runtime_dir_points_to_build_lib() {
        let dirs = in_tree_dirs();
        assert!(dirs.iter().any(|dir| dir.ends_with("build/lib")));
    }

    #[test]
    fn env_override_is_checked_first() {
        let old = env::var(TVM_ENV).ok();
        unsafe {
            env::set_var(TVM_ENV, "/tmp/custom/libtvm.so");
        }
        let candidates = tvm_candidates_for_test();
        assert_eq!(
            candidates.first().map(String::as_str),
            Some("/tmp/custom/libtvm.so")
        );

        match old {
            Some(value) => unsafe {
                env::set_var(TVM_ENV, value);
            },
            None => unsafe {
                env::remove_var(TVM_ENV);
            },
        }
    }
}
