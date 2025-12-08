//! window manager global interface
//!
//! This global interface should only be advertised to the window manager
//! process. Only one window management client may be active at a time. The
//! compositor should use the unavailable event if necessary to enforce this.
//!
//! There are two disjoint categories of state managed by this protocol:
//!
//! Window management state influences the communication between the server
//! and individual window clients (e.g. xdg_toplevels). Window management
//! state includes window dimensions, fullscreen state, keyboard focus,
//! keyboard bindings, and more.
//!
//! Rendering state only affects the rendered output of the compositor and
//! does not influence communication between the server and individual window
//! clients. Rendering state includes the position and rendering order of
//! windows, shell surfaces, decoration surfaces, borders, and more.
//!
//! Window management state may only be modified by the window manager as part
//! of a manage sequence. A manage sequence is started with the manage_start
//! event and ended with the manage_finish request. It is a protocol error to
//! modify window management state outside of a manage sequence.
//!
//! A manage sequence is always followed by at least one render sequence. A
//! render sequence is started with the render_start event and ended with the
//! render_finish request.
//!
//! Rendering state may be modified by the window manager during a manage
//! sequence or a render sequence. Regardless of when the rendering state is
//! modified, it is applied with the next render_finish request. It is a
//! protocol error to modify rendering state outside of a manage or render
//! sequence.
//!
//! The server will start a manage sequence by sending new state and the
//! manage_start event as soon as possible whenever there is a change in state
//! that must be communicated with the window manager.
//!
//! If the window manager client needs to ensure a manage sequence is started
//! due to a state change the compositor is not aware of, it may send the
//! manage_dirty request.
//!
//! The server will start a render sequence by sending new state and the
//! render_start event as soon as possible whenever there is a change in
//! window dimensions that must be communicated with the window manager.
//! Multiple render sequences may be made consecutively without a manage
//! sequence in between, for example if a window independently changes its own
//! dimensions.
//!
//! To summarize, the main loop of this protocol is as follows:
//!
//! 1. The server sends events indicating all changes since the last
//!    manage sequence followed by the manage_start event.
//!
//! 2. The client sends requests modifying window management state or
//!    rendering state (as defined above) followed by the manage_finish
//!    request.
//!
//! 3. The server sends new state to windows and waits for responses.
//!
//! 4. The server sends new window dimensions to the client followed by the
//!    render_start event.
//!
//! 5. The client sends requests modifying rendering state (as defined above)
//!    followed by the render_finish request.
//!
//! 6. If window dimensions change, loop back to step 4.
//!    If state that requires a manage sequence changes or if the client makes
//!    a manage_dirty request, loop back to step 1.
//!
//! For the purposes of frame perfection, the server may delay rendering new
//! state committed by the windows in step 3 until after step 5 is finished.
//!
//! It is a protocol error for the client to make a manage_finish or
//! render_finish request that violates this ordering.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_window_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverWindowManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverWindowManagerV1Handler>,
}

struct DefaultHandler;

impl RiverWindowManagerV1Handler for DefaultHandler { }

impl ConcreteObject for RiverWindowManagerV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverWindowManagerV1;
    const INTERFACE_NAME: &str = "river_window_manager_v1";
}

impl RiverWindowManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverWindowManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverWindowManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverWindowManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverWindowManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverWindowManagerV1 {
    /// Since when the unavailable message is available.
    pub const MSG__UNAVAILABLE__SINCE: u32 = 1;

    /// window management unavailable
    ///
    /// This event indicates that window management is not available to the
    /// client, perhaps due to another window management client already running.
    /// The circumstances causing this event to be sent are compositor policy.
    ///
    /// If sent, this event is guaranteed to be the first and only event sent by
    /// the server.
    ///
    /// The server will send no further events on this object. The client should
    /// destroy this object and all objects created through this interface.
    #[inline]
    pub fn try_send_unavailable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_manager_v1#{}.unavailable()\n", client_id, id);
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

