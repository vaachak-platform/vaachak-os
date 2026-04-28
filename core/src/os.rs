use crate::hal::Hal;
use crate::services::{PowerManager, StorageService};
use crate::ui::activity::ActivityManager;

/// High-level OS shell placeholder.
/// Real runtime orchestration will be introduced after HAL seams are refined.
pub struct VaachakOs<H: Hal> {
    pub hal: H,
    pub activity_manager: ActivityManager,
    pub storage_service: StorageService,
    pub power_manager: PowerManager,
}

impl<H: Hal> VaachakOs<H> {
    pub fn new(hal: H) -> Self {
        Self {
            hal,
            activity_manager: ActivityManager::new(),
            storage_service: StorageService::new(),
            power_manager: PowerManager::new(),
        }
    }

    pub fn boot(&mut self) {
        // TODO: bind boot flow to target-xteink-x4 runtime once extraction starts.
    }
}
