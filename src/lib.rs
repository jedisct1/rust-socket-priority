#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate libc;

mod priority;
pub use priority::Priority;

#[cfg(target_os = "linux")]
pub use socket_priority_linux::*;
#[cfg(target_os = "linux")]
mod socket_priority_linux;

#[cfg(any(target_os = "macos", target_os = "openbsd"))]
pub use socket_priority_tos::*;
#[cfg(any(target_os = "macos", target_os = "openbsd"))]
mod socket_priority_tos;

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub use socket_priority_dummy::*;
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
mod socket_priority_dummy;