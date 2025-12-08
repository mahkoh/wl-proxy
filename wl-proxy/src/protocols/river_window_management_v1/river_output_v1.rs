//! a logical output
//!
//! An area in the compositor's logical coordinate space that should be
//! treated as a single output for window management purposes. This area may
//! correspond to a single physical output or multiple physical outputs in the
//! case of mirroring or tiled monitors depending on the hardware and
//! compositor configuration.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_output_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverOutputV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverOutputV1Handler>,
}

struct DefaultHandler;

impl RiverOutputV1Handler for DefaultHandler { }

impl ConcreteObject for RiverOutputV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverOutputV1;
    const INTERFACE_NAME: &str = "river_output_v1";
}

impl RiverOutputV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverOutputV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverOutputV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverOutputV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverOutputV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverOutputV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the output object
    ///
    /// This request indicates that the client will no longer use the output
    /// object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_output_v1.removed event is
    /// received to complete destruction of the output.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_output_v1#{}.destroy()\n", id);
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

    /// destroy the output object
    ///
    /// This request indicates that the client will no longer use the output
    /// object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_output_v1.removed event is
    /// received to complete destruction of the output.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_output_v1.destroy", &e);
        }
    }

    /// Since when the removed message is available.
    pub const MSG__REMOVED__SINCE: u32 = 1;

    /// the output is removed
    ///
    /// This event indicates that the logical output is no longer conceptually
    /// part of window management space.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_output_v1.destroy) made after this event is
    /// sent. The client should destroy this object with the
    /// river_output_v1.destroy request to free up resources.
    ///
    /// This event may be sent because a corresponding physical output has been
    /// physically unplugged or because some output configuration has changed.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_removed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_output_v1#{}.removed()\n", client_id, id);
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

    /// the output is removed
    ///
    /// This event indicates that the logical output is no longer conceptually
    /// part of window management space.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_output_v1.destroy) made after this event is
    /// sent. The client should destroy this object with the
    /// river_output_v1.destroy request to free up resources.
    ///
    /// This event may be sent because a corresponding physical output has been
    /// physically unplugged or because some output configuration has changed.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_removed(
        &self,
    ) {
        let res = self.try_send_removed(
        );
        if let Err(e) = res {
            log_send("river_output_v1.removed", &e);
        }
    }

    /// Since when the wl_output message is available.
    pub const MSG__WL_OUTPUT__SINCE: u32 = 1;

    /// corresponding wl_output
    ///
    /// The wl_output object corresponding to the river_output_v1. The argument
    /// is the global name of the wl_output advertised with wl_registry.global.
    ///
    /// It is guaranteed that the corresponding wl_output is advertised before
    /// this event is sent.
    ///
    /// This event is sent exactly once. The wl_output associated with a
    /// river_output_v1 cannot change. It is guaranteed that there is a 1-to-1
    /// mapping between wl_output and river_output_v1 objects.
    ///
    /// The global_remove event for the corresponding wl_output may be sent
    /// before the river_output_v1.remove event. This is due to the fact that
    /// river_output_v1 state changes are synced to the river window management
    /// manage sequence while changes to globals are not.
    ///
    /// Rationale: The window manager may need information provided by the
    /// wl_output interface such as the name/description. It also may need the
    /// wl_output object to start screencopy for example.
    ///
    /// # Arguments
    ///
    /// - `name`: name of the wl_output global
    #[inline]
    pub fn try_send_wl_output(
        &self,
        name: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_output_v1#{}.wl_output(name: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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

    /// corresponding wl_output
    ///
    /// The wl_output object corresponding to the river_output_v1. The argument
    /// is the global name of the wl_output advertised with wl_registry.global.
    ///
    /// It is guaranteed that the corresponding wl_output is advertised before
    /// this event is sent.
    ///
    /// This event is sent exactly once. The wl_output associated with a
    /// river_output_v1 cannot change. It is guaranteed that there is a 1-to-1
    /// mapping between wl_output and river_output_v1 objects.
    ///
    /// The global_remove event for the corresponding wl_output may be sent
    /// before the river_output_v1.remove event. This is due to the fact that
    /// river_output_v1 state changes are synced to the river window management
    /// manage sequence while changes to globals are not.
    ///
    /// Rationale: The window manager may need information provided by the
    /// wl_output interface such as the name/description. It also may need the
    /// wl_output object to start screencopy for example.
    ///
    /// # Arguments
    ///
    /// - `name`: name of the wl_output global
    #[inline]
    pub fn send_wl_output(
        &self,
        name: u32,
    ) {
        let res = self.try_send_wl_output(
            name,
        );
        if let Err(e) = res {
            log_send("river_output_v1.wl_output", &e);
        }
    }

    /// Since when the position message is available.
    pub const MSG__POSITION__SINCE: u32 = 1;

    /// output position
    ///
    /// This event indicates the position of the output in the compositor's
    /// logical coordinate space. The x and y coordinates may be positive or
    /// negative.
    ///
    /// This event is sent once when the river_output_v1 is created and again
    /// whenever the position changes.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The server must guarantee that the position and dimensions events do not
    /// cause the areas of multiple logical outputs to overlap when the
    /// corresponding manage_start event is received.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn try_send_position(
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_output_v1#{}.position(x: {}, y: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// output position
    ///
    /// This event indicates the position of the output in the compositor's
    /// logical coordinate space. The x and y coordinates may be positive or
    /// negative.
    ///
    /// This event is sent once when the river_output_v1 is created and again
    /// whenever the position changes.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The server must guarantee that the position and dimensions events do not
    /// cause the areas of multiple logical outputs to overlap when the
    /// corresponding manage_start event is received.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn send_position(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("river_output_v1.position", &e);
        }
    }

    /// Since when the dimensions message is available.
    pub const MSG__DIMENSIONS__SINCE: u32 = 1;

    /// output dimensions
    ///
    /// This event indicates the dimensions of the output in the compositor's
    /// logical coordinate space. The width and height will always be strictly
    /// greater than zero.
    ///
    /// This event is sent once when the river_output_v1 is created and again
    /// whenever the dimensions change.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The server must guarantee that the position and dimensions events do not
    /// cause the areas of multiple logical outputs to overlap when the
    /// corresponding manage_start event is received.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_dimensions(
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_output_v1#{}.dimensions(width: {}, height: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            3,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// output dimensions
    ///
    /// This event indicates the dimensions of the output in the compositor's
    /// logical coordinate space. The width and height will always be strictly
    /// greater than zero.
    ///
    /// This event is sent once when the river_output_v1 is created and again
    /// whenever the dimensions change.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The server must guarantee that the position and dimensions events do not
    /// cause the areas of multiple logical outputs to overlap when the
    /// corresponding manage_start event is received.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_dimensions(
        &self,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_dimensions(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("river_output_v1.dimensions", &e);
        }
    }
}

