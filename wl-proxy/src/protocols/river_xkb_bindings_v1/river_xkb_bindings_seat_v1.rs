//! xkb bindings seat
//!
//! This object manages xkb bindings state associated with a specific seat.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_xkb_bindings_seat_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverXkbBindingsSeatV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverXkbBindingsSeatV1Handler>,
}

struct DefaultHandler;

impl RiverXkbBindingsSeatV1Handler for DefaultHandler { }

impl ConcreteObject for RiverXkbBindingsSeatV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverXkbBindingsSeatV1;
    const INTERFACE_NAME: &str = "river_xkb_bindings_seat_v1";
}

impl RiverXkbBindingsSeatV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverXkbBindingsSeatV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverXkbBindingsSeatV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverXkbBindingsSeatV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverXkbBindingsSeatV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverXkbBindingsSeatV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 2;

    /// destroy the object
    ///
    /// This request indicates that the client will no longer use the object and
    /// that it may be safely destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_bindings_seat_v1#{}.destroy()\n", id);
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
    /// This request indicates that the client will no longer use the object and
    /// that it may be safely destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_xkb_bindings_seat_v1.destroy", &e);
        }
    }

    /// Since when the ensure_next_key_eaten message is available.
    pub const MSG__ENSURE_NEXT_KEY_EATEN__SINCE: u32 = 2;

    /// ensure the next key press event is eaten
    ///
    /// Ensure that the next non-modifier key press and corresponding release
    /// events for this seat are not sent to the currently focused surface.
    ///
    /// If the next non-modifier key press triggers a binding, the
    /// pressed/released events are sent to the river_xkb_binding_v1 object as
    /// usual.
    ///
    /// If the next non-modifier key press does not trigger a binding, the
    /// ate_unbound_key event is sent instead.
    ///
    /// Rationale: the window manager may wish to implement "chorded"
    /// keybindings where triggering a binding activates a "submap" with a
    /// different set of keybindings. Without a way to eat the next key
    /// press event, there is no good way for the window manager to know that it
    /// should error out and exit the submap when a key not bound in the submap
    /// is pressed.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_ensure_next_key_eaten(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_bindings_seat_v1#{}.ensure_next_key_eaten()\n", id);
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

    /// ensure the next key press event is eaten
    ///
    /// Ensure that the next non-modifier key press and corresponding release
    /// events for this seat are not sent to the currently focused surface.
    ///
    /// If the next non-modifier key press triggers a binding, the
    /// pressed/released events are sent to the river_xkb_binding_v1 object as
    /// usual.
    ///
    /// If the next non-modifier key press does not trigger a binding, the
    /// ate_unbound_key event is sent instead.
    ///
    /// Rationale: the window manager may wish to implement "chorded"
    /// keybindings where triggering a binding activates a "submap" with a
    /// different set of keybindings. Without a way to eat the next key
    /// press event, there is no good way for the window manager to know that it
    /// should error out and exit the submap when a key not bound in the submap
    /// is pressed.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_ensure_next_key_eaten(
        &self,
    ) {
        let res = self.try_send_ensure_next_key_eaten(
        );
        if let Err(e) = res {
            log_send("river_xkb_bindings_seat_v1.ensure_next_key_eaten", &e);
        }
    }

    /// Since when the cancel_ensure_next_key_eaten message is available.
    pub const MSG__CANCEL_ENSURE_NEXT_KEY_EATEN__SINCE: u32 = 2;

    /// cancel an ensure_next_key_eaten request
    ///
    /// This requests cancels the effect of the latest ensure_next_key_eaten
    /// request if no key has been eaten due to the request yet. This request
    /// has no effect if a key has already been eaten or no
    /// ensure_next_key_eaten was made.
    ///
    /// Rationale: the window manager may wish cancel an uncompleted "chorded"
    /// keybinding after a timeout of a few seconds. Note that since this
    /// timeout use-case requires the window manager to trigger a manage sequence
    /// with the river_window_manager_v1.manage_dirty request it is possible that
    /// the ate_unbound_key key event may be sent before the window manager has
    /// a chance to make the cancel_ensure_next_key_eaten request.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_cancel_ensure_next_key_eaten(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_bindings_seat_v1#{}.cancel_ensure_next_key_eaten()\n", id);
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
            2,
        ]);
        Ok(())
    }

    /// cancel an ensure_next_key_eaten request
    ///
    /// This requests cancels the effect of the latest ensure_next_key_eaten
    /// request if no key has been eaten due to the request yet. This request
    /// has no effect if a key has already been eaten or no
    /// ensure_next_key_eaten was made.
    ///
    /// Rationale: the window manager may wish cancel an uncompleted "chorded"
    /// keybinding after a timeout of a few seconds. Note that since this
    /// timeout use-case requires the window manager to trigger a manage sequence
    /// with the river_window_manager_v1.manage_dirty request it is possible that
    /// the ate_unbound_key key event may be sent before the window manager has
    /// a chance to make the cancel_ensure_next_key_eaten request.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_cancel_ensure_next_key_eaten(
        &self,
    ) {
        let res = self.try_send_cancel_ensure_next_key_eaten(
        );
        if let Err(e) = res {
            log_send("river_xkb_bindings_seat_v1.cancel_ensure_next_key_eaten", &e);
        }
    }

    /// Since when the ate_unbound_key message is available.
    pub const MSG__ATE_UNBOUND_KEY__SINCE: u32 = 2;

    /// an unbound key press event was eaten
    ///
    /// An unbound key press event was eaten due to the ensure_next_key_eaten
    /// request.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_ate_unbound_key(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_xkb_bindings_seat_v1#{}.ate_unbound_key()\n", client_id, id);
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

    /// an unbound key press event was eaten
    ///
    /// An unbound key press event was eaten due to the ensure_next_key_eaten
    /// request.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_ate_unbound_key(
        &self,
    ) {
        let res = self.try_send_ate_unbound_key(
        );
        if let Err(e) = res {
            log_send("river_xkb_bindings_seat_v1.ate_unbound_key", &e);
        }
    }
}

