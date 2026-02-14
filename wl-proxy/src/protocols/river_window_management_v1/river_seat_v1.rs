//! a window management seat
//!
//! This object represents a single user's collection of input devices. It
//! allows the window manager to route keyboard input to windows, get
//! high-level information about pointer input, define keyboard and pointer
//! bindings, etc.
//!
//! TODO:
//!   - touch input
//!   - tablet input

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_seat_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverSeatV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverSeatV1Handler>,
}

struct DefaultHandler;

impl RiverSeatV1Handler for DefaultHandler { }

impl ConcreteObject for RiverSeatV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverSeatV1;
    const INTERFACE_NAME: &str = "river_seat_v1";
}

impl RiverSeatV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverSeatV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverSeatV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverSeatV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverSeatV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverSeatV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the seat object
    ///
    /// This request indicates that the client will no longer use the seat
    /// object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_seat_v1.removed event is
    /// received to complete destruction of the seat.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_seat_v1#{}.destroy()\n", id);
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

    /// destroy the seat object
    ///
    /// This request indicates that the client will no longer use the seat
    /// object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_seat_v1.removed event is
    /// received to complete destruction of the seat.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_seat_v1.destroy", &e);
        }
    }

    /// Since when the removed message is available.
    pub const MSG__REMOVED__SINCE: u32 = 1;

    /// the seat is removed
    ///
    /// This event indicates that seat is no longer in use and should be
    /// destroyed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_seat_v1.destroy) made after this event is
    /// sent.  The client should destroy this object with the
    /// river_seat_v1.destroy request to free up resources.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_seat_v1#{}.removed()\n", client_id, id);
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

    /// the seat is removed
    ///
    /// This event indicates that seat is no longer in use and should be
    /// destroyed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_seat_v1.destroy) made after this event is
    /// sent.  The client should destroy this object with the
    /// river_seat_v1.destroy request to free up resources.
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
            log_send("river_seat_v1.removed", &e);
        }
    }

    /// Since when the wl_seat message is available.
    pub const MSG__WL_SEAT__SINCE: u32 = 1;

    /// corresponding wl_seat
    ///
    /// The wl_seat object corresponding to the river_seat_v1. The argument is
    /// the global name of the wl_seat advertised with wl_registry.global.
    ///
    /// It is guaranteed that the corresponding wl_seat is advertised before
    /// this event is sent.
    ///
    /// This event is sent exactly once. The wl_seat associated with a
    /// river_seat_v1 cannot change. It is guaranteed that there is a 1-to-1
    /// mapping between wl_seat and river_seat_v1 objects.
    ///
    /// The global_remove event for the corresponding wl_seat may be sent before
    /// the river_seat_v1.remove event. This is due to the fact that
    /// river_seat_v1 state changes are synced to the river window management
    /// manage sequence while changes to globals are not.
    ///
    /// Rationale: The window manager may want to trigger window management
    /// state changes based on normal input events received by its shell
    /// surfaces for example.
    ///
    /// # Arguments
    ///
    /// - `name`: name of the wl_seat global
    #[inline]
    pub fn try_send_wl_seat(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_seat_v1#{}.wl_seat(name: {})\n", client_id, id, arg0);
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

    /// corresponding wl_seat
    ///
    /// The wl_seat object corresponding to the river_seat_v1. The argument is
    /// the global name of the wl_seat advertised with wl_registry.global.
    ///
    /// It is guaranteed that the corresponding wl_seat is advertised before
    /// this event is sent.
    ///
    /// This event is sent exactly once. The wl_seat associated with a
    /// river_seat_v1 cannot change. It is guaranteed that there is a 1-to-1
    /// mapping between wl_seat and river_seat_v1 objects.
    ///
    /// The global_remove event for the corresponding wl_seat may be sent before
    /// the river_seat_v1.remove event. This is due to the fact that
    /// river_seat_v1 state changes are synced to the river window management
    /// manage sequence while changes to globals are not.
    ///
    /// Rationale: The window manager may want to trigger window management
    /// state changes based on normal input events received by its shell
    /// surfaces for example.
    ///
    /// # Arguments
    ///
    /// - `name`: name of the wl_seat global
    #[inline]
    pub fn send_wl_seat(
        &self,
        name: u32,
    ) {
        let res = self.try_send_wl_seat(
            name,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.wl_seat", &e);
        }
    }

    /// Since when the focus_window message is available.
    pub const MSG__FOCUS_WINDOW__SINCE: u32 = 1;

    /// give keyboard focus to a window
    ///
    /// Request that the compositor send keyboard input to the given window.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `window`: window to focus
    #[inline]
    pub fn try_send_focus_window(
        &self,
        window: &Rc<RiverWindowV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            window,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("window"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_seat_v1#{}.focus_window(window: river_window_v1#{})\n", id, arg0);
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
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// give keyboard focus to a window
    ///
    /// Request that the compositor send keyboard input to the given window.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `window`: window to focus
    #[inline]
    pub fn send_focus_window(
        &self,
        window: &Rc<RiverWindowV1>,
    ) {
        let res = self.try_send_focus_window(
            window,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.focus_window", &e);
        }
    }

    /// Since when the focus_shell_surface message is available.
    pub const MSG__FOCUS_SHELL_SURFACE__SINCE: u32 = 1;

    /// give keyboard focus to a shell_surface
    ///
    /// Request that the compositor send keyboard input to the given shell
    /// surface.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `shell_surface`: shell surface to focus
    #[inline]
    pub fn try_send_focus_shell_surface(
        &self,
        shell_surface: &Rc<RiverShellSurfaceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            shell_surface,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("shell_surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_seat_v1#{}.focus_shell_surface(shell_surface: river_shell_surface_v1#{})\n", id, arg0);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// give keyboard focus to a shell_surface
    ///
    /// Request that the compositor send keyboard input to the given shell
    /// surface.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `shell_surface`: shell surface to focus
    #[inline]
    pub fn send_focus_shell_surface(
        &self,
        shell_surface: &Rc<RiverShellSurfaceV1>,
    ) {
        let res = self.try_send_focus_shell_surface(
            shell_surface,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.focus_shell_surface", &e);
        }
    }

    /// Since when the clear_focus message is available.
    pub const MSG__CLEAR_FOCUS__SINCE: u32 = 1;

    /// clear keyboard focus
    ///
    /// Request that the compositor not send keyboard input to any client.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_clear_focus(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_seat_v1#{}.clear_focus()\n", id);
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

    /// clear keyboard focus
    ///
    /// Request that the compositor not send keyboard input to any client.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_clear_focus(
        &self,
    ) {
        let res = self.try_send_clear_focus(
        );
        if let Err(e) = res {
            log_send("river_seat_v1.clear_focus", &e);
        }
    }

    /// Since when the pointer_enter message is available.
    pub const MSG__POINTER_ENTER__SINCE: u32 = 1;

    /// pointer entered a window
    ///
    /// The seat's pointer entered the given window's area.
    ///
    /// The area of a window is defined to include the area defined by the
    /// window dimensions, borders configured using river_window_v1.set_borders,
    /// and the input regions of decoration surfaces. In particular, it does not
    /// include input regions of surfaces belonging to the window that extend
    /// outside the window dimensions.
    ///
    /// The pointer of a seat may only enter a single window at a time. When the
    /// pointer moves between windows, the pointer_leave event for the old
    /// window must be sent before the pointer_enter event for the new window.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `window`: window entered
    #[inline]
    pub fn try_send_pointer_enter(
        &self,
        window: &Rc<RiverWindowV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            window,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("window", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_seat_v1#{}.pointer_enter(window: river_window_v1#{})\n", client_id, id, arg0);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// pointer entered a window
    ///
    /// The seat's pointer entered the given window's area.
    ///
    /// The area of a window is defined to include the area defined by the
    /// window dimensions, borders configured using river_window_v1.set_borders,
    /// and the input regions of decoration surfaces. In particular, it does not
    /// include input regions of surfaces belonging to the window that extend
    /// outside the window dimensions.
    ///
    /// The pointer of a seat may only enter a single window at a time. When the
    /// pointer moves between windows, the pointer_leave event for the old
    /// window must be sent before the pointer_enter event for the new window.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `window`: window entered
    #[inline]
    pub fn send_pointer_enter(
        &self,
        window: &Rc<RiverWindowV1>,
    ) {
        let res = self.try_send_pointer_enter(
            window,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.pointer_enter", &e);
        }
    }

    /// Since when the pointer_leave message is available.
    pub const MSG__POINTER_LEAVE__SINCE: u32 = 1;

    /// pointer left the entered window
    ///
    /// The seat's pointer left the window for which pointer_enter was most
    /// recently sent. See pointer_enter for details.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_pointer_leave(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_seat_v1#{}.pointer_leave()\n", client_id, id);
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
            3,
        ]);
        Ok(())
    }

    /// pointer left the entered window
    ///
    /// The seat's pointer left the window for which pointer_enter was most
    /// recently sent. See pointer_enter for details.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_pointer_leave(
        &self,
    ) {
        let res = self.try_send_pointer_leave(
        );
        if let Err(e) = res {
            log_send("river_seat_v1.pointer_leave", &e);
        }
    }

    /// Since when the window_interaction message is available.
    pub const MSG__WINDOW_INTERACTION__SINCE: u32 = 1;

    /// a window has been interacted with
    ///
    /// A window has been interacted with beyond the pointer merely passing over
    /// it. This event might be sent due to a pointer button press or due to a
    /// touch/tablet tool interaction with the window.
    ///
    /// There are no guarantees regarding how this event is sent in relation to
    /// the pointer_enter and pointer_leave events as the interaction may use
    /// touch or tablet tool input.
    ///
    /// Rationale: this event gives window managers necessary information to
    /// determine when to send keyboard focus, raise a window that already has
    /// keyboard focus, etc. Rather than expose all pointer, touch, and tablet
    /// events to window managers, a policy over mechanism approach is taken.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `window`: window interacted with
    #[inline]
    pub fn try_send_window_interaction(
        &self,
        window: &Rc<RiverWindowV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            window,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("window", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_seat_v1#{}.window_interaction(window: river_window_v1#{})\n", client_id, id, arg0);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// a window has been interacted with
    ///
    /// A window has been interacted with beyond the pointer merely passing over
    /// it. This event might be sent due to a pointer button press or due to a
    /// touch/tablet tool interaction with the window.
    ///
    /// There are no guarantees regarding how this event is sent in relation to
    /// the pointer_enter and pointer_leave events as the interaction may use
    /// touch or tablet tool input.
    ///
    /// Rationale: this event gives window managers necessary information to
    /// determine when to send keyboard focus, raise a window that already has
    /// keyboard focus, etc. Rather than expose all pointer, touch, and tablet
    /// events to window managers, a policy over mechanism approach is taken.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `window`: window interacted with
    #[inline]
    pub fn send_window_interaction(
        &self,
        window: &Rc<RiverWindowV1>,
    ) {
        let res = self.try_send_window_interaction(
            window,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.window_interaction", &e);
        }
    }

    /// Since when the shell_surface_interaction message is available.
    pub const MSG__SHELL_SURFACE_INTERACTION__SINCE: u32 = 1;

    /// a shell surface has been interacted with
    ///
    /// A shell surface has been interacted with beyond the pointer merely
    /// passing over it. This event might be sent due to a pointer button press
    /// or due to a touch/tablet tool interaction with the shell_surface.
    ///
    /// There are no guarantees regarding how this event is sent in relation to
    /// the pointer_enter and pointer_leave events as the interaction may use
    /// touch or tablet tool input.
    ///
    /// Rationale: While the shell surface does receive all wl_pointer,
    /// wl_touch, etc. input events for the surface directly, these events do
    /// not necessarily trigger a manage sequence and therefore do not allow the
    /// window manager to update focus or perform other actions in response to
    /// the input in a race-free way.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `shell_surface`: shell surface interacted with
    #[inline]
    pub fn try_send_shell_surface_interaction(
        &self,
        shell_surface: &Rc<RiverShellSurfaceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            shell_surface,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("shell_surface", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_seat_v1#{}.shell_surface_interaction(shell_surface: river_shell_surface_v1#{})\n", client_id, id, arg0);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// a shell surface has been interacted with
    ///
    /// A shell surface has been interacted with beyond the pointer merely
    /// passing over it. This event might be sent due to a pointer button press
    /// or due to a touch/tablet tool interaction with the shell_surface.
    ///
    /// There are no guarantees regarding how this event is sent in relation to
    /// the pointer_enter and pointer_leave events as the interaction may use
    /// touch or tablet tool input.
    ///
    /// Rationale: While the shell surface does receive all wl_pointer,
    /// wl_touch, etc. input events for the surface directly, these events do
    /// not necessarily trigger a manage sequence and therefore do not allow the
    /// window manager to update focus or perform other actions in response to
    /// the input in a race-free way.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `shell_surface`: shell surface interacted with
    #[inline]
    pub fn send_shell_surface_interaction(
        &self,
        shell_surface: &Rc<RiverShellSurfaceV1>,
    ) {
        let res = self.try_send_shell_surface_interaction(
            shell_surface,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.shell_surface_interaction", &e);
        }
    }

    /// Since when the op_start_pointer message is available.
    pub const MSG__OP_START_POINTER__SINCE: u32 = 1;

    /// start an interactive pointer operation
    ///
    /// Start an interactive pointer operation. During the operation, op_delta
    /// events will be sent based on pointer input.
    ///
    /// When all pointer buttons are released, the op_release event is sent.
    ///
    /// The pointer operation continues until the op_end request is made during
    /// a manage sequence and that manage sequence is finished.
    ///
    /// The window manager may use this operation to implement interactive
    /// move/resize of windows by setting the position of windows and proposing
    /// dimensions based off of the op_delta events.
    ///
    /// This request is ignored if an operation is already in progress.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_op_start_pointer(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_seat_v1#{}.op_start_pointer()\n", id);
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
            4,
        ]);
        Ok(())
    }

    /// start an interactive pointer operation
    ///
    /// Start an interactive pointer operation. During the operation, op_delta
    /// events will be sent based on pointer input.
    ///
    /// When all pointer buttons are released, the op_release event is sent.
    ///
    /// The pointer operation continues until the op_end request is made during
    /// a manage sequence and that manage sequence is finished.
    ///
    /// The window manager may use this operation to implement interactive
    /// move/resize of windows by setting the position of windows and proposing
    /// dimensions based off of the op_delta events.
    ///
    /// This request is ignored if an operation is already in progress.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_op_start_pointer(
        &self,
    ) {
        let res = self.try_send_op_start_pointer(
        );
        if let Err(e) = res {
            log_send("river_seat_v1.op_start_pointer", &e);
        }
    }

    /// Since when the op_delta message is available.
    pub const MSG__OP_DELTA__SINCE: u32 = 1;

    /// total cumulative motion since op start
    ///
    /// This event indicates the total change in position since the start of the
    /// operation of the pointer/touch point/etc.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `dx`: total change in x
    /// - `dy`: total change in y
    #[inline]
    pub fn try_send_op_delta(
        &self,
        dx: i32,
        dy: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            dx,
            dy,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_seat_v1#{}.op_delta(dx: {}, dy: {})\n", client_id, id, arg0, arg1);
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
            6,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// total cumulative motion since op start
    ///
    /// This event indicates the total change in position since the start of the
    /// operation of the pointer/touch point/etc.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `dx`: total change in x
    /// - `dy`: total change in y
    #[inline]
    pub fn send_op_delta(
        &self,
        dx: i32,
        dy: i32,
    ) {
        let res = self.try_send_op_delta(
            dx,
            dy,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.op_delta", &e);
        }
    }

    /// Since when the op_release message is available.
    pub const MSG__OP_RELEASE__SINCE: u32 = 1;

    /// operation input has been released
    ///
    /// The input driving the current interactive operation has been released.
    /// For a pointer op for example, all pointer buttons have been released.
    ///
    /// Depending on the op type, op_delta events may continue to be sent until
    /// the op is ended with the op_end request.
    ///
    /// This event is sent at most once during an interactive operation.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_op_release(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_seat_v1#{}.op_release()\n", client_id, id);
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
            7,
        ]);
        Ok(())
    }

    /// operation input has been released
    ///
    /// The input driving the current interactive operation has been released.
    /// For a pointer op for example, all pointer buttons have been released.
    ///
    /// Depending on the op type, op_delta events may continue to be sent until
    /// the op is ended with the op_end request.
    ///
    /// This event is sent at most once during an interactive operation.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_op_release(
        &self,
    ) {
        let res = self.try_send_op_release(
        );
        if let Err(e) = res {
            log_send("river_seat_v1.op_release", &e);
        }
    }

    /// Since when the op_end message is available.
    pub const MSG__OP_END__SINCE: u32 = 1;

    /// end an interactive operation
    ///
    /// End an interactive operation.
    ///
    /// This request is ignored if there is no operation in progress.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_op_end(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_seat_v1#{}.op_end()\n", id);
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
            5,
        ]);
        Ok(())
    }

    /// end an interactive operation
    ///
    /// End an interactive operation.
    ///
    /// This request is ignored if there is no operation in progress.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_op_end(
        &self,
    ) {
        let res = self.try_send_op_end(
        );
        if let Err(e) = res {
            log_send("river_seat_v1.op_end", &e);
        }
    }

    /// Since when the get_pointer_binding message is available.
    pub const MSG__GET_POINTER_BINDING__SINCE: u32 = 1;

    /// define a new pointer binding
    ///
    /// Define a pointer binding in terms of a pointer button, keyboard
    /// modifiers, and other configurable properties.
    ///
    /// The button argument is a Linux input event code defined in the
    /// linux/input-event-codes.h header file (e.g. BTN_RIGHT).
    ///
    /// The new pointer binding is not enabled until initial configuration is
    /// completed and the enable request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `id`: new pointer binding
    /// - `button`: a Linux input event code
    /// - `modifiers`: keyboard modifiers
    #[inline]
    pub fn try_send_get_pointer_binding(
        &self,
        id: &Rc<RiverPointerBindingV1>,
        button: u32,
        modifiers: RiverSeatV1Modifiers,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            button,
            modifiers,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: RiverSeatV1Modifiers) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_seat_v1#{}.get_pointer_binding(id: river_pointer_binding_v1#{}, button: {}, modifiers: {:?})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2);
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
            6,
            arg0_id,
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// define a new pointer binding
    ///
    /// Define a pointer binding in terms of a pointer button, keyboard
    /// modifiers, and other configurable properties.
    ///
    /// The button argument is a Linux input event code defined in the
    /// linux/input-event-codes.h header file (e.g. BTN_RIGHT).
    ///
    /// The new pointer binding is not enabled until initial configuration is
    /// completed and the enable request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `id`: new pointer binding
    /// - `button`: a Linux input event code
    /// - `modifiers`: keyboard modifiers
    #[inline]
    pub fn send_get_pointer_binding(
        &self,
        id: &Rc<RiverPointerBindingV1>,
        button: u32,
        modifiers: RiverSeatV1Modifiers,
    ) {
        let res = self.try_send_get_pointer_binding(
            id,
            button,
            modifiers,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.get_pointer_binding", &e);
        }
    }

    /// define a new pointer binding
    ///
    /// Define a pointer binding in terms of a pointer button, keyboard
    /// modifiers, and other configurable properties.
    ///
    /// The button argument is a Linux input event code defined in the
    /// linux/input-event-codes.h header file (e.g. BTN_RIGHT).
    ///
    /// The new pointer binding is not enabled until initial configuration is
    /// completed and the enable request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `button`: a Linux input event code
    /// - `modifiers`: keyboard modifiers
    #[inline]
    pub fn new_try_send_get_pointer_binding(
        &self,
        button: u32,
        modifiers: RiverSeatV1Modifiers,
    ) -> Result<Rc<RiverPointerBindingV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_pointer_binding(
            &id,
            button,
            modifiers,
        )?;
        Ok(id)
    }

    /// define a new pointer binding
    ///
    /// Define a pointer binding in terms of a pointer button, keyboard
    /// modifiers, and other configurable properties.
    ///
    /// The button argument is a Linux input event code defined in the
    /// linux/input-event-codes.h header file (e.g. BTN_RIGHT).
    ///
    /// The new pointer binding is not enabled until initial configuration is
    /// completed and the enable request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `button`: a Linux input event code
    /// - `modifiers`: keyboard modifiers
    #[inline]
    pub fn new_send_get_pointer_binding(
        &self,
        button: u32,
        modifiers: RiverSeatV1Modifiers,
    ) -> Rc<RiverPointerBindingV1> {
        let id = self.core.create_child();
        self.send_get_pointer_binding(
            &id,
            button,
            modifiers,
        );
        id
    }

    /// Since when the set_xcursor_theme message is available.
    pub const MSG__SET_XCURSOR_THEME__SINCE: u32 = 2;

    /// set the xcursor theme for the seat
    ///
    /// Set the XCursor theme for the seat. This theme is used for cursors
    /// rendered by the compositor, but not necessarily for cursors rendered by
    /// clients.
    ///
    /// Note: The window manager may also wish to set the XCURSOR_THEME and
    /// XCURSOR_SIZE environment variable for programs it starts.
    ///
    /// # Arguments
    ///
    /// - `name`: xcursor theme name
    /// - `size`: cursor size
    #[inline]
    pub fn try_send_set_xcursor_theme(
        &self,
        name: &str,
        size: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            name,
            size,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_seat_v1#{}.set_xcursor_theme(name: {:?}, size: {})\n", id, arg0, arg1);
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
            7,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1,
        ]);
        Ok(())
    }

    /// set the xcursor theme for the seat
    ///
    /// Set the XCursor theme for the seat. This theme is used for cursors
    /// rendered by the compositor, but not necessarily for cursors rendered by
    /// clients.
    ///
    /// Note: The window manager may also wish to set the XCURSOR_THEME and
    /// XCURSOR_SIZE environment variable for programs it starts.
    ///
    /// # Arguments
    ///
    /// - `name`: xcursor theme name
    /// - `size`: cursor size
    #[inline]
    pub fn send_set_xcursor_theme(
        &self,
        name: &str,
        size: u32,
    ) {
        let res = self.try_send_set_xcursor_theme(
            name,
            size,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.set_xcursor_theme", &e);
        }
    }

    /// Since when the pointer_position message is available.
    pub const MSG__POINTER_POSITION__SINCE: u32 = 2;

    /// The current position of the pointer
    ///
    /// The current position of the pointer in the compositor's logical
    /// coordinate space.
    ///
    /// This state is special in that a change in pointer position alone must
    /// not cause the compositor to start a manage sequence.
    ///
    /// Assuming the seat has a pointer, this event must be sent in every manage
    /// sequence unless there is no change in x/y position since the last time this
    /// event was sent.
    ///
    /// # Arguments
    ///
    /// - `x`: global x coordinate
    /// - `y`: global y coordinate
    #[inline]
    pub fn try_send_pointer_position(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_seat_v1#{}.pointer_position(x: {}, y: {})\n", client_id, id, arg0, arg1);
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
            8,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// The current position of the pointer
    ///
    /// The current position of the pointer in the compositor's logical
    /// coordinate space.
    ///
    /// This state is special in that a change in pointer position alone must
    /// not cause the compositor to start a manage sequence.
    ///
    /// Assuming the seat has a pointer, this event must be sent in every manage
    /// sequence unless there is no change in x/y position since the last time this
    /// event was sent.
    ///
    /// # Arguments
    ///
    /// - `x`: global x coordinate
    /// - `y`: global y coordinate
    #[inline]
    pub fn send_pointer_position(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_pointer_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.pointer_position", &e);
        }
    }

    /// Since when the pointer_warp message is available.
    pub const MSG__POINTER_WARP__SINCE: u32 = 3;

    /// warp the pointer to a given position
    ///
    /// Warp the pointer to the given position in the compositor's logical
    /// coordinate space.
    ///
    /// If the given position is outside the bounds of all outputs, the pointer
    /// will be warped to the closest point inside an output instead.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`: global x coordinate
    /// - `y`: global y coordinate
    #[inline]
    pub fn try_send_pointer_warp(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_seat_v1#{}.pointer_warp(x: {}, y: {})\n", id, arg0, arg1);
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
            8,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// warp the pointer to a given position
    ///
    /// Warp the pointer to the given position in the compositor's logical
    /// coordinate space.
    ///
    /// If the given position is outside the bounds of all outputs, the pointer
    /// will be warped to the closest point inside an output instead.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`: global x coordinate
    /// - `y`: global y coordinate
    #[inline]
    pub fn send_pointer_warp(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_pointer_warp(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("river_seat_v1.pointer_warp", &e);
        }
    }
}

