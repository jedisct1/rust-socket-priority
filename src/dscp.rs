use std::io::Error;
use std::mem::size_of_val;
use std::os::unix::io::RawFd;
use libc::{c_int, c_void, setsockopt, socklen_t, IPPROTO_IP};
use Priority;

const IP_TOS: c_int = 3;
const IPTOS_DSCP_AF31: c_int = 0x68;

#[cfg(not(unix))]
pub fn set_dscp_for_priority(fd: RawFd, prio: Priority) -> Result<(), Error> {
    Ok(())
}

#[cfg(unix)]
pub fn set_dscp_for_priority(fd: RawFd, prio: Priority) -> Result<(), Error> {
    let tos: c_int = match prio {
        Priority::Interactive => IPTOS_DSCP_AF31,
        Priority::Default => 0x0,
        Priority::InteractiveBulk => 0x0,
        Priority::Bulk => 0x0,
    };
    match unsafe {
        setsockopt(fd as c_int,
                   IPPROTO_IP,
                   IP_TOS,
                   &tos as *const _ as *const c_void,
                   size_of_val(&tos) as socklen_t)
    } {
        0 => Ok(()),
        _ => Err(Error::last_os_error()),
    }
}
