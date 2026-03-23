//! manage sessions for applications
//!
//! The xdg_session_manager_v1 interface defines base requests for creating and
//! managing a session for an application. Sessions persist across application
//! and compositor restarts unless explicitly destroyed. A session is created
//! for the purpose of maintaining an application's xdg_toplevel surfaces
//! across compositor or application restarts. The compositor should remember
//! as many states as possible for surfaces in a given session, but there is
//! no requirement for which states must be remembered.
//!
//! Policies such as cache eviction are declared an implementation detail of
//! the compositor. Clients should account for no longer existing sessions.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_session_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgSessionManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgSessionManagerV1Handler>,
}

struct DefaultHandler;

impl XdgSessionManagerV1Handler for DefaultHandler { }

impl ConcreteObject for XdgSessionManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgSessionManagerV1;
    const INTERFACE_NAME: &str = "xdg_session_manager_v1";
}

impl XdgSessionManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgSessionManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgSessionManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgSessionManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgSessionManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgSessionManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// Destroy this object
    ///
    /// Destroy the manager object. The existing session objects will be
    /// unaffected.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_session_manager_v1#{}.destroy()\n", id);
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

    /// Destroy this object
    ///
    /// Destroy the manager object. The existing session objects will be
    /// unaffected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_session_manager_v1.destroy", &e);
        }
    }

    /// Since when the get_session message is available.
    pub const MSG__GET_SESSION__SINCE: u32 = 1;

    /// create or restore a session
    ///
    /// Create a session object corresponding to either an existing session
    /// identified by the given session identifier string or a new session.
    /// While the session object exists, the session is considered to be "in
    /// use".
    ///
    /// If an identifier string represents a session that is currently actively
    /// in use by the the same client, an 'in_use' error is raised. If some
    /// other client is currently using the same session, the new session will
    /// replace managing the associated state.
    ///
    /// NULL is passed to initiate a new session. If a session_id is passed
    /// which does not represent a valid session, the compositor treats it as if
    /// NULL had been passed.
    ///
    /// The session id string must be UTF-8 encoded. It is also limited by the
    /// maximum length of wayland messages (around 4KB). The 'invalid_session_id'
    /// protocol error will be raised if an invalid string is provided.
    ///
    /// A client is allowed to have any number of in use sessions at the same
    /// time.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `reason`: reason for session
    /// - `session_id`: the session to restore
    #[inline]
    pub fn try_send_get_session(
        &self,
        id: &Rc<XdgSessionV1>,
        reason: XdgSessionManagerV1Reason,
        session_id: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            reason,
            session_id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: XdgSessionManagerV1Reason, arg2: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_session_manager_v1#{}.get_session(id: xdg_session_v1#{}, reason: {:?}, session_id: {:?})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2);
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
            arg1.0,
        ]);
        if let Some(arg2) = arg2 {
            fmt.string(arg2);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// create or restore a session
    ///
    /// Create a session object corresponding to either an existing session
    /// identified by the given session identifier string or a new session.
    /// While the session object exists, the session is considered to be "in
    /// use".
    ///
    /// If an identifier string represents a session that is currently actively
    /// in use by the the same client, an 'in_use' error is raised. If some
    /// other client is currently using the same session, the new session will
    /// replace managing the associated state.
    ///
    /// NULL is passed to initiate a new session. If a session_id is passed
    /// which does not represent a valid session, the compositor treats it as if
    /// NULL had been passed.
    ///
    /// The session id string must be UTF-8 encoded. It is also limited by the
    /// maximum length of wayland messages (around 4KB). The 'invalid_session_id'
    /// protocol error will be raised if an invalid string is provided.
    ///
    /// A client is allowed to have any number of in use sessions at the same
    /// time.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `reason`: reason for session
    /// - `session_id`: the session to restore
    #[inline]
    pub fn send_get_session(
        &self,
        id: &Rc<XdgSessionV1>,
        reason: XdgSessionManagerV1Reason,
        session_id: Option<&str>,
    ) {
        let res = self.try_send_get_session(
            id,
            reason,
            session_id,
        );
        if let Err(e) = res {
            log_send("xdg_session_manager_v1.get_session", &e);
        }
    }

    /// create or restore a session
    ///
    /// Create a session object corresponding to either an existing session
    /// identified by the given session identifier string or a new session.
    /// While the session object exists, the session is considered to be "in
    /// use".
    ///
    /// If an identifier string represents a session that is currently actively
    /// in use by the the same client, an 'in_use' error is raised. If some
    /// other client is currently using the same session, the new session will
    /// replace managing the associated state.
    ///
    /// NULL is passed to initiate a new session. If a session_id is passed
    /// which does not represent a valid session, the compositor treats it as if
    /// NULL had been passed.
    ///
    /// The session id string must be UTF-8 encoded. It is also limited by the
    /// maximum length of wayland messages (around 4KB). The 'invalid_session_id'
    /// protocol error will be raised if an invalid string is provided.
    ///
    /// A client is allowed to have any number of in use sessions at the same
    /// time.
    ///
    /// # Arguments
    ///
    /// - `reason`: reason for session
    /// - `session_id`: the session to restore
    #[inline]
    pub fn new_try_send_get_session(
        &self,
        reason: XdgSessionManagerV1Reason,
        session_id: Option<&str>,
    ) -> Result<Rc<XdgSessionV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_session(
            &id,
            reason,
            session_id,
        )?;
        Ok(id)
    }

    /// create or restore a session
    ///
    /// Create a session object corresponding to either an existing session
    /// identified by the given session identifier string or a new session.
    /// While the session object exists, the session is considered to be "in
    /// use".
    ///
    /// If an identifier string represents a session that is currently actively
    /// in use by the the same client, an 'in_use' error is raised. If some
    /// other client is currently using the same session, the new session will
    /// replace managing the associated state.
    ///
    /// NULL is passed to initiate a new session. If a session_id is passed
    /// which does not represent a valid session, the compositor treats it as if
    /// NULL had been passed.
    ///
    /// The session id string must be UTF-8 encoded. It is also limited by the
    /// maximum length of wayland messages (around 4KB). The 'invalid_session_id'
    /// protocol error will be raised if an invalid string is provided.
    ///
    /// A client is allowed to have any number of in use sessions at the same
    /// time.
    ///
    /// # Arguments
    ///
    /// - `reason`: reason for session
    /// - `session_id`: the session to restore
    #[inline]
    pub fn new_send_get_session(
        &self,
        reason: XdgSessionManagerV1Reason,
        session_id: Option<&str>,
    ) -> Rc<XdgSessionV1> {
        let id = self.core.create_child();
        self.send_get_session(
            &id,
            reason,
            session_id,
        );
        id
    }
}

