//! an imported surface handle
//!
//! An xdg_imported object represents an imported reference to surface exported
//! by some client. A client can use this interface to manipulate
//! relationships between its own surfaces and the imported surface.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zxdg_imported_v2 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZxdgImportedV2 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZxdgImportedV2Handler>,
}

struct DefaultHandler;

impl ZxdgImportedV2Handler for DefaultHandler { }

impl ConcreteObject for ZxdgImportedV2 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZxdgImportedV2;
    const INTERFACE_NAME: &str = "zxdg_imported_v2";
}

impl ZxdgImportedV2 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZxdgImportedV2Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZxdgImportedV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZxdgImportedV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZxdgImportedV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZxdgImportedV2 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_imported object
    ///
    /// Notify the compositor that it will no longer use the xdg_imported
    /// object. Any relationship that may have been set up will at this point
    /// be invalidated.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_imported_v2#{}.destroy()\n", id);
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

    /// destroy the xdg_imported object
    ///
    /// Notify the compositor that it will no longer use the xdg_imported
    /// object. Any relationship that may have been set up will at this point
    /// be invalidated.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zxdg_imported_v2.destroy", &e);
        }
    }

    /// Since when the set_parent_of message is available.
    pub const MSG__SET_PARENT_OF__SINCE: u32 = 1;

    /// set as the parent of some surface
    ///
    /// Set the imported surface as the parent of some surface of the client.
    /// The passed surface must be an xdg_toplevel equivalent, otherwise an
    /// invalid_surface protocol error is sent. Calling this function sets up
    /// a surface to surface relation with the same stacking and positioning
    /// semantics as xdg_toplevel.set_parent.
    ///
    /// # Arguments
    ///
    /// - `surface`: the child surface
    #[inline]
    pub fn try_send_set_parent_of(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            surface,
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
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_imported_v2#{}.set_parent_of(surface: wl_surface#{})\n", id, arg0);
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

    /// set as the parent of some surface
    ///
    /// Set the imported surface as the parent of some surface of the client.
    /// The passed surface must be an xdg_toplevel equivalent, otherwise an
    /// invalid_surface protocol error is sent. Calling this function sets up
    /// a surface to surface relation with the same stacking and positioning
    /// semantics as xdg_toplevel.set_parent.
    ///
    /// # Arguments
    ///
    /// - `surface`: the child surface
    #[inline]
    pub fn send_set_parent_of(
        &self,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_set_parent_of(
            surface,
        );
        if let Err(e) = res {
            log_send("zxdg_imported_v2.set_parent_of", &e);
        }
    }

    /// Since when the destroyed message is available.
    pub const MSG__DESTROYED__SINCE: u32 = 1;

    /// the imported surface handle has been destroyed
    ///
    /// The imported surface handle has been destroyed and any relationship set
    /// up has been invalidated. This may happen for various reasons, for
    /// example if the exported surface or the exported surface handle has been
    /// destroyed, if the handle used for importing was invalid.
    #[inline]
    pub fn try_send_destroyed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zxdg_imported_v2#{}.destroyed()\n", client_id, id);
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

    /// the imported surface handle has been destroyed
    ///
    /// The imported surface handle has been destroyed and any relationship set
    /// up has been invalidated. This may happen for various reasons, for
    /// example if the exported surface or the exported surface handle has been
    /// destroyed, if the handle used for importing was invalid.
    #[inline]
    pub fn send_destroyed(
        &self,
    ) {
        let res = self.try_send_destroyed(
        );
        if let Err(e) = res {
            log_send("zxdg_imported_v2.destroyed", &e);
        }
    }
}

/// A message handler for [`ZxdgImportedV2`] proxies.
pub trait ZxdgImportedV2Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZxdgImportedV2>) {
        slf.core.delete_id();
    }

    /// destroy the xdg_imported object
    ///
    /// Notify the compositor that it will no longer use the xdg_imported
    /// object. Any relationship that may have been set up will at this point
    /// be invalidated.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZxdgImportedV2>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zxdg_imported_v2.destroy", &e);
        }
    }

    /// set as the parent of some surface
    ///
    /// Set the imported surface as the parent of some surface of the client.
    /// The passed surface must be an xdg_toplevel equivalent, otherwise an
    /// invalid_surface protocol error is sent. Calling this function sets up
    /// a surface to surface relation with the same stacking and positioning
    /// semantics as xdg_toplevel.set_parent.
    ///
    /// # Arguments
    ///
    /// - `surface`: the child surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_parent_of(
        &mut self,
        slf: &Rc<ZxdgImportedV2>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_parent_of(
            surface,
        );
        if let Err(e) = res {
            log_forward("zxdg_imported_v2.set_parent_of", &e);
        }
    }

    /// the imported surface handle has been destroyed
    ///
    /// The imported surface handle has been destroyed and any relationship set
    /// up has been invalidated. This may happen for various reasons, for
    /// example if the exported surface or the exported surface handle has been
    /// destroyed, if the handle used for importing was invalid.
    #[inline]
    fn handle_destroyed(
        &mut self,
        slf: &Rc<ZxdgImportedV2>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_destroyed(
        );
        if let Err(e) = res {
            log_forward("zxdg_imported_v2.destroyed", &e);
        }
    }
}

impl ObjectPrivate for ZxdgImportedV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZxdgImportedV2, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_imported_v2#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_imported_v2#{}.set_parent_of(surface: wl_surface#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
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
                    (**handler).handle_set_parent_of(&self, arg0);
                } else {
                    DefaultHandler.handle_set_parent_of(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zxdg_imported_v2#{}.destroyed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_destroyed(&self);
                } else {
                    DefaultHandler.handle_destroyed(&self);
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
            1 => "set_parent_of",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroyed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZxdgImportedV2 {
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

impl ZxdgImportedV2 {
    /// Since when the error.invalid_surface enum variant is available.
    pub const ENM__ERROR_INVALID_SURFACE__SINCE: u32 = 1;
}

/// error values
///
/// These errors can be emitted in response to invalid xdg_imported
/// requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZxdgImportedV2Error(pub u32);

impl ZxdgImportedV2Error {
    /// surface is not an xdg_toplevel
    pub const INVALID_SURFACE: Self = Self(0);
}

impl Debug for ZxdgImportedV2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SURFACE => "INVALID_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
