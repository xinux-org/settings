use input::LibinputInterface;
use std::fs::OpenOptions;
use std::os::fd::OwnedFd;
use std::path::Path;

pub struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .read((flags & libc::O_RDONLY) != 0)
            .write((flags & libc::O_WRONLY) != 0 || (flags & libc::O_RDWR) != 0)
            .open(path)
            .map(Into::into)
            .map_err(|err| err.raw_os_error().unwrap())
    }

    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(fd);
    }
}
