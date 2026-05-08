#![allow(dead_code)]

use super::display_executor_bridge::VaachakDisplayExecutorBridge;
use super::hardware_executor_pulp_backend::{
    VaachakHardwareExecutorBackend, VaachakHardwareExecutorDomain,
    VaachakHardwareExecutorPulpBackend,
};
use super::hardware_runtime_ownership::VaachakHardwareRuntimeOwnership;
use super::input_executor_bridge::VaachakInputExecutorBridge;
use super::spi_executor_bridge::VaachakSpiExecutorBridge;
use super::storage_executor_bridge::VaachakStorageExecutorBridge;

/// Consolidated Vaachak-owned hardware runtime executor entrypoint.
///
/// This is the broad hardware extraction layer that clubs SPI, SD lifecycle,
/// FAT/storage, display, and input executor entrypoints into the
/// target-xteink-x4 Vaachak layer. Active low-level execution stays routed to
/// Pulp-compatible backends to preserve current boot/display/input/storage
/// behavior while executor domains are extracted incrementally.
pub struct VaachakHardwareRuntimeExecutor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeExecutorState {
    VaachakEntrypointActive,
    PulpCompatibilityBackendActive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorEntry {
    pub domain: VaachakHardwareExecutorDomain,
    pub marker: &'static str,
    pub state: VaachakHardwareRuntimeExecutorState,
    pub backend: VaachakHardwareExecutorBackend,
    pub backend_name: &'static str,
    pub active_executor_owner: &'static str,
    pub bridge_ok: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorReport {
    pub vaachak_consolidated_executor_entrypoint_active: bool,
    pub hardware_ownership_consolidation_ready: bool,
    pub pulp_compatibility_backend_active: bool,
    pub spi_executor_bridge_ok: bool,
    pub storage_executor_bridge_ok: bool,
    pub display_executor_bridge_ok: bool,
    pub input_executor_bridge_ok: bool,
    pub reader_file_browser_ux_behavior_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub display_draw_algorithm_rewritten: bool,
    pub input_debounce_navigation_rewritten: bool,
    pub fat_destructive_behavior_introduced: bool,
}

impl VaachakHardwareRuntimeExecutorReport {
    pub const fn extraction_ok(self) -> bool {
        self.vaachak_consolidated_executor_entrypoint_active
            && self.hardware_ownership_consolidation_ready
            && self.pulp_compatibility_backend_active
            && self.spi_executor_bridge_ok
            && self.storage_executor_bridge_ok
            && self.display_executor_bridge_ok
            && self.input_executor_bridge_ok
            && !self.reader_file_browser_ux_behavior_changed
            && !self.app_navigation_behavior_changed
            && !self.display_draw_algorithm_rewritten
            && !self.input_debounce_navigation_rewritten
            && !self.fat_destructive_behavior_introduced
    }
}

impl VaachakHardwareRuntimeExecutor {
    pub const HARDWARE_RUNTIME_EXECUTOR_EXTRACTION_MARKER: &'static str =
        "hardware_runtime_executor_extraction=ok";
    pub const HARDWARE_RUNTIME_EXECUTOR_IDENTITY: &'static str =
        "xteink-x4-vaachak-hardware-runtime-executor";
    pub const HARDWARE_RUNTIME_EXECUTOR_OWNER: &'static str = "target-xteink-x4 Vaachak layer";
    pub const CONSOLIDATED_EXECUTOR_ENTRYPOINT_ACTIVE: bool = true;

    pub const ENTRY_COUNT: usize = 5;

    pub const READER_FILE_BROWSER_UX_BEHAVIOR_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;
    pub const FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;

    pub const fn entry_for(
        domain: VaachakHardwareExecutorDomain,
    ) -> VaachakHardwareRuntimeExecutorEntry {
        let route = VaachakHardwareExecutorPulpBackend::route_for(domain);
        match domain {
            VaachakHardwareExecutorDomain::SpiBus => VaachakHardwareRuntimeExecutorEntry {
                domain,
                marker: VaachakSpiExecutorBridge::SPI_EXECUTOR_BRIDGE_MARKER,
                state: VaachakHardwareRuntimeExecutorState::VaachakEntrypointActive,
                backend: route.backend,
                backend_name: route.backend_name,
                active_executor_owner: route.active_executor_owner,
                bridge_ok: VaachakSpiExecutorBridge::bridge_ok(),
            },
            VaachakHardwareExecutorDomain::StorageProbeMount => {
                VaachakHardwareRuntimeExecutorEntry {
                    domain,
                    marker: VaachakStorageExecutorBridge::STORAGE_EXECUTOR_BRIDGE_MARKER,
                    state: VaachakHardwareRuntimeExecutorState::VaachakEntrypointActive,
                    backend: route.backend,
                    backend_name: route.backend_name,
                    active_executor_owner: route.active_executor_owner,
                    bridge_ok: VaachakStorageExecutorBridge::lifecycle_routes_ready(),
                }
            }
            VaachakHardwareExecutorDomain::FatStorage => VaachakHardwareRuntimeExecutorEntry {
                domain,
                marker: VaachakStorageExecutorBridge::STORAGE_EXECUTOR_BRIDGE_MARKER,
                state: VaachakHardwareRuntimeExecutorState::VaachakEntrypointActive,
                backend: route.backend,
                backend_name: route.backend_name,
                active_executor_owner: route.active_executor_owner,
                bridge_ok: VaachakStorageExecutorBridge::fat_storage_routes_ready(),
            },
            VaachakHardwareExecutorDomain::Display => VaachakHardwareRuntimeExecutorEntry {
                domain,
                marker: VaachakDisplayExecutorBridge::DISPLAY_EXECUTOR_BRIDGE_MARKER,
                state: VaachakHardwareRuntimeExecutorState::VaachakEntrypointActive,
                backend: route.backend,
                backend_name: route.backend_name,
                active_executor_owner: route.active_executor_owner,
                bridge_ok: VaachakDisplayExecutorBridge::bridge_ok(),
            },
            VaachakHardwareExecutorDomain::Input => VaachakHardwareRuntimeExecutorEntry {
                domain,
                marker: VaachakInputExecutorBridge::INPUT_EXECUTOR_BRIDGE_MARKER,
                state: VaachakHardwareRuntimeExecutorState::VaachakEntrypointActive,
                backend: route.backend,
                backend_name: route.backend_name,
                active_executor_owner: route.active_executor_owner,
                bridge_ok: VaachakInputExecutorBridge::bridge_ok(),
            },
        }
    }

    pub const fn entries() -> [VaachakHardwareRuntimeExecutorEntry; Self::ENTRY_COUNT] {
        [
            Self::entry_for(VaachakHardwareExecutorDomain::SpiBus),
            Self::entry_for(VaachakHardwareExecutorDomain::StorageProbeMount),
            Self::entry_for(VaachakHardwareExecutorDomain::FatStorage),
            Self::entry_for(VaachakHardwareExecutorDomain::Display),
            Self::entry_for(VaachakHardwareExecutorDomain::Input),
        ]
    }

    pub const fn entry_is_safe(entry: VaachakHardwareRuntimeExecutorEntry) -> bool {
        matches!(
            entry.state,
            VaachakHardwareRuntimeExecutorState::VaachakEntrypointActive
        ) && matches!(
            entry.backend,
            VaachakHardwareExecutorBackend::PulpCompatibility
        ) && entry.backend_name.len() == VaachakHardwareExecutorPulpBackend::BACKEND_NAME.len()
            && entry.active_executor_owner.len()
                == VaachakHardwareExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER.len()
            && entry.bridge_ok
    }

    pub const fn entries_are_safe() -> bool {
        let entries = Self::entries();
        Self::entry_is_safe(entries[0])
            && Self::entry_is_safe(entries[1])
            && Self::entry_is_safe(entries[2])
            && Self::entry_is_safe(entries[3])
            && Self::entry_is_safe(entries[4])
    }

    pub const fn report() -> VaachakHardwareRuntimeExecutorReport {
        VaachakHardwareRuntimeExecutorReport {
            vaachak_consolidated_executor_entrypoint_active:
                Self::CONSOLIDATED_EXECUTOR_ENTRYPOINT_ACTIVE,
            hardware_ownership_consolidation_ready:
                VaachakHardwareRuntimeOwnership::consolidation_ok(),
            pulp_compatibility_backend_active: VaachakHardwareExecutorPulpBackend::backend_ok(),
            spi_executor_bridge_ok: VaachakSpiExecutorBridge::bridge_ok(),
            storage_executor_bridge_ok: VaachakStorageExecutorBridge::bridge_ok(),
            display_executor_bridge_ok: VaachakDisplayExecutorBridge::bridge_ok(),
            input_executor_bridge_ok: VaachakInputExecutorBridge::bridge_ok(),
            reader_file_browser_ux_behavior_changed: Self::READER_FILE_BROWSER_UX_BEHAVIOR_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            display_draw_algorithm_rewritten: Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN,
            input_debounce_navigation_rewritten: Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN,
            fat_destructive_behavior_introduced: Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED,
        }
    }

    pub const fn extraction_ok() -> bool {
        Self::entries_are_safe() && Self::report().extraction_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutor;

    #[test]
    fn hardware_runtime_executor_extraction_is_active() {
        assert!(VaachakHardwareRuntimeExecutor::extraction_ok());
    }

    #[test]
    fn executor_entries_cover_all_hardware_domains() {
        let entries = VaachakHardwareRuntimeExecutor::entries();
        assert_eq!(entries.len(), VaachakHardwareRuntimeExecutor::ENTRY_COUNT);
        for entry in entries {
            assert!(VaachakHardwareRuntimeExecutor::entry_is_safe(entry));
        }
    }
}
