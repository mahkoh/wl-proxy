//! offer to transfer data
//!
//! The wlr_data_control_source object is the source side of a
//! wlr_data_control_offer. It is created by the source client in a data
//! transfer and provides a way to describe the offered data and a way to
//! respond to requests to transfer the data.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_data_control_source_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrDataControlSourceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrDataControlSourceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrDataControlSourceV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrDataControlSourceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrDataControlSourceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrDataControlSourceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrDataControlSourceV1 {
    /// Since when the offer message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
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

    /// destroy this source
    ///
    /// Destroys the data source object.
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

    /// Since when the send message is available.
    #[allow(dead_code)]
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
        fmt.fds.push_back(arg1.clone());
        Ok(())
    }

    /// Since when the cancelled message is available.
    #[allow(dead_code)]
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwlrDataControlSourceV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrDataControlSourceV1MessageHandler {
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
    fn offer(
        &mut self,
        _slf: &Rc<MetaZwlrDataControlSourceV1>,
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
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrDataControlSourceV1>,
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
    fn send(
        &mut self,
        _slf: &Rc<MetaZwlrDataControlSourceV1>,
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
    fn cancelled(
        &mut self,
        _slf: &Rc<MetaZwlrDataControlSourceV1>,
    ) {
        let res = _slf.send_cancelled(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_source_v1.cancelled message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrDataControlSourceV1 {
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
                if let Some(handler) = handler {
                    (**handler).offer(&self, arg0);
                } else {
                    DefaultMessageHandler.offer(&self, arg0);
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
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError);
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).send(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.send(&self, arg0, arg1);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).cancelled(&self);
                } else {
                    DefaultMessageHandler.cancelled(&self);
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

impl MetaZwlrDataControlSourceV1 {
    /// Since when the error.invalid_offer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_OFFER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrDataControlSourceV1Error(pub u32);

impl MetaZwlrDataControlSourceV1Error {
    /// offer sent after wlr_data_control_device.set_selection
    #[allow(dead_code)]
    pub const INVALID_OFFER: Self = Self(1);
}

impl Debug for MetaZwlrDataControlSourceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_OFFER => "INVALID_OFFER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
