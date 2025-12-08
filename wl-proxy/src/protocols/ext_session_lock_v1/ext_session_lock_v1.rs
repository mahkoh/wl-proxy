//! manage lock state and create lock surfaces
//!
//! In response to the creation of this object the compositor must send
//! either the locked or finished event.
//!
//! The locked event indicates that the session is locked. This means
//! that the compositor must stop rendering and providing input to normal
//! clients. Instead the compositor must blank all outputs with an opaque
//! color such that their normal content is fully hidden.
//!
//! The only surfaces that should be rendered while the session is locked
//! are the lock surfaces created through this interface and optionally,
//! at the compositor's discretion, special privileged surfaces such as
//! input methods or portions of desktop shell UIs.
//!
//! The locked event must not be sent until a new "locked" frame (either
//! from a session lock surface or the compositor blanking the output) has
//! been presented on all outputs and no security sensitive normal/unlocked
//! content is possibly visible.
//!
//! The finished event should be sent immediately on creation of this
//! object if the compositor decides that the locked event will not be sent.
//!
//! The compositor may wait for the client to create and render session lock
//! surfaces before sending the locked event to avoid displaying intermediate
//! blank frames. However, it must impose a reasonable time limit if
//! waiting and send the locked event as soon as the hard requirements
//! described above can be met if the time limit expires. Clients should
//! immediately create lock surfaces for all outputs on creation of this
//! object to make this possible.
//!
//! This behavior of the locked event is required in order to prevent
//! possible race conditions with clients that wish to suspend the system
//! or similar after locking the session. Without these semantics, clients
//! triggering a suspend after receiving the locked event would race with
//! the first "locked" frame being presented and normal/unlocked frames
//! might be briefly visible as the system is resumed if the suspend
//! operation wins the race.
//!
//! If the client dies while the session is locked, the compositor must not
//! unlock the session in response. It is acceptable for the session to be
//! permanently locked if this happens. The compositor may choose to continue
//! to display the lock surfaces the client had mapped before it died or
//! alternatively fall back to a solid color, this is compositor policy.
//!
//! Compositors may also allow a secure way to recover the session, the
//! details of this are compositor policy. Compositors may allow a new
//! client to create a ext_session_lock_v1 object and take responsibility
//! for unlocking the session, they may even start a new lock client
//! instance automatically.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_session_lock_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtSessionLockV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtSessionLockV1Handler>,
}

struct DefaultHandler;

impl ExtSessionLockV1Handler for DefaultHandler { }

impl ConcreteObject for ExtSessionLockV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtSessionLockV1;
    const INTERFACE_NAME: &str = "ext_session_lock_v1";
}

