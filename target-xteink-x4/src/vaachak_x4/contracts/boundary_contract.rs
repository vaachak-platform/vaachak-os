#![allow(dead_code)]

/// Phase 24 consolidated Vaachak boundary contract.
///
/// This module is Vaachak-owned. It consolidates the display/input/storage
/// metadata boundaries introduced in Phases 20-23 without moving physical
/// hardware behavior out of the imported X4/Pulp runtime yet.
#[cfg(target_arch = "riscv32")]
pub struct VaachakBoundaryContract;

#[cfg(target_arch = "riscv32")]
impl VaachakBoundaryContract {
    /// Phase 24 acceptance marker.
    pub const PHASE24_MARKER: &'static str = "phase24=x4-boundary-contract-ok";

    /// Current ownership model.
    pub const METADATA_OWNER: &'static str = "Vaachak runtime boundary contract";
    pub const PHYSICAL_BEHAVIOR_OWNER: &'static str = "vendor/pulp-os imported runtime";

    /// Phase 24 does not move physical hardware behavior.
    pub const DISPLAY_BEHAVIOR_MOVED_IN_PHASE24: bool = false;
    pub const INPUT_BEHAVIOR_MOVED_IN_PHASE24: bool = false;
    pub const STORAGE_BEHAVIOR_MOVED_IN_PHASE24: bool = false;

    /// Emit only the Phase 24 contract marker.
    pub fn emit_contract_marker() {
        esp_println::println!("{}", Self::PHASE24_MARKER);
    }

    /// Emit the consolidated boundary marker set.
    ///
    /// Compatibility rule:
    /// - Keep Phase 20 scaffold marker.
    /// - Keep Phase 21 storage boundary marker.
    /// - Keep Phase 22 input boundary marker.
    /// - Keep Phase 23 display boundary marker.
    /// - Add Phase 24 consolidated contract marker.
    pub fn emit_all_boundary_markers() {
        Self::emit_contract_marker();
        crate::vaachak_x4::contracts::display::VaachakDisplayBoundary::emit_phase23_marker();
        crate::vaachak_x4::contracts::storage::VaachakStorageBoundary::emit_boot_marker();
        crate::vaachak_x4::contracts::input::VaachakInputBoundary::emit_boot_marker();
        crate::vaachak_x4::contracts::display::VaachakDisplayBoundary::emit_scaffold_marker();
    }

    /// Human-readable current ownership summary.
    pub fn ownership_summary() -> &'static str {
        "Vaachak owns boundary metadata/contracts; imported X4/Pulp runtime still owns display/input/storage physical behavior"
    }

    /// Physical behavior remains imported until explicit later extraction phases.
    pub fn physical_behavior_is_still_imported() -> bool {
        !Self::DISPLAY_BEHAVIOR_MOVED_IN_PHASE24
            && !Self::INPUT_BEHAVIOR_MOVED_IN_PHASE24
            && !Self::STORAGE_BEHAVIOR_MOVED_IN_PHASE24
    }
}
