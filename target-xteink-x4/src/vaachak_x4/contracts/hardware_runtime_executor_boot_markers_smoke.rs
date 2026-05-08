#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_executor_boot_markers::VaachakHardwareRuntimeExecutorBootMarkers;
use crate::vaachak_x4::physical::hardware_runtime_executor_observability::VaachakHardwareRuntimeExecutorObservability;
use crate::vaachak_x4::physical::hardware_runtime_executor_wiring::VaachakHardwareRuntimeExecutorWiring;

/// Contract smoke for runtime-visible hardware executor boot markers.
pub struct VaachakHardwareRuntimeExecutorBootMarkersSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorBootMarkersSmokeReport {
    pub boot_marker_entrypoint_present: bool,
    pub observability_layer_present: bool,
    pub wiring_layer_present: bool,
    pub boot_marker_count_ok: bool,
    pub boot_marker_set_ready: bool,
    pub emits_to_debug_stream: bool,
    pub writes_to_display: bool,
    pub hardware_behavior_preserved: bool,
}

impl VaachakHardwareRuntimeExecutorBootMarkersSmokeReport {
    pub const fn ok(self) -> bool {
        self.boot_marker_entrypoint_present
            && self.observability_layer_present
            && self.wiring_layer_present
            && self.boot_marker_count_ok
            && self.boot_marker_set_ready
            && self.emits_to_debug_stream
            && !self.writes_to_display
            && self.hardware_behavior_preserved
    }
}

impl VaachakHardwareRuntimeExecutorBootMarkersSmoke {
    pub const MARKER: &'static str =
        VaachakHardwareRuntimeExecutorBootMarkers::HARDWARE_RUNTIME_EXECUTOR_BOOT_MARKERS_MARKER;

    pub const fn report() -> VaachakHardwareRuntimeExecutorBootMarkersSmokeReport {
        VaachakHardwareRuntimeExecutorBootMarkersSmokeReport {
            boot_marker_entrypoint_present:
                VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok(),
            observability_layer_present:
                VaachakHardwareRuntimeExecutorObservability::observability_ok(),
            wiring_layer_present: VaachakHardwareRuntimeExecutorWiring::wiring_ok(),
            // Validator anchor: BOOT_MARKER_COUNT == 8.
            boot_marker_count_ok: VaachakHardwareRuntimeExecutorBootMarkers::BOOT_MARKER_COUNT == 8,
            boot_marker_set_ready: VaachakHardwareRuntimeExecutorBootMarkers::boot_marker_set_ready(
            ),
            emits_to_debug_stream:
                VaachakHardwareRuntimeExecutorBootMarkers::BOOT_MARKERS_EMIT_TO_DEBUG_STREAM,
            writes_to_display:
                VaachakHardwareRuntimeExecutorBootMarkers::BOOT_MARKERS_WRITE_TO_DISPLAY,
            hardware_behavior_preserved:
                VaachakHardwareRuntimeExecutorBootMarkers::behavior_preserved(),
        }
    }

    pub const fn ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorBootMarkersSmoke;

    #[test]
    fn hardware_runtime_executor_boot_markers_smoke_passes() {
        assert!(VaachakHardwareRuntimeExecutorBootMarkersSmoke::ok());
    }
}
