#![allow(dead_code)]

use crate::vaachak_x4::physical::display_backend_native_refresh_shell_cleanup::VaachakDisplayBackendNativeRefreshShellCleanup;

pub struct VaachakDisplayBackendNativeRefreshShellCleanupSmoke;

impl VaachakDisplayBackendNativeRefreshShellCleanupSmoke {
    pub const MARKER: &'static str = VaachakDisplayBackendNativeRefreshShellCleanup::DISPLAY_BACKEND_NATIVE_REFRESH_SHELL_CLEANUP_MARKER;

    pub fn smoke_ok() -> bool {
        VaachakDisplayBackendNativeRefreshShellCleanup::cleanup_ok()
            && Self::MARKER == "display_backend_native_refresh_shell_cleanup=ok"
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakDisplayBackendNativeRefreshShellCleanupSmoke;

    #[test]
    fn display_backend_native_refresh_shell_cleanup_smoke_passes() {
        assert!(VaachakDisplayBackendNativeRefreshShellCleanupSmoke::smoke_ok());
    }
}
