#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

mod dscp;
mod priority;
#[cfg(target_os = "linux")]
pub use socket_priority_linux::*;

pub use crate::priority::Priority;
#[cfg(target_os = "linux")]
mod socket_priority_linux;

#[cfg(not(target_os = "linux"))]
pub use crate::socket_priority_dummy::*;
#[cfg(not(target_os = "linux"))]
mod socket_priority_dummy;
