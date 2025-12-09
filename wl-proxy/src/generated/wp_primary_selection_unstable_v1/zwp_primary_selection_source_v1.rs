//! offer to replace the contents of the primary selection
//!
//! The source side of a wp_primary_selection_offer, it provides a way to
//! describe the offered data and respond to requests to transfer the
//! requested contents of the primary selection clipboard.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_primary_selection_source_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpPrimarySelectionSourceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpPrimarySelectionSourceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpPrimarySelectionSourceV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpPrimarySelectionSourceV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpPrimarySelectionSourceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpPrimarySelectionSourceV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpPrimarySelectionSourceV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpPrimarySelectionSourceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpPrimarySelectionSourceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpPrimarySelectionSourceV1 {
    /// Since when the offer message is available.
    #[allow(dead_code)]
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the primary selection source
    ///
    /// Destroy the primary selection source.
    #[inline]
    pub fn send_destroy(
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the send message is available.
    #[allow(dead_code)]
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
        ]);
        fmt.string(arg0);
        fmt.fds.push_back(arg1.clone());
        Ok(())
    }

    /// Since when the cancelled message is available.
    #[allow(dead_code)]
    pub const MSG__CANCELLED__SINCE: u32 = 1;

    /// request for primary selection contents was canceled
    ///
    /// This primary selection source is no longer valid. The client should
    /// clean up and destroy this primary selection source.
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
        Ok(())
    }
}

/// A message handler for [ZwpPrimarySelectionSourceV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpPrimarySelectionSourceV1MessageHandler {
    /// add an offered mime type
    ///
    /// This request adds a mime type to the set of mime types advertised to
    /// targets. Can be called several times to offer multiple types.
    ///
    /// # Arguments
    ///
    /// - `mime_type`:
    #[inline]
    fn offer(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionSourceV1>,
        mime_type: &str,
    ) {
        let res = _slf.send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_source_v1.offer message: {}", Report::new(e));
        }
    }

    /// destroy the primary selection source
    ///
    /// Destroy the primary selection source.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionSourceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_source_v1.destroy message: {}", Report::new(e));
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
    fn send(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionSourceV1>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_send(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_source_v1.send message: {}", Report::new(e));
        }
    }

    /// request for primary selection contents was canceled
    ///
    /// This primary selection source is no longer valid. The client should
    /// clean up and destroy this primary selection source.
    #[inline]
    fn cancelled(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionSourceV1>,
    ) {
        let res = _slf.send_cancelled(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_source_v1.cancelled message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpPrimarySelectionSourceV1 {
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
                if let Some(handler) = handler {
                    (**handler).offer(&self, arg0);
                } else {
                    DefaultMessageHandler.offer(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
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
                if let Some(handler) = handler {
                    (**handler).send(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.send(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).cancelled(&self);
                } else {
                    DefaultMessageHandler.cancelled(&self);
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