/// A message handler for [`RiverSeatV1`] proxies.
pub trait RiverSeatV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverSeatV1>) {
        slf.core.delete_id();
    }

    /// destroy the seat object
    ///
    /// This request indicates that the client will no longer use the seat
    /// object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_seat_v1.removed event is
    /// received to complete destruction of the seat.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.destroy", &e);
        }
    }

    /// the seat is removed
    ///
    /// This event indicates that seat is no longer in use and should be
    /// destroyed.
    ///
    /// The server will send no further events on this object and ignore any
    /// request (other than river_seat_v1.destroy) made after this event is
    /// sent.  The client should destroy this object with the
    /// river_seat_v1.destroy request to free up resources.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_removed(
        &mut self,
        slf: &Rc<RiverSeatV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_removed(
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.removed", &e);
        }
    }

    /// corresponding wl_seat
    ///
    /// The wl_seat object corresponding to the river_seat_v1. The argument is
    /// the global name of the wl_seat advertised with wl_registry.global.
    ///
    /// It is guaranteed that the corresponding wl_seat is advertised before
    /// this event is sent.
    ///
    /// This event is sent exactly once. The wl_seat associated with a
    /// river_seat_v1 cannot change. It is guaranteed that there is a 1-to-1
    /// mapping between wl_seat and river_seat_v1 objects.
    ///
    /// The global_remove event for the corresponding wl_seat may be sent before
    /// the river_seat_v1.remove event. This is due to the fact that
    /// river_seat_v1 state changes are synced to the river window management
    /// manage sequence while changes to globals are not.
    ///
    /// Rationale: The window manager may want to trigger window management
    /// state changes based on normal input events received by its shell
    /// surfaces for example.
    ///
    /// # Arguments
    ///
    /// - `name`: name of the wl_seat global
    #[inline]
    fn handle_wl_seat(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        name: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_wl_seat(
            name,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.wl_seat", &e);
        }
    }

    /// give keyboard focus to a window
    ///
    /// Request that the compositor send keyboard input to the given window.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `window`: window to focus
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_focus_window(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        window: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_focus_window(
            window,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.focus_window", &e);
        }
    }

    /// give keyboard focus to a shell_surface
    ///
    /// Request that the compositor send keyboard input to the given shell
    /// surface.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `shell_surface`: shell surface to focus
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_focus_shell_surface(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        shell_surface: &Rc<RiverShellSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_focus_shell_surface(
            shell_surface,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.focus_shell_surface", &e);
        }
    }

    /// clear keyboard focus
    ///
    /// Request that the compositor not send keyboard input to any client.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_clear_focus(
        &mut self,
        slf: &Rc<RiverSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_clear_focus(
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.clear_focus", &e);
        }
    }

    /// pointer entered a window
    ///
    /// The seat's pointer entered the given window's area.
    ///
    /// The area of a window is defined to include the area defined by the
    /// window dimensions, borders configured using river_window_v1.set_borders,
    /// and the input regions of decoration surfaces. In particular, it does not
    /// include input regions of surfaces belonging to the window that extend
    /// outside the window dimensions.
    ///
    /// The pointer of a seat may only enter a single window at a time. When the
    /// pointer moves between windows, the pointer_leave event for the old
    /// window must be sent before the pointer_enter event for the new window.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `window`: window entered
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_pointer_enter(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        window: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = window.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_pointer_enter(
            window,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.pointer_enter", &e);
        }
    }

    /// pointer left the entered window
    ///
    /// The seat's pointer left the window for which pointer_enter was most
    /// recently sent. See pointer_enter for details.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_pointer_leave(
        &mut self,
        slf: &Rc<RiverSeatV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_pointer_leave(
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.pointer_leave", &e);
        }
    }

    /// a window has been interacted with
    ///
    /// A window has been interacted with beyond the pointer merely passing over
    /// it. This event might be sent due to a pointer button press or due to a
    /// touch/tablet tool interaction with the window.
    ///
    /// There are no guarantees regarding how this event is sent in relation to
    /// the pointer_enter and pointer_leave events as the interaction may use
    /// touch or tablet tool input.
    ///
    /// Rationale: this event gives window managers necessary information to
    /// determine when to send keyboard focus, raise a window that already has
    /// keyboard focus, etc. Rather than expose all pointer, touch, and tablet
    /// events to window managers, a policy over mechanism approach is taken.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `window`: window interacted with
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_window_interaction(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        window: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = window.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_window_interaction(
            window,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.window_interaction", &e);
        }
    }

    /// a shell surface has been interacted with
    ///
    /// A shell surface has been interacted with beyond the pointer merely
    /// passing over it. This event might be sent due to a pointer button press
    /// or due to a touch/tablet tool interaction with the shell_surface.
    ///
    /// There are no guarantees regarding how this event is sent in relation to
    /// the pointer_enter and pointer_leave events as the interaction may use
    /// touch or tablet tool input.
    ///
    /// Rationale: While the shell surface does receive all wl_pointer,
    /// wl_touch, etc. input events for the surface directly, these events do
    /// not necessarily trigger a manage sequence and therefore do not allow the
    /// window manager to update focus or perform other actions in response to
    /// the input in a race-free way.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `shell_surface`: shell surface interacted with
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_shell_surface_interaction(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        shell_surface: &Rc<RiverShellSurfaceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = shell_surface.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_shell_surface_interaction(
            shell_surface,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.shell_surface_interaction", &e);
        }
    }

    /// start an interactive pointer operation
    ///
    /// Start an interactive pointer operation. During the operation, op_delta
    /// events will be sent based on pointer input.
    ///
    /// When all pointer buttons are released, the op_release event is sent.
    ///
    /// The pointer operation continues until the op_end request is made during
    /// a manage sequence and that manage sequence is finished.
    ///
    /// The window manager may use this operation to implement interactive
    /// move/resize of windows by setting the position of windows and proposing
    /// dimensions based off of the op_delta events.
    ///
    /// This request is ignored if an operation is already in progress.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_op_start_pointer(
        &mut self,
        slf: &Rc<RiverSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_op_start_pointer(
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.op_start_pointer", &e);
        }
    }

    /// total cumulative motion since op start
    ///
    /// This event indicates the total change in position since the start of the
    /// operation of the pointer/touch point/etc.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `dx`: total change in x
    /// - `dy`: total change in y
    #[inline]
    fn handle_op_delta(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        dx: i32,
        dy: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_op_delta(
            dx,
            dy,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.op_delta", &e);
        }
    }

    /// operation input has been released
    ///
    /// The input driving the current interactive operation has been released.
    /// For a pointer op for example, all pointer buttons have been released.
    ///
    /// Depending on the op type, op_delta events may continue to be sent until
    /// the op is ended with the op_end request.
    ///
    /// This event is sent at most once during an interactive operation.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_op_release(
        &mut self,
        slf: &Rc<RiverSeatV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_op_release(
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.op_release", &e);
        }
    }

    /// end an interactive operation
    ///
    /// End an interactive operation.
    ///
    /// This request is ignored if there is no operation in progress.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_op_end(
        &mut self,
        slf: &Rc<RiverSeatV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_op_end(
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.op_end", &e);
        }
    }

    /// define a new pointer binding
    ///
    /// Define a pointer binding in terms of a pointer button, keyboard
    /// modifiers, and other configurable properties.
    ///
    /// The button argument is a Linux input event code defined in the
    /// linux/input-event-codes.h header file (e.g. BTN_RIGHT).
    ///
    /// The new pointer binding is not enabled until initial configuration is
    /// completed and the enable request is made during a manage sequence.
    ///
    /// # Arguments
    ///
    /// - `id`: new pointer binding
    /// - `button`: a Linux input event code
    /// - `modifiers`: keyboard modifiers
    #[inline]
    fn handle_get_pointer_binding(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        id: &Rc<RiverPointerBindingV1>,
        button: u32,
        modifiers: RiverSeatV1Modifiers,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_pointer_binding(
            id,
            button,
            modifiers,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.get_pointer_binding", &e);
        }
    }

    /// set the xcursor theme for the seat
    ///
    /// Set the XCursor theme for the seat. This theme is used for cursors
    /// rendered by the compositor, but not necessarily for cursors rendered by
    /// clients.
    ///
    /// Note: The window manager may also wish to set the XCURSOR_THEME and
    /// XCURSOR_SIZE environment variable for programs it starts.
    ///
    /// # Arguments
    ///
    /// - `name`: xcursor theme name
    /// - `size`: cursor size
    #[inline]
    fn handle_set_xcursor_theme(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        name: &str,
        size: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_xcursor_theme(
            name,
            size,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.set_xcursor_theme", &e);
        }
    }

    /// The current position of the pointer
    ///
    /// The current position of the pointer in the compositor's logical
    /// coordinate space.
    ///
    /// This state is special in that a change in pointer position alone must
    /// not cause the compositor to start a manage sequence.
    ///
    /// Assuming the seat has a pointer, this event must be sent in every manage
    /// sequence unless there is no change in x/y position since the last time this
    /// event was sent.
    ///
    /// # Arguments
    ///
    /// - `x`: global x coordinate
    /// - `y`: global y coordinate
    #[inline]
    fn handle_pointer_position(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_pointer_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.pointer_position", &e);
        }
    }

    /// warp the pointer to a given position
    ///
    /// Warp the pointer to the given position in the compositor's logical
    /// coordinate space.
    ///
    /// If the given position is outside the bounds of all outputs, the pointer
    /// will be warped to the closest point inside an output instead.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`: global x coordinate
    /// - `y`: global y coordinate
    #[inline]
    fn handle_pointer_warp(
        &mut self,
        slf: &Rc<RiverSeatV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_pointer_warp(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("river_seat_v1.pointer_warp", &e);
        }
    }
}

