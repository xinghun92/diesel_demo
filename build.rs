use std::env;
fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        // println!("cargo:rustc-link-lib={}={}", "dylib", "gdi32");
        // println!("cargo:rustc-link-lib={}={}", "dylib", "user32");
        // println!("cargo:rustc-link-lib={}={}", "dylib", "legacy_stdio_definitions");
    }
}