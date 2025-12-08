//! an opened toplevel
//!
//! A zwlr_foreign_toplevel_handle_v1 object represents an opened toplevel
//! window. Each app may have multiple opened toplevels.
//!
//! Each toplevel has a list of outputs it is visible on, conveyed to the
//! client with the output_enter and output_leave events.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_foreign_toplevel_handle_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrForeignToplevelHandleV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrForeignToplevelHandleV1Handler>,
}

struct DefaultHandler;

impl ZwlrForeignToplevelHandleV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrForeignToplevelHandleV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrForeignToplevelHandleV1;
    const INTERFACE_NAME: &str = "zwlr_foreign_toplevel_handle_v1";
}

impl ZwlrForeignToplevelHandleV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrForeignToplevelHandleV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrForeignToplevelHandleV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrForeignToplevelHandleV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrForeignToplevelHandleV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrForeignToplevelHandleV1 {
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
    pub fn try_send_title(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_foreign_toplevel_handle_v1#{}.title(title: {:?})\n", client_id, id, arg0);
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
            0,
        ]);
        fmt.string(arg0);
        Ok(())
    }

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
    ) {
        let res = self.try_send_title(
            title,
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.title", &e);
        }
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
    pub fn try_send_app_id(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_foreign_toplevel_handle_v1#{}.app_id(app_id: {:?})\n", client_id, id, arg0);
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
        ]);
        fmt.string(arg0);
        Ok(())
    }

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
    ) {
        let res = self.try_send_app_id(
            app_id,
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.app_id", &e);
        }
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
    pub fn try_send_output_enter(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("output", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_foreign_toplevel_handle_v1#{}.output_enter(output: wl_output#{})\n", client_id, id, arg0);
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
    ) {
        let res = self.try_send_output_enter(
            output,
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.output_enter", &e);
        }
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
    pub fn try_send_output_leave(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("output", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_foreign_toplevel_handle_v1#{}.output_leave(output: wl_output#{})\n", client_id, id, arg0);
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
            3,
            arg0_id,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_output_leave(
        &self,
        output: &Rc<WlOutput>,
    ) {
        let res = self.try_send_output_leave(
            output,
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.output_leave", &e);
        }
    }

    /// Since when the set_maximized message is available.
    pub const MSG__SET_MAXIMIZED__SINCE: u32 = 1;

    /// requests that the toplevel be maximized
    ///
    /// Requests that the toplevel be maximized. If the maximized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn try_send_set_maximized(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_handle_v1#{}.set_maximized()\n", id);
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

    /// requests that the toplevel be maximized
    ///
    /// Requests that the toplevel be maximized. If the maximized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn send_set_maximized(
        &self,
    ) {
        let res = self.try_send_set_maximized(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.set_maximized", &e);
        }
    }

    /// Since when the unset_maximized message is available.
    pub const MSG__UNSET_MAXIMIZED__SINCE: u32 = 1;

    /// requests that the toplevel be unmaximized
    ///
    /// Requests that the toplevel be unmaximized. If the maximized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn try_send_unset_maximized(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_handle_v1#{}.unset_maximized()\n", id);
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

    /// requests that the toplevel be unmaximized
    ///
    /// Requests that the toplevel be unmaximized. If the maximized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn send_unset_maximized(
        &self,
    ) {
        let res = self.try_send_unset_maximized(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.unset_maximized", &e);
        }
    }

    /// Since when the set_minimized message is available.
    pub const MSG__SET_MINIMIZED__SINCE: u32 = 1;

    /// requests that the toplevel be minimized
    ///
    /// Requests that the toplevel be minimized. If the minimized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn try_send_set_minimized(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_handle_v1#{}.set_minimized()\n", id);
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

    /// requests that the toplevel be minimized
    ///
    /// Requests that the toplevel be minimized. If the minimized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn send_set_minimized(
        &self,
    ) {
        let res = self.try_send_set_minimized(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.set_minimized", &e);
        }
    }

    /// Since when the unset_minimized message is available.
    pub const MSG__UNSET_MINIMIZED__SINCE: u32 = 1;

    /// requests that the toplevel be unminimized
    ///
    /// Requests that the toplevel be unminimized. If the minimized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn try_send_unset_minimized(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_handle_v1#{}.unset_minimized()\n", id);
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

    /// requests that the toplevel be unminimized
    ///
    /// Requests that the toplevel be unminimized. If the minimized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    pub fn send_unset_minimized(
        &self,
    ) {
        let res = self.try_send_unset_minimized(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.unset_minimized", &e);
        }
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
    pub fn try_send_activate(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_handle_v1#{}.activate(seat: wl_seat#{})\n", id, arg0);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

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
    ) {
        let res = self.try_send_activate(
            seat,
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.activate", &e);
        }
    }

    /// Since when the state message is available.
    pub const MSG__STATE__SINCE: u32 = 1;

    /// the toplevel state changed
    ///
    /// This event is emitted immediately after the zlw_foreign_toplevel_handle_v1
    /// is created and each time the toplevel state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    pub fn try_send_state(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_foreign_toplevel_handle_v1#{}.state(state: {})\n", client_id, id, debug_array(arg0));
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
        fmt.array(arg0);
        Ok(())
    }

    /// the toplevel state changed
    ///
    /// This event is emitted immediately after the zlw_foreign_toplevel_handle_v1
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
    ) {
        let res = self.try_send_state(
            state,
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.state", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all information about the toplevel has been sent
    ///
    /// This event is sent after all changes in the toplevel state have been
    /// sent.
    ///
    /// This allows changes to the zwlr_foreign_toplevel_handle_v1 properties
    /// to be seen as atomic, even if they happen via multiple events.
    #[inline]
    pub fn try_send_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_foreign_toplevel_handle_v1#{}.done()\n", client_id, id);
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
            5,
        ]);
        Ok(())
    }

    /// all information about the toplevel has been sent
    ///
    /// This event is sent after all changes in the toplevel state have been
    /// sent.
    ///
    /// This allows changes to the zwlr_foreign_toplevel_handle_v1 properties
    /// to be seen as atomic, even if they happen via multiple events.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.done", &e);
        }
    }

    /// Since when the close message is available.
    pub const MSG__CLOSE__SINCE: u32 = 1;

    /// request that the toplevel be closed
    ///
    /// Send a request to the toplevel to close itself. The compositor would
    /// typically use a shell-specific method to carry out this request, for
    /// example by sending the xdg_toplevel.close event. However, this gives
    /// no guarantees the toplevel will actually be destroyed. If and when
    /// this happens, the zwlr_foreign_toplevel_handle_v1.closed event will
    /// be emitted.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_handle_v1#{}.close()\n", id);
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

    /// request that the toplevel be closed
    ///
    /// Send a request to the toplevel to close itself. The compositor would
    /// typically use a shell-specific method to carry out this request, for
    /// example by sending the xdg_toplevel.close event. However, this gives
    /// no guarantees the toplevel will actually be destroyed. If and when
    /// this happens, the zwlr_foreign_toplevel_handle_v1.closed event will
    /// be emitted.
    #[inline]
    pub fn send_close(
        &self,
    ) {
        let res = self.try_send_close(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.close", &e);
        }
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
    pub fn try_send_set_rectangle(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_handle_v1#{}.set_rectangle(surface: wl_surface#{}, x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2, arg3, arg4);
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
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
            arg4 as u32,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_set_rectangle(
        &self,
        surface: &Rc<WlSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_set_rectangle(
            surface,
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.set_rectangle", &e);
        }
    }

    /// Since when the closed message is available.
    pub const MSG__CLOSED__SINCE: u32 = 1;

    /// this toplevel has been destroyed
    ///
    /// This event means the toplevel has been destroyed. It is guaranteed there
    /// won't be any more events for this zwlr_foreign_toplevel_handle_v1. The
    /// toplevel itself becomes inert so any requests will be ignored except the
    /// destroy request.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_foreign_toplevel_handle_v1#{}.closed()\n", client_id, id);
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
            6,
        ]);
        Ok(())
    }

    /// this toplevel has been destroyed
    ///
    /// This event means the toplevel has been destroyed. It is guaranteed there
    /// won't be any more events for this zwlr_foreign_toplevel_handle_v1. The
    /// toplevel itself becomes inert so any requests will be ignored except the
    /// destroy request.
    #[inline]
    pub fn send_closed(
        &self,
    ) {
        let res = self.try_send_closed(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.closed", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the zwlr_foreign_toplevel_handle_v1 object
    ///
    /// Destroys the zwlr_foreign_toplevel_handle_v1 object.
    ///
    /// This request should be called either when the client does not want to
    /// use the toplevel anymore or after the closed event to finalize the
    /// destruction of the object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_handle_v1#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the zwlr_foreign_toplevel_handle_v1 object
    ///
    /// Destroys the zwlr_foreign_toplevel_handle_v1 object.
    ///
    /// This request should be called either when the client does not want to
    /// use the toplevel anymore or after the closed event to finalize the
    /// destruction of the object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.destroy", &e);
        }
    }

    /// Since when the set_fullscreen message is available.
    pub const MSG__SET_FULLSCREEN__SINCE: u32 = 2;

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
    pub fn try_send_set_fullscreen(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_handle_v1#{}.set_fullscreen(output: wl_output#{})\n", id, arg0);
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
            8,
            arg0_id,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_set_fullscreen(
        &self,
        output: Option<&Rc<WlOutput>>,
    ) {
        let res = self.try_send_set_fullscreen(
            output,
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.set_fullscreen", &e);
        }
    }

    /// Since when the unset_fullscreen message is available.
    pub const MSG__UNSET_FULLSCREEN__SINCE: u32 = 2;

    /// request that the toplevel be unfullscreened
    ///
    /// Requests that the toplevel be unfullscreened. If the fullscreen state
    /// actually changes, this will be indicated by the state event.
    #[inline]
    pub fn try_send_unset_fullscreen(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_foreign_toplevel_handle_v1#{}.unset_fullscreen()\n", id);
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
            9,
        ]);
        Ok(())
    }

    /// request that the toplevel be unfullscreened
    ///
    /// Requests that the toplevel be unfullscreened. If the fullscreen state
    /// actually changes, this will be indicated by the state event.
    #[inline]
    pub fn send_unset_fullscreen(
        &self,
    ) {
        let res = self.try_send_unset_fullscreen(
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.unset_fullscreen", &e);
        }
    }

    /// Since when the parent message is available.
    pub const MSG__PARENT__SINCE: u32 = 3;

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
    pub fn try_send_parent(
        &self,
        parent: Option<&Rc<ZwlrForeignToplevelHandleV1>>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwlr_foreign_toplevel_handle_v1#{}.parent(parent: zwlr_foreign_toplevel_handle_v1#{})\n", client_id, id, arg0);
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
        parent: Option<&Rc<ZwlrForeignToplevelHandleV1>>,
    ) {
        let res = self.try_send_parent(
            parent,
        );
        if let Err(e) = res {
            log_send("zwlr_foreign_toplevel_handle_v1.parent", &e);
        }
    }
}

