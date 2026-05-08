#![allow(dead_code)]

use crate::vaachak_x4::physical::input_pulp_backend::VaachakInputPulpBackend;
use crate::vaachak_x4::physical::input_runtime_owner::{
    VaachakInputRuntimeOperation, VaachakInputRuntimeOwner,
};

/// Contract smoke for the Vaachak-owned input runtime ownership boundary.
///
/// This smoke validates ownership metadata and the Pulp compatibility backend.
/// It does not sample buttons, debounce inputs, dispatch navigation events, or
/// change display/storage/reader/file-browser behavior.
pub struct VaachakInputRuntimeOwnershipSmoke;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakInputRuntimeOwnershipSmokeReport {
    pub ownership_owner_ok: bool,
    pub backend_bridge_ok: bool,
    pub gpio_metadata_ok: bool,
    pub ladder_metadata_ok: bool,
    pub timing_metadata_ok: bool,
    pub shell_input_boundary_ok: bool,
    pub adc_metadata_is_safe: bool,
    pub semantic_metadata_is_safe: bool,
    pub input_behavior_moved: bool,
    pub display_behavior_changed: bool,
    pub storage_behavior_changed: bool,
    pub reader_file_browser_behavior_changed: bool,
}

impl VaachakInputRuntimeOwnershipSmokeReport {
    pub const fn smoke_ok(self) -> bool {
        self.ownership_owner_ok
            && self.backend_bridge_ok
            && self.gpio_metadata_ok
            && self.ladder_metadata_ok
            && self.timing_metadata_ok
            && self.shell_input_boundary_ok
            && self.adc_metadata_is_safe
            && self.semantic_metadata_is_safe
            && !self.input_behavior_moved
            && !self.display_behavior_changed
            && !self.storage_behavior_changed
            && !self.reader_file_browser_behavior_changed
    }
}

impl VaachakInputRuntimeOwnershipSmoke {
    pub const INPUT_RUNTIME_OWNERSHIP_SMOKE_MARKER: &'static str =
        "x4-input-runtime-ownership-smoke-ok";

    pub const INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK: bool = true;
    pub const PULP_COMPATIBILITY_BACKEND_ACTIVE: bool = true;
    pub const ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK: bool = false;
    pub const NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK: bool = false;
    pub const DISPLAY_BEHAVIOR_CHANGED: bool = false;
    pub const STORAGE_BEHAVIOR_CHANGED: bool = false;
    pub const READER_FILE_BROWSER_BEHAVIOR_CHANGED: bool = false;

    pub const fn adc_metadata_is_safe() -> bool {
        VaachakInputRuntimeOwner::operation_metadata_is_safe(
            VaachakInputRuntimeOwner::operation_ownership(
                VaachakInputRuntimeOperation::AdcLadderMetadata,
            ),
        )
    }

    pub const fn semantic_metadata_is_safe() -> bool {
        VaachakInputRuntimeOwner::operation_metadata_is_safe(
            VaachakInputRuntimeOwner::operation_ownership(
                VaachakInputRuntimeOperation::SemanticMappingMetadata,
            ),
        )
    }

    pub const fn input_behavior_moved() -> bool {
        VaachakInputRuntimeOwner::ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK
            || VaachakInputRuntimeOwner::BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK
            || VaachakInputRuntimeOwner::DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK
            || VaachakInputRuntimeOwner::NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK
    }

    pub const fn report() -> VaachakInputRuntimeOwnershipSmokeReport {
        VaachakInputRuntimeOwnershipSmokeReport {
            ownership_owner_ok: VaachakInputRuntimeOwner::ownership_ok(),
            backend_bridge_ok: VaachakInputPulpBackend::bridge_ok(),
            gpio_metadata_ok: VaachakInputRuntimeOwner::pin_map_ok(),
            ladder_metadata_ok: VaachakInputRuntimeOwner::ladder_map_ok(),
            timing_metadata_ok: VaachakInputRuntimeOwner::timing_map_ok(),
            shell_input_boundary_ok: VaachakInputRuntimeOwner::current_input_boundary_documented(),
            adc_metadata_is_safe: Self::adc_metadata_is_safe(),
            semantic_metadata_is_safe: Self::semantic_metadata_is_safe(),
            input_behavior_moved: Self::input_behavior_moved(),
            display_behavior_changed: Self::DISPLAY_BEHAVIOR_CHANGED,
            storage_behavior_changed: Self::STORAGE_BEHAVIOR_CHANGED,
            reader_file_browser_behavior_changed: Self::READER_FILE_BROWSER_BEHAVIOR_CHANGED,
        }
    }

    pub const fn smoke_validate() -> bool {
        Self::INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK
            && Self::PULP_COMPATIBILITY_BACKEND_ACTIVE
            && !Self::ADC_SAMPLING_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::BUTTON_SCAN_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::DEBOUNCE_REPEAT_EXECUTOR_MOVED_TO_VAACHAK
            && !Self::NAVIGATION_EVENT_ROUTING_MOVED_TO_VAACHAK
            && Self::report().smoke_ok()
    }
}
