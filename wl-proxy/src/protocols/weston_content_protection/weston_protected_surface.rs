//! content protection interface to a wl_surface
//!
//! An additional interface to a wl_surface object, which allows a client to
//! request the minimum level of content-protection, request to change the
//! visibility of their contents, and receive notifications about changes in
//! content-protection.
//!
//! A protected surface has a 'status' associated with it, that indicates
//! what type of protection it is currently providing, specified by
//! content-type. Updates to this status are sent to the client
//! via the 'status' event. Before the first status event is sent, the client
//! should assume that the status is 'unprotected'.
//!
//! A client can request a content protection level to be the minimum for an
//! output to be considered secure, using the 'set_type' request.
//! It is responsibility of the client to monitor the actual
//! content-protection level achieved via the 'status' event, and make
//! decisions as to what content to show based on this.
//!
//! The server should make its best effort to achieve the desired
//! content-protection level on all of the outputs the client's contents are
//! being displayed on. Any changes to the content protection status should be
//! reported to the client, even if they are below the requested
//! content-protection level. If the client's contents are being displayed on
//! multiple outputs, the lowest content protection level achieved should be
//! reported.
//!
//! A client can also request that its content only be displayed on outputs
//! that are considered secure. The 'enforce/relax' requests can achieve this.
//! In enforce mode, the content is censored for non-secure outputs.
//! The implementation of censored-visibility is compositor-defined.
//! In relax mode there are no such limitation. On an attempt to show the
//! client on unsecured output, compositor would keep on showing the content
//! and send the 'status' event to the client. Client can take a call to
//! downgrade the content.
//!
//! If the wl_surface associated with the protected_surface is destroyed,
//! the protected_surface becomes inert.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_protected_surface object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonProtectedSurface {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonProtectedSurfaceHandler>,
}

struct DefaultHandler;

impl WestonProtectedSurfaceHandler for DefaultHandler { }

impl ConcreteObject for WestonProtectedSurface {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonProtectedSurface;
    const INTERFACE_NAME: &str = "weston_protected_surface";
}

impl WestonProtectedSurface {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonProtectedSurfaceHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonProtectedSurfaceHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonProtectedSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonProtectedSurface")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonProtectedSurface {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// remove security from the surface
    ///
    /// If the protected_surface is destroyed, the wl_surface desired protection
    /// level returns to unprotected, as if set_type request was sent with type
    /// as 'unprotected'.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_protected_surface#{}.destroy()\n", id);
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

