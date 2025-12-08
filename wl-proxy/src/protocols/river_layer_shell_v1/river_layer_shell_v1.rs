//! river layer shell global interface
//!
//! This global interface should only be advertised to the client if the
//! river_window_manager_v1 global is also advertised. Binding this interface
//! indicates that the window manager supports layer shell.
//!
//! If the window manager does not bind this interface, the compositor should
//! not allow clients to map layer surfaces. This can be achieved by
//! closing layer surfaces immediately.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_layer_shell_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverLayerShellV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverLayerShellV1Handler>,
}

struct DefaultHandler;

impl RiverLayerShellV1Handler for DefaultHandler { }

impl ConcreteObject for RiverLayerShellV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverLayerShellV1;
    const INTERFACE_NAME: &str = "river_layer_shell_v1";
}

impl RiverLayerShellV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverLayerShellV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverLayerShellV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverLayerShellV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverLayerShellV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverLayerShellV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the river_layer_shell_v1 object
    ///
    /// This request indicates that the client will no longer use the
    /// river_layer_shell_v1 object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_layer_shell_v1#{}.destroy()\n", id);
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

    /// destroy the river_layer_shell_v1 object
    ///
    /// This request indicates that the client will no longer use the
    /// river_layer_shell_v1 object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_layer_shell_v1.destroy", &e);
        }
    }

    /// Since when the get_output message is available.
    pub const MSG__GET_OUTPUT__SINCE: u32 = 1;

    /// get layer shell output state
    ///
    /// It is a protocol error to make this request more than once for a given
    /// river_output_v1 object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    #[inline]
    pub fn try_send_get_output(
        &self,
        id: &Rc<RiverLayerShellOutputV1>,
        output: &Rc<RiverOutputV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            output,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_layer_shell_v1#{}.get_output(id: river_layer_shell_output_v1#{}, output: river_output_v1#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg1_id,
        ]);
        Ok(())
    }

    /// get layer shell output state
    ///
    /// It is a protocol error to make this request more than once for a given
    /// river_output_v1 object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    #[inline]
    pub fn send_get_output(
        &self,
        id: &Rc<RiverLayerShellOutputV1>,
        output: &Rc<RiverOutputV1>,
    ) {
        let res = self.try_send_get_output(
            id,
            output,
        );
        if let Err(e) = res {
            log_send("river_layer_shell_v1.get_output", &e);
        }
    }

    /// get layer shell output state
    ///
    /// It is a protocol error to make this request more than once for a given
    /// river_output_v1 object.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn new_try_send_get_output(
        &self,
        output: &Rc<RiverOutputV1>,
    ) -> Result<Rc<RiverLayerShellOutputV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_output(
            &id,
            output,
        )?;
        Ok(id)
    }

    /// get layer shell output state
    ///
    /// It is a protocol error to make this request more than once for a given
    /// river_output_v1 object.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn new_send_get_output(
        &self,
        output: &Rc<RiverOutputV1>,
    ) -> Rc<RiverLayerShellOutputV1> {
        let id = self.core.create_child();
        self.send_get_output(
            &id,
            output,
        );
        id
    }

    /// Since when the get_seat message is available.
    pub const MSG__GET_SEAT__SINCE: u32 = 1;

    /// get layer shell seat state
    ///
    /// It is a protocol error to make this request more than once for a given
    /// river_seat_v1 object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`:
    #[inline]
    pub fn try_send_get_seat(
        &self,
        id: &Rc<RiverLayerShellSeatV1>,
        seat: &Rc<RiverSeatV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_layer_shell_v1#{}.get_seat(id: river_layer_shell_seat_v1#{}, seat: river_seat_v1#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// get layer shell seat state
    ///
    /// It is a protocol error to make this request more than once for a given
    /// river_seat_v1 object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`:
    #[inline]
    pub fn send_get_seat(
        &self,
        id: &Rc<RiverLayerShellSeatV1>,
        seat: &Rc<RiverSeatV1>,
    ) {
        let res = self.try_send_get_seat(
            id,
            seat,
        );
        if let Err(e) = res {
            log_send("river_layer_shell_v1.get_seat", &e);
        }
    }

    /// get layer shell seat state
    ///
    /// It is a protocol error to make this request more than once for a given
    /// river_seat_v1 object.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn new_try_send_get_seat(
        &self,
        seat: &Rc<RiverSeatV1>,
    ) -> Result<Rc<RiverLayerShellSeatV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_seat(
            &id,
            seat,
        )?;
        Ok(id)
    }

    /// get layer shell seat state
    ///
    /// It is a protocol error to make this request more than once for a given
    /// river_seat_v1 object.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn new_send_get_seat(
        &self,
        seat: &Rc<RiverSeatV1>,
    ) -> Rc<RiverLayerShellSeatV1> {
        let id = self.core.create_child();
        self.send_get_seat(
            &id,
            seat,
        );
        id
    }
}

/// A message handler for [`RiverLayerShellV1`] proxies.
pub trait RiverLayerShellV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverLayerShellV1>) {
        slf.core.delete_id();
    }

    /// destroy the river_layer_shell_v1 object
    ///
    /// This request indicates that the client will no longer use the
    /// river_layer_shell_v1 object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverLayerShellV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_layer_shell_v1.destroy", &e);
        }
    }

    /// get layer shell output state
    ///
    /// It is a protocol error to make this request more than once for a given
    /// river_output_v1 object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_output(
        &mut self,
        slf: &Rc<RiverLayerShellV1>,
        id: &Rc<RiverLayerShellOutputV1>,
        output: &Rc<RiverOutputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_output(
            id,
            output,
        );
        if let Err(e) = res {
            log_forward("river_layer_shell_v1.get_output", &e);
        }
    }

    /// get layer shell seat state
    ///
    /// It is a protocol error to make this request more than once for a given
    /// river_seat_v1 object.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_seat(
        &mut self,
        slf: &Rc<RiverLayerShellV1>,
        id: &Rc<RiverLayerShellSeatV1>,
        seat: &Rc<RiverSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_seat(
            id,
            seat,
        );
        if let Err(e) = res {
            log_forward("river_layer_shell_v1.get_seat", &e);
        }
    }
}

impl ObjectPrivate for RiverLayerShellV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverLayerShellV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_layer_shell_v1#{}.destroy()\n", client_id, id);
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_layer_shell_v1#{}.get_output(id: river_layer_shell_output_v1#{}, output: river_output_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLayerShellOutputV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<RiverOutputV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::RiverOutputV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_output(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_output(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_layer_shell_v1#{}.get_seat(id: river_layer_shell_seat_v1#{}, seat: river_seat_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverLayerShellSeatV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<RiverSeatV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::RiverSeatV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_seat(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_seat(&self, arg0, arg1);
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
            1 => "get_output",
            2 => "get_seat",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for RiverLayerShellV1 {
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

impl RiverLayerShellV1 {
    /// Since when the error.object_already_created enum variant is available.
    pub const ENM__ERROR_OBJECT_ALREADY_CREATED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverLayerShellV1Error(pub u32);

impl RiverLayerShellV1Error {
    /// the layer_shell_output/seat object was already created.
    pub const OBJECT_ALREADY_CREATED: Self = Self(0);
}

impl Debug for RiverLayerShellV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::OBJECT_ALREADY_CREATED => "OBJECT_ALREADY_CREATED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
