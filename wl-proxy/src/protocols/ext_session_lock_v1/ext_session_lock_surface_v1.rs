//! a surface displayed while the session is locked
//!
//! The client may use lock surfaces to display a screensaver, render a
//! dialog to enter a password and unlock the session, or however else it
//! sees fit.
//!
//! On binding this interface the compositor will immediately send the
//! first configure event. After making the ack_configure request in
//! response to this event the client should attach and commit the first
//! buffer. Committing the surface before acking the first configure is a
//! protocol error. Committing the surface with a null buffer at any time
//! is a protocol error.
//!
//! The compositor is free to handle keyboard/pointer focus for lock
//! surfaces however it chooses. A reasonable way to do this would be to
//! give the first lock surface created keyboard focus and change keyboard
//! focus if the user clicks on other surfaces.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_session_lock_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtSessionLockSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtSessionLockSurfaceV1Handler>,
}

struct DefaultHandler;

impl ExtSessionLockSurfaceV1Handler for DefaultHandler { }

impl ConcreteObject for ExtSessionLockSurfaceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtSessionLockSurfaceV1;
    const INTERFACE_NAME: &str = "ext_session_lock_surface_v1";
}

impl ExtSessionLockSurfaceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtSessionLockSurfaceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtSessionLockSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtSessionLockSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtSessionLockSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtSessionLockSurfaceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the lock surface object
    ///
    /// This informs the compositor that the lock surface object will no
    /// longer be used.
    ///
    /// It is recommended for a lock client to destroy lock surfaces if
    /// their corresponding wl_output global is removed.
    ///
    /// If a lock surface on an active output is destroyed before the
    /// ext_session_lock_v1.unlock_and_destroy event is sent, the compositor
    /// must fall back to rendering a solid color.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_session_lock_surface_v1#{}.destroy()\n", id);
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

    /// destroy the lock surface object
    ///
    /// This informs the compositor that the lock surface object will no
    /// longer be used.
    ///
    /// It is recommended for a lock client to destroy lock surfaces if
    /// their corresponding wl_output global is removed.
    ///
    /// If a lock surface on an active output is destroyed before the
    /// ext_session_lock_v1.unlock_and_destroy event is sent, the compositor
    /// must fall back to rendering a solid color.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_session_lock_surface_v1.destroy", &e);
        }
    }

    /// Since when the ack_configure message is available.
    pub const MSG__ACK_CONFIGURE__SINCE: u32 = 1;

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the surface
    /// in response to the configure event, then the client must make an
    /// ack_configure request sometime before the commit request, passing
    /// along the serial of the configure event.
    ///
    /// If the client receives multiple configure events before it can
    /// respond to one, it only has to ack the last configure event.
    ///
    /// A client is not required to commit immediately after sending an
    /// ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing,
    /// but only the last request sent before a commit indicates which
    /// configure event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the configure event
    /// referenced by the given serial, as well as all older configure events
    /// sent on this object.
    ///
    /// It is a protocol error to issue multiple ack_configure requests
    /// referencing the same configure event or to issue an ack_configure
    /// request referencing a configure event older than the last configure
    /// event acked for a given lock surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from the configure event
    #[inline]
    pub fn try_send_ack_configure(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_session_lock_surface_v1#{}.ack_configure(serial: {})\n", id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the surface
    /// in response to the configure event, then the client must make an
    /// ack_configure request sometime before the commit request, passing
    /// along the serial of the configure event.
    ///
    /// If the client receives multiple configure events before it can
    /// respond to one, it only has to ack the last configure event.
    ///
    /// A client is not required to commit immediately after sending an
    /// ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing,
    /// but only the last request sent before a commit indicates which
    /// configure event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the configure event
    /// referenced by the given serial, as well as all older configure events
    /// sent on this object.
    ///
    /// It is a protocol error to issue multiple ack_configure requests
    /// referencing the same configure event or to issue an ack_configure
    /// request referencing a configure event older than the last configure
    /// event acked for a given lock surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from the configure event
    #[inline]
    pub fn send_ack_configure(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_ack_configure(
            serial,
        );
        if let Err(e) = res {
            log_send("ext_session_lock_surface_v1.ack_configure", &e);
        }
    }

    /// Since when the configure message is available.
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// the client should resize its surface
    ///
    /// This event is sent once on binding the interface and may be sent again
    /// at the compositor's discretion, for example if output geometry changes.
    ///
    /// The width and height are in surface-local coordinates and are exact
    /// requirements. Failing to match these surface dimensions in the next
    /// commit after acking a configure is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial for use in ack_configure
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn try_send_configure(
        &self,
        serial: u32,
        width: u32,
        height: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            serial,
            width,
            height,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_session_lock_surface_v1#{}.configure(serial: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2);
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
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// the client should resize its surface
    ///
    /// This event is sent once on binding the interface and may be sent again
    /// at the compositor's discretion, for example if output geometry changes.
    ///
    /// The width and height are in surface-local coordinates and are exact
    /// requirements. Failing to match these surface dimensions in the next
    /// commit after acking a configure is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial for use in ack_configure
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_configure(
        &self,
        serial: u32,
        width: u32,
        height: u32,
    ) {
        let res = self.try_send_configure(
            serial,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("ext_session_lock_surface_v1.configure", &e);
        }
    }
}

/// A message handler for [`ExtSessionLockSurfaceV1`] proxies.
pub trait ExtSessionLockSurfaceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtSessionLockSurfaceV1>) {
        slf.core.delete_id();
    }

    /// destroy the lock surface object
    ///
    /// This informs the compositor that the lock surface object will no
    /// longer be used.
    ///
    /// It is recommended for a lock client to destroy lock surfaces if
    /// their corresponding wl_output global is removed.
    ///
    /// If a lock surface on an active output is destroyed before the
    /// ext_session_lock_v1.unlock_and_destroy event is sent, the compositor
    /// must fall back to rendering a solid color.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtSessionLockSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_session_lock_surface_v1.destroy", &e);
        }
    }

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the surface
    /// in response to the configure event, then the client must make an
    /// ack_configure request sometime before the commit request, passing
    /// along the serial of the configure event.
    ///
    /// If the client receives multiple configure events before it can
    /// respond to one, it only has to ack the last configure event.
    ///
    /// A client is not required to commit immediately after sending an
    /// ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing,
    /// but only the last request sent before a commit indicates which
    /// configure event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the configure event
    /// referenced by the given serial, as well as all older configure events
    /// sent on this object.
    ///
    /// It is a protocol error to issue multiple ack_configure requests
    /// referencing the same configure event or to issue an ack_configure
    /// request referencing a configure event older than the last configure
    /// event acked for a given lock surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from the configure event
    #[inline]
    fn handle_ack_configure(
        &mut self,
        slf: &Rc<ExtSessionLockSurfaceV1>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_ack_configure(
            serial,
        );
        if let Err(e) = res {
            log_forward("ext_session_lock_surface_v1.ack_configure", &e);
        }
    }

    /// the client should resize its surface
    ///
    /// This event is sent once on binding the interface and may be sent again
    /// at the compositor's discretion, for example if output geometry changes.
    ///
    /// The width and height are in surface-local coordinates and are exact
    /// requirements. Failing to match these surface dimensions in the next
    /// commit after acking a configure is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial for use in ack_configure
    /// - `width`:
    /// - `height`:
    #[inline]
    fn handle_configure(
        &mut self,
        slf: &Rc<ExtSessionLockSurfaceV1>,
        serial: u32,
        width: u32,
        height: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure(
            serial,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("ext_session_lock_surface_v1.configure", &e);
        }
    }
}