/// A message handler for [`RiverXkbBindingsSeatV1`] proxies.
pub trait RiverXkbBindingsSeatV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverXkbBindingsSeatV1>) {
        slf.core.delete_id();
    }

    /// destroy the object
    ///
    /// This request indicates that the client will no longer use the object and
    /// that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverXkbBindingsSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_xkb_bindings_seat_v1.destroy", &e);
        }
    }

    /// ensure the next key press event is eaten
    ///
    /// Ensure that the next non-modifier key press and corresponding release
    /// events for this seat are not sent to the currently focused surface.
    ///
    /// If the next non-modifier key press triggers a binding, the
    /// pressed/released events are sent to the river_xkb_binding_v1 object as
    /// usual.
    ///
    /// If the next non-modifier key press does not trigger a binding, the
    /// ate_unbound_key event is sent instead.
    ///
    /// Rationale: the window manager may wish to implement "chorded"
    /// keybindings where triggering a binding activates a "submap" with a
    /// different set of keybindings. Without a way to eat the next key
    /// press event, there is no good way for the window manager to know that it
    /// should error out and exit the submap when a key not bound in the submap
    /// is pressed.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_ensure_next_key_eaten(
        &mut self,
        slf: &Rc<RiverXkbBindingsSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_ensure_next_key_eaten(
        );
        if let Err(e) = res {
            log_forward("river_xkb_bindings_seat_v1.ensure_next_key_eaten", &e);
        }
    }

    /// cancel an ensure_next_key_eaten request
    ///
    /// This requests cancels the effect of the latest ensure_next_key_eaten
    /// request if no key has been eaten due to the request yet. This request
    /// has no effect if a key has already been eaten or no
    /// ensure_next_key_eaten was made.
    ///
    /// Rationale: the window manager may wish cancel an uncompleted "chorded"
    /// keybinding after a timeout of a few seconds. Note that since this
    /// timeout use-case requires the window manager to trigger a manage sequence
    /// with the river_window_manager_v1.manage_dirty request it is possible that
    /// the ate_unbound_key key event may be sent before the window manager has
    /// a chance to make the cancel_ensure_next_key_eaten request.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_cancel_ensure_next_key_eaten(
        &mut self,
        slf: &Rc<RiverXkbBindingsSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_cancel_ensure_next_key_eaten(
        );
        if let Err(e) = res {
            log_forward("river_xkb_bindings_seat_v1.cancel_ensure_next_key_eaten", &e);
        }
    }

    /// an unbound key press event was eaten
    ///
    /// An unbound key press event was eaten due to the ensure_next_key_eaten
    /// request.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_ate_unbound_key(
        &mut self,
        slf: &Rc<RiverXkbBindingsSeatV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_ate_unbound_key(
        );
        if let Err(e) = res {
            log_forward("river_xkb_bindings_seat_v1.ate_unbound_key", &e);
        }
    }
}

impl ObjectPrivate for RiverXkbBindingsSeatV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverXkbBindingsSeatV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_bindings_seat_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_bindings_seat_v1#{}.ensure_next_key_eaten()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ensure_next_key_eaten(&self);
                } else {
                    DefaultHandler.handle_ensure_next_key_eaten(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_bindings_seat_v1#{}.cancel_ensure_next_key_eaten()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_cancel_ensure_next_key_eaten(&self);
                } else {
                    DefaultHandler.handle_cancel_ensure_next_key_eaten(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_xkb_bindings_seat_v1#{}.ate_unbound_key()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ate_unbound_key(&self);
                } else {
                    DefaultHandler.handle_ate_unbound_key(&self);
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
            1 => "ensure_next_key_eaten",
            2 => "cancel_ensure_next_key_eaten",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "ate_unbound_key",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverXkbBindingsSeatV1 {
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