impl ExtSessionLockV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtSessionLockV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtSessionLockV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtSessionLockV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtSessionLockV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtSessionLockV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the session lock
    ///
    /// This informs the compositor that the lock object will no longer be
    /// used. Existing objects created through this interface remain valid.
    ///
    /// After this request is made, lock surfaces created through this object
    /// should be destroyed by the client as they will no longer be used by
    /// the compositor.
    ///
    /// It is a protocol error to make this request if the locked event was
    /// sent, the unlock_and_destroy request must be used instead.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_session_lock_v1#{}.destroy()\n", id);
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

    /// destroy the session lock
    ///
    /// This informs the compositor that the lock object will no longer be
    /// used. Existing objects created through this interface remain valid.
    ///
    /// After this request is made, lock surfaces created through this object
    /// should be destroyed by the client as they will no longer be used by
    /// the compositor.
    ///
    /// It is a protocol error to make this request if the locked event was
    /// sent, the unlock_and_destroy request must be used instead.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_session_lock_v1.destroy", &e);
        }
    }

    /// Since when the locked message is available.
    pub const MSG__LOCKED__SINCE: u32 = 1;

    /// session successfully locked
    ///
    /// This client is now responsible for displaying graphics while the
    /// session is locked and deciding when to unlock the session.
    ///
    /// The locked event must not be sent until a new "locked" frame has been
    /// presented on all outputs and no security sensitive normal/unlocked
    /// content is possibly visible.
    ///
    /// If this event is sent, making the destroy request is a protocol error,
    /// the lock object must be destroyed using the unlock_and_destroy request.
    #[inline]
    pub fn try_send_locked(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_session_lock_v1#{}.locked()\n", client_id, id);
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

    /// session successfully locked
    ///
    /// This client is now responsible for displaying graphics while the
    /// session is locked and deciding when to unlock the session.
    ///
    /// The locked event must not be sent until a new "locked" frame has been
    /// presented on all outputs and no security sensitive normal/unlocked
    /// content is possibly visible.
    ///
    /// If this event is sent, making the destroy request is a protocol error,
    /// the lock object must be destroyed using the unlock_and_destroy request.
    #[inline]
    pub fn send_locked(
        &self,
    ) {
        let res = self.try_send_locked(
        );
        if let Err(e) = res {
            log_send("ext_session_lock_v1.locked", &e);
        }
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the session lock object should be destroyed
    ///
    /// The compositor has decided that the session lock should be destroyed
    /// as it will no longer be used by the compositor. Exactly when this
    /// event is sent is compositor policy, but it must never be sent more
    /// than once for a given session lock object.
    ///
    /// This might be sent because there is already another ext_session_lock_v1
    /// object held by a client, or the compositor has decided to deny the
    /// request to lock the session for some other reason. This might also
    /// be sent because the compositor implements some alternative, secure
    /// way to authenticate and unlock the session.
    ///
    /// The finished event should be sent immediately on creation of this
    /// object if the compositor decides that the locked event will not
    /// be sent.
    ///
    /// If the locked event is sent on creation of this object the finished
    /// event may still be sent at some later time in this object's
    /// lifetime. This is compositor policy.
    ///
    /// Upon receiving this event, the client should make either the destroy
    /// request or the unlock_and_destroy request, depending on whether or
    /// not the locked event was received on this object.
    #[inline]
    pub fn try_send_finished(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_session_lock_v1#{}.finished()\n", client_id, id);
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
            1,
        ]);
        Ok(())
    }

    /// the session lock object should be destroyed
    ///
    /// The compositor has decided that the session lock should be destroyed
    /// as it will no longer be used by the compositor. Exactly when this
    /// event is sent is compositor policy, but it must never be sent more
    /// than once for a given session lock object.
    ///
    /// This might be sent because there is already another ext_session_lock_v1
    /// object held by a client, or the compositor has decided to deny the
    /// request to lock the session for some other reason. This might also
    /// be sent because the compositor implements some alternative, secure
    /// way to authenticate and unlock the session.
    ///
    /// The finished event should be sent immediately on creation of this
    /// object if the compositor decides that the locked event will not
    /// be sent.
    ///
    /// If the locked event is sent on creation of this object the finished
    /// event may still be sent at some later time in this object's
    /// lifetime. This is compositor policy.
    ///
    /// Upon receiving this event, the client should make either the destroy
    /// request or the unlock_and_destroy request, depending on whether or
    /// not the locked event was received on this object.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("ext_session_lock_v1.finished", &e);
        }
    }

    /// Since when the get_lock_surface message is available.
    pub const MSG__GET_LOCK_SURFACE__SINCE: u32 = 1;

    /// create a lock surface for a given output
    ///
    /// The client is expected to create lock surfaces for all outputs
    /// currently present and any new outputs as they are advertised. These
    /// won't be displayed by the compositor unless the lock is successful
    /// and the locked event is sent.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error, as is attaching/committing
    /// a buffer before the first ext_session_lock_surface_v1.configure event.
    ///
    /// Attempting to create more than one lock surface for a given output
    /// is a duplicate_output protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    /// - `output`:
    #[inline]
    pub fn try_send_get_lock_surface(
        &self,
        id: &Rc<ExtSessionLockSurfaceV1>,
        surface: &Rc<WlSurface>,
        output: &Rc<WlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            surface,
            output,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_session_lock_v1#{}.get_lock_surface(id: ext_session_lock_surface_v1#{}, surface: wl_surface#{}, output: wl_output#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2_id);
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
        ]);
        Ok(())
    }

    /// create a lock surface for a given output
    ///
    /// The client is expected to create lock surfaces for all outputs
    /// currently present and any new outputs as they are advertised. These
    /// won't be displayed by the compositor unless the lock is successful
    /// and the locked event is sent.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error, as is attaching/committing
    /// a buffer before the first ext_session_lock_surface_v1.configure event.
    ///
    /// Attempting to create more than one lock surface for a given output
    /// is a duplicate_output protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    /// - `output`:
    #[inline]
    pub fn send_get_lock_surface(
        &self,
        id: &Rc<ExtSessionLockSurfaceV1>,
        surface: &Rc<WlSurface>,
        output: &Rc<WlOutput>,
    ) {
        let res = self.try_send_get_lock_surface(
            id,
            surface,
            output,
        );
        if let Err(e) = res {
            log_send("ext_session_lock_v1.get_lock_surface", &e);
        }
    }

    /// create a lock surface for a given output
    ///
    /// The client is expected to create lock surfaces for all outputs
    /// currently present and any new outputs as they are advertised. These
    /// won't be displayed by the compositor unless the lock is successful
    /// and the locked event is sent.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error, as is attaching/committing
    /// a buffer before the first ext_session_lock_surface_v1.configure event.
    ///
    /// Attempting to create more than one lock surface for a given output
    /// is a duplicate_output protocol error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    /// - `output`:
    #[inline]
    pub fn new_try_send_get_lock_surface(
        &self,
        surface: &Rc<WlSurface>,
        output: &Rc<WlOutput>,
    ) -> Result<Rc<ExtSessionLockSurfaceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_lock_surface(
            &id,
            surface,
            output,
        )?;
        Ok(id)
    }

    /// create a lock surface for a given output
    ///
    /// The client is expected to create lock surfaces for all outputs
    /// currently present and any new outputs as they are advertised. These
    /// won't be displayed by the compositor unless the lock is successful
    /// and the locked event is sent.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error, as is attaching/committing
    /// a buffer before the first ext_session_lock_surface_v1.configure event.
    ///
    /// Attempting to create more than one lock surface for a given output
    /// is a duplicate_output protocol error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    /// - `output`:
    #[inline]
    pub fn new_send_get_lock_surface(
        &self,
        surface: &Rc<WlSurface>,
        output: &Rc<WlOutput>,
    ) -> Rc<ExtSessionLockSurfaceV1> {
        let id = self.core.create_child();
        self.send_get_lock_surface(
            &id,
            surface,
            output,
        );
        id
    }

    /// Since when the unlock_and_destroy message is available.
    pub const MSG__UNLOCK_AND_DESTROY__SINCE: u32 = 1;

    /// unlock the session, destroying the object
    ///
    /// This request indicates that the session should be unlocked, for
    /// example because the user has entered their password and it has been
    /// verified by the client.
    ///
    /// This request also informs the compositor that the lock object will
    /// no longer be used and should be destroyed. Existing objects created
    /// through this interface remain valid.
    ///
    /// After this request is made, lock surfaces created through this object
    /// should be destroyed by the client as they will no longer be used by
    /// the compositor.
    ///
    /// It is a protocol error to make this request if the locked event has
    /// not been sent. In that case, the lock object must be destroyed using
    /// the destroy request.
    ///
    /// Note that a correct client that wishes to exit directly after unlocking
    /// the session must use the wl_display.sync request to ensure the server
    /// receives and processes the unlock_and_destroy request. Otherwise
    /// there is no guarantee that the server has unlocked the session due
    /// to the asynchronous nature of the Wayland protocol. For example,
    /// the server might terminate the client with a protocol error before
    /// it processes the unlock_and_destroy request.
    #[inline]
    pub fn try_send_unlock_and_destroy(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_session_lock_v1#{}.unlock_and_destroy()\n", id);
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// unlock the session, destroying the object
    ///
    /// This request indicates that the session should be unlocked, for
    /// example because the user has entered their password and it has been
    /// verified by the client.
    ///
    /// This request also informs the compositor that the lock object will
    /// no longer be used and should be destroyed. Existing objects created
    /// through this interface remain valid.
    ///
    /// After this request is made, lock surfaces created through this object
    /// should be destroyed by the client as they will no longer be used by
    /// the compositor.
    ///
    /// It is a protocol error to make this request if the locked event has
    /// not been sent. In that case, the lock object must be destroyed using
    /// the destroy request.
    ///
    /// Note that a correct client that wishes to exit directly after unlocking
    /// the session must use the wl_display.sync request to ensure the server
    /// receives and processes the unlock_and_destroy request. Otherwise
    /// there is no guarantee that the server has unlocked the session due
    /// to the asynchronous nature of the Wayland protocol. For example,
    /// the server might terminate the client with a protocol error before
    /// it processes the unlock_and_destroy request.
    #[inline]
    pub fn send_unlock_and_destroy(
        &self,
    ) {
        let res = self.try_send_unlock_and_destroy(
        );
        if let Err(e) = res {
            log_send("ext_session_lock_v1.unlock_and_destroy", &e);
        }
    }
}

