#![allow(dead_code)]

use super::display_backend_native_refresh_shell::VaachakDisplayBackendNativeRefreshShell;
use super::hardware_executor_pulp_backend::VaachakHardwareExecutorBackend;
use super::hardware_runtime_backend::{
    VaachakDisplayBackendOperation, VaachakDisplayExecutor, VaachakDisplayRequest,
    VaachakHardwareBackendHandoffResult, VaachakHardwareRuntimeBackendInterface,
    VaachakInputBackendOperation, VaachakInputExecutor, VaachakInputRequest,
    VaachakSpiDisplayTransactionRequest, VaachakSpiStorageTransactionRequest,
    VaachakSpiTransactionExecutor, VaachakStorageAccessBackendOperation,
    VaachakStorageAccessRequest, VaachakStorageFatAccessExecutor, VaachakStoragePathRole,
    VaachakStorageProbeMountBackendIntent, VaachakStorageProbeMountExecutor,
    VaachakStorageProbeMountRequest,
};
use super::hardware_runtime_backend_pulp::VaachakHardwareRuntimePulpCompatibilityBackend;
use super::hardware_runtime_executor_acceptance::VaachakHardwareRuntimeExecutorAcceptance;
use super::hardware_runtime_executor_runtime_use::VaachakHardwareRuntimeExecutorRuntimeUse;
use super::input_backend_native_executor::VaachakInputBackendNativeExecutor;

/// Vaachak-owned backend takeover bridge.
///
/// This is the first layer where live handoff can call a Vaachak-owned backend
/// trait interface instead of only exposing marker metadata. The selected backend
/// remains PulpCompatibility and the low-level executor remains the imported
/// Pulp runtime.
pub struct VaachakHardwareRuntimeBackendTakeover;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeBackendTakeoverReport {
    pub backend_traits_owned_by_vaachak: bool,
    pub request_result_structs_owned_by_vaachak: bool,
    pub active_backend: VaachakHardwareExecutorBackend,
    pub active_backend_name: &'static str,
    pub backend_owner: &'static str,
    pub low_level_executor_owner: &'static str,
    pub pulp_compatibility_implements_backend_interface: bool,
    pub live_handoff_can_call_backend_interface: bool,
    pub runtime_use_acceptance_required: bool,
    pub display_draw_algorithm_rewritten: bool,
    pub sd_mmc_fat_algorithm_rewritten: bool,
    pub input_debounce_navigation_rewritten: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_changed: bool,
    pub destructive_storage_behavior_added: bool,
}

impl VaachakHardwareRuntimeBackendTakeoverReport {
    pub const fn ok(self) -> bool {
        self.backend_traits_owned_by_vaachak
            && self.request_result_structs_owned_by_vaachak
            && matches!(
                self.active_backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
            && self.pulp_compatibility_implements_backend_interface
            && self.live_handoff_can_call_backend_interface
            && self.runtime_use_acceptance_required
            && !self.display_draw_algorithm_rewritten
            && !self.sd_mmc_fat_algorithm_rewritten
            && !self.input_debounce_navigation_rewritten
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_changed
            && !self.destructive_storage_behavior_added
    }
}

impl VaachakHardwareRuntimeBackendTakeover {
    pub const HARDWARE_RUNTIME_BACKEND_TAKEOVER_MARKER: &'static str =
        "hardware_runtime_backend_takeover_bridge=ok";
    pub const BACKEND_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const LOW_LEVEL_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_BACKEND: VaachakHardwareExecutorBackend =
        VaachakHardwareExecutorBackend::PulpCompatibility;
    pub const ACTIVE_BACKEND_NAME: &'static str = "PulpCompatibility";

    pub const DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false;
    pub const SD_MMC_FAT_ALGORITHM_REWRITTEN: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_CHANGED: bool = false;
    pub const DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED: bool = false;

    fn backend() -> VaachakHardwareRuntimePulpCompatibilityBackend {
        VaachakHardwareRuntimePulpCompatibilityBackend
    }

