use {
    isnt::std_1::primitive::IsntSliceExt,
    smallvec::SmallVec,
    std::{
        collections::VecDeque,
        io,
        mem::{self, MaybeUninit},
        os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd},
        rc::Rc,
        slice,
    },
    thiserror::Error,
    uapi::{Errno, Msghdr, MsghdrMut, c, sockaddr_none_mut, sockaddr_none_ref},
};

#[cfg(test)]
mod tests;

const WORD_SIZE: usize = size_of::<u32>();
const MAX_MESSAGE_SIZE: usize = 4096;
const MAX_MESSAGE_WORDS: usize = MAX_MESSAGE_SIZE / WORD_SIZE;
const BUFFER_LEN: usize = MAX_MESSAGE_WORDS * 2;
const BUFFER_SIZE: usize = BUFFER_LEN * WORD_SIZE;
const HEADER_WORDS: usize = 2;
const HEADER_SIZE: usize = HEADER_WORDS * WORD_SIZE;

pub(crate) struct InputBuffer {
    buffer: [u32; BUFFER_LEN],
    valid_from_word: usize,
    valid_bytes: usize,
}

pub(crate) struct OutputBuffer {
    buffer: [u32; BUFFER_LEN],
    valid_from_byte: usize,
    valid_to_byte: usize,
    fds: VecDeque<Rc<OwnedFd>>,
    fd_offsets: VecDeque<FdOffset>,
}

struct FdOffset {
    offset_bytes: usize,
    num_fds: usize,
}

#[derive(Eq, PartialEq)]
pub(crate) enum FlushResult {
    Done,
    Blocked,
}

pub(crate) struct MessageFormatter<'a> {
    pub(crate) buffer: &'a mut [u32],
    pub(crate) words_written: usize,
    pub(crate) fds: &'a mut VecDeque<Rc<OwnedFd>>,
    old_fds_len: usize,
    fd_offsets: &'a mut VecDeque<FdOffset>,
    valid_to_byte: &'a mut usize,
}

#[derive(Default)]
pub(crate) struct OutputSwapchain {
    pending: VecDeque<Box<OutputBuffer>>,
    stash: Vec<Box<OutputBuffer>>,
}

#[derive(Debug, Error)]
pub enum TransError {
    #[error("failed to read from socket")]
    ReadFromSocket(#[source] io::Error),
    #[error("failed to write to socket")]
    WriteToSocket(#[source] io::Error),
    #[error("the connection is closed")]
    Closed,
    #[error("message has a supposed length {0} < {HEADER_SIZE}")]
    MessageTooSmall(usize),
    #[error("message has a supposed length {0} > {MAX_MESSAGE_SIZE}")]
    MessageTooLarge(usize),
    #[error("message has a supposed length {0} that is not a multiple of {WORD_SIZE}")]
    MessageNotAligned(usize),
}

pub(crate) fn read_message<'a>(
    socket: RawFd,
    may_read_from_socket: &mut bool,
    buffer: &'a mut InputBuffer,
    fds: &mut VecDeque<Rc<OwnedFd>>,
) -> Result<Option<&'a [u32]>, TransError> {
    if buffer.valid_bytes == 0 {
        buffer.valid_from_word = 0;
    }
    if buffer.valid_from_word + HEADER_WORDS > BUFFER_LEN {
        buffer.buffer.copy_within(buffer.valid_from_word.., 0);
        buffer.valid_from_word = 0;
    }
    if buffer.valid_bytes < HEADER_SIZE {
        if mem::take(may_read_from_socket) {
            read_from_socket(socket, buffer, fds)?;
        }
        if buffer.valid_bytes < HEADER_SIZE {
            return Ok(None);
        }
    }
    let size = (buffer.buffer[buffer.valid_from_word + 1] >> 16) as usize;
    if size < HEADER_SIZE {
        return Err(TransError::MessageTooSmall(size));
    }
    if size > MAX_MESSAGE_SIZE {
        return Err(TransError::MessageTooLarge(size));
    }
    if size % WORD_SIZE != 0 {
        return Err(TransError::MessageNotAligned(size));
    }
    let size_words = size / WORD_SIZE;
    if buffer.valid_from_word + size_words > BUFFER_LEN {
        let start = buffer.valid_from_word * WORD_SIZE;
        let buf = uapi::as_bytes_mut(&mut buffer.buffer);
        buf.copy_within(start..start + buffer.valid_bytes, 0);
        buffer.valid_from_word = 0;
    };
    if size > buffer.valid_bytes {
        if mem::take(may_read_from_socket) {
            read_from_socket(socket, buffer, fds)?;
        }
        if size > buffer.valid_bytes {
            return Ok(None);
        }
    }
    let start = buffer.valid_from_word;
    let end = start + size_words;
    buffer.valid_from_word += size_words;
    buffer.valid_bytes -= size;
    Ok(Some(&buffer.buffer[start..end]))
}

