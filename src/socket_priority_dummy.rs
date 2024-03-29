use std::io::Error;
use std::os::unix::io::RawFd;

use crate::dscp::set_dscp_for_priority;
use crate::Priority;

#[allow(unused_variables)]
pub fn set_priority(fd: RawFd, prio: Priority) -> Result<(), Error> {
    set_dscp_for_priority(fd, prio)
}
