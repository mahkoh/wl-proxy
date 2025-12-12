//! An opened toplevel
//!
//! A treeland_foreign_toplevel_handle_v1 object represents an opened toplevel window. Each
//! app may have multiple opened toplevels.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_foreign_toplevel_handle_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandForeignToplevelHandleV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandForeignToplevelHandleV1Handler>,
}

struct DefaultHandler;

impl TreelandForeignToplevelHandleV1Handler for DefaultHandler { }

impl TreelandForeignToplevelHandleV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::TreelandForeignToplevelHandleV1;
    pub const INTERFACE_NAME: &str = "treeland_foreign_toplevel_handle_v1";
}

impl TreelandForeignToplevelHandleV1 {
    pub fn set_handler(&self, handler: impl TreelandForeignToplevelHandleV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandForeignToplevelHandleV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandForeignToplevelHandleV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandForeignToplevelHandleV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandForeignToplevelHandleV1 {
    /// Since when the pid message is available.
    pub const MSG__PID__SINCE: u32 = 1;

    /// Process id of application owning the window has changed
    ///
    /// This event will be sent when the compositor has set the process id this window
    /// belongs to. This should be set once before the initial_state is sent.
    ///
    /// # Arguments
    ///
    /// - `pid`:
    #[inline]
    pub fn send_pid(
        &self,
        pid: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            pid,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_handle_v1#{}.pid(pid: {})\n", client.endpoint.id, id, arg0);
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

    /// Since when the title message is available.
    pub const MSG__TITLE__SINCE: u32 = 1;

    /// title change
    ///
    /// This event is emitted whenever the title of the toplevel changes.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    pub fn send_title(
        &self,
        title: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            title,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_handle_v1#{}.title(title: {:?})\n", client.endpoint.id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the app_id message is available.
    pub const MSG__APP_ID__SINCE: u32 = 1;

    /// app-id change
    ///
    /// This event is emitted whenever the app-id of the toplevel changes.
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    pub fn send_app_id(
        &self,
        app_id: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            app_id,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_handle_v1#{}.app_id(app_id: {:?})\n", client.endpoint.id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the identifier message is available.
    pub const MSG__IDENTIFIER__SINCE: u32 = 1;

    /// a stable identifier for a toplevel
    ///
    /// The identifier of each top level and its handle must be unique.
    /// Two different top layers cannot have the same identifier.
    /// This identifier is only valid as long as the top level is mapped.
    /// Identifiers must not be reused if the top level is not mapped.
    /// The compositor must not reuse identifiers to ensure there are no races when
    /// identifiers are shared between processes.
    ///
    /// # Arguments
    ///
    /// - `identifier`:
    #[inline]
    pub fn send_identifier(
        &self,
        identifier: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            identifier,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_handle_v1#{}.identifier(identifier: {})\n", client.endpoint.id, id, arg0);
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
            3,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the output_enter message is available.
    pub const MSG__OUTPUT_ENTER__SINCE: u32 = 1;

    /// toplevel entered an output
    ///
    /// This event is emitted whenever the toplevel becomes visible on
    /// the given output. A toplevel may be visible on multiple outputs.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn send_output_enter(
        &self,
        output: &Rc<WlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("output", client.endpoint.id));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_handle_v1#{}.output_enter(output: wl_output#{})\n", client.endpoint.id, id, arg0_id);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the output_leave message is available.
    pub const MSG__OUTPUT_LEAVE__SINCE: u32 = 1;

    /// toplevel left an output
    ///
    /// This event is emitted whenever the toplevel stops being visible on
    /// the given output. It is guaranteed that an entered-output event
    /// with the same output has been emitted before this event.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn send_output_leave(
        &self,
        output: &Rc<WlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError::ArgNoClientId("output", client.endpoint.id));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_handle_v1#{}.output_leave(output: wl_output#{})\n", client.endpoint.id, id, arg0_id);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the set_maximized message is available.
    pub const MSG__SET_MAXIMIZED__SINCE: u32 = 1;

    /// requests that the toplevel be maximized
    ///
    /// Requests that the toplevel be maximized. If the maximized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn send_set_maximized(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_handle_v1#{}.set_maximized()\n", id);
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
        ]);
        Ok(())
    }

