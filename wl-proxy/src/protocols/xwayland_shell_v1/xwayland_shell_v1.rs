//! context object for Xwayland shell
//!
//! xwayland_shell_v1 is a singleton global object that
//! provides the ability to create a xwayland_surface_v1 object
//! for a given wl_surface.
//!
//! This interface is intended to be bound by the Xwayland server.
//!
//! A compositor must not allow clients other than Xwayland to
//! bind to this interface. A compositor should hide this global
//! from other clients' wl_registry.
//! A client the compositor does not consider to be an Xwayland
//! server attempting to bind this interface will result in
//! an implementation-defined error.
//!
//! An Xwayland server that has bound this interface must not
//! set the `WL_SURFACE_ID` atom on a window.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xwayland_shell_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XwaylandShellV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XwaylandShellV1Handler>,
}

struct DefaultHandler;

impl XwaylandShellV1Handler for DefaultHandler { }

impl XwaylandShellV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::XwaylandShellV1;
    pub const INTERFACE_NAME: &str = "xwayland_shell_v1";
}

impl XwaylandShellV1 {
    pub fn set_handler(&self, handler: impl XwaylandShellV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn XwaylandShellV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XwaylandShellV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XwaylandShellV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XwaylandShellV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the Xwayland shell object
    ///
    /// Destroy the xwayland_shell_v1 object.
    ///
    /// The child objects created via this interface are unaffected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xwayland_shell_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
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
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the get_xwayland_surface message is available.
    pub const MSG__GET_XWAYLAND_SURFACE__SINCE: u32 = 1;

    /// assign the xwayland_surface surface role
    ///
    /// Create an xwayland_surface_v1 interface for a given wl_surface
    /// object and gives it the xwayland_surface role.
    ///
    /// It is illegal to create an xwayland_surface_v1 for a wl_surface
    /// which already has an assigned role and this will result in the
    /// `role` protocol error.
    ///
    /// See the documentation of xwayland_surface_v1 for more details
    /// about what an xwayland_surface_v1 is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_xwayland_surface(
        &self,
        id: &Rc<XwaylandSurfaceV1>,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xwayland_shell_v1#{}.get_xwayland_surface(id: xwayland_surface_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [XwaylandShellV1] proxies.
pub trait XwaylandShellV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XwaylandShellV1>) {
        let _ = slf.core.delete_id();
    }

    /// destroy the Xwayland shell object
    ///
    /// Destroy the xwayland_shell_v1 object.
    ///
    /// The child objects created via this interface are unaffected.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<XwaylandShellV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xwayland_shell_v1.destroy message: {}", Report::new(e));
        }
    }

    /// assign the xwayland_surface surface role
    ///
    /// Create an xwayland_surface_v1 interface for a given wl_surface
    /// object and gives it the xwayland_surface role.
    ///
    /// It is illegal to create an xwayland_surface_v1 for a wl_surface
    /// which already has an assigned role and this will result in the
    /// `role` protocol error.
    ///
    /// See the documentation of xwayland_surface_v1 for more details
    /// about what an xwayland_surface_v1 is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_xwayland_surface(
        &mut self,
        _slf: &Rc<XwaylandShellV1>,
        id: &Rc<XwaylandSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_get_xwayland_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xwayland_shell_v1.get_xwayland_surface message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for XwaylandShellV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XwaylandShellV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xwayland_shell_v1#{}.destroy()\n", client_id, id);
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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xwayland_shell_v1#{}.get_xwayland_surface(id: xwayland_surface_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = XwaylandSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_xwayland_surface(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_xwayland_surface(&self, arg0, arg1);
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
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "get_xwayland_surface",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for XwaylandShellV1 {
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

impl XwaylandShellV1 {
    /// Since when the error.role enum variant is available.
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XwaylandShellV1Error(pub u32);

impl XwaylandShellV1Error {
    /// given wl_surface has another role
    pub const ROLE: Self = Self(0);
}

impl Debug for XwaylandShellV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
