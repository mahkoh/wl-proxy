//! offer to replace the contents of the primary selection
//!
//! The source side of a wp_primary_selection_offer, it provides a way to
//! describe the offered data and respond to requests to transfer the
//! requested contents of the primary selection clipboard.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_primary_selection_source_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpPrimarySelectionSourceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpPrimarySelectionSourceV1Handler>,
}

struct DefaultHandler;

impl ZwpPrimarySelectionSourceV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpPrimarySelectionSourceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpPrimarySelectionSourceV1;
    const INTERFACE_NAME: &str = "zwp_primary_selection_source_v1";
}

impl ZwpPrimarySelectionSourceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpPrimarySelectionSourceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpPrimarySelectionSourceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpPrimarySelectionSourceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpPrimarySelectionSourceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpPrimarySelectionSourceV1 {
    /// Since when the offer message is available.
    pub const MSG__OFFER__SINCE: u32 = 1;

    /// add an offered mime type
    ///
    /// This request adds a mime type to the set of mime types advertised to
    /// targets. Can be called several times to offer multiple types.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    #[inline]
    pub fn try_send_offer(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_primary_selection_source_v1#{}.offer(mime_type: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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

    /// add an offered mime type
    ///
    /// This request adds a mime type to the set of mime types advertised to
    /// targets. Can be called several times to offer multiple types.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    #[inline]
    pub fn send_offer(
        &self,
        mime_type: &str,
    ) {
        let res = self.try_send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log_send("zwp_primary_selection_source_v1.offer", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the primary selection source
    ///
    /// Destroy the primary selection source.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_primary_selection_source_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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

    /// destroy the primary selection source
    ///
    /// Destroy the primary selection source.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_primary_selection_source_v1.destroy", &e);
        }
    }

    /// Since when the send message is available.
    pub const MSG__SEND__SINCE: u32 = 1;

    /// send the primary selection contents
    ///
    /// Request for the current primary selection contents from the client.
    /// Send the specified mime type over the passed file descriptor, then
    /// close it.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    /// - `fd`:
    #[inline]
    pub fn try_send_send(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_primary_selection_source_v1#{}.send(mime_type: {:?}, fd: {})\n", client_id, id, arg0, arg1);
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

    /// send the primary selection contents
    ///
    /// Request for the current primary selection contents from the client.
    /// Send the specified mime type over the passed file descriptor, then
    /// close it.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    /// - `fd`:
    #[inline]
    pub fn send_send(
        &self,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = self.try_send_send(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log_send("zwp_primary_selection_source_v1.send", &e);
        }
    }

    /// Since when the cancelled message is available.
    pub const MSG__CANCELLED__SINCE: u32 = 1;

    /// request for primary selection contents was canceled
    ///
    /// This primary selection source is no longer valid. The client should
    /// clean up and destroy this primary selection source.
    #[inline]
    pub fn try_send_cancelled(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_primary_selection_source_v1#{}.cancelled()\n", client_id, id);
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

    /// request for primary selection contents was canceled
    ///
    /// This primary selection source is no longer valid. The client should
    /// clean up and destroy this primary selection source.
    #[inline]
    pub fn send_cancelled(
        &self,
    ) {
        let res = self.try_send_cancelled(
        );
        if let Err(e) = res {
            log_send("zwp_primary_selection_source_v1.cancelled", &e);
        }
    }
}

/// A message handler for [`ZwpPrimarySelectionSourceV1`] proxies.
pub trait ZwpPrimarySelectionSourceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpPrimarySelectionSourceV1>) {
        slf.core.delete_id();
    }

    /// add an offered mime type
    ///
    /// This request adds a mime type to the set of mime types advertised to
    /// targets. Can be called several times to offer multiple types.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    #[inline]
    fn handle_offer(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionSourceV1>,
        mime_type: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_source_v1.offer", &e);
        }
    }

    /// destroy the primary selection source
    ///
    /// Destroy the primary selection source.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionSourceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_source_v1.destroy", &e);
        }
    }

    /// send the primary selection contents
    ///
    /// Request for the current primary selection contents from the client.
    /// Send the specified mime type over the passed file descriptor, then
    /// close it.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    /// - `fd`:
    #[inline]
    fn handle_send(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionSourceV1>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_send(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_source_v1.send", &e);
        }
    }

    /// request for primary selection contents was canceled
    ///
    /// This primary selection source is no longer valid. The client should
    /// clean up and destroy this primary selection source.
    #[inline]
    fn handle_cancelled(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionSourceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_cancelled(
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_source_v1.cancelled", &e);
        }
    }
}

impl ObjectPrivate for ZwpPrimarySelectionSourceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpPrimarySelectionSourceV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "mime_type")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_primary_selection_source_v1#{}.offer(mime_type: {:?})\n", client_id, id, arg0);
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_primary_selection_source_v1#{}.destroy()\n", client_id, id);
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
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "mime_type")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd")));
                };
                let arg1 = &arg1;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_primary_selection_source_v1#{}.send(mime_type: {:?}, fd: {})\n", id, arg0, arg1);
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_primary_selection_source_v1#{}.cancelled()\n", id);
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
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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

impl Object for ZwpPrimarySelectionSourceV1 {
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