    /// window management unavailable
    ///
    /// This event indicates that window management is not available to the
    /// client, perhaps due to another window management client already running.
    /// The circumstances causing this event to be sent are compositor policy.
    ///
    /// If sent, this event is guaranteed to be the first and only event sent by
    /// the server.
    ///
    /// The server will send no further events on this object. The client should
    /// destroy this object and all objects created through this interface.
    #[inline]
    pub fn send_unavailable(
        &self,
    ) {
        let res = self.try_send_unavailable(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.unavailable", &e);
        }
    }

    /// Since when the stop message is available.
    pub const MSG__STOP__SINCE: u32 = 1;

    /// stop sending events
    ///
    /// This request indicates that the client no longer wishes to receive
    /// events on this object.
    ///
    /// The Wayland protocol is asynchronous, which means the server may send
    /// further events until the stop request is processed. The client must wait
    /// for a river_window_manager_v1.finished event before destroying this
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_manager_v1#{}.stop()\n", id);
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
    /// for a river_window_manager_v1.finished event before destroying this
    /// object.
    #[inline]
    pub fn send_stop(
        &self,
    ) {
        let res = self.try_send_stop(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.stop", &e);
        }
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the server has finished with the window manager
    ///
    /// This event indicates that the server will send no further events on this
    /// object. The client should destroy the object. See
    /// river_window_manager_v1.destroy for more information.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_manager_v1#{}.finished()\n", client_id, id);
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

    /// the server has finished with the window manager
    ///
    /// This event indicates that the server will send no further events on this
    /// object. The client should destroy the object. See
    /// river_window_manager_v1.destroy for more information.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.finished", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the river_window_manager_v1 object
    ///
    /// This request should be called after the finished event has been received
    /// to complete destruction of the object.
    ///
    /// If a client wishes to destroy this object it should send a
    /// river_window_manager_v1.stop request and wait for a
    /// river_window_manager_v1.finished event. Once the finished event is
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_manager_v1#{}.destroy()\n", id);
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

    /// destroy the river_window_manager_v1 object
    ///
    /// This request should be called after the finished event has been received
    /// to complete destruction of the object.
    ///
    /// If a client wishes to destroy this object it should send a
    /// river_window_manager_v1.stop request and wait for a
    /// river_window_manager_v1.finished event. Once the finished event is
    /// received it is safe to destroy this object and any other objects created
    /// through this interface.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.destroy", &e);
        }
    }

    /// Since when the manage_start message is available.
    pub const MSG__MANAGE_START__SINCE: u32 = 1;

    /// start a manage sequence
    ///
    /// This event indicates that the server has sent events indicating all
    /// state changes since the last manage sequence.
    ///
    /// In response to this event, the client should make requests modifying
    /// window management state as it chooses. Then, the client must make the
    /// manage_finish request.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    pub fn try_send_manage_start(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_manager_v1#{}.manage_start()\n", client_id, id);
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
            2,
        ]);
        Ok(())
    }

    /// start a manage sequence
    ///
    /// This event indicates that the server has sent events indicating all
    /// state changes since the last manage sequence.
    ///
    /// In response to this event, the client should make requests modifying
    /// window management state as it chooses. Then, the client must make the
    /// manage_finish request.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    pub fn send_manage_start(
        &self,
    ) {
        let res = self.try_send_manage_start(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.manage_start", &e);
        }
    }

    /// Since when the manage_finish message is available.
    pub const MSG__MANAGE_FINISH__SINCE: u32 = 1;

    /// finish a manage sequence
    ///
    /// This request indicates that the client has made all changes to window
    /// management state it wishes to include in the current manage sequence and
    /// that the server should atomically send these state changes to the
    /// windows and continue with the manage sequence.
    ///
    /// After sending this request, it is a protocol error for the client to
    /// make further changes to window management state until the next
    /// manage_start event is received.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    pub fn try_send_manage_finish(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_manager_v1#{}.manage_finish()\n", id);
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

    /// finish a manage sequence
    ///
    /// This request indicates that the client has made all changes to window
    /// management state it wishes to include in the current manage sequence and
    /// that the server should atomically send these state changes to the
    /// windows and continue with the manage sequence.
    ///
    /// After sending this request, it is a protocol error for the client to
    /// make further changes to window management state until the next
    /// manage_start event is received.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    pub fn send_manage_finish(
        &self,
    ) {
        let res = self.try_send_manage_finish(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.manage_finish", &e);
        }
    }

    /// Since when the manage_dirty message is available.
    pub const MSG__MANAGE_DIRTY__SINCE: u32 = 1;

    /// ensure a manage sequence is started
    ///
    /// This request ensures a manage sequence is started and that a
    /// manage_start event is sent by the server. If this request is made during
    /// an ongoing manage sequence, a new manage sequence will be started as
    /// soon as the current one is completed.
    ///
    /// The client may want to use this request due to an internal state change
    /// that the compositor is not aware of (e.g. a dbus event) which should
    /// affect window management or rendering state.
    #[inline]
    pub fn try_send_manage_dirty(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_manager_v1#{}.manage_dirty()\n", id);
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

    /// ensure a manage sequence is started
    ///
    /// This request ensures a manage sequence is started and that a
    /// manage_start event is sent by the server. If this request is made during
    /// an ongoing manage sequence, a new manage sequence will be started as
    /// soon as the current one is completed.
    ///
    /// The client may want to use this request due to an internal state change
    /// that the compositor is not aware of (e.g. a dbus event) which should
    /// affect window management or rendering state.
    #[inline]
    pub fn send_manage_dirty(
        &self,
    ) {
        let res = self.try_send_manage_dirty(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.manage_dirty", &e);
        }
    }

    /// Since when the render_start message is available.
    pub const MSG__RENDER_START__SINCE: u32 = 1;

    /// start a render sequence
    ///
    /// This event indicates that the server has sent all river_node_v1.position
    /// and river_window_v1.dimensions events necessary.
    ///
    /// In response to this event, the client should make requests modifying
    /// rendering state as it chooses. Then, the client must make the
    /// render_finish request.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    pub fn try_send_render_start(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_manager_v1#{}.render_start()\n", client_id, id);
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

    /// start a render sequence
    ///
    /// This event indicates that the server has sent all river_node_v1.position
    /// and river_window_v1.dimensions events necessary.
    ///
    /// In response to this event, the client should make requests modifying
    /// rendering state as it chooses. Then, the client must make the
    /// render_finish request.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    pub fn send_render_start(
        &self,
    ) {
        let res = self.try_send_render_start(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.render_start", &e);
        }
    }

    /// Since when the render_finish message is available.
    pub const MSG__RENDER_FINISH__SINCE: u32 = 1;

    /// finish a render sequence
    ///
    /// This request indicates that the client has made all changes to rendering
    /// state it wishes to include in the current manage sequence and that the
    /// server should atomically apply and display these state changes to the
    /// user.
    ///
    /// After sending this request, it is a protocol error for the client to
    /// make further changes to rendering state until the next manage_start or
    /// render_start event is received, whichever comes first.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    pub fn try_send_render_finish(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_manager_v1#{}.render_finish()\n", id);
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

    /// finish a render sequence
    ///
    /// This request indicates that the client has made all changes to rendering
    /// state it wishes to include in the current manage sequence and that the
    /// server should atomically apply and display these state changes to the
    /// user.
    ///
    /// After sending this request, it is a protocol error for the client to
    /// make further changes to rendering state until the next manage_start or
    /// render_start event is received, whichever comes first.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    pub fn send_render_finish(
        &self,
    ) {
        let res = self.try_send_render_finish(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.render_finish", &e);
        }
    }

    /// Since when the session_locked message is available.
    pub const MSG__SESSION_LOCKED__SINCE: u32 = 1;

    /// the session has been locked
    ///
    /// This event indicates that the session has been locked.
    ///
    /// The window manager may wish to restrict which key bindings are available
    /// while locked or otherwise use this information.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_session_locked(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_manager_v1#{}.session_locked()\n", client_id, id);
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
            4,
        ]);
        Ok(())
    }

    /// the session has been locked
    ///
    /// This event indicates that the session has been locked.
    ///
    /// The window manager may wish to restrict which key bindings are available
    /// while locked or otherwise use this information.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_session_locked(
        &self,
    ) {
        let res = self.try_send_session_locked(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.session_locked", &e);
        }
    }

    /// Since when the session_unlocked message is available.
    pub const MSG__SESSION_UNLOCKED__SINCE: u32 = 1;

    /// the session has been unlocked
    ///
    /// This event indicates that the session has been unlocked.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn try_send_session_unlocked(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_manager_v1#{}.session_unlocked()\n", client_id, id);
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

    /// the session has been unlocked
    ///
    /// This event indicates that the session has been unlocked.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn send_session_unlocked(
        &self,
    ) {
        let res = self.try_send_session_unlocked(
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.session_unlocked", &e);
        }
    }

    /// Since when the window message is available.
    pub const MSG__WINDOW__SINCE: u32 = 1;

    /// new window
    ///
    /// A new window has been created.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_window(
        &self,
        id: &Rc<RiverWindowV1>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_manager_v1#{}.window(id: river_window_v1#{})\n", client_id, id, arg0);
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
            6,
            arg0_id,
        ]);
        Ok(())
    }

    /// new window
    ///
    /// A new window has been created.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_window(
        &self,
        id: &Rc<RiverWindowV1>,
    ) {
        let res = self.try_send_window(
            id,
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.window", &e);
        }
    }

    /// new window
    ///
    /// A new window has been created.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn new_try_send_window(
        &self,
    ) -> Result<Rc<RiverWindowV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_window(
            &id,
        )?;
        Ok(id)
    }

    /// new window
    ///
    /// A new window has been created.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn new_send_window(
        &self,
    ) -> Rc<RiverWindowV1> {
        let id = self.core.create_child();
        self.send_window(
            &id,
        );
        id
    }

    /// Since when the output message is available.
    pub const MSG__OUTPUT__SINCE: u32 = 1;

    /// new output
    ///
    /// A new logical output has been created, perhaps due to a new physical
    /// monitor being plugged in or perhaps due to a change in configuration.
    ///
    /// This event will be followed by river_output_v1.position and dimensions
    /// events as well as a manage_start event after all other new state has
    /// been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_output(
        &self,
        id: &Rc<RiverOutputV1>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_manager_v1#{}.output(id: river_output_v1#{})\n", client_id, id, arg0);
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

    /// new output
    ///
    /// A new logical output has been created, perhaps due to a new physical
    /// monitor being plugged in or perhaps due to a change in configuration.
    ///
    /// This event will be followed by river_output_v1.position and dimensions
    /// events as well as a manage_start event after all other new state has
    /// been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_output(
        &self,
        id: &Rc<RiverOutputV1>,
    ) {
        let res = self.try_send_output(
            id,
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.output", &e);
        }
    }

    /// new output
    ///
    /// A new logical output has been created, perhaps due to a new physical
    /// monitor being plugged in or perhaps due to a change in configuration.
    ///
    /// This event will be followed by river_output_v1.position and dimensions
    /// events as well as a manage_start event after all other new state has
    /// been sent by the server.
    #[inline]
    pub fn new_try_send_output(
        &self,
    ) -> Result<Rc<RiverOutputV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_output(
            &id,
        )?;
        Ok(id)
    }

    /// new output
    ///
    /// A new logical output has been created, perhaps due to a new physical
    /// monitor being plugged in or perhaps due to a change in configuration.
    ///
    /// This event will be followed by river_output_v1.position and dimensions
    /// events as well as a manage_start event after all other new state has
    /// been sent by the server.
    #[inline]
    pub fn new_send_output(
        &self,
    ) -> Rc<RiverOutputV1> {
        let id = self.core.create_child();
        self.send_output(
            &id,
        );
        id
    }

    /// Since when the seat message is available.
    pub const MSG__SEAT__SINCE: u32 = 1;

    /// new seat
    ///
    /// A new seat has been created.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_seat(
        &self,
        id: &Rc<RiverSeatV1>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= river_window_manager_v1#{}.seat(id: river_seat_v1#{})\n", client_id, id, arg0);
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
            8,
            arg0_id,
        ]);
        Ok(())
    }

    /// new seat
    ///
    /// A new seat has been created.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_seat(
        &self,
        id: &Rc<RiverSeatV1>,
    ) {
        let res = self.try_send_seat(
            id,
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.seat", &e);
        }
    }

    /// new seat
    ///
    /// A new seat has been created.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn new_try_send_seat(
        &self,
    ) -> Result<Rc<RiverSeatV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_seat(
            &id,
        )?;
        Ok(id)
    }

    /// new seat
    ///
    /// A new seat has been created.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    pub fn new_send_seat(
        &self,
    ) -> Rc<RiverSeatV1> {
        let id = self.core.create_child();
        self.send_seat(
            &id,
        );
        id
    }

    /// Since when the get_shell_surface message is available.
    pub const MSG__GET_SHELL_SURFACE__SINCE: u32 = 1;

    /// assign the river_shell_surface_v1 surface role
    ///
    /// Create a new shell surface for window manager UI and assign the
    /// river_shell_surface_v1 role to the surface.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn try_send_get_shell_surface(
        &self,
        id: &Rc<RiverShellSurfaceV1>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_window_manager_v1#{}.get_shell_surface(id: river_shell_surface_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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
            5,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// assign the river_shell_surface_v1 surface role
    ///
    /// Create a new shell surface for window manager UI and assign the
    /// river_shell_surface_v1 role to the surface.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_shell_surface(
        &self,
        id: &Rc<RiverShellSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_shell_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("river_window_manager_v1.get_shell_surface", &e);
        }
    }

    /// assign the river_shell_surface_v1 surface role
    ///
    /// Create a new shell surface for window manager UI and assign the
    /// river_shell_surface_v1 role to the surface.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_try_send_get_shell_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<RiverShellSurfaceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_shell_surface(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// assign the river_shell_surface_v1 surface role
    ///
    /// Create a new shell surface for window manager UI and assign the
    /// river_shell_surface_v1 role to the surface.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_send_get_shell_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<RiverShellSurfaceV1> {
        let id = self.core.create_child();
        self.send_get_shell_surface(
            &id,
            surface,
        );
        id
    }
}