impl ObjectPrivate for ExtSessionLockSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtSessionLockSurfaceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_session_lock_surface_v1#{}.destroy()\n", client_id, id);
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_session_lock_surface_v1#{}.ack_configure(serial: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ack_configure(&self, arg0);
                } else {
                    DefaultHandler.handle_ack_configure(&self, arg0);
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
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_session_lock_surface_v1#{}.configure(serial: {}, width: {}, height: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_configure(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_configure(&self, arg0, arg1, arg2);
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
            1 => "ack_configure",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "configure",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ExtSessionLockSurfaceV1 {
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

impl ExtSessionLockSurfaceV1 {
    /// Since when the error.commit_before_first_ack enum variant is available.
    pub const ENM__ERROR_COMMIT_BEFORE_FIRST_ACK__SINCE: u32 = 1;
    /// Since when the error.null_buffer enum variant is available.
    pub const ENM__ERROR_NULL_BUFFER__SINCE: u32 = 1;
    /// Since when the error.dimensions_mismatch enum variant is available.
    pub const ENM__ERROR_DIMENSIONS_MISMATCH__SINCE: u32 = 1;
    /// Since when the error.invalid_serial enum variant is available.
    pub const ENM__ERROR_INVALID_SERIAL__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtSessionLockSurfaceV1Error(pub u32);

impl ExtSessionLockSurfaceV1Error {
    /// surface committed before first ack_configure request
    pub const COMMIT_BEFORE_FIRST_ACK: Self = Self(0);

    /// surface committed with a null buffer
    pub const NULL_BUFFER: Self = Self(1);

    /// failed to match ack'd width/height
    pub const DIMENSIONS_MISMATCH: Self = Self(2);

    /// serial provided in ack_configure is invalid
    pub const INVALID_SERIAL: Self = Self(3);
}

impl Debug for ExtSessionLockSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::COMMIT_BEFORE_FIRST_ACK => "COMMIT_BEFORE_FIRST_ACK",
            Self::NULL_BUFFER => "NULL_BUFFER",
            Self::DIMENSIONS_MISMATCH => "DIMENSIONS_MISMATCH",
            Self::INVALID_SERIAL => "INVALID_SERIAL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
