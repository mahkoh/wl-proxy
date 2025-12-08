//! offer to transfer primary selection contents
//!
//! A wp_primary_selection_offer represents an offer to transfer the contents
//! of the primary selection clipboard to the client. Similar to
//! wl_data_offer, the offer also describes the mime types that the data can
//! be converted to and provides the mechanisms for transferring the data
//! directly to the client.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_primary_selection_offer_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpPrimarySelectionOfferV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpPrimarySelectionOfferV1Handler>,
}

struct DefaultHandler;

impl ZwpPrimarySelectionOfferV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpPrimarySelectionOfferV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpPrimarySelectionOfferV1;
    const INTERFACE_NAME: &str = "zwp_primary_selection_offer_v1";
}

impl ZwpPrimarySelectionOfferV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpPrimarySelectionOfferV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpPrimarySelectionOfferV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpPrimarySelectionOfferV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpPrimarySelectionOfferV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpPrimarySelectionOfferV1 {
    /// Since when the receive message is available.
    pub const MSG__RECEIVE__SINCE: u32 = 1;

    /// request that the data is transferred
    ///
    /// To transfer the contents of the primary selection clipboard, the client
    /// issues this request and indicates the mime type that it wants to
    /// receive. The transfer happens through the passed file descriptor
    /// (typically created with the pipe system call). The source client writes
    /// the data in the mime type representation requested and then closes the
    /// file descriptor.
    ///
    /// The receiving client reads from the read end of the pipe until EOF and
    /// closes its end, at which point the transfer is complete.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    /// - `fd`:
    #[inline]
    pub fn try_send_receive(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_primary_selection_offer_v1#{}.receive(mime_type: {:?}, fd: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1.as_raw_fd());
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
        fmt.fds.push_back(arg1.clone());
        Ok(())
    }

    /// request that the data is transferred
    ///
    /// To transfer the contents of the primary selection clipboard, the client
    /// issues this request and indicates the mime type that it wants to
    /// receive. The transfer happens through the passed file descriptor
    /// (typically created with the pipe system call). The source client writes
    /// the data in the mime type representation requested and then closes the
    /// file descriptor.
    ///
    /// The receiving client reads from the read end of the pipe until EOF and
    /// closes its end, at which point the transfer is complete.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    /// - `fd`:
    #[inline]
    pub fn send_receive(
        &self,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = self.try_send_receive(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log_send("zwp_primary_selection_offer_v1.receive", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the primary selection offer
    ///
    /// Destroy the primary selection offer.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_primary_selection_offer_v1#{}.destroy()\n", id);
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

    /// destroy the primary selection offer
    ///
    /// Destroy the primary selection offer.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_primary_selection_offer_v1.destroy", &e);
        }
    }

    /// Since when the offer message is available.
    pub const MSG__OFFER__SINCE: u32 = 1;

    /// advertise offered mime type
    ///
    /// Sent immediately after creating announcing the
    /// wp_primary_selection_offer through
    /// wp_primary_selection_device.data_offer. One event is sent per offered
    /// mime type.
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_primary_selection_offer_v1#{}.offer(mime_type: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
        Ok(())
    }

    /// advertise offered mime type
    ///
    /// Sent immediately after creating announcing the
    /// wp_primary_selection_offer through
    /// wp_primary_selection_device.data_offer. One event is sent per offered
    /// mime type.
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
            log_send("zwp_primary_selection_offer_v1.offer", &e);
        }
    }
}

/// A message handler for [`ZwpPrimarySelectionOfferV1`] proxies.
pub trait ZwpPrimarySelectionOfferV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpPrimarySelectionOfferV1>) {
        slf.core.delete_id();
    }

    /// request that the data is transferred
    ///
    /// To transfer the contents of the primary selection clipboard, the client
    /// issues this request and indicates the mime type that it wants to
    /// receive. The transfer happens through the passed file descriptor
    /// (typically created with the pipe system call). The source client writes
    /// the data in the mime type representation requested and then closes the
    /// file descriptor.
    ///
    /// The receiving client reads from the read end of the pipe until EOF and
    /// closes its end, at which point the transfer is complete.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    /// - `fd`:
    #[inline]
    fn handle_receive(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionOfferV1>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_receive(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_offer_v1.receive", &e);
        }
    }

    /// destroy the primary selection offer
    ///
    /// Destroy the primary selection offer.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionOfferV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_offer_v1.destroy", &e);
        }
    }

    /// advertise offered mime type
    ///
    /// Sent immediately after creating announcing the
    /// wp_primary_selection_offer through
    /// wp_primary_selection_device.data_offer. One event is sent per offered
    /// mime type.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    #[inline]
    fn handle_offer(
        &mut self,
        slf: &Rc<ZwpPrimarySelectionOfferV1>,
        mime_type: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log_forward("zwp_primary_selection_offer_v1.offer", &e);
        }
    }
}

impl ObjectPrivate for ZwpPrimarySelectionOfferV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpPrimarySelectionOfferV1, version),
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
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd")));
                };
                let arg1 = &arg1;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_primary_selection_offer_v1#{}.receive(mime_type: {:?}, fd: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1.as_raw_fd());
                }
                if let Some(handler) = handler {
                    (**handler).handle_receive(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_receive(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_primary_selection_offer_v1#{}.destroy()\n", client_id, id);
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_primary_selection_offer_v1#{}.offer(mime_type: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_offer(&self, arg0);
                } else {
                    DefaultHandler.handle_offer(&self, arg0);
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
            0 => "receive",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "offer",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpPrimarySelectionOfferV1 {
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