/// A message handler for [`ExtSessionLockV1`] proxies.
pub trait ExtSessionLockV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtSessionLockV1>) {
        slf.core.delete_id();
    }

    /// destroy the session lock
    ///
    /// This informs the compositor that the lock object will no longer be
    /// used. Existing objects created through this interface remain valid.
    ///
    /// After this request is made, lock surfaces created through this object
    /// should be destroyed by the client as they will no longer be used by
    /// the compositor.
    ///
    /// It is a protocol error to make this request if the locked event was
    /// sent, the unlock_and_destroy request must be used instead.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtSessionLockV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_session_lock_v1.destroy", &e);
        }
    }

    /// session successfully locked
    ///
    /// This client is now responsible for displaying graphics while the
    /// session is locked and deciding when to unlock the session.
    ///
    /// The locked event must not be sent until a new "locked" frame has been
    /// presented on all outputs and no security sensitive normal/unlocked
    /// content is possibly visible.
    ///
    /// If this event is sent, making the destroy request is a protocol error,
    /// the lock object must be destroyed using the unlock_and_destroy request.
    #[inline]
    fn handle_locked(
        &mut self,
        slf: &Rc<ExtSessionLockV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_locked(
        );
        if let Err(e) = res {
            log_forward("ext_session_lock_v1.locked", &e);
        }
    }

    /// the session lock object should be destroyed
    ///
    /// The compositor has decided that the session lock should be destroyed
    /// as it will no longer be used by the compositor. Exactly when this
    /// event is sent is compositor policy, but it must never be sent more
    /// than once for a given session lock object.
    ///
    /// This might be sent because there is already another ext_session_lock_v1
    /// object held by a client, or the compositor has decided to deny the
    /// request to lock the session for some other reason. This might also
    /// be sent because the compositor implements some alternative, secure
    /// way to authenticate and unlock the session.
    ///
    /// The finished event should be sent immediately on creation of this
    /// object if the compositor decides that the locked event will not
    /// be sent.
    ///
    /// If the locked event is sent on creation of this object the finished
    /// event may still be sent at some later time in this object's
    /// lifetime. This is compositor policy.
    ///
    /// Upon receiving this event, the client should make either the destroy
    /// request or the unlock_and_destroy request, depending on whether or
    /// not the locked event was received on this object.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<ExtSessionLockV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("ext_session_lock_v1.finished", &e);
        }
    }

    /// create a lock surface for a given output
    ///
    /// The client is expected to create lock surfaces for all outputs
    /// currently present and any new outputs as they are advertised. These
    /// won't be displayed by the compositor unless the lock is successful
    /// and the locked event is sent.
    ///
    /// Providing a wl_surface which already has a role or already has a buffer
    /// attached or committed is a protocol error, as is attaching/committing
    /// a buffer before the first ext_session_lock_surface_v1.configure event.
    ///
    /// Attempting to create more than one lock surface for a given output
    /// is a duplicate_output protocol error.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_lock_surface(
        &mut self,
        slf: &Rc<ExtSessionLockV1>,
        id: &Rc<ExtSessionLockSurfaceV1>,
        surface: &Rc<WlSurface>,
        output: &Rc<WlOutput>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_lock_surface(
            id,
            surface,
            output,
        );
        if let Err(e) = res {
            log_forward("ext_session_lock_v1.get_lock_surface", &e);
        }
    }

    /// unlock the session, destroying the object
    ///
    /// This request indicates that the session should be unlocked, for
    /// example because the user has entered their password and it has been
    /// verified by the client.
    ///
    /// This request also informs the compositor that the lock object will
    /// no longer be used and should be destroyed. Existing objects created
    /// through this interface remain valid.
    ///
    /// After this request is made, lock surfaces created through this object
    /// should be destroyed by the client as they will no longer be used by
    /// the compositor.
    ///
    /// It is a protocol error to make this request if the locked event has
    /// not been sent. In that case, the lock object must be destroyed using
    /// the destroy request.
    ///
    /// Note that a correct client that wishes to exit directly after unlocking
    /// the session must use the wl_display.sync request to ensure the server
    /// receives and processes the unlock_and_destroy request. Otherwise
    /// there is no guarantee that the server has unlocked the session due
    /// to the asynchronous nature of the Wayland protocol. For example,
    /// the server might terminate the client with a protocol error before
    /// it processes the unlock_and_destroy request.
    #[inline]
    fn handle_unlock_and_destroy(
        &mut self,
        slf: &Rc<ExtSessionLockV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unlock_and_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_session_lock_v1.unlock_and_destroy", &e);
        }
    }
}

