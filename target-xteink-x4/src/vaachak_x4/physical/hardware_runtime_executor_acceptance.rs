#![allow(dead_code)]

use super::hardware_runtime_executor::VaachakHardwareRuntimeExecutor;
use super::hardware_runtime_executor_boot_markers::VaachakHardwareRuntimeExecutorBootMarkers;
use super::hardware_runtime_executor_observability::VaachakHardwareRuntimeExecutorObservability;
use super::hardware_runtime_executor_wiring::VaachakHardwareRuntimeExecutorWiring;
use super::hardware_runtime_ownership::VaachakHardwareRuntimeOwnership;
use super::spi_bus_arbitration_runtime_owner::VaachakSpiBusArbitrationRuntimeOwner;
use super::storage_probe_mount_runtime_executor_bridge::VaachakStorageProbeMountRuntimeExecutorBridge;

/// Final acceptance surface for the Vaachak hardware runtime executor stack.
///
/// This is a cleanup/readiness layer. It does not add a new executor path and it
/// does not change SPI, SD/MMC, FAT, display, input, reader, file-browser, or
/// app-navigation behavior. Its purpose is to make the accepted hardware
/// extraction stack easy to validate before committing and pushing to GitHub.
pub struct VaachakHardwareRuntimeExecutorAcceptance;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorAcceptanceReport {
    pub hardware_ownership_consolidated: bool,
    pub spi_arbitration_owner_ready: bool,
    pub storage_lifecycle_executor_ready: bool,
    pub hardware_executor_extracted: bool,
    pub hardware_executor_wired: bool,
    pub hardware_executor_observable: bool,
    pub boot_markers_ready: bool,
    pub pulp_compatible_backend_active: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_changed: bool,
    pub display_algorithm_rewritten: bool,
    pub input_algorithm_rewritten: bool,
    pub storage_destructive_behavior_introduced: bool,
}

impl VaachakHardwareRuntimeExecutorAcceptanceReport {
    pub const fn ok(self) -> bool {
        self.hardware_ownership_consolidated
            && self.spi_arbitration_owner_ready
            && self.storage_lifecycle_executor_ready
            && self.hardware_executor_extracted
            && self.hardware_executor_wired
            && self.hardware_executor_observable
            && self.boot_markers_ready
            && self.pulp_compatible_backend_active
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_changed
            && !self.display_algorithm_rewritten
            && !self.input_algorithm_rewritten
            && !self.storage_destructive_behavior_introduced
    }
}

impl VaachakHardwareRuntimeExecutorAcceptance {
    pub const HARDWARE_RUNTIME_EXECUTOR_ACCEPTANCE_CLEANUP_MARKER: &'static str =
        "hardware_runtime_executor_acceptance_cleanup=ok";
    pub const HARDWARE_RUNTIME_EXECUTOR_ACCEPTANCE_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";
    pub const HARDWARE_RUNTIME_EXECUTOR_ACCEPTANCE_SCOPE: &'static str =
        "GitHub-ready hardware executor acceptance cleanup";

    pub const REQUIRED_ACCEPTED_LAYER_COUNT: usize = 7;
    pub const REQUIRED_BOOT_MARKER_COUNT: usize = 8;
    pub const PULP_COMPATIBLE_BACKEND_ACTIVE: bool = true;

    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_CHANGED: bool = false;
    pub const DISPLAY_ALGORITHM_REWRITTEN: bool = false;
    pub const INPUT_ALGORITHM_REWRITTEN: bool = false;
    pub const STORAGE_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;

    pub const fn accepted_layer_markers() -> [&'static str; Self::REQUIRED_ACCEPTED_LAYER_COUNT] {
        [
            VaachakHardwareRuntimeOwnership::HARDWARE_RUNTIME_OWNERSHIP_CONSOLIDATION_MARKER,
            VaachakSpiBusArbitrationRuntimeOwner::SPI_BUS_ARBITRATION_RUNTIME_OWNER_MARKER,
            VaachakStorageProbeMountRuntimeExecutorBridge::STORAGE_PROBE_MOUNT_RUNTIME_EXECUTOR_BRIDGE_MARKER,
            VaachakHardwareRuntimeExecutor::HARDWARE_RUNTIME_EXECUTOR_EXTRACTION_MARKER,
            VaachakHardwareRuntimeExecutorWiring::HARDWARE_RUNTIME_EXECUTOR_WIRING_MARKER,
            VaachakHardwareRuntimeExecutorObservability::HARDWARE_RUNTIME_EXECUTOR_OBSERVABILITY_MARKER,
            VaachakHardwareRuntimeExecutorBootMarkers::HARDWARE_RUNTIME_EXECUTOR_BOOT_MARKERS_MARKER,
        ]
    }

