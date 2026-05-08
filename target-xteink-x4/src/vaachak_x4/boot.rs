pub struct VaachakBoot;

impl VaachakBoot {
    pub const RUNTIME_READY_MARKER: &'static str = "vaachak=x4-runtime-ready";
    pub fn emit_hardware_runtime_executor_boot_markers() {
        use crate::vaachak_x4::physical::hardware_runtime_executor_boot_markers::VaachakHardwareRuntimeExecutorBootMarkers;
        VaachakHardwareRuntimeExecutorBootMarkers::emit_boot_markers();
    }

    #[cfg(target_arch = "riscv32")]
    pub fn emit_runtime_ready_marker() {
        esp_println::println!("{}", Self::RUNTIME_READY_MARKER);
    }

    #[cfg(not(target_arch = "riscv32"))]
    pub fn emit_runtime_ready_marker() {
        println!("{}", Self::RUNTIME_READY_MARKER);
    }
}
