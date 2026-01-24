use std::path::PathBuf;

fn main() {
    if let Some(ring_home) = std::env::var("RING")
        .ok()
        .or_else(|| std::env::var("ring").ok())
    {
        let lib_path = PathBuf::from(&ring_home).join("lib");
        println!("cargo:rustc-link-search=native={}", lib_path.display());

        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());
        }
    }
    println!("cargo:rustc-link-lib=dylib=ring");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=RING");
    println!("cargo:rerun-if-env-changed=ring");
}
