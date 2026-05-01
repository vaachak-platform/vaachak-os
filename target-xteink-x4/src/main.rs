#![cfg_attr(target_arch = "riscv32", no_std)]
#![cfg_attr(target_arch = "riscv32", no_main)]

#[cfg(target_arch = "riscv32")]
mod vaachak_x4;

#[cfg(not(target_arch = "riscv32"))]
fn main() {
    println!("VaachakOS X4 host placeholder: vaachak=x4-runtime-ready");
}
