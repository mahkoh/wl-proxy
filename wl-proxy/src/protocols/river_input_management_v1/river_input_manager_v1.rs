//! input manager global interface
//!
//! Input manager global interface.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_input_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverInputManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverInputManagerV1Handler>,
}

struct DefaultHandler;

impl RiverInputManagerV1Handler for DefaultHandler { }

impl ConcreteObject for RiverInputManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverInputManagerV1;
    const INTERFACE_NAME: &str = "river_input_manager_v1";
}

impl RiverInputManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverInputManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverInputManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverInputManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverInputManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverInputManagerV1 {
    /// Since when the stop message is available.
    pub const MSG__STOP__SINCE: u32 = 1;

    /// stop sending events
    ///
    /// This request indicates that the client no longer wishes to receive
    /// events on this object.
    ///
    /// The Wayland protocol is asynchronous, which means the server may send
    /// further events until the stop request is processed. The client must wait
    /// for a river_input_manager_v1.finished event before destroying this
    /// object.
    #[inline]
    pub fn try_send_stop(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_input_manager_v1#{}.stop()\n", id);
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
        Ok(())
    }

    /// stop sending events
    ///
    /// This request indicates that the client no longer wishes to receive
    /// events on this object.
    ///
    /// The Wayland protocol is asynchronous, which means the server may send
    /// further events until the stop request is processed. The client must wait
    /// for a river_input_manager_v1.finished event before destroying this
    /// object.
    #[inline]
    pub fn send_stop(
        &self,
    ) {
        let res = self.try_send_stop(
        );
        if let Err(e) = res {
            log_send("river_input_manager_v1.stop", &e);
        }
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the server has finished with the input manager
    ///
    /// This event indicates that the server will send no further events on this
    /// object. The client should destroy the object. See
    /// river_input_manager_v1.destroy for more information.
    #[inline]
    pub fn try_send_finished(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_input_manager_v1#{}.finished()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
        ]);
        Ok(())
    }

    /// the server has finished with the input manager
    ///
    /// This event indicates that the server will send no further events on this
    /// object. The client should destroy the object. See
    /// river_input_manager_v1.destroy for more information.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("river_input_manager_v1.finished", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the river_input_manager_v1 object
    ///
    /// This request should be called after the finished event has been received
    /// to complete destruction of the object.
    ///
    /// It is a protocol error to make this request before the finished event
    /// has been received.
    ///
    /// If a client wishes to destroy this object it should send a
    /// river_input_manager_v1.stop request and wait for a
    /// river_input_manager_v1.finished event. Once the finished event is
    /// received it is safe to destroy this object and any other objects created
    /// through this interface.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_input_manager_v1#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the river_input_manager_v1 object
    ///
    /// This request should be called after the finished event has been received
    /// to complete destruction of the object.
    ///
    /// It is a protocol error to make this request before the finished event
    /// has been received.
    ///
    /// If a client wishes to destroy this object it should send a
    /// river_input_manager_v1.stop request and wait for a
    /// river_input_manager_v1.finished event. Once the finished event is
    /// received it is safe to destroy this object and any other objects created
    /// through this interface.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_input_manager_v1.destroy", &e);
        }
    }

    /// Since when the create_seat message is available.
    pub const MSG__CREATE_SEAT__SINCE: u32 = 1;

    /// create a new seat
    ///
    /// Create a new seat with the given name. Has no effect if a seat with the
    /// given name already exists.
    ///
    /// The default seat with name "default" always exists and does not need to
    /// be explicitly created.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn try_send_create_seat(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_input_manager_v1#{}.create_seat(name: {:?})\n", id, arg0);
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
            2,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// create a new seat
    ///
    /// Create a new seat with the given name. Has no effect if a seat with the
    /// given name already exists.
    ///
    /// The default seat with name "default" always exists and does not need to
    /// be explicitly created.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn send_create_seat(
        &self,
        name: &str,
    ) {
        let res = self.try_send_create_seat(
            name,
        );
        if let Err(e) = res {
            log_send("river_input_manager_v1.create_seat", &e);
        }
    }

    /// Since when the destroy_seat message is available.
    pub const MSG__DESTROY_SEAT__SINCE: u32 = 1;

    /// destroy a seat
    ///
    /// Destroy the seat with the given name. Has no effect if a seat with the
    /// given name does not exist.
    ///
    /// The default seat with name "default" cannot be destroyed and attempting
    /// to destroy it will have no effect.
    ///
    /// Any input devices assigned to the destroyed seat at the time of
    /// destruction are assigned to the default seat.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn try_send_destroy_seat(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_input_manager_v1#{}.destroy_seat(name: {:?})\n", id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// destroy a seat
    ///
    /// Destroy the seat with the given name. Has no effect if a seat with the
    /// given name does not exist.
    ///
    /// The default seat with name "default" cannot be destroyed and attempting
    /// to destroy it will have no effect.
    ///
    /// Any input devices assigned to the destroyed seat at the time of
    /// destruction are assigned to the default seat.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn send_destroy_seat(
        &self,
        name: &str,
    ) {
        let res = self.try_send_destroy_seat(
            name,
        );
        if let Err(e) = res {
            log_send("river_input_manager_v1.destroy_seat", &e);
        }
    }

    /// Since when the input_device message is available.
    pub const MSG__INPUT_DEVICE__SINCE: u32 = 1;

    /// new input device
    ///
    /// A new input device has been created.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_input_device(
        &self,
        id: &Rc<RiverInputDeviceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        arg0.generate_client_id(client, arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("id", e)))?;
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_input_manager_v1#{}.input_device(id: river_input_device_v1#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// new input device
    ///
    /// A new input device has been created.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_input_device(
        &self,
        id: &Rc<RiverInputDeviceV1>,
    ) {
        let res = self.try_send_input_device(
            id,
        );
        if let Err(e) = res {
            log_send("river_input_manager_v1.input_device", &e);
        }
    }

    /// new input device
    ///
    /// A new input device has been created.
    #[inline]
    pub fn new_try_send_input_device(
        &self,
    ) -> Result<Rc<RiverInputDeviceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_input_device(
            &id,
        )?;
        Ok(id)
    }

    /// new input device
    ///
    /// A new input device has been created.
    #[inline]
    pub fn new_send_input_device(
        &self,
    ) -> Rc<RiverInputDeviceV1> {
        let id = self.core.create_child();
        self.send_input_device(
            &id,
        );
        id
    }
}

