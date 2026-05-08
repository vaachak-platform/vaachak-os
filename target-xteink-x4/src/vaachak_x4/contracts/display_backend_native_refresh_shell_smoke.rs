#![allow(dead_code)]

use crate::vaachak_x4::physical::display_backend_native_refresh_shell::VaachakDisplayBackendNativeRefreshShell;
use crate::vaachak_x4::physical::hardware_runtime_backend_takeover::VaachakHardwareRuntimeBackendTakeover;

/// Smoke contract for the Vaachak-native display refresh shell slice.
pub struct VaachakDisplayBackendNativeRefreshShellSmoke;

impl VaachakDisplayBackendNativeRefreshShellSmoke {
    pub const DISPLAY_BACKEND_NATIVE_REFRESH_SHELL_SMOKE_MARKER: &'static str =
        "display_backend_native_refresh_shell=ok";

    pub fn smoke_ok() -> bool {
        VaachakDisplayBackendNativeRefreshShell::native_refresh_shell_ok()
            && VaachakHardwareRuntimeBackendTakeover::takeover_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakDisplayBackendNativeRefreshShellSmoke;

    #[test]
    fn display_backend_native_refresh_shell_smoke_is_ready() {
        assert!(VaachakDisplayBackendNativeRefreshShellSmoke::smoke_ok());
    }
}