/// A message handler for [`XdgSessionManagerV1`] proxies.
pub trait XdgSessionManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgSessionManagerV1>) {
        slf.core.delete_id();
    }

    /// Destroy this object
    ///
    /// Destroy the manager object. The existing session objects will be
    /// unaffected.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgSessionManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_session_manager_v1.destroy", &e);
        }
    }

    /// create or restore a session
    ///
    /// Create a session object corresponding to either an existing session
    /// identified by the given session identifier string or a new session.
    /// While the session object exists, the session is considered to be "in
    /// use".
    ///
    /// If an identifier string represents a session that is currently actively
    /// in use by the the same client, an 'in_use' error is raised. If some
    /// other client is currently using the same session, the new session will
    /// replace managing the associated state.
    ///
    /// NULL is passed to initiate a new session. If a session_id is passed
    /// which does not represent a valid session, the compositor treats it as if
    /// NULL had been passed.
    ///
    /// The session id string must be UTF-8 encoded. It is also limited by the
    /// maximum length of wayland messages (around 4KB). The 'invalid_session_id'
    /// protocol error will be raised if an invalid string is provided.
    ///
    /// A client is allowed to have any number of in use sessions at the same
    /// time.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `reason`: reason for session
    /// - `session_id`: the session to restore
    #[inline]
    fn handle_get_session(
        &mut self,
        slf: &Rc<XdgSessionManagerV1>,
        id: &Rc<XdgSessionV1>,
        reason: XdgSessionManagerV1Reason,
        session_id: Option<&str>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_session(
            id,
            reason,
            session_id,
        );
        if let Err(e) = res {
            log_forward("xdg_session_manager_v1.get_session", &e);
        }
    }
}

impl ObjectPrivate for XdgSessionManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgSessionManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_session_manager_v1#{}.destroy()\n", client_id, id);
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("reason")));
                };
                offset += 1;
                let arg2;
                (arg2, offset) = parse_string::<NullableString>(msg, offset, "session_id")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                let arg1 = XdgSessionManagerV1Reason(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: XdgSessionManagerV1Reason, arg2: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_session_manager_v1#{}.get_session(id: xdg_session_v1#{}, reason: {:?}, session_id: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = XdgSessionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_session(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_get_session(&self, arg0, arg1, arg2);
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
            1 => "get_session",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for XdgSessionManagerV1 {
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

impl XdgSessionManagerV1 {
    /// Since when the error.in_use enum variant is available.
    pub const ENM__ERROR_IN_USE__SINCE: u32 = 1;
    /// Since when the error.invalid_session_id enum variant is available.
    pub const ENM__ERROR_INVALID_SESSION_ID__SINCE: u32 = 1;

    /// Since when the reason.launch enum variant is available.
    pub const ENM__REASON_LAUNCH__SINCE: u32 = 1;
    /// Since when the reason.recover enum variant is available.
    pub const ENM__REASON_RECOVER__SINCE: u32 = 1;
    /// Since when the reason.session_restore enum variant is available.
    pub const ENM__REASON_SESSION_RESTORE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgSessionManagerV1Error(pub u32);

impl XdgSessionManagerV1Error {
    /// a requested session is already in use
    pub const IN_USE: Self = Self(1);

    /// invalid session identifier
    pub const INVALID_SESSION_ID: Self = Self(2);
}

impl Debug for XdgSessionManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::IN_USE => "IN_USE",
            Self::INVALID_SESSION_ID => "INVALID_SESSION_ID",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// reason for getting a session
///
/// The reason may determine in what way a session restores the window
/// management state of associated toplevels.
///
/// For example newly launched applications might be launched on the active
/// workspace with restored size and position, while a recovered
/// application might restore additional state such as active workspace and
/// stacking order.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgSessionManagerV1Reason(pub u32);

impl XdgSessionManagerV1Reason {
    /// an app is newly launched
    ///
    /// A new app instance is launched, for example from an app launcher.
    pub const LAUNCH: Self = Self(1);

    /// an app recovered
    ///
    /// A app instance is recovering from for example a compositor or app crash.
    pub const RECOVER: Self = Self(2);

    /// an app restored
    ///
    /// A app instance is restored, for example part of a restored session, or
    /// restored from having been temporarily terminated due to resource
    /// constraints.
    pub const SESSION_RESTORE: Self = Self(3);
}

impl Debug for XdgSessionManagerV1Reason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::LAUNCH => "LAUNCH",
            Self::RECOVER => "RECOVER",
            Self::SESSION_RESTORE => "SESSION_RESTORE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
