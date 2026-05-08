#![allow(dead_code)]

use crate::vaachak_x4::physical::display_backend_native_refresh_command_executor_cleanup::VaachakDisplayBackendNativeRefreshCommandExecutorCleanup;

pub fn display_backend_native_refresh_command_executor_cleanup_smoke_ok() -> bool {
    VaachakDisplayBackendNativeRefreshCommandExecutorCleanup::cleanup_ok()
}

#[cfg(test)]
mod tests {
    use super::display_backend_native_refresh_command_executor_cleanup_smoke_ok;

    #[test]
    fn display_backend_native_refresh_command_executor_cleanup_contract_is_ready() {
        assert!(display_backend_native_refresh_command_executor_cleanup_smoke_ok());
    }
}
