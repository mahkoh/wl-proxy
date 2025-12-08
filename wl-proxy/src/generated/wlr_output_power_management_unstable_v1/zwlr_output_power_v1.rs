//! adjust power management mode for an output
//!
//! This object offers requests to set the power management mode of
//! an output.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_power_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrOutputPowerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrOutputPowerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrOutputPowerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrOutputPowerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrOutputPowerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrOutputPowerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrOutputPowerV1 {
    /// Since when the set_mode message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MODE__SINCE: u32 = 1;

    /// Set an outputs power save mode
    ///
    /// Set an output's power save mode to the given mode. The mode change
    /// is effective immediately. If the output does not support the given
    /// mode a failed event is sent.
    ///
    /// # Arguments
    ///
    /// - `mode`: the power save mode to set
    #[inline]
    pub fn send_set_mode(
        &self,
        mode: MetaZwlrOutputPowerV1Mode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the mode message is available.
    #[allow(dead_code)]
    pub const MSG__MODE__SINCE: u32 = 1;

    /// Report a power management mode change
    ///
    /// Report the power management mode change of an output.
    ///
    /// The mode event is sent after an output changed its power
    /// management mode. The reason can be a client using set_mode or the
    /// compositor deciding to change an output's mode.
    /// This event is also sent immediately when the object is created
    /// so the client is informed about the current power management mode.
    ///
    /// # Arguments
    ///
    /// - `mode`: the output's new power management mode
    #[inline]
    pub fn send_mode(
        &self,
        mode: MetaZwlrOutputPowerV1Mode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the failed message is available.
    #[allow(dead_code)]
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// object no longer valid
    ///
    /// This event indicates that the output power management mode control
    /// is no longer valid. This can happen for a number of reasons,
    /// including:
    /// - The output doesn't support power management
    /// - Another client already has exclusive power management mode control
    ///   for this output
    /// - The output disappeared
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    pub fn send_failed(
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

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this power management
    ///
    /// Destroys the output power management mode control object.
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
}

/// A message handler for [ZwlrOutputPowerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrOutputPowerV1MessageHandler {
    /// Set an outputs power save mode
    ///
    /// Set an output's power save mode to the given mode. The mode change
    /// is effective immediately. If the output does not support the given
    /// mode a failed event is sent.
    ///
    /// # Arguments
    ///
    /// - `mode`: the power save mode to set
    #[inline]
    fn set_mode(
        &mut self,
        _slf: &Rc<MetaZwlrOutputPowerV1>,
        mode: MetaZwlrOutputPowerV1Mode,
    ) {
        let res = _slf.send_set_mode(
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_power_v1.set_mode message: {}", Report::new(e));
        }
    }

    /// Report a power management mode change
    ///
    /// Report the power management mode change of an output.
    ///
    /// The mode event is sent after an output changed its power
    /// management mode. The reason can be a client using set_mode or the
    /// compositor deciding to change an output's mode.
    /// This event is also sent immediately when the object is created
    /// so the client is informed about the current power management mode.
    ///
    /// # Arguments
    ///
    /// - `mode`: the output's new power management mode
    #[inline]
    fn mode(
        &mut self,
        _slf: &Rc<MetaZwlrOutputPowerV1>,
        mode: MetaZwlrOutputPowerV1Mode,
    ) {
        let res = _slf.send_mode(
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_power_v1.mode message: {}", Report::new(e));
        }
    }

    /// object no longer valid
    ///
    /// This event indicates that the output power management mode control
    /// is no longer valid. This can happen for a number of reasons,
    /// including:
    /// - The output doesn't support power management
    /// - Another client already has exclusive power management mode control
    ///   for this output
    /// - The output disappeared
    ///
    /// Upon receiving this event, the client should destroy this object.
    #[inline]
    fn failed(
        &mut self,
        _slf: &Rc<MetaZwlrOutputPowerV1>,
    ) {
        let res = _slf.send_failed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_power_v1.failed message: {}", Report::new(e));
        }
    }

    /// destroy this power management
    ///
    /// Destroys the output power management mode control object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrOutputPowerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_power_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrOutputPowerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaZwlrOutputPowerV1Mode(arg0);
                if let Some(handler) = handler {
                    (**handler).set_mode(&self, arg0);
                } else {
                    DefaultMessageHandler.set_mode(&self, arg0);
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaZwlrOutputPowerV1Mode(arg0);
                if let Some(handler) = handler {
                    (**handler).mode(&self, arg0);
                } else {
                    DefaultMessageHandler.mode(&self, arg0);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).failed(&self);
                } else {
                    DefaultMessageHandler.failed(&self);
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

impl MetaZwlrOutputPowerV1 {
    /// Since when the mode.off enum variant is available.
    #[allow(dead_code)]
    pub const ENM__MODE_OFF__SINCE: u32 = 1;
    /// Since when the mode.on enum variant is available.
    #[allow(dead_code)]
    pub const ENM__MODE_ON__SINCE: u32 = 1;

    /// Since when the error.invalid_mode enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_MODE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrOutputPowerV1Mode(pub u32);

impl MetaZwlrOutputPowerV1Mode {
    /// Output is turned off.
    #[allow(dead_code)]
    pub const OFF: Self = Self(0);

    /// Output is turned on, no power saving
    #[allow(dead_code)]
    pub const ON: Self = Self(1);
}

impl Debug for MetaZwlrOutputPowerV1Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::OFF => "OFF",
            Self::ON => "ON",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrOutputPowerV1Error(pub u32);

impl MetaZwlrOutputPowerV1Error {
    /// nonexistent power save mode
    #[allow(dead_code)]
    pub const INVALID_MODE: Self = Self(1);
}

impl Debug for MetaZwlrOutputPowerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_MODE => "INVALID_MODE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
