//! configure a xkb key binding, receive trigger events
//!
//! This object allows the window manager to configure a xkbcommon key binding
//! and receive events when the key binding is triggered.
//!
//! The new key binding is not enabled until the enable request is made during
//! a manage sequence.
//!
//! Normally, all key events are sent to the surface with keyboard focus by
//! the compositor. Key events that trigger a key binding are not sent to the
//! surface with keyboard focus.
//!
//! If multiple key bindings would be triggered by a single physical key event
//! on the compositor side, it is compositor policy which key binding(s) will
//! receive press/release events or if all of the matched key bindings receive
//! press/release events.
//!
//! Key bindings might be matched by the same physical key event due to shared
//! keysym and modifiers. The layout override feature may also cause the same
//! physical key event to trigger two key bindings with different keysyms and
//! different layout overrides configured.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_xkb_binding_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverXkbBindingV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverXkbBindingV1Handler>,
}

struct DefaultHandler;

impl RiverXkbBindingV1Handler for DefaultHandler { }

impl ConcreteObject for RiverXkbBindingV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverXkbBindingV1;
    const INTERFACE_NAME: &str = "river_xkb_binding_v1";
}

impl RiverXkbBindingV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverXkbBindingV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverXkbBindingV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverXkbBindingV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverXkbBindingV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverXkbBindingV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xkb binding object
    ///
    /// This request indicates that the client will no longer use the xkb key
    /// binding object and that it may be safely destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_binding_v1#{}.destroy()\n", id);
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

    /// destroy the xkb binding object
    ///
    /// This request indicates that the client will no longer use the xkb key
    /// binding object and that it may be safely destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_xkb_binding_v1.destroy", &e);
        }
    }

    /// Since when the set_layout_override message is available.
    pub const MSG__SET_LAYOUT_OVERRIDE__SINCE: u32 = 1;

    /// override currently active xkb layout
    ///
    /// Specify an xkb layout that should be used to translate key events for
    /// the purpose of triggering this key binding irrespective of the currently
    /// active xkb layout.
    ///
    /// The layout argument is a 0-indexed xkbcommon layout number for the
    /// keyboard that generated the key event.
    ///
    /// If this request is never made, the currently active xkb layout of the
    /// keyboard that generated the key event will be used.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `layout`: 0-indexed xkbcommon layout
    #[inline]
    pub fn try_send_set_layout_override(
        &self,
        layout: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            layout,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_binding_v1#{}.set_layout_override(layout: {})\n", id, arg0);
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
            1,
            arg0,
        ]);
        Ok(())
    }

    /// override currently active xkb layout
    ///
    /// Specify an xkb layout that should be used to translate key events for
    /// the purpose of triggering this key binding irrespective of the currently
    /// active xkb layout.
    ///
    /// The layout argument is a 0-indexed xkbcommon layout number for the
    /// keyboard that generated the key event.
    ///
    /// If this request is never made, the currently active xkb layout of the
    /// keyboard that generated the key event will be used.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `layout`: 0-indexed xkbcommon layout
    #[inline]
    pub fn send_set_layout_override(
        &self,
        layout: u32,
    ) {
        let res = self.try_send_set_layout_override(
            layout,
        );
        if let Err(e) = res {
            log_send("river_xkb_binding_v1.set_layout_override", &e);
        }
    }

    /// Since when the enable message is available.
    pub const MSG__ENABLE__SINCE: u32 = 1;

    /// enable the key binding
    ///
    /// This request should be made after all initial configuration has been
    /// completed and the window manager wishes the key binding to be able to be
    /// triggered.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_enable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_binding_v1#{}.enable()\n", id);
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

    /// enable the key binding
    ///
    /// This request should be made after all initial configuration has been
    /// completed and the window manager wishes the key binding to be able to be
    /// triggered.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_enable(
        &self,
    ) {
        let res = self.try_send_enable(
        );
        if let Err(e) = res {
            log_send("river_xkb_binding_v1.enable", &e);
        }
    }

    /// Since when the disable message is available.
    pub const MSG__DISABLE__SINCE: u32 = 1;

    /// disable the key binding
    ///
    /// This request may be used to temporarily disable the key binding. It may
    /// be later re-enabled with the enable request.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_disable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_xkb_binding_v1#{}.disable()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// disable the key binding
    ///
    /// This request may be used to temporarily disable the key binding. It may
    /// be later re-enabled with the enable request.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_disable(
        &self,
    ) {
        let res = self.try_send_disable(
        );
        if let Err(e) = res {
            log_send("river_xkb_binding_v1.disable", &e);
        }
    }

    /// Since when the pressed message is available.
    pub const MSG__PRESSED__SINCE: u32 = 1;

    /// the key triggering the binding has been pressed
    ///
    /// This event indicates that the physical key triggering the binding has
    /// been pressed.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The compositor should wait for the manage sequence to complete before
    /// processing further input events. This allows the window manager client
    /// to, for example, modify key bindings and keyboard focus without racing
    /// against future input events. The window manager should of course respond
    /// as soon as possible as the capacity of the compositor to buffer incoming
    /// input events is finite.
    #[inline]
    pub fn try_send_pressed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_xkb_binding_v1#{}.pressed()\n", client_id, id);
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

    /// the key triggering the binding has been pressed
    ///
    /// This event indicates that the physical key triggering the binding has
    /// been pressed.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The compositor should wait for the manage sequence to complete before
    /// processing further input events. This allows the window manager client
    /// to, for example, modify key bindings and keyboard focus without racing
    /// against future input events. The window manager should of course respond
    /// as soon as possible as the capacity of the compositor to buffer incoming
    /// input events is finite.
    #[inline]
    pub fn send_pressed(
        &self,
    ) {
        let res = self.try_send_pressed(
        );
        if let Err(e) = res {
            log_send("river_xkb_binding_v1.pressed", &e);
        }
    }

    /// Since when the released message is available.
    pub const MSG__RELEASED__SINCE: u32 = 1;

    /// the key triggering the binding has been released
    ///
    /// This event indicates that the physical key triggering the binding has
    /// been released.
    ///
    /// Releasing the modifiers for the binding without releasing the "main"
    /// physical key that produces the bound keysym does not trigger the release
    /// event. This event is sent when the "main" key is released, even if the
    /// modifiers have changed since the pressed event.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The compositor should wait for the manage sequence to complete before
    /// processing further input events. This allows the window manager client
    /// to, for example, modify key bindings and keyboard focus without racing
    /// against future input events. The window manager should of course respond
    /// as soon as possible as the capacity of the compositor to buffer incoming
    /// input events is finite.
    #[inline]
    pub fn try_send_released(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_xkb_binding_v1#{}.released()\n", client_id, id);
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
            1,
        ]);
        Ok(())
    }

    /// the key triggering the binding has been released
    ///
    /// This event indicates that the physical key triggering the binding has
    /// been released.
    ///
    /// Releasing the modifiers for the binding without releasing the "main"
    /// physical key that produces the bound keysym does not trigger the release
    /// event. This event is sent when the "main" key is released, even if the
    /// modifiers have changed since the pressed event.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The compositor should wait for the manage sequence to complete before
    /// processing further input events. This allows the window manager client
    /// to, for example, modify key bindings and keyboard focus without racing
    /// against future input events. The window manager should of course respond
    /// as soon as possible as the capacity of the compositor to buffer incoming
    /// input events is finite.
    #[inline]
    pub fn send_released(
        &self,
    ) {
        let res = self.try_send_released(
        );
        if let Err(e) = res {
            log_send("river_xkb_binding_v1.released", &e);
        }
    }
}

