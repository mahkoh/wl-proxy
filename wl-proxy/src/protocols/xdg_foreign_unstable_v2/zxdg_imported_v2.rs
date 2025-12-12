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

impl ZxdgImportedV2 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZxdgImportedV2;
    pub const INTERFACE_NAME: &str = "zxdg_imported_v2";
}

impl ZxdgImportedV2 {
    pub fn set_handler(&self, handler: impl ZxdgImportedV2Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_imported_v2#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
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
    pub fn send_set_parent_of(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_imported_v2#{}.set_parent_of(surface: wl_surface#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
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
    pub fn send_destroyed(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zxdg_imported_v2#{}.destroyed()\n", client.endpoint.id, id);
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
        ]);
        Ok(())
    }
}

/// A message handler for [ZxdgImportedV2] proxies.
pub trait ZxdgImportedV2Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZxdgImportedV2>) {
        let _ = slf.core.delete_id();
    }

    /// destroy the xdg_imported object
    ///
    /// Notify the compositor that it will no longer use the xdg_imported
    /// object. Any relationship that may have been set up will at this point
    /// be invalidated.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<ZxdgImportedV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_imported_v2.destroy message: {}", Report::new(e));
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
        _slf: &Rc<ZxdgImportedV2>,
        surface: &Rc<WlSurface>,
    ) {
        let res = _slf.send_set_parent_of(
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_imported_v2.set_parent_of message: {}", Report::new(e));
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
        _slf: &Rc<ZxdgImportedV2>,
    ) {
        let res = _slf.send_destroyed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_imported_v2.destroyed message: {}", Report::new(e));
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
        let Some(mut handler) = self.handler.try_borrow() else {
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_imported_v2#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_imported_v2#{}.set_parent_of(surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface));
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zxdg_imported_v2#{}.destroyed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_destroyed(&self);
                } else {
                    DefaultHandler.handle_destroyed(&self);
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
