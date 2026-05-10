pub struct VaachakBoot;

impl VaachakBoot {
    pub const RUNTIME_READY_MARKER: &'static str = "vaachak=x4-runtime-ready";

    #[cfg(target_arch = "riscv32")]
    pub fn emit_runtime_ready_marker() {
        esp_println::println!("{}", Self::RUNTIME_READY_MARKER);
    }

    #[cfg(not(target_arch = "riscv32"))]
    pub fn emit_runtime_ready_marker() {
        println!("{}", Self::RUNTIME_READY_MARKER);
    }
}