fn read_from_socket(
    fd: RawFd,
    buffer: &mut InputBuffer,
    fds: &mut VecDeque<Rc<OwnedFd>>,
) -> Result<(), TransError> {
    let mut iovec =
        &mut uapi::as_bytes_mut(&mut buffer.buffer[buffer.valid_from_word..])[buffer.valid_bytes..];
    let mut control_buf = [0u8; 128];
    let mut header = MsghdrMut {
        iov: slice::from_mut(&mut iovec),
        control: Some(&mut control_buf),
        name: sockaddr_none_mut(),
        flags: 0,
    };
    let (init, _, mut control) =
        match uapi::recvmsg(fd, &mut header, c::MSG_CMSG_CLOEXEC | c::MSG_DONTWAIT) {
            Ok(r) => r,
            Err(e) if e.0 == c::EAGAIN => return Ok(()),
            Err(e) => {
                return Err(TransError::ReadFromSocket(io::Error::from_raw_os_error(
                    e.0,
                )));
            }
        };
    buffer.valid_bytes += init.len();
    while control.is_not_empty() {
        let (_, hdr, data) = uapi::cmsg_read(&mut control).unwrap();
        if hdr.cmsg_level != c::SOL_SOCKET || hdr.cmsg_type != c::SCM_RIGHTS {
            continue;
        }
        for fd in uapi::pod_iter::<RawFd, _>(data).unwrap() {
            // SAFETY: The kernel guarantees that fd is valid
            unsafe {
                fds.push_back(Rc::new(OwnedFd::from_raw_fd(fd)));
            }
        }
    }
    Ok(())
}

pub(crate) fn flush_buffer(
    socket: RawFd,
    buffer: &mut OutputBuffer,
) -> Result<FlushResult, TransError> {
    loop {
        if buffer.valid_to_byte == buffer.valid_from_byte {
            return Ok(FlushResult::Done);
        }
        if write_to_socket(socket, buffer)? == FlushResult::Blocked {
            return Ok(FlushResult::Blocked);
        }
    }
}