    /// remove security from the surface
    ///
    /// If the protected_surface is destroyed, the wl_surface desired protection
    /// level returns to unprotected, as if set_type request was sent with type
    /// as 'unprotected'.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("weston_protected_surface.destroy", &e);
        }
    }

    /// Since when the set_type message is available.
    pub const MSG__SET_TYPE__SINCE: u32 = 1;

    /// set the acceptable level of content protection
    ///
    /// Informs the server about the type of content. The level of
    /// content-protection depends upon the content-type set by the client
    /// through this request. Initially, this is set to 'unprotected'.
    ///
    /// If the requested value is not a valid content_type enum value, the
    /// 'invalid_type' protocol error is raised. It is not an error to request
    /// a valid protection type the compositor does not implement or cannot
    /// achieve.
    ///
    /// The requested content protection is double-buffered, see
    /// wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `r#type`: the desired type of content protection
    #[inline]
    pub fn try_send_set_type(
        &self,
        r#type: WestonProtectedSurfaceType,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            r#type,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WestonProtectedSurfaceType) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_protected_surface#{}.set_type(type: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// set the acceptable level of content protection
    ///
    /// Informs the server about the type of content. The level of
    /// content-protection depends upon the content-type set by the client
    /// through this request. Initially, this is set to 'unprotected'.
    ///
    /// If the requested value is not a valid content_type enum value, the
    /// 'invalid_type' protocol error is raised. It is not an error to request
    /// a valid protection type the compositor does not implement or cannot
    /// achieve.
    ///
    /// The requested content protection is double-buffered, see
    /// wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `r#type`: the desired type of content protection
    #[inline]
    pub fn send_set_type(
        &self,
        r#type: WestonProtectedSurfaceType,
    ) {
        let res = self.try_send_set_type(
            r#type,
        );
        if let Err(e) = res {
            log_send("weston_protected_surface.set_type", &e);
        }
    }

    /// Since when the enforce message is available.
    pub const MSG__ENFORCE__SINCE: u32 = 1;

    /// enforce censored-visibility constrain
    ///
    /// Censor the visibility of the wl_surface contents on non-secure outputs.
    /// See weston_protected_surface for the description.
    ///
    /// The force constrain mode is double-buffered, see wl_surface.commit
    #[inline]
    pub fn try_send_enforce(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_protected_surface#{}.enforce()\n", id);
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
        Ok(())
    }

    /// enforce censored-visibility constrain
    ///
    /// Censor the visibility of the wl_surface contents on non-secure outputs.
    /// See weston_protected_surface for the description.
    ///
    /// The force constrain mode is double-buffered, see wl_surface.commit
    #[inline]
    pub fn send_enforce(
        &self,
    ) {
        let res = self.try_send_enforce(
        );
        if let Err(e) = res {
            log_send("weston_protected_surface.enforce", &e);
        }
    }

    /// Since when the relax message is available.
    pub const MSG__RELAX__SINCE: u32 = 1;

    /// relax the censored-visibility constrain
    ///
    /// Do not enforce censored-visibility of the wl_surface contents on
    /// non-secure-outputs. See weston_protected_surface for the description.
    ///
    /// The relax mode is selected by default, if no explicit request is made
    /// for enforcing the censored-visibility.
    ///
    /// The relax mode is double-buffered, see wl_surface.commit
    #[inline]
    pub fn try_send_relax(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_protected_surface#{}.relax()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// relax the censored-visibility constrain
    ///
    /// Do not enforce censored-visibility of the wl_surface contents on
    /// non-secure-outputs. See weston_protected_surface for the description.
    ///
    /// The relax mode is selected by default, if no explicit request is made
    /// for enforcing the censored-visibility.
    ///
    /// The relax mode is double-buffered, see wl_surface.commit
    #[inline]
    pub fn send_relax(
        &self,
    ) {
        let res = self.try_send_relax(
        );
        if let Err(e) = res {
            log_send("weston_protected_surface.relax", &e);
        }
    }

    /// Since when the status message is available.
    pub const MSG__STATUS__SINCE: u32 = 1;

    /// security status changed
    ///
    /// This event is sent to the client to inform about the actual protection
    /// level for its surface in the relax mode.
    ///
    /// The 'type' argument indicates what that current level of content
    /// protection that the server has currently established.
    ///
    /// The 'status' event is first sent, when a weston_protected_surface is
    /// created.
    ///
    /// Until this event is sent for the first time, the client should assume
    /// that its contents are not secure, and the type is 'unprotected'.
    ///
    /// Possible reasons the content protection status can change is due to
    /// change in censored-visibility mode from enforced to relaxed, a new
    /// connector being added, movement of window to another output, or,
    /// the client attaching a buffer too large for what the server may secure.
    /// However, it is not limited to these reasons.
    ///
    /// A client may want to listen to this event and lower the resolution of
    /// their content until it can successfully be shown securely.
    ///
    /// In case of "enforce" mode, the client will not get any status event.
    /// If the mode is then changed to "relax", the client will receive the
    /// status event.
    ///
    /// # Arguments
    ///
    /// - `r#type`: the current content protection level
    #[inline]
    pub fn try_send_status(
        &self,
        r#type: WestonProtectedSurfaceType,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            r#type,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: WestonProtectedSurfaceType) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_protected_surface#{}.status(type: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// security status changed
    ///
    /// This event is sent to the client to inform about the actual protection
    /// level for its surface in the relax mode.
    ///
    /// The 'type' argument indicates what that current level of content
    /// protection that the server has currently established.
    ///
    /// The 'status' event is first sent, when a weston_protected_surface is
    /// created.
    ///
    /// Until this event is sent for the first time, the client should assume
    /// that its contents are not secure, and the type is 'unprotected'.
    ///
    /// Possible reasons the content protection status can change is due to
    /// change in censored-visibility mode from enforced to relaxed, a new
    /// connector being added, movement of window to another output, or,
    /// the client attaching a buffer too large for what the server may secure.
    /// However, it is not limited to these reasons.
    ///
    /// A client may want to listen to this event and lower the resolution of
    /// their content until it can successfully be shown securely.
    ///
    /// In case of "enforce" mode, the client will not get any status event.
    /// If the mode is then changed to "relax", the client will receive the
    /// status event.
    ///
    /// # Arguments
    ///
    /// - `r#type`: the current content protection level
    #[inline]
    pub fn send_status(
        &self,
        r#type: WestonProtectedSurfaceType,
    ) {
        let res = self.try_send_status(
            r#type,
        );
        if let Err(e) = res {
            log_send("weston_protected_surface.status", &e);
        }
    }
}

/// A message handler for [`WestonProtectedSurface`] proxies.
pub trait WestonProtectedSurfaceHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonProtectedSurface>) {
        slf.core.delete_id();
    }

    /// remove security from the surface
    ///
    /// If the protected_surface is destroyed, the wl_surface desired protection
    /// level returns to unprotected, as if set_type request was sent with type
    /// as 'unprotected'.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WestonProtectedSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("weston_protected_surface.destroy", &e);
        }
    }

    /// set the acceptable level of content protection
    ///
    /// Informs the server about the type of content. The level of
    /// content-protection depends upon the content-type set by the client
    /// through this request. Initially, this is set to 'unprotected'.
    ///
    /// If the requested value is not a valid content_type enum value, the
    /// 'invalid_type' protocol error is raised. It is not an error to request
    /// a valid protection type the compositor does not implement or cannot
    /// achieve.
    ///
    /// The requested content protection is double-buffered, see
    /// wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `r#type`: the desired type of content protection
    #[inline]
    fn handle_set_type(
        &mut self,
        slf: &Rc<WestonProtectedSurface>,
        r#type: WestonProtectedSurfaceType,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_type(
            r#type,
        );
        if let Err(e) = res {
            log_forward("weston_protected_surface.set_type", &e);
        }
    }

    /// enforce censored-visibility constrain
    ///
    /// Censor the visibility of the wl_surface contents on non-secure outputs.
    /// See weston_protected_surface for the description.
    ///
    /// The force constrain mode is double-buffered, see wl_surface.commit
    #[inline]
    fn handle_enforce(
        &mut self,
        slf: &Rc<WestonProtectedSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_enforce(
        );
        if let Err(e) = res {
            log_forward("weston_protected_surface.enforce", &e);
        }
    }

    /// relax the censored-visibility constrain
    ///
    /// Do not enforce censored-visibility of the wl_surface contents on
    /// non-secure-outputs. See weston_protected_surface for the description.
    ///
    /// The relax mode is selected by default, if no explicit request is made
    /// for enforcing the censored-visibility.
    ///
    /// The relax mode is double-buffered, see wl_surface.commit
    #[inline]
    fn handle_relax(
        &mut self,
        slf: &Rc<WestonProtectedSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_relax(
        );
        if let Err(e) = res {
            log_forward("weston_protected_surface.relax", &e);
        }
    }

    /// security status changed
    ///
    /// This event is sent to the client to inform about the actual protection
    /// level for its surface in the relax mode.
    ///
    /// The 'type' argument indicates what that current level of content
    /// protection that the server has currently established.
    ///
    /// The 'status' event is first sent, when a weston_protected_surface is
    /// created.
    ///
    /// Until this event is sent for the first time, the client should assume
    /// that its contents are not secure, and the type is 'unprotected'.
    ///
    /// Possible reasons the content protection status can change is due to
    /// change in censored-visibility mode from enforced to relaxed, a new
    /// connector being added, movement of window to another output, or,
    /// the client attaching a buffer too large for what the server may secure.
    /// However, it is not limited to these reasons.
    ///
    /// A client may want to listen to this event and lower the resolution of
    /// their content until it can successfully be shown securely.
    ///
    /// In case of "enforce" mode, the client will not get any status event.
    /// If the mode is then changed to "relax", the client will receive the
    /// status event.
    ///
    /// # Arguments
    ///
    /// - `r#type`: the current content protection level
    #[inline]
    fn handle_status(
        &mut self,
        slf: &Rc<WestonProtectedSurface>,
        r#type: WestonProtectedSurfaceType,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_status(
            r#type,
        );
        if let Err(e) = res {
            log_forward("weston_protected_surface.status", &e);
        }
    }
}

