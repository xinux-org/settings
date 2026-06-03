use input::LibinputInterface;
use libc::{O_ACCMODE, O_RDONLY, O_RDWR, O_WRONLY};
use std::fs::OpenOptions;
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;

pub struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_ACCMODE == O_RDONLY) | (flags & O_ACCMODE == O_RDWR))
            .write((flags & O_ACCMODE == O_WRONLY) | (flags & O_ACCMODE == O_RDWR))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }

    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(fd);
    }
}
