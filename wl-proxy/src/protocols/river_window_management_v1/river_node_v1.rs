//! a node in the render list
//!
//! The render list is a list of nodes that determines the rendering order of
//! the compositor. Nodes may correspond to windows or shell surfaces. The
//! relative ordering of nodes may be changed with the place_above and
//! place_below requests, changing the rendering order.
//!
//! The initial position of a node in the render list is undefined, the window
//! manager client must use the place_above or place_below request to
//! guarantee a specific rendering order.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_node_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverNodeV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverNodeV1Handler>,
}

struct DefaultHandler;

impl RiverNodeV1Handler for DefaultHandler { }

impl ConcreteObject for RiverNodeV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverNodeV1;
    const INTERFACE_NAME: &str = "river_node_v1";
}

impl RiverNodeV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverNodeV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverNodeV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverNodeV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverNodeV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverNodeV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the decoration object
    ///
    /// This request indicates that the client will no longer use the node
    /// object and that it may be safely destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_node_v1#{}.destroy()\n", id);
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

    /// destroy the decoration object
    ///
    /// This request indicates that the client will no longer use the node
    /// object and that it may be safely destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_node_v1.destroy", &e);
        }
    }

    /// Since when the set_position message is available.
    pub const MSG__SET_POSITION__SINCE: u32 = 1;

    /// set absolute position of the node
    ///
    /// Set the absolute position of the node in the compositor's logical
    /// coordinate space. The x and y coordinates may be positive or negative.
    ///
    /// Note that the position of a river_window_v1 refers to the position of
    /// the window content and is unaffected by the presence of borders or
    /// decoration surfaces.
    ///
    /// If this request is never sent, the position of the node is undefined by
    /// this protocol and left up to the compositor.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn try_send_set_position(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_node_v1#{}.set_position(x: {}, y: {})\n", id, arg0, arg1);
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
            1,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// set absolute position of the node
    ///
    /// Set the absolute position of the node in the compositor's logical
    /// coordinate space. The x and y coordinates may be positive or negative.
    ///
    /// Note that the position of a river_window_v1 refers to the position of
    /// the window content and is unaffected by the presence of borders or
    /// decoration surfaces.
    ///
    /// If this request is never sent, the position of the node is undefined by
    /// this protocol and left up to the compositor.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn send_set_position(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_set_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("river_node_v1.set_position", &e);
        }
    }

    /// Since when the place_top message is available.
    pub const MSG__PLACE_TOP__SINCE: u32 = 1;

    /// place node above all other nodes
    ///
    /// This request places the node above all other nodes in the compositor's
    /// render list.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_place_top(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_node_v1#{}.place_top()\n", id);
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

    /// place node above all other nodes
    ///
    /// This request places the node above all other nodes in the compositor's
    /// render list.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_place_top(
        &self,
    ) {
        let res = self.try_send_place_top(
        );
        if let Err(e) = res {
            log_send("river_node_v1.place_top", &e);
        }
    }

    /// Since when the place_bottom message is available.
    pub const MSG__PLACE_BOTTOM__SINCE: u32 = 1;

    /// place node below all other nodes
    ///
    /// This request places the node below all other nodes in the compositor's
    /// render list.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_place_bottom(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_node_v1#{}.place_bottom()\n", id);
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

    /// place node below all other nodes
    ///
    /// This request places the node below all other nodes in the compositor's
    /// render list.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_place_bottom(
        &self,
    ) {
        let res = self.try_send_place_bottom(
        );
        if let Err(e) = res {
            log_send("river_node_v1.place_bottom", &e);
        }
    }

    /// Since when the place_above message is available.
    pub const MSG__PLACE_ABOVE__SINCE: u32 = 1;

    /// place node above another node
    ///
    /// This request places the node directly above another node in the
    /// compositor's render list.
    ///
    /// Attempting to place a node above itself has no effect.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `other`:
    #[inline]
    pub fn try_send_place_above(
        &self,
        other: &Rc<RiverNodeV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            other,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("other"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_node_v1#{}.place_above(other: river_node_v1#{})\n", id, arg0);
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

    /// place node above another node
    ///
    /// This request places the node directly above another node in the
    /// compositor's render list.
    ///
    /// Attempting to place a node above itself has no effect.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `other`:
    #[inline]
    pub fn send_place_above(
        &self,
        other: &Rc<RiverNodeV1>,
    ) {
        let res = self.try_send_place_above(
            other,
        );
        if let Err(e) = res {
            log_send("river_node_v1.place_above", &e);
        }
    }

    /// Since when the place_below message is available.
    pub const MSG__PLACE_BELOW__SINCE: u32 = 1;

    /// place node below another node
    ///
    /// This request places the node directly below another node in the
    /// compositor's render list.
    ///
    /// Attempting to place a node below itself has no effect.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `other`:
    #[inline]
    pub fn try_send_place_below(
        &self,
        other: &Rc<RiverNodeV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            other,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("other"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_node_v1#{}.place_below(other: river_node_v1#{})\n", id, arg0);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// place node below another node
    ///
    /// This request places the node directly below another node in the
    /// compositor's render list.
    ///
    /// Attempting to place a node below itself has no effect.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `other`:
    #[inline]
    pub fn send_place_below(
        &self,
        other: &Rc<RiverNodeV1>,
    ) {
        let res = self.try_send_place_below(
            other,
        );
        if let Err(e) = res {
            log_send("river_node_v1.place_below", &e);
        }
    }
}

/// A message handler for [`RiverNodeV1`] proxies.
pub trait RiverNodeV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverNodeV1>) {
        slf.core.delete_id();
    }

    /// destroy the decoration object
    ///
    /// This request indicates that the client will no longer use the node
    /// object and that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverNodeV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_node_v1.destroy", &e);
        }
    }

    /// set absolute position of the node
    ///
    /// Set the absolute position of the node in the compositor's logical
    /// coordinate space. The x and y coordinates may be positive or negative.
    ///
    /// Note that the position of a river_window_v1 refers to the position of
    /// the window content and is unaffected by the presence of borders or
    /// decoration surfaces.
    ///
    /// If this request is never sent, the position of the node is undefined by
    /// this protocol and left up to the compositor.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    fn handle_set_position(
        &mut self,
        slf: &Rc<RiverNodeV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_position(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("river_node_v1.set_position", &e);
        }
    }

    /// place node above all other nodes
    ///
    /// This request places the node above all other nodes in the compositor's
    /// render list.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_place_top(
        &mut self,
        slf: &Rc<RiverNodeV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_place_top(
        );
        if let Err(e) = res {
            log_forward("river_node_v1.place_top", &e);
        }
    }

    /// place node below all other nodes
    ///
    /// This request places the node below all other nodes in the compositor's
    /// render list.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_place_bottom(
        &mut self,
        slf: &Rc<RiverNodeV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_place_bottom(
        );
        if let Err(e) = res {
            log_forward("river_node_v1.place_bottom", &e);
        }
    }

    /// place node above another node
    ///
    /// This request places the node directly above another node in the
    /// compositor's render list.
    ///
    /// Attempting to place a node above itself has no effect.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `other`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_place_above(
        &mut self,
        slf: &Rc<RiverNodeV1>,
        other: &Rc<RiverNodeV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_place_above(
            other,
        );
        if let Err(e) = res {
            log_forward("river_node_v1.place_above", &e);
        }
    }

    /// place node below another node
    ///
    /// This request places the node directly below another node in the
    /// compositor's render list.
    ///
    /// Attempting to place a node below itself has no effect.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `other`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_place_below(
        &mut self,
        slf: &Rc<RiverNodeV1>,
        other: &Rc<RiverNodeV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_place_below(
            other,
        );
        if let Err(e) = res {
            log_forward("river_node_v1.place_below", &e);
        }
    }
}

impl ObjectPrivate for RiverNodeV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverNodeV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_node_v1#{}.destroy()\n", client_id, id);
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
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_node_v1#{}.set_position(x: {}, y: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_position(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_position(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_node_v1#{}.place_top()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_place_top(&self);
                } else {
                    DefaultHandler.handle_place_top(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_node_v1#{}.place_bottom()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_place_bottom(&self);
                } else {
                    DefaultHandler.handle_place_bottom(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_node_v1#{}.place_above(other: river_node_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverNodeV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("other", o.core().interface, ObjectInterface::RiverNodeV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_place_above(&self, arg0);
                } else {
                    DefaultHandler.handle_place_above(&self, arg0);
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_node_v1#{}.place_below(other: river_node_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<RiverNodeV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("other", o.core().interface, ObjectInterface::RiverNodeV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_place_below(&self, arg0);
                } else {
                    DefaultHandler.handle_place_below(&self, arg0);
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
            1 => "set_position",
            2 => "place_top",
            3 => "place_bottom",
            4 => "place_above",
            5 => "place_below",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for RiverNodeV1 {
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

