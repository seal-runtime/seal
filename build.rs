fn main() {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target == "linux" {
        println!("cargo:rustc-link-arg=-Wl,--exclude-libs,ALL");
        println!("cargo:rustc-link-arg=-Wl,--export-dynamic");
    }

    if target == "windows" {
        // try to make symbols in src/interop/externs/ffi_api.rs visible as dll equivalent
        println!("cargo:rustc-flag=-Zexport-executable-symbols");
    }

    if target == "macos" {
        println!("cargo:rustc-flag=-Zexport-executable-symbols");
    }
}
