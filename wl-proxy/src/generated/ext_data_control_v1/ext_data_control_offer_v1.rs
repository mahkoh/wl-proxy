//! offer to transfer data
//!
//! A ext_data_control_offer represents a piece of data offered for transfer
//! by another client (the source client). The offer describes the different
//! MIME types that the data can be converted to and provides the mechanism
//! for transferring the data directly from the source client.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_data_control_offer_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtDataControlOfferV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtDataControlOfferV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtDataControlOfferV1MessageHandler for DefaultMessageHandler { }

impl MetaExtDataControlOfferV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtDataControlOfferV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtDataControlOfferV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtDataControlOfferV1 {
    /// Since when the receive message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
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

    /// destroy this offer
    ///
    /// Destroys the data offer object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        Ok(())
    }

    /// Since when the offer message is available.
    #[allow(dead_code)]
    pub const MSG__OFFER__SINCE: u32 = 1;

    /// advertise offered MIME type
    ///
    /// Sent immediately after creating the ext_data_control_offer object.
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
        ]);
        fmt.string(arg0);
        Ok(())
    }
}

/// A message handler for [ExtDataControlOfferV1] proxies.
#[allow(dead_code)]
pub trait MetaExtDataControlOfferV1MessageHandler {
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
        _slf: &Rc<MetaExtDataControlOfferV1>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_receive(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_offer_v1.receive message: {}", Report::new(e));
        }
    }

    /// destroy this offer
    ///
    /// Destroys the data offer object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtDataControlOfferV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_offer_v1.destroy message: {}", Report::new(e));
        }
    }

    /// advertise offered MIME type
    ///
    /// Sent immediately after creating the ext_data_control_offer object.
    /// One event per offered MIME type.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: offered MIME type
    #[inline]
    fn offer(
        &mut self,
        _slf: &Rc<MetaExtDataControlOfferV1>,
        mime_type: &str,
    ) {
        let res = _slf.send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_offer_v1.offer message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtDataControlOfferV1 {
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

