//! background effects for a surface
//!
//! The background effect object provides a way to specify a region behind
//! a surface that should have background effects like blur applied.
//!
//! If the wl_surface associated with the ext_background_effect_surface_v1
//! object has been destroyed, this object becomes inert.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_background_effect_surface_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtBackgroundEffectSurfaceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtBackgroundEffectSurfaceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtBackgroundEffectSurfaceV1MessageHandler for DefaultMessageHandler { }

impl MetaExtBackgroundEffectSurfaceV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaExtBackgroundEffectSurfaceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtBackgroundEffectSurfaceV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtBackgroundEffectSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtBackgroundEffectSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtBackgroundEffectSurfaceV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// release the blur object
    ///
    /// Informs the server that the client will no longer be using this protocol
    /// object. The effect regions will be removed on the next commit.
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

    /// Since when the set_blur_region message is available.
    #[allow(dead_code)]
    pub const MSG__SET_BLUR_REGION__SINCE: u32 = 1;

    /// set blur region
    ///
    /// This request sets the region of the surface that will have its
    /// background blurred.
    ///
    /// The blur region is specified in the surface-local coordinates, and
    /// clipped by the compositor to the surface size.
    ///
    /// The initial value for the blur region is empty. Setting the pending
    /// blur region has copy semantics, and the wl_region object can be
    /// destroyed immediately. A NULL wl_region removes the effect.
    ///
    /// The blur region is double-buffered state, and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The blur algorithm is subject to compositor policies.
    ///
    /// If the associated surface has been destroyed, the surface_destroyed
    /// error will be raised.
    ///
    /// # Arguments
    ///
    /// - `region`: blur region of the surface
    #[inline]
    pub fn send_set_blur_region(
        &self,
        region: Option<&Rc<MetaWlRegion>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            region,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError),
                Some(id) => id,
            },
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
}

/// A message handler for [ExtBackgroundEffectSurfaceV1] proxies.
#[allow(dead_code)]
pub trait MetaExtBackgroundEffectSurfaceV1MessageHandler {
    /// release the blur object
    ///
    /// Informs the server that the client will no longer be using this protocol
    /// object. The effect regions will be removed on the next commit.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtBackgroundEffectSurfaceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_background_effect_surface_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set blur region
    ///
    /// This request sets the region of the surface that will have its
    /// background blurred.
    ///
    /// The blur region is specified in the surface-local coordinates, and
    /// clipped by the compositor to the surface size.
    ///
    /// The initial value for the blur region is empty. Setting the pending
    /// blur region has copy semantics, and the wl_region object can be
    /// destroyed immediately. A NULL wl_region removes the effect.
    ///
    /// The blur region is double-buffered state, and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The blur algorithm is subject to compositor policies.
    ///
    /// If the associated surface has been destroyed, the surface_destroyed
    /// error will be raised.
    ///
    /// # Arguments
    ///
    /// - `region`: blur region of the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_blur_region(
        &mut self,
        _slf: &Rc<MetaExtBackgroundEffectSurfaceV1>,
        region: Option<&Rc<MetaWlRegion>>,
    ) {
        let res = _slf.send_set_blur_region(
            region,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_background_effect_surface_v1.set_blur_region message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtBackgroundEffectSurfaceV1 {
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
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let Some(arg0) = client.endpoint.lookup(arg0) else {
                        return Err(ObjectError);
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlRegion>() else {
                        return Err(ObjectError);
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).set_blur_region(&self, arg0);
                } else {
                    DefaultMessageHandler.set_blur_region(&self, arg0);
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
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

impl MetaExtBackgroundEffectSurfaceV1 {
    /// Since when the error.surface_destroyed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_SURFACE_DESTROYED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaExtBackgroundEffectSurfaceV1Error(pub u32);

impl MetaExtBackgroundEffectSurfaceV1Error {
    /// the associated surface has been destroyed
    #[allow(dead_code)]
    pub const SURFACE_DESTROYED: Self = Self(0);
}

impl Debug for MetaExtBackgroundEffectSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SURFACE_DESTROYED => "SURFACE_DESTROYED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
