//! corner radius toplevel
//!
//! The corner-radius object provides a way to specify a corner-radius
//! for it's associated toplevel.
//!
//! If the xdg_toplevel associated with the cosmic_corner_radius_toplevel_v1
//! object has been destroyed, this object becomes inert. Any further requests
//! other than destroy will raise the toplevel_destroyed protocol error.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A cosmic_corner_radius_toplevel_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct CosmicCornerRadiusToplevelV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn CosmicCornerRadiusToplevelV1Handler>,
}

struct DefaultHandler;

impl CosmicCornerRadiusToplevelV1Handler for DefaultHandler { }

impl ConcreteObject for CosmicCornerRadiusToplevelV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::CosmicCornerRadiusToplevelV1;
    const INTERFACE_NAME: &str = "cosmic_corner_radius_toplevel_v1";
}

impl CosmicCornerRadiusToplevelV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl CosmicCornerRadiusToplevelV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn CosmicCornerRadiusToplevelV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for CosmicCornerRadiusToplevelV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CosmicCornerRadiusToplevelV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl CosmicCornerRadiusToplevelV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// Destroy the corner-radius object
    ///
    /// Informs the server that the client will no longer be using this protocol
    /// object. The corner radius will be unset on the next commit.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_corner_radius_toplevel_v1#{}.destroy()\n", id);
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

    /// Destroy the corner-radius object
    ///
    /// Informs the server that the client will no longer be using this protocol
    /// object. The corner radius will be unset on the next commit.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("cosmic_corner_radius_toplevel_v1.destroy", &e);
        }
    }

    /// Since when the set_radius message is available.
    pub const MSG__SET_RADIUS__SINCE: u32 = 1;

    /// Set corner radius
    ///
    /// This request sets the hinted corner radius values for rectangular windows.
    ///
    /// The corner radius hint is double-buffered state and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The value is given in logical space relative to the window geometry of the
    /// associated xdg_toplevel. If any value exceeds a quarter of either dimension
    /// of the window geometry the radius_too_large protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `top_left`: top-left corner radius
    /// - `top_right`: top-right corner radius
    /// - `bottom_right`: bottom-right corner radius
    /// - `bottom_left`: bottom-left corner radius
    #[inline]
    pub fn try_send_set_radius(
        &self,
        top_left: u32,
        top_right: u32,
        bottom_right: u32,
        bottom_left: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_corner_radius_toplevel_v1#{}.set_radius(top_left: {}, top_right: {}, bottom_right: {}, bottom_left: {})\n", id, arg0, arg1, arg2, arg3);
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
            1,
            arg0,
            arg1,
            arg2,
            arg3,
        ]);
        Ok(())
    }

    /// Set corner radius
    ///
    /// This request sets the hinted corner radius values for rectangular windows.
    ///
    /// The corner radius hint is double-buffered state and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The value is given in logical space relative to the window geometry of the
    /// associated xdg_toplevel. If any value exceeds a quarter of either dimension
    /// of the window geometry the radius_too_large protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `top_left`: top-left corner radius
    /// - `top_right`: top-right corner radius
    /// - `bottom_right`: bottom-right corner radius
    /// - `bottom_left`: bottom-left corner radius
    #[inline]
    pub fn send_set_radius(
        &self,
        top_left: u32,
        top_right: u32,
        bottom_right: u32,
        bottom_left: u32,
    ) {
        let res = self.try_send_set_radius(
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        );
        if let Err(e) = res {
            log_send("cosmic_corner_radius_toplevel_v1.set_radius", &e);
        }
    }

    /// Since when the unset_radius message is available.
    pub const MSG__UNSET_RADIUS__SINCE: u32 = 1;

    /// Unset corner radius
    ///
    /// Unsets any previously hinted corner radius values without invalidating the object for later use.
    /// Can be used by clients that possibly have temporary irregular shapes.
    #[inline]
    pub fn try_send_unset_radius(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_corner_radius_toplevel_v1#{}.unset_radius()\n", id);
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

    /// Unset corner radius
    ///
    /// Unsets any previously hinted corner radius values without invalidating the object for later use.
    /// Can be used by clients that possibly have temporary irregular shapes.
    #[inline]
    pub fn send_unset_radius(
        &self,
    ) {
        let res = self.try_send_unset_radius(
        );
        if let Err(e) = res {
            log_send("cosmic_corner_radius_toplevel_v1.unset_radius", &e);
        }
    }
}

