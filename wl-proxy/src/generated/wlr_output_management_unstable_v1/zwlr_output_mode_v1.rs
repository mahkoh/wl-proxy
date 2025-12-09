//! output mode
//!
//! This object describes an output mode.
//!
//! Some heads don't support output modes, in which case modes won't be
//! advertised.
//!
//! Properties sent via this interface are applied atomically via the
//! wlr_output_manager.done event. No guarantees are made regarding the order
//! in which properties are sent.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_mode_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrOutputModeV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrOutputModeV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrOutputModeV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrOutputModeV1 {
    pub const XML_VERSION: u32 = 3;
}

impl MetaZwlrOutputModeV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrOutputModeV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwlrOutputModeV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwlrOutputModeV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrOutputModeV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrOutputModeV1 {
    /// Since when the size message is available.
    #[allow(dead_code)]
    pub const MSG__SIZE__SINCE: u32 = 1;

    /// mode size
    ///
    /// This event describes the mode size. The size is given in physical
    /// hardware units of the output device. This is not necessarily the same as
    /// the output size in the global compositor space. For instance, the output
    /// may be scaled or transformed.
    ///
    /// # Arguments
    ///
    /// - `width`: width of the mode in hardware units
    /// - `height`: height of the mode in hardware units
    #[inline]
    pub fn send_size(
        &self,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            width,
            height,
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the refresh message is available.
    #[allow(dead_code)]
    pub const MSG__REFRESH__SINCE: u32 = 1;

    /// mode refresh rate
    ///
    /// This event describes the mode's fixed vertical refresh rate. It is only
    /// sent if the mode has a fixed refresh rate.
    ///
    /// # Arguments
    ///
    /// - `refresh`: vertical refresh rate in mHz
    #[inline]
    pub fn send_refresh(
        &self,
        refresh: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            refresh,
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// Since when the preferred message is available.
    #[allow(dead_code)]
    pub const MSG__PREFERRED__SINCE: u32 = 1;

    /// mode is preferred
    ///
    /// This event advertises this mode as preferred.
    #[inline]
    pub fn send_preferred(
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
            2,
        ]);
        Ok(())
    }

    /// Since when the finished message is available.
    #[allow(dead_code)]
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the mode has disappeared
    ///
    /// This event indicates that the mode is no longer available. The mode
    /// object becomes inert. Clients should send a destroy request and release
    /// any resources associated with it.
    #[inline]
    pub fn send_finished(
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
            3,
        ]);
        Ok(())
    }

    /// Since when the release message is available.
    #[allow(dead_code)]
    pub const MSG__RELEASE__SINCE: u32 = 3;

    /// destroy the mode object
    ///
    /// This request indicates that the client will no longer use this mode
    /// object.
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
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ZwlrOutputModeV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrOutputModeV1MessageHandler {
    /// mode size
    ///
    /// This event describes the mode size. The size is given in physical
    /// hardware units of the output device. This is not necessarily the same as
    /// the output size in the global compositor space. For instance, the output
    /// may be scaled or transformed.
    ///
    /// # Arguments
    ///
    /// - `width`: width of the mode in hardware units
    /// - `height`: height of the mode in hardware units
    #[inline]
    fn size(
        &mut self,
        _slf: &Rc<MetaZwlrOutputModeV1>,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_size(
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_mode_v1.size message: {}", Report::new(e));
        }
    }

    /// mode refresh rate
    ///
    /// This event describes the mode's fixed vertical refresh rate. It is only
    /// sent if the mode has a fixed refresh rate.
    ///
    /// # Arguments
    ///
    /// - `refresh`: vertical refresh rate in mHz
    #[inline]
    fn refresh(
        &mut self,
        _slf: &Rc<MetaZwlrOutputModeV1>,
        refresh: i32,
    ) {
        let res = _slf.send_refresh(
            refresh,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_mode_v1.refresh message: {}", Report::new(e));
        }
    }

    /// mode is preferred
    ///
    /// This event advertises this mode as preferred.
    #[inline]
    fn preferred(
        &mut self,
        _slf: &Rc<MetaZwlrOutputModeV1>,
    ) {
        let res = _slf.send_preferred(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_mode_v1.preferred message: {}", Report::new(e));
        }
    }

    /// the mode has disappeared
    ///
    /// This event indicates that the mode is no longer available. The mode
    /// object becomes inert. Clients should send a destroy request and release
    /// any resources associated with it.
    #[inline]
    fn finished(
        &mut self,
        _slf: &Rc<MetaZwlrOutputModeV1>,
    ) {
        let res = _slf.send_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_mode_v1.finished message: {}", Report::new(e));
        }
    }

    /// destroy the mode object
    ///
    /// This request indicates that the client will no longer use this mode
    /// object.
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<MetaZwlrOutputModeV1>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_mode_v1.release message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrOutputModeV1 {
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).size(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.size(&self, arg0, arg1);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = arg0 as i32;
                if let Some(handler) = handler {
                    (**handler).refresh(&self, arg0);
                } else {
                    DefaultMessageHandler.refresh(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).preferred(&self);
                } else {
                    DefaultMessageHandler.preferred(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).finished(&self);
                } else {
                    DefaultMessageHandler.finished(&self);
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
            0 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "size",
            1 => "refresh",
            2 => "preferred",
            3 => "finished",
            _ => return None,
        };
        Some(name)
    }
}

