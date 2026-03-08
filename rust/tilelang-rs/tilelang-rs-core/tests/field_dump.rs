// This is a standalone test - run it as a cargo test in tilelang-rs-core
// to inspect ForFrame field schemas via the real TVM FFI.

#[test]
fn dump_forframe_field_schemas() {
    tilelang_rs_core::runtime::ensure_runtime_loaded().expect("runtime");

    let type_keys = [
        "script.ir_builder.tir.ForFrame",
        "script.ir_builder.tir.TIRFrame",
        "tl.KernelLaunchFrame",
    ];

    for type_key in &type_keys {
        let key_ba = tvm_ffi::tvm_ffi_sys::TVMFFIByteArray {
            data: type_key.as_ptr(),
            size: type_key.len(),
        };
        let mut tindex: i32 = 0;
        let ret = unsafe {
            tvm_ffi::tvm_ffi_sys::TVMFFITypeKeyToIndex(&key_ba as *const _, &mut tindex as *mut _)
        };
        if ret != 0 {
            println!("{type_key}: not registered");
            continue;
        }
        let info = unsafe { tvm_ffi::tvm_ffi_sys::TVMFFIGetTypeInfo(tindex) };
        if info.is_null() {
            println!("{type_key}: null info");
            continue;
        }
        let info = unsafe { &*info };
        println!("\n{type_key} (index={tindex}, depth={}):", info.type_depth);
        let fields = unsafe { std::slice::from_raw_parts(info.fields, info.num_fields as usize) };
        for f in fields {
            let name = if f.name.data.is_null() {
                "?".to_string()
            } else {
                unsafe {
                    std::str::from_utf8(std::slice::from_raw_parts(
                        f.name.data as *const u8,
                        f.name.size,
                    ))
                    .unwrap_or("?")
                    .to_string()
                }
            };
            let meta = if f.metadata.data.is_null() {
                "".to_string()
            } else {
                unsafe {
                    std::str::from_utf8(std::slice::from_raw_parts(
                        f.metadata.data as *const u8,
                        f.metadata.size,
                    ))
                    .unwrap_or("?")
                    .to_string()
                }
            };
            println!(
                "  field: {name:20}  offset={:3}  size={:2}  schema={meta:?}",
                f.offset, f.size
            );
        }
    }
}
