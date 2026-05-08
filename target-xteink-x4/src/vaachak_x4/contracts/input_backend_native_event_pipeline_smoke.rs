#![allow(dead_code)]

use crate::vaachak_x4::physical::display_backend_native_refresh_shell_cleanup::VaachakDisplayBackendNativeRefreshShellCleanup;
use crate::vaachak_x4::physical::hardware_runtime_backend_takeover::VaachakHardwareRuntimeBackendTakeover;
use crate::vaachak_x4::physical::hardware_runtime_backend_takeover_cleanup::VaachakHardwareRuntimeBackendTakeoverCleanup;
use crate::vaachak_x4::physical::input_backend_native_event_pipeline::VaachakInputBackendNativeEventPipeline;
use crate::vaachak_x4::physical::input_backend_native_executor::VaachakInputBackendNativeExecutor;
use crate::vaachak_x4::physical::input_backend_native_executor_cleanup::VaachakInputBackendNativeExecutorCleanup;

/// Contract smoke for the Vaachak-native input event pipeline behavior move.
pub struct VaachakInputBackendNativeEventPipelineSmoke;

impl VaachakInputBackendNativeEventPipelineSmoke {
    pub const INPUT_BACKEND_NATIVE_EVENT_PIPELINE_SMOKE_MARKER: &'static str =
        "input_backend_native_event_pipeline=ok";

    pub fn smoke_ok() -> bool {
        VaachakInputBackendNativeEventPipeline::event_pipeline_ok()
            && VaachakInputBackendNativeExecutor::native_executor_ok()
            && VaachakInputBackendNativeExecutorCleanup::input_native_executor_ok()
            && VaachakHardwareRuntimeBackendTakeover::takeover_ok()
            && VaachakHardwareRuntimeBackendTakeoverCleanup::backend_takeover_cleanup_ok()
            && VaachakDisplayBackendNativeRefreshShellCleanup::display_native_refresh_shell_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakInputBackendNativeEventPipelineSmoke;

    #[test]
    fn input_backend_native_event_pipeline_smoke_is_ready() {
        assert!(VaachakInputBackendNativeEventPipelineSmoke::smoke_ok());
    }
}
