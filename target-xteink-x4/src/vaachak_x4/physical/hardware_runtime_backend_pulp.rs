#![allow(dead_code)]

use super::hardware_executor_pulp_backend::VaachakHardwareExecutorBackend;
use super::hardware_runtime_backend::{
    VaachakDisplayBackendOperation, VaachakDisplayExecutor, VaachakDisplayRequest,
    VaachakHardwareBackendDomain, VaachakHardwareBackendHandoffResult, VaachakInputExecutor,
    VaachakInputRequest, VaachakSpiDisplayTransactionRequest, VaachakSpiStorageTransactionRequest,
    VaachakSpiTransactionExecutor, VaachakStorageAccessBackendOperation,
    VaachakStorageAccessRequest, VaachakStorageFatAccessExecutor, VaachakStoragePathRole,
    VaachakStorageProbeMountExecutor, VaachakStorageProbeMountRequest,
};

/// PulpCompatibility implementation of the Vaachak-owned backend traits.
///
/// This backend intentionally describes and routes handoff acceptance to the
/// imported Pulp runtime. It does not reimplement SSD1677 drawing, SD/MMC/FAT,
/// physical SPI transfer, chip-select toggling, or input debounce/navigation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VaachakHardwareRuntimePulpCompatibilityBackend;

impl VaachakHardwareRuntimePulpCompatibilityBackend {
    pub const BACKEND_NAME: &'static str = "PulpCompatibility";
    pub const BACKEND_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const LOW_LEVEL_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_BACKEND: VaachakHardwareExecutorBackend =
        VaachakHardwareExecutorBackend::PulpCompatibility;

    pub const DISPLAY_CS_GPIO: u8 = 21;
    pub const STORAGE_CS_GPIO: u8 = 12;

    pub const PHYSICAL_SPI_TRANSFER_REWRITTEN: bool = false;
    pub const CHIP_SELECT_TOGGLING_REWRITTEN: bool = false;
    pub const SD_MMC_FAT_ALGORITHMS_REWRITTEN: bool = false;
    pub const SSD1677_DRAW_ALGORITHMS_REWRITTEN: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_CHANGED: bool = false;
    pub const DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED: bool = false;

    fn result(
        domain: VaachakHardwareBackendDomain,
        accepted: bool,
    ) -> VaachakHardwareBackendHandoffResult {
        VaachakHardwareBackendHandoffResult {
            domain,
            accepted,
            backend: Self::ACTIVE_BACKEND,
            backend_name: Self::BACKEND_NAME,
            backend_owner: Self::BACKEND_OWNER,
            low_level_executor_owner: Self::LOW_LEVEL_EXECUTOR_OWNER,
            pulp_compatible_backend_active: true,
            low_level_behavior_changed: Self::PHYSICAL_SPI_TRANSFER_REWRITTEN
                || Self::CHIP_SELECT_TOGGLING_REWRITTEN
                || Self::SD_MMC_FAT_ALGORITHMS_REWRITTEN
                || Self::SSD1677_DRAW_ALGORITHMS_REWRITTEN
                || Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN,
            destructive_behavior_added: Self::DESTRUCTIVE_STORAGE_BEHAVIOR_ADDED,
            ux_behavior_changed: Self::READER_FILE_BROWSER_UX_CHANGED
                || Self::APP_NAVIGATION_CHANGED,
        }
    }

    pub fn backend_ok() -> bool {
        let backend = Self;
        backend
            .execute_spi_display_transaction(VaachakSpiDisplayTransactionRequest {
                transaction_owner: "display",
                display_chip_select_gpio: Self::DISPLAY_CS_GPIO,
                safe_arbitration_handoff_required: true,
            })
            .ok()
            && backend
                .execute_spi_storage_transaction(VaachakSpiStorageTransactionRequest {
                    transaction_owner: "storage",
                    storage_chip_select_gpio: Self::STORAGE_CS_GPIO,
                    safe_arbitration_handoff_required: true,
                })
                .ok()
            && backend
                .execute_storage_probe_mount(VaachakStorageProbeMountRequest {
                    intent: super::hardware_runtime_backend::VaachakStorageProbeMountBackendIntent::MountReadiness,
                    card_present_expected: true,
                    mount_readiness_required: true,
                })
                .ok()
            && backend
                .execute_storage_access(VaachakStorageAccessRequest {
                    operation: VaachakStorageAccessBackendOperation::DirectoryListing,
                    path_role: VaachakStoragePathRole::LibraryRoot,
                    destructive_operation_allowed: false,
                })
                .ok()
            && backend
                .execute_display(VaachakDisplayRequest {
                    operation: VaachakDisplayBackendOperation::FullRefresh,
                    spi_handoff_required: true,
                    display_draw_algorithm_rewrite_allowed: false,
                })
                .ok()
            && backend
                .execute_input(VaachakInputRequest {
                    operation: super::hardware_runtime_backend::VaachakInputBackendOperation::ButtonScan,
                    adc_ladder_owner_required: true,
                    input_debounce_navigation_rewrite_allowed: false,
                })
                .ok()
    }
}

