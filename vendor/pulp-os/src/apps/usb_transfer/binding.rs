//! X4 USB Serial SD Bulk Transfer byte-stream binding scaffold.
//!
//! This module is intentionally compiled but non-owning. The final board step
//! should connect ESP32-C3 USB_SERIAL_JTAG bytes to this binding where the HAL
//! peripheral ownership is available.
//!
//! Do not directly steal/take USB_SERIAL_JTAG here: it may be used by logging
//! or the espflash monitor path.

use super::runtime::{UsbTransferProgress, UsbTransferRuntime, UsbTransferRuntimeStatus};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbTransferBindingState {
    Disabled,
    WaitingForHost,
    Receiving,
    Complete,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbTransferBindingStatus {
    pub state: UsbTransferBindingState,
    pub progress: UsbTransferProgress,
}

impl UsbTransferBindingStatus {
    pub const fn disabled() -> Self {
        Self {
            state: UsbTransferBindingState::Disabled,
            progress: UsbTransferProgress::empty(),
        }
    }

    pub const fn waiting() -> Self {
        Self {
            state: UsbTransferBindingState::WaitingForHost,
            progress: UsbTransferProgress::empty(),
        }
    }
}

/// Runtime owner for the future USB byte stream.
///
/// The runtime object is present now so the UI/app path can be wired safely.
/// The next board-specific step should feed complete raw frames to
/// `runtime.accept_raw_frame(raw, sd_target)` and send OK/ERR back to host.
#[derive(Debug)]
pub struct UsbTransferBinding {
    runtime: UsbTransferRuntime,
    enabled: bool,
}

impl UsbTransferBinding {
    pub const fn new() -> Self {
        Self {
            runtime: UsbTransferRuntime::new(),
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.runtime.reset();
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.runtime.reset();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn status(&self) -> UsbTransferBindingStatus {
        if !self.enabled {
            return UsbTransferBindingStatus::disabled();
        }

        let progress = self.runtime.progress();
        let state = match progress.status {
            UsbTransferRuntimeStatus::Idle | UsbTransferRuntimeStatus::Ready => {
                UsbTransferBindingState::WaitingForHost
            }
            UsbTransferRuntimeStatus::Receiving => UsbTransferBindingState::Receiving,
            UsbTransferRuntimeStatus::Complete => UsbTransferBindingState::Complete,
            UsbTransferRuntimeStatus::Failed => UsbTransferBindingState::Failed,
        };

        UsbTransferBindingStatus { state, progress }
    }

    pub fn runtime_mut(&mut self) -> &mut UsbTransferRuntime {
        &mut self.runtime
    }
}

/// Board binding placeholder.
///
/// This must be called from the future Tools > USB Transfer app tick once the
/// ESP32-C3 USB_SERIAL_JTAG byte stream is safely available.
pub fn poll_usb_transfer_binding(binding: &mut UsbTransferBinding) -> UsbTransferBindingStatus {
    if !binding.is_enabled() {
        binding.enable();
    }

    binding.status()
}
