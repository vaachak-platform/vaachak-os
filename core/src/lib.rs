#![no_std]

extern crate alloc;

pub mod hal;
pub mod ui;
pub mod apps;
pub mod services;
pub mod os;

pub use hal::Hal;
pub use os::VaachakOs;