/// A message handler for [`RiverInputManagerV1`] proxies.
pub trait RiverInputManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverInputManagerV1>) {
        slf.core.delete_id();
    }

    /// stop sending events
    ///
    /// This request indicates that the client no longer wishes to receive
    /// events on this object.
    ///
    /// The Wayland protocol is asynchronous, which means the server may send
    /// further events until the stop request is processed. The client must wait
    /// for a river_input_manager_v1.finished event before destroying this
    /// object.
    #[inline]
    fn handle_stop(
        &mut self,
        slf: &Rc<RiverInputManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_stop(
        );
        if let Err(e) = res {
            log_forward("river_input_manager_v1.stop", &e);
        }
    }

    /// the server has finished with the input manager
    ///
    /// This event indicates that the server will send no further events on this
    /// object. The client should destroy the object. See
    /// river_input_manager_v1.destroy for more information.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<RiverInputManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("river_input_manager_v1.finished", &e);
        }
    }

    /// destroy the river_input_manager_v1 object
    ///
    /// This request should be called after the finished event has been received
    /// to complete destruction of the object.
    ///
    /// It is a protocol error to make this request before the finished event
    /// has been received.
    ///
    /// If a client wishes to destroy this object it should send a
    /// river_input_manager_v1.stop request and wait for a
    /// river_input_manager_v1.finished event. Once the finished event is
    /// received it is safe to destroy this object and any other objects created
    /// through this interface.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverInputManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_input_manager_v1.destroy", &e);
        }
    }

    /// create a new seat
    ///
    /// Create a new seat with the given name. Has no effect if a seat with the
    /// given name already exists.
    ///
    /// The default seat with name "default" always exists and does not need to
    /// be explicitly created.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    fn handle_create_seat(
        &mut self,
        slf: &Rc<RiverInputManagerV1>,
        name: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_seat(
            name,
        );
        if let Err(e) = res {
            log_forward("river_input_manager_v1.create_seat", &e);
        }
    }

    /// destroy a seat
    ///
    /// Destroy the seat with the given name. Has no effect if a seat with the
    /// given name does not exist.
    ///
    /// The default seat with name "default" cannot be destroyed and attempting
    /// to destroy it will have no effect.
    ///
    /// Any input devices assigned to the destroyed seat at the time of
    /// destruction are assigned to the default seat.
    ///
    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    fn handle_destroy_seat(
        &mut self,
        slf: &Rc<RiverInputManagerV1>,
        name: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy_seat(
            name,
        );
        if let Err(e) = res {
            log_forward("river_input_manager_v1.destroy_seat", &e);
        }
    }

    /// new input device
    ///
    /// A new input device has been created.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_input_device(
        &mut self,
        slf: &Rc<RiverInputManagerV1>,
        id: &Rc<RiverInputDeviceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_input_device(
            id,
        );
        if let Err(e) = res {
            log_forward("river_input_manager_v1.input_device", &e);
        }
    }
}

impl ObjectPrivate for RiverInputManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverInputManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_input_manager_v1#{}.stop()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_stop(&self);
                } else {
                    DefaultHandler.handle_stop(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_input_manager_v1#{}.destroy()\n", client_id, id);
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
            2 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_input_manager_v1#{}.create_seat(name: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_create_seat(&self, arg0);
                } else {
                    DefaultHandler.handle_create_seat(&self, arg0);
                }
            }
            3 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_input_manager_v1#{}.destroy_seat(name: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_destroy_seat(&self, arg0);
                } else {
                    DefaultHandler.handle_destroy_seat(&self, arg0);
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
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_input_manager_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_input_manager_v1#{}.input_device(id: river_input_device_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = RiverInputDeviceV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_input_device(&self, arg0);
                } else {
                    DefaultHandler.handle_input_device(&self, arg0);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "stop",
            1 => "destroy",
            2 => "create_seat",
            3 => "destroy_seat",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "finished",
            1 => "input_device",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverInputManagerV1 {
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

impl RiverInputManagerV1 {
    /// Since when the error.invalid_destroy enum variant is available.
    pub const ENM__ERROR_INVALID_DESTROY__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverInputManagerV1Error(pub u32);

impl RiverInputManagerV1Error {
    pub const INVALID_DESTROY: Self = Self(0);
}

impl Debug for RiverInputManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_DESTROY => "INVALID_DESTROY",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
