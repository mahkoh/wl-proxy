//! offer to transfer primary selection contents
//!
//! A wp_primary_selection_offer represents an offer to transfer the contents
//! of the primary selection clipboard to the client. Similar to
//! wl_data_offer, the offer also describes the mime types that the data can
//! be converted to and provides the mechanisms for transferring the data
//! directly to the client.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_primary_selection_offer_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpPrimarySelectionOfferV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpPrimarySelectionOfferV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpPrimarySelectionOfferV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpPrimarySelectionOfferV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpPrimarySelectionOfferV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpPrimarySelectionOfferV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpPrimarySelectionOfferV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpPrimarySelectionOfferV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpPrimarySelectionOfferV1 {
    /// Since when the receive message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
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
        fmt.fds.push_back(arg1.clone());
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the primary selection offer
    ///
    /// Destroy the primary selection offer.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
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

    /// Since when the offer message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
        ]);
        fmt.string(arg0);
        Ok(())
    }
}

/// A message handler for [ZwpPrimarySelectionOfferV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpPrimarySelectionOfferV1MessageHandler {
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
    fn receive(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionOfferV1>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_receive(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_offer_v1.receive message: {}", Report::new(e));
        }
    }

    /// destroy the primary selection offer
    ///
    /// Destroy the primary selection offer.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionOfferV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_offer_v1.destroy message: {}", Report::new(e));
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
    fn offer(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionOfferV1>,
        mime_type: &str,
    ) {
        let res = _slf.send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_offer_v1.offer message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpPrimarySelectionOfferV1 {
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
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError);
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).receive(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.receive(&self, arg0, arg1);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
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
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                if let Some(handler) = handler {
                    (**handler).offer(&self, arg0);
                } else {
                    DefaultMessageHandler.offer(&self, arg0);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

