use crate::vaachak_x4::physical::storage_backend_native_sd_mmc_fat_executor::VaachakStorageBackendNativeSdMmcFatExecutor;
use crate::vaachak_x4::physical::storage_backend_native_sd_mmc_fat_executor_cleanup::VaachakStorageBackendNativeSdMmcFatExecutorCleanup;

pub fn storage_backend_native_sd_mmc_fat_executor_cleanup_smoke_ok() -> bool {
    VaachakStorageBackendNativeSdMmcFatExecutor::native_sd_mmc_fat_executor_ok()
        && VaachakStorageBackendNativeSdMmcFatExecutorCleanup::cleanup_ok()
        && VaachakStorageBackendNativeSdMmcFatExecutorCleanup::report().ok()
}

#[cfg(test)]
mod tests {
    use super::storage_backend_native_sd_mmc_fat_executor_cleanup_smoke_ok;

    #[test]
    fn storage_backend_native_sd_mmc_fat_executor_cleanup_contract_is_ready() {
        assert!(storage_backend_native_sd_mmc_fat_executor_cleanup_smoke_ok());
    }
}
