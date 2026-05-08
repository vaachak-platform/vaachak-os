#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_executor_acceptance::VaachakHardwareRuntimeExecutorAcceptance;
use crate::vaachak_x4::physical::hardware_runtime_executor_boot_markers::VaachakHardwareRuntimeExecutorBootMarkers;
use crate::vaachak_x4::physical::hardware_runtime_executor_observability::VaachakHardwareRuntimeExecutorObservability;
use crate::vaachak_x4::physical::hardware_runtime_executor_wiring::VaachakHardwareRuntimeExecutorWiring;
use crate::vaachak_x4::physical::hardware_runtime_ownership::VaachakHardwareRuntimeOwnership;

/// Smoke contract for the final hardware runtime executor acceptance cleanup.
pub struct VaachakHardwareRuntimeExecutorAcceptanceSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorAcceptanceSmokeReport {
    pub acceptance_surface_present: bool,
    pub ownership_consolidation_present: bool,
    pub executor_wiring_present: bool,
    pub observability_present: bool,
    pub boot_markers_present: bool,
    pub accepted_layer_count_ok: bool,
    pub boot_marker_count_ok: bool,
    pub behavior_preserved: bool,
}

impl VaachakHardwareRuntimeExecutorAcceptanceSmokeReport {
    pub const fn ok(self) -> bool {
        self.acceptance_surface_present
            && self.ownership_consolidation_present
            && self.executor_wiring_present
            && self.observability_present
            && self.boot_markers_present
            && self.accepted_layer_count_ok
            && self.boot_marker_count_ok
            && self.behavior_preserved
    }
}

impl VaachakHardwareRuntimeExecutorAcceptanceSmoke {
    pub const MARKER: &'static str =
        VaachakHardwareRuntimeExecutorAcceptance::HARDWARE_RUNTIME_EXECUTOR_ACCEPTANCE_CLEANUP_MARKER;

    pub const fn report() -> VaachakHardwareRuntimeExecutorAcceptanceSmokeReport {
        VaachakHardwareRuntimeExecutorAcceptanceSmokeReport {
            acceptance_surface_present: VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok(),
            ownership_consolidation_present: VaachakHardwareRuntimeOwnership::consolidation_ok(),
            executor_wiring_present: VaachakHardwareRuntimeExecutorWiring::wiring_ok(),
            observability_present: VaachakHardwareRuntimeExecutorObservability::observability_ok(),
            boot_markers_present: VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok(),
            accepted_layer_count_ok:
                VaachakHardwareRuntimeExecutorAcceptance::accepted_layer_markers().len()
                    == VaachakHardwareRuntimeExecutorAcceptance::REQUIRED_ACCEPTED_LAYER_COUNT,
            boot_marker_count_ok: VaachakHardwareRuntimeExecutorAcceptance::boot_marker_count_ok(),
            behavior_preserved: VaachakHardwareRuntimeExecutorAcceptance::behavior_preserved(),
        }
    }

    pub const fn ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorAcceptanceSmoke;

    #[test]
    fn hardware_runtime_executor_acceptance_smoke_passes() {
        assert!(VaachakHardwareRuntimeExecutorAcceptanceSmoke::ok());
    }
}