impl ObjectPrivate for ExtSessionLockV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtSessionLockV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_session_lock_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_session_lock_v1#{}.get_lock_surface(id: ext_session_lock_surface_v1#{}, surface: wl_surface#{}, output: wl_output#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = ExtSessionLockSurfaceV1::new(&self.core.state, self.core.version);
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
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_get_lock_surface(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_get_lock_surface(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_session_lock_v1#{}.unlock_and_destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_unlock_and_destroy(&self);
                } else {
                    DefaultHandler.handle_unlock_and_destroy(&self);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_session_lock_v1#{}.locked()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_locked(&self);
                } else {
                    DefaultHandler.handle_locked(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_session_lock_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
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
            1 => "get_lock_surface",
            2 => "unlock_and_destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "locked",
            1 => "finished",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ExtSessionLockV1 {
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

impl ExtSessionLockV1 {
    /// Since when the error.invalid_destroy enum variant is available.
    pub const ENM__ERROR_INVALID_DESTROY__SINCE: u32 = 1;
    /// Since when the error.invalid_unlock enum variant is available.
    pub const ENM__ERROR_INVALID_UNLOCK__SINCE: u32 = 1;
    /// Since when the error.role enum variant is available.
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.duplicate_output enum variant is available.
    pub const ENM__ERROR_DUPLICATE_OUTPUT__SINCE: u32 = 1;
    /// Since when the error.already_constructed enum variant is available.
    pub const ENM__ERROR_ALREADY_CONSTRUCTED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtSessionLockV1Error(pub u32);

impl ExtSessionLockV1Error {
    /// attempted to destroy session lock while locked
    pub const INVALID_DESTROY: Self = Self(0);

    /// unlock requested but locked event was never sent
    pub const INVALID_UNLOCK: Self = Self(1);

    /// given wl_surface already has a role
    pub const ROLE: Self = Self(2);

    /// given output already has a lock surface
    pub const DUPLICATE_OUTPUT: Self = Self(3);

    /// given wl_surface has a buffer attached or committed
    pub const ALREADY_CONSTRUCTED: Self = Self(4);
}

impl Debug for ExtSessionLockV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_DESTROY => "INVALID_DESTROY",
            Self::INVALID_UNLOCK => "INVALID_UNLOCK",
            Self::ROLE => "ROLE",
            Self::DUPLICATE_OUTPUT => "DUPLICATE_OUTPUT",
            Self::ALREADY_CONSTRUCTED => "ALREADY_CONSTRUCTED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
