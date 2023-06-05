use evdev::InputEvent;
use libc;
use nix;
use std::{fs::File, os::fd::AsRawFd};

/// `https://github.com/emberian/evdev/blob/master/src/raw_stream.rs`
/// Read a maximum of `num` events into the internal buffer. If the underlying fd is not
/// O_NONBLOCK, this will block.
///
/// Returns the number of events that were read, or an error.
pub(crate) fn fill_events(
    event_buf: &mut Vec<libc::input_event>,
    fd: &File,
) -> std::io::Result<usize> {
    event_buf.reserve(32);
    let spare_capacity = event_buf.spare_capacity_mut();
    let spare_capacity_size = std::mem::size_of_val(spare_capacity);

    // use libc::read instead of nix::unistd::read b/c we need to pass an uninitialized buf
    let res = unsafe {
        libc::read(
            fd.as_raw_fd(),
            spare_capacity.as_mut_ptr() as _,
            spare_capacity_size,
        )
    };
    let bytes_read = nix::errno::Errno::result(res)?;
    let num_read = bytes_read as usize / std::mem::size_of::<libc::input_event>();
    unsafe {
        let len = event_buf.len();
        event_buf.set_len(len + num_read);
    }
    Ok(num_read)
}

/// `https://github.com/emberian/evdev/blob/master/src/raw_stream.rs`
/// Fetches and returns events from the kernel ring buffer without doing synchronization on
/// SYN_DROPPED.
///
/// By default this will block until events are available. Typically, users will want to call
/// this in a tight loop within a thread.
pub fn fetch_events<'a>(
    event_buf: &'a mut Vec<libc::input_event>,
    fd: &'a File,
) -> std::io::Result<impl Iterator<Item = InputEvent> + 'a> {
    fill_events(event_buf, fd)?;
    Ok(event_buf.drain(..).map(InputEvent::from))
}

fn fd_write_all(fd: std::os::fd::BorrowedFd<'_>, mut data: &[u8]) -> nix::Result<()> {
    loop {
        match nix::unistd::write(fd.as_raw_fd(), data) {
            Ok(0) => return Ok(()),
            Ok(n) => data = &data[n..],
            Err(e) if e == nix::Error::EINTR => {}
            Err(e) => return Err(e),
        }
    }
}

pub(crate) unsafe fn cast_to_bytes<T: ?Sized>(mem: &T) -> &[u8] {
    std::slice::from_raw_parts(mem as *const T as *const u8, std::mem::size_of_val(mem))
}

pub fn write_events(fd: std::os::fd::BorrowedFd<'_>, events: &[InputEvent]) -> nix::Result<()> {
    let bytes = unsafe { cast_to_bytes(events) };
    fd_write_all(fd, bytes)
}
