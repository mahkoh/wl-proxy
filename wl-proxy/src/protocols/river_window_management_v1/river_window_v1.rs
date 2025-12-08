//! a logical window
//!
//! This represents a logical window. For example, a window may correspond to
//! an xdg_toplevel or Xwayland window.
//!
//! A newly created window will not be displayed until the window manager
//! proposes window dimensions with the propose_dimensions request as part of
//! a manage sequence, the server replies with a dimensions event as part of
//! a render sequence, and that render sequence is finished.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_window_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverWindowV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverWindowV1Handler>,
}

struct DefaultHandler;

impl RiverWindowV1Handler for DefaultHandler { }

impl ConcreteObject for RiverWindowV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverWindowV1;
    const INTERFACE_NAME: &str = "river_window_v1";
}

impl RiverWindowV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverWindowV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverWindowV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverWindowV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverWindowV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverWindowV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the window object
    ///
    /// This request indicates that the client will no longer use the window
    /// object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_window_v1.closed event or
    /// river_window_manager_v1.finished is received to complete destruction of
    /// the window.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.destroy()\n", id);
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

    /// destroy the window object
    ///
    /// This request indicates that the client will no longer use the window
    /// object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_window_v1.closed event or
    /// river_window_manager_v1.finished is received to complete destruction of
    /// the window.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_window_v1.destroy", &e);
        }
    }

    /// Since when the closed message is available.
    pub const MSG__CLOSED__SINCE: u32 = 1;

    /// the window has been closed
    ///
    /// The window has been closed by the server, perhaps due to an
    /// xdg_toplevel.close request or similar.
    ///
    /// The server will send no further events on this object and ignore any
    /// request other than river_window_v1.destroy made after this event is
    /// sent. The client should destroy this object with the
    /// river_window_v1.destroy request to free up resources.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_closed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.closed()\n", client_id, id);
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

    /// the window has been closed
    ///
    /// The window has been closed by the server, perhaps due to an
    /// xdg_toplevel.close request or similar.
    ///
    /// The server will send no further events on this object and ignore any
    /// request other than river_window_v1.destroy made after this event is
    /// sent. The client should destroy this object with the
    /// river_window_v1.destroy request to free up resources.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_closed(
        &self,
    ) {
        let res = self.try_send_closed(
        );
        if let Err(e) = res {
            log_send("river_window_v1.closed", &e);
        }
    }

    /// Since when the close message is available.
    pub const MSG__CLOSE__SINCE: u32 = 1;

    /// request that the window be closed
    ///
    /// Request that the window be closed. The window may ignore this request or
    /// only close after some delay, perhaps opening a dialog asking the user to
    /// save their work or similar.
    ///
    /// The server will send a river_window_v1.closed event if/when the window
    /// has been closed.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_close(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.close()\n", id);
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

    /// request that the window be closed
    ///
    /// Request that the window be closed. The window may ignore this request or
    /// only close after some delay, perhaps opening a dialog asking the user to
    /// save their work or similar.
    ///
    /// The server will send a river_window_v1.closed event if/when the window
    /// has been closed.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_close(
        &self,
    ) {
        let res = self.try_send_close(
        );
        if let Err(e) = res {
            log_send("river_window_v1.close", &e);
        }
    }

    /// Since when the get_node message is available.
    pub const MSG__GET_NODE__SINCE: u32 = 1;

    /// get the window's render list node
    ///
    /// Get the node in the render list corresponding to the window.
    ///
    /// It is a protocol error to make this request more than once for a single
    /// window.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_get_node(
        &self,
        id: &Rc<RiverNodeV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.get_node(id: river_node_v1#{})\n", id, arg0);
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

    /// get the window's render list node
    ///
    /// Get the node in the render list corresponding to the window.
    ///
    /// It is a protocol error to make this request more than once for a single
    /// window.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_get_node(
        &self,
        id: &Rc<RiverNodeV1>,
    ) {
        let res = self.try_send_get_node(
            id,
        );
        if let Err(e) = res {
            log_send("river_window_v1.get_node", &e);
        }
    }

    /// get the window's render list node
    ///
    /// Get the node in the render list corresponding to the window.
    ///
    /// It is a protocol error to make this request more than once for a single
    /// window.
    #[inline]
    pub fn new_try_send_get_node(
        &self,
    ) -> Result<Rc<RiverNodeV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_node(
            &id,
        )?;
        Ok(id)
    }

    /// get the window's render list node
    ///
    /// Get the node in the render list corresponding to the window.
    ///
    /// It is a protocol error to make this request more than once for a single
    /// window.
    #[inline]
    pub fn new_send_get_node(
        &self,
    ) -> Rc<RiverNodeV1> {
        let id = self.core.create_child();
        self.send_get_node(
            &id,
        );
        id
    }

    /// Since when the dimensions_hint message is available.
    pub const MSG__DIMENSIONS_HINT__SINCE: u32 = 1;

    /// the window's preferred min/max dimensions
    ///
    /// This event informs the window manager of the window's preferred min/max
    /// dimensions. These preferences are a hint, and the window manager is free
    /// to propose dimensions outside of these bounds.
    ///
    /// All min/max width/height values must be strictly greater than or equal
    /// to 0. A value of 0 indicates that the window has no preference for that
    /// value.
    ///
    /// The min_width/min_height must be strictly less than or equal to the
    /// max_width/max_height.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `min_width`:
    /// - `min_height`:
    /// - `max_width`:
    /// - `max_height`:
    #[inline]
    pub fn try_send_dimensions_hint(
        &self,
        min_width: i32,
        min_height: i32,
        max_width: i32,
        max_height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            min_width,
            min_height,
            max_width,
            max_height,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.dimensions_hint(min_width: {}, min_height: {}, max_width: {}, max_height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
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
            1,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// the window's preferred min/max dimensions
    ///
    /// This event informs the window manager of the window's preferred min/max
    /// dimensions. These preferences are a hint, and the window manager is free
    /// to propose dimensions outside of these bounds.
    ///
    /// All min/max width/height values must be strictly greater than or equal
    /// to 0. A value of 0 indicates that the window has no preference for that
    /// value.
    ///
    /// The min_width/min_height must be strictly less than or equal to the
    /// max_width/max_height.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `min_width`:
    /// - `min_height`:
    /// - `max_width`:
    /// - `max_height`:
    #[inline]
    pub fn send_dimensions_hint(
        &self,
        min_width: i32,
        min_height: i32,
        max_width: i32,
        max_height: i32,
    ) {
        let res = self.try_send_dimensions_hint(
            min_width,
            min_height,
            max_width,
            max_height,
        );
        if let Err(e) = res {
            log_send("river_window_v1.dimensions_hint", &e);
        }
    }

    /// Since when the dimensions message is available.
    pub const MSG__DIMENSIONS__SINCE: u32 = 1;

    /// window dimensions
    ///
    /// This event indicates the dimensions of the window in the compositor's
    /// logical coordinate space. The width and height must be strictly greater
    /// than zero.
    ///
    /// Note that the dimensions of a river_window_v1 refer to the dimensions of
    /// the window content and are unaffected by the presence of borders or
    /// decoration surfaces.
    ///
    /// This event is sent as part of a render sequence before the render_start
    /// event.
    ///
    /// It may be sent due to a propose_dimensions request in a previous manage
    /// sequence or because a window independently decides to change its
    /// dimensions.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.dimensions(width: {}, height: {})\n", client_id, id, arg0, arg1);
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

    /// window dimensions
    ///
    /// This event indicates the dimensions of the window in the compositor's
    /// logical coordinate space. The width and height must be strictly greater
    /// than zero.
    ///
    /// Note that the dimensions of a river_window_v1 refer to the dimensions of
    /// the window content and are unaffected by the presence of borders or
    /// decoration surfaces.
    ///
    /// This event is sent as part of a render sequence before the render_start
    /// event.
    ///
    /// It may be sent due to a propose_dimensions request in a previous manage
    /// sequence or because a window independently decides to change its
    /// dimensions.
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
            log_send("river_window_v1.dimensions", &e);
        }
    }

    /// Since when the propose_dimensions message is available.
    pub const MSG__PROPOSE_DIMENSIONS__SINCE: u32 = 1;

    /// propose window dimensions
    ///
    /// This request proposes dimensions for the window in the compositor's
    /// logical coordinate space.
    ///
    /// The width and height must be greater than or equal to zero. If the width
    /// or height is zero the window will be allowed to decide its own
    /// dimensions.
    ///
    /// The window may not take the exact dimensions proposed. The actual
    /// dimensions taken by the window will be sent in a subsequent
    /// river_window_v1.dimensions event. For example, a terminal emulator may
    /// only allow dimensions that are multiple of the cell size.
    ///
    /// When a propose_dimensions request is made, the server must send a
    /// dimensions event in response as soon as possible. It may not be possible
    /// to send a dimensions event in the very next render sequence if, for
    /// example, the window takes too long to respond to the first proposed
    /// dimensions. In this case, the server will send the dimensions event in a
    /// future render sequence. The window will not be displayed until the first
    /// dimensions event is received and the render sequence is finished.
    ///
    /// Note that the dimensions of a river_window_v1 refer to the dimensions of
    /// the window content and are unaffected by the presence of borders or
    /// decoration surfaces.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_propose_dimensions(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.propose_dimensions(width: {}, height: {})\n", id, arg0, arg1);
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
            3,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// propose window dimensions
    ///
    /// This request proposes dimensions for the window in the compositor's
    /// logical coordinate space.
    ///
    /// The width and height must be greater than or equal to zero. If the width
    /// or height is zero the window will be allowed to decide its own
    /// dimensions.
    ///
    /// The window may not take the exact dimensions proposed. The actual
    /// dimensions taken by the window will be sent in a subsequent
    /// river_window_v1.dimensions event. For example, a terminal emulator may
    /// only allow dimensions that are multiple of the cell size.
    ///
    /// When a propose_dimensions request is made, the server must send a
    /// dimensions event in response as soon as possible. It may not be possible
    /// to send a dimensions event in the very next render sequence if, for
    /// example, the window takes too long to respond to the first proposed
    /// dimensions. In this case, the server will send the dimensions event in a
    /// future render sequence. The window will not be displayed until the first
    /// dimensions event is received and the render sequence is finished.
    ///
    /// Note that the dimensions of a river_window_v1 refer to the dimensions of
    /// the window content and are unaffected by the presence of borders or
    /// decoration surfaces.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_propose_dimensions(
        &self,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_propose_dimensions(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("river_window_v1.propose_dimensions", &e);
        }
    }

    /// Since when the hide message is available.
    pub const MSG__HIDE__SINCE: u32 = 1;

    /// request that the window be hidden
    ///
    /// Request that the window be hidden. Has no effect if the window is
    /// already hidden. Hides any window borders and decorations as well.
    ///
    /// Newly created windows are considered shown unless explicitly hidden with
    /// the hide request.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_hide(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.hide()\n", id);
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

    /// request that the window be hidden
    ///
    /// Request that the window be hidden. Has no effect if the window is
    /// already hidden. Hides any window borders and decorations as well.
    ///
    /// Newly created windows are considered shown unless explicitly hidden with
    /// the hide request.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_hide(
        &self,
    ) {
        let res = self.try_send_hide(
        );
        if let Err(e) = res {
            log_send("river_window_v1.hide", &e);
        }
    }

    /// Since when the show message is available.
    pub const MSG__SHOW__SINCE: u32 = 1;

    /// request that the window be shown
    ///
    /// Request that the window be shown. Has no effect if the window is not
    /// hidden. Does not guarantee that the window is visible as it may be
    /// completely obscured by other windows placed above it for example.
    ///
    /// Newly created windows are considered shown unless explicitly hidden with
    /// the hide request.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_show(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.show()\n", id);
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

    /// request that the window be shown
    ///
    /// Request that the window be shown. Has no effect if the window is not
    /// hidden. Does not guarantee that the window is visible as it may be
    /// completely obscured by other windows placed above it for example.
    ///
    /// Newly created windows are considered shown unless explicitly hidden with
    /// the hide request.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_show(
        &self,
    ) {
        let res = self.try_send_show(
        );
        if let Err(e) = res {
            log_send("river_window_v1.show", &e);
        }
    }

    /// Since when the app_id message is available.
    pub const MSG__APP_ID__SINCE: u32 = 1;

    /// the window set an application ID
    ///
    /// The window set an application ID.
    ///
    /// The app_id argument will be null if the window has never set an
    /// application ID or if the window cleared its application ID. (Xwayland
    /// windows may do this for example, though xdg-toplevels may not.)
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    pub fn try_send_app_id(
        &self,
        app_id: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            app_id,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.app_id(app_id: {:?})\n", client_id, id, arg0);
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
            3,
        ]);
        if let Some(arg0) = arg0 {
            fmt.string(arg0);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// the window set an application ID
    ///
    /// The window set an application ID.
    ///
    /// The app_id argument will be null if the window has never set an
    /// application ID or if the window cleared its application ID. (Xwayland
    /// windows may do this for example, though xdg-toplevels may not.)
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    pub fn send_app_id(
        &self,
        app_id: Option<&str>,
    ) {
        let res = self.try_send_app_id(
            app_id,
        );
        if let Err(e) = res {
            log_send("river_window_v1.app_id", &e);
        }
    }

    /// Since when the title message is available.
    pub const MSG__TITLE__SINCE: u32 = 1;

    /// the window set a title
    ///
    /// The window set a title.
    ///
    /// The title argument will be null if the window has never set a title or
    /// if the window cleared its title. (Xwayland windows may do this for
    /// example, though xdg-toplevels may not.)
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    pub fn try_send_title(
        &self,
        title: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            title,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.title(title: {:?})\n", client_id, id, arg0);
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
            4,
        ]);
        if let Some(arg0) = arg0 {
            fmt.string(arg0);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// the window set a title
    ///
    /// The window set a title.
    ///
    /// The title argument will be null if the window has never set a title or
    /// if the window cleared its title. (Xwayland windows may do this for
    /// example, though xdg-toplevels may not.)
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    pub fn send_title(
        &self,
        title: Option<&str>,
    ) {
        let res = self.try_send_title(
            title,
        );
        if let Err(e) = res {
            log_send("river_window_v1.title", &e);
        }
    }

    /// Since when the parent message is available.
    pub const MSG__PARENT__SINCE: u32 = 1;

    /// the window set a parent
    ///
    /// The window set a parent window. If this event is never received or if
    /// the parent argument is null then the window has no parent.
    ///
    /// A surface with a parent set might be a dialog, file picker, or similar
    /// for the parent window.
    ///
    /// Child windows should generally be rendered directly above their parent.
    ///
    /// The compositor must guarantee that there are no loops in the window
    /// tree: a parent must not be the descendant of one of its children.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    #[inline]
    pub fn try_send_parent(
        &self,
        parent: Option<&Rc<RiverWindowV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            parent,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if let Some(arg0) = arg0 {
            if arg0.client_id.get() != Some(client.endpoint.id) {
                return Err(ObjectError(ObjectErrorKind::ArgNoClientId("parent", client.endpoint.id)));
            }
        }
        let arg0_id = arg0.and_then(|arg0| arg0.client_obj_id.get()).unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.parent(parent: river_window_v1#{})\n", client_id, id, arg0);
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

    /// the window set a parent
    ///
    /// The window set a parent window. If this event is never received or if
    /// the parent argument is null then the window has no parent.
    ///
    /// A surface with a parent set might be a dialog, file picker, or similar
    /// for the parent window.
    ///
    /// Child windows should generally be rendered directly above their parent.
    ///
    /// The compositor must guarantee that there are no loops in the window
    /// tree: a parent must not be the descendant of one of its children.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    #[inline]
    pub fn send_parent(
        &self,
        parent: Option<&Rc<RiverWindowV1>>,
    ) {
        let res = self.try_send_parent(
            parent,
        );
        if let Err(e) = res {
            log_send("river_window_v1.parent", &e);
        }
    }

    /// Since when the decoration_hint message is available.
    pub const MSG__DECORATION_HINT__SINCE: u32 = 1;

    /// supported/preferred decoration style
    ///
    /// Information from the window about the supported and preferred client
    /// side/server side decoration options.
    ///
    /// This event may be sent multiple times over the lifetime of the window if
    /// the window changes its preferences.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    #[inline]
    pub fn try_send_decoration_hint(
        &self,
        hint: RiverWindowV1DecorationHint,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            hint,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: RiverWindowV1DecorationHint) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.decoration_hint(hint: {:?})\n", client_id, id, arg0);
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
            6,
            arg0.0,
        ]);
        Ok(())
    }

    /// supported/preferred decoration style
    ///
    /// Information from the window about the supported and preferred client
    /// side/server side decoration options.
    ///
    /// This event may be sent multiple times over the lifetime of the window if
    /// the window changes its preferences.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    #[inline]
    pub fn send_decoration_hint(
        &self,
        hint: RiverWindowV1DecorationHint,
    ) {
        let res = self.try_send_decoration_hint(
            hint,
        );
        if let Err(e) = res {
            log_send("river_window_v1.decoration_hint", &e);
        }
    }

    /// Since when the use_csd message is available.
    pub const MSG__USE_CSD__SINCE: u32 = 1;

    /// tell the client to use CSD
    ///
    /// Tell the client to use client side decoration and draw its own title
    /// bar, borders, etc.
    ///
    /// This is the default if neither this request nor the use_ssd request is
    /// ever made.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_use_csd(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.use_csd()\n", id);
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
            6,
        ]);
        Ok(())
    }

    /// tell the client to use CSD
    ///
    /// Tell the client to use client side decoration and draw its own title
    /// bar, borders, etc.
    ///
    /// This is the default if neither this request nor the use_ssd request is
    /// ever made.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_use_csd(
        &self,
    ) {
        let res = self.try_send_use_csd(
        );
        if let Err(e) = res {
            log_send("river_window_v1.use_csd", &e);
        }
    }

    /// Since when the use_ssd message is available.
    pub const MSG__USE_SSD__SINCE: u32 = 1;

    /// tell the client to use SSD
    ///
    /// Tell the client to use server side decoration and not draw any client
    /// side decorations.
    ///
    /// This request will have no effect if the client only supports client side
    /// decoration, see the decoration_hint event.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_use_ssd(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.use_ssd()\n", id);
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
            7,
        ]);
        Ok(())
    }

    /// tell the client to use SSD
    ///
    /// Tell the client to use server side decoration and not draw any client
    /// side decorations.
    ///
    /// This request will have no effect if the client only supports client side
    /// decoration, see the decoration_hint event.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_use_ssd(
        &self,
    ) {
        let res = self.try_send_use_ssd(
        );
        if let Err(e) = res {
            log_send("river_window_v1.use_ssd", &e);
        }
    }

    /// Since when the set_borders message is available.
    pub const MSG__SET_BORDERS__SINCE: u32 = 1;

    /// set window borders
    ///
    /// This request decorates the window with borders drawn by the compositor
    /// on the specified edges of the window. Borders are drawn above the window
    /// content.
    ///
    /// Corners are drawn only between borders on adjacent edges. If e.g. the
    /// left edge has a border and the top edge does not, the border drawn on
    /// the left edge will not extend vertically beyond the top edge of the
    /// window.
    ///
    /// Borders are not drawn while the window is fullscreen.
    ///
    /// The color is defined by four 32-bit RGBA values. Unless specified in
    /// another protocol extension, the RGBA values use pre-multiplied alpha.
    ///
    /// Setting the edges to none or the width to 0 disables the borders.
    /// Setting a negative width is a protocol error.
    ///
    /// This request completely overrides all previous set_borders requests.
    /// Only the most recent set_borders request has an effect.
    ///
    /// Note that the position/dimensions of a river_window_v1 refer to the
    /// position/dimensions of the window content and are unaffected by the
    /// presence of borders or decoration surfaces.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `edges`:
    /// - `width`:
    /// - `r`:
    /// - `g`:
    /// - `b`:
    /// - `a`:
    #[inline]
    pub fn try_send_set_borders(
        &self,
        edges: RiverWindowV1Edges,
        width: i32,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            edges,
            width,
            r,
            g,
            b,
            a,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: RiverWindowV1Edges, arg1: i32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.set_borders(edges: {:?}, width: {}, r: {}, g: {}, b: {}, a: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3, arg4, arg5);
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
            arg0.0,
            arg1 as u32,
            arg2,
            arg3,
            arg4,
            arg5,
        ]);
        Ok(())
    }

    /// set window borders
    ///
    /// This request decorates the window with borders drawn by the compositor
    /// on the specified edges of the window. Borders are drawn above the window
    /// content.
    ///
    /// Corners are drawn only between borders on adjacent edges. If e.g. the
    /// left edge has a border and the top edge does not, the border drawn on
    /// the left edge will not extend vertically beyond the top edge of the
    /// window.
    ///
    /// Borders are not drawn while the window is fullscreen.
    ///
    /// The color is defined by four 32-bit RGBA values. Unless specified in
    /// another protocol extension, the RGBA values use pre-multiplied alpha.
    ///
    /// Setting the edges to none or the width to 0 disables the borders.
    /// Setting a negative width is a protocol error.
    ///
    /// This request completely overrides all previous set_borders requests.
    /// Only the most recent set_borders request has an effect.
    ///
    /// Note that the position/dimensions of a river_window_v1 refer to the
    /// position/dimensions of the window content and are unaffected by the
    /// presence of borders or decoration surfaces.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `edges`:
    /// - `width`:
    /// - `r`:
    /// - `g`:
    /// - `b`:
    /// - `a`:
    #[inline]
    pub fn send_set_borders(
        &self,
        edges: RiverWindowV1Edges,
        width: i32,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) {
        let res = self.try_send_set_borders(
            edges,
            width,
            r,
            g,
            b,
            a,
        );
        if let Err(e) = res {
            log_send("river_window_v1.set_borders", &e);
        }
    }

    /// Since when the set_tiled message is available.
    pub const MSG__SET_TILED__SINCE: u32 = 1;

    /// set window tiled state
    ///
    /// Inform the window that it is part of a tiled layout and adjacent to
    /// other elements in the tiled layout on the given edges.
    ///
    /// The window should use this information to change the style of its client
    /// side decorations and avoid drawing e.g. drop shadows outside of the
    /// window dimensions on the tiled edges.
    ///
    /// Setting the edges argument to none informs the window that it is not
    /// part of a tiled layout. If this request is never made, the window is
    /// informed that it is not part of a tiled layout.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `edges`:
    #[inline]
    pub fn try_send_set_tiled(
        &self,
        edges: RiverWindowV1Edges,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            edges,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: RiverWindowV1Edges) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.set_tiled(edges: {:?})\n", id, arg0);
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
            9,
            arg0.0,
        ]);
        Ok(())
    }

    /// set window tiled state
    ///
    /// Inform the window that it is part of a tiled layout and adjacent to
    /// other elements in the tiled layout on the given edges.
    ///
    /// The window should use this information to change the style of its client
    /// side decorations and avoid drawing e.g. drop shadows outside of the
    /// window dimensions on the tiled edges.
    ///
    /// Setting the edges argument to none informs the window that it is not
    /// part of a tiled layout. If this request is never made, the window is
    /// informed that it is not part of a tiled layout.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `edges`:
    #[inline]
    pub fn send_set_tiled(
        &self,
        edges: RiverWindowV1Edges,
    ) {
        let res = self.try_send_set_tiled(
            edges,
        );
        if let Err(e) = res {
            log_send("river_window_v1.set_tiled", &e);
        }
    }

    /// Since when the get_decoration_above message is available.
    pub const MSG__GET_DECORATION_ABOVE__SINCE: u32 = 1;

    /// create a decoration surface above the window
    ///
    /// Create a decoration surface and assign the river_decoration_v1 role to
    /// the surface. The created decoration is placed above the window in
    /// rendering order, see the description of river_decoration_v1.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn try_send_get_decoration_above(
        &self,
        id: &Rc<RiverDecorationV1>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.get_decoration_above(id: river_decoration_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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
            10,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// create a decoration surface above the window
    ///
    /// Create a decoration surface and assign the river_decoration_v1 role to
    /// the surface. The created decoration is placed above the window in
    /// rendering order, see the description of river_decoration_v1.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_decoration_above(
        &self,
        id: &Rc<RiverDecorationV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_decoration_above(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("river_window_v1.get_decoration_above", &e);
        }
    }

    /// create a decoration surface above the window
    ///
    /// Create a decoration surface and assign the river_decoration_v1 role to
    /// the surface. The created decoration is placed above the window in
    /// rendering order, see the description of river_decoration_v1.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_try_send_get_decoration_above(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<RiverDecorationV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_decoration_above(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// create a decoration surface above the window
    ///
    /// Create a decoration surface and assign the river_decoration_v1 role to
    /// the surface. The created decoration is placed above the window in
    /// rendering order, see the description of river_decoration_v1.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_send_get_decoration_above(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<RiverDecorationV1> {
        let id = self.core.create_child();
        self.send_get_decoration_above(
            &id,
            surface,
        );
        id
    }

    /// Since when the get_decoration_below message is available.
    pub const MSG__GET_DECORATION_BELOW__SINCE: u32 = 1;

    /// create a decoration surface below the window
    ///
    /// Create a decoration surface and assign the river_decoration_v1 role to
    /// the surface. The created decoration is placed below the window in
    /// rendering order, see the description of river_decoration_v1.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn try_send_get_decoration_below(
        &self,
        id: &Rc<RiverDecorationV1>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.get_decoration_below(id: river_decoration_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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
            11,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// create a decoration surface below the window
    ///
    /// Create a decoration surface and assign the river_decoration_v1 role to
    /// the surface. The created decoration is placed below the window in
    /// rendering order, see the description of river_decoration_v1.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_decoration_below(
        &self,
        id: &Rc<RiverDecorationV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_decoration_below(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("river_window_v1.get_decoration_below", &e);
        }
    }

    /// create a decoration surface below the window
    ///
    /// Create a decoration surface and assign the river_decoration_v1 role to
    /// the surface. The created decoration is placed below the window in
    /// rendering order, see the description of river_decoration_v1.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_try_send_get_decoration_below(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<RiverDecorationV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_decoration_below(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// create a decoration surface below the window
    ///
    /// Create a decoration surface and assign the river_decoration_v1 role to
    /// the surface. The created decoration is placed below the window in
    /// rendering order, see the description of river_decoration_v1.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_send_get_decoration_below(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<RiverDecorationV1> {
        let id = self.core.create_child();
        self.send_get_decoration_below(
            &id,
            surface,
        );
        id
    }

    /// Since when the pointer_move_requested message is available.
    pub const MSG__POINTER_MOVE_REQUESTED__SINCE: u32 = 1;

    /// window requested interactive pointer move
    ///
    /// This event informs the window manager that the window has requested to
    /// be interactively moved using the pointer. The seat argument indicates the
    /// seat for the move.
    ///
    /// The xdg-shell protocol for example allows windows to request that an
    /// interactive move be started, perhaps when a client-side rendered
    /// titlebar is dragged.
    ///
    /// The window manager may use the river_seat_v1.op_start_pointer request to
    /// interactively move the window or ignore this event entirely.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn try_send_pointer_move_requested(
        &self,
        seat: &Rc<RiverSeatV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            seat,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("seat", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.pointer_move_requested(seat: river_seat_v1#{})\n", client_id, id, arg0);
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
            7,
            arg0_id,
        ]);
        Ok(())
    }

    /// window requested interactive pointer move
    ///
    /// This event informs the window manager that the window has requested to
    /// be interactively moved using the pointer. The seat argument indicates the
    /// seat for the move.
    ///
    /// The xdg-shell protocol for example allows windows to request that an
    /// interactive move be started, perhaps when a client-side rendered
    /// titlebar is dragged.
    ///
    /// The window manager may use the river_seat_v1.op_start_pointer request to
    /// interactively move the window or ignore this event entirely.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn send_pointer_move_requested(
        &self,
        seat: &Rc<RiverSeatV1>,
    ) {
        let res = self.try_send_pointer_move_requested(
            seat,
        );
        if let Err(e) = res {
            log_send("river_window_v1.pointer_move_requested", &e);
        }
    }

    /// Since when the pointer_resize_requested message is available.
    pub const MSG__POINTER_RESIZE_REQUESTED__SINCE: u32 = 1;

    /// window requested interactive pointer resize
    ///
    /// This event informs the window manager that the window has requested to
    /// be interactively resized using the pointer. The seat argument indicates
    /// the seat for the resize.
    ///
    /// The edges argument indicates which edges the window has requested to be
    /// resized from. The edges argument will never be none and will never have
    /// both top and bottom or both left and right edges set.
    ///
    /// The xdg-shell protocol for example allows windows to request that an
    /// interactive resize be started, perhaps when the corner of client-side
    /// rendered decorations is dragged.
    ///
    /// The window manager may use the river_seat_v1.op_start_pointer request to
    /// interactively resize the window or ignore this event entirely.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `edges`:
    #[inline]
    pub fn try_send_pointer_resize_requested(
        &self,
        seat: &Rc<RiverSeatV1>,
        edges: RiverWindowV1Edges,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            seat,
            edges,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("seat", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: RiverWindowV1Edges) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.pointer_resize_requested(seat: river_seat_v1#{}, edges: {:?})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id, arg1);
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
            arg0_id,
            arg1.0,
        ]);
        Ok(())
    }

    /// window requested interactive pointer resize
    ///
    /// This event informs the window manager that the window has requested to
    /// be interactively resized using the pointer. The seat argument indicates
    /// the seat for the resize.
    ///
    /// The edges argument indicates which edges the window has requested to be
    /// resized from. The edges argument will never be none and will never have
    /// both top and bottom or both left and right edges set.
    ///
    /// The xdg-shell protocol for example allows windows to request that an
    /// interactive resize be started, perhaps when the corner of client-side
    /// rendered decorations is dragged.
    ///
    /// The window manager may use the river_seat_v1.op_start_pointer request to
    /// interactively resize the window or ignore this event entirely.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `edges`:
    #[inline]
    pub fn send_pointer_resize_requested(
        &self,
        seat: &Rc<RiverSeatV1>,
        edges: RiverWindowV1Edges,
    ) {
        let res = self.try_send_pointer_resize_requested(
            seat,
            edges,
        );
        if let Err(e) = res {
            log_send("river_window_v1.pointer_resize_requested", &e);
        }
    }

    /// Since when the inform_resize_start message is available.
    pub const MSG__INFORM_RESIZE_START__SINCE: u32 = 1;

    /// inform the window it is being resized
    ///
    /// Inform the window that it is being resized. The window manager should
    /// use this request to inform windows that are the target of an interactive
    /// resize for example.
    ///
    /// The window manager remains responsible for handling the position and
    /// dimensions of the window while it is resizing.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_inform_resize_start(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.inform_resize_start()\n", id);
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
            12,
        ]);
        Ok(())
    }

    /// inform the window it is being resized
    ///
    /// Inform the window that it is being resized. The window manager should
    /// use this request to inform windows that are the target of an interactive
    /// resize for example.
    ///
    /// The window manager remains responsible for handling the position and
    /// dimensions of the window while it is resizing.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_inform_resize_start(
        &self,
    ) {
        let res = self.try_send_inform_resize_start(
        );
        if let Err(e) = res {
            log_send("river_window_v1.inform_resize_start", &e);
        }
    }

    /// Since when the inform_resize_end message is available.
    pub const MSG__INFORM_RESIZE_END__SINCE: u32 = 1;

    /// inform the window it no longer being resized
    ///
    /// Inform the window that it is no longer being resized. The window manager
    /// should use this request to inform windows that are the target of an
    /// interactive resize that the interactive resize has ended for example.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_inform_resize_end(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.inform_resize_end()\n", id);
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
            13,
        ]);
        Ok(())
    }

    /// inform the window it no longer being resized
    ///
    /// Inform the window that it is no longer being resized. The window manager
    /// should use this request to inform windows that are the target of an
    /// interactive resize that the interactive resize has ended for example.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_inform_resize_end(
        &self,
    ) {
        let res = self.try_send_inform_resize_end(
        );
        if let Err(e) = res {
            log_send("river_window_v1.inform_resize_end", &e);
        }
    }

    /// Since when the set_capabilities message is available.
    pub const MSG__SET_CAPABILITIES__SINCE: u32 = 1;

    /// inform window of supported capabilities
    ///
    /// This request informs the window of the capabilities supported by the
    /// window manager. If the window manager, for example, ignores requests to
    /// be maximized from the window it should not tell the window that it
    /// supports the maximize capability.
    ///
    /// The window might use this information to, for example, only show a
    /// maximize button if the window manager supports the maximize capability.
    ///
    /// The window manager client should use this request to set capabilities
    /// for all new windows. If this request is never made, the compositor will
    /// inform windows that all capabilities are supported.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `caps`:
    #[inline]
    pub fn try_send_set_capabilities(
        &self,
        caps: RiverWindowV1Capabilities,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            caps,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: RiverWindowV1Capabilities) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.set_capabilities(caps: {:?})\n", id, arg0);
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
            14,
            arg0.0,
        ]);
        Ok(())
    }

    /// inform window of supported capabilities
    ///
    /// This request informs the window of the capabilities supported by the
    /// window manager. If the window manager, for example, ignores requests to
    /// be maximized from the window it should not tell the window that it
    /// supports the maximize capability.
    ///
    /// The window might use this information to, for example, only show a
    /// maximize button if the window manager supports the maximize capability.
    ///
    /// The window manager client should use this request to set capabilities
    /// for all new windows. If this request is never made, the compositor will
    /// inform windows that all capabilities are supported.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `caps`:
    #[inline]
    pub fn send_set_capabilities(
        &self,
        caps: RiverWindowV1Capabilities,
    ) {
        let res = self.try_send_set_capabilities(
            caps,
        );
        if let Err(e) = res {
            log_send("river_window_v1.set_capabilities", &e);
        }
    }

    /// Since when the show_window_menu_requested message is available.
    pub const MSG__SHOW_WINDOW_MENU_REQUESTED__SINCE: u32 = 1;

    /// window requested that the window menu be shown
    ///
    /// The xdg-shell protocol for example allows windows to request that a
    /// window menu be shown, for example when the user right clicks on client
    /// side window decorations.
    ///
    /// A window menu might include options to maximize or minimize the window.
    ///
    /// The window manager is free to ignore this request and decide what the
    /// window menu contains if it does choose to show one.
    ///
    /// The x and y arguments indicate where the window requested that the
    /// window menu be shown.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `x`: x offset from top left corner
    /// - `y`: y offset from top left corner
    #[inline]
    pub fn try_send_show_window_menu_requested(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.show_window_menu_requested(x: {}, y: {})\n", client_id, id, arg0, arg1);
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
            9,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// window requested that the window menu be shown
    ///
    /// The xdg-shell protocol for example allows windows to request that a
    /// window menu be shown, for example when the user right clicks on client
    /// side window decorations.
    ///
    /// A window menu might include options to maximize or minimize the window.
    ///
    /// The window manager is free to ignore this request and decide what the
    /// window menu contains if it does choose to show one.
    ///
    /// The x and y arguments indicate where the window requested that the
    /// window menu be shown.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `x`: x offset from top left corner
    /// - `y`: y offset from top left corner
    #[inline]
    pub fn send_show_window_menu_requested(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_show_window_menu_requested(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("river_window_v1.show_window_menu_requested", &e);
        }
    }

    /// Since when the maximize_requested message is available.
    pub const MSG__MAXIMIZE_REQUESTED__SINCE: u32 = 1;

    /// the window requested to be maximized
    ///
    /// The xdg-shell protocol for example allows windows to request to be
    /// maximized.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.inform_maximize or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_maximize_requested(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.maximize_requested()\n", client_id, id);
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
            10,
        ]);
        Ok(())
    }

    /// the window requested to be maximized
    ///
    /// The xdg-shell protocol for example allows windows to request to be
    /// maximized.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.inform_maximize or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_maximize_requested(
        &self,
    ) {
        let res = self.try_send_maximize_requested(
        );
        if let Err(e) = res {
            log_send("river_window_v1.maximize_requested", &e);
        }
    }

    /// Since when the unmaximize_requested message is available.
    pub const MSG__UNMAXIMIZE_REQUESTED__SINCE: u32 = 1;

    /// the window requested to be unmaximized
    ///
    /// The xdg-shell protocol for example allows windows to request to be
    /// unmaximized.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.inform_unmaximized or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_unmaximize_requested(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.unmaximize_requested()\n", client_id, id);
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
            11,
        ]);
        Ok(())
    }

    /// the window requested to be unmaximized
    ///
    /// The xdg-shell protocol for example allows windows to request to be
    /// unmaximized.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.inform_unmaximized or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_unmaximize_requested(
        &self,
    ) {
        let res = self.try_send_unmaximize_requested(
        );
        if let Err(e) = res {
            log_send("river_window_v1.unmaximize_requested", &e);
        }
    }

    /// Since when the inform_maximized message is available.
    pub const MSG__INFORM_MAXIMIZED__SINCE: u32 = 1;

    /// inform the window that it is maximized
    ///
    /// Inform the window that it is maximized. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// The window manager remains responsible for handling the position and
    /// dimensions of the window while it is maximized.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_inform_maximized(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.inform_maximized()\n", id);
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
            15,
        ]);
        Ok(())
    }

    /// inform the window that it is maximized
    ///
    /// Inform the window that it is maximized. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// The window manager remains responsible for handling the position and
    /// dimensions of the window while it is maximized.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_inform_maximized(
        &self,
    ) {
        let res = self.try_send_inform_maximized(
        );
        if let Err(e) = res {
            log_send("river_window_v1.inform_maximized", &e);
        }
    }

    /// Since when the inform_unmaximized message is available.
    pub const MSG__INFORM_UNMAXIMIZED__SINCE: u32 = 1;

    /// inform the window that it is unmaximized
    ///
    /// Inform the window that it is unmaximized. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_inform_unmaximized(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.inform_unmaximized()\n", id);
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
            16,
        ]);
        Ok(())
    }

    /// inform the window that it is unmaximized
    ///
    /// Inform the window that it is unmaximized. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_inform_unmaximized(
        &self,
    ) {
        let res = self.try_send_inform_unmaximized(
        );
        if let Err(e) = res {
            log_send("river_window_v1.inform_unmaximized", &e);
        }
    }

    /// Since when the fullscreen_requested message is available.
    pub const MSG__FULLSCREEN_REQUESTED__SINCE: u32 = 1;

    /// the window requested to be fullscreen
    ///
    /// The xdg-shell protocol for example allows windows to request that they
    /// be made fullscreen and allows them to provide an output preference.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.fullscreen or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn try_send_fullscreen_requested(
        &self,
        output: Option<&Rc<RiverOutputV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if let Some(arg0) = arg0 {
            if arg0.client_id.get() != Some(client.endpoint.id) {
                return Err(ObjectError(ObjectErrorKind::ArgNoClientId("output", client.endpoint.id)));
            }
        }
        let arg0_id = arg0.and_then(|arg0| arg0.client_obj_id.get()).unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.fullscreen_requested(output: river_output_v1#{})\n", client_id, id, arg0);
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
            12,
            arg0_id,
        ]);
        Ok(())
    }

    /// the window requested to be fullscreen
    ///
    /// The xdg-shell protocol for example allows windows to request that they
    /// be made fullscreen and allows them to provide an output preference.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.fullscreen or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn send_fullscreen_requested(
        &self,
        output: Option<&Rc<RiverOutputV1>>,
    ) {
        let res = self.try_send_fullscreen_requested(
            output,
        );
        if let Err(e) = res {
            log_send("river_window_v1.fullscreen_requested", &e);
        }
    }

    /// Since when the exit_fullscreen_requested message is available.
    pub const MSG__EXIT_FULLSCREEN_REQUESTED__SINCE: u32 = 1;

    /// the window requested to exit fullscreen
    ///
    /// The xdg-shell protocol for example allows windows to request to exit
    /// fullscreen.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.exit_fullscreen or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_exit_fullscreen_requested(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.exit_fullscreen_requested()\n", client_id, id);
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
            13,
        ]);
        Ok(())
    }

    /// the window requested to exit fullscreen
    ///
    /// The xdg-shell protocol for example allows windows to request to exit
    /// fullscreen.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.exit_fullscreen or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_exit_fullscreen_requested(
        &self,
    ) {
        let res = self.try_send_exit_fullscreen_requested(
        );
        if let Err(e) = res {
            log_send("river_window_v1.exit_fullscreen_requested", &e);
        }
    }

    /// Since when the inform_fullscreen message is available.
    pub const MSG__INFORM_FULLSCREEN__SINCE: u32 = 1;

    /// inform the window that it is fullscreen
    ///
    /// Inform the window that it is fullscreen. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// This request does not affect the size/position of the window or cause it
    /// to become the only window rendered, see the river_window_v1.fullscreen
    /// and exit_fullscreen requests for that.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_inform_fullscreen(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.inform_fullscreen()\n", id);
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
            17,
        ]);
        Ok(())
    }

    /// inform the window that it is fullscreen
    ///
    /// Inform the window that it is fullscreen. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// This request does not affect the size/position of the window or cause it
    /// to become the only window rendered, see the river_window_v1.fullscreen
    /// and exit_fullscreen requests for that.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_inform_fullscreen(
        &self,
    ) {
        let res = self.try_send_inform_fullscreen(
        );
        if let Err(e) = res {
            log_send("river_window_v1.inform_fullscreen", &e);
        }
    }

    /// Since when the inform_not_fullscreen message is available.
    pub const MSG__INFORM_NOT_FULLSCREEN__SINCE: u32 = 1;

    /// inform the window that it is not fullscreen
    ///
    /// Inform the window that it is not fullscreen. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// This request does not affect the size/position of the window or cause it
    /// to become the only window rendered, see the river_window_v1.fullscreen
    /// and exit_fullscreen requests for that.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_inform_not_fullscreen(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.inform_not_fullscreen()\n", id);
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
            18,
        ]);
        Ok(())
    }

    /// inform the window that it is not fullscreen
    ///
    /// Inform the window that it is not fullscreen. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// This request does not affect the size/position of the window or cause it
    /// to become the only window rendered, see the river_window_v1.fullscreen
    /// and exit_fullscreen requests for that.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_inform_not_fullscreen(
        &self,
    ) {
        let res = self.try_send_inform_not_fullscreen(
        );
        if let Err(e) = res {
            log_send("river_window_v1.inform_not_fullscreen", &e);
        }
    }

    /// Since when the fullscreen message is available.
    pub const MSG__FULLSCREEN__SINCE: u32 = 1;

    /// make the window fullscreen
    ///
    /// Make the window fullscreen on the given output. If multiple windows are
    /// fullscreen on the same output at the same time only the "top" window in
    /// rendering order shall be displayed.
    ///
    /// All river_shell_surface_v1 objects above the top fullscreen window in
    /// the rendering order will continue to be rendered.
    ///
    /// The compositor will handle the position and dimensions of the window
    /// while it is fullscreen. The set_position and propose_dimensions requests
    /// shall not affect the current position and dimensions of a fullscreen
    /// window.
    ///
    /// The compositor will clip window content, decoration surfaces, and
    /// borders to the given output's dimensions while the window is fullscreen.
    /// The effects of set_clip_box and set_content_clip_box are ignored while
    /// the window is fullscreen.
    ///
    /// If the output on which a window is currently fullscreen is removed, the
    /// windowing state is modified as if there were an exit_fullscreen request
    /// made in the same manage sequence as the river_output_v1.removed event.
    ///
    /// This request does not inform the window that it is fullscreen, see the
    /// river_window_v1.inform_fullscreen and inform_not_fullscreen requests.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn try_send_fullscreen(
        &self,
        output: &Rc<RiverOutputV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.fullscreen(output: river_output_v1#{})\n", id, arg0);
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
            19,
            arg0_id,
        ]);
        Ok(())
    }

    /// make the window fullscreen
    ///
    /// Make the window fullscreen on the given output. If multiple windows are
    /// fullscreen on the same output at the same time only the "top" window in
    /// rendering order shall be displayed.
    ///
    /// All river_shell_surface_v1 objects above the top fullscreen window in
    /// the rendering order will continue to be rendered.
    ///
    /// The compositor will handle the position and dimensions of the window
    /// while it is fullscreen. The set_position and propose_dimensions requests
    /// shall not affect the current position and dimensions of a fullscreen
    /// window.
    ///
    /// The compositor will clip window content, decoration surfaces, and
    /// borders to the given output's dimensions while the window is fullscreen.
    /// The effects of set_clip_box and set_content_clip_box are ignored while
    /// the window is fullscreen.
    ///
    /// If the output on which a window is currently fullscreen is removed, the
    /// windowing state is modified as if there were an exit_fullscreen request
    /// made in the same manage sequence as the river_output_v1.removed event.
    ///
    /// This request does not inform the window that it is fullscreen, see the
    /// river_window_v1.inform_fullscreen and inform_not_fullscreen requests.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn send_fullscreen(
        &self,
        output: &Rc<RiverOutputV1>,
    ) {
        let res = self.try_send_fullscreen(
            output,
        );
        if let Err(e) = res {
            log_send("river_window_v1.fullscreen", &e);
        }
    }

    /// Since when the exit_fullscreen message is available.
    pub const MSG__EXIT_FULLSCREEN__SINCE: u32 = 1;

    /// make the window not fullscreen
    ///
    /// Make the window not fullscreen.
    ///
    /// The position and dimensions are undefined after this request is made
    /// until a manage sequence in which the window manager makes the
    /// propose_dimensions and set_position requests is completed.
    ///
    /// The window manager should make propose_dimensions and set_position
    /// requests in the same manage sequence as the exit_fullscreen request for
    /// frame perfection.
    ///
    /// This request does not inform the window that it is fullscreen, see the
    /// river_window_v1.inform_fullscreen and inform_not_fullscreen requests.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_exit_fullscreen(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.exit_fullscreen()\n", id);
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
            20,
        ]);
        Ok(())
    }

    /// make the window not fullscreen
    ///
    /// Make the window not fullscreen.
    ///
    /// The position and dimensions are undefined after this request is made
    /// until a manage sequence in which the window manager makes the
    /// propose_dimensions and set_position requests is completed.
    ///
    /// The window manager should make propose_dimensions and set_position
    /// requests in the same manage sequence as the exit_fullscreen request for
    /// frame perfection.
    ///
    /// This request does not inform the window that it is fullscreen, see the
    /// river_window_v1.inform_fullscreen and inform_not_fullscreen requests.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_exit_fullscreen(
        &self,
    ) {
        let res = self.try_send_exit_fullscreen(
        );
        if let Err(e) = res {
            log_send("river_window_v1.exit_fullscreen", &e);
        }
    }

    /// Since when the minimize_requested message is available.
    pub const MSG__MINIMIZE_REQUESTED__SINCE: u32 = 1;

    /// the window requested to be minimized
    ///
    /// The xdg-shell protocol for example allows windows to request to be
    /// minimized.
    ///
    /// The window manager is free to ignore this request, hide the window, or
    /// do whatever else it chooses.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_minimize_requested(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.minimize_requested()\n", client_id, id);
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
            14,
        ]);
        Ok(())
    }

    /// the window requested to be minimized
    ///
    /// The xdg-shell protocol for example allows windows to request to be
    /// minimized.
    ///
    /// The window manager is free to ignore this request, hide the window, or
    /// do whatever else it chooses.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_minimize_requested(
        &self,
    ) {
        let res = self.try_send_minimize_requested(
        );
        if let Err(e) = res {
            log_send("river_window_v1.minimize_requested", &e);
        }
    }

    /// Since when the set_clip_box message is available.
    pub const MSG__SET_CLIP_BOX__SINCE: u32 = 2;

    /// clip the window to a given box
    ///
    /// Clip the window, including borders and decoration surfaces, to the box
    /// specified by the x, y, width, and height arguments. The x/y position of
    /// the box is relative to the top left corner of the window.
    ///
    /// The width and height arguments must be greater than or equal to 0.
    ///
    /// Setting a clip box with 0 width or height disables clipping.
    ///
    /// The clip box is ignored while the window is fullscreen.
    ///
    /// Both set_clip_box and set_content_clip_box may be enabled simultaneously.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_set_clip_box(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.set_clip_box(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3);
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
            21,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// clip the window to a given box
    ///
    /// Clip the window, including borders and decoration surfaces, to the box
    /// specified by the x, y, width, and height arguments. The x/y position of
    /// the box is relative to the top left corner of the window.
    ///
    /// The width and height arguments must be greater than or equal to 0.
    ///
    /// Setting a clip box with 0 width or height disables clipping.
    ///
    /// The clip box is ignored while the window is fullscreen.
    ///
    /// Both set_clip_box and set_content_clip_box may be enabled simultaneously.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_set_clip_box(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_set_clip_box(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("river_window_v1.set_clip_box", &e);
        }
    }

    /// Since when the unreliable_pid message is available.
    pub const MSG__UNRELIABLE_PID__SINCE: u32 = 2;

    /// unreliable PID of the window's creator
    ///
    /// This event gives an unreliable PID of the process that created the
    /// window. Obtaining this information is inherently racy due to PID reuse.
    /// Therefore, this PID must not be used for anything security sensitive.
    ///
    /// Note also that a single process may create multiple windows, so there is
    /// not necessarily a 1-to-1 mapping from PID to window. Multiple windows
    /// may have the same PID.
    ///
    /// This event is sent once when the river_window_v1 is created and never
    /// sent again.
    ///
    /// # Arguments
    ///
    /// - `unreliable_pid`:
    #[inline]
    pub fn try_send_unreliable_pid(
        &self,
        unreliable_pid: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            unreliable_pid,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_v1#{}.unreliable_pid(unreliable_pid: {})\n", client_id, id, arg0);
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
            15,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// unreliable PID of the window's creator
    ///
    /// This event gives an unreliable PID of the process that created the
    /// window. Obtaining this information is inherently racy due to PID reuse.
    /// Therefore, this PID must not be used for anything security sensitive.
    ///
    /// Note also that a single process may create multiple windows, so there is
    /// not necessarily a 1-to-1 mapping from PID to window. Multiple windows
    /// may have the same PID.
    ///
    /// This event is sent once when the river_window_v1 is created and never
    /// sent again.
    ///
    /// # Arguments
    ///
    /// - `unreliable_pid`:
    #[inline]
    pub fn send_unreliable_pid(
        &self,
        unreliable_pid: i32,
    ) {
        let res = self.try_send_unreliable_pid(
            unreliable_pid,
        );
        if let Err(e) = res {
            log_send("river_window_v1.unreliable_pid", &e);
        }
    }

    /// Since when the set_content_clip_box message is available.
    pub const MSG__SET_CONTENT_CLIP_BOX__SINCE: u32 = 3;

    /// clip the window content to a given box
    ///
    /// Clip the content of the window, excluding borders and decoration
    /// surfaces, to the box specified by the x, y, width, and height arguments.
    /// The x/y position of the box is relative to the top left corner of the
    /// window.
    ///
    /// Borders drawn by the compositor (see set_borders) are placed around the
    /// intersection of the window content (as defined by the dimensions event)
    /// and the content clip box when content clipping is enabled.
    ///
    /// The width and height arguments must be greater than or equal to 0.
    ///
    /// Setting a box with 0 width or height disables content clipping.
    ///
    /// The content clip box is ignored while the window is fullscreen.
    ///
    /// Both set_clip_box and set_content_clip_box may be enabled simultaneously.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_set_content_clip_box(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_v1#{}.set_content_clip_box(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3);
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
            22,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// clip the window content to a given box
    ///
    /// Clip the content of the window, excluding borders and decoration
    /// surfaces, to the box specified by the x, y, width, and height arguments.
    /// The x/y position of the box is relative to the top left corner of the
    /// window.
    ///
    /// Borders drawn by the compositor (see set_borders) are placed around the
    /// intersection of the window content (as defined by the dimensions event)
    /// and the content clip box when content clipping is enabled.
    ///
    /// The width and height arguments must be greater than or equal to 0.
    ///
    /// Setting a box with 0 width or height disables content clipping.
    ///
    /// The content clip box is ignored while the window is fullscreen.
    ///
    /// Both set_clip_box and set_content_clip_box may be enabled simultaneously.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_set_content_clip_box(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_set_content_clip_box(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("river_window_v1.set_content_clip_box", &e);
        }
    }
}

