use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("device.x"))
            .unwrap()
            .write_all(include_bytes!("device.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=device.x");
    }

    // TODO Tracking Issue: https://github.com/rust-lang/rust/issues/94039
    let Some(target) = rustc_target() else { return };
    if target_has_atomic(&target) {
        println!("cargo:rustc-cfg=target_has_atomic");
    }

    println!("cargo:rerun-if-changed=build.rs");
}

fn rustc_target() -> Option<String> {
    env::var("TARGET").ok()
}

fn target_has_atomic(target: &str) -> bool {
    match target {
        "riscv32imac-unknown-none-elf" => true,
        "riscv32i-unknown-none-elf" => false,
        _ => false,
    }
}
