//! offer to transfer data
//!
//! A wlr_data_control_offer represents a piece of data offered for transfer
//! by another client (the source client). The offer describes the different
//! MIME types that the data can be converted to and provides the mechanism
//! for transferring the data directly from the source client.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_data_control_offer_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrDataControlOfferV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwlrDataControlOfferV1Handler>,
}

struct DefaultHandler;

impl ZwlrDataControlOfferV1Handler for DefaultHandler { }

impl ZwlrDataControlOfferV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::ZwlrDataControlOfferV1;
    pub const INTERFACE_NAME: &str = "zwlr_data_control_offer_v1";
}

impl ZwlrDataControlOfferV1 {
    pub fn set_handler(&self, handler: impl ZwlrDataControlOfferV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrDataControlOfferV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrDataControlOfferV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrDataControlOfferV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrDataControlOfferV1 {
    /// Since when the receive message is available.
    pub const MSG__RECEIVE__SINCE: u32 = 1;

    /// request that the data is transferred
    ///
    /// To transfer the offered data, the client issues this request and
    /// indicates the MIME type it wants to receive. The transfer happens
    /// through the passed file descriptor (typically created with the pipe
    /// system call). The source client writes the data in the MIME type
    /// representation requested and then closes the file descriptor.
    ///
    /// The receiving client reads from the read end of the pipe until EOF and
    /// then closes its end, at which point the transfer is complete.
    ///
    /// This request may happen multiple times for different MIME types.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: MIME type desired by receiver
    /// - `fd`: file descriptor for data transfer
    #[inline]
    pub fn send_receive(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_data_control_offer_v1#{}.receive(mime_type: {:?}, fd: {})\n", id, arg0, arg1.as_raw_fd());
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
        fmt.fds.push_back(arg1.clone());
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this offer
    ///
    /// Destroys the data offer object.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_data_control_offer_v1#{}.destroy()\n", id);
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

    /// Since when the offer message is available.
    pub const MSG__OFFER__SINCE: u32 = 1;

    /// advertise offered MIME type
    ///
    /// Sent immediately after creating the wlr_data_control_offer object.
    /// One event per offered MIME type.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: offered MIME type
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_data_control_offer_v1#{}.offer(mime_type: {:?})\n", client.endpoint.id, id, arg0);
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
        Ok(())
    }
}

/// A message handler for [ZwlrDataControlOfferV1] proxies.
pub trait ZwlrDataControlOfferV1Handler: Any {
    /// request that the data is transferred
    ///
    /// To transfer the offered data, the client issues this request and
    /// indicates the MIME type it wants to receive. The transfer happens
    /// through the passed file descriptor (typically created with the pipe
    /// system call). The source client writes the data in the MIME type
    /// representation requested and then closes the file descriptor.
    ///
    /// The receiving client reads from the read end of the pipe until EOF and
    /// then closes its end, at which point the transfer is complete.
    ///
    /// This request may happen multiple times for different MIME types.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: MIME type desired by receiver
    /// - `fd`: file descriptor for data transfer
    #[inline]
    fn receive(
        &mut self,
        _slf: &Rc<ZwlrDataControlOfferV1>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_receive(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_offer_v1.receive message: {}", Report::new(e));
        }
    }

    /// destroy this offer
    ///
    /// Destroys the data offer object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZwlrDataControlOfferV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_offer_v1.destroy message: {}", Report::new(e));
        }
    }

    /// advertise offered MIME type
    ///
    /// Sent immediately after creating the wlr_data_control_offer object.
    /// One event per offered MIME type.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: offered MIME type
    #[inline]
    fn offer(
        &mut self,
        _slf: &Rc<ZwlrDataControlOfferV1>,
        mime_type: &str,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_offer_v1.offer message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZwlrDataControlOfferV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwlrDataControlOfferV1, version),
            handler: Default::default(),
        })
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
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg1 = &arg1;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_data_control_offer_v1#{}.receive(mime_type: {:?}, fd: {})\n", client.endpoint.id, msg[0], arg0, arg1.as_raw_fd());
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).receive(&self, arg0, arg1);
                } else {
                    DefaultHandler.receive(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_data_control_offer_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_data_control_offer_v1#{}.offer(mime_type: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).offer(&self, arg0);
                } else {
                    DefaultHandler.offer(&self, arg0);
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

impl Proxy for ZwlrDataControlOfferV1 {
    fn core(&self) -> &ProxyCore {
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

