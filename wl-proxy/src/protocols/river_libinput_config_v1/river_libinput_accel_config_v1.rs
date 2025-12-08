//! acceleration config
//!
//! The result returned by libinput on setting configuration for a device.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_libinput_accel_config_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverLibinputAccelConfigV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverLibinputAccelConfigV1Handler>,
}

struct DefaultHandler;

impl RiverLibinputAccelConfigV1Handler for DefaultHandler { }

impl ConcreteObject for RiverLibinputAccelConfigV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverLibinputAccelConfigV1;
    const INTERFACE_NAME: &str = "river_libinput_accel_config_v1";
}

impl RiverLibinputAccelConfigV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverLibinputAccelConfigV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverLibinputAccelConfigV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverLibinputAccelConfigV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverLibinputAccelConfigV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverLibinputAccelConfigV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the accel object
    ///
    /// This request indicates that the client will no longer use the accel
    /// config object and that it may be safely destroyed.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_accel_config_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the accel object
    ///
    /// This request indicates that the client will no longer use the accel
    /// config object and that it may be safely destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_libinput_accel_config_v1.destroy", &e);
        }
    }

    /// Since when the set_points message is available.
    pub const MSG__SET_POINTS__SINCE: u32 = 1;

    /// define custom acceleration function
    ///
    /// Defines the acceleration function for a given movement type
    /// in an acceleration configuration with custom accel profile.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `r#type`:
    /// - `step`: double
    /// - `points`: array of doubles
    #[inline]
    pub fn try_send_set_points(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        r#type: RiverLibinputAccelConfigV1AccelType,
        step: &[u8],
        points: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            result,
            r#type,
            step,
            points,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("result", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: RiverLibinputAccelConfigV1AccelType, arg2: &[u8], arg3: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_libinput_accel_config_v1#{}.set_points(result: river_libinput_result_v1#{}, type: {:?}, step: {}, points: {})\n", id, arg0, arg1, debug_array(arg2), debug_array(arg3));
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2, arg3);
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
            arg0_id,
            arg1.0,
        ]);
        fmt.array(arg2);
        fmt.array(arg3);
        Ok(())
    }

    /// define custom acceleration function
    ///
    /// Defines the acceleration function for a given movement type
    /// in an acceleration configuration with custom accel profile.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `r#type`:
    /// - `step`: double
    /// - `points`: array of doubles
    #[inline]
    pub fn send_set_points(
        &self,
        result: &Rc<RiverLibinputResultV1>,
        r#type: RiverLibinputAccelConfigV1AccelType,
        step: &[u8],
        points: &[u8],
    ) {
        let res = self.try_send_set_points(
            result,
            r#type,
            step,
            points,
        );
        if let Err(e) = res {
            log_send("river_libinput_accel_config_v1.set_points", &e);
        }
    }

    /// define custom acceleration function
    ///
    /// Defines the acceleration function for a given movement type
    /// in an acceleration configuration with custom accel profile.
    ///
    /// # Arguments
    ///
    /// - `r#type`:
    /// - `step`: double
    /// - `points`: array of doubles
    #[inline]
    pub fn new_try_send_set_points(
        &self,
        r#type: RiverLibinputAccelConfigV1AccelType,
        step: &[u8],
        points: &[u8],
    ) -> Result<Rc<RiverLibinputResultV1>, ObjectError> {
        let result = self.core.create_child();
        self.try_send_set_points(
            &result,
            r#type,
            step,
            points,
        )?;
        Ok(result)
    }

    /// define custom acceleration function
    ///
    /// Defines the acceleration function for a given movement type
    /// in an acceleration configuration with custom accel profile.
    ///
    /// # Arguments
    ///
    /// - `r#type`:
    /// - `step`: double
    /// - `points`: array of doubles
    #[inline]
    pub fn new_send_set_points(
        &self,
        r#type: RiverLibinputAccelConfigV1AccelType,
        step: &[u8],
        points: &[u8],
    ) -> Rc<RiverLibinputResultV1> {
        let result = self.core.create_child();
        self.send_set_points(
            &result,
            r#type,
            step,
            points,
        );
        result
    }
}

/// A message handler for [`RiverLibinputAccelConfigV1`] proxies.
pub trait RiverLibinputAccelConfigV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverLibinputAccelConfigV1>) {
        slf.core.delete_id();
    }

    /// destroy the accel object
    ///
    /// This request indicates that the client will no longer use the accel
    /// config object and that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverLibinputAccelConfigV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_libinput_accel_config_v1.destroy", &e);
        }
    }

    /// define custom acceleration function
    ///
    /// Defines the acceleration function for a given movement type
    /// in an acceleration configuration with custom accel profile.
    ///
    /// # Arguments
    ///
    /// - `result`:
    /// - `r#type`:
    /// - `step`: double
    /// - `points`: array of doubles
    #[inline]
    fn handle_set_points(
        &mut self,
        slf: &Rc<RiverLibinputAccelConfigV1>,
        result: &Rc<RiverLibinputResultV1>,
        r#type: RiverLibinputAccelConfigV1AccelType,
        step: &[u8],
        points: &[u8],
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_points(
            result,
            r#type,
            step,
            points,
        );
        if let Err(e) = res {
            log_forward("river_libinput_accel_config_v1.set_points", &e);
        }
    }
}

impl ObjectPrivate for RiverLibinputAccelConfigV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverLibinputAccelConfigV1, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_accel_config_v1#{}.destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
            }
            1 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("result")));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("type")));
                };
                offset += 1;
                let arg2;
                (arg2, offset) = parse_array(msg, offset, "step")?;
                let arg3;
                (arg3, offset) = parse_array(msg, offset, "points")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                let arg1 = RiverLibinputAccelConfigV1AccelType(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverLibinputAccelConfigV1AccelType, arg2: &[u8], arg3: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_libinput_accel_config_v1#{}.set_points(result: river_libinput_result_v1#{}, type: {:?}, step: {}, points: {})\n", client_id, id, arg0, arg1, debug_array(arg2), debug_array(arg3));
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                let arg0_id = arg0;
                let arg0 = RiverLibinputResultV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "result", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_points(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_points(&self, arg0, arg1, arg2, arg3);
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
            0 => "destroy",
            1 => "set_points",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for RiverLibinputAccelConfigV1 {
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

impl RiverLibinputAccelConfigV1 {
    /// Since when the error.invalid_arg enum variant is available.
    pub const ENM__ERROR_INVALID_ARG__SINCE: u32 = 1;

    /// Since when the accel_type.fallback enum variant is available.
    pub const ENM__ACCEL_TYPE_FALLBACK__SINCE: u32 = 1;
    /// Since when the accel_type.motion enum variant is available.
    pub const ENM__ACCEL_TYPE_MOTION__SINCE: u32 = 1;
    /// Since when the accel_type.scroll enum variant is available.
    pub const ENM__ACCEL_TYPE_SCROLL__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputAccelConfigV1Error(pub u32);

impl RiverLibinputAccelConfigV1Error {
    /// invalid enum value or similar
    pub const INVALID_ARG: Self = Self(0);
}

impl Debug for RiverLibinputAccelConfigV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_ARG => "INVALID_ARG",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLibinputAccelConfigV1AccelType(pub u32);

impl RiverLibinputAccelConfigV1AccelType {
    pub const FALLBACK: Self = Self(0);

    pub const MOTION: Self = Self(1);

    pub const SCROLL: Self = Self(2);
}

impl Debug for RiverLibinputAccelConfigV1AccelType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::FALLBACK => "FALLBACK",
            Self::MOTION => "MOTION",
            Self::SCROLL => "SCROLL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
