fn main() {
    println!("cargo:rerun-if-env-changed=TARGET");

    let target = std::env::var("TARGET").unwrap_or_default();
    if target == "riscv32imc-unknown-none-elf" {
        println!("cargo:rustc-link-arg=-Tlinkall.x");
    }
}
