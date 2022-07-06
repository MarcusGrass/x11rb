use crate::connect::Connect;
use crate::cookie::VoidCookie;
use crate::errors::ConnectionError;
use crate::errors::{ConnectError, ReplyOrIdError};
use crate::id_allocator::IdAllocator;
use crate::parse_display::ConnectAddress;
use crate::protocol::xproto::{Atom, PropMode, Setup, Window, GE_GENERIC_EVENT};
use crate::x11_utils::{ExtensionInfoProvider, ExtensionInformation};
use crate::xcb::xproto::GetInputFocusRequest;

use heapless::{FnvIndexMap, FnvIndexSet};
use nix::libc::c_int;
use nix::poll::{poll, PollFd, PollFlags};
use std::borrow::Cow;

use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;
use std::os::unix::prelude::RawFd;
use std::time::{Duration, Instant};

const CACHE_SIZE: usize = 256;

#[derive(Debug)]
pub struct SocketConnection {
    buf: SockBuf,
    setup: Setup,
    sock_fd: RawFd,
    cur_seq: u16,
    expected_reply_seq: u16,
    event_cache: VecDeque<Vec<u8>>,
    reply_cache: FnvIndexMap<u16, Vec<u8>, CACHE_SIZE>,
    keep_seqs: FnvIndexSet<u16, CACHE_SIZE>,
    id_allocator: IdAllocator,
    max_request_length: usize,
    pub extensions: ExtensionInfoProvider,
}

impl SocketConnection {
    pub fn connect(dpy_name: Option<&str>) -> Result<(Self, usize), ConnectError> {
        // Parse display information
        let parsed_display = crate::parse_display::parse_display(dpy_name)
            .ok_or(ConnectError::DisplayParsingError)?;
        let screen: usize = parsed_display.screen.into();
        // Establish connection by iterating over ConnectAddresses until we find one that
        // works.
        let mut error = None;

        for addr in parsed_display.connect_instruction() {
            if let ConnectAddress::Socket(path) = addr {
                match UnixStream::connect(path) {
                    Ok(mut socket) => {
                        let (mut connect, setup_request) =
                            Connect::with_authorization(vec![], vec![]);
                        // write the connect() setup request
                        let mut nwritten = 0;
                        while nwritten != setup_request.len() {
                            // poll returned successfully, so the stream is writable.
                            match socket.write(&setup_request[nwritten..]) {
                                Ok(0) => {
                                    return Err(std::io::Error::new(
                                        std::io::ErrorKind::WriteZero,
                                        "failed to write whole buffer",
                                    )
                                    .into())
                                }
                                Ok(n) => nwritten += n,
                                // Spurious wakeup from poll, try again
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                                Err(e) => return Err(e.into()),
                            }
                        }

                        // read in the setup
                        loop {
                            let adv = match socket.read(connect.buffer()) {
                                Ok(0) => {
                                    return Err(std::io::Error::new(
                                        std::io::ErrorKind::UnexpectedEof,
                                        "failed to read whole buffer",
                                    )
                                    .into())
                                }
                                Ok(n) => n,
                                // Spurious wakeup from poll, try again
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    continue
                                }
                                Err(e) => return Err(e.into()),
                            };

                            // advance the internal buffer
                            if connect.advance(adv) {
                                break;
                            }
                        }

                        // resolve the setup
                        let setup = connect.into_setup()?;

                        // Check that we got a valid screen number
                        if screen >= setup.roots.len() {
                            return Err(ConnectError::InvalidScreen);
                        }
                        let sock_fd = socket.as_raw_fd();
                        let mut con = SocketConnection::new(SockBuf::new(socket), setup, sock_fd);
                        // TODO: Better err
                        con.init_extensions()
                            .map_err(|_| ConnectError::UnknownError)?;
                        con.check_for_big_req()
                            .map_err(|_| ConnectError::UnknownError)?;
                        return Ok((con, screen));
                    }
                    Err(e) => {
                        error = Some(e);
                        continue;
                    }
                }
            }
        }

