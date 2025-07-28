use nix::unistd::Pid;
use nix::libc::{c_void, iovec, pid_t};
use nix::sys::uio::pread;
use std::fs::File;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

/// Read `size` bytes from the given file at a specific offset
pub fn read_at_offset(file: &File, offset: usize, size: usize) -> io::Result<Vec<u8>> {
    let mut buffer = vec![0u8; size];
    let bytes_read = pread(file, &mut buffer, offset as i64).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("pread failed: {:?}", e))
    })?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}

/// Open a file and read `size` bytes at `offset`
pub fn read_file_offset(path: &str, offset: usize, size: usize) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    read_at_offset(&file, offset, size)
}
