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
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
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
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
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
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultMessageHandler.release(&self);
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
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
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                if let Some(handler) = handler {
                    (**handler).refresh(&self, arg0);
                } else {
                    DefaultMessageHandler.refresh(&self, arg0);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).preferred(&self);
                } else {
                    DefaultMessageHandler.preferred(&self);
                }
            }
            3 => {
                if let Some(handler) = handler {
                    (**handler).finished(&self);
                } else {
                    DefaultMessageHandler.finished(&self);
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