impl ObjectPrivate for WestonProtectedSurface {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonProtectedSurface, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_protected_surface#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WestonProtectedSurfaceType(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WestonProtectedSurfaceType) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_protected_surface#{}.set_type(type: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_type(&self, arg0);
                } else {
                    DefaultHandler.handle_set_type(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_protected_surface#{}.enforce()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_enforce(&self);
                } else {
                    DefaultHandler.handle_enforce(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_protected_surface#{}.relax()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_relax(&self);
                } else {
                    DefaultHandler.handle_relax(&self);
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WestonProtectedSurfaceType(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WestonProtectedSurfaceType) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_protected_surface#{}.status(type: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_status(&self, arg0);
                } else {
                    DefaultHandler.handle_status(&self, arg0);
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
            1 => "set_type",
            2 => "enforce",
            3 => "relax",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "status",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WestonProtectedSurface {
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

impl WestonProtectedSurface {
    /// Since when the error.invalid_type enum variant is available.
    pub const ENM__ERROR_INVALID_TYPE__SINCE: u32 = 1;

    /// Since when the type.unprotected enum variant is available.
    pub const ENM__TYPE_UNPROTECTED__SINCE: u32 = 1;
    /// Since when the type.hdcp_0 enum variant is available.
    pub const ENM__TYPE_HDCP_0__SINCE: u32 = 1;
    /// Since when the type.hdcp_1 enum variant is available.
    pub const ENM__TYPE_HDCP_1__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonProtectedSurfaceError(pub u32);

impl WestonProtectedSurfaceError {
    /// provided type was not valid
    pub const INVALID_TYPE: Self = Self(0);
}

impl Debug for WestonProtectedSurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_TYPE => "INVALID_TYPE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// content types
///
/// Description of a particular type of content protection.
///
/// A server may not necessarily support all of these types.
///
/// Note that there is no ordering between enum members unless specified.
/// Over time, different types of content protection may be added, which
/// may be considered less secure than what is already here.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonProtectedSurfaceType(pub u32);

impl WestonProtectedSurfaceType {
    /// no protection required
    pub const UNPROTECTED: Self = Self(0);

    /// HDCP type 0
    pub const HDCP_0: Self = Self(1);

    /// HDCP type 1. This is a more secure than HDCP type 0.
    pub const HDCP_1: Self = Self(2);
}

impl Debug for WestonProtectedSurfaceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::UNPROTECTED => "UNPROTECTED",
            Self::HDCP_0 => "HDCP_0",
            Self::HDCP_1 => "HDCP_1",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
