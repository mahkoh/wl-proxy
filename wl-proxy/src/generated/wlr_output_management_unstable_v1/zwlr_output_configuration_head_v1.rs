//! head configuration
//!
//! This object is used by the client to update a single head's configuration.
//!
//! It is a protocol error to set the same property twice.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_configuration_head_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrOutputConfigurationHeadV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrOutputConfigurationHeadV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrOutputConfigurationHeadV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrOutputConfigurationHeadV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrOutputConfigurationHeadV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrOutputConfigurationHeadV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrOutputConfigurationHeadV1 {
    /// Since when the set_mode message is available.
    #[allow(dead_code)]
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
        mode: &Rc<MetaZwlrOutputModeV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mode,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the set_custom_mode message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
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
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
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
    #[allow(dead_code)]
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
        transform: MetaWlOutputTransform,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            transform,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
            arg0.0 as u32,
        ]);
        Ok(())
    }

    /// Since when the set_scale message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            4,
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the set_adaptive_sync message is available.
    #[allow(dead_code)]
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
        state: MetaZwlrOutputHeadV1AdaptiveSyncState,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
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
#[allow(dead_code)]
pub trait MetaZwlrOutputConfigurationHeadV1MessageHandler {
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
    fn set_mode(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationHeadV1>,
        mode: &Rc<MetaZwlrOutputModeV1>,
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
    fn set_custom_mode(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationHeadV1>,
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
    fn set_position(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationHeadV1>,
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
    fn set_transform(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationHeadV1>,
        transform: MetaWlOutputTransform,
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
    fn set_scale(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationHeadV1>,
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
    fn set_adaptive_sync(
        &mut self,
        _slf: &Rc<MetaZwlrOutputConfigurationHeadV1>,
        state: MetaZwlrOutputHeadV1AdaptiveSyncState,
    ) {
        let res = _slf.send_set_adaptive_sync(
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_configuration_head_v1.set_adaptive_sync message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrOutputConfigurationHeadV1 {
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
                let Some(arg0) = client.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaZwlrOutputModeV1>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_mode(&self, arg0);
                } else {
                    DefaultMessageHandler.set_mode(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                if let Some(handler) = handler {
                    (**handler).set_custom_mode(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.set_custom_mode(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).set_position(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_position(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWlOutputTransform(arg0);
                if let Some(handler) = handler {
                    (**handler).set_transform(&self, arg0);
                } else {
                    DefaultMessageHandler.set_transform(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                if let Some(handler) = handler {
                    (**handler).set_scale(&self, arg0);
                } else {
                    DefaultMessageHandler.set_scale(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaZwlrOutputHeadV1AdaptiveSyncState(arg0);
                if let Some(handler) = handler {
                    (**handler).set_adaptive_sync(&self, arg0);
                } else {
                    DefaultMessageHandler.set_adaptive_sync(&self, arg0);
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
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

impl MetaZwlrOutputConfigurationHeadV1 {
    /// Since when the error.already_set enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_SET__SINCE: u32 = 1;
    /// Since when the error.invalid_mode enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_MODE__SINCE: u32 = 1;
    /// Since when the error.invalid_custom_mode enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_CUSTOM_MODE__SINCE: u32 = 1;
    /// Since when the error.invalid_transform enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_TRANSFORM__SINCE: u32 = 1;
    /// Since when the error.invalid_scale enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SCALE__SINCE: u32 = 1;
    /// Since when the error.invalid_adaptive_sync_state enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_ADAPTIVE_SYNC_STATE__SINCE: u32 = 4;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrOutputConfigurationHeadV1Error(pub u32);

impl MetaZwlrOutputConfigurationHeadV1Error {
    /// property has already been set
    #[allow(dead_code)]
    pub const ALREADY_SET: Self = Self(1);

    /// mode doesn't belong to head
    #[allow(dead_code)]
    pub const INVALID_MODE: Self = Self(2);

    /// mode is invalid
    #[allow(dead_code)]
    pub const INVALID_CUSTOM_MODE: Self = Self(3);

    /// transform value outside enum
    #[allow(dead_code)]
    pub const INVALID_TRANSFORM: Self = Self(4);

    /// scale negative or zero
    #[allow(dead_code)]
    pub const INVALID_SCALE: Self = Self(5);

    /// invalid enum value used in the set_adaptive_sync request
    #[allow(dead_code)]
    pub const INVALID_ADAPTIVE_SYNC_STATE: Self = Self(6);
}

impl Debug for MetaZwlrOutputConfigurationHeadV1Error {
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