/// A message handler for [`RiverWindowManagerV1`] proxies.
pub trait RiverWindowManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverWindowManagerV1>) {
        slf.core.delete_id();
    }

    /// window management unavailable
    ///
    /// This event indicates that window management is not available to the
    /// client, perhaps due to another window management client already running.
    /// The circumstances causing this event to be sent are compositor policy.
    ///
    /// If sent, this event is guaranteed to be the first and only event sent by
    /// the server.
    ///
    /// The server will send no further events on this object. The client should
    /// destroy this object and all objects created through this interface.
    #[inline]
    fn handle_unavailable(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_unavailable(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.unavailable", &e);
        }
    }

    /// stop sending events
    ///
    /// This request indicates that the client no longer wishes to receive
    /// events on this object.
    ///
    /// The Wayland protocol is asynchronous, which means the server may send
    /// further events until the stop request is processed. The client must wait
    /// for a river_window_manager_v1.finished event before destroying this
    /// object.
    #[inline]
    fn handle_stop(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_stop(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.stop", &e);
        }
    }

    /// the server has finished with the window manager
    ///
    /// This event indicates that the server will send no further events on this
    /// object. The client should destroy the object. See
    /// river_window_manager_v1.destroy for more information.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.finished", &e);
        }
    }

    /// destroy the river_window_manager_v1 object
    ///
    /// This request should be called after the finished event has been received
    /// to complete destruction of the object.
    ///
    /// If a client wishes to destroy this object it should send a
    /// river_window_manager_v1.stop request and wait for a
    /// river_window_manager_v1.finished event. Once the finished event is
    /// received it is safe to destroy this object and any other objects created
    /// through this interface.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.destroy", &e);
        }
    }

    /// start a manage sequence
    ///
    /// This event indicates that the server has sent events indicating all
    /// state changes since the last manage sequence.
    ///
    /// In response to this event, the client should make requests modifying
    /// window management state as it chooses. Then, the client must make the
    /// manage_finish request.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    fn handle_manage_start(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_manage_start(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.manage_start", &e);
        }
    }

    /// finish a manage sequence
    ///
    /// This request indicates that the client has made all changes to window
    /// management state it wishes to include in the current manage sequence and
    /// that the server should atomically send these state changes to the
    /// windows and continue with the manage sequence.
    ///
    /// After sending this request, it is a protocol error for the client to
    /// make further changes to window management state until the next
    /// manage_start event is received.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    fn handle_manage_finish(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_manage_finish(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.manage_finish", &e);
        }
    }

    /// ensure a manage sequence is started
    ///
    /// This request ensures a manage sequence is started and that a
    /// manage_start event is sent by the server. If this request is made during
    /// an ongoing manage sequence, a new manage sequence will be started as
    /// soon as the current one is completed.
    ///
    /// The client may want to use this request due to an internal state change
    /// that the compositor is not aware of (e.g. a dbus event) which should
    /// affect window management or rendering state.
    #[inline]
    fn handle_manage_dirty(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_manage_dirty(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.manage_dirty", &e);
        }
    }

    /// start a render sequence
    ///
    /// This event indicates that the server has sent all river_node_v1.position
    /// and river_window_v1.dimensions events necessary.
    ///
    /// In response to this event, the client should make requests modifying
    /// rendering state as it chooses. Then, the client must make the
    /// render_finish request.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    fn handle_render_start(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_render_start(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.render_start", &e);
        }
    }

    /// finish a render sequence
    ///
    /// This request indicates that the client has made all changes to rendering
    /// state it wishes to include in the current manage sequence and that the
    /// server should atomically apply and display these state changes to the
    /// user.
    ///
    /// After sending this request, it is a protocol error for the client to
    /// make further changes to rendering state until the next manage_start or
    /// render_start event is received, whichever comes first.
    ///
    /// See the description of the river_window_manager_v1 interface for a
    /// complete overview of the manage/render sequence loop.
    #[inline]
    fn handle_render_finish(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_render_finish(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.render_finish", &e);
        }
    }

    /// the session has been locked
    ///
    /// This event indicates that the session has been locked.
    ///
    /// The window manager may wish to restrict which key bindings are available
    /// while locked or otherwise use this information.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_session_locked(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_session_locked(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.session_locked", &e);
        }
    }

    /// the session has been unlocked
    ///
    /// This event indicates that the session has been unlocked.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    #[inline]
    fn handle_session_unlocked(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_session_unlocked(
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.session_unlocked", &e);
        }
    }

    /// new window
    ///
    /// A new window has been created.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_window(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
        id: &Rc<RiverWindowV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_window(
            id,
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.window", &e);
        }
    }

    /// new output
    ///
    /// A new logical output has been created, perhaps due to a new physical
    /// monitor being plugged in or perhaps due to a change in configuration.
    ///
    /// This event will be followed by river_output_v1.position and dimensions
    /// events as well as a manage_start event after all other new state has
    /// been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_output(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
        id: &Rc<RiverOutputV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_output(
            id,
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.output", &e);
        }
    }

    /// new seat
    ///
    /// A new seat has been created.
    ///
    /// This event will be followed by a manage_start event after all other new
    /// state has been sent by the server.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_seat(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
        id: &Rc<RiverSeatV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_seat(
            id,
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.seat", &e);
        }
    }

    /// assign the river_shell_surface_v1 surface role
    ///
    /// Create a new shell surface for window manager UI and assign the
    /// river_shell_surface_v1 role to the surface.
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
    fn handle_get_shell_surface(
        &mut self,
        slf: &Rc<RiverWindowManagerV1>,
        id: &Rc<RiverShellSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_shell_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("river_window_manager_v1.get_shell_surface", &e);
        }
    }
}

