use dscp::set_dscp_for_priority;
use std::io::Error;
use std::mem::size_of_val;
use std::os::unix::io::RawFd;
use libc::{c_int, c_void, setsockopt, socklen_t, SOL_SOCKET};

use Priority;

const SO_PRIORITY: c_int = 12;

pub fn set_priority(fd: RawFd, prio: Priority) -> Result<(), Error> {
    let linux_prio: c_int = match prio {
        Priority::Interactive => 6,
        Priority::Default => 0,
        Priority::InteractiveBulk => 4,
        Priority::Bulk => 2,
    };
    match unsafe {
        setsockopt(fd as c_int,
                   SOL_SOCKET,
                   SO_PRIORITY,
                   &linux_prio as *const _ as *const c_void,
                   size_of_val(&linux_prio) as socklen_t)
    } {
        0 => Ok(()),
        _ => Err(Error::last_os_error()),
    }?;
    set_dscp_for_priority(fd, prio)
}