    /// Since when the unset_maximized message is available.
    pub const MSG__UNSET_MAXIMIZED__SINCE: u32 = 1;

    /// requests that the toplevel be unmaximized
    ///
    /// Requests that the toplevel be unmaximized. If the maximized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn send_unset_maximized(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_handle_v1#{}.unset_maximized()\n", id);
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
        ]);
        Ok(())
    }

    /// Since when the set_minimized message is available.
    pub const MSG__SET_MINIMIZED__SINCE: u32 = 1;

    /// requests that the toplevel be minimized
    ///
    /// Requests that the toplevel be minimized. If the minimized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn send_set_minimized(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_handle_v1#{}.set_minimized()\n", id);
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

    /// Since when the unset_minimized message is available.
    pub const MSG__UNSET_MINIMIZED__SINCE: u32 = 1;

    /// requests that the toplevel be unminimized
    ///
    /// Requests that the toplevel be unminimized. If the minimized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn send_unset_minimized(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_handle_v1#{}.unset_minimized()\n", id);
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
        Ok(())
    }

    /// Since when the activate message is available.
    pub const MSG__ACTIVATE__SINCE: u32 = 1;

    /// activate the toplevel
    ///
    /// Request that this toplevel be activated on the given seat.
    /// There is no guarantee the toplevel will be actually activated.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    pub fn send_activate(
        &self,
        seat: &Rc<WlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            seat,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_handle_v1#{}.activate(seat: wl_seat#{})\n", id, arg0_id);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the state message is available.
    pub const MSG__STATE__SINCE: u32 = 1;

    /// the toplevel state changed
    ///
    /// This event is emitted immediately after the treeland_foreign_toplevel_handle_v1
    /// is created and each time the toplevel state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn send_state(
        &self,
        state: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_handle_v1#{}.state(state: {})\n", client.endpoint.id, id, debug_array(arg0));
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
            6,
        ]);
        fmt.array(arg0);
        Ok(())
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all information about the toplevel has been sent
    ///
    /// This event is sent after all changes in the toplevel state have been
    /// sent.
    ///
    /// This allows changes to the treeland_foreign_toplevel_handle_v1 properties
    /// to be seen as atomic, even if they happen via multiple events.
    #[inline]
    pub fn send_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_handle_v1#{}.done()\n", client.endpoint.id, id);
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
            7,
        ]);
        Ok(())
    }

    /// Since when the close message is available.
    pub const MSG__CLOSE__SINCE: u32 = 1;

    /// request that the toplevel be closed
    ///
    /// Send a request to the toplevel to close itself. The compositor would
    /// typically use a shell-specific method to carry out this request, for
    /// example by sending the xdg_toplevel.close event. However, this gives
    /// no guarantees the toplevel will actually be destroyed. If and when
    /// this happens, the treeland_foreign_toplevel_handle_v1.closed event will
    /// be emitted.
    #[inline]
    pub fn send_close(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_handle_v1#{}.close()\n", id);
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
            5,
        ]);
        Ok(())
    }

    /// Since when the set_rectangle message is available.
    pub const MSG__SET_RECTANGLE__SINCE: u32 = 1;

    /// the rectangle which represents the toplevel
    ///
    /// The rectangle of the surface specified in this request corresponds to
    /// the place where the app using this protocol represents the given toplevel.
    /// It can be used by the compositor as a hint for some operations, e.g
    /// minimizing. The client is however not required to set this, in which
    /// case the compositor is free to decide some default value.
    ///
    /// If the client specifies more than one rectangle, only the last one is
    /// considered.
    ///
    /// The dimensions are given in surface-local coordinates.
    /// Setting width=height=0 removes the already-set rectangle.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_set_rectangle(
        &self,
        surface: &Rc<WlSurface>,
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
            arg4,
        ) = (
            surface,
            x,
            y,
            width,
            height,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_handle_v1#{}.set_rectangle(surface: wl_surface#{}, x: {}, y: {}, width: {}, height: {})\n", id, arg0_id, arg1, arg2, arg3, arg4);
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
            6,
            arg0_id,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
            arg4 as u32,
        ]);
        Ok(())
    }

    /// Since when the closed message is available.
    pub const MSG__CLOSED__SINCE: u32 = 1;

    /// this toplevel has been destroyed
    ///
    /// This event means the toplevel has been destroyed. It is guaranteed there
    /// won't be any more events for this treeland_foreign_toplevel_handle_v1. The
    /// toplevel itself becomes inert so any requests will be ignored except the
    /// destroy request.
    #[inline]
    pub fn send_closed(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_handle_v1#{}.closed()\n", client.endpoint.id, id);
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
            8,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the treeland_foreign_toplevel_handle_v1 object
    ///
    /// Destroys the treeland_foreign_toplevel_handle_v1 object.
    ///
    /// This request should be called either when the client does not want to
    /// use the toplevel anymore or after the closed event to finalize the
    /// destruction of the object.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_handle_v1#{}.destroy()\n", id);
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
            7,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the set_fullscreen message is available.
    pub const MSG__SET_FULLSCREEN__SINCE: u32 = 1;

    /// request that the toplevel be fullscreened
    ///
    /// Requests that the toplevel be fullscreened on the given output. If the
    /// fullscreen state and/or the outputs the toplevel is visible on actually
    /// change, this will be indicated by the state and output_enter/leave
    /// events.
    ///
    /// The output parameter is only a hint to the compositor. Also, if output
    /// is NULL, the compositor should decide which output the toplevel will be
    /// fullscreened on, if at all.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn send_set_fullscreen(
        &self,
        output: Option<&Rc<WlOutput>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("output")),
                Some(id) => id,
            },
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_handle_v1#{}.set_fullscreen(output: wl_output#{})\n", id, arg0_id);
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
            8,
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the unset_fullscreen message is available.
    pub const MSG__UNSET_FULLSCREEN__SINCE: u32 = 1;

    /// request that the toplevel be unfullscreened
    ///
    /// Requests that the toplevel be unfullscreened. If the fullscreen state
    /// actually changes, this will be indicated by the state event.
    #[inline]
    pub fn send_unset_fullscreen(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_foreign_toplevel_handle_v1#{}.unset_fullscreen()\n", id);
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
            9,
        ]);
        Ok(())
    }

    /// Since when the parent message is available.
    pub const MSG__PARENT__SINCE: u32 = 1;

    /// parent change
    ///
    /// This event is emitted whenever the parent of the toplevel changes.
    ///
    /// No event is emitted when the parent handle is destroyed by the client.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    #[inline]
    pub fn send_parent(
        &self,
        parent: Option<&Rc<TreelandForeignToplevelHandleV1>>,
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if let Some(arg0) = arg0 {
            if arg0.client_id.get() != Some(client.endpoint.id) {
                return Err(ObjectError::ArgNoClientId("parent", client.endpoint.id));
            }
        }
        let arg0_id = arg0.and_then(|arg0| arg0.client_obj_id.get()).unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_foreign_toplevel_handle_v1#{}.parent(parent: treeland_foreign_toplevel_handle_v1#{})\n", client.endpoint.id, id, arg0_id);
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
            9,
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandForeignToplevelHandleV1] proxies.
pub trait TreelandForeignToplevelHandleV1Handler: Any {
    /// Process id of application owning the window has changed
    ///
    /// This event will be sent when the compositor has set the process id this window
    /// belongs to. This should be set once before the initial_state is sent.
    ///
    /// # Arguments
    ///
    /// - `pid`:
    #[inline]
    fn pid(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        pid: u32,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_pid(
            pid,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.pid message: {}", Report::new(e));
        }
    }

