#![allow(dead_code)]

use super::hardware_runtime_executor_observability::{
    VaachakHardwareRuntimeExecutorObservability, VaachakHardwareRuntimeObservationRecord,
};

/// Vaachak-owned boot/debug marker surface for the consolidated hardware
/// runtime executor path.
///
/// This layer is the runtime evidence hook for the accepted executor,
/// wiring, and observability slices. It only emits marker text to the boot
/// debug stream and intentionally avoids display rendering, input scanning,
/// SPI transfer, SD/MMC, FAT, reader, file-browser, or app-navigation behavior.
pub struct VaachakHardwareRuntimeExecutorBootMarkers;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VaachakHardwareRuntimeExecutorBootMarkerReport {
    pub boot_marker_entrypoint_active: bool,
    pub observability_layer_ready: bool,
    pub boot_marker_count_ok: bool,
    pub boot_markers_selected: bool,
    pub debug_markers_selected: bool,
    pub emits_to_boot_debug_stream: bool,
    pub writes_to_display: bool,
    pub touches_spi_transfer: bool,
    pub touches_storage_execution: bool,
    pub touches_display_rendering: bool,
    pub touches_input_execution: bool,
    pub reader_file_browser_ux_changed: bool,
    pub app_navigation_changed: bool,
}

impl VaachakHardwareRuntimeExecutorBootMarkerReport {
    pub const fn ok(self) -> bool {
        self.boot_marker_entrypoint_active
            && self.observability_layer_ready
            && self.boot_marker_count_ok
            && self.boot_markers_selected
            && self.debug_markers_selected
            && self.emits_to_boot_debug_stream
            && !self.writes_to_display
            && !self.touches_spi_transfer
            && !self.touches_storage_execution
            && !self.touches_display_rendering
            && !self.touches_input_execution
            && !self.reader_file_browser_ux_changed
            && !self.app_navigation_changed
    }
}

impl VaachakHardwareRuntimeExecutorBootMarkers {
    pub const HARDWARE_RUNTIME_EXECUTOR_BOOT_MARKERS_MARKER: &'static str =
        "hardware_runtime_executor_boot_markers=ok";
    pub const HARDWARE_RUNTIME_EXECUTOR_BOOT_MARKERS_IDENTITY: &'static str =
        "xteink-x4-vaachak-hardware-runtime-executor-boot-markers";
    pub const HARDWARE_RUNTIME_EXECUTOR_BOOT_MARKERS_OWNER: &'static str =
        "target-xteink-x4 Vaachak layer";

    pub const BOOT_MARKER_ENTRYPOINT_ACTIVE: bool = true;
    pub const BOOT_MARKER_COUNT: usize =
        VaachakHardwareRuntimeExecutorObservability::BOOT_MARKER_COUNT;
    pub const BOOT_MARKERS_ROUTE_THROUGH_OBSERVABILITY: bool = true;
    pub const BOOT_MARKERS_EMIT_TO_DEBUG_STREAM: bool = true;
    pub const BOOT_MARKERS_WRITE_TO_DISPLAY: bool = false;
    pub const BOOT_MARKERS_TOUCH_SPI_TRANSFER: bool = false;
    pub const BOOT_MARKERS_TOUCH_STORAGE_EXECUTION: bool = false;
    pub const BOOT_MARKERS_TOUCH_DISPLAY_RENDERING: bool = false;
    pub const BOOT_MARKERS_TOUCH_INPUT_EXECUTION: bool = false;
    pub const READER_FILE_BROWSER_UX_CHANGED: bool = false;
    pub const APP_NAVIGATION_CHANGED: bool = false;

    pub const fn boot_marker_record_count_ok() -> bool {
        Self::BOOT_MARKER_COUNT == 8
    }

    pub const fn record_visible_at_boot(record: VaachakHardwareRuntimeObservationRecord) -> bool {
        VaachakHardwareRuntimeExecutorObservability::record_ok(record)
            && record.marker_text.len() > 0
            && Self::BOOT_MARKERS_ROUTE_THROUGH_OBSERVABILITY
            && Self::BOOT_MARKERS_EMIT_TO_DEBUG_STREAM
            && !Self::BOOT_MARKERS_WRITE_TO_DISPLAY
    }

