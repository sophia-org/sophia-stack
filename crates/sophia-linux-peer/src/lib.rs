//! Linux socket operations absent from the runtime's safe syscall dependency.
use std::io;
use std::os::fd::{AsRawFd, BorrowedFd, FromRawFd, OwnedFd};

/// A reference to the connector captured by the socket, immune to numeric PID reuse.
pub fn socket_peer_pidfd(socket: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    let mut descriptor: libc::c_int = -1;
    let mut length = std::mem::size_of_val(&descriptor) as libc::socklen_t;
    // SAFETY: both output pointers describe live, correctly sized stack objects.
    let result = unsafe {
        libc::getsockopt(
            socket.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERPIDFD,
            (&mut descriptor as *mut libc::c_int).cast(),
            &mut length,
        )
    };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }
    if descriptor < 0 {
        return Err(io::Error::other("SO_PEERPIDFD returned no descriptor"));
    }
    // SAFETY: a successful SO_PEERPIDFD transfers a fresh close-on-exec descriptor.
    let descriptor = unsafe { OwnedFd::from_raw_fd(descriptor) };
    if length as usize != std::mem::size_of::<libc::c_int>() {
        return Err(io::Error::other("SO_PEERPIDFD size"));
    }
    Ok(descriptor)
}

/// Receive only ordinary bytes. Kernel-installed rights are closed before
/// rejecting ancillary data, even on a truncated message or peek.
pub fn recv_plain(socket: BorrowedFd<'_>, buffer: &mut [u8], peek: bool) -> io::Result<usize> {
    let mut iov = libc::iovec {
        iov_base: buffer.as_mut_ptr().cast(),
        iov_len: buffer.len(),
    };
    // Word alignment also satisfies cmsghdr alignment. The kernel closes rights
    // which did not fit this buffer; we own and close every one which did.
    let mut ancillary = [0_usize; 32];
    // SAFETY: zero is a valid empty msghdr; its pointer fields are filled below.
    let mut header: libc::msghdr = unsafe { std::mem::zeroed() };
    header.msg_iov = &mut iov;
    header.msg_iovlen = 1;
    header.msg_control = ancillary.as_mut_ptr().cast();
    header.msg_controllen = std::mem::size_of_val(&ancillary);
    let flags = libc::MSG_CMSG_CLOEXEC | libc::MSG_DONTWAIT | if peek { libc::MSG_PEEK } else { 0 };
    // SAFETY: the msghdr refers only to the live buffers above for this syscall.
    let count = unsafe { libc::recvmsg(socket.as_raw_fd(), &mut header, flags) };
    if count < 0 {
        return Err(io::Error::last_os_error());
    }
    let had_ancillary = header.msg_controllen != 0;
    // SAFETY: libc's traversal checks the bounds of the kernel-filled buffer.
    unsafe {
        let mut item = libc::CMSG_FIRSTHDR(&header);
        while !item.is_null() {
            if (*item).cmsg_level == libc::SOL_SOCKET && (*item).cmsg_type == libc::SCM_RIGHTS {
                let bytes = (*item).cmsg_len.saturating_sub(libc::CMSG_LEN(0) as usize);
                for index in 0..bytes / std::mem::size_of::<libc::c_int>() {
                    let fd = std::ptr::read_unaligned(
                        libc::CMSG_DATA(item).cast::<libc::c_int>().add(index),
                    );
                    drop(OwnedFd::from_raw_fd(fd));
                }
            }
            item = libc::CMSG_NXTHDR(&header, item);
        }
    }
    if had_ancillary || header.msg_flags & (libc::MSG_CTRUNC | libc::MSG_TRUNC) != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "control ancillary data is forbidden",
        ));
    }
    Ok(count as usize)
}

/// Start identity read while the socket-pinned connector is still alive. PID
/// numbers alone cannot authenticate provenance after a connector exits.
pub fn socket_peer_start_time(socket: BorrowedFd<'_>, pid: u32) -> io::Result<u64> {
    let pin = socket_peer_pidfd(socket)?;
    // SAFETY: zero initializes a plain credential record; getsockopt fills it.
    let mut credentials: libc::ucred = unsafe { std::mem::zeroed() };
    let mut length = std::mem::size_of_val(&credentials) as libc::socklen_t;
    // SAFETY: the output pointer and length refer to the live record above.
    let result = unsafe {
        libc::getsockopt(
            socket.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            (&mut credentials as *mut libc::ucred).cast(),
            &mut length,
        )
    };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }
    if length as usize != std::mem::size_of::<libc::ucred>()
        || credentials.pid <= 0
        || credentials.pid as u32 != pid
    {
        return Err(io::Error::other(
            "process identity does not name the socket peer",
        ));
    }
    let alive = || -> io::Result<()> {
        let mut descriptor = libc::pollfd {
            fd: pin.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        // SAFETY: poll receives one live initialized descriptor record.
        let ready = unsafe { libc::poll(&mut descriptor, 1, 0) };
        if ready == 0 {
            Ok(())
        } else {
            Err(io::Error::other(
                "X connector exited during identity capture",
            ))
        }
    };
    alive()?;
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat"))?;
    let fields = stat
        .rsplit_once(')')
        .ok_or_else(|| io::Error::other("process stat lacks comm"))?
        .1
        .split_whitespace()
        .collect::<Vec<_>>();
    let start = fields
        .get(19)
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|v| *v != 0)
        .ok_or_else(|| io::Error::other("process stat lacks start time"))?;
    alive()?;
    Ok(start)
}
