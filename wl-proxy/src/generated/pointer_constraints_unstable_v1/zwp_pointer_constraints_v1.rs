//! constrain the movement of a pointer
//!
//! The global interface exposing pointer constraining functionality. It
//! exposes two requests: lock_pointer for locking the pointer to its
//! position, and confine_pointer for locking the pointer to a region.
//!
//! The lock_pointer and confine_pointer requests create the objects
//! wp_locked_pointer and wp_confined_pointer respectively, and the client can
//! use these objects to interact with the lock.
//!
//! For any surface, only one lock or confinement may be active across all
//! wl_pointer objects of the same seat. If a lock or confinement is requested
//! when another lock or confinement is active or requested on the same surface
//! and with any of the wl_pointer objects of the same seat, an
//! 'already_constrained' error will be raised.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_pointer_constraints_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpPointerConstraintsV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpPointerConstraintsV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpPointerConstraintsV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpPointerConstraintsV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpPointerConstraintsV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpPointerConstraintsV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpPointerConstraintsV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the pointer constraints manager object
    ///
    /// Used by the client to notify the server that it will no longer use this
    /// pointer constraints object.
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

    /// Since when the lock_pointer message is available.
    #[allow(dead_code)]
    pub const MSG__LOCK_POINTER__SINCE: u32 = 1;

    /// lock pointer to a position
    ///
    /// The lock_pointer request lets the client request to disable movements of
    /// the virtual pointer (i.e. the cursor), effectively locking the pointer
    /// to a position. This request may not take effect immediately; in the
    /// future, when the compositor deems implementation-specific constraints
    /// are satisfied, the pointer lock will be activated and the compositor
    /// sends a locked event.
    ///
    /// The protocol provides no guarantee that the constraints are ever
    /// satisfied, and does not require the compositor to send an error if the
    /// constraints cannot ever be satisfied. It is thus possible to request a
    /// lock that will never activate.
    ///
    /// There may not be another pointer constraint of any kind requested or
    /// active on the surface for any of the wl_pointer objects of the seat of
    /// the passed pointer when requesting a lock. If there is, an error will be
    /// raised. See general pointer lock documentation for more details.
    ///
    /// The intersection of the region passed with this request and the input
    /// region of the surface is used to determine where the pointer must be
    /// in order for the lock to activate. It is up to the compositor whether to
    /// warp the pointer or require some kind of user interaction for the lock
    /// to activate. If the region is null the surface input region is used.
    ///
    /// A surface may receive pointer focus without the lock being activated.
    ///
    /// The request creates a new object wp_locked_pointer which is used to
    /// interact with the lock as well as receive updates about its state. See
    /// the the description of wp_locked_pointer for further information.
    ///
    /// Note that while a pointer is locked, the wl_pointer objects of the
    /// corresponding seat will not emit any wl_pointer.motion events, but
    /// relative motion events will still be emitted via wp_relative_pointer
    /// objects of the same seat. wl_pointer.axis and wl_pointer.button events
    /// are unaffected.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: surface to lock pointer to
    /// - `pointer`: the pointer that should be locked
    /// - `region`: region of surface
    /// - `lifetime`: lock lifetime
    #[inline]
    pub fn send_lock_pointer(
        &self,
        id: &Rc<MetaZwpLockedPointerV1>,
        surface: &Rc<MetaWlSurface>,
        pointer: &Rc<MetaWlPointer>,
        region: Option<&Rc<MetaWlRegion>>,
        lifetime: MetaZwpPointerConstraintsV1Lifetime,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            id,
            surface,
            pointer,
            region,
            lifetime,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let arg3 = arg3.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let arg2 = match arg2.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let arg3 = match arg3 {
            None => 0,
            Some(arg3) => match arg3.server_obj_id.get() {
                None => return Err(ObjectError),
                Some(id) => id,
            },
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
            arg2,
            arg3,
            arg4.0,
        ]);
        Ok(())
    }

    /// Since when the confine_pointer message is available.
    #[allow(dead_code)]
    pub const MSG__CONFINE_POINTER__SINCE: u32 = 1;

    /// confine pointer to a region
    ///
    /// The confine_pointer request lets the client request to confine the
    /// pointer cursor to a given region. This request may not take effect
    /// immediately; in the future, when the compositor deems implementation-
    /// specific constraints are satisfied, the pointer confinement will be
    /// activated and the compositor sends a confined event.
    ///
    /// The intersection of the region passed with this request and the input
    /// region of the surface is used to determine where the pointer must be
    /// in order for the confinement to activate. It is up to the compositor
    /// whether to warp the pointer or require some kind of user interaction for
    /// the confinement to activate. If the region is null the surface input
    /// region is used.
    ///
    /// The request will create a new object wp_confined_pointer which is used
    /// to interact with the confinement as well as receive updates about its
    /// state. See the the description of wp_confined_pointer for further
    /// information.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: surface to lock pointer to
    /// - `pointer`: the pointer that should be confined
    /// - `region`: region of surface
    /// - `lifetime`: confinement lifetime
    #[inline]
    pub fn send_confine_pointer(
        &self,
        id: &Rc<MetaZwpConfinedPointerV1>,
        surface: &Rc<MetaWlSurface>,
        pointer: &Rc<MetaWlPointer>,
        region: Option<&Rc<MetaWlRegion>>,
        lifetime: MetaZwpPointerConstraintsV1Lifetime,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            id,
            surface,
            pointer,
            region,
            lifetime,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let arg3 = arg3.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let arg2 = match arg2.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let arg3 = match arg3 {
            None => 0,
            Some(arg3) => match arg3.server_obj_id.get() {
                None => return Err(ObjectError),
                Some(id) => id,
            },
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
            arg2,
            arg3,
            arg4.0,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpPointerConstraintsV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpPointerConstraintsV1MessageHandler {
    /// destroy the pointer constraints manager object
    ///
    /// Used by the client to notify the server that it will no longer use this
    /// pointer constraints object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpPointerConstraintsV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_constraints_v1.destroy message: {}", Report::new(e));
        }
    }

    /// lock pointer to a position
    ///
    /// The lock_pointer request lets the client request to disable movements of
    /// the virtual pointer (i.e. the cursor), effectively locking the pointer
    /// to a position. This request may not take effect immediately; in the
    /// future, when the compositor deems implementation-specific constraints
    /// are satisfied, the pointer lock will be activated and the compositor
    /// sends a locked event.
    ///
    /// The protocol provides no guarantee that the constraints are ever
    /// satisfied, and does not require the compositor to send an error if the
    /// constraints cannot ever be satisfied. It is thus possible to request a
    /// lock that will never activate.
    ///
    /// There may not be another pointer constraint of any kind requested or
    /// active on the surface for any of the wl_pointer objects of the seat of
    /// the passed pointer when requesting a lock. If there is, an error will be
    /// raised. See general pointer lock documentation for more details.
    ///
    /// The intersection of the region passed with this request and the input
    /// region of the surface is used to determine where the pointer must be
    /// in order for the lock to activate. It is up to the compositor whether to
    /// warp the pointer or require some kind of user interaction for the lock
    /// to activate. If the region is null the surface input region is used.
    ///
    /// A surface may receive pointer focus without the lock being activated.
    ///
    /// The request creates a new object wp_locked_pointer which is used to
    /// interact with the lock as well as receive updates about its state. See
    /// the the description of wp_locked_pointer for further information.
    ///
    /// Note that while a pointer is locked, the wl_pointer objects of the
    /// corresponding seat will not emit any wl_pointer.motion events, but
    /// relative motion events will still be emitted via wp_relative_pointer
    /// objects of the same seat. wl_pointer.axis and wl_pointer.button events
    /// are unaffected.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: surface to lock pointer to
    /// - `pointer`: the pointer that should be locked
    /// - `region`: region of surface
    /// - `lifetime`: lock lifetime
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn lock_pointer(
        &mut self,
        _slf: &Rc<MetaZwpPointerConstraintsV1>,
        id: &Rc<MetaZwpLockedPointerV1>,
        surface: &Rc<MetaWlSurface>,
        pointer: &Rc<MetaWlPointer>,
        region: Option<&Rc<MetaWlRegion>>,
        lifetime: MetaZwpPointerConstraintsV1Lifetime,
    ) {
        let res = _slf.send_lock_pointer(
            id,
            surface,
            pointer,
            region,
            lifetime,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_constraints_v1.lock_pointer message: {}", Report::new(e));
        }
    }

    /// confine pointer to a region
    ///
    /// The confine_pointer request lets the client request to confine the
    /// pointer cursor to a given region. This request may not take effect
    /// immediately; in the future, when the compositor deems implementation-
    /// specific constraints are satisfied, the pointer confinement will be
    /// activated and the compositor sends a confined event.
    ///
    /// The intersection of the region passed with this request and the input
    /// region of the surface is used to determine where the pointer must be
    /// in order for the confinement to activate. It is up to the compositor
    /// whether to warp the pointer or require some kind of user interaction for
    /// the confinement to activate. If the region is null the surface input
    /// region is used.
    ///
    /// The request will create a new object wp_confined_pointer which is used
    /// to interact with the confinement as well as receive updates about its
    /// state. See the the description of wp_confined_pointer for further
    /// information.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: surface to lock pointer to
    /// - `pointer`: the pointer that should be confined
    /// - `region`: region of surface
    /// - `lifetime`: confinement lifetime
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn confine_pointer(
        &mut self,
        _slf: &Rc<MetaZwpPointerConstraintsV1>,
        id: &Rc<MetaZwpConfinedPointerV1>,
        surface: &Rc<MetaWlSurface>,
        pointer: &Rc<MetaWlPointer>,
        region: Option<&Rc<MetaWlRegion>>,
        lifetime: MetaZwpPointerConstraintsV1Lifetime,
    ) {
        let res = _slf.send_confine_pointer(
            id,
            surface,
            pointer,
            region,
            lifetime,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_pointer_constraints_v1.confine_pointer message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpPointerConstraintsV1 {
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
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpLockedPointerV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let Some(arg2) = client.lookup(arg2) else {
                    return Err(ObjectError);
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlPointer>() else {
                    return Err(ObjectError);
                };
                let arg3 = if arg3 == 0 {
                    None
                } else {
                    let Some(arg3) = client.lookup(arg3) else {
                        return Err(ObjectError);
                    };
                    let Ok(arg3) = (arg3 as Rc<dyn Any>).downcast::<MetaWlRegion>() else {
                        return Err(ObjectError);
                    };
                    Some(arg3)
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                let arg3 = arg3.as_ref();
                let arg4 = MetaZwpPointerConstraintsV1Lifetime(arg4);
                if let Some(handler) = handler {
                    (**handler).lock_pointer(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultMessageHandler.lock_pointer(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpConfinedPointerV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let Some(arg2) = client.lookup(arg2) else {
                    return Err(ObjectError);
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlPointer>() else {
                    return Err(ObjectError);
                };
                let arg3 = if arg3 == 0 {
                    None
                } else {
                    let Some(arg3) = client.lookup(arg3) else {
                        return Err(ObjectError);
                    };
                    let Ok(arg3) = (arg3 as Rc<dyn Any>).downcast::<MetaWlRegion>() else {
                        return Err(ObjectError);
                    };
                    Some(arg3)
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                let arg3 = arg3.as_ref();
                let arg4 = MetaZwpPointerConstraintsV1Lifetime(arg4);
                if let Some(handler) = handler {
                    (**handler).confine_pointer(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultMessageHandler.confine_pointer(&self, arg0, arg1, arg2, arg3, arg4);
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

impl MetaZwpPointerConstraintsV1 {
    /// Since when the error.already_constrained enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_CONSTRAINED__SINCE: u32 = 1;

    /// Since when the lifetime.oneshot enum variant is available.
    #[allow(dead_code)]
    pub const ENM__LIFETIME_ONESHOT__SINCE: u32 = 1;
    /// Since when the lifetime.persistent enum variant is available.
    #[allow(dead_code)]
    pub const ENM__LIFETIME_PERSISTENT__SINCE: u32 = 1;
}

/// wp_pointer_constraints error values
///
/// These errors can be emitted in response to wp_pointer_constraints
/// requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwpPointerConstraintsV1Error(pub u32);

impl MetaZwpPointerConstraintsV1Error {
    /// pointer constraint already requested on that surface
    #[allow(dead_code)]
    pub const ALREADY_CONSTRAINED: Self = Self(1);
}

impl Debug for MetaZwpPointerConstraintsV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_CONSTRAINED => "ALREADY_CONSTRAINED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// constraint lifetime
///
/// These values represent different lifetime semantics. They are passed
/// as arguments to the factory requests to specify how the constraint
/// lifetimes should be managed.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwpPointerConstraintsV1Lifetime(pub u32);

impl MetaZwpPointerConstraintsV1Lifetime {
    /// the pointer constraint is defunct once deactivated
    ///
    /// A oneshot pointer constraint will never reactivate once it has been
    /// deactivated. See the corresponding deactivation event
    /// (wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for
    /// details.
    #[allow(dead_code)]
    pub const ONESHOT: Self = Self(1);

    /// the pointer constraint may reactivate
    ///
    /// A persistent pointer constraint may again reactivate once it has
    /// been deactivated. See the corresponding deactivation event
    /// (wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for
    /// details.
    #[allow(dead_code)]
    pub const PERSISTENT: Self = Self(2);
}

impl Debug for MetaZwpPointerConstraintsV1Lifetime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ONESHOT => "ONESHOT",
            Self::PERSISTENT => "PERSISTENT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