/// A message handler for [`RiverOutputV1`] proxies.
pub trait RiverOutputV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverOutputV1>) {
        slf.core.delete_id();
    }

    /// destroy the output object
    ///
    /// This request indicates that the client will no longer use the output
    /// object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_output_v1.removed event is
    /// received to complete destruction of the output.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverOutputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_output_v1.destroy", &e);
        }
    }

    /// the output is removed
    ///
    /// This event indicates that the logical output is no longer conceptually
    /// part of window management space.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_output_v1.destroy) made after this event is
    /// sent. The client should destroy this object with the
    /// river_output_v1.destroy request to free up resources.
    ///
    /// This event may be sent because a corresponding physical output has been
    /// physically unplugged or because some output configuration has changed.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_removed(
        &mut self,
        slf: &Rc<RiverOutputV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_removed(
        );
        if let Err(e) = res {
            log_forward("river_output_v1.removed", &e);
        }
    }

    /// corresponding wl_output
    ///
    /// The wl_output object corresponding to the river_output_v1. The argument
    /// is the global name of the wl_output advertised with wl_registry.global.
    ///
    /// It is guaranteed that the corresponding wl_output is advertised before
    /// this event is sent.
    ///
    /// This event is sent exactly once. The wl_output associated with a
    /// river_output_v1 cannot change. It is guaranteed that there is a 1-to-1
    /// mapping between wl_output and river_output_v1 objects.
    ///
    /// The global_remove event for the corresponding wl_output may be sent
    /// before the river_output_v1.remove event. This is due to the fact that
    /// river_output_v1 state changes are synced to the river window management
    /// manage sequence while changes to globals are not.
    ///
    /// Rationale: The window manager may need information provided by the
    /// wl_output interface such as the name/description. It also may need the
    /// wl_output object to start screencopy for example.
    ///
    /// # Arguments
    ///
    /// - `name`: name of the wl_output global
    #[inline]
    fn handle_wl_output(
        &mut self,
        slf: &Rc<RiverOutputV1>,
        name: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_wl_output(
            name,
        );
        if let Err(e) = res {
            log_forward("river_output_v1.wl_output", &e);
        }
    }

    /// output position
    ///
    /// This event indicates the position of the output in the compositor's
    /// logical coordinate space. The x and y coordinates may be positive or
    /// negative.
    ///
    /// This event is sent once when the river_output_v1 is created and again
    /// whenever the position changes.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The server must guarantee that the position and dimensions events do not
    /// cause the areas of multiple logical outputs to overlap when the
    /// corresponding manage_start event is received.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    fn handle_position(
        &mut self,
        slf: &Rc<RiverOutputV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("river_output_v1.position", &e);
        }
    }

    /// output dimensions
    ///
    /// This event indicates the dimensions of the output in the compositor's
    /// logical coordinate space. The width and height will always be strictly
    /// greater than zero.
    ///
    /// This event is sent once when the river_output_v1 is created and again
    /// whenever the dimensions change.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The server must guarantee that the position and dimensions events do not
    /// cause the areas of multiple logical outputs to overlap when the
    /// corresponding manage_start event is received.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_dimensions(
        &mut self,
        slf: &Rc<RiverOutputV1>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dimensions(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("river_output_v1.dimensions", &e);
        }
    }
}

impl ObjectPrivate for RiverOutputV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverOutputV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_output_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_output_v1#{}.removed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_removed(&self);
                } else {
                    DefaultHandler.handle_removed(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_output_v1#{}.wl_output(name: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_wl_output(&self, arg0);
                } else {
                    DefaultHandler.handle_wl_output(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_output_v1#{}.position(x: {}, y: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_position(&self, arg0, arg1);
                }
            }
            3 => {
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
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_output_v1#{}.dimensions(width: {}, height: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dimensions(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_dimensions(&self, arg0, arg1);
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
            0 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "removed",
            1 => "wl_output",
            2 => "position",
            3 => "dimensions",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverOutputV1 {
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

