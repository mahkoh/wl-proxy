//! Color control interface for output.
//!
//! Protocol for controlling color temperature and brightness settings of a specific output.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_output_color_control_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandOutputColorControlV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandOutputColorControlV1Handler>,
}

struct DefaultHandler;

impl TreelandOutputColorControlV1Handler for DefaultHandler { }

impl TreelandOutputColorControlV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::TreelandOutputColorControlV1;
    pub const INTERFACE_NAME: &str = "treeland_output_color_control_v1";
}

impl TreelandOutputColorControlV1 {
    pub fn set_handler(&self, handler: impl TreelandOutputColorControlV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandOutputColorControlV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandOutputColorControlV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandOutputColorControlV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandOutputColorControlV1 {
    /// Since when the set_color_temperature message is available.
    pub const MSG__SET_COLOR_TEMPERATURE__SINCE: u32 = 1;

    /// Set color temperature for output
    ///
    /// Color temperature settings are applied only after a commit request is made.
    /// Setting a value outside the range [1000, 20000] is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `temperature`: color temperature in Kelvin
    #[inline]
    pub fn send_set_color_temperature(
        &self,
        temperature: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            temperature,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_output_color_control_v1#{}.set_color_temperature(temperature: {})\n", id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// Since when the set_brightness message is available.
    pub const MSG__SET_BRIGHTNESS__SINCE: u32 = 1;

    /// Set brightness for output.
    ///
    /// Brightness settings are applied only after a commit request is made.
    /// Setting a value outside the range [0.0, 100.0] is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `brightness`: brightness level (in range [0.0, 100.0])
    #[inline]
    pub fn send_set_brightness(
        &self,
        brightness: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            brightness,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_output_color_control_v1#{}.set_brightness(brightness: {})\n", id, arg0);
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
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// Commit the pending color settings changes for output.
    #[inline]
    pub fn send_commit(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_output_color_control_v1#{}.commit()\n", id);
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
        ]);
        Ok(())
    }

    /// Since when the result message is available.
    pub const MSG__RESULT__SINCE: u32 = 1;

    /// The result of the last commit request.
    ///
    /// # Arguments
    ///
    /// - `success`: 1 if the commit was successful, 0 otherwise.
    #[inline]
    pub fn send_result(
        &self,
        success: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            success,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_output_color_control_v1#{}.result(success: {})\n", client.endpoint.id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// Since when the color_temperature message is available.
    pub const MSG__COLOR_TEMPERATURE__SINCE: u32 = 1;

    /// Current color temperature for output.
    ///
    /// Provides the current color temperature setting of the output.
    /// Color temperature is valued in the range [1000, 20000].
    /// Color temperature is defined as the corresponding temperature (in Kelvin) of the current white point
    /// of the display on a Planckian locus.
    /// With the current implementation, the neutral temperature is 6600K.
    /// This event is sent once after the treeland_output_color_control_v1 object is created,
    /// or right after when a color temperature change for the output is successfully commited.
    ///
    /// # Arguments
    ///
    /// - `temperature`: current color temperature in Kelvin
    #[inline]
    pub fn send_color_temperature(
        &self,
        temperature: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            temperature,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_output_color_control_v1#{}.color_temperature(temperature: {})\n", client.endpoint.id, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// Since when the brightness message is available.
    pub const MSG__BRIGHTNESS__SINCE: u32 = 1;

    /// Current brightness for output.
    ///
    /// Provides the current brightness setting of the output.
    /// Brightness is valued in the range [0.0, 100.0].
    /// This event is sent once after the treeland_output_color_control_v1 object is created,
    /// or right after when a brightness change for the output is successfully commited.
    ///
    /// # Arguments
    ///
    /// - `brightness`: current brightness level (in range [0.0, 100.0])
    #[inline]
    pub fn send_brightness(
        &self,
        brightness: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            brightness,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_output_color_control_v1#{}.brightness(brightness: {})\n", client.endpoint.id, id, arg0);
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
            2,
            arg0.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// Destroy the color control interface.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_output_color_control_v1#{}.destroy()\n", id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [TreelandOutputColorControlV1] proxies.
pub trait TreelandOutputColorControlV1Handler: Any {
    /// Set color temperature for output
    ///
    /// Color temperature settings are applied only after a commit request is made.
    /// Setting a value outside the range [1000, 20000] is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `temperature`: color temperature in Kelvin
    #[inline]
    fn set_color_temperature(
        &mut self,
        _slf: &Rc<TreelandOutputColorControlV1>,
        temperature: u32,
    ) {
        let res = _slf.send_set_color_temperature(
            temperature,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_output_color_control_v1.set_color_temperature message: {}", Report::new(e));
        }
    }

    /// Set brightness for output.
    ///
    /// Brightness settings are applied only after a commit request is made.
    /// Setting a value outside the range [0.0, 100.0] is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `brightness`: brightness level (in range [0.0, 100.0])
    #[inline]
    fn set_brightness(
        &mut self,
        _slf: &Rc<TreelandOutputColorControlV1>,
        brightness: Fixed,
    ) {
        let res = _slf.send_set_brightness(
            brightness,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_output_color_control_v1.set_brightness message: {}", Report::new(e));
        }
    }

    /// Commit the pending color settings changes for output.
    #[inline]
    fn commit(
        &mut self,
        _slf: &Rc<TreelandOutputColorControlV1>,
    ) {
        let res = _slf.send_commit(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_output_color_control_v1.commit message: {}", Report::new(e));
        }
    }

    /// The result of the last commit request.
    ///
    /// # Arguments
    ///
    /// - `success`: 1 if the commit was successful, 0 otherwise.
    #[inline]
    fn result(
        &mut self,
        _slf: &Rc<TreelandOutputColorControlV1>,
        success: u32,
    ) {
        let res = _slf.send_result(
            success,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_output_color_control_v1.result message: {}", Report::new(e));
        }
    }

    /// Current color temperature for output.
    ///
    /// Provides the current color temperature setting of the output.
    /// Color temperature is valued in the range [1000, 20000].
    /// Color temperature is defined as the corresponding temperature (in Kelvin) of the current white point
    /// of the display on a Planckian locus.
    /// With the current implementation, the neutral temperature is 6600K.
    /// This event is sent once after the treeland_output_color_control_v1 object is created,
    /// or right after when a color temperature change for the output is successfully commited.
    ///
    /// # Arguments
    ///
    /// - `temperature`: current color temperature in Kelvin
    #[inline]
    fn color_temperature(
        &mut self,
        _slf: &Rc<TreelandOutputColorControlV1>,
        temperature: u32,
    ) {
        let res = _slf.send_color_temperature(
            temperature,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_output_color_control_v1.color_temperature message: {}", Report::new(e));
        }
    }

    /// Current brightness for output.
    ///
    /// Provides the current brightness setting of the output.
    /// Brightness is valued in the range [0.0, 100.0].
    /// This event is sent once after the treeland_output_color_control_v1 object is created,
    /// or right after when a brightness change for the output is successfully commited.
    ///
    /// # Arguments
    ///
    /// - `brightness`: current brightness level (in range [0.0, 100.0])
    #[inline]
    fn brightness(
        &mut self,
        _slf: &Rc<TreelandOutputColorControlV1>,
        brightness: Fixed,
    ) {
        let res = _slf.send_brightness(
            brightness,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_output_color_control_v1.brightness message: {}", Report::new(e));
        }
    }

    /// Destroy the color control interface.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandOutputColorControlV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_output_color_control_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandOutputColorControlV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandOutputColorControlV1, version),
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_output_color_control_v1#{}.set_color_temperature(temperature: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_color_temperature(&self, arg0);
                } else {
                    DefaultHandler.set_color_temperature(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_output_color_control_v1#{}.set_brightness(brightness: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_brightness(&self, arg0);
                } else {
                    DefaultHandler.set_brightness(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_output_color_control_v1#{}.commit()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).commit(&self);
                } else {
                    DefaultHandler.commit(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_output_color_control_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_output_color_control_v1#{}.result(success: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).result(&self, arg0);
                } else {
                    DefaultHandler.result(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_output_color_control_v1#{}.color_temperature(temperature: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).color_temperature(&self, arg0);
                } else {
                    DefaultHandler.color_temperature(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_output_color_control_v1#{}.brightness(brightness: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).brightness(&self, arg0);
                } else {
                    DefaultHandler.brightness(&self, arg0);
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
            0 => "set_color_temperature",
            1 => "set_brightness",
            2 => "commit",
            3 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "result",
            1 => "color_temperature",
            2 => "brightness",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for TreelandOutputColorControlV1 {
    fn core(&self) -> &ProxyCore {
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

impl TreelandOutputColorControlV1 {
    /// Since when the error.invalid_color_temperature enum variant is available.
    pub const ENM__ERROR_INVALID_COLOR_TEMPERATURE__SINCE: u32 = 1;
    /// Since when the error.invalid_brightness enum variant is available.
    pub const ENM__ERROR_INVALID_BRIGHTNESS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandOutputColorControlV1Error(pub u32);

impl TreelandOutputColorControlV1Error {
    /// Invalid color temperature value provided.
    pub const INVALID_COLOR_TEMPERATURE: Self = Self(1);

    /// Invalid brightness value provided.
    pub const INVALID_BRIGHTNESS: Self = Self(2);
}

impl Debug for TreelandOutputColorControlV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_COLOR_TEMPERATURE => "INVALID_COLOR_TEMPERATURE",
            Self::INVALID_BRIGHTNESS => "INVALID_BRIGHTNESS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