    /// title change
    ///
    /// This event is emitted whenever the title of the toplevel changes.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    fn title(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        title: &str,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_title(
            title,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.title message: {}", Report::new(e));
        }
    }

    /// app-id change
    ///
    /// This event is emitted whenever the app-id of the toplevel changes.
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    fn app_id(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        app_id: &str,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_app_id(
            app_id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.app_id message: {}", Report::new(e));
        }
    }

    /// a stable identifier for a toplevel
    ///
    /// The identifier of each top level and its handle must be unique.
    /// Two different top layers cannot have the same identifier.
    /// This identifier is only valid as long as the top level is mapped.
    /// Identifiers must not be reused if the top level is not mapped.
    /// The compositor must not reuse identifiers to ensure there are no races when
    /// identifiers are shared between processes.
    ///
    /// # Arguments
    ///
    /// - `identifier`:
    #[inline]
    fn identifier(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        identifier: u32,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_identifier(
            identifier,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.identifier message: {}", Report::new(e));
        }
    }

    /// toplevel entered an output
    ///
    /// This event is emitted whenever the toplevel becomes visible on
    /// the given output. A toplevel may be visible on multiple outputs.
    ///
    /// # Arguments
    ///
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn output_enter(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        output: &Rc<WlOutput>,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        if output.core().zombie.get() {
            return;
        }
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = output.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_output_enter(
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.output_enter message: {}", Report::new(e));
        }
    }

    /// toplevel left an output
    ///
    /// This event is emitted whenever the toplevel stops being visible on
    /// the given output. It is guaranteed that an entered-output event
    /// with the same output has been emitted before this event.
    ///
    /// # Arguments
    ///
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn output_leave(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        output: &Rc<WlOutput>,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        if output.core().zombie.get() {
            return;
        }
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = output.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_output_leave(
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.output_leave message: {}", Report::new(e));
        }
    }

    /// requests that the toplevel be maximized
    ///
    /// Requests that the toplevel be maximized. If the maximized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    fn set_maximized(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_set_maximized(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.set_maximized message: {}", Report::new(e));
        }
    }

    /// requests that the toplevel be unmaximized
    ///
    /// Requests that the toplevel be unmaximized. If the maximized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    fn unset_maximized(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_unset_maximized(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.unset_maximized message: {}", Report::new(e));
        }
    }

    /// requests that the toplevel be minimized
    ///
    /// Requests that the toplevel be minimized. If the minimized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    fn set_minimized(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_set_minimized(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.set_minimized message: {}", Report::new(e));
        }
    }

    /// requests that the toplevel be unminimized
    ///
    /// Requests that the toplevel be unminimized. If the minimized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    fn unset_minimized(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_unset_minimized(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.unset_minimized message: {}", Report::new(e));
        }
    }

    /// activate the toplevel
    ///
    /// Request that this toplevel be activated on the given seat.
    /// There is no guarantee the toplevel will be actually activated.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn activate(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        seat: &Rc<WlSeat>,
    ) {
        let res = _slf.send_activate(
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.activate message: {}", Report::new(e));
        }
    }

    /// the toplevel state changed
    ///
    /// This event is emitted immediately after the treeland_foreign_toplevel_handle_v1
    /// is created and each time the toplevel state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn state(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        state: &[u8],
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_state(
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.state message: {}", Report::new(e));
        }
    }

    /// all information about the toplevel has been sent
    ///
    /// This event is sent after all changes in the toplevel state have been
    /// sent.
    ///
    /// This allows changes to the treeland_foreign_toplevel_handle_v1 properties
    /// to be seen as atomic, even if they happen via multiple events.
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.done message: {}", Report::new(e));
        }
    }

    /// request that the toplevel be closed
    ///
    /// Send a request to the toplevel to close itself. The compositor would
    /// typically use a shell-specific method to carry out this request, for
    /// example by sending the xdg_toplevel.close event. However, this gives
    /// no guarantees the toplevel will actually be destroyed. If and when
    /// this happens, the treeland_foreign_toplevel_handle_v1.closed event will
    /// be emitted.
    #[inline]
    fn close(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_close(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.close message: {}", Report::new(e));
        }
    }

    /// the rectangle which represents the toplevel
    ///
    /// The rectangle of the surface specified in this request corresponds to
    /// the place where the app using this protocol represents the given toplevel.
    /// It can be used by the compositor as a hint for some operations, e.g
    /// minimizing. The client is however not required to set this, in which
    /// case the compositor is free to decide some default value.
    ///
    /// If the client specifies more than one rectangle, only the last one is
    /// considered.
    ///
    /// The dimensions are given in surface-local coordinates.
    /// Setting width=height=0 removes the already-set rectangle.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_rectangle(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        surface: &Rc<WlSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_set_rectangle(
            surface,
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.set_rectangle message: {}", Report::new(e));
        }
    }

    /// this toplevel has been destroyed
    ///
    /// This event means the toplevel has been destroyed. It is guaranteed there
    /// won't be any more events for this treeland_foreign_toplevel_handle_v1. The
    /// toplevel itself becomes inert so any requests will be ignored except the
    /// destroy request.
    #[inline]
    fn closed(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_closed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.closed message: {}", Report::new(e));
        }
    }

    /// destroy the treeland_foreign_toplevel_handle_v1 object
    ///
    /// Destroys the treeland_foreign_toplevel_handle_v1 object.
    ///
    /// This request should be called either when the client does not want to
    /// use the toplevel anymore or after the closed event to finalize the
    /// destruction of the object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.destroy message: {}", Report::new(e));
        }
    }

    /// request that the toplevel be fullscreened
    ///
    /// Requests that the toplevel be fullscreened on the given output. If the
    /// fullscreen state and/or the outputs the toplevel is visible on actually
    /// change, this will be indicated by the state and output_enter/leave
    /// events.
    ///
    /// The output parameter is only a hint to the compositor. Also, if output
    /// is NULL, the compositor should decide which output the toplevel will be
    /// fullscreened on, if at all.
    ///
    /// # Arguments
    ///
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_fullscreen(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        output: Option<&Rc<WlOutput>>,
    ) {
        let res = _slf.send_set_fullscreen(
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.set_fullscreen message: {}", Report::new(e));
        }
    }

    /// request that the toplevel be unfullscreened
    ///
    /// Requests that the toplevel be unfullscreened. If the fullscreen state
    /// actually changes, this will be indicated by the state event.
    #[inline]
    fn unset_fullscreen(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_unset_fullscreen(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.unset_fullscreen message: {}", Report::new(e));
        }
    }

    /// parent change
    ///
    /// This event is emitted whenever the parent of the toplevel changes.
    ///
    /// No event is emitted when the parent handle is destroyed by the client.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn parent(
        &mut self,
        _slf: &Rc<TreelandForeignToplevelHandleV1>,
        parent: Option<&Rc<TreelandForeignToplevelHandleV1>>,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        if let Some(parent) = parent {
            if parent.core().zombie.get() {
                return;
            }
        }
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(parent) = parent {
                if let Some(client_id_2) = parent.core().client_id.get() {
                    if client_id != client_id_2 {
                        return;
                    }
                }
            }
        }
        let res = _slf.send_parent(
            parent,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_foreign_toplevel_handle_v1.parent message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandForeignToplevelHandleV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandForeignToplevelHandleV1, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_handle_v1#{}.set_maximized()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_maximized(&self);
                } else {
                    DefaultHandler.set_maximized(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_handle_v1#{}.unset_maximized()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).unset_maximized(&self);
                } else {
                    DefaultHandler.unset_maximized(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_handle_v1#{}.set_minimized()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_minimized(&self);
                } else {
                    DefaultHandler.set_minimized(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_handle_v1#{}.unset_minimized()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).unset_minimized(&self);
                } else {
                    DefaultHandler.unset_minimized(&self);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_handle_v1#{}.activate(seat: wl_seat#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).activate(&self, arg0);
                } else {
                    DefaultHandler.activate(&self, arg0);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_handle_v1#{}.close()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).close(&self);
                } else {
                    DefaultHandler.close(&self);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_handle_v1#{}.set_rectangle(surface: wl_surface#{}, x: {}, y: {}, width: {}, height: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_rectangle(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.set_rectangle(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_handle_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            8 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_handle_v1#{}.set_fullscreen(output: wl_output#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).set_fullscreen(&self, arg0);
                } else {
                    DefaultHandler.set_fullscreen(&self, arg0);
                }
            }
            9 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_foreign_toplevel_handle_v1#{}.unset_fullscreen()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).unset_fullscreen(&self);
                } else {
                    DefaultHandler.unset_fullscreen(&self);
                }
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_handle_v1#{}.pid(pid: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).pid(&self, arg0);
                } else {
                    DefaultHandler.pid(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("title"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("title"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("title"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("title"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_handle_v1#{}.title(title: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).title(&self, arg0);
                } else {
                    DefaultHandler.title(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("app_id"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("app_id"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("app_id"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("app_id"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_handle_v1#{}.app_id(app_id: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).app_id(&self, arg0);
                } else {
                    DefaultHandler.app_id(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_handle_v1#{}.identifier(identifier: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).identifier(&self, arg0);
                } else {
                    DefaultHandler.identifier(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_handle_v1#{}.output_enter(output: wl_output#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).output_enter(&self, arg0);
                } else {
                    DefaultHandler.output_enter(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_handle_v1#{}.output_leave(output: wl_output#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                    return Err(ObjectError::NoServerObject(arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = self.core.state.server.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).output_leave(&self, arg0);
                } else {
                    DefaultHandler.output_leave(&self, arg0);
                }
            }
            6 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("state"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("state"));
                    }
                    let start = offset;
                    offset += words;
                    &uapi::as_bytes(&msg[start..])[..len]
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_handle_v1#{}.state(state: {})\n", msg[0], debug_array(arg0));
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).state(&self, arg0);
                } else {
                    DefaultHandler.state(&self, arg0);
                }
            }
            7 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_handle_v1#{}.done()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).done(&self);
                } else {
                    DefaultHandler.done(&self);
                }
            }
            8 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_handle_v1#{}.closed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).closed(&self);
                } else {
                    DefaultHandler.closed(&self);
                }
            }
            9 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_foreign_toplevel_handle_v1#{}.parent(parent: treeland_foreign_toplevel_handle_v1#{})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = self.core.state.server.lookup(arg0_id) else {
                        return Err(ObjectError::NoServerObject(arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<TreelandForeignToplevelHandleV1>() else {
                        let o = self.core.state.server.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("parent", o.core().interface, ProxyInterface::TreelandForeignToplevelHandleV1));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).parent(&self, arg0);
                } else {
                    DefaultHandler.parent(&self, arg0);
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
            0 => "set_maximized",
            1 => "unset_maximized",
            2 => "set_minimized",
            3 => "unset_minimized",
            4 => "activate",
            5 => "close",
            6 => "set_rectangle",
            7 => "destroy",
            8 => "set_fullscreen",
            9 => "unset_fullscreen",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "pid",
            1 => "title",
            2 => "app_id",
            3 => "identifier",
            4 => "output_enter",
            5 => "output_leave",
            6 => "state",
            7 => "done",
            8 => "closed",
            9 => "parent",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for TreelandForeignToplevelHandleV1 {
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

impl TreelandForeignToplevelHandleV1 {
    /// Since when the state.maximized enum variant is available.
    pub const ENM__STATE_MAXIMIZED__SINCE: u32 = 1;
    /// Since when the state.minimized enum variant is available.
    pub const ENM__STATE_MINIMIZED__SINCE: u32 = 1;
    /// Since when the state.activated enum variant is available.
    pub const ENM__STATE_ACTIVATED__SINCE: u32 = 1;
    /// Since when the state.fullscreen enum variant is available.
    pub const ENM__STATE_FULLSCREEN__SINCE: u32 = 1;

    /// Since when the error.invalid_rectangle enum variant is available.
    pub const ENM__ERROR_INVALID_RECTANGLE__SINCE: u32 = 1;
}

/// types of states on the toplevel
///
/// The different states that a toplevel can have. These have the same meaning
/// as the states with the same names defined in xdg-toplevel
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandForeignToplevelHandleV1State(pub u32);

impl TreelandForeignToplevelHandleV1State {
    /// the toplevel is maximized
    pub const MAXIMIZED: Self = Self(0);

    /// the toplevel is minimized
    pub const MINIMIZED: Self = Self(1);

    /// the toplevel is active
    pub const ACTIVATED: Self = Self(2);

    /// the toplevel is fullscreen
    pub const FULLSCREEN: Self = Self(3);
}

impl Debug for TreelandForeignToplevelHandleV1State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::MAXIMIZED => "MAXIMIZED",
            Self::MINIMIZED => "MINIMIZED",
            Self::ACTIVATED => "ACTIVATED",
            Self::FULLSCREEN => "FULLSCREEN",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandForeignToplevelHandleV1Error(pub u32);

impl TreelandForeignToplevelHandleV1Error {
    /// the provided rectangle is invalid
    pub const INVALID_RECTANGLE: Self = Self(0);
}

impl Debug for TreelandForeignToplevelHandleV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_RECTANGLE => "INVALID_RECTANGLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
