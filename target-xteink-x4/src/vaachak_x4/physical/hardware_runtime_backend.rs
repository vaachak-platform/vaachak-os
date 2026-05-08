#![allow(dead_code)]

use super::hardware_executor_pulp_backend::VaachakHardwareExecutorBackend;

/// Vaachak-owned backend interface for hardware runtime execution.
///
/// These traits are the takeover seam: Vaachak now owns callable backend
/// request/result contracts for SPI, storage lifecycle, storage/FAT access,
/// display, and input. The active implementation remains PulpCompatibility
/// until each low-level executor is migrated safely.
pub struct VaachakHardwareRuntimeBackendInterface;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareBackendDomain {
    SpiTransaction,
    StorageProbeMount,
    StorageFatAccess,
    Display,
    Input,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageProbeMountBackendIntent {
    CardAvailability,
    ProbeCard,
    MountReadiness,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageAccessBackendOperation {
    DirectoryListing,
    FileOpen,
    FileReadChunk,
    StateCachePathResolution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStoragePathRole {
    LibraryRoot,
    ReaderBookPath,
    StatePath,
    CachePath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakDisplayBackendOperation {
    FullRefresh,
    PartialRefresh,
    Clear,
    Sleep,
    RenderIntent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakInputBackendOperation {
    ButtonScan,
    NavigationHandoff,
    InputTaskHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiDisplayTransactionRequest {
    pub transaction_owner: &'static str,
    pub display_chip_select_gpio: u8,
    pub safe_arbitration_handoff_required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakSpiStorageTransactionRequest {
    pub transaction_owner: &'static str,
    pub storage_chip_select_gpio: u8,
    pub safe_arbitration_handoff_required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageProbeMountRequest {
    pub intent: VaachakStorageProbeMountBackendIntent,
    pub card_present_expected: bool,
    pub mount_readiness_required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageAccessRequest {
    pub operation: VaachakStorageAccessBackendOperation,
    pub path_role: VaachakStoragePathRole,
    pub destructive_operation_allowed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakDisplayRequest {
    pub operation: VaachakDisplayBackendOperation,
    pub spi_handoff_required: bool,
    pub display_draw_algorithm_rewrite_allowed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputRequest {
    pub operation: VaachakInputBackendOperation,
    pub adc_ladder_owner_required: bool,
    pub input_debounce_navigation_rewrite_allowed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareBackendHandoffResult {
    pub domain: VaachakHardwareBackendDomain,
    pub accepted: bool,
    pub backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub backend_owner: &'static str,
    pub low_level_executor_owner: &'static str,
    pub pulp_compatible_backend_active: bool,
    pub low_level_behavior_changed: bool,
    pub destructive_behavior_added: bool,
    pub ux_behavior_changed: bool,
}

impl VaachakHardwareBackendHandoffResult {
    pub const fn ok(self) -> bool {
        self.accepted
            && matches!(
                self.backend,
                VaachakHardwareExecutorBackend::PulpCompatibility
            )
            && self.pulp_compatible_backend_active
            && !self.low_level_behavior_changed
            && !self.destructive_behavior_added
            && !self.ux_behavior_changed
    }
}

pub trait VaachakSpiTransactionExecutor {
    fn execute_spi_display_transaction(
        &self,
        request: VaachakSpiDisplayTransactionRequest,
    ) -> VaachakHardwareBackendHandoffResult;

    fn execute_spi_storage_transaction(
        &self,
        request: VaachakSpiStorageTransactionRequest,
    ) -> VaachakHardwareBackendHandoffResult;
}

pub trait VaachakStorageProbeMountExecutor {
    fn execute_storage_probe_mount(
        &self,
        request: VaachakStorageProbeMountRequest,
    ) -> VaachakHardwareBackendHandoffResult;
}

pub trait VaachakStorageFatAccessExecutor {
    fn execute_storage_access(
        &self,
        request: VaachakStorageAccessRequest,
    ) -> VaachakHardwareBackendHandoffResult;
}

pub trait VaachakDisplayExecutor {
    fn execute_display(
        &self,
        request: VaachakDisplayRequest,
    ) -> VaachakHardwareBackendHandoffResult;
}

pub trait VaachakInputExecutor {
    fn execute_input(&self, request: VaachakInputRequest) -> VaachakHardwareBackendHandoffResult;
}

pub trait VaachakHardwareRuntimeBackend:
    VaachakSpiTransactionExecutor
    + VaachakStorageProbeMountExecutor
    + VaachakStorageFatAccessExecutor
    + VaachakDisplayExecutor
    + VaachakInputExecutor
{
}

impl<T> VaachakHardwareRuntimeBackend for T where
    T: VaachakSpiTransactionExecutor
        + VaachakStorageProbeMountExecutor
        + VaachakStorageFatAccessExecutor
        + VaachakDisplayExecutor
        + VaachakInputExecutor
{
}

impl VaachakHardwareRuntimeBackendInterface {
    pub const BACKEND_TRAITS_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const BACKEND_TRAIT_COUNT: usize = 5;
    pub const REQUEST_RESULT_STRUCTS_OWNED_BY_VAACHAK: bool = true;
    pub const LOW_LEVEL_PULP_EXECUTOR_REMAINS_ACTIVE: bool = true;
    pub const DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false;
    pub const SD_MMC_FAT_ALGORITHM_REWRITTEN: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_CHANGED: bool = false;

    pub const fn interface_ok() -> bool {
        Self::BACKEND_TRAIT_COUNT == 5
            && Self::REQUEST_RESULT_STRUCTS_OWNED_BY_VAACHAK
            && Self::LOW_LEVEL_PULP_EXECUTOR_REMAINS_ACTIVE
            && !Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN
            && !Self::SD_MMC_FAT_ALGORITHM_REWRITTEN
            && !Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN
            && !Self::READER_FILE_BROWSER_UX_CHANGED
            && !Self::APP_NAVIGATION_CHANGED
    }
}
