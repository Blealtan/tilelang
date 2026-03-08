use std::env;
use std::process::Command;

fn update_ld_library_path(lib_dir: &str) {
    let os_env_var = match env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("windows") => "PATH",
        Ok("macos") => "DYLD_LIBRARY_PATH",
        Ok("linux") => "LD_LIBRARY_PATH",
        _ => "",
    };
    if os_env_var.is_empty() {
        return;
    }

    let current_val = env::var(os_env_var).unwrap_or_else(|_| String::new());
    let separator = if os_env_var == "PATH" { ";" } else { ":" };
    let new_ld_path = if current_val.is_empty() {
        lib_dir.to_string()
    } else {
        format!("{}{}{}", current_val, separator, lib_dir)
    };
    println!("cargo:rustc-env={}={}", os_env_var, new_ld_path);
}

fn main() {
    let output = Command::new("tvm-ffi-config")
        .arg("--libdir")
        .output()
        .expect("Failed to run tvm-ffi-config");
    if !output.status.success() {
        panic!("tvm-ffi-config --libdir failed");
    }

    let lib_dir = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 output from tvm-ffi-config")
        .trim()
        .to_string();
    if lib_dir.is_empty() {
        panic!("tvm-ffi-config returned empty library path");
    }

    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=tvm_ffi");
    println!("cargo:rustc-link-arg-tests=-Wl,-rpath,{}", lib_dir);
    update_ld_library_path(&lib_dir);
}
