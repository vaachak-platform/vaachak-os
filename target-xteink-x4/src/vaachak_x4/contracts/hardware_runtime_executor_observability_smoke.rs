#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_executor::VaachakHardwareRuntimeExecutor;
use crate::vaachak_x4::physical::hardware_runtime_executor_observability::VaachakHardwareRuntimeExecutorObservability;
use crate::vaachak_x4::physical::hardware_runtime_executor_wiring::VaachakHardwareRuntimeExecutorWiring;
use crate::vaachak_x4::physical::hardware_runtime_observability_pulp_backend::VaachakHardwareRuntimeObservabilityPulpBackend;

/// Contract smoke for hardware runtime executor observability.
pub struct VaachakHardwareRuntimeExecutorObservabilitySmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorObservabilitySmokeReport {
    pub observability_entrypoint_present: bool,
    pub executor_layer_present: bool,
    pub wiring_layer_present: bool,
    pub pulp_backend_active: bool,
    pub boot_marker_count_ok: bool,
    pub boot_markers_selected: bool,
    pub wired_paths_observed: bool,
    pub spi_paths_observed: bool,
    pub storage_paths_observed: bool,
    pub display_paths_observed: bool,
    pub input_paths_observed: bool,
    pub metadata_only: bool,
    pub behavior_preserved: bool,
}

impl VaachakHardwareRuntimeExecutorObservabilitySmokeReport {
    pub const fn ok(self) -> bool {
        self.observability_entrypoint_present
            && self.executor_layer_present
            && self.wiring_layer_present
            && self.pulp_backend_active
            && self.boot_marker_count_ok
            && self.boot_markers_selected
            && self.wired_paths_observed
            && self.spi_paths_observed
            && self.storage_paths_observed
            && self.display_paths_observed
            && self.input_paths_observed
            && self.metadata_only
            && self.behavior_preserved
    }
}

impl VaachakHardwareRuntimeExecutorObservabilitySmoke {
    pub const MARKER: &'static str =
        VaachakHardwareRuntimeExecutorObservability::HARDWARE_RUNTIME_EXECUTOR_OBSERVABILITY_MARKER;

    pub const fn report() -> VaachakHardwareRuntimeExecutorObservabilitySmokeReport {
        VaachakHardwareRuntimeExecutorObservabilitySmokeReport {
            observability_entrypoint_present:
                VaachakHardwareRuntimeExecutorObservability::observability_ok(),
            executor_layer_present: VaachakHardwareRuntimeExecutor::extraction_ok(),
            wiring_layer_present: VaachakHardwareRuntimeExecutorWiring::wiring_ok(),
            pulp_backend_active: VaachakHardwareRuntimeObservabilityPulpBackend::backend_ok(),
            // Validator anchor: BOOT_MARKER_COUNT == 8.
            boot_marker_count_ok: VaachakHardwareRuntimeExecutorObservability::BOOT_MARKER_COUNT
                == 8,
            boot_markers_selected:
                VaachakHardwareRuntimeExecutorObservability::boot_markers_selected(),
            wired_paths_observed:
                VaachakHardwareRuntimeExecutorObservability::all_wired_paths_observed(),
            spi_paths_observed: VaachakHardwareRuntimeExecutorWiring::spi_paths_wired(),
            storage_paths_observed: VaachakHardwareRuntimeExecutorWiring::storage_paths_wired(),
            display_paths_observed: VaachakHardwareRuntimeExecutorWiring::display_paths_wired(),
            input_paths_observed: VaachakHardwareRuntimeExecutorWiring::input_paths_wired(),
            metadata_only: VaachakHardwareRuntimeExecutorObservability::metadata_only(),
            behavior_preserved: VaachakHardwareRuntimeExecutorObservability::behavior_preserved(),
        }
    }

    pub const fn ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorObservabilitySmoke;

    #[test]
    fn hardware_runtime_executor_observability_smoke_passes() {
        assert!(VaachakHardwareRuntimeExecutorObservabilitySmoke::ok());
    }
}
