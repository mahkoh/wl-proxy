//! used to lock the session
//!
//! This interface is used to request that the session be locked.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_session_lock_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtSessionLockManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtSessionLockManagerV1Handler>,
}

struct DefaultHandler;

impl ExtSessionLockManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ExtSessionLockManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtSessionLockManagerV1;
    const INTERFACE_NAME: &str = "ext_session_lock_manager_v1";
}

impl ExtSessionLockManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtSessionLockManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtSessionLockManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtSessionLockManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtSessionLockManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtSessionLockManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the session lock manager object
    ///
    /// This informs the compositor that the session lock manager object will
    /// no longer be used. Existing objects created through this interface
    /// remain valid.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_session_lock_manager_v1#{}.destroy()\n", id);
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

    /// destroy the session lock manager object
    ///
    /// This informs the compositor that the session lock manager object will
    /// no longer be used. Existing objects created through this interface
    /// remain valid.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_session_lock_manager_v1.destroy", &e);
        }
    }

    /// Since when the lock message is available.
    pub const MSG__LOCK__SINCE: u32 = 1;

    /// attempt to lock the session
    ///
    /// This request creates a session lock and asks the compositor to lock the
    /// session. The compositor will send either the ext_session_lock_v1.locked
    /// or ext_session_lock_v1.finished event on the created object in
    /// response to this request.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_lock(
        &self,
        id: &Rc<ExtSessionLockV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_session_lock_manager_v1#{}.lock(id: ext_session_lock_v1#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
        ]);
        Ok(())
    }

    /// attempt to lock the session
    ///
    /// This request creates a session lock and asks the compositor to lock the
    /// session. The compositor will send either the ext_session_lock_v1.locked
    /// or ext_session_lock_v1.finished event on the created object in
    /// response to this request.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_lock(
        &self,
        id: &Rc<ExtSessionLockV1>,
    ) {
        let res = self.try_send_lock(
            id,
        );
        if let Err(e) = res {
            log_send("ext_session_lock_manager_v1.lock", &e);
        }
    }

    /// attempt to lock the session
    ///
    /// This request creates a session lock and asks the compositor to lock the
    /// session. The compositor will send either the ext_session_lock_v1.locked
    /// or ext_session_lock_v1.finished event on the created object in
    /// response to this request.
    #[inline]
    pub fn new_try_send_lock(
        &self,
    ) -> Result<Rc<ExtSessionLockV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_lock(
            &id,
        )?;
        Ok(id)
    }

    /// attempt to lock the session
    ///
    /// This request creates a session lock and asks the compositor to lock the
    /// session. The compositor will send either the ext_session_lock_v1.locked
    /// or ext_session_lock_v1.finished event on the created object in
    /// response to this request.
    #[inline]
    pub fn new_send_lock(
        &self,
    ) -> Rc<ExtSessionLockV1> {
        let id = self.core.create_child();
        self.send_lock(
            &id,
        );
        id
    }
}

/// A message handler for [`ExtSessionLockManagerV1`] proxies.
pub trait ExtSessionLockManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtSessionLockManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the session lock manager object
    ///
    /// This informs the compositor that the session lock manager object will
    /// no longer be used. Existing objects created through this interface
    /// remain valid.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtSessionLockManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_session_lock_manager_v1.destroy", &e);
        }
    }

    /// attempt to lock the session
    ///
    /// This request creates a session lock and asks the compositor to lock the
    /// session. The compositor will send either the ext_session_lock_v1.locked
    /// or ext_session_lock_v1.finished event on the created object in
    /// response to this request.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_lock(
        &mut self,
        slf: &Rc<ExtSessionLockManagerV1>,
        id: &Rc<ExtSessionLockV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_lock(
            id,
        );
        if let Err(e) = res {
            log_forward("ext_session_lock_manager_v1.lock", &e);
        }
    }
}

impl ObjectPrivate for ExtSessionLockManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtSessionLockManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_session_lock_manager_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_session_lock_manager_v1#{}.lock(id: ext_session_lock_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = ExtSessionLockV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_lock(&self, arg0);
                } else {
                    DefaultHandler.handle_lock(&self, arg0);
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
            1 => "lock",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ExtSessionLockManagerV1 {
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

