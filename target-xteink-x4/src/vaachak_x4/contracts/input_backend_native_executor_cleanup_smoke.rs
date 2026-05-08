#![allow(dead_code)]

use crate::vaachak_x4::physical::input_backend_native_executor_cleanup::VaachakInputBackendNativeExecutorCleanup;

pub struct VaachakInputBackendNativeExecutorCleanupSmoke;

impl VaachakInputBackendNativeExecutorCleanupSmoke {
    pub const MARKER: &'static str =
        VaachakInputBackendNativeExecutorCleanup::INPUT_BACKEND_NATIVE_EXECUTOR_CLEANUP_MARKER;

    pub fn smoke_ok() -> bool {
        VaachakInputBackendNativeExecutorCleanup::cleanup_ok()
            && Self::MARKER == "input_backend_native_executor_cleanup=ok"
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakInputBackendNativeExecutorCleanupSmoke;

    #[test]
    fn input_backend_native_executor_cleanup_smoke_passes() {
        assert!(VaachakInputBackendNativeExecutorCleanupSmoke::smoke_ok());
    }
}