    pub fn execute_spi_display_transaction_handoff() -> VaachakHardwareBackendHandoffResult {
        Self::backend().execute_spi_display_transaction(VaachakSpiDisplayTransactionRequest {
            transaction_owner: "display",
            display_chip_select_gpio:
                VaachakHardwareRuntimePulpCompatibilityBackend::DISPLAY_CS_GPIO,
            safe_arbitration_handoff_required: true,
        })
    }

    pub fn execute_spi_storage_transaction_handoff() -> VaachakHardwareBackendHandoffResult {
        Self::backend().execute_spi_storage_transaction(VaachakSpiStorageTransactionRequest {
            transaction_owner: "storage",
            storage_chip_select_gpio:
                VaachakHardwareRuntimePulpCompatibilityBackend::STORAGE_CS_GPIO,
            safe_arbitration_handoff_required: true,
        })
    }

    pub fn execute_storage_probe_mount_handoff() -> VaachakHardwareBackendHandoffResult {
        Self::backend().execute_storage_probe_mount(VaachakStorageProbeMountRequest {
            intent: VaachakStorageProbeMountBackendIntent::MountReadiness,
            card_present_expected: true,
            mount_readiness_required: true,
        })
    }

    pub fn execute_storage_directory_listing_handoff() -> VaachakHardwareBackendHandoffResult {
        Self::backend().execute_storage_access(VaachakStorageAccessRequest {
            operation: VaachakStorageAccessBackendOperation::DirectoryListing,
            path_role: VaachakStoragePathRole::LibraryRoot,
            destructive_operation_allowed: false,
        })
    }

    pub fn execute_storage_file_open_handoff() -> VaachakHardwareBackendHandoffResult {
        Self::backend().execute_storage_access(VaachakStorageAccessRequest {
            operation: VaachakStorageAccessBackendOperation::FileOpen,
            path_role: VaachakStoragePathRole::ReaderBookPath,
            destructive_operation_allowed: false,
        })
    }

    pub fn execute_storage_file_read_handoff() -> VaachakHardwareBackendHandoffResult {
        Self::backend().execute_storage_access(VaachakStorageAccessRequest {
            operation: VaachakStorageAccessBackendOperation::FileReadChunk,
            path_role: VaachakStoragePathRole::ReaderBookPath,
            destructive_operation_allowed: false,
        })
    }

    pub fn execute_storage_path_resolution_handoff() -> VaachakHardwareBackendHandoffResult {
        Self::backend().execute_storage_access(VaachakStorageAccessRequest {
            operation: VaachakStorageAccessBackendOperation::StateCachePathResolution,
            path_role: VaachakStoragePathRole::CachePath,
            destructive_operation_allowed: false,
        })
    }

    pub fn execute_display_full_refresh_handoff() -> VaachakHardwareBackendHandoffResult {
        let native_refresh =
            VaachakDisplayBackendNativeRefreshShell::execute_full_refresh_handoff();
        if !native_refresh.ok() {
            return Self::backend().execute_display(VaachakDisplayRequest {
                operation: VaachakDisplayBackendOperation::FullRefresh,
                spi_handoff_required: true,
                display_draw_algorithm_rewrite_allowed: false,
            });
        }

        Self::backend().execute_display(
            VaachakDisplayBackendNativeRefreshShell::display_request_for(
                super::display_backend_native_refresh_shell::VaachakDisplayRefreshCommand::FullRefresh,
            ),
        )
    }

    pub fn execute_display_partial_refresh_handoff() -> VaachakHardwareBackendHandoffResult {
        let native_refresh =
            VaachakDisplayBackendNativeRefreshShell::execute_partial_refresh_handoff();
        if !native_refresh.ok() {
            return Self::backend().execute_display(VaachakDisplayRequest {
                operation: VaachakDisplayBackendOperation::PartialRefresh,
                spi_handoff_required: true,
                display_draw_algorithm_rewrite_allowed: false,
            });
        }

        Self::backend().execute_display(
            VaachakDisplayBackendNativeRefreshShell::display_request_for(
                super::display_backend_native_refresh_shell::VaachakDisplayRefreshCommand::PartialRefresh,
            ),
        )
    }

