use std::os::fd::AsFd;

/// Brackets CPU writes with the DMA-BUF exporter's synchronization hooks.
/// The caller must end every successfully begun access, including on map failure.
#[cfg(target_os = "linux")]
pub fn native_dmabuf_cpu_write_access(fd: impl AsFd, end: bool) -> std::io::Result<()> {
    const SYNC: rustix::ioctl::Opcode = rustix::ioctl::opcode::write::<u64>(b'b', 0);
    // GBM map_mut requests read/write access; END is bit 2.
    let flags: u64 = 3 | if end { 4 } else { 0 };
    loop {
        // SAFETY: DMA_BUF_IOCTL_SYNC reads dma_buf_sync, whose only field is u64.
        let result = unsafe {
            rustix::ioctl::ioctl(fd.as_fd(), rustix::ioctl::Setter::<SYNC, u64>::new(flags))
        };
        match result {
            Ok(()) => return Ok(()),
            Err(rustix::io::Errno::INTR | rustix::io::Errno::AGAIN) => continue,
            Err(error) => return Err(error.into()),
        }
    }
}

#[cfg(not(target_os = "linux"))]
pub fn native_dmabuf_cpu_write_access(_: impl AsFd, _: bool) -> std::io::Result<()> {
    Err(std::io::ErrorKind::Unsupported.into())
}
