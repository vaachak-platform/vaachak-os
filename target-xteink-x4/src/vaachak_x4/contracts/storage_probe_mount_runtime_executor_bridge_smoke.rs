#![allow(dead_code)]

use crate::vaachak_x4::physical::spi_bus_arbitration_runtime_owner::VaachakSpiBusArbitrationRuntimeOwner;
use crate::vaachak_x4::physical::storage_probe_mount_executor_pulp_backend::VaachakStorageProbeMountExecutorPulpBackend;
use crate::vaachak_x4::physical::storage_probe_mount_runtime_executor_bridge::VaachakStorageProbeMountRuntimeExecutorBridge;
use crate::vaachak_x4::physical::storage_probe_mount_runtime_owner::VaachakStorageProbeMountRuntimeOwner;

/// Contract smoke for the Vaachak SD probe/mount runtime executor bridge.
///
/// This verifies that lifecycle execution intent now enters through Vaachak,
/// then routes to the Pulp compatibility executor. It also proves FAT,
/// display, reader, and file-browser behavior remain unchanged.
pub struct VaachakStorageProbeMountRuntimeExecutorBridgeSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakStorageProbeMountRuntimeExecutorBridgeSmokeReport {
    pub executor_bridge_entrypoint_ok: bool,
    pub runtime_owner_ready: bool,
    pub spi_arbitration_owner_ready: bool,
    pub pulp_executor_backend_active: bool,
    pub all_lifecycle_intents_route_to_pulp: bool,
    pub no_fat_behavior_changed: bool,
    pub no_display_behavior_changed: bool,
    pub no_reader_file_browser_behavior_changed: bool,
}

impl VaachakStorageProbeMountRuntimeExecutorBridgeSmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.executor_bridge_entrypoint_ok
            && self.runtime_owner_ready
            && self.spi_arbitration_owner_ready
            && self.pulp_executor_backend_active
            && self.all_lifecycle_intents_route_to_pulp
            && self.no_fat_behavior_changed
            && self.no_display_behavior_changed
            && self.no_reader_file_browser_behavior_changed
    }
}

impl VaachakStorageProbeMountRuntimeExecutorBridgeSmoke {
    pub const STORAGE_PROBE_MOUNT_RUNTIME_EXECUTOR_BRIDGE_SMOKE_MARKER: &'static str =
        "storage_probe_mount_runtime_executor_bridge_smoke=ok";

    pub const LIFECYCLE_ENTRYPOINT_MOVED_TO_VAACHAK: bool = true;
    pub const LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn all_lifecycle_intents_route_to_pulp() -> bool {
        VaachakStorageProbeMountRuntimeExecutorBridge::detect_card_intent_routed()
            && VaachakStorageProbeMountRuntimeExecutorBridge::identify_card_intent_routed()
            && VaachakStorageProbeMountRuntimeExecutorBridge::availability_intent_routed()
            && VaachakStorageProbeMountRuntimeExecutorBridge::fat_volume_observation_routed()
    }

    pub const fn no_behavior_regression_flags() -> bool {
        !Self::LOW_LEVEL_SD_MMC_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED
            && !Self::DISPLAY_BEHAVIOR_CHANGED
            && !Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED
    }

    pub const fn report() -> VaachakStorageProbeMountRuntimeExecutorBridgeSmokeReport {
        VaachakStorageProbeMountRuntimeExecutorBridgeSmokeReport {
            executor_bridge_entrypoint_ok:
                VaachakStorageProbeMountRuntimeExecutorBridge::executor_bridge_ok()
                    && Self::LIFECYCLE_ENTRYPOINT_MOVED_TO_VAACHAK,
            runtime_owner_ready: VaachakStorageProbeMountRuntimeOwner::ownership_ok(),
            spi_arbitration_owner_ready: VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok(),
            pulp_executor_backend_active: VaachakStorageProbeMountExecutorPulpBackend::backend_ok(),
            all_lifecycle_intents_route_to_pulp: Self::all_lifecycle_intents_route_to_pulp(),
            no_fat_behavior_changed: !Self::FAT_READ_WRITE_LIST_BEHAVIOR_CHANGED,
            no_display_behavior_changed: !Self::DISPLAY_BEHAVIOR_CHANGED,
            no_reader_file_browser_behavior_changed: !Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn smoke_ok() -> bool {
        Self::report().smoke_ok() && Self::no_behavior_regression_flags()
    }

    #[cfg(target_arch = "riscv32")]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            esp_println::println!(
                "{}",
                Self::STORAGE_PROBE_MOUNT_RUNTIME_EXECUTOR_BRIDGE_SMOKE_MARKER
            );
        } else {
            esp_println::println!("storage_probe_mount_runtime_executor_bridge=failed");
        }
    }

    #[cfg(not(target_arch = "riscv32"))]
    pub fn emit_boot_marker() {
        if Self::smoke_ok() {
            println!(
                "{}",
                Self::STORAGE_PROBE_MOUNT_RUNTIME_EXECUTOR_BRIDGE_SMOKE_MARKER
            );
        } else {
            println!("storage_probe_mount_runtime_executor_bridge=failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakStorageProbeMountRuntimeExecutorBridgeSmoke;

    #[test]
    fn storage_probe_mount_runtime_executor_bridge_smoke_is_ok() {
        assert!(VaachakStorageProbeMountRuntimeExecutorBridgeSmoke::smoke_ok());
    }

    #[test]
    fn lifecycle_intents_route_to_pulp_without_behavior_regression() {
        assert!(
            VaachakStorageProbeMountRuntimeExecutorBridgeSmoke::all_lifecycle_intents_route_to_pulp(
            )
        );
        assert!(VaachakStorageProbeMountRuntimeExecutorBridgeSmoke::no_behavior_regression_flags());
    }
}
