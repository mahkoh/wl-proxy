//! manager for app id resolver
//!
//! Create a resolver object. Typically exactly one privileged helper
//! (a Wayland client with DBus access) binds this interface and serves
//! identification requests coming from the compositor.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_app_id_resolver_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandAppIdResolverManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandAppIdResolverManagerV1Handler>,
}

struct DefaultHandler;

impl TreelandAppIdResolverManagerV1Handler for DefaultHandler { }

impl ConcreteObject for TreelandAppIdResolverManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::TreelandAppIdResolverManagerV1;
    const INTERFACE_NAME: &str = "treeland_app_id_resolver_manager_v1";
}

impl TreelandAppIdResolverManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl TreelandAppIdResolverManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandAppIdResolverManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandAppIdResolverManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandAppIdResolverManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandAppIdResolverManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_app_id_resolver_manager_v1#{}.destroy()\n", id);
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

    /// destroy the manager
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("treeland_app_id_resolver_manager_v1.destroy", &e);
        }
    }

    /// Since when the get_resolver message is available.
    pub const MSG__GET_RESOLVER__SINCE: u32 = 1;

    /// create/bind a resolver object
    ///
    /// Create or bind a resolver object. Only one resolver may be registered
    /// per session. Treeland is a multi-user compositor; different user
    /// sessions may each register their own resolver. If a resolver is
    /// already bound in the same session, the compositor will report an
    /// error on the manager and will NOT create a new resolver object for
    /// this request.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_get_resolver(
        &self,
        id: &Rc<TreelandAppIdResolverV1>,
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
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_app_id_resolver_manager_v1#{}.get_resolver(id: treeland_app_id_resolver_v1#{})\n", id, arg0);
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

    /// create/bind a resolver object
    ///
    /// Create or bind a resolver object. Only one resolver may be registered
    /// per session. Treeland is a multi-user compositor; different user
    /// sessions may each register their own resolver. If a resolver is
    /// already bound in the same session, the compositor will report an
    /// error on the manager and will NOT create a new resolver object for
    /// this request.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_get_resolver(
        &self,
        id: &Rc<TreelandAppIdResolverV1>,
    ) {
        let res = self.try_send_get_resolver(
            id,
        );
        if let Err(e) = res {
            log_send("treeland_app_id_resolver_manager_v1.get_resolver", &e);
        }
    }

    /// create/bind a resolver object
    ///
    /// Create or bind a resolver object. Only one resolver may be registered
    /// per session. Treeland is a multi-user compositor; different user
    /// sessions may each register their own resolver. If a resolver is
    /// already bound in the same session, the compositor will report an
    /// error on the manager and will NOT create a new resolver object for
    /// this request.
    #[inline]
    pub fn new_try_send_get_resolver(
        &self,
    ) -> Result<Rc<TreelandAppIdResolverV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_resolver(
            &id,
        )?;
        Ok(id)
    }

    /// create/bind a resolver object
    ///
    /// Create or bind a resolver object. Only one resolver may be registered
    /// per session. Treeland is a multi-user compositor; different user
    /// sessions may each register their own resolver. If a resolver is
    /// already bound in the same session, the compositor will report an
    /// error on the manager and will NOT create a new resolver object for
    /// this request.
    #[inline]
    pub fn new_send_get_resolver(
        &self,
    ) -> Rc<TreelandAppIdResolverV1> {
        let id = self.core.create_child();
        self.send_get_resolver(
            &id,
        );
        id
    }
}

/// A message handler for [`TreelandAppIdResolverManagerV1`] proxies.
pub trait TreelandAppIdResolverManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandAppIdResolverManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the manager
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<TreelandAppIdResolverManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("treeland_app_id_resolver_manager_v1.destroy", &e);
        }
    }

    /// create/bind a resolver object
    ///
    /// Create or bind a resolver object. Only one resolver may be registered
    /// per session. Treeland is a multi-user compositor; different user
    /// sessions may each register their own resolver. If a resolver is
    /// already bound in the same session, the compositor will report an
    /// error on the manager and will NOT create a new resolver object for
    /// this request.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_get_resolver(
        &mut self,
        slf: &Rc<TreelandAppIdResolverManagerV1>,
        id: &Rc<TreelandAppIdResolverV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_resolver(
            id,
        );
        if let Err(e) = res {
            log_forward("treeland_app_id_resolver_manager_v1.get_resolver", &e);
        }
    }
}

impl ObjectPrivate for TreelandAppIdResolverManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandAppIdResolverManagerV1, version),
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
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_app_id_resolver_manager_v1#{}.destroy()\n", client_id, id);
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
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_app_id_resolver_manager_v1#{}.get_resolver(id: treeland_app_id_resolver_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = TreelandAppIdResolverV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_resolver(&self, arg0);
                } else {
                    DefaultHandler.handle_get_resolver(&self, arg0);
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
            1 => "get_resolver",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for TreelandAppIdResolverManagerV1 {
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

impl TreelandAppIdResolverManagerV1 {
    /// Since when the error.resolver_already_exists enum variant is available.
    pub const ENM__ERROR_RESOLVER_ALREADY_EXISTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandAppIdResolverManagerV1Error(pub u32);

impl TreelandAppIdResolverManagerV1Error {
    /// A resolver is already registered in this session with the manager
    pub const RESOLVER_ALREADY_EXISTS: Self = Self(1);
}

impl Debug for TreelandAppIdResolverManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RESOLVER_ALREADY_EXISTS => "RESOLVER_ALREADY_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
