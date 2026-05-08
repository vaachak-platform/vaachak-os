#![allow(dead_code)]

use super::spi_bus_arbitration_runtime_owner::VaachakSpiBusArbitrationRuntimeOwner;
use super::spi_bus_runtime_owner::{VaachakSpiRuntimeUser, VaachakSpiTransactionKind};
use super::storage_probe_mount_executor_pulp_backend::{
    VaachakStorageProbeMountExecutorPath, VaachakStorageProbeMountExecutorPulpBackend,
};
use super::storage_probe_mount_runtime_owner::VaachakStorageProbeMountRuntimeOwner;

/// Vaachak-owned SD probe/mount lifecycle execution entrypoint.
///
/// This is the first narrow SD probe/mount executor migration slice. Vaachak now
/// owns the lifecycle intent entrypoint and routes intent through the accepted
/// SD probe/mount runtime owner and SPI arbitration owner. The active low-level
/// SD/MMC and FAT implementation remains the Pulp-compatible executor.
pub struct VaachakStorageProbeMountRuntimeExecutorBridge;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageProbeMountExecutorBackend {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageProbeMountLifecycleIntent {
    DetectCard,
    IdentifyCardAtSafeSpeed,
    ObserveCardAvailability,
    ObserveFatVolumeAvailability,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakStorageProbeMountLifecycleDecision {
    RoutedToPulpCompatibilityExecutor,
    RejectedBeforeHardwareExecution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageProbeMountLifecycleExecution {
    pub intent: VaachakStorageProbeMountLifecycleIntent,
    pub decision: VaachakStorageProbeMountLifecycleDecision,
    pub lifecycle_entrypoint_owner: &'static str,
    pub active_backend: VaachakStorageProbeMountExecutorBackend,
    pub active_backend_name: &'static str,
    pub active_executor_owner: &'static str,
    pub requested_spi_user: VaachakSpiRuntimeUser,
    pub requested_spi_kind: VaachakSpiTransactionKind,
    pub sd_chip_select_gpio: u8,
    pub sd_identification_khz: u32,
    pub fat_read_write_list_behavior_changed: bool,
    pub display_behavior_changed: bool,
    pub reader_file_browser_behavior_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageProbeMountRuntimeExecutorReport {
    pub lifecycle_entrypoint_moved_to_vaachak: bool,
    pub runtime_owner_ready: bool,
    pub spi_arbitration_owner_ready: bool,
    pub pulp_executor_backend_active: bool,
    pub detect_card_intent_routed: bool,
    pub identify_card_intent_routed: bool,
    pub availability_intent_routed: bool,
    pub fat_volume_observation_routed: bool,
    pub low_level_sd_mmc_executor_moved_to_vaachak: bool,
    pub fat_read_write_list_behavior_changed: bool,
    pub display_behavior_changed: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakStorageProbeMountRuntimeExecutorReport {
    pub const fn executor_bridge_ok(self) -> bool {
        self.lifecycle_entrypoint_moved_to_vaachak
            && self.runtime_owner_ready
            && self.spi_arbitration_owner_ready
            && self.pulp_executor_backend_active
            && self.detect_card_intent_routed
            && self.identify_card_intent_routed
            && self.availability_intent_routed
            && self.fat_volume_observation_routed
            && !self.low_level_sd_mmc_executor_moved_to_vaachak
            && !self.fat_read_write_list_behavior_changed
            && !self.display_behavior_changed
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakStorageProbeMountRuntimeExecutorBridge {
    pub const STORAGE_PROBE_MOUNT_RUNTIME_EXECUTOR_BRIDGE_MARKER: &'static str =
        "storage_probe_mount_runtime_executor_bridge=ok";
    pub const STORAGE_PROBE_MOUNT_RUNTIME_EXECUTOR_IDENTITY: &'static str =
        "xteink-x4-sd-probe-mount-runtime-executor-bridge";

    pub const LIFECYCLE_ENTRYPOINT_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const LIFECYCLE_ENTRYPOINT_MOVED_TO_VAACHAK: bool = true;

    pub const ACTIVE_BACKEND: VaachakStorageProbeMountExecutorBackend =
        VaachakStorageProbeMountExecutorBackend::PulpCompatibility;
    pub const ACTIVE_BACKEND_NAME: &'static str =
        VaachakStorageProbeMountExecutorPulpBackend::BACKEND_NAME;
    pub const ACTIVE_EXECUTOR_OWNER: &'static str =
        VaachakStorageProbeMountExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER;

    pub const LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn backend_path_for(
        intent: VaachakStorageProbeMountLifecycleIntent,
    ) -> VaachakStorageProbeMountExecutorPath {
        match intent {
            VaachakStorageProbeMountLifecycleIntent::DetectCard => {
                VaachakStorageProbeMountExecutorPath::CardDetectIntent
            }
            VaachakStorageProbeMountLifecycleIntent::IdentifyCardAtSafeSpeed => {
                VaachakStorageProbeMountExecutorPath::CardIdentifyIntent
            }
            VaachakStorageProbeMountLifecycleIntent::ObserveCardAvailability => {
                VaachakStorageProbeMountExecutorPath::AvailabilityIntent
            }
            VaachakStorageProbeMountLifecycleIntent::ObserveFatVolumeAvailability => {
                VaachakStorageProbeMountExecutorPath::FatVolumeObservationIntent
            }
        }
    }

    pub const fn spi_kind_for(
        intent: VaachakStorageProbeMountLifecycleIntent,
    ) -> VaachakSpiTransactionKind {
        match intent {
            VaachakStorageProbeMountLifecycleIntent::DetectCard
            | VaachakStorageProbeMountLifecycleIntent::IdentifyCardAtSafeSpeed
            | VaachakStorageProbeMountLifecycleIntent::ObserveCardAvailability => {
                VaachakSpiTransactionKind::StorageProbeMetadata
            }
            VaachakStorageProbeMountLifecycleIntent::ObserveFatVolumeAvailability => {
                VaachakSpiTransactionKind::StorageMountMetadata
            }
        }
    }

    pub const fn execute_lifecycle_intent(
        intent: VaachakStorageProbeMountLifecycleIntent,
    ) -> VaachakStorageProbeMountLifecycleExecution {
        let spi_kind = Self::spi_kind_for(intent);
        let request = VaachakSpiBusArbitrationRuntimeOwner::request_for(
            VaachakSpiRuntimeUser::Storage,
            spi_kind,
        );
        let grant = VaachakSpiBusArbitrationRuntimeOwner::grant_for(request);
        let decision = if VaachakSpiBusArbitrationRuntimeOwner::grant_is_safe(grant)
            && VaachakStorageProbeMountRuntimeOwner::ownership_ok()
            && VaachakStorageProbeMountExecutorPulpBackend::backend_ok()
        {
            VaachakStorageProbeMountLifecycleDecision::RoutedToPulpCompatibilityExecutor
        } else {
            VaachakStorageProbeMountLifecycleDecision::RejectedBeforeHardwareExecution
        };

        VaachakStorageProbeMountLifecycleExecution {
            intent,
            decision,
            lifecycle_entrypoint_owner: Self::LIFECYCLE_ENTRYPOINT_OWNER,
            active_backend: Self::ACTIVE_BACKEND,
            active_backend_name: Self::ACTIVE_BACKEND_NAME,
            active_executor_owner: VaachakStorageProbeMountExecutorPulpBackend::executor_owner_for(
                Self::backend_path_for(intent),
            ),
            requested_spi_user: VaachakSpiRuntimeUser::Storage,
            requested_spi_kind: spi_kind,
            sd_chip_select_gpio: VaachakStorageProbeMountRuntimeOwner::STORAGE_SD_CS_GPIO,
            sd_identification_khz: VaachakStorageProbeMountRuntimeOwner::SD_IDENTIFICATION_KHZ,
            fat_read_write_list_behavior_changed: Self::FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn execution_is_routed(
        execution: VaachakStorageProbeMountLifecycleExecution,
    ) -> bool {
        matches!(
            execution.decision,
            VaachakStorageProbeMountLifecycleDecision::RoutedToPulpCompatibilityExecutor
        ) && matches!(
            execution.active_backend,
            VaachakStorageProbeMountExecutorBackend::PulpCompatibility
        ) && execution.lifecycle_entrypoint_owner.len() == Self::LIFECYCLE_ENTRYPOINT_OWNER.len()
            && execution.active_backend_name.len() == Self::ACTIVE_BACKEND_NAME.len()
            && execution.sd_chip_select_gpio
                == VaachakStorageProbeMountRuntimeOwner::STORAGE_SD_CS_GPIO
            && execution.sd_identification_khz
                == VaachakStorageProbeMountRuntimeOwner::SD_IDENTIFICATION_KHZ
            && !execution.fat_read_write_list_behavior_changed
            && !execution.display_behavior_changed
            && !execution.reader_file_browser_behavior_changed
    }

    pub const fn detect_card_intent_routed() -> bool {
        Self::execution_is_routed(Self::execute_lifecycle_intent(
            VaachakStorageProbeMountLifecycleIntent::DetectCard,
        ))
    }

    pub const fn identify_card_intent_routed() -> bool {
        Self::execution_is_routed(Self::execute_lifecycle_intent(
            VaachakStorageProbeMountLifecycleIntent::IdentifyCardAtSafeSpeed,
        ))
    }

    pub const fn availability_intent_routed() -> bool {
        Self::execution_is_routed(Self::execute_lifecycle_intent(
            VaachakStorageProbeMountLifecycleIntent::ObserveCardAvailability,
        ))
    }

    pub const fn fat_volume_observation_routed() -> bool {
        Self::execution_is_routed(Self::execute_lifecycle_intent(
            VaachakStorageProbeMountLifecycleIntent::ObserveFatVolumeAvailability,
        ))
    }

    pub const fn report() -> VaachakStorageProbeMountRuntimeExecutorReport {
        VaachakStorageProbeMountRuntimeExecutorReport {
            lifecycle_entrypoint_moved_to_vaachak: Self::LIFECYCLE_ENTRYPOINT_MOVED_TO_VAACHAK,
            runtime_owner_ready: VaachakStorageProbeMountRuntimeOwner::ownership_ok(),
            spi_arbitration_owner_ready: VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok(),
            pulp_executor_backend_active: VaachakStorageProbeMountExecutorPulpBackend::backend_ok(),
            detect_card_intent_routed: Self::detect_card_intent_routed(),
            identify_card_intent_routed: Self::identify_card_intent_routed(),
            availability_intent_routed: Self::availability_intent_routed(),
            fat_volume_observation_routed: Self::fat_volume_observation_routed(),
            low_level_sd_mmc_executor_moved_to_vaachak:
                Self::LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK,
            fat_read_write_list_behavior_changed: Self::FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED,
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn executor_bridge_ok() -> bool {
        Self::report().executor_bridge_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        VaachakStorageProbeMountLifecycleDecision, VaachakStorageProbeMountLifecycleIntent,
        VaachakStorageProbeMountRuntimeExecutorBridge,
    };

    #[test]
    fn storage_probe_mount_runtime_executor_bridge_is_active() {
        assert!(VaachakStorageProbeMountRuntimeExecutorBridge::executor_bridge_ok());
    }

    #[test]
    fn lifecycle_intent_routes_through_vaachak_entrypoint_to_pulp_backend() {
        let execution = VaachakStorageProbeMountRuntimeExecutorBridge::execute_lifecycle_intent(
            VaachakStorageProbeMountLifecycleIntent::DetectCard,
        );
        assert_eq!(
            execution.decision,
            VaachakStorageProbeMountLifecycleDecision::RoutedToPulpCompatibilityExecutor
        );
        assert!(VaachakStorageProbeMountRuntimeExecutorBridge::execution_is_routed(execution));
    }
}
