//! a window decoration
//!
//! The rendering order of windows with decorations is follows:
//!
//! 1. Decorations created with get_decoration_below at the bottom
//! 2. Window content
//! 3. Borders configured with river_window_v1.set_borders
//! 4. Decorations created with get_decoration_above at the top
//!
//! The relative ordering of decoration surfaces above/below a window is
//! undefined by this protocol and left up to the compositor.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A river_decoration_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct RiverDecorationV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn RiverDecorationV1Handler>,
}

struct DefaultHandler;

impl RiverDecorationV1Handler for DefaultHandler { }

impl ConcreteObject for RiverDecorationV1 {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::RiverDecorationV1;
    const INTERFACE_NAME: &str = "river_decoration_v1";
}

impl RiverDecorationV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl RiverDecorationV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn RiverDecorationV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for RiverDecorationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiverDecorationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl RiverDecorationV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the decoration object
    ///
    /// This request indicates that the client will no longer use the decoration
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_decoration_v1#{}.destroy()\n", id);
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
    /// This request indicates that the client will no longer use the decoration
    /// object and that it may be safely destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("river_decoration_v1.destroy", &e);
        }
    }

    /// Since when the set_offset message is available.
    pub const MSG__SET_OFFSET__SINCE: u32 = 1;

    /// set offset from the window's top left corner
    ///
    /// This request sets the offset of the decoration surface from the top left
    /// corner of the window.
    ///
    /// If this request is never sent, the x and y offsets are undefined by this
    /// protocol and left up to the compositor.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn try_send_set_offset(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_decoration_v1#{}.set_offset(x: {}, y: {})\n", id, arg0, arg1);
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

    /// set offset from the window's top left corner
    ///
    /// This request sets the offset of the decoration surface from the top left
    /// corner of the window.
    ///
    /// If this request is never sent, the x and y offsets are undefined by this
    /// protocol and left up to the compositor.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    pub fn send_set_offset(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_set_offset(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("river_decoration_v1.set_offset", &e);
        }
    }

    /// Since when the sync_next_commit message is available.
    pub const MSG__SYNC_NEXT_COMMIT__SINCE: u32 = 1;

    /// sync next commit with other rendering state
    ///
    /// Synchronize application of the next wl_surface.commit request on the
    /// decoration surface with rest of the state atomically applied with the
    /// next river_window_manager_v1.render_finish request.
    ///
    /// The client must make a wl_surface.commit request on the decoration
    /// surface after this request and before the render_finish request, failure
    /// to do so is a protocol error.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= river_decoration_v1#{}.sync_next_commit()\n", id);
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

    /// sync next commit with other rendering state
    ///
    /// Synchronize application of the next wl_surface.commit request on the
    /// decoration surface with rest of the state atomically applied with the
    /// next river_window_manager_v1.render_finish request.
    ///
    /// The client must make a wl_surface.commit request on the decoration
    /// surface after this request and before the render_finish request, failure
    /// to do so is a protocol error.
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
            log_send("river_decoration_v1.sync_next_commit", &e);
        }
    }
}

/// A message handler for [`RiverDecorationV1`] proxies.
pub trait RiverDecorationV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<RiverDecorationV1>) {
        slf.core.delete_id();
    }

    /// destroy the decoration object
    ///
    /// This request indicates that the client will no longer use the decoration
    /// object and that it may be safely destroyed.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<RiverDecorationV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("river_decoration_v1.destroy", &e);
        }
    }

    /// set offset from the window's top left corner
    ///
    /// This request sets the offset of the decoration surface from the top left
    /// corner of the window.
    ///
    /// If this request is never sent, the x and y offsets are undefined by this
    /// protocol and left up to the compositor.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    #[inline]
    fn handle_set_offset(
        &mut self,
        slf: &Rc<RiverDecorationV1>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_offset(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("river_decoration_v1.set_offset", &e);
        }
    }

    /// sync next commit with other rendering state
    ///
    /// Synchronize application of the next wl_surface.commit request on the
    /// decoration surface with rest of the state atomically applied with the
    /// next river_window_manager_v1.render_finish request.
    ///
    /// The client must make a wl_surface.commit request on the decoration
    /// surface after this request and before the render_finish request, failure
    /// to do so is a protocol error.
    ///
    /// This request modifies rendering state and may only be made as part of a
    /// render sequence, see the river_window_manager_v1 description.
    #[inline]
    fn handle_sync_next_commit(
        &mut self,
        slf: &Rc<RiverDecorationV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_sync_next_commit(
        );
        if let Err(e) = res {
            log_forward("river_decoration_v1.sync_next_commit", &e);
        }
    }
}

impl ObjectPrivate for RiverDecorationV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::RiverDecorationV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_decoration_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_decoration_v1#{}.set_offset(x: {}, y: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_offset(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_offset(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> river_decoration_v1#{}.sync_next_commit()\n", client_id, id);
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
            1 => "set_offset",
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

impl Object for RiverDecorationV1 {
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

impl RiverDecorationV1 {
    /// Since when the error.no_commit enum variant is available.
    pub const ENM__ERROR_NO_COMMIT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RiverDecorationV1Error(pub u32);

impl RiverDecorationV1Error {
    /// failed to commit the surface before the window manager commit
    pub const NO_COMMIT: Self = Self(0);
}

impl Debug for RiverDecorationV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NO_COMMIT => "NO_COMMIT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