        // none of the addresses worked
        Err(match error {
            Some(e) => ConnectError::IoError(e),
            None => ConnectError::DisplayParsingError,
        })
    }

    pub(crate) fn write(&mut self, buf: &[u8], forget: bool) -> Result<u16, ConnectionError> {
        let updated = update_length_field(buf, self.max_request_length)?;
        self.buf.write(&updated)?;
        let sent_seq = self.cur_seq;
        if !forget && self.keep_seqs.insert(sent_seq).is_err() {
            // TODO: Remove
            crate::debug!("Set full on write! {:?}", self.keep_seqs);
        }
        self.cur_seq = self.cur_seq.overflowing_add(1).0;
        Ok(sent_seq)
    }

    pub(crate) fn forget(&mut self, seq: u16) {
        self.keep_seqs.remove(&seq);
        self.reply_cache.remove(&seq);
    }

    /// If a reply is not expected a non-error request would result in an indefinite hang
    /// Everything is sent in sequence however, so we check to see if we've gotten a reply with a
    /// higher sequence than the void request
    pub(crate) fn block_check_for_err(
        &mut self,
        seq: u16,
    ) -> Result<Option<Vec<u8>>, ConnectionError> {
        self.keep_seqs.remove(&seq);
        if self.expected_reply_seq > seq {
            Ok(None)
        } else {
            let sync_seq = self.write(GetInputFocusRequest.serialize().as_ref(), false)?;
            let _ = self.block_for_reply(sync_seq)?;
            Ok(self.reply_cache.remove(&seq))
        }
    }

    /// Check cache if we already have the sequence otherwise read from the socket until we get it
    pub(crate) fn block_for_reply(&mut self, seq: u16) -> Result<Vec<u8>, ConnectionError> {
        if let Some(reply) = self.reply_cache.remove(&seq) {
            Ok(reply)
        } else {
            let mut target = None;
            self.keep_seqs.remove(&seq);
            while target.is_none() {
                for rr in self.buf.read_next()? {
                    match rr {
                        ReadResult::Event(e) => {
                            self.event_cache.push_back(e);
                        }
                        ReadResult::Reply(got_seq, buf) => {
                            if got_seq == seq {
                                target = Some(buf);
                            } else if self.keep_seqs.remove(&got_seq) {
                                #[allow(clippy::collapsible_if)]
                                if self.reply_cache.insert(got_seq, buf).is_err() {
                                    // TODO: Remove
                                    crate::debug!(
                                        "Map full on block for reply {:?}",
                                        self.reply_cache
                                    );
                                }
                            }
                        }
                        ReadResult::Error(got_seq, buf) => {
                            crate::debug!(
                                "Got err {:?}",
                                crate::x11_utils::X11Error::try_parse(&buf, &self.extensions)
                            );
                            if got_seq == seq {
                                target = Some(buf);
                            } else if self.keep_seqs.remove(&got_seq) {
                                #[allow(clippy::collapsible_if)]
                                if self.reply_cache.insert(got_seq, buf).is_err() {
                                    // TODO: Remove
                                    crate::debug!(
                                        "Map full on block for reply {:?}",
                                        self.reply_cache
                                    );
                                }
                            }
                        }
                    }
                }
            }
            Ok(target.unwrap())
        }
    }

    pub fn generate_id(&mut self) -> Result<u32, ReplyOrIdError> {
        if let Some(id) = self.id_allocator.generate_id() {
            Ok(id)
        } else if self
            .extension_information(crate::protocol::xc_misc::X11_EXTENSION_NAME)
            .is_none()
        {
            // IDs are exhausted and XC-MISC is not available
            Err(ReplyOrIdError::IdsExhausted)
        } else {
            let range = crate::xcb::xc_misc::get_xid_range(self, false)?.reply(self)?;
            self.id_allocator
                .update_xid_range(&range)
                .map_err(|_| ReplyOrIdError::IdsExhausted)?;
            self.id_allocator
                .generate_id()
                .ok_or(ReplyOrIdError::IdsExhausted)
        }
    }

    pub fn sync(&mut self) -> Result<(), ConnectionError> {
        self.drain()?;
        #[cfg(feature = "debug")]
        if !self.keep_seqs.is_empty() {
            crate::debug!("Forgetting {} kept seqs", self.keep_seqs.len());
        }
        self.keep_seqs.clear();
        #[cfg(feature = "debug")]
        if !self.reply_cache.is_empty() {
            crate::debug!("Forgetting {} replies", self.reply_cache.len());
        }
        self.reply_cache.clear();
        Ok(())
    }

    fn drain(&mut self) -> Result<(), ConnectionError> {
        let sync_seq = self.write(GetInputFocusRequest.serialize().as_ref(), false)?;
        let _ = self.block_for_reply(sync_seq)?;
        #[cfg(feature = "debug")]
        for (_, reply) in &self.reply_cache {
            if let Ok(err) = crate::x11_utils::X11Error::try_parse(reply, &self.extensions) {
                crate::debug!("Drained err {err:?}");
            } else {
                crate::debug!("Drained something that's not an err");
            }
        }
        Ok(())
    }

    pub fn read_next_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Vec<u8>>, ConnectionError> {
        if let Some(cached) = self.event_cache.pop_front() {
            Ok(Some(cached))
        } else {
            let start = Instant::now();
            let mut got_event = false;
            while start.elapsed() < timeout && !got_event {
                if poll_readable(self.sock_fd, timeout - start.elapsed()) {
                    for rr in self.buf.read_next()? {
                        match rr {
                            ReadResult::Event(e) => {
                                got_event = true;
                                self.event_cache.push_back(e);
                            }
                            ReadResult::Reply(seq, buf) => {
                                crate::debug!("Got reply on seq {seq}");
                                if self.keep_seqs.remove(&seq) {
                                    #[allow(clippy::collapsible_if)]
                                    if self.reply_cache.insert(seq, buf).is_err() {
                                        crate::debug!(
                                            "Map full on read next {:?}",
                                            self.reply_cache
                                        );
                                    }
                                }
                                self.expected_reply_seq =
                                    self.expected_reply_seq.overflowing_add(1).0;
                            }
                            ReadResult::Error(seq, buf) => {
                                crate::debug!(
                                    "Got err {:?}",
                                    crate::x11_utils::X11Error::try_parse(&buf, &self.extensions)
                                );
                                if self.keep_seqs.remove(&seq) {
                                    #[allow(clippy::collapsible_if)]
                                    if self.reply_cache.insert(seq, buf).is_err() {
                                        crate::debug!(
                                            "Map full on read next {:?}",
                                            self.reply_cache
                                        );
                                    };
                                }
                                self.expected_reply_seq =
                                    self.expected_reply_seq.overflowing_add(1).0;
                            }
                        }
                    }
                }
            }
            Ok(self.event_cache.pop_front())
        }
    }

    #[must_use]
    pub fn setup(&self) -> &Setup {
        &self.setup
    }

    pub fn change_property8<A, B>(
        &mut self,
        mode: PropMode,
        window: Window,
        property: A,
        type_: B,
        data: &[u8],
        forget: bool,
    ) -> Result<VoidCookie, ConnectionError>
    where
        A: Into<Atom>,
        B: Into<Atom>,
    {
        crate::xcb::xproto::change_property(
            self,
            mode,
            window,
            property,
            type_,
            8,
            data.len().try_into().expect("`data` has too many elements"),
            data,
            forget,
        )
    }

    /// Change a property on a window with format 16.
    pub fn change_property16<A, B>(
        &mut self,
        mode: PropMode,
        window: Window,
        property: A,
        type_: B,
        data: &[u16],
        forget: bool,
    ) -> Result<VoidCookie, ConnectionError>
    where
        A: Into<Atom>,
        B: Into<Atom>,
    {
        let mut data_u8 = Vec::with_capacity(data.len() * 2);
        for item in data {
            data_u8.extend(&item.to_ne_bytes());
        }
        crate::xcb::xproto::change_property(
            self,
            mode,
            window,
            property,
            type_,
            16,
            data.len().try_into().expect("`data` has too many elements"),
            &data_u8,
            forget,
        )
    }

    /// Change a property on a window with format 32.
    pub fn change_property32<A, B>(
        &mut self,
        mode: PropMode,
        window: Window,
        property: A,
        type_: B,
        data: &[u32],
        forget: bool,
    ) -> Result<VoidCookie, ConnectionError>
    where
        A: Into<Atom>,
        B: Into<Atom>,
    {
        let mut data_u8 = Vec::with_capacity(data.len() * 4);
        for item in data {
            data_u8.extend(&item.to_ne_bytes());
        }
        crate::xcb::xproto::change_property(
            self,
            mode,
            window,
            property,
            type_,
            32,
            data.len().try_into().expect("`data` has too many elements"),
            &data_u8,
            forget,
        )
    }

    pub(crate) fn extension_information(&self, name: &'static str) -> Option<ExtensionInformation> {
        self.extensions.get_by_name(name)
    }

    // Preload all extensions immediately
    fn init_extensions(&mut self) -> Result<(), ConnectionError> {
        let listed = crate::xcb::xproto::list_extensions(self, false)?;
        let reply = listed.reply(self).unwrap();
        let mut extensions = vec![];
        for name in &reply.names {
            let cookie = crate::xcb::xproto::query_extension(self, &name.name, false)?;
            extensions.push((name.name.as_slice(), cookie));
        }
        for (name, cookie) in extensions {
            let response = cookie.reply(self).unwrap();
            if response.present {
                let _ = self.extensions.extensions.push((
                    String::from_utf8(name.to_vec()).unwrap(),
                    ExtensionInformation {
                        major_opcode: response.major_opcode,
                        first_event: response.first_event,
                        first_error: response.first_error,
                    },
                ));
            }
        }
        Ok(())
    }

    fn check_for_big_req(&mut self) -> Result<(), ConnectionError> {
        crate::debug!(
            "Checking bigreq, current max_request_length = {}",
            self.max_request_length
        );
        if self
            .extension_information(crate::xcb::bigreq::X11_EXTENSION_NAME)
            .is_some()
        {
            let reply = crate::xcb::bigreq::enable(self, false)?
                .reply(self)
                .unwrap();
            self.max_request_length = reply.maximum_request_length as usize;
            crate::debug!(
                "Got max_request_length = {} from bigreq",
                self.max_request_length
            );
        }

        Ok(())
    }

    fn new(buf: SockBuf, setup: Setup, sock_fd: RawFd) -> Self {
        Self {
            max_request_length: setup.maximum_request_length as usize,
            buf,
            id_allocator: IdAllocator::new(setup.resource_id_base, setup.resource_id_mask).unwrap(),
            setup,
            sock_fd,
            cur_seq: 1,
            expected_reply_seq: 1,
            event_cache: VecDeque::new(),
            reply_cache: FnvIndexMap::new(),
            keep_seqs: FnvIndexSet::new(),
            extensions: ExtensionInfoProvider::default(),
        }
    }
}

