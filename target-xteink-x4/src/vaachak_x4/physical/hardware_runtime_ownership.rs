#![allow(dead_code)]

use super::display_runtime_owner::VaachakDisplayRuntimeOwner;
use super::input_runtime_owner::VaachakInputRuntimeOwner;
use super::sd_fat_runtime_readonly_owner::VaachakSdFatRuntimeReadonlyOwner;
use super::spi_bus_runtime_owner::VaachakSpiBusRuntimeOwner;
use super::storage_probe_mount_runtime_owner::VaachakStorageProbeMountRuntimeOwner;

/// Canonical Vaachak-owned hardware runtime ownership map for Xteink X4.
///
/// This module clubs the accepted SPI, SD probe/mount, SD/FAT read-only,
/// display, and input runtime ownership slices into one entrypoint. It records
/// that Vaachak now owns hardware runtime authority boundaries while the active
/// hardware executors remain the existing imported Pulp compatibility runtime.
/// It does not move SPI transfers, SD probe/mount execution, FAT file I/O,
/// SSD1677 drawing/refresh, button ADC sampling, debounce, navigation dispatch,
/// reader behavior, or file-browser behavior.
pub struct VaachakHardwareRuntimeOwnership;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeOwnerKind {
    SpiBus,
    StorageProbeMount,
    SdFatReadonly,
    Display,
    Input,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeBackend {
    PulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeMigrationState {
    AuthorityOwnedByVaachak,
    ExecutorRemainsPulpCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeOwnershipEntry {
    pub owner: VaachakHardwareRuntimeOwnerKind,
    pub identity: &'static str,
    pub marker: &'static str,
    pub migration_state: VaachakHardwareRuntimeMigrationState,
    pub active_backend: VaachakHardwareRuntimeBackend,
    pub active_backend_name: &'static str,
    pub active_executor_owner: &'static str,
    pub ownership_authority_moved_to_vaachak: bool,
    pub ownership_ok: bool,
    pub executor_moved_to_vaachak: bool,
    pub reader_file_browser_behavior_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeOwnershipReport {
    pub spi_bus_owner_ok: bool,
    pub storage_probe_mount_owner_ok: bool,
    pub sd_fat_readonly_owner_ok: bool,
    pub display_owner_ok: bool,
    pub input_owner_ok: bool,
    pub all_authorities_moved_to_vaachak: bool,
    pub all_active_backends_are_pulp_compatibility: bool,
    pub no_executor_behavior_moved_to_vaachak: bool,
    pub no_reader_file_browser_behavior_changed: bool,
    pub no_storage_behavior_changed_by_display_or_input: bool,
    pub no_display_behavior_changed_by_storage_or_input: bool,
}

impl VaachakHardwareRuntimeOwnershipReport {
    pub const fn consolidation_ok(self) -> bool {
        self.spi_bus_owner_ok
            && self.storage_probe_mount_owner_ok
            && self.sd_fat_readonly_owner_ok
            && self.display_owner_ok
            && self.input_owner_ok
            && self.all_authorities_moved_to_vaachak
            && self.all_active_backends_are_pulp_compatibility
            && self.no_executor_behavior_moved_to_vaachak
            && self.no_reader_file_browser_behavior_changed
            && self.no_storage_behavior_changed_by_display_or_input
            && self.no_display_behavior_changed_by_storage_or_input
    }
}

impl VaachakHardwareRuntimeOwnership {
    pub const HARDWARE_RUNTIME_OWNERSHIP_CONSOLIDATION_MARKER: &'static str =
        "hardware_runtime_ownership_consolidation=ok";
    pub const HARDWARE_RUNTIME_OWNERSHIP_IDENTITY: &'static str =
        "xteink-x4-hardware-runtime-ownership";
    pub const OWNERSHIP_AUTHORITY: &'static str = "target-xteink-x4 Vaachak layer";
    pub const ACTIVE_EXECUTOR_OWNER: &'static str = "vendor/pulp-os imported runtime";
    pub const ACTIVE_BACKEND: VaachakHardwareRuntimeBackend =
        VaachakHardwareRuntimeBackend::PulpCompatibility;

    pub const OWNER_COUNT: usize = 5;

    pub const fn spi_bus_entry() -> VaachakHardwareRuntimeOwnershipEntry {
        VaachakHardwareRuntimeOwnershipEntry {
            owner: VaachakHardwareRuntimeOwnerKind::SpiBus,
            identity: VaachakSpiBusRuntimeOwner::SPI_BUS_IDENTITY,
            marker: VaachakSpiBusRuntimeOwner::SPI_BUS_RUNTIME_OWNERSHIP_MARKER,
            migration_state: VaachakHardwareRuntimeMigrationState::AuthorityOwnedByVaachak,
            active_backend: Self::ACTIVE_BACKEND,
            active_backend_name: VaachakSpiBusRuntimeOwner::ACTIVE_BACKEND_NAME,
            active_executor_owner: VaachakSpiBusRuntimeOwner::ACTIVE_EXECUTOR_OWNER,
            ownership_authority_moved_to_vaachak:
                VaachakSpiBusRuntimeOwner::SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            ownership_ok: VaachakSpiBusRuntimeOwner::ownership_bridge_ok(),
            executor_moved_to_vaachak:
                VaachakSpiBusRuntimeOwner::ARBITRATION_POLICY_MOVED_TO_VAACHAK,
            reader_file_browser_behavior_changed:
                VaachakSpiBusRuntimeOwner::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn storage_probe_mount_entry() -> VaachakHardwareRuntimeOwnershipEntry {
        VaachakHardwareRuntimeOwnershipEntry {
            owner: VaachakHardwareRuntimeOwnerKind::StorageProbeMount,
            identity: VaachakStorageProbeMountRuntimeOwner::STORAGE_PROBE_MOUNT_IDENTITY,
            marker:
                VaachakStorageProbeMountRuntimeOwner::STORAGE_PROBE_MOUNT_RUNTIME_OWNERSHIP_MARKER,
            migration_state: VaachakHardwareRuntimeMigrationState::AuthorityOwnedByVaachak,
            active_backend: Self::ACTIVE_BACKEND,
            active_backend_name: VaachakStorageProbeMountRuntimeOwner::ACTIVE_BACKEND_NAME,
            active_executor_owner: VaachakStorageProbeMountRuntimeOwner::ACTIVE_EXECUTOR_OWNER,
            ownership_authority_moved_to_vaachak:
                VaachakStorageProbeMountRuntimeOwner::STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            ownership_ok: VaachakStorageProbeMountRuntimeOwner::ownership_ok(),
            executor_moved_to_vaachak:
                VaachakStorageProbeMountRuntimeOwner::SD_EXECUTOR_MOVED_TO_VAACHAK,
            reader_file_browser_behavior_changed:
                VaachakStorageProbeMountRuntimeOwner::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn sd_fat_readonly_entry() -> VaachakHardwareRuntimeOwnershipEntry {
        VaachakHardwareRuntimeOwnershipEntry {
            owner: VaachakHardwareRuntimeOwnerKind::SdFatReadonly,
            identity: VaachakSdFatRuntimeReadonlyOwner::SD_FAT_RUNTIME_READONLY_IDENTITY,
            marker: VaachakSdFatRuntimeReadonlyOwner::SD_FAT_RUNTIME_READONLY_OWNERSHIP_MARKER,
            migration_state: VaachakHardwareRuntimeMigrationState::AuthorityOwnedByVaachak,
            active_backend: Self::ACTIVE_BACKEND,
            active_backend_name: VaachakSdFatRuntimeReadonlyOwner::ACTIVE_BACKEND_NAME,
            active_executor_owner: VaachakSdFatRuntimeReadonlyOwner::ACTIVE_EXECUTOR_OWNER,
            ownership_authority_moved_to_vaachak:
                VaachakSdFatRuntimeReadonlyOwner::SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            ownership_ok: VaachakSdFatRuntimeReadonlyOwner::ownership_ok(),
            executor_moved_to_vaachak:
                VaachakSdFatRuntimeReadonlyOwner::FAT_READONLY_EXECUTOR_MOVED_TO_VAACHAK,
            reader_file_browser_behavior_changed:
                VaachakSdFatRuntimeReadonlyOwner::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn display_entry() -> VaachakHardwareRuntimeOwnershipEntry {
        VaachakHardwareRuntimeOwnershipEntry {
            owner: VaachakHardwareRuntimeOwnerKind::Display,
            identity: VaachakDisplayRuntimeOwner::DISPLAY_RUNTIME_IDENTITY,
            marker: VaachakDisplayRuntimeOwner::DISPLAY_RUNTIME_OWNERSHIP_MARKER,
            migration_state: VaachakHardwareRuntimeMigrationState::AuthorityOwnedByVaachak,
            active_backend: Self::ACTIVE_BACKEND,
            active_backend_name: VaachakDisplayRuntimeOwner::ACTIVE_BACKEND_NAME,
            active_executor_owner: VaachakDisplayRuntimeOwner::ACTIVE_EXECUTOR_OWNER,
            ownership_authority_moved_to_vaachak:
                VaachakDisplayRuntimeOwner::DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            ownership_ok: VaachakDisplayRuntimeOwner::ownership_ok(),
            executor_moved_to_vaachak:
                VaachakDisplayRuntimeOwner::SSD1677_EXECUTOR_MOVED_TO_VAACHAK,
            reader_file_browser_behavior_changed:
                VaachakDisplayRuntimeOwner::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn input_entry() -> VaachakHardwareRuntimeOwnershipEntry {
        VaachakHardwareRuntimeOwnershipEntry {
            owner: VaachakHardwareRuntimeOwnerKind::Input,
            identity: VaachakInputRuntimeOwner::INPUT_RUNTIME_IDENTITY,
            marker: VaachakInputRuntimeOwner::INPUT_RUNTIME_OWNERSHIP_MARKER,
            migration_state: VaachakHardwareRuntimeMigrationState::AuthorityOwnedByVaachak,
            active_backend: Self::ACTIVE_BACKEND,
            active_backend_name: VaachakInputRuntimeOwner::ACTIVE_BACKEND_NAME,
            active_executor_owner: VaachakInputRuntimeOwner::ACTIVE_EXECUTOR_OWNER,
            ownership_authority_moved_to_vaachak:
                VaachakInputRuntimeOwner::INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK,
            ownership_ok: VaachakInputRuntimeOwner::ownership_ok(),
            executor_moved_to_vaachak:
                VaachakInputRuntimeOwner::BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK,
            reader_file_browser_behavior_changed:
                VaachakInputRuntimeOwner::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn entries() -> [VaachakHardwareRuntimeOwnershipEntry; Self::OWNER_COUNT] {
        [
            Self::spi_bus_entry(),
            Self::storage_probe_mount_entry(),
            Self::sd_fat_readonly_entry(),
            Self::display_entry(),
            Self::input_entry(),
        ]
    }

    pub const fn all_authorities_moved_to_vaachak() -> bool {
        VaachakSpiBusRuntimeOwner::SPI_BUS_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK
            && VaachakStorageProbeMountRuntimeOwner::STORAGE_PROBE_MOUNT_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK
            && VaachakSdFatRuntimeReadonlyOwner::SD_FAT_READONLY_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK
            && VaachakDisplayRuntimeOwner::DISPLAY_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK
            && VaachakInputRuntimeOwner::INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK
    }

    pub const fn all_active_backends_are_pulp_compatibility() -> bool {
        VaachakSpiBusRuntimeOwner::ownership_bridge_ok()
            && VaachakStorageProbeMountRuntimeOwner::ownership_ok()
            && VaachakSdFatRuntimeReadonlyOwner::ownership_ok()
            && VaachakDisplayRuntimeOwner::ownership_ok()
            && VaachakInputRuntimeOwner::ownership_ok()
    }

    pub const fn no_executor_behavior_moved_to_vaachak() -> bool {
        !VaachakSpiBusRuntimeOwner::ARBITRATION_POLICY_MOVED_TO_VAACHAK
            && !VaachakStorageProbeMountRuntimeOwner::SD_EXECUTOR_MOVED_TO_VAACHAK
            && !VaachakStorageProbeMountRuntimeOwner::FAT_BEHAVIOR_MOVED_TO_VAACHAK
            && !VaachakSdFatRuntimeReadonlyOwner::FAT_READONLY_EXECUTOR_MOVED_TO_VAACHAK
            && !VaachakSdFatRuntimeReadonlyOwner::FAT_WRITABLE_BEHAVIOR_MOVED_TO_VAACHAK
            && !VaachakDisplayRuntimeOwner::SSD1677_EXECUTOR_MOVED_TO_VAACHAK
            && !VaachakDisplayRuntimeOwner::DISPLAY_DRAW_EXECUTOR_MOVED_TO_VAACHAK
            && !VaachakDisplayRuntimeOwner::DISPLAY_REFRESH_EXECUTOR_MOVED_TO_VAACHAK
            && !VaachakDisplayRuntimeOwner::DISPLAY_PARTIAL_REFRESH_EXECUTOR_MOVED_TO_VAACHAK
            && !VaachakInputRuntimeOwner::ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK
            && !VaachakInputRuntimeOwner::BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK
            && !VaachakInputRuntimeOwner::DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK
            && !VaachakInputRuntimeOwner::NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK
    }

    pub const fn no_reader_file_browser_behavior_changed() -> bool {
        !VaachakSpiBusRuntimeOwner::READER_FILE_BROWSER_BEHAVIOR_CHANGED
            && !VaachakStorageProbeMountRuntimeOwner::READER_FILE_BROWSER_BEHAVIOR_CHANGED
            && !VaachakSdFatRuntimeReadonlyOwner::READER_FILE_BROWSER_BEHAVIOR_CHANGED
            && !VaachakDisplayRuntimeOwner::READER_FILE_BROWSER_BEHAVIOR_CHANGED
            && !VaachakInputRuntimeOwner::READER_FILE_BROWSER_BEHAVIOR_CHANGED
    }

    pub const fn no_storage_behavior_changed_by_display_or_input() -> bool {
        !VaachakDisplayRuntimeOwner::STORAGE_BEHAVIOR_CHANGED
            && !VaachakInputRuntimeOwner::STORAGE_BEHAVIOR_CHANGED
    }

    pub const fn no_display_behavior_changed_by_storage_or_input() -> bool {
        !VaachakStorageProbeMountRuntimeOwner::DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK
            && !VaachakSdFatRuntimeReadonlyOwner::DISPLAY_BEHAVIOR_MOVED_TO_VAACHAK
            && !VaachakInputRuntimeOwner::DISPLAY_BEHAVIOR_CHANGED
    }

    pub const fn report() -> VaachakHardwareRuntimeOwnershipReport {
        VaachakHardwareRuntimeOwnershipReport {
            spi_bus_owner_ok: VaachakSpiBusRuntimeOwner::ownership_bridge_ok(),
            storage_probe_mount_owner_ok: VaachakStorageProbeMountRuntimeOwner::ownership_ok(),
            sd_fat_readonly_owner_ok: VaachakSdFatRuntimeReadonlyOwner::ownership_ok(),
            display_owner_ok: VaachakDisplayRuntimeOwner::ownership_ok(),
            input_owner_ok: VaachakInputRuntimeOwner::ownership_ok(),
            all_authorities_moved_to_vaachak: Self::all_authorities_moved_to_vaachak(),
            all_active_backends_are_pulp_compatibility:
                Self::all_active_backends_are_pulp_compatibility(),
            no_executor_behavior_moved_to_vaachak: Self::no_executor_behavior_moved_to_vaachak(),
            no_reader_file_browser_behavior_changed: Self::no_reader_file_browser_behavior_changed(
            ),
            no_storage_behavior_changed_by_display_or_input:
                Self::no_storage_behavior_changed_by_display_or_input(),
            no_display_behavior_changed_by_storage_or_input:
                Self::no_display_behavior_changed_by_storage_or_input(),
        }
    }

    pub const fn consolidation_ok() -> bool {
        Self::report().consolidation_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{VaachakHardwareRuntimeOwnerKind, VaachakHardwareRuntimeOwnership};

    #[test]
    fn hardware_runtime_ownership_is_consolidated() {
        assert!(VaachakHardwareRuntimeOwnership::consolidation_ok());
    }

    #[test]
    fn all_five_owner_entries_are_present() {
        let entries = VaachakHardwareRuntimeOwnership::entries();
        assert_eq!(entries.len(), VaachakHardwareRuntimeOwnership::OWNER_COUNT);
        assert!(matches!(
            entries[0].owner,
            VaachakHardwareRuntimeOwnerKind::SpiBus
        ));
        assert!(matches!(
            entries[1].owner,
            VaachakHardwareRuntimeOwnerKind::StorageProbeMount
        ));
        assert!(matches!(
            entries[2].owner,
            VaachakHardwareRuntimeOwnerKind::SdFatReadonly
        ));
        assert!(matches!(
            entries[3].owner,
            VaachakHardwareRuntimeOwnerKind::Display
        ));
        assert!(matches!(
            entries[4].owner,
            VaachakHardwareRuntimeOwnerKind::Input
        ));
    }
}
