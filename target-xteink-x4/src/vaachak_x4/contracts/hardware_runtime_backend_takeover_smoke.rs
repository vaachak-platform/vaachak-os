#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_backend::VaachakHardwareRuntimeBackendInterface;
use crate::vaachak_x4::physical::hardware_runtime_backend_pulp::VaachakHardwareRuntimePulpCompatibilityBackend;
use crate::vaachak_x4::physical::hardware_runtime_backend_takeover::VaachakHardwareRuntimeBackendTakeover;
use crate::vaachak_x4::physical::hardware_runtime_executor_live_handoff::VaachakHardwareRuntimeExecutorLiveHandoff;

pub struct VaachakHardwareRuntimeBackendTakeoverSmoke;

impl VaachakHardwareRuntimeBackendTakeoverSmoke {
    pub const HARDWARE_RUNTIME_BACKEND_TAKEOVER_SMOKE_MARKER: &'static str =
        "hardware_runtime_backend_takeover_bridge=ok";

    pub fn smoke_ok() -> bool {
        VaachakHardwareRuntimeBackendInterface::interface_ok()
            && VaachakHardwareRuntimePulpCompatibilityBackend::backend_ok()
            && VaachakHardwareRuntimeBackendTakeover::takeover_ok()
            && VaachakHardwareRuntimeExecutorLiveHandoff::live_handoff_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeBackendTakeoverSmoke;

    #[test]
    fn hardware_runtime_backend_takeover_smoke_passes() {
        assert!(VaachakHardwareRuntimeBackendTakeoverSmoke::smoke_ok());
    }
}