#[inline]
fn update_length_field(buf: &[u8], max_request_bytes: usize) -> Result<Cow<[u8]>, ConnectionError> {
    let length = buf.len();
    debug_assert!(buf.len() % 4 == 0);
    if u16::try_from(length).is_ok() {
        return Ok(Cow::Borrowed(buf));
    }
    crate::debug!(
        "Got a request larger than {} bytes ({}) attempting to send with big_req",
        u16::MAX,
        length
    );
    if length > max_request_bytes {
        return Err(ConnectionError::MaximumRequestLengthExceeded);
    }
    let wire_length: u32 = (length / 4)
        .checked_add(1)
        .ok_or(ConnectionError::MaximumRequestLengthExceeded)?
        .try_into()
        .expect("Extreme request length"); // TODO: should never happen but clean up error
    let wire_length_bytes = wire_length.to_ne_bytes();
    let mut start = vec![0; (wire_length * 4) as usize];
    start[..4].copy_from_slice(&[buf[0], buf[1], 0, 0]);
    start[4..8].copy_from_slice(&wire_length_bytes);
    start[8..].copy_from_slice(&buf[4..]);
    Ok(Cow::Owned(start))
}

const BUF_SIZE: usize = 65536;

#[derive(Debug)]
struct SockBuf {
    byte_buf: Vec<u8>,
    write_offset: usize,
    read_offset: usize,
    sock: UnixStream,
}

