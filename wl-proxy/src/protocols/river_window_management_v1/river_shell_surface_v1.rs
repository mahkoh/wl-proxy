//! a surface for window manager UI
//!
//! The window manager might use a shell surface to display a status bar,
//! background image, desktop notifications, launcher, desktop menu, or
//! whatever else it wants.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_shell_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverShellSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverShellSurfaceV1Handler>,
}

struct DefaultHandler;

impl RiverShellSurfaceV1Handler for DefaultHandler { }

impl ConcreteObject for RiverShellSurfaceV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverShellSurfaceV1;
    const INTERFACE_NAME: &str = "river_shell_surface_v1";
}

impl RiverShellSurfaceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverShellSurfaceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverShellSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverShellSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverShellSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverShellSurfaceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the shell surface object
    ///
    /// This request indicates that the client will no longer use the shell
    /// surface object and that it may be safely destroyed.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_shell_surface_v1#{}.destroy()\n", id);
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

    /// destroy the shell surface object
    ///
    /// This request indicates that the client will no longer use the shell
    /// surface object and that it may be safely destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_shell_surface_v1.destroy", &e);
        }
    }

    /// Since when the get_node message is available.
    pub const MSG__GET_NODE__SINCE: u32 = 1;

    /// get the shell surface's render list node
    ///
    /// Get the node in the render list corresponding to the shell surface.
    ///
    /// It is a protocol error to make this request more than once for a single
    /// shell surface.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_shell_surface_v1#{}.get_node(id: river_node_v1#{})\n", id, arg0);
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

    /// get the shell surface's render list node
    ///
    /// Get the node in the render list corresponding to the shell surface.
    ///
    /// It is a protocol error to make this request more than once for a single
    /// shell surface.
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
            log_send("river_shell_surface_v1.get_node", &e);
        }
    }

    /// get the shell surface's render list node
    ///
    /// Get the node in the render list corresponding to the shell surface.
    ///
    /// It is a protocol error to make this request more than once for a single
    /// shell surface.
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

    /// get the shell surface's render list node
    ///
    /// Get the node in the render list corresponding to the shell surface.
    ///
    /// It is a protocol error to make this request more than once for a single
    /// shell surface.
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

    /// Since when the sync_next_commit message is available.
    pub const MSG__SYNC_NEXT_COMMIT__SINCE: u32 = 1;

    /// sync next surface commit to window manager commit
    ///
    /// Synchronize application of the next wl_surface.commit request on the
    /// shell surface with rest of the rendering state atomically applied with
    /// the next river_window_manager_v1.render_finish request.
    ///
    /// The client must make a wl_surface.commit request on the shell surface
    /// after this request and before the render_finish request, failure to do
    /// so is a protocol error.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn try_send_sync_next_commit(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_shell_surface_v1#{}.sync_next_commit()\n", id);
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

    /// sync next surface commit to window manager commit
    ///
    /// Synchronize application of the next wl_surface.commit request on the
    /// shell surface with rest of the rendering state atomically applied with
    /// the next river_window_manager_v1.render_finish request.
    ///
    /// The client must make a wl_surface.commit request on the shell surface
    /// after this request and before the render_finish request, failure to do
    /// so is a protocol error.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    pub fn send_sync_next_commit(
        &self,
    ) {
        let res = self.try_send_sync_next_commit(
        );
        if let Err(e) = res {
            log_send("river_shell_surface_v1.sync_next_commit", &e);
        }
    }
}

/// A message handler for [`RiverShellSurfaceV1`] proxies.
pub trait RiverShellSurfaceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverShellSurfaceV1>) {
        slf.core.delete_id();
    }

    /// destroy the shell surface object
    ///
    /// This request indicates that the client will no longer use the shell
    /// surface object and that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverShellSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_shell_surface_v1.destroy", &e);
        }
    }

    /// get the shell surface's render list node
    ///
    /// Get the node in the render list corresponding to the shell surface.
    ///
    /// It is a protocol error to make this request more than once for a single
    /// shell surface.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_get_node(
        &mut self,
        slf: &Rc<RiverShellSurfaceV1>,
        id: &Rc<RiverNodeV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_node(
            id,
        );
        if let Err(e) = res {
            log_forward("river_shell_surface_v1.get_node", &e);
        }
    }

    /// sync next surface commit to window manager commit
    ///
    /// Synchronize application of the next wl_surface.commit request on the
    /// shell surface with rest of the rendering state atomically applied with
    /// the next river_window_manager_v1.render_finish request.
    ///
    /// The client must make a wl_surface.commit request on the shell surface
    /// after this request and before the render_finish request, failure to do
    /// so is a protocol error.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_sync_next_commit(
        &mut self,
        slf: &Rc<RiverShellSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_sync_next_commit(
        );
        if let Err(e) = res {
            log_forward("river_shell_surface_v1.sync_next_commit", &e);
        }
    }
}

impl ObjectPrivate for RiverShellSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverShellSurfaceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_shell_surface_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_shell_surface_v1#{}.get_node(id: river_node_v1#{})\n", client_id, id, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_shell_surface_v1#{}.sync_next_commit()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_sync_next_commit(&self);
                } else {
                    DefaultHandler.handle_sync_next_commit(&self);
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
            1 => "get_node",
            2 => "sync_next_commit",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for RiverShellSurfaceV1 {
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

impl RiverShellSurfaceV1 {
    /// Since when the error.node_exists enum variant is available.
    pub const ENM__ERROR_NODE_EXISTS__SINCE: u32 = 1;
    /// Since when the error.no_commit enum variant is available.
    pub const ENM__ERROR_NO_COMMIT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverShellSurfaceV1Error(pub u32);

impl RiverShellSurfaceV1Error {
    /// shell surface already has a node object
    pub const NODE_EXISTS: Self = Self(0);

    /// failed to commit the surface before the window manager commit
    pub const NO_COMMIT: Self = Self(1);
}

impl Debug for RiverShellSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NODE_EXISTS => "NODE_EXISTS",
            Self::NO_COMMIT => "NO_COMMIT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