impl VaachakSpiTransactionExecutor for VaachakHardwareRuntimePulpCompatibilityBackend {
    fn execute_spi_display_transaction(
        &self,
        request: VaachakSpiDisplayTransactionRequest,
    ) -> VaachakHardwareBackendHandoffResult {
        Self::result(
            VaachakHardwareBackendDomain::SpiTransaction,
            request.display_chip_select_gpio == Self::DISPLAY_CS_GPIO
                && request.safe_arbitration_handoff_required
                && !request.transaction_owner.is_empty(),
        )
    }

    fn execute_spi_storage_transaction(
        &self,
        request: VaachakSpiStorageTransactionRequest,
    ) -> VaachakHardwareBackendHandoffResult {
        Self::result(
            VaachakHardwareBackendDomain::SpiTransaction,
            request.storage_chip_select_gpio == Self::STORAGE_CS_GPIO
                && request.safe_arbitration_handoff_required
                && !request.transaction_owner.is_empty(),
        )
    }
}

impl VaachakStorageProbeMountExecutor for VaachakHardwareRuntimePulpCompatibilityBackend {
    fn execute_storage_probe_mount(
        &self,
        request: VaachakStorageProbeMountRequest,
    ) -> VaachakHardwareBackendHandoffResult {
        let accepted = request.card_present_expected || request.mount_readiness_required;
        Self::result(VaachakHardwareBackendDomain::StorageProbeMount, accepted)
    }
}

impl VaachakStorageFatAccessExecutor for VaachakHardwareRuntimePulpCompatibilityBackend {
    fn execute_storage_access(
        &self,
        request: VaachakStorageAccessRequest,
    ) -> VaachakHardwareBackendHandoffResult {
        let supported_operation = matches!(
            request.operation,
            VaachakStorageAccessBackendOperation::DirectoryListing
                | VaachakStorageAccessBackendOperation::FileOpen
                | VaachakStorageAccessBackendOperation::FileReadChunk
                | VaachakStorageAccessBackendOperation::StateCachePathResolution
        );
        let supported_path = matches!(
            request.path_role,
            VaachakStoragePathRole::LibraryRoot
                | VaachakStoragePathRole::ReaderBookPath
                | VaachakStoragePathRole::StatePath
                | VaachakStoragePathRole::CachePath
        );
        Self::result(
            VaachakHardwareBackendDomain::StorageFatAccess,
            supported_operation && supported_path && !request.destructive_operation_allowed,
        )
    }
}

impl VaachakDisplayExecutor for VaachakHardwareRuntimePulpCompatibilityBackend {
    fn execute_display(
        &self,
        request: VaachakDisplayRequest,
    ) -> VaachakHardwareBackendHandoffResult {
        let supported_operation = matches!(
            request.operation,
            VaachakDisplayBackendOperation::FullRefresh
                | VaachakDisplayBackendOperation::PartialRefresh
                | VaachakDisplayBackendOperation::Clear
                | VaachakDisplayBackendOperation::Sleep
                | VaachakDisplayBackendOperation::RenderIntent
        );
        Self::result(
            VaachakHardwareBackendDomain::Display,
            supported_operation
                && request.spi_handoff_required
                && !request.display_draw_algorithm_rewrite_allowed,
        )
    }
}

impl VaachakInputExecutor for VaachakHardwareRuntimePulpCompatibilityBackend {
    fn execute_input(&self, request: VaachakInputRequest) -> VaachakHardwareBackendHandoffResult {
        let supported_operation = matches!(
            request.operation,
            super::hardware_runtime_backend::VaachakInputBackendOperation::ButtonScan
                | super::hardware_runtime_backend::VaachakInputBackendOperation::NavigationHandoff
                | super::hardware_runtime_backend::VaachakInputBackendOperation::InputTaskHandoff
        );
        Self::result(
            VaachakHardwareBackendDomain::Input,
            supported_operation
                && request.adc_ladder_owner_required
                && !request.input_debounce_navigation_rewrite_allowed,
        )
    }
}
