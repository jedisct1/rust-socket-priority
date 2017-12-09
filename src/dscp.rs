use libc::c_int;
use std::io::Error;
use std::os::unix::io::RawFd;
use Priority;

#[cfg(target_os = "linux")]
const IP_TOS: c_int = 1;

#[cfg(any(target_os = "openbsd", target_os = "freebsd", target_os = "dragonfly",
          target_os = "bitrig", target_os = "macos"))]
const IP_TOS: c_int = 3;

#[cfg(not(any(target_os = "linux", target_os = "openbsd", target_os = "freebsd",
              target_os = "dragonfly", target_os = "bitrig", target_os = "macos")))]
#[allow(unused_variables)]
pub fn set_dscp_for_priority(fd: RawFd, prio: Priority) -> Result<(), Error> {
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "openbsd", target_os = "freebsd",
          target_os = "dragonfly", target_os = "bitrig", target_os = "macos"))]
pub fn set_dscp_for_priority(fd: RawFd, prio: Priority) -> Result<(), Error> {
    use std::mem::size_of_val;
    use libc::{c_void, setsockopt, socklen_t, IPPROTO_IP};

    const IPTOS_DSCP_AF31: c_int = 0x68;

    let tos: c_int = match prio {
        Priority::Interactive => IPTOS_DSCP_AF31,
        Priority::Default => 0x0,
        Priority::InteractiveBulk => 0x0,
        Priority::Bulk => 0x0,
    };
    match unsafe {
        setsockopt(
            fd as c_int,
            IPPROTO_IP,
            IP_TOS,
            &tos as *const _ as *const c_void,
            size_of_val(&tos) as socklen_t,
        )
    } {
        0 => Ok(()),
        _ => Err(Error::last_os_error()),
    }
}
