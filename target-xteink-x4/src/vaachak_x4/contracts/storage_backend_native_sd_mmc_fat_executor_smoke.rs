use crate::vaachak_x4::physical::storage_backend_native_sd_mmc_fat_executor::VaachakStorageBackendNativeSdMmcFatExecutor;

pub fn storage_backend_native_sd_mmc_fat_executor_smoke_ok() -> bool {
    VaachakStorageBackendNativeSdMmcFatExecutor::native_sd_mmc_fat_executor_ok()
        && VaachakStorageBackendNativeSdMmcFatExecutor::adopt_storage_availability_handoff()
        && VaachakStorageBackendNativeSdMmcFatExecutor::adopt_storage_fat_access_handoff()
        && VaachakStorageBackendNativeSdMmcFatExecutor::execute_destructive_operation_denial().ok()
}

#[cfg(test)]
mod tests {
    use super::storage_backend_native_sd_mmc_fat_executor_smoke_ok;

    #[test]
    fn storage_backend_native_sd_mmc_fat_executor_contract_is_ready() {
        assert!(storage_backend_native_sd_mmc_fat_executor_smoke_ok());
    }
}