/// A message handler for [`RiverXkbBindingV1`] proxies.
pub trait RiverXkbBindingV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverXkbBindingV1>) {
        slf.core.delete_id();
    }

    /// destroy the xkb binding object
    ///
    /// This request indicates that the client will no longer use the xkb key
    /// binding object and that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverXkbBindingV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_xkb_binding_v1.destroy", &e);
        }
    }

    /// override currently active xkb layout
    ///
    /// Specify an xkb layout that should be used to translate key events for
    /// the purpose of triggering this key binding irrespective of the currently
    /// active xkb layout.
    ///
    /// The layout argument is a 0-indexed xkbcommon layout number for the
    /// keyboard that generated the key event.
    ///
    /// If this request is never made, the currently active xkb layout of the
    /// keyboard that generated the key event will be used.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `layout`: 0-indexed xkbcommon layout
    #[inline]
    fn handle_set_layout_override(
        &mut self,
        slf: &Rc<RiverXkbBindingV1>,
        layout: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_layout_override(
            layout,
        );
        if let Err(e) = res {
            log_forward("river_xkb_binding_v1.set_layout_override", &e);
        }
    }

    /// enable the key binding
    ///
    /// This request should be made after all initial configuration has been
    /// completed and the window manager wishes the key binding to be able to be
    /// triggered.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_enable(
        &mut self,
        slf: &Rc<RiverXkbBindingV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_enable(
        );
        if let Err(e) = res {
            log_forward("river_xkb_binding_v1.enable", &e);
        }
    }

    /// disable the key binding
    ///
    /// This request may be used to temporarily disable the key binding. It may
    /// be later re-enabled with the enable request.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_disable(
        &mut self,
        slf: &Rc<RiverXkbBindingV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_disable(
        );
        if let Err(e) = res {
            log_forward("river_xkb_binding_v1.disable", &e);
        }
    }

    /// the key triggering the binding has been pressed
    ///
    /// This event indicates that the physical key triggering the binding has
    /// been pressed.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The compositor should wait for the manage sequence to complete before
    /// processing further input events. This allows the window manager client
    /// to, for example, modify key bindings and keyboard focus without racing
    /// against future input events. The window manager should of course respond
    /// as soon as possible as the capacity of the compositor to buffer incoming
    /// input events is finite.
    #[inline]
    fn handle_pressed(
        &mut self,
        slf: &Rc<RiverXkbBindingV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_pressed(
        );
        if let Err(e) = res {
            log_forward("river_xkb_binding_v1.pressed", &e);
        }
    }

    /// the key triggering the binding has been released
    ///
    /// This event indicates that the physical key triggering the binding has
    /// been released.
    ///
    /// Releasing the modifiers for the binding without releasing the "main"
    /// physical key that produces the bound keysym does not trigger the release
    /// event. This event is sent when the "main" key is released, even if the
    /// modifiers have changed since the pressed event.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// The compositor should wait for the manage sequence to complete before
    /// processing further input events. This allows the window manager client
    /// to, for example, modify key bindings and keyboard focus without racing
    /// against future input events. The window manager should of course respond
    /// as soon as possible as the capacity of the compositor to buffer incoming
    /// input events is finite.
    #[inline]
    fn handle_released(
        &mut self,
        slf: &Rc<RiverXkbBindingV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_released(
        );
        if let Err(e) = res {
            log_forward("river_xkb_binding_v1.released", &e);
        }
    }
}

impl ObjectPrivate for RiverXkbBindingV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverXkbBindingV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_binding_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_binding_v1#{}.set_layout_override(layout: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_layout_override(&self, arg0);
                } else {
                    DefaultHandler.handle_set_layout_override(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_binding_v1#{}.enable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_enable(&self);
                } else {
                    DefaultHandler.handle_enable(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_xkb_binding_v1#{}.disable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_disable(&self);
                } else {
                    DefaultHandler.handle_disable(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_xkb_binding_v1#{}.pressed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_pressed(&self);
                } else {
                    DefaultHandler.handle_pressed(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_xkb_binding_v1#{}.released()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_released(&self);
                } else {
                    DefaultHandler.handle_released(&self);
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
            1 => "set_layout_override",
            2 => "enable",
            3 => "disable",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "pressed",
            1 => "released",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverXkbBindingV1 {
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