/// A message handler for [`RiverWindowV1`] proxies.
pub trait RiverWindowV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverWindowV1>) {
        slf.core.delete_id();
    }

    /// destroy the window object
    ///
    /// This request indicates that the client will no longer use the window
    /// object and that it may be safely destroyed.
    ///
    /// This request should be made after the river_window_v1.closed event or
    /// river_window_manager_v1.finished is received to complete destruction of
    /// the window.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.destroy", &e);
        }
    }

    /// the window has been closed
    ///
    /// The window has been closed by the server, perhaps due to an
    /// xdg_toplevel.close request or similar.
    ///
    /// The server will send no further events on this object and ignore any
    /// request other than river_window_v1.destroy made after this event is
    /// sent. The client should destroy this object with the
    /// river_window_v1.destroy request to free up resources.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_closed(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_closed(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.closed", &e);
        }
    }

    /// request that the window be closed
    ///
    /// Request that the window be closed. The window may ignore this request or
    /// only close after some delay, perhaps opening a dialog asking the user to
    /// save their work or similar.
    ///
    /// The server will send a river_window_v1.closed event if/when the window
    /// has been closed.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_close(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_close(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.close", &e);
        }
    }

    /// get the window's render list node
    ///
    /// Get the node in the render list corresponding to the window.
    ///
    /// It is a protocol error to make this request more than once for a single
    /// window.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_get_node(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        id: &Rc<RiverNodeV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_node(
            id,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.get_node", &e);
        }
    }

    /// the window's preferred min/max dimensions
    ///
    /// This event informs the window manager of the window's preferred min/max
    /// dimensions. These preferences are a hint, and the window manager is free
    /// to propose dimensions outside of these bounds.
    ///
    /// All min/max width/height values must be strictly greater than or equal
    /// to 0. A value of 0 indicates that the window has no preference for that
    /// value.
    ///
    /// The min_width/min_height must be strictly less than or equal to the
    /// max_width/max_height.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `min_width`:
    /// - `min_height`:
    /// - `max_width`:
    /// - `max_height`:
    #[inline]
    fn handle_dimensions_hint(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        min_width: i32,
        min_height: i32,
        max_width: i32,
        max_height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dimensions_hint(
            min_width,
            min_height,
            max_width,
            max_height,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.dimensions_hint", &e);
        }
    }

    /// window dimensions
    ///
    /// This event indicates the dimensions of the window in the compositor's
    /// logical coordinate space. The width and height must be strictly greater
    /// than zero.
    ///
    /// Note that the dimensions of a river_window_v1 refer to the dimensions of
    /// the window content and are unaffected by the presence of borders or
    /// decoration surfaces.
    ///
    /// This event is sent as part of a render sequence before the render_start
    /// event.
    ///
    /// It may be sent due to a propose_dimensions request in a previous manage
    /// sequence or because a window independently decides to change its
    /// dimensions.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_dimensions(
        &mut self,
        slf: &Rc<RiverWindowV1>,
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
            log_forward("river_window_v1.dimensions", &e);
        }
    }

    /// propose window dimensions
    ///
    /// This request proposes dimensions for the window in the compositor's
    /// logical coordinate space.
    ///
    /// The width and height must be greater than or equal to zero. If the width
    /// or height is zero the window will be allowed to decide its own
    /// dimensions.
    ///
    /// The window may not take the exact dimensions proposed. The actual
    /// dimensions taken by the window will be sent in a subsequent
    /// river_window_v1.dimensions event. For example, a terminal emulator may
    /// only allow dimensions that are multiple of the cell size.
    ///
    /// When a propose_dimensions request is made, the server must send a
    /// dimensions event in response as soon as possible. It may not be possible
    /// to send a dimensions event in the very next render sequence if, for
    /// example, the window takes too long to respond to the first proposed
    /// dimensions. In this case, the server will send the dimensions event in a
    /// future render sequence. The window will not be displayed until the first
    /// dimensions event is received and the render sequence is finished.
    ///
    /// Note that the dimensions of a river_window_v1 refer to the dimensions of
    /// the window content and are unaffected by the presence of borders or
    /// decoration surfaces.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_propose_dimensions(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_propose_dimensions(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.propose_dimensions", &e);
        }
    }

    /// request that the window be hidden
    ///
    /// Request that the window be hidden. Has no effect if the window is
    /// already hidden. Hides any window borders and decorations as well.
    ///
    /// Newly created windows are considered shown unless explicitly hidden with
    /// the hide request.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_hide(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_hide(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.hide", &e);
        }
    }

    /// request that the window be shown
    ///
    /// Request that the window be shown. Has no effect if the window is not
    /// hidden. Does not guarantee that the window is visible as it may be
    /// completely obscured by other windows placed above it for example.
    ///
    /// Newly created windows are considered shown unless explicitly hidden with
    /// the hide request.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_show(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_show(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.show", &e);
        }
    }

    /// the window set an application ID
    ///
    /// The window set an application ID.
    ///
    /// The app_id argument will be null if the window has never set an
    /// application ID or if the window cleared its application ID. (Xwayland
    /// windows may do this for example, though xdg-toplevels may not.)
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    fn handle_app_id(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        app_id: Option<&str>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_app_id(
            app_id,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.app_id", &e);
        }
    }

    /// the window set a title
    ///
    /// The window set a title.
    ///
    /// The title argument will be null if the window has never set a title or
    /// if the window cleared its title. (Xwayland windows may do this for
    /// example, though xdg-toplevels may not.)
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    fn handle_title(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        title: Option<&str>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_title(
            title,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.title", &e);
        }
    }

    /// the window set a parent
    ///
    /// The window set a parent window. If this event is never received or if
    /// the parent argument is null then the window has no parent.
    ///
    /// A surface with a parent set might be a dialog, file picker, or similar
    /// for the parent window.
    ///
    /// Child windows should generally be rendered directly above their parent.
    ///
    /// The compositor must guarantee that there are no loops in the window
    /// tree: a parent must not be the descendant of one of its children.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_parent(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        parent: Option<&Rc<RiverWindowV1>>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(parent) = parent {
                if let Some(client_id_2) = parent.core().client_id.get() {
                    if client_id != client_id_2 {
                        return;
                    }
                }
            }
        }
        let res = slf.try_send_parent(
            parent,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.parent", &e);
        }
    }

    /// supported/preferred decoration style
    ///
    /// Information from the window about the supported and preferred client
    /// side/server side decoration options.
    ///
    /// This event may be sent multiple times over the lifetime of the window if
    /// the window changes its preferences.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    #[inline]
    fn handle_decoration_hint(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        hint: RiverWindowV1DecorationHint,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_decoration_hint(
            hint,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.decoration_hint", &e);
        }
    }

    /// tell the client to use CSD
    ///
    /// Tell the client to use client side decoration and draw its own title
    /// bar, borders, etc.
    ///
    /// This is the default if neither this request nor the use_ssd request is
    /// ever made.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_use_csd(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_use_csd(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.use_csd", &e);
        }
    }

    /// tell the client to use SSD
    ///
    /// Tell the client to use server side decoration and not draw any client
    /// side decorations.
    ///
    /// This request will have no effect if the client only supports client side
    /// decoration, see the decoration_hint event.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_use_ssd(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_use_ssd(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.use_ssd", &e);
        }
    }

    /// set window borders
    ///
    /// This request decorates the window with borders drawn by the compositor
    /// on the specified edges of the window. Borders are drawn above the window
    /// content.
    ///
    /// Corners are drawn only between borders on adjacent edges. If e.g. the
    /// left edge has a border and the top edge does not, the border drawn on
    /// the left edge will not extend vertically beyond the top edge of the
    /// window.
    ///
    /// Borders are not drawn while the window is fullscreen.
    ///
    /// The color is defined by four 32-bit RGBA values. Unless specified in
    /// another protocol extension, the RGBA values use pre-multiplied alpha.
    ///
    /// Setting the edges to none or the width to 0 disables the borders.
    /// Setting a negative width is a protocol error.
    ///
    /// This request completely overrides all previous set_borders requests.
    /// Only the most recent set_borders request has an effect.
    ///
    /// Note that the position/dimensions of a river_window_v1 refer to the
    /// position/dimensions of the window content and are unaffected by the
    /// presence of borders or decoration surfaces.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `edges`:
    /// - `width`:
    /// - `r`:
    /// - `g`:
    /// - `b`:
    /// - `a`:
    #[inline]
    fn handle_set_borders(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        edges: RiverWindowV1Edges,
        width: i32,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_borders(
            edges,
            width,
            r,
            g,
            b,
            a,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.set_borders", &e);
        }
    }

    /// set window tiled state
    ///
    /// Inform the window that it is part of a tiled layout and adjacent to
    /// other elements in the tiled layout on the given edges.
    ///
    /// The window should use this information to change the style of its client
    /// side decorations and avoid drawing e.g. drop shadows outside of the
    /// window dimensions on the tiled edges.
    ///
    /// Setting the edges argument to none informs the window that it is not
    /// part of a tiled layout. If this request is never made, the window is
    /// informed that it is not part of a tiled layout.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `edges`:
    #[inline]
    fn handle_set_tiled(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        edges: RiverWindowV1Edges,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_tiled(
            edges,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.set_tiled", &e);
        }
    }

    /// create a decoration surface above the window
    ///
    /// Create a decoration surface and assign the river_decoration_v1 role to
    /// the surface. The created decoration is placed above the window in
    /// rendering order, see the description of river_decoration_v1.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_decoration_above(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        id: &Rc<RiverDecorationV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_decoration_above(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.get_decoration_above", &e);
        }
    }

    /// create a decoration surface below the window
    ///
    /// Create a decoration surface and assign the river_decoration_v1 role to
    /// the surface. The created decoration is placed below the window in
    /// rendering order, see the description of river_decoration_v1.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_decoration_below(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        id: &Rc<RiverDecorationV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_decoration_below(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.get_decoration_below", &e);
        }
    }

    /// window requested interactive pointer move
    ///
    /// This event informs the window manager that the window has requested to
    /// be interactively moved using the pointer. The seat argument indicates the
    /// seat for the move.
    ///
    /// The xdg-shell protocol for example allows windows to request that an
    /// interactive move be started, perhaps when a client-side rendered
    /// titlebar is dragged.
    ///
    /// The window manager may use the river_seat_v1.op_start_pointer request to
    /// interactively move the window or ignore this event entirely.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_pointer_move_requested(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        seat: &Rc<RiverSeatV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = seat.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_pointer_move_requested(
            seat,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.pointer_move_requested", &e);
        }
    }

    /// window requested interactive pointer resize
    ///
    /// This event informs the window manager that the window has requested to
    /// be interactively resized using the pointer. The seat argument indicates
    /// the seat for the resize.
    ///
    /// The edges argument indicates which edges the window has requested to be
    /// resized from. The edges argument will never be none and will never have
    /// both top and bottom or both left and right edges set.
    ///
    /// The xdg-shell protocol for example allows windows to request that an
    /// interactive resize be started, perhaps when the corner of client-side
    /// rendered decorations is dragged.
    ///
    /// The window manager may use the river_seat_v1.op_start_pointer request to
    /// interactively resize the window or ignore this event entirely.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `edges`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_pointer_resize_requested(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        seat: &Rc<RiverSeatV1>,
        edges: RiverWindowV1Edges,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = seat.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_pointer_resize_requested(
            seat,
            edges,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.pointer_resize_requested", &e);
        }
    }

    /// inform the window it is being resized
    ///
    /// Inform the window that it is being resized. The window manager should
    /// use this request to inform windows that are the target of an interactive
    /// resize for example.
    ///
    /// The window manager remains responsible for handling the position and
    /// dimensions of the window while it is resizing.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_inform_resize_start(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_inform_resize_start(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.inform_resize_start", &e);
        }
    }

    /// inform the window it no longer being resized
    ///
    /// Inform the window that it is no longer being resized. The window manager
    /// should use this request to inform windows that are the target of an
    /// interactive resize that the interactive resize has ended for example.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_inform_resize_end(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_inform_resize_end(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.inform_resize_end", &e);
        }
    }

    /// inform window of supported capabilities
    ///
    /// This request informs the window of the capabilities supported by the
    /// window manager. If the window manager, for example, ignores requests to
    /// be maximized from the window it should not tell the window that it
    /// supports the maximize capability.
    ///
    /// The window might use this information to, for example, only show a
    /// maximize button if the window manager supports the maximize capability.
    ///
    /// The window manager client should use this request to set capabilities
    /// for all new windows. If this request is never made, the compositor will
    /// inform windows that all capabilities are supported.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `caps`:
    #[inline]
    fn handle_set_capabilities(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        caps: RiverWindowV1Capabilities,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_capabilities(
            caps,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.set_capabilities", &e);
        }
    }

    /// window requested that the window menu be shown
    ///
    /// The xdg-shell protocol for example allows windows to request that a
    /// window menu be shown, for example when the user right clicks on client
    /// side window decorations.
    ///
    /// A window menu might include options to maximize or minimize the window.
    ///
    /// The window manager is free to ignore this request and decide what the
    /// window menu contains if it does choose to show one.
    ///
    /// The x and y arguments indicate where the window requested that the
    /// window menu be shown.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `x`: x offset from top left corner
    /// - `y`: y offset from top left corner
    #[inline]
    fn handle_show_window_menu_requested(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_show_window_menu_requested(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.show_window_menu_requested", &e);
        }
    }

    /// the window requested to be maximized
    ///
    /// The xdg-shell protocol for example allows windows to request to be
    /// maximized.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.inform_maximize or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_maximize_requested(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_maximize_requested(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.maximize_requested", &e);
        }
    }

    /// the window requested to be unmaximized
    ///
    /// The xdg-shell protocol for example allows windows to request to be
    /// unmaximized.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.inform_unmaximized or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_unmaximize_requested(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_unmaximize_requested(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.unmaximize_requested", &e);
        }
    }

    /// inform the window that it is maximized
    ///
    /// Inform the window that it is maximized. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// The window manager remains responsible for handling the position and
    /// dimensions of the window while it is maximized.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_inform_maximized(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_inform_maximized(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.inform_maximized", &e);
        }
    }

    /// inform the window that it is unmaximized
    ///
    /// Inform the window that it is unmaximized. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_inform_unmaximized(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_inform_unmaximized(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.inform_unmaximized", &e);
        }
    }

    /// the window requested to be fullscreen
    ///
    /// The xdg-shell protocol for example allows windows to request that they
    /// be made fullscreen and allows them to provide an output preference.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.fullscreen or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_fullscreen_requested(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        output: Option<&Rc<RiverOutputV1>>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(output) = output {
                if let Some(client_id_2) = output.core().client_id.get() {
                    if client_id != client_id_2 {
                        return;
                    }
                }
            }
        }
        let res = slf.try_send_fullscreen_requested(
            output,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.fullscreen_requested", &e);
        }
    }

    /// the window requested to exit fullscreen
    ///
    /// The xdg-shell protocol for example allows windows to request to exit
    /// fullscreen.
    ///
    /// The window manager is free to honor this request using
    /// river_window_v1.exit_fullscreen or ignore it.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_exit_fullscreen_requested(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_exit_fullscreen_requested(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.exit_fullscreen_requested", &e);
        }
    }

    /// inform the window that it is fullscreen
    ///
    /// Inform the window that it is fullscreen. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// This request does not affect the size/position of the window or cause it
    /// to become the only window rendered, see the river_window_v1.fullscreen
    /// and exit_fullscreen requests for that.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_inform_fullscreen(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_inform_fullscreen(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.inform_fullscreen", &e);
        }
    }

    /// inform the window that it is not fullscreen
    ///
    /// Inform the window that it is not fullscreen. The window might use this
    /// information to adapt the style of its client-side window decorations for
    /// example.
    ///
    /// This request does not affect the size/position of the window or cause it
    /// to become the only window rendered, see the river_window_v1.fullscreen
    /// and exit_fullscreen requests for that.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_inform_not_fullscreen(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_inform_not_fullscreen(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.inform_not_fullscreen", &e);
        }
    }

    /// make the window fullscreen
    ///
    /// Make the window fullscreen on the given output. If multiple windows are
    /// fullscreen on the same output at the same time only the "top" window in
    /// rendering order shall be displayed.
    ///
    /// All river_shell_surface_v1 objects above the top fullscreen window in
    /// the rendering order will continue to be rendered.
    ///
    /// The compositor will handle the position and dimensions of the window
    /// while it is fullscreen. The set_position and propose_dimensions requests
    /// shall not affect the current position and dimensions of a fullscreen
    /// window.
    ///
    /// The compositor will clip window content, decoration surfaces, and
    /// borders to the given output's dimensions while the window is fullscreen.
    /// The effects of set_clip_box and set_content_clip_box are ignored while
    /// the window is fullscreen.
    ///
    /// If the output on which a window is currently fullscreen is removed, the
    /// windowing state is modified as if there were an exit_fullscreen request
    /// made in the same manage sequence as the river_output_v1.removed event.
    ///
    /// This request does not inform the window that it is fullscreen, see the
    /// river_window_v1.inform_fullscreen and inform_not_fullscreen requests.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_fullscreen(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        output: &Rc<RiverOutputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_fullscreen(
            output,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.fullscreen", &e);
        }
    }

    /// make the window not fullscreen
    ///
    /// Make the window not fullscreen.
    ///
    /// The position and dimensions are undefined after this request is made
    /// until a manage sequence in which the window manager makes the
    /// propose_dimensions and set_position requests is completed.
    ///
    /// The window manager should make propose_dimensions and set_position
    /// requests in the same manage sequence as the exit_fullscreen request for
    /// frame perfection.
    ///
    /// This request does not inform the window that it is fullscreen, see the
    /// river_window_v1.inform_fullscreen and inform_not_fullscreen requests.
    ///
    /// This request modifies window management state and may only be made as
    /// part of a manage sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_exit_fullscreen(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_exit_fullscreen(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.exit_fullscreen", &e);
        }
    }

    /// the window requested to be minimized
    ///
    /// The xdg-shell protocol for example allows windows to request to be
    /// minimized.
    ///
    /// The window manager is free to ignore this request, hide the window, or
    /// do whatever else it chooses.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_minimize_requested(
        &mut self,
        slf: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_minimize_requested(
        );
        if let Err(e) = res {
            log_forward("river_window_v1.minimize_requested", &e);
        }
    }

    /// clip the window to a given box
    ///
    /// Clip the window, including borders and decoration surfaces, to the box
    /// specified by the x, y, width, and height arguments. The x/y position of
    /// the box is relative to the top left corner of the window.
    ///
    /// The width and height arguments must be greater than or equal to 0.
    ///
    /// Setting a clip box with 0 width or height disables clipping.
    ///
    /// The clip box is ignored while the window is fullscreen.
    ///
    /// Both set_clip_box and set_content_clip_box may be enabled simultaneously.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_set_clip_box(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_clip_box(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.set_clip_box", &e);
        }
    }

    /// unreliable PID of the window's creator
    ///
    /// This event gives an unreliable PID of the process that created the
    /// window. Obtaining this information is inherently racy due to PID reuse.
    /// Therefore, this PID must not be used for anything security sensitive.
    ///
    /// Note also that a single process may create multiple windows, so there is
    /// not necessarily a 1-to-1 mapping from PID to window. Multiple windows
    /// may have the same PID.
    ///
    /// This event is sent once when the river_window_v1 is created and never
    /// sent again.
    ///
    /// # Arguments
    ///
    /// - `unreliable_pid`:
    #[inline]
    fn handle_unreliable_pid(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        unreliable_pid: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_unreliable_pid(
            unreliable_pid,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.unreliable_pid", &e);
        }
    }

    /// clip the window content to a given box
    ///
    /// Clip the content of the window, excluding borders and decoration
    /// surfaces, to the box specified by the x, y, width, and height arguments.
    /// The x/y position of the box is relative to the top left corner of the
    /// window.
    ///
    /// Borders drawn by the compositor (see set_borders) are placed around the
    /// intersection of the window content (as defined by the dimensions event)
    /// and the content clip box when content clipping is enabled.
    ///
    /// The width and height arguments must be greater than or equal to 0.
    ///
    /// Setting a box with 0 width or height disables content clipping.
    ///
    /// The content clip box is ignored while the window is fullscreen.
    ///
    /// Both set_clip_box and set_content_clip_box may be enabled simultaneously.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_set_content_clip_box(
        &mut self,
        slf: &Rc<RiverWindowV1>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_content_clip_box(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("river_window_v1.set_content_clip_box", &e);
        }
    }
}

