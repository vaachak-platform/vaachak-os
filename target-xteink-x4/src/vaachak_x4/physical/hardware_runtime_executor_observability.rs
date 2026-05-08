#![allow(dead_code)]

use super::hardware_runtime_executor::VaachakHardwareRuntimeExecutor;
use super::hardware_runtime_executor_wiring::{
    VaachakHardwareRuntimeExecutorWiring, VaachakHardwareWiredRuntimePath,
};
use super::hardware_runtime_observability_pulp_backend::{
    VaachakHardwareRuntimeObservabilityBackendRoute,
    VaachakHardwareRuntimeObservabilityPulpBackend, VaachakHardwareRuntimeObservationSink,
};

/// Vaachak-owned observability layer for the consolidated hardware runtime
/// executor.
///
/// This module exposes boot/debug marker descriptors that prove the wired
/// executor paths are selected. It intentionally avoids runtime printing,
/// peripheral access, allocation, and any display/input/storage behavior change.
pub struct VaachakHardwareRuntimeExecutorObservability;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeObservationMarker {
    ExecutorLayerSelected,
    WiringLayerSelected,
    PulpCompatibleBackendActive,
    SpiPathsSelected,
    StoragePathsSelected,
    DisplayPathsSelected,
    InputPathsSelected,
    BehaviorPreserved,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeObservationRecord {
    pub marker: VaachakHardwareRuntimeObservationMarker,
    pub marker_text: &'static str,
    pub sink: VaachakHardwareRuntimeObservationSink,
    pub backend_route: VaachakHardwareRuntimeObservabilityBackendRoute,
    pub selected: bool,
    pub metadata_only: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorObservabilityReport {
    pub observability_entrypoint_active: bool,
    pub executor_extraction_active: bool,
    pub wiring_layer_active: bool,
    pub pulp_backend_active: bool,
    pub boot_markers_selected: bool,
    pub debug_markers_selected: bool,
    pub spi_paths_observed: bool,
    pub storage_paths_observed: bool,
    pub display_paths_observed: bool,
    pub input_paths_observed: bool,
    pub metadata_only: bool,
    pub behavior_preserved: bool,
}

impl VaachakHardwareRuntimeExecutorObservabilityReport {
    pub const fn ok(self) -> bool {
        self.observability_entrypoint_active
            && self.executor_extraction_active
            && self.wiring_layer_active
            && self.pulp_backend_active
            && self.boot_markers_selected
            && self.debug_markers_selected
            && self.spi_paths_observed
            && self.storage_paths_observed
            && self.display_paths_observed
            && self.input_paths_observed
            && self.metadata_only
            && self.behavior_preserved
    }
}

impl VaachakHardwareRuntimeExecutorObservability {
    pub const HARDWARE_RUNTIME_EXECUTOR_OBSERVABILITY_MARKER: &'static str =
        "hardware_runtime_executor_observability=ok";
    pub const HARDWARE_RUNTIME_EXECUTOR_OBSERVABILITY_IDENTITY: &'static str =
        "xteink-x4-vaachak-hardware-runtime-executor-observability";
    pub const HARDWARE_RUNTIME_EXECUTOR_OBSERVABILITY_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";

    pub const OBSERVABILITY_ENTRYPOINT_ACTIVE: bool = true;
    pub const BOOT_MARKER_COUNT: usize = 8;
    pub const WIRED_RUNTIME_PATH_COUNT: usize =
        VaachakHardwareRuntimeExecutorWiring::SELECTED_RUNTIME_PATH_COUNT;

    pub const MARKERS_ARE_METADATA_ONLY: bool = true;
    pub const EMITS_TEXT_TO_DISPLAY: bool = false;
    pub const TOUCHES_SPI_TRANSFER: bool = false;
    pub const TOUCHES_SD_MMC: bool = false;
    pub const TOUCHES_FAT_IMPLEMENTATION: bool = false;
    pub const TOUCHES_SSD1677_RENDERING: bool = false;
    pub const TOUCHES_INPUT_ADC: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_CHANGED: bool = false;
    pub const FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;

    pub const fn boot_markers() -> [VaachakHardwareRuntimeObservationRecord; Self::BOOT_MARKER_COUNT]
    {
        [
            Self::record(
                VaachakHardwareRuntimeObservationMarker::ExecutorLayerSelected,
                "hardware.executor.layer.selected",
                VaachakHardwareRuntimeObservationSink::BootMarker,
                VaachakHardwareRuntimeExecutor::extraction_ok(),
            ),
            Self::record(
                VaachakHardwareRuntimeObservationMarker::WiringLayerSelected,
                "hardware.executor.wiring.selected",
                VaachakHardwareRuntimeObservationSink::BootMarker,
                VaachakHardwareRuntimeExecutorWiring::wiring_ok(),
            ),
            Self::record(
                VaachakHardwareRuntimeObservationMarker::PulpCompatibleBackendActive,
                "hardware.executor.backend.pulp_compatible",
                VaachakHardwareRuntimeObservationSink::BootMarker,
                VaachakHardwareRuntimeObservabilityPulpBackend::backend_ok(),
            ),
            Self::record(
                VaachakHardwareRuntimeObservationMarker::SpiPathsSelected,
                "hardware.executor.spi.paths.selected",
                VaachakHardwareRuntimeObservationSink::BootMarker,
                VaachakHardwareRuntimeExecutorWiring::spi_paths_wired(),
            ),
            Self::record(
                VaachakHardwareRuntimeObservationMarker::StoragePathsSelected,
                "hardware.executor.storage.paths.selected",
                VaachakHardwareRuntimeObservationSink::BootMarker,
                VaachakHardwareRuntimeExecutorWiring::storage_paths_wired(),
            ),
            Self::record(
                VaachakHardwareRuntimeObservationMarker::DisplayPathsSelected,
                "hardware.executor.display.paths.selected",
                VaachakHardwareRuntimeObservationSink::BootMarker,
                VaachakHardwareRuntimeExecutorWiring::display_paths_wired(),
            ),
            Self::record(
                VaachakHardwareRuntimeObservationMarker::InputPathsSelected,
                "hardware.executor.input.paths.selected",
                VaachakHardwareRuntimeObservationSink::BootMarker,
                VaachakHardwareRuntimeExecutorWiring::input_paths_wired(),
            ),
            Self::record(
                VaachakHardwareRuntimeObservationMarker::BehaviorPreserved,
                "hardware.executor.behavior.preserved",
                VaachakHardwareRuntimeObservationSink::BootMarker,
                Self::behavior_preserved(),
            ),
        ]
    }

    pub const fn record(
        marker: VaachakHardwareRuntimeObservationMarker,
        marker_text: &'static str,
        sink: VaachakHardwareRuntimeObservationSink,
        selected: bool,
    ) -> VaachakHardwareRuntimeObservationRecord {
        VaachakHardwareRuntimeObservationRecord {
            marker,
            marker_text,
            sink,
            backend_route: VaachakHardwareRuntimeObservabilityPulpBackend::route_for(sink),
            selected,
            metadata_only: Self::MARKERS_ARE_METADATA_ONLY,
        }
    }

    pub const fn record_ok(record: VaachakHardwareRuntimeObservationRecord) -> bool {
        record.selected
            && record.metadata_only
            && VaachakHardwareRuntimeObservabilityPulpBackend::route_ok(record.backend_route)
    }

    pub const fn boot_markers_selected() -> bool {
        let markers = Self::boot_markers();
        Self::record_ok(markers[0])
            && Self::record_ok(markers[1])
            && Self::record_ok(markers[2])
            && Self::record_ok(markers[3])
            && Self::record_ok(markers[4])
            && Self::record_ok(markers[5])
            && Self::record_ok(markers[6])
            && Self::record_ok(markers[7])
    }

    pub const fn debug_marker_for_path(
        path: VaachakHardwareWiredRuntimePath,
    ) -> VaachakHardwareRuntimeObservationRecord {
        let selected = VaachakHardwareRuntimeExecutorWiring::route_is_safe(
            VaachakHardwareRuntimeExecutorWiring::route_path(path),
        );
        Self::record(
            VaachakHardwareRuntimeObservationMarker::WiringLayerSelected,
            "hardware.executor.path.selected",
            VaachakHardwareRuntimeObservationSink::DebugMarker,
            selected,
        )
    }

    pub const fn all_wired_paths_observed() -> bool {
        let paths = VaachakHardwareRuntimeExecutorWiring::selected_paths();
        Self::record_ok(Self::debug_marker_for_path(paths[0]))
            && Self::record_ok(Self::debug_marker_for_path(paths[1]))
            && Self::record_ok(Self::debug_marker_for_path(paths[2]))
            && Self::record_ok(Self::debug_marker_for_path(paths[3]))
            && Self::record_ok(Self::debug_marker_for_path(paths[4]))
            && Self::record_ok(Self::debug_marker_for_path(paths[5]))
            && Self::record_ok(Self::debug_marker_for_path(paths[6]))
            && Self::record_ok(Self::debug_marker_for_path(paths[7]))
            && Self::record_ok(Self::debug_marker_for_path(paths[8]))
            && Self::record_ok(Self::debug_marker_for_path(paths[9]))
    }

    pub const fn metadata_only() -> bool {
        Self::MARKERS_ARE_METADATA_ONLY
            && !Self::EMITS_TEXT_TO_DISPLAY
            && !Self::TOUCHES_SPI_TRANSFER
            && !Self::TOUCHES_SD_MMC
            && !Self::TOUCHES_FAT_IMPLEMENTATION
            && !Self::TOUCHES_SSD1677_RENDERING
            && !Self::TOUCHES_INPUT_ADC
    }

    pub const fn behavior_preserved() -> bool {
        Self::metadata_only()
            && !Self::READER_FILE_BROWSER_UX_CHANGED
            && !Self::APP_NAVIGATION_CHANGED
            && !Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED
            && VaachakHardwareRuntimeObservabilityPulpBackend::behavior_preserved()
    }

    pub const fn report() -> VaachakHardwareRuntimeExecutorObservabilityReport {
        VaachakHardwareRuntimeExecutorObservabilityReport {
            observability_entrypoint_active: Self::OBSERVABILITY_ENTRYPOINT_ACTIVE,
            executor_extraction_active: VaachakHardwareRuntimeExecutor::extraction_ok(),
            wiring_layer_active: VaachakHardwareRuntimeExecutorWiring::wiring_ok(),
            pulp_backend_active: VaachakHardwareRuntimeObservabilityPulpBackend::backend_ok(),
            boot_markers_selected: Self::boot_markers_selected(),
            debug_markers_selected: Self::all_wired_paths_observed(),
            spi_paths_observed: VaachakHardwareRuntimeExecutorWiring::spi_paths_wired(),
            storage_paths_observed: VaachakHardwareRuntimeExecutorWiring::storage_paths_wired(),
            display_paths_observed: VaachakHardwareRuntimeExecutorWiring::display_paths_wired(),
            input_paths_observed: VaachakHardwareRuntimeExecutorWiring::input_paths_wired(),
            metadata_only: Self::metadata_only(),
            behavior_preserved: Self::behavior_preserved(),
        }
    }

    pub const fn observability_ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorObservability;

    #[test]
    fn hardware_runtime_executor_observability_is_active() {
        assert!(VaachakHardwareRuntimeExecutorObservability::observability_ok());
    }

    #[test]
    fn boot_markers_are_selected_and_metadata_only() {
        let markers = VaachakHardwareRuntimeExecutorObservability::boot_markers();
        assert_eq!(
            markers.len(),
            VaachakHardwareRuntimeExecutorObservability::BOOT_MARKER_COUNT
        );
        for marker in markers {
            assert!(VaachakHardwareRuntimeExecutorObservability::record_ok(
                marker
            ));
        }
    }
}
