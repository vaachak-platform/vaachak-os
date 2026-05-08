#![allow(dead_code)]

use crate::vaachak_x4::physical::input_backend_native_event_pipeline_cleanup::VaachakInputBackendNativeEventPipelineCleanup;

pub struct VaachakInputBackendNativeEventPipelineCleanupSmoke;

impl VaachakInputBackendNativeEventPipelineCleanupSmoke {
    pub const MARKER: &'static str =
        VaachakInputBackendNativeEventPipelineCleanup::INPUT_BACKEND_NATIVE_EVENT_PIPELINE_CLEANUP_MARKER;

    pub fn smoke_ok() -> bool {
        VaachakInputBackendNativeEventPipelineCleanup::cleanup_ok()
            && Self::MARKER == "input_backend_native_event_pipeline_cleanup=ok"
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakInputBackendNativeEventPipelineCleanupSmoke;

    #[test]
    fn input_backend_native_event_pipeline_cleanup_smoke_passes() {
        assert!(VaachakInputBackendNativeEventPipelineCleanupSmoke::smoke_ok());
    }
}
