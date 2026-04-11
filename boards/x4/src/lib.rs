#![no_std]

pub mod board;
pub mod pins;

pub use board::{STRIP_BYTES, STRIP_HEIGHT, X4Board, X4Display, X4StripTarget};
pub use vaachak_core::{LOGICAL_HEIGHT, LOGICAL_WIDTH};
pub use vaachak_drivers::ssd1677::{NATIVE_HEIGHT, NATIVE_WIDTH};