    pub fn execute_input_scan_handoff() -> VaachakHardwareBackendHandoffResult {
        let native_scan = VaachakInputBackendNativeExecutor::execute_scan_handoff();
        if !native_scan.ok() {
            return Self::backend().execute_input(VaachakInputRequest {
                operation: VaachakInputBackendOperation::ButtonScan,
                adc_ladder_owner_required: true,
                input_debounce_navigation_rewrite_allowed: false,
            });
        }

        Self::backend().execute_input(VaachakInputRequest {
            operation: VaachakInputBackendOperation::ButtonScan,
            adc_ladder_owner_required: true,
            input_debounce_navigation_rewrite_allowed: false,
        })
    }

    pub fn execute_input_navigation_handoff() -> VaachakHardwareBackendHandoffResult {
        let native_navigation = VaachakInputBackendNativeExecutor::execute_navigation_handoff();
        if !native_navigation.ok() {
            return Self::backend().execute_input(VaachakInputRequest {
                operation: VaachakInputBackendOperation::NavigationHandoff,
                adc_ladder_owner_required: true,
                input_debounce_navigation_rewrite_allowed: false,
            });
        }

        Self::backend().execute_input(VaachakInputRequest {
            operation: VaachakInputBackendOperation::NavigationHandoff,
            adc_ladder_owner_required: true,
            input_debounce_navigation_rewrite_allowed: false,
        })
    }

    pub fn backend_interface_calls_ok() -> bool {
        let display_native_refresh_ready =
            VaachakDisplayBackendNativeRefreshShell::native_refresh_shell_ok();
        display_native_refresh_ready
            && Self::execute_spi_display_transaction_handoff().ok()
            && Self::execute_spi_storage_transaction_handoff().ok()
            && Self::execute_storage_probe_mount_handoff().ok()
            && Self::execute_storage_directory_listing_handoff().ok()
            && Self::execute_storage_file_open_handoff().ok()
            && Self::execute_storage_file_read_handoff().ok()
            && Self::execute_storage_path_resolution_handoff().ok()
            && Self::execute_display_full_refresh_handoff().ok()
            && Self::execute_display_partial_refresh_handoff().ok()
            && Self::execute_input_scan_handoff().ok()
            && Self::execute_input_navigation_handoff().ok()
    }

    pub fn report() -> VaachakHardwareRuntimeBackendTakeoverReport {
        VaachakHardwareRuntimeBackendTakeoverReport {
            backend_traits_owned_by_vaachak: VaachakHardwareRuntimeBackendInterface::interface_ok(),
            request_result_structs_owned_by_vaachak:
                VaachakHardwareRuntimeBackendInterface::REQUEST_RESULT_STRUCTS_OWNED_BY_VAACHAK,
            active_backend: Self::ACTIVE_BACKEND,
            active_backend_name: Self::ACTIVE_BACKEND_NAME,
            backend_owner: Self::BACKEND_OWNER,
            low_level_executor_owner: Self::LOW_LEVEL_EXECUTOR_OWNER,
            pulp_compatibility_implements_backend_interface:
                VaachakHardwareRuntimePulpCompatibilityBackend::backend_ok(),
            live_handoff_can_call_backend_interface: Self::backend_interface_calls_ok(),
            runtime_use_acceptance_required:
                VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()
                    && VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok(),
            display_draw_algorithm_rewritten: Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN,
            sd_mmc_fat_algorithm_rewritten: Self::SD_MMC_FAT_ALGORITHM_REWRITTEN,
            input_debounce_navigation_rewritten: Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_changed: Self::APP_NAVIGATION_CHANGED,
            destructive_storage_behavior_added: Self::DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED,
        }
    }

    pub fn takeover_ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeBackendTakeover;

    #[test]
    fn hardware_runtime_backend_takeover_bridge_is_ready() {
        assert!(VaachakHardwareRuntimeBackendTakeover::takeover_ok());
    }

    #[test]
    fn backend_interface_calls_remain_pulp_compatible() {
        assert!(VaachakHardwareRuntimeBackendTakeover::backend_interface_calls_ok());
    }
}