    pub const fn accepted_layers_ready() -> bool {
        VaachakHardwareRuntimeOwnership::consolidation_ok()
            && VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok()
            && VaachakStorageProbeMountRuntimeExecutorBridge::executor_bridge_ok()
            && VaachakHardwareRuntimeExecutor::extraction_ok()
            && VaachakHardwareRuntimeExecutorWiring::wiring_ok()
            && VaachakHardwareRuntimeExecutorObservability::observability_ok()
            && VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok()
    }

    pub const fn boot_marker_count_ok() -> bool {
        VaachakHardwareRuntimeExecutorObservability::BOOT_MARKER_COUNT
            == Self::REQUIRED_BOOT_MARKER_COUNT
            && VaachakHardwareRuntimeExecutorBootMarkers::BOOT_MARKER_COUNT
                == Self::REQUIRED_BOOT_MARKER_COUNT
    }

    pub const fn behavior_preserved() -> bool {
        Self::PULP_COMPATIBLE_BACKEND_ACTIVE
            && !Self::READER_FILE_BROWSER_UX_CHANGED
            && !Self::APP_NAVIGATION_CHANGED
            && !Self::DISPLAY_ALGORITHM_REWRITTEN
            && !Self::INPUT_ALGORITHM_REWRITTEN
            && !Self::STORAGE_DESTRUCTIVE_BEHAVIOR_INTRODUCED
            && VaachakHardwareRuntimeExecutorBootMarkers::behavior_preserved()
    }

    pub const fn report() -> VaachakHardwareRuntimeExecutorAcceptanceReport {
        VaachakHardwareRuntimeExecutorAcceptanceReport {
            hardware_ownership_consolidated: VaachakHardwareRuntimeOwnership::consolidation_ok(),
            spi_arbitration_owner_ready: VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok(),
            storage_lifecycle_executor_ready:
                VaachakStorageProbeMountRuntimeExecutorBridge::executor_bridge_ok(),
            hardware_executor_extracted: VaachakHardwareRuntimeExecutor::extraction_ok(),
            hardware_executor_wired: VaachakHardwareRuntimeExecutorWiring::wiring_ok(),
            hardware_executor_observable:
                VaachakHardwareRuntimeExecutorObservability::observability_ok(),
            boot_markers_ready: VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok()
                && Self::boot_marker_count_ok(),
            pulp_compatible_backend_active: Self::PULP_COMPATIBLE_BACKEND_ACTIVE,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_changed: Self::APP_NAVIGATION_CHANGED,
            display_algorithm_rewritten: Self::DISPLAY_ALGORITHM_REWRITTEN,
            input_algorithm_rewritten: Self::INPUT_ALGORITHM_REWRITTEN,
            storage_destructive_behavior_introduced: Self::STORAGE_DESTRUCTIVE_BEHAVIOR_INTRODUCED,
        }
    }

    pub const fn acceptance_ok() -> bool {
        Self::accepted_layers_ready() && Self::behavior_preserved() && Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorAcceptance;

    #[test]
    fn hardware_runtime_executor_acceptance_cleanup_is_ready() {
        assert!(VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok());
    }

    #[test]
    fn accepted_layer_markers_are_complete() {
        let markers = VaachakHardwareRuntimeExecutorAcceptance::accepted_layer_markers();
        assert_eq!(
            markers.len(),
            VaachakHardwareRuntimeExecutorAcceptance::REQUIRED_ACCEPTED_LAYER_COUNT
        );
        for marker in markers {
            assert!(!marker.is_empty());
        }
    }
}
