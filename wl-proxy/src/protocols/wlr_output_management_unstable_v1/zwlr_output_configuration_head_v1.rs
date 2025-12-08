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

impl ConcreteObject for ZwlrOutputConfigurationHeadV1 {
    const XML_VERSION: u32 = 4;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrOutputConfigurationHeadV1;
    const INTERFACE_NAME: &str = "zwlr_output_configuration_head_v1";
}

impl ZwlrOutputConfigurationHeadV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrOutputConfigurationHeadV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
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
    pub fn try_send_set_mode(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("mode"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_mode(mode: zwlr_output_mode_v1#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
    ) {
        let res = self.try_send_set_mode(
            mode,
        );
        if let Err(e) = res {
            log_send("zwlr_output_configuration_head_v1.set_mode", &e);
        }
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
    pub fn try_send_set_custom_mode(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_custom_mode(width: {}, height: {}, refresh: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
    ) {
        let res = self.try_send_set_custom_mode(
            width,
            height,
            refresh,
        );
        if let Err(e) = res {
            log_send("zwlr_output_configuration_head_v1.set_custom_mode", &e);
        }
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
    pub fn try_send_set_position(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_position(x: {}, y: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
    ) {
        let res = self.try_send_set_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("zwlr_output_configuration_head_v1.set_position", &e);
        }
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
    pub fn try_send_set_transform(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WlOutputTransform) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_transform(transform: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
    ) {
        let res = self.try_send_set_transform(
            transform,
        );
        if let Err(e) = res {
            log_send("zwlr_output_configuration_head_v1.set_transform", &e);
        }
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
    pub fn try_send_set_scale(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_scale(scale: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
    ) {
        let res = self.try_send_set_scale(
            scale,
        );
        if let Err(e) = res {
            log_send("zwlr_output_configuration_head_v1.set_scale", &e);
        }
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
    pub fn try_send_set_adaptive_sync(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: ZwlrOutputHeadV1AdaptiveSyncState) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_output_configuration_head_v1#{}.set_adaptive_sync(state: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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
    ) {
        let res = self.try_send_set_adaptive_sync(
            state,
        );
        if let Err(e) = res {
            log_send("zwlr_output_configuration_head_v1.set_adaptive_sync", &e);
        }
    }
}

/// A message handler for [`ZwlrOutputConfigurationHeadV1`] proxies.
pub trait ZwlrOutputConfigurationHeadV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrOutputConfigurationHeadV1>) {
        slf.core.delete_id();
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
        slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        mode: &Rc<ZwlrOutputModeV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_mode(
            mode,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_configuration_head_v1.set_mode", &e);
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
        slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        width: i32,
        height: i32,
        refresh: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_custom_mode(
            width,
            height,
            refresh,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_configuration_head_v1.set_custom_mode", &e);
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
        slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_configuration_head_v1.set_position", &e);
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
        slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        transform: WlOutputTransform,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_transform(
            transform,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_configuration_head_v1.set_transform", &e);
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
        slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        scale: Fixed,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_scale(
            scale,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_configuration_head_v1.set_scale", &e);
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
        slf: &Rc<ZwlrOutputConfigurationHeadV1>,
        state: ZwlrOutputHeadV1AdaptiveSyncState,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_adaptive_sync(
            state,
        );
        if let Err(e) = res {
            log_forward("zwlr_output_configuration_head_v1.set_adaptive_sync", &e);
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
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_mode(mode: zwlr_output_mode_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrOutputModeV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("mode", o.core().interface, ObjectInterface::ZwlrOutputModeV1)));
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_custom_mode(width: {}, height: {}, refresh: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_position(x: {}, y: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WlOutputTransform(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WlOutputTransform) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_transform(transform: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_scale(scale: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = ZwlrOutputHeadV1AdaptiveSyncState(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: ZwlrOutputHeadV1AdaptiveSyncState) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_output_configuration_head_v1#{}.set_adaptive_sync(state: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
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
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
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
