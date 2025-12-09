//! group of input devices
//!
//! A seat is a group of keyboards, pointer and touch devices. This
//! object is published as a global during start up, or when such a
//! device is hot plugged.  A seat typically has a pointer and
//! maintains a keyboard focus and a pointer focus.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_seat proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlSeat {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlSeatMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlSeatMessageHandler for DefaultMessageHandler { }

impl MetaWlSeat {
    pub const XML_VERSION: u32 = 10;
}

impl MetaWlSeat {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlSeat, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaWlSeatMessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaWlSeat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlSeat")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlSeat {
    /// Since when the capabilities message is available.
    #[allow(dead_code)]
    pub const MSG__CAPABILITIES__SINCE: u32 = 1;

    /// seat capabilities changed
    ///
    /// This is sent on binding to the seat global or whenever a seat gains
    /// or loses the pointer, keyboard or touch capabilities.
    /// The argument is a capability enum containing the complete set of
    /// capabilities this seat has.
    ///
    /// When the pointer capability is added, a client may create a
    /// wl_pointer object using the wl_seat.get_pointer request. This object
    /// will receive pointer events until the capability is removed in the
    /// future.
    ///
    /// When the pointer capability is removed, a client should destroy the
    /// wl_pointer objects associated with the seat where the capability was
    /// removed, using the wl_pointer.release request. No further pointer
    /// events will be received on these objects.
    ///
    /// In some compositors, if a seat regains the pointer capability and a
    /// client has a previously obtained wl_pointer object of version 4 or
    /// less, that object may start sending pointer events again. This
    /// behavior is considered a misinterpretation of the intended behavior
    /// and must not be relied upon by the client. wl_pointer objects of
    /// version 5 or later must not send events if created before the most
    /// recent event notifying the client of an added pointer capability.
    ///
    /// The above behavior also applies to wl_keyboard and wl_touch with the
    /// keyboard and touch capabilities, respectively.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities of the seat
    #[inline]
    pub fn send_capabilities(
        &self,
        capabilities: MetaWlSeatCapability,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            capabilities,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the get_pointer message is available.
    #[allow(dead_code)]
    pub const MSG__GET_POINTER__SINCE: u32 = 1;

    /// return pointer object
    ///
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn send_get_pointer(
        &self,
        id: &Rc<MetaWlPointer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the get_keyboard message is available.
    #[allow(dead_code)]
    pub const MSG__GET_KEYBOARD__SINCE: u32 = 1;

    /// return keyboard object
    ///
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn send_get_keyboard(
        &self,
        id: &Rc<MetaWlKeyboard>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the get_touch message is available.
    #[allow(dead_code)]
    pub const MSG__GET_TOUCH__SINCE: u32 = 1;

    /// return touch object
    ///
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn send_get_touch(
        &self,
        id: &Rc<MetaWlTouch>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the name message is available.
    #[allow(dead_code)]
    pub const MSG__NAME__SINCE: u32 = 2;

    /// unique identifier for this seat
    ///
    /// In a multi-seat configuration the seat name can be used by clients to
    /// help identify which physical devices the seat represents.
    ///
    /// The seat name is a UTF-8 string with no convention defined for its
    /// contents. Each name is unique among all wl_seat globals. The name is
    /// only guaranteed to be unique for the current compositor instance.
    ///
    /// The same seat names are used for all clients. Thus, the name can be
    /// shared across processes to refer to a specific wl_seat global.
    ///
    /// The name event is sent after binding to the seat global, and should be sent
    /// before announcing capabilities. This event only sent once per seat object,
    /// and the name does not change over the lifetime of the wl_seat global.
    ///
    /// Compositors may re-use the same seat name if the wl_seat global is
    /// destroyed and re-created later.
    ///
    /// # Arguments
    ///
    /// - `name`: seat identifier
    #[inline]
    pub fn send_name(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the release message is available.
    #[allow(dead_code)]
    pub const MSG__RELEASE__SINCE: u32 = 5;

    /// release the seat object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the seat object anymore.
    #[inline]
    pub fn send_release(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [WlSeat] proxies.
#[allow(dead_code)]
pub trait MetaWlSeatMessageHandler {
    /// seat capabilities changed
    ///
    /// This is sent on binding to the seat global or whenever a seat gains
    /// or loses the pointer, keyboard or touch capabilities.
    /// The argument is a capability enum containing the complete set of
    /// capabilities this seat has.
    ///
    /// When the pointer capability is added, a client may create a
    /// wl_pointer object using the wl_seat.get_pointer request. This object
    /// will receive pointer events until the capability is removed in the
    /// future.
    ///
    /// When the pointer capability is removed, a client should destroy the
    /// wl_pointer objects associated with the seat where the capability was
    /// removed, using the wl_pointer.release request. No further pointer
    /// events will be received on these objects.
    ///
    /// In some compositors, if a seat regains the pointer capability and a
    /// client has a previously obtained wl_pointer object of version 4 or
    /// less, that object may start sending pointer events again. This
    /// behavior is considered a misinterpretation of the intended behavior
    /// and must not be relied upon by the client. wl_pointer objects of
    /// version 5 or later must not send events if created before the most
    /// recent event notifying the client of an added pointer capability.
    ///
    /// The above behavior also applies to wl_keyboard and wl_touch with the
    /// keyboard and touch capabilities, respectively.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities of the seat
    #[inline]
    fn capabilities(
        &mut self,
        _slf: &Rc<MetaWlSeat>,
        capabilities: MetaWlSeatCapability,
    ) {
        let res = _slf.send_capabilities(
            capabilities,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_seat.capabilities message: {}", Report::new(e));
        }
    }

    /// return pointer object
    ///
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat pointer
    #[inline]
    fn get_pointer(
        &mut self,
        _slf: &Rc<MetaWlSeat>,
        id: &Rc<MetaWlPointer>,
    ) {
        let res = _slf.send_get_pointer(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_seat.get_pointer message: {}", Report::new(e));
        }
    }

    /// return keyboard object
    ///
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat keyboard
    #[inline]
    fn get_keyboard(
        &mut self,
        _slf: &Rc<MetaWlSeat>,
        id: &Rc<MetaWlKeyboard>,
    ) {
        let res = _slf.send_get_keyboard(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_seat.get_keyboard message: {}", Report::new(e));
        }
    }

    /// return touch object
    ///
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `id`: seat touch interface
    #[inline]
    fn get_touch(
        &mut self,
        _slf: &Rc<MetaWlSeat>,
        id: &Rc<MetaWlTouch>,
    ) {
        let res = _slf.send_get_touch(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_seat.get_touch message: {}", Report::new(e));
        }
    }

    /// unique identifier for this seat
    ///
    /// In a multi-seat configuration the seat name can be used by clients to
    /// help identify which physical devices the seat represents.
    ///
    /// The seat name is a UTF-8 string with no convention defined for its
    /// contents. Each name is unique among all wl_seat globals. The name is
    /// only guaranteed to be unique for the current compositor instance.
    ///
    /// The same seat names are used for all clients. Thus, the name can be
    /// shared across processes to refer to a specific wl_seat global.
    ///
    /// The name event is sent after binding to the seat global, and should be sent
    /// before announcing capabilities. This event only sent once per seat object,
    /// and the name does not change over the lifetime of the wl_seat global.
    ///
    /// Compositors may re-use the same seat name if the wl_seat global is
    /// destroyed and re-created later.
    ///
    /// # Arguments
    ///
    /// - `name`: seat identifier
    #[inline]
    fn name(
        &mut self,
        _slf: &Rc<MetaWlSeat>,
        name: &str,
    ) {
        let res = _slf.send_name(
            name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_seat.name message: {}", Report::new(e));
        }
    }

    /// release the seat object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the seat object anymore.
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_seat.release message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlSeat {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaWlPointer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_pointer(&self, arg0);
                } else {
                    DefaultMessageHandler.get_pointer(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaWlKeyboard::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_keyboard(&self, arg0);
                } else {
                    DefaultMessageHandler.get_keyboard(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaWlTouch::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_touch(&self, arg0);
                } else {
                    DefaultMessageHandler.get_touch(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultMessageHandler.release(&self);
                }
                self.core.handle_client_destroy();
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaWlSeatCapability(arg0);
                if let Some(handler) = handler {
                    (**handler).capabilities(&self, arg0);
                } else {
                    DefaultMessageHandler.capabilities(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if let Some(handler) = handler {
                    (**handler).name(&self, arg0);
                } else {
                    DefaultMessageHandler.name(&self, arg0);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "get_pointer",
            1 => "get_keyboard",
            2 => "get_touch",
            3 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "capabilities",
            1 => "name",
            _ => return None,
        };
        Some(name)
    }
}

impl MetaWlSeat {
    /// Since when the capability.pointer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_POINTER__SINCE: u32 = 1;
    /// Since when the capability.keyboard enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_KEYBOARD__SINCE: u32 = 1;
    /// Since when the capability.touch enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_TOUCH__SINCE: u32 = 1;

    /// Since when the error.missing_capability enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_MISSING_CAPABILITY__SINCE: u32 = 1;
}

/// seat capability bitmask
///
/// This is a bitmask of capabilities this seat has; if a member is
/// set, then it is present on the seat.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
#[allow(dead_code)]
pub struct MetaWlSeatCapability(pub u32);

/// An iterator over the set bits in a [MetaWlSeatCapability].
///
/// You can construct this with the `IntoIterator` implementation of `MetaWlSeatCapability`.
#[derive(Clone, Debug)]
pub struct MetaWlSeatCapabilityIter(pub u32);

impl MetaWlSeatCapability {
    /// the seat has pointer devices
    #[allow(dead_code)]
    pub const POINTER: Self = Self(1);

    /// the seat has one or more keyboards
    #[allow(dead_code)]
    pub const KEYBOARD: Self = Self(2);

    /// the seat has touch devices
    #[allow(dead_code)]
    pub const TOUCH: Self = Self(4);
}

#[allow(dead_code)]
impl MetaWlSeatCapability {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 1 | 2 | 4)
    }
}

impl Iterator for MetaWlSeatCapabilityIter {
    type Item = MetaWlSeatCapability;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(MetaWlSeatCapability(bit))
    }
}

impl IntoIterator for MetaWlSeatCapability {
    type Item = MetaWlSeatCapability;
    type IntoIter = MetaWlSeatCapabilityIter;

    fn into_iter(self) -> Self::IntoIter {
        MetaWlSeatCapabilityIter(self.0)
    }
}

impl BitAnd for MetaWlSeatCapability {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for MetaWlSeatCapability {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for MetaWlSeatCapability {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for MetaWlSeatCapability {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for MetaWlSeatCapability {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for MetaWlSeatCapability {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for MetaWlSeatCapability {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for MetaWlSeatCapability {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for MetaWlSeatCapability {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for MetaWlSeatCapability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("POINTER")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("KEYBOARD")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("TOUCH")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}

/// wl_seat error values
///
/// These errors can be emitted in response to wl_seat requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlSeatError(pub u32);

impl MetaWlSeatError {
    /// get_pointer, get_keyboard or get_touch called on seat without the matching capability
    #[allow(dead_code)]
    pub const MISSING_CAPABILITY: Self = Self(0);
}

impl Debug for MetaWlSeatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::MISSING_CAPABILITY => "MISSING_CAPABILITY",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
