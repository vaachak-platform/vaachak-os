#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_backend_takeover_cleanup::VaachakHardwareRuntimeBackendTakeoverCleanup;

pub struct VaachakHardwareRuntimeBackendTakeoverCleanupSmoke;

impl VaachakHardwareRuntimeBackendTakeoverCleanupSmoke {
    pub const MARKER: &'static str =
        VaachakHardwareRuntimeBackendTakeoverCleanup::HARDWARE_RUNTIME_BACKEND_TAKEOVER_CLEANUP_MARKER;

    pub fn smoke_ok() -> bool {
        VaachakHardwareRuntimeBackendTakeoverCleanup::backend_takeover_cleanup_ok()
            && Self::MARKER == "hardware_runtime_backend_takeover_cleanup=ok"
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeBackendTakeoverCleanupSmoke;

    #[test]
    fn hardware_runtime_backend_takeover_cleanup_smoke_passes() {
        assert!(VaachakHardwareRuntimeBackendTakeoverCleanupSmoke::smoke_ok());
    }
}
