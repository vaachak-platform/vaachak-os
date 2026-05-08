#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_executor_live_handoff::VaachakHardwareRuntimeExecutorLiveHandoff;
use crate::vaachak_x4::physical::hardware_runtime_executor_runtime_use::VaachakHardwareRuntimeExecutorRuntimeUse;

/// Static smoke for the live Vaachak hardware executor handoff path.
pub struct VaachakHardwareRuntimeExecutorLiveHandoffSmoke;

impl VaachakHardwareRuntimeExecutorLiveHandoffSmoke {
    pub const HARDWARE_RUNTIME_EXECUTOR_LIVE_HANDOFF_SMOKE_MARKER: &'static str =
        "hardware_runtime_executor_live_path_handoff=ok";

    pub const fn smoke_ok() -> bool {
        VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()
            && VaachakHardwareRuntimeExecutorLiveHandoff::live_handoff_ok()
            && VaachakHardwareRuntimeExecutorLiveHandoff::LIVE_HANDOFF_SITE_COUNT == 5
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorLiveHandoffSmoke;

    #[test]
    fn hardware_runtime_executor_live_handoff_smoke_is_ready() {
        assert!(VaachakHardwareRuntimeExecutorLiveHandoffSmoke::smoke_ok());
    }
}
