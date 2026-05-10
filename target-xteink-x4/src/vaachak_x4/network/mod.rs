#![allow(dead_code)]

//! Vaachak-owned Wi-Fi runtime for Xteink X4.
//!
//! Active Wi-Fi setup, scan, transfer, and network-time code lives here.
//! `target-xteink-x4/src/vaachak_x4` remains an imported compatibility/runtime reference and
//! must not receive new Wi-Fi features.

pub mod biscuit_wifi;
pub mod network_time;
pub mod time_status;
pub mod upload;
pub mod wifi_scan;

pub const VAACHAK_WIFI_RUNTIME_MARKER: &str = "vaachak-wifi-runtime-owned-ok";

pub struct VaachakWifiRuntimeOwnership;

impl VaachakWifiRuntimeOwnership {
    pub const fn marker() -> &'static str {
        VAACHAK_WIFI_RUNTIME_MARKER
    }
}
