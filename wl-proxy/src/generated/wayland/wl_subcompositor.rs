//! sub-surface compositing
//!
//! The global interface exposing sub-surface compositing capabilities.
//! A wl_surface, that has sub-surfaces associated, is called the
//! parent surface. Sub-surfaces can be arbitrarily nested and create
//! a tree of sub-surfaces.
//!
//! The root surface in a tree of sub-surfaces is the main
//! surface. The main surface cannot be a sub-surface, because
//! sub-surfaces must always have a parent.
//!
//! A main surface with its sub-surfaces forms a (compound) window.
//! For window management purposes, this set of wl_surface objects is
//! to be considered as a single window, and it should also behave as
//! such.
//!
//! The aim of sub-surfaces is to offload some of the compositing work
//! within a window from clients to the compositor. A prime example is
//! a video player with decorations and video in separate wl_surface
//! objects. This should allow the compositor to pass YUV video buffer
//! processing to dedicated overlay hardware when possible.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_subcompositor proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlSubcompositor {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlSubcompositorMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlSubcompositorMessageHandler for DefaultMessageHandler { }

impl MetaWlSubcompositor {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWlSubcompositor {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlSubcompositor, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlSubcompositor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlSubcompositor")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlSubcompositor {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unbind from the subcompositor interface
    ///
    /// Informs the server that the client will not be using this
    /// protocol object anymore. This does not affect any other
    /// objects, wl_subsurface objects included.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_subcompositor#{}.destroy()", id);
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

    /// Since when the get_subsurface message is available.
    #[allow(dead_code)]
    pub const MSG__GET_SUBSURFACE__SINCE: u32 = 1;

    /// give a surface the role sub-surface
    ///
    /// Create a sub-surface interface for the given surface, and
    /// associate it with the given parent surface. This turns a
    /// plain wl_surface into a sub-surface.
    ///
    /// The to-be sub-surface must not already have another role, and it
    /// must not have an existing wl_subsurface object. Otherwise the
    /// bad_surface protocol error is raised.
    ///
    /// Adding sub-surfaces to a parent is a double-buffered operation on the
    /// parent (see wl_surface.commit). The effect of adding a sub-surface
    /// becomes visible on the next time the state of the parent surface is
    /// applied.
    ///
    /// The parent surface must not be one of the child surface's descendants,
    /// and the parent must be different from the child surface, otherwise the
    /// bad_parent protocol error is raised.
    ///
    /// This request modifies the behaviour of wl_surface.commit request on
    /// the sub-surface, see the documentation on wl_subsurface interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the new sub-surface object ID
    /// - `surface`: the surface to be turned into a sub-surface
    /// - `parent`: the parent surface
    #[inline]
    pub fn send_get_subsurface(
        &self,
        id: &Rc<MetaWlSubsurface>,
        surface: &Rc<MetaWlSurface>,
        parent: &Rc<MetaWlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            surface,
            parent,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("parent")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        eprintln!("server      <= wl_subcompositor#{}.get_subsurface(id: wl_subsurface#{}, surface: wl_surface#{}, parent: wl_surface#{})", id, arg0_id, arg1_id, arg2_id);
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
            arg0_id,
            arg1_id,
            arg2_id,
        ]);
        Ok(())
    }
}

/// A message handler for [WlSubcompositor] proxies.
#[allow(dead_code)]
pub trait MetaWlSubcompositorMessageHandler {
    /// unbind from the subcompositor interface
    ///
    /// Informs the server that the client will not be using this
    /// protocol object anymore. This does not affect any other
    /// objects, wl_subsurface objects included.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWlSubcompositor>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_subcompositor.destroy message: {}", Report::new(e));
        }
    }

    /// give a surface the role sub-surface
    ///
    /// Create a sub-surface interface for the given surface, and
    /// associate it with the given parent surface. This turns a
    /// plain wl_surface into a sub-surface.
    ///
    /// The to-be sub-surface must not already have another role, and it
    /// must not have an existing wl_subsurface object. Otherwise the
    /// bad_surface protocol error is raised.
    ///
    /// Adding sub-surfaces to a parent is a double-buffered operation on the
    /// parent (see wl_surface.commit). The effect of adding a sub-surface
    /// becomes visible on the next time the state of the parent surface is
    /// applied.
    ///
    /// The parent surface must not be one of the child surface's descendants,
    /// and the parent must be different from the child surface, otherwise the
    /// bad_parent protocol error is raised.
    ///
    /// This request modifies the behaviour of wl_surface.commit request on
    /// the sub-surface, see the documentation on wl_subsurface interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the new sub-surface object ID
    /// - `surface`: the surface to be turned into a sub-surface
    /// - `parent`: the parent surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_subsurface(
        &mut self,
        _slf: &Rc<MetaWlSubcompositor>,
        id: &Rc<MetaWlSubsurface>,
        surface: &Rc<MetaWlSurface>,
        parent: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_subsurface(
            id,
            surface,
            parent,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_subcompositor.get_subsurface message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlSubcompositor {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> wl_subcompositor#{}.destroy()", client.endpoint.id, msg[0]);
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
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                eprintln!("client#{:04} -> wl_subcompositor#{}.get_subsurface(id: wl_subsurface#{}, surface: wl_surface#{}, parent: wl_surface#{})", client.endpoint.id, msg[0], arg0, arg1, arg2);
                let arg0_id = arg0;
                let arg0 = MetaWlSubsurface::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError::WrongObjectType("parent", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).get_subsurface(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.get_subsurface(&self, arg0, arg1, arg2);
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
        let handler = &mut *self.handler.borrow();
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
            1 => "get_subsurface",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaWlSubcompositor {
    /// Since when the error.bad_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_SURFACE__SINCE: u32 = 1;
    /// Since when the error.bad_parent enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_PARENT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlSubcompositorError(pub u32);

impl MetaWlSubcompositorError {
    /// the to-be sub-surface is invalid
    #[allow(dead_code)]
    pub const BAD_SURFACE: Self = Self(0);

    /// the to-be sub-surface parent is invalid
    #[allow(dead_code)]
    pub const BAD_PARENT: Self = Self(1);
}

impl Debug for MetaWlSubcompositorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BAD_SURFACE => "BAD_SURFACE",
            Self::BAD_PARENT => "BAD_PARENT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
