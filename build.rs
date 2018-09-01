use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target == "x86_64-pc-windows-msvc" {
        let base_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_dir: PathBuf = [&base_dir, "lib", "msvc", "64"].iter().collect();
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
    }
}