impl ObjectPrivate for RiverWindowV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverWindowV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.close()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_close(&self);
                } else {
                    DefaultHandler.handle_close(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.get_node(id: river_node_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = RiverNodeV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_node(&self, arg0);
                } else {
                    DefaultHandler.handle_get_node(&self, arg0);
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.propose_dimensions(width: {}, height: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_propose_dimensions(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_propose_dimensions(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.hide()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_hide(&self);
                } else {
                    DefaultHandler.handle_hide(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.show()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_show(&self);
                } else {
                    DefaultHandler.handle_show(&self);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.use_csd()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_use_csd(&self);
                } else {
                    DefaultHandler.handle_use_csd(&self);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.use_ssd()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_use_ssd(&self);
                } else {
                    DefaultHandler.handle_use_ssd(&self);
                }
            }
            8 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 32)));
                };
                let arg0 = RiverWindowV1Edges(arg0);
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: RiverWindowV1Edges, arg1: i32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.set_borders(edges: {:?}, width: {}, r: {}, g: {}, b: {}, a: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4, arg5);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_borders(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.handle_set_borders(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                }
            }
            9 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverWindowV1Edges(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: RiverWindowV1Edges) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.set_tiled(edges: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_tiled(&self, arg0);
                } else {
                    DefaultHandler.handle_set_tiled(&self, arg0);
                }
            }
            10 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.get_decoration_above(id: river_decoration_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverDecorationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_decoration_above(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_decoration_above(&self, arg0, arg1);
                }
            }
            11 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.get_decoration_below(id: river_decoration_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverDecorationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_decoration_below(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_decoration_below(&self, arg0, arg1);
                }
            }
            12 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.inform_resize_start()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_inform_resize_start(&self);
                } else {
                    DefaultHandler.handle_inform_resize_start(&self);
                }
            }
            13 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.inform_resize_end()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_inform_resize_end(&self);
                } else {
                    DefaultHandler.handle_inform_resize_end(&self);
                }
            }
            14 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverWindowV1Capabilities(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: RiverWindowV1Capabilities) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.set_capabilities(caps: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_capabilities(&self, arg0);
                } else {
                    DefaultHandler.handle_set_capabilities(&self, arg0);
                }
            }
            15 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.inform_maximized()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_inform_maximized(&self);
                } else {
                    DefaultHandler.handle_inform_maximized(&self);
                }
            }
            16 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.inform_unmaximized()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_inform_unmaximized(&self);
                } else {
                    DefaultHandler.handle_inform_unmaximized(&self);
                }
            }
            17 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.inform_fullscreen()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_inform_fullscreen(&self);
                } else {
                    DefaultHandler.handle_inform_fullscreen(&self);
                }
            }
            18 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.inform_not_fullscreen()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_inform_not_fullscreen(&self);
                } else {
                    DefaultHandler.handle_inform_not_fullscreen(&self);
                }
            }
            19 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.fullscreen(output: river_output_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverOutputV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::RiverOutputV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_fullscreen(&self, arg0);
                } else {
                    DefaultHandler.handle_fullscreen(&self, arg0);
                }
            }
            20 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.exit_fullscreen()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_exit_fullscreen(&self);
                } else {
                    DefaultHandler.handle_exit_fullscreen(&self);
                }
            }
            21 => {
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.set_clip_box(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_clip_box(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_clip_box(&self, arg0, arg1, arg2, arg3);
                }
            }
            22 => {
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_v1#{}.set_content_clip_box(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_content_clip_box(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_content_clip_box(&self, arg0, arg1, arg2, arg3);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.closed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_closed(&self);
                } else {
                    DefaultHandler.handle_closed(&self);
                }
            }
            1 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.dimensions_hint(min_width: {}, min_height: {}, max_width: {}, max_height: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dimensions_hint(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_dimensions_hint(&self, arg0, arg1, arg2, arg3);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.dimensions(width: {}, height: {})\n", id, arg0, arg1);
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
            3 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NullableString>(msg, offset, "app_id")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.app_id(app_id: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_app_id(&self, arg0);
                } else {
                    DefaultHandler.handle_app_id(&self, arg0);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NullableString>(msg, offset, "title")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.title(title: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_title(&self, arg0);
                } else {
                    DefaultHandler.handle_title(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.parent(parent: river_window_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = server.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverWindowV1>() else {
                        let o = server.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("parent", o.core().interface, ObjectInterface::RiverWindowV1)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_parent(&self, arg0);
                } else {
                    DefaultHandler.handle_parent(&self, arg0);
                }
            }
            6 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = RiverWindowV1DecorationHint(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: RiverWindowV1DecorationHint) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.decoration_hint(hint: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_decoration_hint(&self, arg0);
                } else {
                    DefaultHandler.handle_decoration_hint(&self, arg0);
                }
            }
            7 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.pointer_move_requested(seat: river_seat_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverSeatV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::RiverSeatV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_pointer_move_requested(&self, arg0);
                } else {
                    DefaultHandler.handle_pointer_move_requested(&self, arg0);
                }
            }
            8 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = RiverWindowV1Edges(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: RiverWindowV1Edges) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.pointer_resize_requested(seat: river_seat_v1#{}, edges: {:?})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverSeatV1>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::RiverSeatV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_pointer_resize_requested(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_pointer_resize_requested(&self, arg0, arg1);
                }
            }
            9 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.show_window_menu_requested(x: {}, y: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_show_window_menu_requested(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_show_window_menu_requested(&self, arg0, arg1);
                }
            }
            10 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.maximize_requested()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_maximize_requested(&self);
                } else {
                    DefaultHandler.handle_maximize_requested(&self);
                }
            }
            11 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.unmaximize_requested()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unmaximize_requested(&self);
                } else {
                    DefaultHandler.handle_unmaximize_requested(&self);
                }
            }
            12 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.fullscreen_requested(output: river_output_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = server.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverOutputV1>() else {
                        let o = server.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::RiverOutputV1)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_fullscreen_requested(&self, arg0);
                } else {
                    DefaultHandler.handle_fullscreen_requested(&self, arg0);
                }
            }
            13 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.exit_fullscreen_requested()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_exit_fullscreen_requested(&self);
                } else {
                    DefaultHandler.handle_exit_fullscreen_requested(&self);
                }
            }
            14 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.minimize_requested()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_minimize_requested(&self);
                } else {
                    DefaultHandler.handle_minimize_requested(&self);
                }
            }
            15 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_v1#{}.unreliable_pid(unreliable_pid: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unreliable_pid(&self, arg0);
                } else {
                    DefaultHandler.handle_unreliable_pid(&self, arg0);
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
            1 => "close",
            2 => "get_node",
            3 => "propose_dimensions",
            4 => "hide",
            5 => "show",
            6 => "use_csd",
            7 => "use_ssd",
            8 => "set_borders",
            9 => "set_tiled",
            10 => "get_decoration_above",
            11 => "get_decoration_below",
            12 => "inform_resize_start",
            13 => "inform_resize_end",
            14 => "set_capabilities",
            15 => "inform_maximized",
            16 => "inform_unmaximized",
            17 => "inform_fullscreen",
            18 => "inform_not_fullscreen",
            19 => "fullscreen",
            20 => "exit_fullscreen",
            21 => "set_clip_box",
            22 => "set_content_clip_box",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "closed",
            1 => "dimensions_hint",
            2 => "dimensions",
            3 => "app_id",
            4 => "title",
            5 => "parent",
            6 => "decoration_hint",
            7 => "pointer_move_requested",
            8 => "pointer_resize_requested",
            9 => "show_window_menu_requested",
            10 => "maximize_requested",
            11 => "unmaximize_requested",
            12 => "fullscreen_requested",
            13 => "exit_fullscreen_requested",
            14 => "minimize_requested",
            15 => "unreliable_pid",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverWindowV1 {
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

impl RiverWindowV1 {
    /// Since when the error.node_exists enum variant is available.
    pub const ENM__ERROR_NODE_EXISTS__SINCE: u32 = 1;
    /// Since when the error.invalid_dimensions enum variant is available.
    pub const ENM__ERROR_INVALID_DIMENSIONS__SINCE: u32 = 1;
    /// Since when the error.invalid_border enum variant is available.
    pub const ENM__ERROR_INVALID_BORDER__SINCE: u32 = 1;
    /// Since when the error.invalid_clip_box enum variant is available.
    pub const ENM__ERROR_INVALID_CLIP_BOX__SINCE: u32 = 1;

    /// Since when the decoration_hint.only_supports_csd enum variant is available.
    pub const ENM__DECORATION_HINT_ONLY_SUPPORTS_CSD__SINCE: u32 = 1;
    /// Since when the decoration_hint.prefers_csd enum variant is available.
    pub const ENM__DECORATION_HINT_PREFERS_CSD__SINCE: u32 = 1;
    /// Since when the decoration_hint.prefers_ssd enum variant is available.
    pub const ENM__DECORATION_HINT_PREFERS_SSD__SINCE: u32 = 1;
    /// Since when the decoration_hint.no_preference enum variant is available.
    pub const ENM__DECORATION_HINT_NO_PREFERENCE__SINCE: u32 = 1;

    /// Since when the edges.none enum variant is available.
    pub const ENM__EDGES_NONE__SINCE: u32 = 1;
    /// Since when the edges.top enum variant is available.
    pub const ENM__EDGES_TOP__SINCE: u32 = 1;
    /// Since when the edges.bottom enum variant is available.
    pub const ENM__EDGES_BOTTOM__SINCE: u32 = 1;
    /// Since when the edges.left enum variant is available.
    pub const ENM__EDGES_LEFT__SINCE: u32 = 1;
    /// Since when the edges.right enum variant is available.
    pub const ENM__EDGES_RIGHT__SINCE: u32 = 1;

    /// Since when the capabilities.window_menu enum variant is available.
    pub const ENM__CAPABILITIES_WINDOW_MENU__SINCE: u32 = 1;
    /// Since when the capabilities.maximize enum variant is available.
    pub const ENM__CAPABILITIES_MAXIMIZE__SINCE: u32 = 1;
    /// Since when the capabilities.fullscreen enum variant is available.
    pub const ENM__CAPABILITIES_FULLSCREEN__SINCE: u32 = 1;
    /// Since when the capabilities.minimize enum variant is available.
    pub const ENM__CAPABILITIES_MINIMIZE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverWindowV1Error(pub u32);

impl RiverWindowV1Error {
    /// window already has a node object
    pub const NODE_EXISTS: Self = Self(0);

    /// proposed dimensions out of bounds
    pub const INVALID_DIMENSIONS: Self = Self(1);

    /// invalid arg to set_borders
    pub const INVALID_BORDER: Self = Self(2);

    /// invalid arg to set_clip_box
    pub const INVALID_CLIP_BOX: Self = Self(3);
}

impl Debug for RiverWindowV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NODE_EXISTS => "NODE_EXISTS",
            Self::INVALID_DIMENSIONS => "INVALID_DIMENSIONS",
            Self::INVALID_BORDER => "INVALID_BORDER",
            Self::INVALID_CLIP_BOX => "INVALID_CLIP_BOX",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverWindowV1DecorationHint(pub u32);

impl RiverWindowV1DecorationHint {
    /// only supports client side decoration
    pub const ONLY_SUPPORTS_CSD: Self = Self(0);

    /// client side decoration preferred, both CSD and SSD supported
    pub const PREFERS_CSD: Self = Self(1);

    /// server side decoration preferred, both CSD and SSD supported
    pub const PREFERS_SSD: Self = Self(2);

    /// no preference, both CSD and SSD supported
    pub const NO_PREFERENCE: Self = Self(3);
}

impl Debug for RiverWindowV1DecorationHint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ONLY_SUPPORTS_CSD => "ONLY_SUPPORTS_CSD",
            Self::PREFERS_CSD => "PREFERS_CSD",
            Self::PREFERS_SSD => "PREFERS_SSD",
            Self::NO_PREFERENCE => "NO_PREFERENCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct RiverWindowV1Edges(pub u32);

/// An iterator over the set bits in a [`RiverWindowV1Edges`].
///
/// You can construct this with the `IntoIterator` implementation of `RiverWindowV1Edges`.
#[derive(Clone, Debug)]
pub struct RiverWindowV1EdgesIter(pub u32);

impl RiverWindowV1Edges {
    pub const NONE: Self = Self(0);

    pub const TOP: Self = Self(1);

    pub const BOTTOM: Self = Self(2);

    pub const LEFT: Self = Self(4);

    pub const RIGHT: Self = Self(8);
}

impl RiverWindowV1Edges {
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
        Self(0 | 0 | 1 | 2 | 4 | 8)
    }
}

impl Iterator for RiverWindowV1EdgesIter {
    type Item = RiverWindowV1Edges;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(RiverWindowV1Edges(bit))
    }
}

impl IntoIterator for RiverWindowV1Edges {
    type Item = RiverWindowV1Edges;
    type IntoIter = RiverWindowV1EdgesIter;

    fn into_iter(self) -> Self::IntoIter {
        RiverWindowV1EdgesIter(self.0)
    }
}

impl BitAnd for RiverWindowV1Edges {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for RiverWindowV1Edges {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for RiverWindowV1Edges {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for RiverWindowV1Edges {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for RiverWindowV1Edges {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for RiverWindowV1Edges {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for RiverWindowV1Edges {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for RiverWindowV1Edges {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for RiverWindowV1Edges {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for RiverWindowV1Edges {
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
            f.write_str("TOP")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BOTTOM")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("LEFT")?;
        }
        if v & 8 == 8 {
            v &= !8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("RIGHT")?;
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct RiverWindowV1Capabilities(pub u32);

/// An iterator over the set bits in a [`RiverWindowV1Capabilities`].
///
/// You can construct this with the `IntoIterator` implementation of `RiverWindowV1Capabilities`.
#[derive(Clone, Debug)]
pub struct RiverWindowV1CapabilitiesIter(pub u32);

impl RiverWindowV1Capabilities {
    pub const WINDOW_MENU: Self = Self(1);

    pub const MAXIMIZE: Self = Self(2);

    pub const FULLSCREEN: Self = Self(4);

    pub const MINIMIZE: Self = Self(8);
}

impl RiverWindowV1Capabilities {
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
        Self(0 | 1 | 2 | 4 | 8)
    }
}

impl Iterator for RiverWindowV1CapabilitiesIter {
    type Item = RiverWindowV1Capabilities;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(RiverWindowV1Capabilities(bit))
    }
}

impl IntoIterator for RiverWindowV1Capabilities {
    type Item = RiverWindowV1Capabilities;
    type IntoIter = RiverWindowV1CapabilitiesIter;

    fn into_iter(self) -> Self::IntoIter {
        RiverWindowV1CapabilitiesIter(self.0)
    }
}

impl BitAnd for RiverWindowV1Capabilities {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for RiverWindowV1Capabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for RiverWindowV1Capabilities {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for RiverWindowV1Capabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for RiverWindowV1Capabilities {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for RiverWindowV1Capabilities {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for RiverWindowV1Capabilities {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for RiverWindowV1Capabilities {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for RiverWindowV1Capabilities {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for RiverWindowV1Capabilities {
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
            f.write_str("WINDOW_MENU")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("MAXIMIZE")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("FULLSCREEN")?;
        }
        if v & 8 == 8 {
            v &= !8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("MINIMIZE")?;
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
            f.write_str("0")?;
        }
        Ok(())
    }
}