impl ObjectPrivate for RiverSeatV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverSeatV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_seat_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_seat_v1#{}.focus_window(window: river_window_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverWindowV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("window", o.core().interface, ObjectInterface::RiverWindowV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_focus_window(&self, arg0);
                } else {
                    DefaultHandler.handle_focus_window(&self, arg0);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_seat_v1#{}.focus_shell_surface(shell_surface: river_shell_surface_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverShellSurfaceV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("shell_surface", o.core().interface, ObjectInterface::RiverShellSurfaceV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_focus_shell_surface(&self, arg0);
                } else {
                    DefaultHandler.handle_focus_shell_surface(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_seat_v1#{}.clear_focus()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_clear_focus(&self);
                } else {
                    DefaultHandler.handle_clear_focus(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_seat_v1#{}.op_start_pointer()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_op_start_pointer(&self);
                } else {
                    DefaultHandler.handle_op_start_pointer(&self);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_seat_v1#{}.op_end()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_op_end(&self);
                } else {
                    DefaultHandler.handle_op_end(&self);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg2 = RiverSeatV1Modifiers(arg2);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: RiverSeatV1Modifiers) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_seat_v1#{}.get_pointer_binding(id: river_pointer_binding_v1#{}, button: {}, modifiers: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = RiverPointerBindingV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_pointer_binding(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_get_pointer_binding(&self, arg0, arg1, arg2);
                }
            }
            7 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("size")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_seat_v1#{}.set_xcursor_theme(name: {:?}, size: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_xcursor_theme(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_xcursor_theme(&self, arg0, arg1);
                }
            }
            8 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_seat_v1#{}.pointer_warp(x: {}, y: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_pointer_warp(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_pointer_warp(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_seat_v1#{}.removed()\n", id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_seat_v1#{}.wl_seat(name: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_wl_seat(&self, arg0);
                } else {
                    DefaultHandler.handle_wl_seat(&self, arg0);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_seat_v1#{}.pointer_enter(window: river_window_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverWindowV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("window", o.core().interface, ObjectInterface::RiverWindowV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_pointer_enter(&self, arg0);
                } else {
                    DefaultHandler.handle_pointer_enter(&self, arg0);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_seat_v1#{}.pointer_leave()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_pointer_leave(&self);
                } else {
                    DefaultHandler.handle_pointer_leave(&self);
                }
            }
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_seat_v1#{}.window_interaction(window: river_window_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverWindowV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("window", o.core().interface, ObjectInterface::RiverWindowV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_window_interaction(&self, arg0);
                } else {
                    DefaultHandler.handle_window_interaction(&self, arg0);
                }
            }
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_seat_v1#{}.shell_surface_interaction(shell_surface: river_shell_surface_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverShellSurfaceV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("shell_surface", o.core().interface, ObjectInterface::RiverShellSurfaceV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_shell_surface_interaction(&self, arg0);
                } else {
                    DefaultHandler.handle_shell_surface_interaction(&self, arg0);
                }
            }
            6 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_seat_v1#{}.op_delta(dx: {}, dy: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_op_delta(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_op_delta(&self, arg0, arg1);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_seat_v1#{}.op_release()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_op_release(&self);
                } else {
                    DefaultHandler.handle_op_release(&self);
                }
            }
            8 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_seat_v1#{}.pointer_position(x: {}, y: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_pointer_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_pointer_position(&self, arg0, arg1);
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
            1 => "focus_window",
            2 => "focus_shell_surface",
            3 => "clear_focus",
            4 => "op_start_pointer",
            5 => "op_end",
            6 => "get_pointer_binding",
            7 => "set_xcursor_theme",
            8 => "pointer_warp",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "removed",
            1 => "wl_seat",
            2 => "pointer_enter",
            3 => "pointer_leave",
            4 => "window_interaction",
            5 => "shell_surface_interaction",
            6 => "op_delta",
            7 => "op_release",
            8 => "pointer_position",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverSeatV1 {
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

impl RiverSeatV1 {
    /// Since when the modifiers.none enum variant is available.
    pub const ENM__MODIFIERS_NONE__SINCE: u32 = 1;
    /// Since when the modifiers.shift enum variant is available.
    pub const ENM__MODIFIERS_SHIFT__SINCE: u32 = 1;
    /// Since when the modifiers.ctrl enum variant is available.
    pub const ENM__MODIFIERS_CTRL__SINCE: u32 = 1;
    /// Since when the modifiers.mod1 enum variant is available.
    pub const ENM__MODIFIERS_MOD1__SINCE: u32 = 1;
    /// Since when the modifiers.mod3 enum variant is available.
    pub const ENM__MODIFIERS_MOD3__SINCE: u32 = 1;
    /// Since when the modifiers.mod4 enum variant is available.
    pub const ENM__MODIFIERS_MOD4__SINCE: u32 = 1;
    /// Since when the modifiers.mod5 enum variant is available.
    pub const ENM__MODIFIERS_MOD5__SINCE: u32 = 1;
}

/// a set of keyboard modifiers
///
/// This enum is used to describe the keyboard modifiers that must be held
/// down to trigger a key binding or pointer binding.
///
/// Note that river and wlroots use the values 2 and 16 for capslock and
/// numlock internally. It doesn't make sense to use locked modifiers for
/// bindings however so these values are not included in this enum.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct RiverSeatV1Modifiers(pub u32);

/// An iterator over the set bits in a [`RiverSeatV1Modifiers`].
///
/// You can construct this with the `IntoIterator` implementation of `RiverSeatV1Modifiers`.
#[derive(Clone, Debug)]
pub struct RiverSeatV1ModifiersIter(pub u32);

impl RiverSeatV1Modifiers {
    pub const NONE: Self = Self(0);

    pub const SHIFT: Self = Self(1);

    pub const CTRL: Self = Self(4);

    /// commonly called alt
    pub const MOD1: Self = Self(8);

    pub const MOD3: Self = Self(32);

    /// commonly called super or logo
    pub const MOD4: Self = Self(64);

    pub const MOD5: Self = Self(128);
}

impl RiverSeatV1Modifiers {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 0 | 1 | 4 | 8 | 32 | 64 | 128)
    }
}

