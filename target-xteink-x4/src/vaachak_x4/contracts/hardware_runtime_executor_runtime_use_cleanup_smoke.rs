#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_executor_acceptance::VaachakHardwareRuntimeExecutorAcceptance;
use crate::vaachak_x4::physical::hardware_runtime_executor_runtime_use::VaachakHardwareRuntimeExecutorRuntimeUse;
use crate::vaachak_x4::physical::hardware_runtime_executor_runtime_use_cleanup::VaachakHardwareRuntimeExecutorRuntimeUseCleanup;

/// Smoke contract for the GitHub-ready runtime-use cleanup checkpoint.
pub struct VaachakHardwareRuntimeExecutorRuntimeUseCleanupSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorRuntimeUseCleanupSmokeReport {
    pub cleanup_surface_present: bool,
    pub runtime_use_surface_present: bool,
    pub acceptance_surface_present: bool,
    pub runtime_use_site_count_ok: bool,
    pub validator_fix_folded_in: bool,
    pub behavior_preserved: bool,
}

impl VaachakHardwareRuntimeExecutorRuntimeUseCleanupSmokeReport {
    pub const fn ok(self) -> bool {
        self.cleanup_surface_present
            && self.runtime_use_surface_present
            && self.acceptance_surface_present
            && self.runtime_use_site_count_ok
            && self.validator_fix_folded_in
            && self.behavior_preserved
    }
}

impl VaachakHardwareRuntimeExecutorRuntimeUseCleanupSmoke {
    pub const MARKER: &'static str = VaachakHardwareRuntimeExecutorRuntimeUseCleanup::HARDWARE_RUNTIME_EXECUTOR_RUNTIME_USE_CLEANUP_MARKER;

    pub const fn report() -> VaachakHardwareRuntimeExecutorRuntimeUseCleanupSmokeReport {
        VaachakHardwareRuntimeExecutorRuntimeUseCleanupSmokeReport {
            cleanup_surface_present: VaachakHardwareRuntimeExecutorRuntimeUseCleanup::cleanup_ok(),
            runtime_use_surface_present: VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok(),
            acceptance_surface_present: VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok(),
            runtime_use_site_count_ok:
                VaachakHardwareRuntimeExecutorRuntimeUseCleanup::runtime_use_site_count_ok(),
            validator_fix_folded_in:
                VaachakHardwareRuntimeExecutorRuntimeUseCleanup::RUNTIME_USE_VALIDATOR_FIX_FOLDED_IN,
            behavior_preserved: VaachakHardwareRuntimeExecutorRuntimeUseCleanup::behavior_preserved(
            ),
        }
    }

    pub const fn ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorRuntimeUseCleanupSmoke;

    #[test]
    fn hardware_runtime_executor_runtime_use_cleanup_smoke_passes() {
        assert!(VaachakHardwareRuntimeExecutorRuntimeUseCleanupSmoke::ok());
    }
}
