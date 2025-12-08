//! receive relative pointer motion events
//!
//! The wp_locked_pointer interface represents a locked pointer state.
//!
//! While the lock of this object is active, the wl_pointer objects of the
//! associated seat will not emit any wl_pointer.motion events.
//!
//! This object will send the event 'locked' when the lock is activated.
//! Whenever the lock is activated, it is guaranteed that the locked surface
//! will already have received pointer focus and that the pointer will be
//! within the region passed to the request creating this object.
//!
//! To unlock the pointer, send the destroy request. This will also destroy
//! the wp_locked_pointer object.
//!
//! If the compositor decides to unlock the pointer the unlocked event is
//! sent. See wp_locked_pointer.unlock for details.
//!
//! When unlocking, the compositor may warp the cursor position to the set
//! cursor position hint. If it does, it will not result in any relative
//! motion events emitted via wp_relative_pointer.
//!
//! If the surface the lock was requested on is destroyed and the lock is not
//! yet activated, the wp_locked_pointer object is now defunct and must be
//! destroyed.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_locked_pointer_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpLockedPointerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpLockedPointerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpLockedPointerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpLockedPointerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpLockedPointerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpLockedPointerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpLockedPointerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpLockedPointerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpLockedPointerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the locked pointer object
    ///
    /// Destroy the locked pointer object. If applicable, the compositor will
    /// unlock the pointer.
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

    /// Since when the set_cursor_position_hint message is available.
    #[allow(dead_code)]
    pub const MSG__SET_CURSOR_POSITION_HINT__SINCE: u32 = 1;

    /// set the pointer cursor position hint
    ///
    /// Set the cursor position hint relative to the top left corner of the
    /// surface.
    ///
    /// If the client is drawing its own cursor, it should update the position
    /// hint to the position of its own cursor. A compositor may use this
    /// information to warp the pointer upon unlock in order to avoid pointer
    /// jumps.
    ///
    /// The cursor position hint is double-buffered state, see
    /// wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `surface_x`: surface-local x coordinate
    /// - `surface_y`: surface-local y coordinate
    #[inline]
    pub fn send_set_cursor_position_hint(
        &self,
        surface_x: Fixed,
        surface_y: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            surface_x,
            surface_y,
        );
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
            1,
            arg0.to_wire() as u32,
            arg1.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the set_region message is available.
    #[allow(dead_code)]
    pub const MSG__SET_REGION__SINCE: u32 = 1;

    /// set a new lock region
    ///
    /// Set a new region used to lock the pointer.
    ///
    /// The new lock region is double-buffered, see wl_surface.commit.
    ///
    /// For details about the lock region, see wp_locked_pointer.
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
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the locked message is available.
    #[allow(dead_code)]
    pub const MSG__LOCKED__SINCE: u32 = 1;

    /// lock activation event
    ///
    /// Notification that the pointer lock of the seat's pointer is activated.
    #[inline]
    pub fn send_locked(
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

    /// Since when the unlocked message is available.
    #[allow(dead_code)]
    pub const MSG__UNLOCKED__SINCE: u32 = 1;

    /// lock deactivation event
    ///
    /// Notification that the pointer lock of the seat's pointer is no longer
    /// active. If this is a oneshot pointer lock (see
    /// wp_pointer_constraints.lifetime) this object is now defunct and should
    /// be destroyed. If this is a persistent pointer lock (see
    /// wp_pointer_constraints.lifetime) this pointer lock may again
    /// reactivate in the future.
    #[inline]
    pub fn send_unlocked(
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
            1,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpLockedPointerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpLockedPointerV1MessageHandler {
    /// destroy the locked pointer object
    ///
    /// Destroy the locked pointer object. If applicable, the compositor will
    /// unlock the pointer.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpLockedPointerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_locked_pointer_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set the pointer cursor position hint
    ///
    /// Set the cursor position hint relative to the top left corner of the
    /// surface.
    ///
    /// If the client is drawing its own cursor, it should update the position
    /// hint to the position of its own cursor. A compositor may use this
    /// information to warp the pointer upon unlock in order to avoid pointer
    /// jumps.
    ///
    /// The cursor position hint is double-buffered state, see
    /// wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `surface_x`: surface-local x coordinate
    /// - `surface_y`: surface-local y coordinate
    #[inline]
    fn set_cursor_position_hint(
        &mut self,
        _slf: &Rc<MetaZwpLockedPointerV1>,
        surface_x: Fixed,
        surface_y: Fixed,
    ) {
        let res = _slf.send_set_cursor_position_hint(
            surface_x,
            surface_y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_locked_pointer_v1.set_cursor_position_hint message: {}", Report::new(e));
        }
    }

    /// set a new lock region
    ///
    /// Set a new region used to lock the pointer.
    ///
    /// The new lock region is double-buffered, see wl_surface.commit.
    ///
    /// For details about the lock region, see wp_locked_pointer.
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
        _slf: &Rc<MetaZwpLockedPointerV1>,
        region: Option<&Rc<MetaWlRegion>>,
    ) {
        let res = _slf.send_set_region(
            region,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_locked_pointer_v1.set_region message: {}", Report::new(e));
        }
    }

    /// lock activation event
    ///
    /// Notification that the pointer lock of the seat's pointer is activated.
    #[inline]
    fn locked(
        &mut self,
        _slf: &Rc<MetaZwpLockedPointerV1>,
    ) {
        let res = _slf.send_locked(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_locked_pointer_v1.locked message: {}", Report::new(e));
        }
    }

    /// lock deactivation event
    ///
    /// Notification that the pointer lock of the seat's pointer is no longer
    /// active. If this is a oneshot pointer lock (see
    /// wp_pointer_constraints.lifetime) this object is now defunct and should
    /// be destroyed. If this is a persistent pointer lock (see
    /// wp_pointer_constraints.lifetime) this pointer lock may again
    /// reactivate in the future.
    #[inline]
    fn unlocked(
        &mut self,
        _slf: &Rc<MetaZwpLockedPointerV1>,
    ) {
        let res = _slf.send_unlocked(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_locked_pointer_v1.unlocked message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpLockedPointerV1 {
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                let arg1 = Fixed::from_wire(arg1 as i32);
                if let Some(handler) = handler {
                    (**handler).set_cursor_position_hint(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_cursor_position_hint(&self, arg0, arg1);
                }
            }
            2 => {
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
                    (**handler).locked(&self);
                } else {
                    DefaultMessageHandler.locked(&self);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).unlocked(&self);
                } else {
                    DefaultMessageHandler.unlocked(&self);
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

