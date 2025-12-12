//! head configuration
//!
//! This object is used by the client to update a single head's configuration.
//!
//! It is a protocol error to set the same property twice.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_configuration_head_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrOutputConfigurationHeadV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrOutputConfigurationHeadV1Handler>,
}

struct DefaultHandler;

impl ZwlrOutputConfigurationHeadV1Handler for DefaultHandler { }

impl ZwlrOutputConfigurationHeadV1 {
    pub const XML_VERSION: u32 = 4;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwlrOutputConfigurationHeadV1;
    pub const INTERFACE_NAME: &str = "zwlr_output_configuration_head_v1";
}

impl ZwlrOutputConfigurationHeadV1 {
    pub fn set_handler(&self, handler: impl ZwlrOutputConfigurationHeadV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrOutputConfigurationHeadV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrOutputConfigurationHeadV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrOutputConfigurationHeadV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrOutputConfigurationHeadV1 {
    /// Since when the set_mode message is available.
    pub const MSG__SET_MODE__SINCE: u32 = 1;

    /// set the mode
    ///
    /// This request sets the head's mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    #[inline]
    pub fn send_set_mode(
        &self,
        mode: &Rc<ZwlrOutputModeV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("mode")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_mode(mode: zwlr_output_mode_v1#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the set_custom_mode message is available.
    pub const MSG__SET_CUSTOM_MODE__SINCE: u32 = 1;

    /// set a custom mode
    ///
    /// This request assigns a custom mode to the head. The size is given in
    /// physical hardware units of the output device. If set to zero, the
    /// refresh rate is unspecified.
    ///
    /// It is a protocol error to set both a mode and a custom mode.
    ///
    /// # Arguments
    ///
    /// - `width`: width of the mode in hardware units
    /// - `height`: height of the mode in hardware units
    /// - `refresh`: vertical refresh rate in mHz or zero
    #[inline]
    pub fn send_set_custom_mode(
        &self,
        width: i32,
        height: i32,
        refresh: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            width,
            height,
            refresh,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_custom_mode(width: {}, height: {}, refresh: {})\n", id, arg0, arg1, arg2);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
        ]);
        Ok(())
    }

    /// Since when the set_position message is available.
    pub const MSG__SET_POSITION__SINCE: u32 = 1;

    /// set the position
    ///
    /// This request sets the head's position in the global compositor space.
    ///
    /// # Arguments
    ///
    /// - `x`: x position in the global compositor space
    /// - `y`: y position in the global compositor space
    #[inline]
    pub fn send_set_position(
        &self,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            x,
            y,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_position(x: {}, y: {})\n", id, arg0, arg1);
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
            2,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the set_transform message is available.
    pub const MSG__SET_TRANSFORM__SINCE: u32 = 1;

    /// set the transform
    ///
    /// This request sets the head's transform.
    ///
    /// # Arguments
    ///
    /// - `transform`:
    #[inline]
    pub fn send_set_transform(
        &self,
        transform: WlOutputTransform,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            transform,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_transform(transform: {:?})\n", id, arg0);
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
            3,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the set_scale message is available.
    pub const MSG__SET_SCALE__SINCE: u32 = 1;

    /// set the scale
    ///
    /// This request sets the head's scale.
    ///
    /// # Arguments
    ///
    /// - `scale`:
    #[inline]
    pub fn send_set_scale(
        &self,
        scale: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            scale,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_scale(scale: {})\n", id, arg0);
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
            4,
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the set_adaptive_sync message is available.
    pub const MSG__SET_ADAPTIVE_SYNC__SINCE: u32 = 4;

    /// enable/disable adaptive sync
    ///
    /// This request enables/disables adaptive sync. Adaptive sync is also
    /// known as Variable Refresh Rate or VRR.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_set_adaptive_sync(
        &self,
        state: ZwlrOutputHeadV1AdaptiveSyncState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_adaptive_sync(state: {:?})\n", id, arg0);
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
            5,
            arg0.0,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwlrOutputConfigurationHeadV1] proxies.
pub trait ZwlrOutputConfigurationHeadV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrOutputConfigurationHeadV1>) {
        let _ = slf.core.delete_id();
    }

    /// set the mode
    ///
    /// This request sets the head's mode.
    ///
    /// # Arguments
    ///
    /// - `mode`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_mode(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        mode: &Rc<ZwlrOutputModeV1>,
    ) {
        let res = _slf.send_set_mode(
            mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_head_v1.set_mode message: {}", Report::new(e));
        }
    }

    /// set a custom mode
    ///
    /// This request assigns a custom mode to the head. The size is given in
    /// physical hardware units of the output device. If set to zero, the
    /// refresh rate is unspecified.
    ///
    /// It is a protocol error to set both a mode and a custom mode.
    ///
    /// # Arguments
    ///
    /// - `width`: width of the mode in hardware units
    /// - `height`: height of the mode in hardware units
    /// - `refresh`: vertical refresh rate in mHz or zero
    #[inline]
    fn handle_set_custom_mode(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        width: i32,
        height: i32,
        refresh: i32,
    ) {
        let res = _slf.send_set_custom_mode(
            width,
            height,
            refresh,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_head_v1.set_custom_mode message: {}", Report::new(e));
        }
    }

    /// set the position
    ///
    /// This request sets the head's position in the global compositor space.
    ///
    /// # Arguments
    ///
    /// - `x`: x position in the global compositor space
    /// - `y`: y position in the global compositor space
    #[inline]
    fn handle_set_position(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        x: i32,
        y: i32,
    ) {
        let res = _slf.send_set_position(
            x,
            y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_head_v1.set_position message: {}", Report::new(e));
        }
    }

    /// set the transform
    ///
    /// This request sets the head's transform.
    ///
    /// # Arguments
    ///
    /// - `transform`:
    #[inline]
    fn handle_set_transform(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        transform: WlOutputTransform,
    ) {
        let res = _slf.send_set_transform(
            transform,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_head_v1.set_transform message: {}", Report::new(e));
        }
    }

    /// set the scale
    ///
    /// This request sets the head's scale.
    ///
    /// # Arguments
    ///
    /// - `scale`:
    #[inline]
    fn handle_set_scale(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        scale: Fixed,
    ) {
        let res = _slf.send_set_scale(
            scale,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_head_v1.set_scale message: {}", Report::new(e));
        }
    }

    /// enable/disable adaptive sync
    ///
    /// This request enables/disables adaptive sync. Adaptive sync is also
    /// known as Variable Refresh Rate or VRR.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_set_adaptive_sync(
        &mut self,
        _slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        state: ZwlrOutputHeadV1AdaptiveSyncState,
    ) {
        let res = _slf.send_set_adaptive_sync(
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_head_v1.set_adaptive_sync message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ZwlrOutputConfigurationHeadV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrOutputConfigurationHeadV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_mode(mode: zwlr_output_mode_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrOutputModeV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("mode", o.core().interface, ObjectInterface::ZwlrOutputModeV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_set_mode(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_custom_mode(width: {}, height: {}, refresh: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_custom_mode(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_set_custom_mode(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_position(x: {}, y: {})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_position(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = WlOutputTransform(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_transform(transform: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_transform(&self, arg0);
                } else {
                    DefaultHandler.handle_set_transform(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_scale(scale: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_scale(&self, arg0);
                } else {
                    DefaultHandler.handle_set_scale(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = ZwlrOutputHeadV1AdaptiveSyncState(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_adaptive_sync(state: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_adaptive_sync(&self, arg0);
                } else {
                    DefaultHandler.handle_set_adaptive_sync(&self, arg0);
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
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "set_mode",
            1 => "set_custom_mode",
            2 => "set_position",
            3 => "set_transform",
            4 => "set_scale",
            5 => "set_adaptive_sync",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZwlrOutputConfigurationHeadV1 {
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

impl ZwlrOutputConfigurationHeadV1 {
    /// Since when the error.already_set enum variant is available.
    pub const ENM__ERROR_ALREADY_SET__SINCE: u32 = 1;
    /// Since when the error.invalid_mode enum variant is available.
    pub const ENM__ERROR_INVALID_MODE__SINCE: u32 = 1;
    /// Since when the error.invalid_custom_mode enum variant is available.
    pub const ENM__ERROR_INVALID_CUSTOM_MODE__SINCE: u32 = 1;
    /// Since when the error.invalid_transform enum variant is available.
    pub const ENM__ERROR_INVALID_TRANSFORM__SINCE: u32 = 1;
    /// Since when the error.invalid_scale enum variant is available.
    pub const ENM__ERROR_INVALID_SCALE__SINCE: u32 = 1;
    /// Since when the error.invalid_adaptive_sync_state enum variant is available.
    pub const ENM__ERROR_INVALID_ADAPTIVE_SYNC_STATE__SINCE: u32 = 4;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrOutputConfigurationHeadV1Error(pub u32);

impl ZwlrOutputConfigurationHeadV1Error {
    /// property has already been set
    pub const ALREADY_SET: Self = Self(1);

    /// mode doesn't belong to head
    pub const INVALID_MODE: Self = Self(2);

    /// mode is invalid
    pub const INVALID_CUSTOM_MODE: Self = Self(3);

    /// transform value outside enum
    pub const INVALID_TRANSFORM: Self = Self(4);

    /// scale negative or zero
    pub const INVALID_SCALE: Self = Self(5);

    /// invalid enum value used in the set_adaptive_sync request
    pub const INVALID_ADAPTIVE_SYNC_STATE: Self = Self(6);
}

impl Debug for ZwlrOutputConfigurationHeadV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_SET => "ALREADY_SET",
            Self::INVALID_MODE => "INVALID_MODE",
            Self::INVALID_CUSTOM_MODE => "INVALID_CUSTOM_MODE",
            Self::INVALID_TRANSFORM => "INVALID_TRANSFORM",
            Self::INVALID_SCALE => "INVALID_SCALE",
            Self::INVALID_ADAPTIVE_SYNC_STATE => "INVALID_ADAPTIVE_SYNC_STATE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
