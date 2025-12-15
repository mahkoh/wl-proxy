//! offer to transfer data
//!
//! The ext_data_control_source object is the source side of a
//! ext_data_control_offer. It is created by the source client in a data
//! transfer and provides a way to describe the offered data and a way to
//! respond to requests to transfer the data.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_data_control_source_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtDataControlSourceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtDataControlSourceV1Handler>,
}

struct DefaultHandler;

impl ExtDataControlSourceV1Handler for DefaultHandler { }

impl ExtDataControlSourceV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ExtDataControlSourceV1;
    pub const INTERFACE_NAME: &str = "ext_data_control_source_v1";
}

impl ExtDataControlSourceV1 {
    pub fn set_handler(&self, handler: impl ExtDataControlSourceV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ExtDataControlSourceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtDataControlSourceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtDataControlSourceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtDataControlSourceV1 {
    /// Since when the offer message is available.
    pub const MSG__OFFER__SINCE: u32 = 1;

    /// add an offered MIME type
    ///
    /// This request adds a MIME type to the set of MIME types advertised to
    /// targets. Can be called several times to offer multiple types.
    ///
    /// Calling this after ext_data_control_device.set_selection is a protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: MIME type offered by the data source
    #[inline]
    pub fn send_offer(
        &self,
        mime_type: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mime_type,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_data_control_source_v1#{}.offer(mime_type: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this source
    ///
    /// Destroys the data source object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_data_control_source_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the send message is available.
    pub const MSG__SEND__SINCE: u32 = 1;

    /// send the data
    ///
    /// Request for data from the client. Send the data as the specified MIME
    /// type over the passed file descriptor, then close it.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: MIME type for the data
    /// - `fd`: file descriptor for the data
    #[inline]
    pub fn send_send(
        &self,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            mime_type,
            fd,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_data_control_source_v1#{}.send(mime_type: {:?}, fd: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1.as_raw_fd());
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        fmt.string(arg0);
        fmt.fds.push_back(arg1.clone());
        Ok(())
    }

    /// Since when the cancelled message is available.
    pub const MSG__CANCELLED__SINCE: u32 = 1;

    /// selection was cancelled
    ///
    /// This data source is no longer valid. The data source has been replaced
    /// by another data source.
    ///
    /// The client should clean up and destroy this data source.
    #[inline]
    pub fn send_cancelled(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_data_control_source_v1#{}.cancelled()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtDataControlSourceV1] proxies.
pub trait ExtDataControlSourceV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtDataControlSourceV1>) {
        let _ = slf.core.delete_id();
    }

    /// add an offered MIME type
    ///
    /// This request adds a MIME type to the set of MIME types advertised to
    /// targets. Can be called several times to offer multiple types.
    ///
    /// Calling this after ext_data_control_device.set_selection is a protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: MIME type offered by the data source
    #[inline]
    fn handle_offer(
        &mut self,
        _slf: &Rc<ExtDataControlSourceV1>,
        mime_type: &str,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_source_v1.offer message: {}", Report::new(e));
        }
    }

    /// destroy this source
    ///
    /// Destroys the data source object.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<ExtDataControlSourceV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_source_v1.destroy message: {}", Report::new(e));
        }
    }

    /// send the data
    ///
    /// Request for data from the client. Send the data as the specified MIME
    /// type over the passed file descriptor, then close it.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: MIME type for the data
    /// - `fd`: file descriptor for the data
    #[inline]
    fn handle_send(
        &mut self,
        _slf: &Rc<ExtDataControlSourceV1>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.send_send(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_source_v1.send message: {}", Report::new(e));
        }
    }

    /// selection was cancelled
    ///
    /// This data source is no longer valid. The data source has been replaced
    /// by another data source.
    ///
    /// The client should clean up and destroy this data source.
    #[inline]
    fn handle_cancelled(
        &mut self,
        _slf: &Rc<ExtDataControlSourceV1>,
    ) {
        if !_slf.core.forward_to_client.get() {
            return;
        }
        let res = _slf.send_cancelled(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_source_v1.cancelled message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ExtDataControlSourceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtDataControlSourceV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("mime_type"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("mime_type"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("mime_type"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("mime_type"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_data_control_source_v1#{}.offer(mime_type: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_offer(&self, arg0);
                } else {
                    DefaultHandler.handle_offer(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_data_control_source_v1#{}.destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
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
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("mime_type"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("mime_type"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("mime_type"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("mime_type"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg1 = &arg1;
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_data_control_source_v1#{}.send(mime_type: {:?}, fd: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1.as_raw_fd());
                }
                if let Some(handler) = handler {
                    (**handler).handle_send(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_send(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_data_control_source_v1#{}.cancelled()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_cancelled(&self);
                } else {
                    DefaultHandler.handle_cancelled(&self);
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
            0 => "offer",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "send",
            1 => "cancelled",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ExtDataControlSourceV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl ExtDataControlSourceV1 {
    /// Since when the error.invalid_offer enum variant is available.
    pub const ENM__ERROR_INVALID_OFFER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtDataControlSourceV1Error(pub u32);

impl ExtDataControlSourceV1Error {
    /// offer sent after ext_data_control_device.set_selection
    pub const INVALID_OFFER: Self = Self(1);
}

impl Debug for ExtDataControlSourceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_OFFER => "INVALID_OFFER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
