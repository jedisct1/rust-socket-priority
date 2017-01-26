use std::io::Error;
use std::os::unix::io::RawFd;

use Priority;

#[allow(unused_variables)]
pub fn set_priority(fd: RawFd, prio: Priority) -> Result<(), Error> {
    Ok(())
}