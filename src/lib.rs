#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate libc;

mod dscp;
mod priority;
pub use priority::Priority;

#[cfg(target_os = "linux")]
pub use socket_priority_linux::*;
#[cfg(target_os = "linux")]
mod socket_priority_linux;

#[cfg(not(target_os = "linux"))]
pub use socket_priority_dummy::*;
#[cfg(not(target_os = "linux"))]
mod socket_priority_dummy;
