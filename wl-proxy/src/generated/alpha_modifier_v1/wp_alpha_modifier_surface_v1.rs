//! alpha modifier object for a surface
//!
//! This interface allows the client to set a factor for the alpha values on
//! a surface, which can be used to offload such operations to the compositor.
//! The default factor is UINT32_MAX.
//!
//! This object has to be destroyed before the associated wl_surface. Once the
//! wl_surface is destroyed, all request on this object will raise the
//! no_surface error.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_alpha_modifier_surface_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpAlphaModifierSurfaceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpAlphaModifierSurfaceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpAlphaModifierSurfaceV1MessageHandler for DefaultMessageHandler { }

impl MetaWpAlphaModifierSurfaceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpAlphaModifierSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpAlphaModifierSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpAlphaModifierSurfaceV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the alpha modifier object
    ///
    /// This destroys the object, and is equivalent to set_multiplier with
    /// a value of UINT32_MAX, with the same double-buffered semantics as
    /// set_multiplier.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the set_multiplier message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MULTIPLIER__SINCE: u32 = 1;

    /// specify the alpha multiplier
    ///
    /// Sets the alpha multiplier for the surface. The alpha multiplier is
    /// double-buffered state, see wl_surface.commit for details.
    ///
    /// This factor is applied in the compositor's blending space, as an
    /// additional step after the processing of per-pixel alpha values for the
    /// wl_surface. The exact meaning of the factor is thus undefined, unless
    /// the blending space is specified in a different extension.
    ///
    /// This multiplier is applied even if the buffer attached to the
    /// wl_surface doesn't have an alpha channel; in that case an alpha value
    /// of one is used instead.
    ///
    /// Zero means completely transparent, UINT32_MAX means completely opaque.
    ///
    /// # Arguments
    ///
    /// - `factor`:
    #[inline]
    pub fn send_set_multiplier(
        &self,
        factor: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            factor,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [WpAlphaModifierSurfaceV1] proxies.
#[allow(dead_code)]
pub trait MetaWpAlphaModifierSurfaceV1MessageHandler {
    /// destroy the alpha modifier object
    ///
    /// This destroys the object, and is equivalent to set_multiplier with
    /// a value of UINT32_MAX, with the same double-buffered semantics as
    /// set_multiplier.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpAlphaModifierSurfaceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_alpha_modifier_surface_v1.destroy message: {}", Report::new(e));
        }
    }

    /// specify the alpha multiplier
    ///
    /// Sets the alpha multiplier for the surface. The alpha multiplier is
    /// double-buffered state, see wl_surface.commit for details.
    ///
    /// This factor is applied in the compositor's blending space, as an
    /// additional step after the processing of per-pixel alpha values for the
    /// wl_surface. The exact meaning of the factor is thus undefined, unless
    /// the blending space is specified in a different extension.
    ///
    /// This multiplier is applied even if the buffer attached to the
    /// wl_surface doesn't have an alpha channel; in that case an alpha value
    /// of one is used instead.
    ///
    /// Zero means completely transparent, UINT32_MAX means completely opaque.
    ///
    /// # Arguments
    ///
    /// - `factor`:
    #[inline]
    fn set_multiplier(
        &mut self,
        _slf: &Rc<MetaWpAlphaModifierSurfaceV1>,
        factor: u32,
    ) {
        let res = _slf.send_set_multiplier(
            factor,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_alpha_modifier_surface_v1.set_multiplier message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpAlphaModifierSurfaceV1 {
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
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).set_multiplier(&self, arg0);
                } else {
                    DefaultMessageHandler.set_multiplier(&self, arg0);
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

impl MetaWpAlphaModifierSurfaceV1 {
    /// Since when the error.no_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_SURFACE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpAlphaModifierSurfaceV1Error(pub u32);

impl MetaWpAlphaModifierSurfaceV1Error {
    /// wl_surface was destroyed
    #[allow(dead_code)]
    pub const NO_SURFACE: Self = Self(0);
}

impl Debug for MetaWpAlphaModifierSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NO_SURFACE => "NO_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
