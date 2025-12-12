//! adjust power management mode for an output
//!
//! This object offers requests to set the power management mode of
//! an output.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_power_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrOutputPowerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrOutputPowerV1Handler>,
}

struct DefaultHandler;

impl ZwlrOutputPowerV1Handler for DefaultHandler { }

impl ZwlrOutputPowerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwlrOutputPowerV1;
    pub const INTERFACE_NAME: &str = "zwlr_output_power_v1";
}

impl ZwlrOutputPowerV1 {
    pub fn set_handler(&self, handler: impl ZwlrOutputPowerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrOutputPowerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrOutputPowerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrOutputPowerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrOutputPowerV1 {
    /// Since when the set_mode message is available.
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
        mode: ZwlrOutputPowerV1Mode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_power_v1#{}.set_mode(mode: {:?})\n", id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the mode message is available.
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
        mode: ZwlrOutputPowerV1Mode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_power_v1#{}.mode(mode: {:?})\n", client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the failed message is available.
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_output_power_v1#{}.failed()\n", client.endpoint.id, id);
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
            1,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_power_v1#{}.destroy()\n", id);
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
}

/// A message handler for [ZwlrOutputPowerV1] proxies.
pub trait ZwlrOutputPowerV1Handler: Any {
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
        _slf: &Rc<ZwlrOutputPowerV1>,
        mode: ZwlrOutputPowerV1Mode,
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
        _slf: &Rc<ZwlrOutputPowerV1>,
        mode: ZwlrOutputPowerV1Mode,
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
        _slf: &Rc<ZwlrOutputPowerV1>,
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
        _slf: &Rc<ZwlrOutputPowerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_power_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ZwlrOutputPowerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrOutputPowerV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = ZwlrOutputPowerV1Mode(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_power_v1#{}.set_mode(mode: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_mode(&self, arg0);
                } else {
                    DefaultHandler.set_mode(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_power_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = ZwlrOutputPowerV1Mode(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_power_v1#{}.mode(mode: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).mode(&self, arg0);
                } else {
                    DefaultHandler.mode(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_output_power_v1#{}.failed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).failed(&self);
                } else {
                    DefaultHandler.failed(&self);
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
            0 => "set_mode",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "mode",
            1 => "failed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwlrOutputPowerV1 {
    fn core(&self) -> &ObjectCore {
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

impl ZwlrOutputPowerV1 {
    /// Since when the mode.off enum variant is available.
    pub const ENM__MODE_OFF__SINCE: u32 = 1;
    /// Since when the mode.on enum variant is available.
    pub const ENM__MODE_ON__SINCE: u32 = 1;

    /// Since when the error.invalid_mode enum variant is available.
    pub const ENM__ERROR_INVALID_MODE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrOutputPowerV1Mode(pub u32);

impl ZwlrOutputPowerV1Mode {
    /// Output is turned off.
    pub const OFF: Self = Self(0);

    /// Output is turned on, no power saving
    pub const ON: Self = Self(1);
}

impl Debug for ZwlrOutputPowerV1Mode {
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
pub struct ZwlrOutputPowerV1Error(pub u32);

impl ZwlrOutputPowerV1Error {
    /// nonexistent power save mode
    pub const INVALID_MODE: Self = Self(1);
}

impl Debug for ZwlrOutputPowerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_MODE => "INVALID_MODE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
