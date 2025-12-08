//! confined pointer object
//!
//! The wp_confined_pointer interface represents a confined pointer state.
//!
//! This object will send the event 'confined' when the confinement is
//! activated. Whenever the confinement is activated, it is guaranteed that
//! the surface the pointer is confined to will already have received pointer
//! focus and that the pointer will be within the region passed to the request
//! creating this object. It is up to the compositor to decide whether this
//! requires some user interaction and if the pointer will warp to within the
//! passed region if outside.
//!
//! To unconfine the pointer, send the destroy request. This will also destroy
//! the wp_confined_pointer object.
//!
//! If the compositor decides to unconfine the pointer the unconfined event is
//! sent. The wp_confined_pointer object is at this point defunct and should
//! be destroyed.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_confined_pointer_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpConfinedPointerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpConfinedPointerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpConfinedPointerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpConfinedPointerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpConfinedPointerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpConfinedPointerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpConfinedPointerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the confined pointer object
    ///
    /// Destroy the confined pointer object. If applicable, the compositor will
    /// unconfine the pointer.
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

    /// Since when the set_region message is available.
    #[allow(dead_code)]
    pub const MSG__SET_REGION__SINCE: u32 = 1;

    /// set a new confine region
    ///
    /// Set a new region used to confine the pointer.
    ///
    /// The new confine region is double-buffered, see wl_surface.commit.
    ///
    /// If the confinement is active when the new confinement region is applied
    /// and the pointer ends up outside of newly applied region, the pointer may
    /// warped to a position within the new confinement region. If warped, a
    /// wl_pointer.motion event will be emitted, but no
    /// wp_relative_pointer.relative_motion event.
    ///
    /// The compositor may also, instead of using the new region, unconfine the
    /// pointer.
    ///
    /// For details about the confine region, see wp_confined_pointer.
    ///
    /// # Arguments
    ///
    /// - `region`: region of surface
    #[inline]
    pub fn send_set_region(
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
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the confined message is available.
    #[allow(dead_code)]
    pub const MSG__CONFINED__SINCE: u32 = 1;

    /// pointer confined
    ///
    /// Notification that the pointer confinement of the seat's pointer is
    /// activated.
    #[inline]
    pub fn send_confined(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
        ]);
        Ok(())
    }

    /// Since when the unconfined message is available.
    #[allow(dead_code)]
    pub const MSG__UNCONFINED__SINCE: u32 = 1;

    /// pointer unconfined
    ///
    /// Notification that the pointer confinement of the seat's pointer is no
    /// longer active. If this is a oneshot pointer confinement (see
    /// wp_pointer_constraints.lifetime) this object is now defunct and should
    /// be destroyed. If this is a persistent pointer confinement (see
    /// wp_pointer_constraints.lifetime) this pointer confinement may again
    /// reactivate in the future.
    #[inline]
    pub fn send_unconfined(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpConfinedPointerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpConfinedPointerV1MessageHandler {
    /// destroy the confined pointer object
    ///
    /// Destroy the confined pointer object. If applicable, the compositor will
    /// unconfine the pointer.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpConfinedPointerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_confined_pointer_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set a new confine region
    ///
    /// Set a new region used to confine the pointer.
    ///
    /// The new confine region is double-buffered, see wl_surface.commit.
    ///
    /// If the confinement is active when the new confinement region is applied
    /// and the pointer ends up outside of newly applied region, the pointer may
    /// warped to a position within the new confinement region. If warped, a
    /// wl_pointer.motion event will be emitted, but no
    /// wp_relative_pointer.relative_motion event.
    ///
    /// The compositor may also, instead of using the new region, unconfine the
    /// pointer.
    ///
    /// For details about the confine region, see wp_confined_pointer.
    ///
    /// # Arguments
    ///
    /// - `region`: region of surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_region(
        &mut self,
        _slf: &Rc<MetaZwpConfinedPointerV1>,
        region: Option<&Rc<MetaWlRegion>>,
    ) {
        let res = _slf.send_set_region(
            region,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_confined_pointer_v1.set_region message: {}", Report::new(e));
        }
    }

    /// pointer confined
    ///
    /// Notification that the pointer confinement of the seat's pointer is
    /// activated.
    #[inline]
    fn confined(
        &mut self,
        _slf: &Rc<MetaZwpConfinedPointerV1>,
    ) {
        let res = _slf.send_confined(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_confined_pointer_v1.confined message: {}", Report::new(e));
        }
    }

    /// pointer unconfined
    ///
    /// Notification that the pointer confinement of the seat's pointer is no
    /// longer active. If this is a oneshot pointer confinement (see
    /// wp_pointer_constraints.lifetime) this object is now defunct and should
    /// be destroyed. If this is a persistent pointer confinement (see
    /// wp_pointer_constraints.lifetime) this pointer confinement may again
    /// reactivate in the future.
    #[inline]
    fn unconfined(
        &mut self,
        _slf: &Rc<MetaZwpConfinedPointerV1>,
    ) {
        let res = _slf.send_unconfined(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_confined_pointer_v1.unconfined message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpConfinedPointerV1 {
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
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let Some(arg0) = client.lookup(arg0) else {
                        return Err(ObjectError);
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlRegion>() else {
                        return Err(ObjectError);
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).set_region(&self, arg0);
                } else {
                    DefaultMessageHandler.set_region(&self, arg0);
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
                    (**handler).confined(&self);
                } else {
                    DefaultMessageHandler.confined(&self);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).unconfined(&self);
                } else {
                    DefaultMessageHandler.unconfined(&self);
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