/// A message handler for [`ZwlrForeignToplevelHandleV1`] proxies.
pub trait ZwlrForeignToplevelHandleV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrForeignToplevelHandleV1>) {
        slf.core.delete_id();
    }

    /// title change
    ///
    /// This event is emitted whenever the title of the toplevel changes.
    ///
    /// # Arguments
    ///
    /// - `title`:
    #[inline]
    fn handle_title(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
        title: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_title(
            title,
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.title", &e);
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
    fn handle_app_id(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
        app_id: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_app_id(
            app_id,
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.app_id", &e);
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
    fn handle_output_enter(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
        output: &Rc<WlOutput>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = output.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_output_enter(
            output,
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.output_enter", &e);
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
    fn handle_output_leave(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
        output: &Rc<WlOutput>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = output.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_output_leave(
            output,
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.output_leave", &e);
        }
    }

    /// requests that the toplevel be maximized
    ///
    /// Requests that the toplevel be maximized. If the maximized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    fn handle_set_maximized(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_maximized(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.set_maximized", &e);
        }
    }

    /// requests that the toplevel be unmaximized
    ///
    /// Requests that the toplevel be unmaximized. If the maximized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    fn handle_unset_maximized(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unset_maximized(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.unset_maximized", &e);
        }
    }

    /// requests that the toplevel be minimized
    ///
    /// Requests that the toplevel be minimized. If the minimized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    fn handle_set_minimized(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_minimized(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.set_minimized", &e);
        }
    }

    /// requests that the toplevel be unminimized
    ///
    /// Requests that the toplevel be unminimized. If the minimized state actually
    /// changes, this will be indicated by the state event.
    #[inline]
    fn handle_unset_minimized(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unset_minimized(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.unset_minimized", &e);
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
    fn handle_activate(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
        seat: &Rc<WlSeat>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_activate(
            seat,
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.activate", &e);
        }
    }

    /// the toplevel state changed
    ///
    /// This event is emitted immediately after the zlw_foreign_toplevel_handle_v1
    /// is created and each time the toplevel state changes, either because of a
    /// compositor action or because of a request in this protocol.
    ///
    /// # Arguments
    ///
    /// - `state`:
    #[inline]
    fn handle_state(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
        state: &[u8],
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_state(
            state,
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.state", &e);
        }
    }

    /// all information about the toplevel has been sent
    ///
    /// This event is sent after all changes in the toplevel state have been
    /// sent.
    ///
    /// This allows changes to the zwlr_foreign_toplevel_handle_v1 properties
    /// to be seen as atomic, even if they happen via multiple events.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.done", &e);
        }
    }

    /// request that the toplevel be closed
    ///
    /// Send a request to the toplevel to close itself. The compositor would
    /// typically use a shell-specific method to carry out this request, for
    /// example by sending the xdg_toplevel.close event. However, this gives
    /// no guarantees the toplevel will actually be destroyed. If and when
    /// this happens, the zwlr_foreign_toplevel_handle_v1.closed event will
    /// be emitted.
    #[inline]
    fn handle_close(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_close(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.close", &e);
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
    fn handle_set_rectangle(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
        surface: &Rc<WlSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_rectangle(
            surface,
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.set_rectangle", &e);
        }
    }

    /// this toplevel has been destroyed
    ///
    /// This event means the toplevel has been destroyed. It is guaranteed there
    /// won't be any more events for this zwlr_foreign_toplevel_handle_v1. The
    /// toplevel itself becomes inert so any requests will be ignored except the
    /// destroy request.
    #[inline]
    fn handle_closed(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_closed(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.closed", &e);
        }
    }

    /// destroy the zwlr_foreign_toplevel_handle_v1 object
    ///
    /// Destroys the zwlr_foreign_toplevel_handle_v1 object.
    ///
    /// This request should be called either when the client does not want to
    /// use the toplevel anymore or after the closed event to finalize the
    /// destruction of the object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.destroy", &e);
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
    fn handle_set_fullscreen(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
        output: Option<&Rc<WlOutput>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_fullscreen(
            output,
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.set_fullscreen", &e);
        }
    }

    /// request that the toplevel be unfullscreened
    ///
    /// Requests that the toplevel be unfullscreened. If the fullscreen state
    /// actually changes, this will be indicated by the state event.
    #[inline]
    fn handle_unset_fullscreen(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unset_fullscreen(
        );
        if let Err(e) = res {
            log_forward("zwlr_foreign_toplevel_handle_v1.unset_fullscreen", &e);
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
    fn handle_parent(
        &mut self,
        slf: &Rc<ZwlrForeignToplevelHandleV1>,
        parent: Option<&Rc<ZwlrForeignToplevelHandleV1>>,
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
            log_forward("zwlr_foreign_toplevel_handle_v1.parent", &e);
        }
    }
}

impl ObjectPrivate for ZwlrForeignToplevelHandleV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrForeignToplevelHandleV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_handle_v1#{}.set_maximized()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_maximized(&self);
                } else {
                    DefaultHandler.handle_set_maximized(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_handle_v1#{}.unset_maximized()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unset_maximized(&self);
                } else {
                    DefaultHandler.handle_unset_maximized(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_handle_v1#{}.set_minimized()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_minimized(&self);
                } else {
                    DefaultHandler.handle_set_minimized(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_handle_v1#{}.unset_minimized()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unset_minimized(&self);
                } else {
                    DefaultHandler.handle_unset_minimized(&self);
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_handle_v1#{}.activate(seat: wl_seat#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_activate(&self, arg0);
                } else {
                    DefaultHandler.handle_activate(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_handle_v1#{}.close()\n", client_id, id);
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
            6 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_handle_v1#{}.set_rectangle(surface: wl_surface#{}, x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_rectangle(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_set_rectangle(&self, arg0, arg1, arg2, arg3, arg4);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_handle_v1#{}.destroy()\n", client_id, id);
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
            8 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_handle_v1#{}.set_fullscreen(output: wl_output#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_fullscreen(&self, arg0);
                } else {
                    DefaultHandler.handle_set_fullscreen(&self, arg0);
                }
            }
            9 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_foreign_toplevel_handle_v1#{}.unset_fullscreen()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unset_fullscreen(&self);
                } else {
                    DefaultHandler.handle_unset_fullscreen(&self);
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "title")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_foreign_toplevel_handle_v1#{}.title(title: {:?})\n", id, arg0);
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
            1 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "app_id")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_foreign_toplevel_handle_v1#{}.app_id(app_id: {:?})\n", id, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_foreign_toplevel_handle_v1#{}.output_enter(output: wl_output#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_output_enter(&self, arg0);
                } else {
                    DefaultHandler.handle_output_enter(&self, arg0);
                }
            }
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_foreign_toplevel_handle_v1#{}.output_leave(output: wl_output#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_output_leave(&self, arg0);
                } else {
                    DefaultHandler.handle_output_leave(&self, arg0);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_array(msg, offset, "state")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_foreign_toplevel_handle_v1#{}.state(state: {})\n", id, debug_array(arg0));
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_state(&self, arg0);
                } else {
                    DefaultHandler.handle_state(&self, arg0);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_foreign_toplevel_handle_v1#{}.done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_foreign_toplevel_handle_v1#{}.closed()\n", id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwlr_foreign_toplevel_handle_v1#{}.parent(parent: zwlr_foreign_toplevel_handle_v1#{})\n", id, arg0);
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
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ZwlrForeignToplevelHandleV1>() else {
                        let o = server.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("parent", o.core().interface, ObjectInterface::ZwlrForeignToplevelHandleV1)));
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
            0 => "title",
            1 => "app_id",
            2 => "output_enter",
            3 => "output_leave",
            4 => "state",
            5 => "done",
            6 => "closed",
            7 => "parent",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwlrForeignToplevelHandleV1 {
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

impl ZwlrForeignToplevelHandleV1 {
    /// Since when the state.maximized enum variant is available.
    pub const ENM__STATE_MAXIMIZED__SINCE: u32 = 1;
    /// Since when the state.minimized enum variant is available.
    pub const ENM__STATE_MINIMIZED__SINCE: u32 = 1;
    /// Since when the state.activated enum variant is available.
    pub const ENM__STATE_ACTIVATED__SINCE: u32 = 1;
    /// Since when the state.fullscreen enum variant is available.
    pub const ENM__STATE_FULLSCREEN__SINCE: u32 = 2;

    /// Since when the error.invalid_rectangle enum variant is available.
    pub const ENM__ERROR_INVALID_RECTANGLE__SINCE: u32 = 1;
}

/// types of states on the toplevel
///
/// The different states that a toplevel can have. These have the same meaning
/// as the states with the same names defined in xdg-toplevel
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrForeignToplevelHandleV1State(pub u32);

impl ZwlrForeignToplevelHandleV1State {
    /// the toplevel is maximized
    pub const MAXIMIZED: Self = Self(0);

    /// the toplevel is minimized
    pub const MINIMIZED: Self = Self(1);

    /// the toplevel is active
    pub const ACTIVATED: Self = Self(2);

    /// the toplevel is fullscreen
    pub const FULLSCREEN: Self = Self(3);
}

impl Debug for ZwlrForeignToplevelHandleV1State {
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
pub struct ZwlrForeignToplevelHandleV1Error(pub u32);

impl ZwlrForeignToplevelHandleV1Error {
    /// the provided rectangle is invalid
    pub const INVALID_RECTANGLE: Self = Self(0);
}

impl Debug for ZwlrForeignToplevelHandleV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_RECTANGLE => "INVALID_RECTANGLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
