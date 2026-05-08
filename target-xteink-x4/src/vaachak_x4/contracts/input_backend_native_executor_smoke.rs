#![allow(dead_code)]

use crate::vaachak_x4::physical::hardware_runtime_backend_takeover::VaachakHardwareRuntimeBackendTakeover;
use crate::vaachak_x4::physical::input_backend_native_executor::VaachakInputBackendNativeExecutor;

/// Smoke contract for the Vaachak-native input backend executor slice.
pub struct VaachakInputBackendNativeExecutorSmoke;

impl VaachakInputBackendNativeExecutorSmoke {
    pub const INPUT_BACKEND_NATIVE_EXECUTOR_SMOKE_MARKER: &'static str =
        "input_backend_native_executor=ok";

    pub fn smoke_ok() -> bool {
        VaachakInputBackendNativeExecutor::native_executor_ok()
            && VaachakHardwareRuntimeBackendTakeover::takeover_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakInputBackendNativeExecutorSmoke;

    #[test]
    fn input_backend_native_executor_smoke_is_ready() {
        assert!(VaachakInputBackendNativeExecutorSmoke::smoke_ok());
    }
}