const ERROR: u8 = 0;
const REPLY: u8 = 1;

enum ReadResult {
    Event(Vec<u8>),
    Reply(u16, Vec<u8>),
    Error(u16, Vec<u8>),
}

impl SockBuf {
    fn new(sock: UnixStream) -> Self {
        Self {
            byte_buf: vec![0; BUF_SIZE],
            write_offset: 0,
            read_offset: 0,
            sock,
        }
    }

    fn read_next(&mut self) -> Result<Vec<ReadResult>, ConnectionError> {
        self.write_offset += self
            .sock
            .read(&mut self.byte_buf[self.write_offset..])
            .map_err(ConnectionError::IoError)?;
        let mut read_results = vec![];
        while let Some(rr) = self.drain_next() {
            read_results.push(rr);
        }
        let remainder_len = self.write_offset - self.read_offset;
        self.byte_buf
            .copy_within(self.read_offset..self.write_offset, 0);
        self.read_offset = 0;
        self.write_offset = remainder_len;
        Ok(read_results)
    }

    fn write(&mut self, payload: &[u8]) -> Result<(), ConnectionError> {
        self.sock
            .write_all(payload)
            .map_err(ConnectionError::IoError)
    }

    #[allow(clippy::match_on_vec_items)]
    fn drain_next(&mut self) -> Option<ReadResult> {
        let has_length_field = match self.byte_buf.get(self.read_offset) {
            Some(&REPLY) => true,
            Some(x) if x & 0x7f == GE_GENERIC_EVENT => true,
            _ => false,
        };
        let additional_length = if has_length_field {
            if let Some(length_field) = self
                .byte_buf
                .get(self.read_offset + 4..self.read_offset + 8)
            {
                let length_field = u32::from_ne_bytes(length_field.try_into().unwrap());
                let length_field = usize::try_from(length_field).unwrap();
                debug_assert!(length_field <= usize::MAX / 4);
                4 * length_field
            } else {
                0
            }
        } else {
            0
        };
        // All packets are at least 32 bytes
        let packet_length = 32 + additional_length;
        if self.write_offset - self.read_offset < packet_length {
            // Need more data
            #[cfg(feature = "debug")]
            if self.read_offset != self.write_offset {
                crate::debug!(
                    "Need more data wo = {} ro = {}, len = {}",
                    self.write_offset,
                    self.read_offset,
                    packet_length
                );
            }
            None
        } else {
            // Got at least one full packet
            let read_result = match self.byte_buf[self.read_offset] {
                ERROR => ReadResult::Error(
                    parse_seq(&self.byte_buf[self.read_offset..]),
                    self.byte_buf[self.read_offset..self.write_offset].to_vec(),
                ),
                REPLY => ReadResult::Reply(
                    parse_seq(&self.byte_buf[self.read_offset..]),
                    self.byte_buf[self.read_offset..self.write_offset].to_vec(),
                ),
                _ => ReadResult::Event(self.byte_buf[self.read_offset..self.write_offset].to_vec()),
            };
            self.read_offset += packet_length;
            Some(read_result)
        }
    }
}

#[inline]
fn parse_seq(raw_reply: &[u8]) -> u16 {
    // The seq is at the same byte offset for both replies and errors
    u16::from_ne_bytes(raw_reply[2..4].try_into().unwrap())
}

#[inline]
fn poll_readable(fd: RawFd, deadline: Duration) -> bool {
    let mut poll_fds = [PollFd::new(fd, PollFlags::POLLIN)];
    let start_instant = Instant::now();
    loop {
        if let Some(timeout_millis) = deadline
            .checked_sub(start_instant.elapsed())
            .map(|remaining| c_int::try_from(remaining.as_millis()).unwrap_or(c_int::MAX))
        {
            match poll(&mut poll_fds, timeout_millis) {
                Ok(_) => {
                    if poll_fds[0]
                        .revents()
                        .unwrap_or_else(PollFlags::empty)
                        .contains(PollFlags::POLLIN)
                    {
                        return true;
                    }
                }
                // try again
                Err(nix::Error::EINTR) => {}
                Err(e) => panic!("{e}"),
            }
        } else {
            return false;
        }
        if start_instant.elapsed() >= deadline {
            return false;
        }
    }
}
