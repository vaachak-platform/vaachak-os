#![allow(dead_code)]

use super::display_executor_bridge::{VaachakDisplayExecutorBridge, VaachakDisplayExecutorIntent};
use super::hardware_executor_pulp_backend::{
    VaachakHardwareExecutorBackend, VaachakHardwareExecutorDomain,
    VaachakHardwareExecutorPulpBackend,
};
use super::hardware_runtime_executor::{
    VaachakHardwareRuntimeExecutor, VaachakHardwareRuntimeExecutorEntry,
    VaachakHardwareRuntimeExecutorState,
};
use super::hardware_runtime_wiring_pulp_backend::{
    VaachakHardwareRuntimeWiringBackendRoute, VaachakHardwareRuntimeWiringPulpBackend,
};
use super::input_executor_bridge::{VaachakInputExecutorBridge, VaachakInputExecutorIntent};
use super::spi_bus_runtime_owner::{VaachakSpiRuntimeUser, VaachakSpiTransactionKind};
use super::spi_executor_bridge::{VaachakSpiExecutorBridge, VaachakSpiExecutorIntent};
use super::storage_executor_bridge::{VaachakStorageExecutorBridge, VaachakStorageExecutorIntent};

/// Vaachak-owned hardware runtime executor wiring layer.
///
/// This layer starts routing selected internal runtime intents through the
/// consolidated `VaachakHardwareRuntimeExecutor` entrypoint. It intentionally
/// keeps the low-level Pulp-compatible executors active so the current boot,
/// display, input, SD/storage, reader, file-browser, and app navigation behavior
/// remains unchanged.
pub struct VaachakHardwareRuntimeExecutorWiring;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareWiredRuntimePath {
    BootStorageAvailability,
    LibraryDirectoryListing,
    ReaderFileOpenIntent,
    ReaderFileChunkIntent,
    DisplayFullRefreshHandoff,
    DisplayPartialRefreshHandoff,
    InputButtonScanHandoff,
    InputNavigationHandoff,
    SharedSpiDisplayHandoff,
    SharedSpiStorageHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaachakHardwareRuntimeWiringDecision {
    RoutedThroughVaachakHardwareRuntimeExecutor,
    RejectedBeforePulpCompatibleBackend,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeWiringRoute {
    pub path: VaachakHardwareWiredRuntimePath,
    pub domain: VaachakHardwareExecutorDomain,
    pub decision: VaachakHardwareRuntimeWiringDecision,
    pub executor_entry: VaachakHardwareRuntimeExecutorEntry,
    pub backend_route: VaachakHardwareRuntimeWiringBackendRoute,
    pub consolidated_executor_ready: bool,
    pub domain_bridge_ready: bool,
    pub low_level_backend_still_pulp_compatible: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_behavior_changed: bool,
    pub display_draw_algorithm_rewritten: bool,
    pub input_debounce_navigation_rewritten: bool,
    pub fat_destructive_behavior_introduced: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorWiringReport {
    pub wiring_entrypoint_active: bool,
    pub consolidated_executor_ready: bool,
    pub pulp_backend_active: bool,
    pub spi_paths_wired: bool,
    pub storage_paths_wired: bool,
    pub display_paths_wired: bool,
    pub input_paths_wired: bool,
    pub reader_file_browser_ux_unchanged: bool,
    pub app_navigation_unchanged: bool,
    pub display_draw_algorithm_unchanged: bool,
    pub input_debounce_navigation_unchanged: bool,
    pub no_fat_destructive_behavior: bool,
}

impl VaachakHardwareRuntimeExecutorWiringReport {
    pub const fn ok(self) -> bool {
        self.wiring_entrypoint_active
            && self.consolidated_executor_ready
            && self.pulp_backend_active
            && self.spi_paths_wired
            && self.storage_paths_wired
            && self.display_paths_wired
            && self.input_paths_wired
            && self.reader_file_browser_ux_unchanged
            && self.app_navigation_unchanged
            && self.display_draw_algorithm_unchanged
            && self.input_debounce_navigation_unchanged
            && self.no_fat_destructive_behavior
    }
}

impl VaachakHardwareRuntimeExecutorWiring {
    pub const HARDWARE_RUNTIME_EXECUTOR_WIRING_MARKER: &'static str =
        "hardware_runtime_executor_wiring=ok";
    pub const HARDWARE_RUNTIME_EXECUTOR_WIRING_IDENTITY: &'static str =
        "xteink-x4-vaachak-hardware-runtime-executor-wiring";
    pub const HARDWARE_RUNTIME_EXECUTOR_WIRING_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";

    pub const WIRING_ENTRYPOINT_ACTIVE: bool = true;
    pub const SELECTED_RUNTIME_PATH_COUNT: usize = 10;

    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_BEHAVIOR_CHANGED: bool = false;
    pub const DISPLAY_DRAW_ALGORITHM_REWRITTEN: bool = false;
    pub const INPUT_DEBOUNCE_NAVIGATION_REWRITTEN: bool = false;
    pub const FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED: bool = false;

    pub const fn domain_for_path(
        path: VaachakHardwareWiredRuntimePath,
    ) -> VaachakHardwareExecutorDomain {
        match path {
            VaachakHardwareWiredRuntimePath::BootStorageAvailability => {
                VaachakHardwareExecutorDomain::StorageProbeMount
            }
            VaachakHardwareWiredRuntimePath::LibraryDirectoryListing
            | VaachakHardwareWiredRuntimePath::ReaderFileOpenIntent
            | VaachakHardwareWiredRuntimePath::ReaderFileChunkIntent => {
                VaachakHardwareExecutorDomain::FatStorage
            }
            VaachakHardwareWiredRuntimePath::DisplayFullRefreshHandoff
            | VaachakHardwareWiredRuntimePath::DisplayPartialRefreshHandoff => {
                VaachakHardwareExecutorDomain::Display
            }
            VaachakHardwareWiredRuntimePath::InputButtonScanHandoff
            | VaachakHardwareWiredRuntimePath::InputNavigationHandoff => {
                VaachakHardwareExecutorDomain::Input
            }
            VaachakHardwareWiredRuntimePath::SharedSpiDisplayHandoff
            | VaachakHardwareWiredRuntimePath::SharedSpiStorageHandoff => {
                VaachakHardwareExecutorDomain::SpiBus
            }
        }
    }

    pub const fn selected_paths()
    -> [VaachakHardwareWiredRuntimePath; Self::SELECTED_RUNTIME_PATH_COUNT] {
        [
            VaachakHardwareWiredRuntimePath::BootStorageAvailability,
            VaachakHardwareWiredRuntimePath::LibraryDirectoryListing,
            VaachakHardwareWiredRuntimePath::ReaderFileOpenIntent,
            VaachakHardwareWiredRuntimePath::ReaderFileChunkIntent,
            VaachakHardwareWiredRuntimePath::DisplayFullRefreshHandoff,
            VaachakHardwareWiredRuntimePath::DisplayPartialRefreshHandoff,
            VaachakHardwareWiredRuntimePath::InputButtonScanHandoff,
            VaachakHardwareWiredRuntimePath::InputNavigationHandoff,
            VaachakHardwareWiredRuntimePath::SharedSpiDisplayHandoff,
            VaachakHardwareWiredRuntimePath::SharedSpiStorageHandoff,
        ]
    }

    pub const fn domain_bridge_ready(path: VaachakHardwareWiredRuntimePath) -> bool {
        match path {
            VaachakHardwareWiredRuntimePath::BootStorageAvailability => {
                VaachakStorageExecutorBridge::route_is_safe(
                    VaachakStorageExecutorBridge::route_intent(
                        VaachakStorageExecutorIntent::StorageAvailableState,
                    ),
                )
            }
            VaachakHardwareWiredRuntimePath::LibraryDirectoryListing => {
                VaachakStorageExecutorBridge::route_is_safe(
                    VaachakStorageExecutorBridge::route_intent(
                        VaachakStorageExecutorIntent::DirectoryListingIntent,
                    ),
                )
            }
            VaachakHardwareWiredRuntimePath::ReaderFileOpenIntent => {
                VaachakStorageExecutorBridge::route_is_safe(
                    VaachakStorageExecutorBridge::route_intent(
                        VaachakStorageExecutorIntent::FileOpenReadIntent,
                    ),
                )
            }
            VaachakHardwareWiredRuntimePath::ReaderFileChunkIntent => {
                VaachakStorageExecutorBridge::route_is_safe(
                    VaachakStorageExecutorBridge::route_intent(
                        VaachakStorageExecutorIntent::FileReadChunkIntent,
                    ),
                )
            }
            VaachakHardwareWiredRuntimePath::DisplayFullRefreshHandoff => {
                VaachakDisplayExecutorBridge::route_is_safe(
                    VaachakDisplayExecutorBridge::route_intent(
                        VaachakDisplayExecutorIntent::FullRefresh,
                    ),
                )
            }
            VaachakHardwareWiredRuntimePath::DisplayPartialRefreshHandoff => {
                VaachakDisplayExecutorBridge::route_is_safe(
                    VaachakDisplayExecutorBridge::route_intent(
                        VaachakDisplayExecutorIntent::PartialRefresh,
                    ),
                )
            }
            VaachakHardwareWiredRuntimePath::InputButtonScanHandoff => {
                VaachakInputExecutorBridge::route_is_safe(VaachakInputExecutorBridge::route_intent(
                    VaachakInputExecutorIntent::ButtonScan,
                ))
            }
            VaachakHardwareWiredRuntimePath::InputNavigationHandoff => {
                VaachakInputExecutorBridge::route_is_safe(VaachakInputExecutorBridge::route_intent(
                    VaachakInputExecutorIntent::NavigationHandoff,
                ))
            }
            VaachakHardwareWiredRuntimePath::SharedSpiDisplayHandoff => {
                VaachakSpiExecutorBridge::route_is_safe(
                    VaachakSpiExecutorBridge::route_transaction_intent(
                        VaachakSpiExecutorIntent::DisplayTransaction,
                        VaachakSpiRuntimeUser::Display,
                        VaachakSpiTransactionKind::DisplayRefreshMetadata,
                    ),
                )
            }
            VaachakHardwareWiredRuntimePath::SharedSpiStorageHandoff => {
                VaachakSpiExecutorBridge::route_is_safe(
                    VaachakSpiExecutorBridge::route_transaction_intent(
                        VaachakSpiExecutorIntent::StorageTransaction,
                        VaachakSpiRuntimeUser::Storage,
                        VaachakSpiTransactionKind::StorageFatIoMetadata,
                    ),
                )
            }
        }
    }

    pub const fn route_path(
        path: VaachakHardwareWiredRuntimePath,
    ) -> VaachakHardwareRuntimeWiringRoute {
        let domain = Self::domain_for_path(path);
        let executor_entry = VaachakHardwareRuntimeExecutor::entry_for(domain);
        let backend_route = VaachakHardwareRuntimeWiringPulpBackend::route_for(domain);
        let consolidated_ready = VaachakHardwareRuntimeExecutor::entry_is_safe(executor_entry)
            && VaachakHardwareRuntimeExecutor::extraction_ok();
        let domain_bridge_ready = Self::domain_bridge_ready(path);
        let backend_ready = VaachakHardwareRuntimeWiringPulpBackend::route_ok(backend_route);
        let decision = if consolidated_ready && domain_bridge_ready && backend_ready {
            VaachakHardwareRuntimeWiringDecision::RoutedThroughVaachakHardwareRuntimeExecutor
        } else {
            VaachakHardwareRuntimeWiringDecision::RejectedBeforePulpCompatibleBackend
        };

        VaachakHardwareRuntimeWiringRoute {
            path,
            domain,
            decision,
            executor_entry,
            backend_route,
            consolidated_executor_ready: consolidated_ready,
            domain_bridge_ready,
            low_level_backend_still_pulp_compatible: backend_ready,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_behavior_changed: Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            display_draw_algorithm_rewritten: Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN,
            input_debounce_navigation_rewritten: Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN,
            fat_destructive_behavior_introduced: Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED,
        }
    }

    pub const fn route_is_safe(route: VaachakHardwareRuntimeWiringRoute) -> bool {
        matches!(
            route.decision,
            VaachakHardwareRuntimeWiringDecision::RoutedThroughVaachakHardwareRuntimeExecutor
        ) && matches!(
            route.executor_entry.state,
            VaachakHardwareRuntimeExecutorState::VaachakEntrypointActive
        ) && matches!(
            route.executor_entry.backend,
            VaachakHardwareExecutorBackend::PulpCompatibility
        ) && route.executor_entry.backend_name.len()
            == VaachakHardwareExecutorPulpBackend::BACKEND_NAME.len()
            && route.executor_entry.active_executor_owner.len()
                == VaachakHardwareExecutorPulpBackend::ACTIVE_EXECUTOR_OWNER.len()
            && VaachakHardwareRuntimeWiringPulpBackend::route_ok(route.backend_route)
            && route.consolidated_executor_ready
            && route.domain_bridge_ready
            && route.low_level_backend_still_pulp_compatible
            && !route.reader_file_browser_ux_changed
            && !route.app_navigation_behavior_changed
            && !route.display_draw_algorithm_rewritten
            && !route.input_debounce_navigation_rewritten
            && !route.fat_destructive_behavior_introduced
    }

    pub const fn storage_paths_wired() -> bool {
        Self::route_is_safe(Self::route_path(
            VaachakHardwareWiredRuntimePath::BootStorageAvailability,
        )) && Self::route_is_safe(Self::route_path(
            VaachakHardwareWiredRuntimePath::LibraryDirectoryListing,
        )) && Self::route_is_safe(Self::route_path(
            VaachakHardwareWiredRuntimePath::ReaderFileOpenIntent,
        )) && Self::route_is_safe(Self::route_path(
            VaachakHardwareWiredRuntimePath::ReaderFileChunkIntent,
        ))
    }

    pub const fn display_paths_wired() -> bool {
        Self::route_is_safe(Self::route_path(
            VaachakHardwareWiredRuntimePath::DisplayFullRefreshHandoff,
        )) && Self::route_is_safe(Self::route_path(
            VaachakHardwareWiredRuntimePath::DisplayPartialRefreshHandoff,
        ))
    }

    pub const fn input_paths_wired() -> bool {
        Self::route_is_safe(Self::route_path(
            VaachakHardwareWiredRuntimePath::InputButtonScanHandoff,
        )) && Self::route_is_safe(Self::route_path(
            VaachakHardwareWiredRuntimePath::InputNavigationHandoff,
        ))
    }

    pub const fn spi_paths_wired() -> bool {
        Self::route_is_safe(Self::route_path(
            VaachakHardwareWiredRuntimePath::SharedSpiDisplayHandoff,
        )) && Self::route_is_safe(Self::route_path(
            VaachakHardwareWiredRuntimePath::SharedSpiStorageHandoff,
        ))
    }

    pub const fn report() -> VaachakHardwareRuntimeExecutorWiringReport {
        VaachakHardwareRuntimeExecutorWiringReport {
            wiring_entrypoint_active: Self::WIRING_ENTRYPOINT_ACTIVE,
            consolidated_executor_ready: VaachakHardwareRuntimeExecutor::extraction_ok(),
            pulp_backend_active: VaachakHardwareRuntimeWiringPulpBackend::backend_ok(),
            spi_paths_wired: Self::spi_paths_wired(),
            storage_paths_wired: Self::storage_paths_wired(),
            display_paths_wired: Self::display_paths_wired(),
            input_paths_wired: Self::input_paths_wired(),
            reader_file_browser_ux_unchanged: !Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_unchanged: !Self::APP_NAVIGATION_BEHAVIOR_CHANGED,
            display_draw_algorithm_unchanged: !Self::DISPLAY_DRAW_ALGORITHM_REWRITTEN,
            input_debounce_navigation_unchanged: !Self::INPUT_DEBOUNCE_NAVIGATION_REWRITTEN,
            no_fat_destructive_behavior: !Self::FAT_DESTRUCTIVE_BEHAVIOR_INTRODUCED,
        }
    }

    pub const fn wiring_ok() -> bool {
        Self::report().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorWiring;

    #[test]
    fn hardware_runtime_executor_wiring_is_active() {
        assert!(VaachakHardwareRuntimeExecutorWiring::wiring_ok());
    }

    #[test]
    fn all_selected_runtime_paths_are_safe() {
        let paths = VaachakHardwareRuntimeExecutorWiring::selected_paths();
        assert_eq!(
            paths.len(),
            VaachakHardwareRuntimeExecutorWiring::SELECTED_RUNTIME_PATH_COUNT
        );
        for path in paths {
            assert!(VaachakHardwareRuntimeExecutorWiring::route_is_safe(
                VaachakHardwareRuntimeExecutorWiring::route_path(path)
            ));
        }
    }
}
