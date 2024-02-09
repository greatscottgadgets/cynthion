use std::env;
use std::str;

fn main() {
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