impl Iterator for RiverSeatV1ModifiersIter {
    type Item = RiverSeatV1Modifiers;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(RiverSeatV1Modifiers(bit))
    }
}

impl IntoIterator for RiverSeatV1Modifiers {
    type Item = RiverSeatV1Modifiers;
    type IntoIter = RiverSeatV1ModifiersIter;

    fn into_iter(self) -> Self::IntoIter {
        RiverSeatV1ModifiersIter(self.0)
    }
}

impl BitAnd for RiverSeatV1Modifiers {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for RiverSeatV1Modifiers {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for RiverSeatV1Modifiers {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for RiverSeatV1Modifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for RiverSeatV1Modifiers {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for RiverSeatV1Modifiers {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for RiverSeatV1Modifiers {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for RiverSeatV1Modifiers {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for RiverSeatV1Modifiers {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for RiverSeatV1Modifiers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("SHIFT")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("CTRL")?;
        }
        if v & 8 == 8 {
            v &= !8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("MOD1")?;
        }
        if v & 32 == 32 {
            v &= !32;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("MOD3")?;
        }
        if v & 64 == 64 {
            v &= !64;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("MOD4")?;
        }
        if v & 128 == 128 {
            v &= !128;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("MOD5")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("NONE")?;
        }
        Ok(())
    }
}
