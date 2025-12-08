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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_pointer_constraints_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpPointerConstraintsV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpPointerConstraintsV1Handler>,
}

struct DefaultHandler;

impl ZwpPointerConstraintsV1Handler for DefaultHandler { }

impl ConcreteObject for ZwpPointerConstraintsV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwpPointerConstraintsV1;
    const INTERFACE_NAME: &str = "zwp_pointer_constraints_v1";
}

impl ZwpPointerConstraintsV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwpPointerConstraintsV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpPointerConstraintsV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpPointerConstraintsV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpPointerConstraintsV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpPointerConstraintsV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the pointer constraints manager object
    ///
    /// Used by the client to notify the server that it will no longer use this
    /// pointer constraints object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_pointer_constraints_v1#{}.destroy()\n", id);
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

    /// destroy the pointer constraints manager object
    ///
    /// Used by the client to notify the server that it will no longer use this
    /// pointer constraints object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwp_pointer_constraints_v1.destroy", &e);
        }
    }

    /// Since when the lock_pointer message is available.
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
    pub fn try_send_lock_pointer(
        &self,
        id: &Rc<ZwpLockedPointerV1>,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        region: Option<&Rc<WlRegion>>,
        lifetime: ZwpPointerConstraintsV1Lifetime,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("pointer"))),
            Some(id) => id,
        };
        let arg3_id = match arg3 {
            None => 0,
            Some(arg3) => match arg3.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("region"))),
                Some(id) => id,
            },
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: ZwpPointerConstraintsV1Lifetime) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_pointer_constraints_v1#{}.lock_pointer(id: zwp_locked_pointer_v1#{}, surface: wl_surface#{}, pointer: wl_pointer#{}, region: wl_region#{}, lifetime: {:?})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2_id, arg3_id, arg4);
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
            arg1_id,
            arg2_id,
            arg3_id,
            arg4.0,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_lock_pointer(
        &self,
        id: &Rc<ZwpLockedPointerV1>,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        region: Option<&Rc<WlRegion>>,
        lifetime: ZwpPointerConstraintsV1Lifetime,
    ) {
        let res = self.try_send_lock_pointer(
            id,
            surface,
            pointer,
            region,
            lifetime,
        );
        if let Err(e) = res {
            log_send("zwp_pointer_constraints_v1.lock_pointer", &e);
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
    /// - `surface`: surface to lock pointer to
    /// - `pointer`: the pointer that should be locked
    /// - `region`: region of surface
    /// - `lifetime`: lock lifetime
    #[inline]
    pub fn new_try_send_lock_pointer(
        &self,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        region: Option<&Rc<WlRegion>>,
        lifetime: ZwpPointerConstraintsV1Lifetime,
    ) -> Result<Rc<ZwpLockedPointerV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_lock_pointer(
            &id,
            surface,
            pointer,
            region,
            lifetime,
        )?;
        Ok(id)
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
    /// - `surface`: surface to lock pointer to
    /// - `pointer`: the pointer that should be locked
    /// - `region`: region of surface
    /// - `lifetime`: lock lifetime
    #[inline]
    pub fn new_send_lock_pointer(
        &self,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        region: Option<&Rc<WlRegion>>,
        lifetime: ZwpPointerConstraintsV1Lifetime,
    ) -> Rc<ZwpLockedPointerV1> {
        let id = self.core.create_child();
        self.send_lock_pointer(
            &id,
            surface,
            pointer,
            region,
            lifetime,
        );
        id
    }

    /// Since when the confine_pointer message is available.
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
    pub fn try_send_confine_pointer(
        &self,
        id: &Rc<ZwpConfinedPointerV1>,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        region: Option<&Rc<WlRegion>>,
        lifetime: ZwpPointerConstraintsV1Lifetime,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("pointer"))),
            Some(id) => id,
        };
        let arg3_id = match arg3 {
            None => 0,
            Some(arg3) => match arg3.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("region"))),
                Some(id) => id,
            },
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: ZwpPointerConstraintsV1Lifetime) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_pointer_constraints_v1#{}.confine_pointer(id: zwp_confined_pointer_v1#{}, surface: wl_surface#{}, pointer: wl_pointer#{}, region: wl_region#{}, lifetime: {:?})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2_id, arg3_id, arg4);
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
            2,
            arg0_id,
            arg1_id,
            arg2_id,
            arg3_id,
            arg4.0,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_confine_pointer(
        &self,
        id: &Rc<ZwpConfinedPointerV1>,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        region: Option<&Rc<WlRegion>>,
        lifetime: ZwpPointerConstraintsV1Lifetime,
    ) {
        let res = self.try_send_confine_pointer(
            id,
            surface,
            pointer,
            region,
            lifetime,
        );
        if let Err(e) = res {
            log_send("zwp_pointer_constraints_v1.confine_pointer", &e);
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
    /// - `surface`: surface to lock pointer to
    /// - `pointer`: the pointer that should be confined
    /// - `region`: region of surface
    /// - `lifetime`: confinement lifetime
    #[inline]
    pub fn new_try_send_confine_pointer(
        &self,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        region: Option<&Rc<WlRegion>>,
        lifetime: ZwpPointerConstraintsV1Lifetime,
    ) -> Result<Rc<ZwpConfinedPointerV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_confine_pointer(
            &id,
            surface,
            pointer,
            region,
            lifetime,
        )?;
        Ok(id)
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
    /// - `surface`: surface to lock pointer to
    /// - `pointer`: the pointer that should be confined
    /// - `region`: region of surface
    /// - `lifetime`: confinement lifetime
    #[inline]
    pub fn new_send_confine_pointer(
        &self,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        region: Option<&Rc<WlRegion>>,
        lifetime: ZwpPointerConstraintsV1Lifetime,
    ) -> Rc<ZwpConfinedPointerV1> {
        let id = self.core.create_child();
        self.send_confine_pointer(
            &id,
            surface,
            pointer,
            region,
            lifetime,
        );
        id
    }
}

/// A message handler for [`ZwpPointerConstraintsV1`] proxies.
pub trait ZwpPointerConstraintsV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwpPointerConstraintsV1>) {
        slf.core.delete_id();
    }

    /// destroy the pointer constraints manager object
    ///
    /// Used by the client to notify the server that it will no longer use this
    /// pointer constraints object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwpPointerConstraintsV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwp_pointer_constraints_v1.destroy", &e);
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
    fn handle_lock_pointer(
        &mut self,
        slf: &Rc<ZwpPointerConstraintsV1>,
        id: &Rc<ZwpLockedPointerV1>,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        region: Option<&Rc<WlRegion>>,
        lifetime: ZwpPointerConstraintsV1Lifetime,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_lock_pointer(
            id,
            surface,
            pointer,
            region,
            lifetime,
        );
        if let Err(e) = res {
            log_forward("zwp_pointer_constraints_v1.lock_pointer", &e);
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
    fn handle_confine_pointer(
        &mut self,
        slf: &Rc<ZwpPointerConstraintsV1>,
        id: &Rc<ZwpConfinedPointerV1>,
        surface: &Rc<WlSurface>,
        pointer: &Rc<WlPointer>,
        region: Option<&Rc<WlRegion>>,
        lifetime: ZwpPointerConstraintsV1Lifetime,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_confine_pointer(
            id,
            surface,
            pointer,
            region,
            lifetime,
        );
        if let Err(e) = res {
            log_forward("zwp_pointer_constraints_v1.confine_pointer", &e);
        }
    }
}

impl ObjectPrivate for ZwpPointerConstraintsV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpPointerConstraintsV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_pointer_constraints_v1#{}.destroy()\n", client_id, id);
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
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let arg4 = ZwpPointerConstraintsV1Lifetime(arg4);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: ZwpPointerConstraintsV1Lifetime) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_pointer_constraints_v1#{}.lock_pointer(id: zwp_locked_pointer_v1#{}, surface: wl_surface#{}, pointer: wl_pointer#{}, region: wl_region#{}, lifetime: {:?})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                let arg0_id = arg0;
                let arg0 = ZwpLockedPointerV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlPointer>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("pointer", o.core().interface, ObjectInterface::WlPointer)));
                };
                let arg3 = if arg3 == 0 {
                    None
                } else {
                    let arg3_id = arg3;
                    let Some(arg3) = client.endpoint.lookup(arg3_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg3_id)));
                    };
                    let Ok(arg3) = (arg3 as Rc<dyn Any>).downcast::<WlRegion>() else {
                        let o = client.endpoint.lookup(arg3_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("region", o.core().interface, ObjectInterface::WlRegion)));
                    };
                    Some(arg3)
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                let arg3 = arg3.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_lock_pointer(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_lock_pointer(&self, arg0, arg1, arg2, arg3, arg4);
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 28)));
                };
                let arg4 = ZwpPointerConstraintsV1Lifetime(arg4);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: ZwpPointerConstraintsV1Lifetime) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_pointer_constraints_v1#{}.confine_pointer(id: zwp_confined_pointer_v1#{}, surface: wl_surface#{}, pointer: wl_pointer#{}, region: wl_region#{}, lifetime: {:?})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                let arg0_id = arg0;
                let arg0 = ZwpConfinedPointerV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlPointer>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("pointer", o.core().interface, ObjectInterface::WlPointer)));
                };
                let arg3 = if arg3 == 0 {
                    None
                } else {
                    let arg3_id = arg3;
                    let Some(arg3) = client.endpoint.lookup(arg3_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg3_id)));
                    };
                    let Ok(arg3) = (arg3 as Rc<dyn Any>).downcast::<WlRegion>() else {
                        let o = client.endpoint.lookup(arg3_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("region", o.core().interface, ObjectInterface::WlRegion)));
                    };
                    Some(arg3)
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                let arg3 = arg3.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_confine_pointer(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_confine_pointer(&self, arg0, arg1, arg2, arg3, arg4);
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
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "lock_pointer",
            2 => "confine_pointer",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZwpPointerConstraintsV1 {
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

impl ZwpPointerConstraintsV1 {
    /// Since when the error.already_constrained enum variant is available.
    pub const ENM__ERROR_ALREADY_CONSTRAINED__SINCE: u32 = 1;

    /// Since when the lifetime.oneshot enum variant is available.
    pub const ENM__LIFETIME_ONESHOT__SINCE: u32 = 1;
    /// Since when the lifetime.persistent enum variant is available.
    pub const ENM__LIFETIME_PERSISTENT__SINCE: u32 = 1;
}

/// wp_pointer_constraints error values
///
/// These errors can be emitted in response to wp_pointer_constraints
/// requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpPointerConstraintsV1Error(pub u32);

impl ZwpPointerConstraintsV1Error {
    /// pointer constraint already requested on that surface
    pub const ALREADY_CONSTRAINED: Self = Self(1);
}

impl Debug for ZwpPointerConstraintsV1Error {
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
pub struct ZwpPointerConstraintsV1Lifetime(pub u32);

impl ZwpPointerConstraintsV1Lifetime {
    /// the pointer constraint is defunct once deactivated
    ///
    /// A oneshot pointer constraint will never reactivate once it has been
    /// deactivated. See the corresponding deactivation event
    /// (wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for
    /// details.
    pub const ONESHOT: Self = Self(1);

    /// the pointer constraint may reactivate
    ///
    /// A persistent pointer constraint may again reactivate once it has
    /// been deactivated. See the corresponding deactivation event
    /// (wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for
    /// details.
    pub const PERSISTENT: Self = Self(2);
}

impl Debug for ZwpPointerConstraintsV1Lifetime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ONESHOT => "ONESHOT",
            Self::PERSISTENT => "PERSISTENT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