/// A message handler for [`CosmicCornerRadiusToplevelV1`] proxies.
pub trait CosmicCornerRadiusToplevelV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<CosmicCornerRadiusToplevelV1>) {
        slf.core.delete_id();
    }

    /// Destroy the corner-radius object
    ///
    /// Informs the server that the client will no longer be using this protocol
    /// object. The corner radius will be unset on the next commit.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<CosmicCornerRadiusToplevelV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("cosmic_corner_radius_toplevel_v1.destroy", &e);
        }
    }

    /// Set corner radius
    ///
    /// This request sets the hinted corner radius values for rectangular windows.
    ///
    /// The corner radius hint is double-buffered state and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The value is given in logical space relative to the window geometry of the
    /// associated xdg_toplevel. If any value exceeds a quarter of either dimension
    /// of the window geometry the radius_too_large protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `top_left`: top-left corner radius
    /// - `top_right`: top-right corner radius
    /// - `bottom_right`: bottom-right corner radius
    /// - `bottom_left`: bottom-left corner radius
    #[inline]
    fn handle_set_radius(
        &mut self,
        slf: &Rc<CosmicCornerRadiusToplevelV1>,
        top_left: u32,
        top_right: u32,
        bottom_right: u32,
        bottom_left: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_radius(
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        );
        if let Err(e) = res {
            log_forward("cosmic_corner_radius_toplevel_v1.set_radius", &e);
        }
    }

    /// Unset corner radius
    ///
    /// Unsets any previously hinted corner radius values without invalidating the object for later use.
    /// Can be used by clients that possibly have temporary irregular shapes.
    #[inline]
    fn handle_unset_radius(
        &mut self,
        slf: &Rc<CosmicCornerRadiusToplevelV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unset_radius(
        );
        if let Err(e) = res {
            log_forward("cosmic_corner_radius_toplevel_v1.unset_radius", &e);
        }
    }
}

impl ObjectPrivate for CosmicCornerRadiusToplevelV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::CosmicCornerRadiusToplevelV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_corner_radius_toplevel_v1#{}.destroy()\n", client_id, id);
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
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_corner_radius_toplevel_v1#{}.set_radius(top_left: {}, top_right: {}, bottom_right: {}, bottom_left: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_radius(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_radius(&self, arg0, arg1, arg2, arg3);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_corner_radius_toplevel_v1#{}.unset_radius()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unset_radius(&self);
                } else {
                    DefaultHandler.handle_unset_radius(&self);
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
            1 => "set_radius",
            2 => "unset_radius",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for CosmicCornerRadiusToplevelV1 {
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

impl CosmicCornerRadiusToplevelV1 {
    /// Since when the error.toplevel_destroyed enum variant is available.
    pub const ENM__ERROR_TOPLEVEL_DESTROYED__SINCE: u32 = 1;
    /// Since when the error.radius_too_large enum variant is available.
    pub const ENM__ERROR_RADIUS_TOO_LARGE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CosmicCornerRadiusToplevelV1Error(pub u32);

impl CosmicCornerRadiusToplevelV1Error {
    /// the associated toplevel object has been already destroyed
    pub const TOPLEVEL_DESTROYED: Self = Self(0);

    /// the associated toplevel's window geometry isn't large enough for the requested radius
    pub const RADIUS_TOO_LARGE: Self = Self(1);
}

impl Debug for CosmicCornerRadiusToplevelV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TOPLEVEL_DESTROYED => "TOPLEVEL_DESTROYED",
            Self::RADIUS_TOO_LARGE => "RADIUS_TOO_LARGE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
