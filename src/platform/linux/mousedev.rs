#![allow(dead_code)]
use libc::{input_event, timeval};
use std::fmt::{self, Debug, Display};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;

/// Identifier of an input device.
///
/// Whenever you receive an event arising from a particular input device, this event contains a `DeviceId` which
/// identifies its origin. Note that devices may be virtual (representing an on-screen cursor and keyboard focus) or
/// physical. Virtual devices typically aggregate inputs from multiple physical devices.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(usize);

pub struct Device {
    file: File,
}

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl Device {
    pub fn open<P: AsRef<Path>>(path: P, non_block: bool) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        if non_block {
            unsafe {
                let fd = file.as_raw_fd();
                let mut flags = libc::fcntl(fd, libc::F_GETFL, 0);
                flags |= libc::O_NONBLOCK;
                libc::fcntl(fd, libc::F_SETFL, flags);
            }
        }
        Ok(Self { file })
    }
}
