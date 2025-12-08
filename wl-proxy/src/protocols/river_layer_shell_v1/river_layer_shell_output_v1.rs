//! layer shell output state
//!
//! The lifetime of this object is tied to the corresponding river_output_v1.
//! This object is made inert when the river_output_v1.removed event is sent
//! and should be destroyed.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_layer_shell_output_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverLayerShellOutputV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverLayerShellOutputV1Handler>,
}

struct DefaultHandler;

impl RiverLayerShellOutputV1Handler for DefaultHandler { }

impl ConcreteObject for RiverLayerShellOutputV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverLayerShellOutputV1;
    const INTERFACE_NAME: &str = "river_layer_shell_output_v1";
}

impl RiverLayerShellOutputV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverLayerShellOutputV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverLayerShellOutputV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverLayerShellOutputV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverLayerShellOutputV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverLayerShellOutputV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the object
    ///
    /// This request indicates that the client will no longer use the
    /// river_layer_shell_output_v1 object and that it may be safely destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_layer_shell_output_v1#{}.destroy()\n", id);
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

    /// destroy the object
    ///
    /// This request indicates that the client will no longer use the
    /// river_layer_shell_output_v1 object and that it may be safely destroyed.
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
            log_send("river_layer_shell_output_v1.destroy", &e);
        }
    }

    /// Since when the non_exclusive_area message is available.
    pub const MSG__NON_EXCLUSIVE_AREA__SINCE: u32 = 1;

    /// area left after subtracting exclusive zones
    ///
    /// This event indicates the area of the output remaining after subtracting
    /// the exclusive zones of layer surfaces. Exclusive zones are a hint, the
    /// window manager is free to ignore this area hint if it wishes.
    ///
    /// The x and y values are in the global coordinate space, not relative to
    /// the position of the output.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_non_exclusive_area(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            x,
            y,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_layer_shell_output_v1#{}.non_exclusive_area(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// area left after subtracting exclusive zones
    ///
    /// This event indicates the area of the output remaining after subtracting
    /// the exclusive zones of layer surfaces. Exclusive zones are a hint, the
    /// window manager is free to ignore this area hint if it wishes.
    ///
    /// The x and y values are in the global coordinate space, not relative to
    /// the position of the output.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_non_exclusive_area(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_non_exclusive_area(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("river_layer_shell_output_v1.non_exclusive_area", &e);
        }
    }

    /// Since when the set_default message is available.
    pub const MSG__SET_DEFAULT__SINCE: u32 = 1;

    /// Set default output for layer surfaces
    ///
    /// Mark this output as the default for new layer surfaces which do not
    /// request a specific output themselves. This request overrides any
    /// previous set_default request on any river_layer_shell_output_v1 object.
    ///
    /// If no set_default request is made or if the default output is destroyed,
    /// the default output is undefined until the next set_default request.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_set_default(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_layer_shell_output_v1#{}.set_default()\n", id);
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
        Ok(())
    }

    /// Set default output for layer surfaces
    ///
    /// Mark this output as the default for new layer surfaces which do not
    /// request a specific output themselves. This request overrides any
    /// previous set_default request on any river_layer_shell_output_v1 object.
    ///
    /// If no set_default request is made or if the default output is destroyed,
    /// the default output is undefined until the next set_default request.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_set_default(
        &self,
    ) {
        let res = self.try_send_set_default(
        );
        if let Err(e) = res {
            log_send("river_layer_shell_output_v1.set_default", &e);
        }
    }
}

/// A message handler for [`RiverLayerShellOutputV1`] proxies.
pub trait RiverLayerShellOutputV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverLayerShellOutputV1>) {
        slf.core.delete_id();
    }

    /// destroy the object
    ///
    /// This request indicates that the client will no longer use the
    /// river_layer_shell_output_v1 object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_output_v1.removed event is
    /// received to complete destruction of the output.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverLayerShellOutputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_layer_shell_output_v1.destroy", &e);
        }
    }

    /// area left after subtracting exclusive zones
    ///
    /// This event indicates the area of the output remaining after subtracting
    /// the exclusive zones of layer surfaces. Exclusive zones are a hint, the
    /// window manager is free to ignore this area hint if it wishes.
    ///
    /// The x and y values are in the global coordinate space, not relative to
    /// the position of the output.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_non_exclusive_area(
        &mut self,
        slf: &Rc<RiverLayerShellOutputV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_non_exclusive_area(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("river_layer_shell_output_v1.non_exclusive_area", &e);
        }
    }

    /// Set default output for layer surfaces
    ///
    /// Mark this output as the default for new layer surfaces which do not
    /// request a specific output themselves. This request overrides any
    /// previous set_default request on any river_layer_shell_output_v1 object.
    ///
    /// If no set_default request is made or if the default output is destroyed,
    /// the default output is undefined until the next set_default request.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_set_default(
        &mut self,
        slf: &Rc<RiverLayerShellOutputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_default(
        );
        if let Err(e) = res {
            log_forward("river_layer_shell_output_v1.set_default", &e);
        }
    }
}

impl ObjectPrivate for RiverLayerShellOutputV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverLayerShellOutputV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_layer_shell_output_v1#{}.destroy()\n", client_id, id);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_layer_shell_output_v1#{}.set_default()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_default(&self);
                } else {
                    DefaultHandler.handle_set_default(&self);
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
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_layer_shell_output_v1#{}.non_exclusive_area(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_non_exclusive_area(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_non_exclusive_area(&self, arg0, arg1, arg2, arg3);
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
            1 => "set_default",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "non_exclusive_area",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverLayerShellOutputV1 {
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