impl ObjectPrivate for RiverWindowManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverWindowManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_manager_v1#{}.stop()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_manager_v1#{}.destroy()\n", client_id, id);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_manager_v1#{}.manage_finish()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_manage_finish(&self);
                } else {
                    DefaultHandler.handle_manage_finish(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_manager_v1#{}.manage_dirty()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_manage_dirty(&self);
                } else {
                    DefaultHandler.handle_manage_dirty(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_manager_v1#{}.render_finish()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_render_finish(&self);
                } else {
                    DefaultHandler.handle_render_finish(&self);
                }
            }
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_window_manager_v1#{}.get_shell_surface(id: river_shell_surface_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = RiverShellSurfaceV1::new(&self.core.state, self.core.version);
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
                    (**handler).handle_get_shell_surface(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_shell_surface(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_manager_v1#{}.unavailable()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unavailable(&self);
                } else {
                    DefaultHandler.handle_unavailable(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_manager_v1#{}.finished()\n", id);
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
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_manager_v1#{}.manage_start()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_manage_start(&self);
                } else {
                    DefaultHandler.handle_manage_start(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_manager_v1#{}.render_start()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_render_start(&self);
                } else {
                    DefaultHandler.handle_render_start(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_manager_v1#{}.session_locked()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_session_locked(&self);
                } else {
                    DefaultHandler.handle_session_locked(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_manager_v1#{}.session_unlocked()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_session_unlocked(&self);
                } else {
                    DefaultHandler.handle_session_unlocked(&self);
                }
            }
            6 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_manager_v1#{}.window(id: river_window_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = RiverWindowV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_window(&self, arg0);
                } else {
                    DefaultHandler.handle_window(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_manager_v1#{}.output(id: river_output_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = RiverOutputV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_output(&self, arg0);
                } else {
                    DefaultHandler.handle_output(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> river_window_manager_v1#{}.seat(id: river_seat_v1#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = RiverSeatV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_seat(&self, arg0);
                } else {
                    DefaultHandler.handle_seat(&self, arg0);
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
            2 => "manage_finish",
            3 => "manage_dirty",
            4 => "render_finish",
            5 => "get_shell_surface",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "unavailable",
            1 => "finished",
            2 => "manage_start",
            3 => "render_start",
            4 => "session_locked",
            5 => "session_unlocked",
            6 => "window",
            7 => "output",
            8 => "seat",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for RiverWindowManagerV1 {
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

impl RiverWindowManagerV1 {
    /// Since when the error.sequence_order enum variant is available.
    pub const ENM__ERROR_SEQUENCE_ORDER__SINCE: u32 = 1;
    /// Since when the error.role enum variant is available.
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.unresponsive enum variant is available.
    pub const ENM__ERROR_UNRESPONSIVE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverWindowManagerV1Error(pub u32);

impl RiverWindowManagerV1Error {
    /// request violates manage/render sequence ordering
    pub const SEQUENCE_ORDER: Self = Self(0);

    /// given wl_surface already has a role
    pub const ROLE: Self = Self(1);

    /// window manager unresponsive
    pub const UNRESPONSIVE: Self = Self(2);
}

impl Debug for RiverWindowManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SEQUENCE_ORDER => "SEQUENCE_ORDER",
            Self::ROLE => "ROLE",
            Self::UNRESPONSIVE => "UNRESPONSIVE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
