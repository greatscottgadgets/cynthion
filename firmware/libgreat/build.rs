use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("riscv32") {
        println!("cargo:rustc-cfg=riscv");
        println!("cargo:rustc-cfg=riscv32");
    }
}
