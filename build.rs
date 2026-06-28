//! Build script — tells the linker where to find LLVM.
//!
//! On Linux/macOS we use `llvm-config` to locate the shared library.
//! On Windows we fall back to the environment variable `LLVM_PREFIX`.

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "windows" {
        let llvm_prefix = std::env::var("LLVM_PREFIX")
            .unwrap_or_else(|_| "C:\\LLVM18".to_string());

        let lib_dir = format!("{}\\lib", llvm_prefix);
        println!("cargo:rustc-link-search=native={}", lib_dir);
        println!("cargo:rustc-link-lib=LLVM-C");

        let bin_dir = format!("{}\\bin", llvm_prefix);
        println!("cargo:rustc-link-search=native={}", bin_dir);
    } else {
        let llvm_config = std::env::var("LLVM_CONFIG")
            .unwrap_or_else(|_| "llvm-config".to_string());

        let lib_dir = std::process::Command::new(&llvm_config)
            .arg("--libdir")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "/usr/lib".to_string());

        println!("cargo:rustc-link-search=native={}", lib_dir);

        let llvm_lib = std::process::Command::new(&llvm_config)
            .arg("--libs")
            .arg("core")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default();

        let lib_name = llvm_lib
            .split_whitespace()
            .find(|s| s.starts_with("-l"))
            .map(|s| s.trim_start_matches("-l"))
            .unwrap_or("LLVM");

        println!("cargo:rustc-link-lib={}", lib_name);
        println!("cargo:rerun-if-env-changed=LLVM_CONFIG");
    }
}
