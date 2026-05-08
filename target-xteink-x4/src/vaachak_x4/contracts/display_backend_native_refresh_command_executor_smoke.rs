#![allow(dead_code)]

use crate::vaachak_x4::physical::display_backend_native_refresh_command_executor::VaachakDisplayBackendNativeRefreshCommandExecutor;

pub fn display_backend_native_refresh_command_executor_smoke_ok() -> bool {
    VaachakDisplayBackendNativeRefreshCommandExecutor::command_executor_ok()
}

#[cfg(test)]
mod tests {
    use super::display_backend_native_refresh_command_executor_smoke_ok;

    #[test]
    fn display_backend_native_refresh_command_executor_contract_is_ready() {
        assert!(display_backend_native_refresh_command_executor_smoke_ok());
    }
}
