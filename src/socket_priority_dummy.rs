use crate::dscp::set_dscp_for_priority;
use crate::Priority;
use std::io::Error;
use std::os::unix::io::RawFd;

#[allow(unused_variables)]
pub fn set_priority(fd: RawFd, prio: Priority) -> Result<(), Error> {
    set_dscp_for_priority(fd, prio)
}
