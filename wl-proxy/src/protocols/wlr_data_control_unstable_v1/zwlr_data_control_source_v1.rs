//! offer to transfer data
//!
//! The wlr_data_control_source object is the source side of a
//! wlr_data_control_offer. It is created by the source client in a data
//! transfer and provides a way to describe the offered data and a way to
//! respond to requests to transfer the data.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_data_control_source_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrDataControlSourceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrDataControlSourceV1Handler>,
}

struct DefaultHandler;

impl ZwlrDataControlSourceV1Handler for DefaultHandler { }

impl ZwlrDataControlSourceV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwlrDataControlSourceV1;
    pub const INTERFACE_NAME: &str = "zwlr_data_control_source_v1";
}

impl ZwlrDataControlSourceV1 {
    pub fn set_handler(&self, handler: impl ZwlrDataControlSourceV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrDataControlSourceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrDataControlSourceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrDataControlSourceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrDataControlSourceV1 {
    /// Since when the offer message is available.
    pub const MSG__OFFER__SINCE: u32 = 1;

    /// add an offered MIME type
    ///
    /// This request adds a MIME type to the set of MIME types advertised to
    /// targets. Can be called several times to offer multiple types.
    ///
    /// Calling this after wlr_data_control_device.set_selection is a protocol
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_data_control_source_v1#{}.offer(mime_type: {:?})\n", id, arg0);
            self.core.state.log(args);
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_data_control_source_v1#{}.destroy()\n", id);
            self.core.state.log(args);
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_data_control_source_v1#{}.send(mime_type: {:?}, fd: {})\n", client.endpoint.id, id, arg0, arg1.as_raw_fd());
            self.core.state.log(args);
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
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_data_control_source_v1#{}.cancelled()\n", client.endpoint.id, id);
            self.core.state.log(args);
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

/// A message handler for [ZwlrDataControlSourceV1] proxies.
pub trait ZwlrDataControlSourceV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrDataControlSourceV1>) {
        let _ = slf.core.delete_id();
    }

    /// add an offered MIME type
    ///
    /// This request adds a MIME type to the set of MIME types advertised to
    /// targets. Can be called several times to offer multiple types.
    ///
    /// Calling this after wlr_data_control_device.set_selection is a protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: MIME type offered by the data source
    #[inline]
    fn handle_offer(
        &mut self,
        _slf: &Rc<ZwlrDataControlSourceV1>,
        mime_type: &str,
    ) {
        let res = _slf.send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_source_v1.offer message: {}", Report::new(e));
        }
    }

    /// destroy this source
    ///
    /// Destroys the data source object.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<ZwlrDataControlSourceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_source_v1.destroy message: {}", Report::new(e));
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
        _slf: &Rc<ZwlrDataControlSourceV1>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_send(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_source_v1.send message: {}", Report::new(e));
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
        _slf: &Rc<ZwlrDataControlSourceV1>,
    ) {
        let res = _slf.send_cancelled(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_source_v1.cancelled message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ZwlrDataControlSourceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrDataControlSourceV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow() else {
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
        let Some(mut handler) = self.handler.try_borrow() else {
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
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_data_control_source_v1#{}.offer(mime_type: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
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
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_data_control_source_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
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
        let Some(mut handler) = self.handler.try_borrow() else {
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
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_data_control_source_v1#{}.send(mime_type: {:?}, fd: {})\n", msg[0], arg0, arg1.as_raw_fd());
                    self.core.state.log(args);
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
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_data_control_source_v1#{}.cancelled()\n", msg[0]);
                    self.core.state.log(args);
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

impl Object for ZwlrDataControlSourceV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl ZwlrDataControlSourceV1 {
    /// Since when the error.invalid_offer enum variant is available.
    pub const ENM__ERROR_INVALID_OFFER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrDataControlSourceV1Error(pub u32);

impl ZwlrDataControlSourceV1Error {
    /// offer sent after wlr_data_control_device.set_selection
    pub const INVALID_OFFER: Self = Self(1);
}

impl Debug for ZwlrDataControlSourceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_OFFER => "INVALID_OFFER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
