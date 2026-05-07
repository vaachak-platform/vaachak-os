//! USB Transfer screen text helpers.
//!
//! Home currently renders this screen from home.rs to avoid introducing a new
//! app trait object during the scaffold step.

use super::binding::{UsbTransferBindingState, UsbTransferBindingStatus};

pub fn status_title(status: UsbTransferBindingStatus) -> &'static str {
    match status.state {
        UsbTransferBindingState::Disabled => "USB Transfer Disabled",
        UsbTransferBindingState::WaitingForHost => "Waiting for host",
        UsbTransferBindingState::Receiving => "Receiving",
        UsbTransferBindingState::Complete => "Complete",
        UsbTransferBindingState::Failed => "Failed",
    }
}

pub fn status_hint(status: UsbTransferBindingStatus) -> &'static str {
    match status.state {
        UsbTransferBindingState::Disabled => "Open this screen to enable USB transfer",
        UsbTransferBindingState::WaitingForHost => {
            "Run tools/usb_transfer/send_folder.py after byte binding is enabled"
        }
        UsbTransferBindingState::Receiving => "Keep USB connected",
        UsbTransferBindingState::Complete => "Transfer finished",
        UsbTransferBindingState::Failed => "Transfer failed; retry from host",
    }
}