    pub const fn boot_marker_set_ready() -> bool {
        let markers = VaachakHardwareRuntimeExecutorObservability::boot_markers();
        Self::boot_marker_record_count_ok()
            && Self::record_visible_at_boot(markers[0])
            && Self::record_visible_at_boot(markers[1])
            && Self::record_visible_at_boot(markers[2])
            && Self::record_visible_at_boot(markers[3])
            && Self::record_visible_at_boot(markers[4])
            && Self::record_visible_at_boot(markers[5])
            && Self::record_visible_at_boot(markers[6])
            && Self::record_visible_at_boot(markers[7])
    }

    pub const fn behavior_preserved() -> bool {
        Self::BOOT_MARKERS_EMIT_TO_DEBUG_STREAM
            && !Self::BOOT_MARKERS_WRITE_TO_DISPLAY
            && !Self::BOOT_MARKERS_TOUCH_SPI_TRANSFER
            && !Self::BOOT_MARKERS_TOUCH_STORAGE_EXECUTION
            && !Self::BOOT_MARKERS_TOUCH_DISPLAY_RENDERING
            && !Self::BOOT_MARKERS_TOUCH_INPUT_EXECUTION
            && !Self::READER_FILE_BROWSER_UX_CHANGED
            && !Self::APP_NAVIGATION_CHANGED
            && VaachakHardwareRuntimeExecutorObservability::behavior_preserved()
    }

    pub const fn report() -> VaachakHardwareRuntimeExecutorBootMarkerReport {
        VaachakHardwareRuntimeExecutorBootMarkerReport {
            boot_marker_entrypoint_active: Self::BOOT_MARKER_ENTRYPOINT_ACTIVE,
            observability_layer_ready:
                VaachakHardwareRuntimeExecutorObservability::observability_ok(),
            boot_marker_count_ok: Self::boot_marker_record_count_ok(),
            boot_markers_selected: Self::boot_marker_set_ready(),
            debug_markers_selected:
                VaachakHardwareRuntimeExecutorObservability::all_wired_paths_observed(),
            emits_to_boot_debug_stream: Self::BOOT_MARKERS_EMIT_TO_DEBUG_STREAM,
            writes_to_display: Self::BOOT_MARKERS_WRITE_TO_DISPLAY,
            touches_spi_transfer: Self::BOOT_MARKERS_TOUCH_SPI_TRANSFER,
            touches_storage_execution: Self::BOOT_MARKERS_TOUCH_STORAGE_EXECUTION,
            touches_display_rendering: Self::BOOT_MARKERS_TOUCH_DISPLAY_RENDERING,
            touches_input_execution: Self::BOOT_MARKERS_TOUCH_INPUT_EXECUTION,
            reader_file_browser_ux_changed: Self::READER_FILE_BROWSER_UX_CHANGED,
            app_navigation_changed: Self::APP_NAVIGATION_CHANGED,
        }
    }

    pub const fn boot_markers_ok() -> bool {
        Self::report().ok() && Self::behavior_preserved()
    }

    /// Emit the executor boot marker set to the serial/debug boot stream.
    ///
    /// This is intentionally a debug-stream side effect only. It does not draw
    /// text to the e-paper display, touch storage, sample input, or mutate
    /// runtime hardware behavior.
    pub fn emit_boot_markers() {
        if !Self::boot_markers_ok() {
            Self::emit_line("hardware_runtime_executor_boot_markers=failed");
            return;
        }

        Self::emit_line(Self::HARDWARE_RUNTIME_EXECUTOR_BOOT_MARKERS_MARKER);
        Self::emit_line(
            VaachakHardwareRuntimeExecutorObservability::HARDWARE_RUNTIME_EXECUTOR_OBSERVABILITY_MARKER,
        );

        let markers = VaachakHardwareRuntimeExecutorObservability::boot_markers();
        for marker in markers {
            if Self::record_visible_at_boot(marker) {
                Self::emit_line(marker.marker_text);
            } else {
                Self::emit_line("hardware.executor.marker.failed");
            }
        }
    }

    #[cfg(all(target_arch = "riscv32", target_os = "none"))]
    fn emit_line(line: &str) {
        esp_println::println!("{}", line);
    }

    #[cfg(not(all(target_arch = "riscv32", target_os = "none")))]
    fn emit_line(line: &str) {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::VaachakHardwareRuntimeExecutorBootMarkers;

    #[test]
    fn hardware_runtime_executor_boot_markers_are_ready() {
        assert!(VaachakHardwareRuntimeExecutorBootMarkers::boot_markers_ok());
    }
}