fn write_to_socket(socket: RawFd, buffer: &mut OutputBuffer) -> Result<FlushResult, TransError> {
    let start = buffer.valid_from_byte;
    let mut end = buffer.valid_to_byte;
    let mut fd_offset = None;
    if let Some(fdo) = buffer.fd_offsets.front()
        && fdo.offset_bytes == start
    {
        fd_offset = buffer.fd_offsets.pop_front();
    }
    if let Some(fdo) = buffer.fd_offsets.front() {
        end = fdo.offset_bytes;
    }
    let mut control_buf = SmallVec::<[MaybeUninit<u8>; 128]>::new();
    let mut control = None;
    if let Some(fdo) = &fd_offset {
        let data_len = size_of::<RawFd>() * fdo.num_fds;
        let cmsg_space = uapi::cmsg_space(data_len);
        control_buf.reserve_exact(cmsg_space);
        // SAFETY: control_buf contains only MaybeUninit elements.
        unsafe {
            control_buf.set_len(cmsg_space);
        }
        let hdr = c::cmsghdr {
            cmsg_len: 0,
            cmsg_level: c::SOL_SOCKET,
            cmsg_type: c::SCM_RIGHTS,
        };
        let mut fds = SmallVec::<[RawFd; 128 / 4]>::new();
        for idx in 0..fdo.num_fds {
            fds.push(buffer.fds[idx].as_raw_fd());
        }
        let mut buf = &mut control_buf[..];
        uapi::cmsg_write(&mut buf, hdr, &fds[..]).unwrap();
        control = Some(&control_buf[..]);
    }
    let buf = &uapi::as_bytes(&buffer.buffer[..])[start..end];
    let msghdr = Msghdr {
        iov: slice::from_ref(&buf),
        control,
        name: sockaddr_none_ref(),
    };
    match uapi::sendmsg(socket, &msghdr, c::MSG_NOSIGNAL | c::MSG_DONTWAIT) {
        Ok(n) => {
            if let Some(fdo) = fd_offset {
                buffer.fds.drain(..fdo.num_fds);
            }
            buffer.valid_from_byte += n;
            Ok(FlushResult::Done)
        }
        Err(e) if e.0 == c::EAGAIN => {
            if let Some(fdo) = fd_offset {
                buffer.fd_offsets.push_front(fdo);
            }
            Ok(FlushResult::Blocked)
        }
        Err(Errno(c::ECONNRESET)) => Err(TransError::Closed),
        Err(Errno(c::EPIPE)) => Err(TransError::Closed),
        Err(e) => Err(TransError::WriteToSocket(io::Error::from_raw_os_error(e.0))),
    }
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            buffer: [0; BUFFER_LEN],
            valid_from_word: 0,
            valid_bytes: 0,
        }
    }
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self {
            buffer: [0; BUFFER_LEN],
            valid_from_byte: 0,
            valid_to_byte: 0,
            fds: Default::default(),
            fd_offsets: Default::default(),
        }
    }
}

impl OutputBuffer {
    pub(crate) fn formatter(&mut self) -> Option<MessageFormatter<'_>> {
        if self.valid_from_byte == self.valid_to_byte {
            self.valid_from_byte = 0;
            self.valid_to_byte = 0;
        }
        if self.valid_to_byte + MAX_MESSAGE_SIZE > BUFFER_SIZE {
            return None;
        }
        assert_eq!(self.valid_to_byte % WORD_SIZE, 0);
        Some(MessageFormatter {
            buffer: &mut self.buffer[self.valid_to_byte / 4..],
            words_written: 0,
            old_fds_len: self.fds.len(),
            fds: &mut self.fds,
            fd_offsets: &mut self.fd_offsets,
            valid_to_byte: &mut self.valid_to_byte,
        })
    }
}

impl Drop for MessageFormatter<'_> {
    fn drop(&mut self) {
        assert!(self.words_written >= HEADER_WORDS);
        let num_fds = self.fds.len() - self.old_fds_len;
        if num_fds > 0 {
            self.fd_offsets.push_back(FdOffset {
                offset_bytes: *self.valid_to_byte,
                num_fds,
            });
        }
        let message_size = self.words_written * 4;
        self.buffer[1] |= (message_size as u32) << 16;
        *self.valid_to_byte += message_size;
    }
}

impl OutputSwapchain {
    pub(crate) fn formatter(&mut self) -> MessageFormatter<'_> {
        if let Some(last) = self.pending.back_mut()
            && let Some(fmt) = last.formatter()
        {
            // This is a limitation in the borrow checker. Without this transmute, the
            // return causes the self.pending borrow to last till the end of the function.
            return unsafe { mem::transmute::<MessageFormatter<'_>, MessageFormatter<'_>>(fmt) };
        }
        let fmt = self.stash.pop().unwrap_or_default();
        self.pending.push_back(fmt);
        self.pending.back_mut().unwrap().formatter().unwrap()
    }

    pub(crate) fn flush(&mut self, fd: RawFd) -> Result<FlushResult, TransError> {
        while let Some(buf) = self.pending.front_mut() {
            match flush_buffer(fd, buf)? {
                FlushResult::Done => {
                    let buf = self.pending.pop_front().unwrap();
                    self.stash.push(buf);
                }
                FlushResult::Blocked => return Ok(FlushResult::Blocked),
            }
        }
        Ok(FlushResult::Done)
    }
}
