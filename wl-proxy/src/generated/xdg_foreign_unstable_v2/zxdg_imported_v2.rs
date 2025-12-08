//! an imported surface handle
//!
//! An xdg_imported object represents an imported reference to surface exported
//! by some client. A client can use this interface to manipulate
//! relationships between its own surfaces and the imported surface.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zxdg_imported_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZxdgImportedV2 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZxdgImportedV2MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZxdgImportedV2MessageHandler for DefaultMessageHandler { }

impl MetaZxdgImportedV2 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZxdgImportedV2 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZxdgImportedV2, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZxdgImportedV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZxdgImportedV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZxdgImportedV2 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
    #[allow(dead_code)]
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
        surface: &Rc<MetaWlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            surface,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the destroyed message is available.
    #[allow(dead_code)]
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
        ]);
        Ok(())
    }
}

/// A message handler for [ZxdgImportedV2] proxies.
#[allow(dead_code)]
pub trait MetaZxdgImportedV2MessageHandler {
    /// destroy the xdg_imported object
    ///
    /// Notify the compositor that it will no longer use the xdg_imported
    /// object. Any relationship that may have been set up will at this point
    /// be invalidated.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZxdgImportedV2>,
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
    fn set_parent_of(
        &mut self,
        _slf: &Rc<MetaZxdgImportedV2>,
        surface: &Rc<MetaWlSurface>,
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
    fn destroyed(
        &mut self,
        _slf: &Rc<MetaZxdgImportedV2>,
    ) {
        let res = _slf.send_destroyed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_imported_v2.destroyed message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZxdgImportedV2 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.endpoint.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_parent_of(&self, arg0);
                } else {
                    DefaultMessageHandler.set_parent_of(&self, arg0);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroyed(&self);
                } else {
                    DefaultMessageHandler.destroyed(&self);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

impl MetaZxdgImportedV2 {
    /// Since when the error.invalid_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SURFACE__SINCE: u32 = 1;
}

/// error values
///
/// These errors can be emitted in response to invalid xdg_imported
/// requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZxdgImportedV2Error(pub u32);

impl MetaZxdgImportedV2Error {
    /// surface is not an xdg_toplevel
    #[allow(dead_code)]
    pub const INVALID_SURFACE: Self = Self(0);
}

impl Debug for MetaZxdgImportedV2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SURFACE => "INVALID_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
