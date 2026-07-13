//! wayland protocol fixes
//!
//! This global fixes problems with other core-protocol interfaces that
//! cannot be fixed in these interfaces themselves.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_fixes object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlFixes {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlFixesHandler>,
}

struct DefaultHandler;

impl WlFixesHandler for DefaultHandler { }

impl ConcreteObject for WlFixes {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WlFixes;
    const INTERFACE_NAME: &str = "wl_fixes";
}

impl WlFixes {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlFixesHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlFixesHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlFixes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlFixes")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlFixes {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroys this object
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_fixes#{}.destroy()\n", id);
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

    /// destroys this object
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wl_fixes.destroy", &e);
        }
    }

    /// Since when the destroy_registry message is available.
    pub const MSG__DESTROY_REGISTRY__SINCE: u32 = 1;

    /// destroy a wl_registry
    ///
    /// This request destroys a wl_registry object.
    ///
    /// The client should no longer use the wl_registry after making this
    /// request.
    ///
    /// The compositor will emit a wl_display.delete_id event with the object ID
    /// of the registry and will no longer emit any events on the registry. The
    /// client should re-use the object ID once it receives the
    /// wl_display.delete_id event.
    ///
    /// # Arguments
    ///
    /// - `registry`: the registry to destroy
    #[inline]
    pub fn try_send_destroy_registry(
        &self,
        registry: &Rc<WlRegistry>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            registry,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("registry"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_fixes#{}.destroy_registry(registry: wl_registry#{})\n", id, arg0);
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
        arg0.handle_server_destroy();
        Ok(())
    }

    /// destroy a wl_registry
    ///
    /// This request destroys a wl_registry object.
    ///
    /// The client should no longer use the wl_registry after making this
    /// request.
    ///
    /// The compositor will emit a wl_display.delete_id event with the object ID
    /// of the registry and will no longer emit any events on the registry. The
    /// client should re-use the object ID once it receives the
    /// wl_display.delete_id event.
    ///
    /// # Arguments
    ///
    /// - `registry`: the registry to destroy
    #[inline]
    pub fn send_destroy_registry(
        &self,
        registry: &Rc<WlRegistry>,
    ) {
        let res = self.try_send_destroy_registry(
            registry,
        );
        if let Err(e) = res {
            log_send("wl_fixes.destroy_registry", &e);
        }
    }

    /// Since when the ack_global_remove message is available.
    pub const MSG__ACK_GLOBAL_REMOVE__SINCE: u32 = 2;

    /// acknowledge global removal
    ///
    /// Acknowledge the removal of the specified global.
    ///
    /// If no global with the specified name exists or the global is not removed,
    /// the wl_fixes.invalid_ack_remove protocol error will be posted.
    ///
    /// Due to the Wayland protocol being asynchronous, the wl_global objects
    /// cannot be destroyed immediately. For example, if a wl_global is removed
    /// and a client attempts to bind that global around same time, it can
    /// result in a protocol error due to an unknown global name in the bind
    /// request.
    ///
    /// In order to avoid crashing clients, the compositor should remove the
    /// wl_global once it is guaranteed that no more bind requests will come.
    ///
    /// The wl_fixes.ack_global_remove() request is used to signal to the
    /// compositor that the client will not bind the given global anymore. After
    /// all clients acknowledge the removal of the global, the compositor can
    /// safely destroy it.
    ///
    /// The client must call the wl_fixes.ack_global_remove() request in
    /// response to a wl_registry.global_remove() event even if it did not bind
    /// the corresponding global.
    ///
    /// # Arguments
    ///
    /// - `registry`: the registry object
    /// - `name`: unique name of the global
    #[inline]
    pub fn try_send_ack_global_remove(
        &self,
        registry: &Rc<WlRegistry>,
        name: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            registry,
            name,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("registry"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_fixes#{}.ack_global_remove(registry: wl_registry#{}, name: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg1,
        ]);
        Ok(())
    }

    /// acknowledge global removal
    ///
    /// Acknowledge the removal of the specified global.
    ///
    /// If no global with the specified name exists or the global is not removed,
    /// the wl_fixes.invalid_ack_remove protocol error will be posted.
    ///
    /// Due to the Wayland protocol being asynchronous, the wl_global objects
    /// cannot be destroyed immediately. For example, if a wl_global is removed
    /// and a client attempts to bind that global around same time, it can
    /// result in a protocol error due to an unknown global name in the bind
    /// request.
    ///
    /// In order to avoid crashing clients, the compositor should remove the
    /// wl_global once it is guaranteed that no more bind requests will come.
    ///
    /// The wl_fixes.ack_global_remove() request is used to signal to the
    /// compositor that the client will not bind the given global anymore. After
    /// all clients acknowledge the removal of the global, the compositor can
    /// safely destroy it.
    ///
    /// The client must call the wl_fixes.ack_global_remove() request in
    /// response to a wl_registry.global_remove() event even if it did not bind
    /// the corresponding global.
    ///
    /// # Arguments
    ///
    /// - `registry`: the registry object
    /// - `name`: unique name of the global
    #[inline]
    pub fn send_ack_global_remove(
        &self,
        registry: &Rc<WlRegistry>,
        name: u32,
    ) {
        let res = self.try_send_ack_global_remove(
            registry,
            name,
        );
        if let Err(e) = res {
            log_send("wl_fixes.ack_global_remove", &e);
        }
    }
}

/// A message handler for [`WlFixes`] proxies.
pub trait WlFixesHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlFixes>) {
        slf.core.delete_id();
    }

    /// destroys this object
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WlFixes>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wl_fixes.destroy", &e);
        }
    }

    /// destroy a wl_registry
    ///
    /// This request destroys a wl_registry object.
    ///
    /// The client should no longer use the wl_registry after making this
    /// request.
    ///
    /// The compositor will emit a wl_display.delete_id event with the object ID
    /// of the registry and will no longer emit any events on the registry. The
    /// client should re-use the object ID once it receives the
    /// wl_display.delete_id event.
    ///
    /// # Arguments
    ///
    /// - `registry`: the registry to destroy
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_destroy_registry(
        &mut self,
        slf: &Rc<WlFixes>,
        registry: &Rc<WlRegistry>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy_registry(
            registry,
        );
        if let Err(e) = res {
            log_forward("wl_fixes.destroy_registry", &e);
        }
    }

    /// acknowledge global removal
    ///
    /// Acknowledge the removal of the specified global.
    ///
    /// If no global with the specified name exists or the global is not removed,
    /// the wl_fixes.invalid_ack_remove protocol error will be posted.
    ///
    /// Due to the Wayland protocol being asynchronous, the wl_global objects
    /// cannot be destroyed immediately. For example, if a wl_global is removed
    /// and a client attempts to bind that global around same time, it can
    /// result in a protocol error due to an unknown global name in the bind
    /// request.
    ///
    /// In order to avoid crashing clients, the compositor should remove the
    /// wl_global once it is guaranteed that no more bind requests will come.
    ///
    /// The wl_fixes.ack_global_remove() request is used to signal to the
    /// compositor that the client will not bind the given global anymore. After
    /// all clients acknowledge the removal of the global, the compositor can
    /// safely destroy it.
    ///
    /// The client must call the wl_fixes.ack_global_remove() request in
    /// response to a wl_registry.global_remove() event even if it did not bind
    /// the corresponding global.
    ///
    /// # Arguments
    ///
    /// - `registry`: the registry object
    /// - `name`: unique name of the global
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_ack_global_remove(
        &mut self,
        slf: &Rc<WlFixes>,
        registry: &Rc<WlRegistry>,
        name: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_ack_global_remove(
            registry,
            name,
        );
        if let Err(e) = res {
            log_forward("wl_fixes.ack_global_remove", &e);
        }
    }
}

impl ObjectPrivate for WlFixes {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlFixes, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_fixes#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_fixes#{}.destroy_registry(registry: wl_registry#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlRegistry>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("registry", o.core().interface, ObjectInterface::WlRegistry)));
                };
                let arg0 = &arg0;
                arg0.core().handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy_registry(&self, arg0);
                } else {
                    DefaultHandler.handle_destroy_registry(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_fixes#{}.ack_global_remove(registry: wl_registry#{}, name: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlRegistry>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("registry", o.core().interface, ObjectInterface::WlRegistry)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_ack_global_remove(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_ack_global_remove(&self, arg0, arg1);
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
            1 => "destroy_registry",
            2 => "ack_global_remove",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for WlFixes {
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

impl WlFixes {
    /// Since when the error.invalid_ack_remove enum variant is available.
    pub const ENM__ERROR_INVALID_ACK_REMOVE__SINCE: u32 = 1;
}

/// wl_fixes error values
///
/// These errors can be emitted in response to wl_fixes requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlFixesError(pub u32);

impl WlFixesError {
    /// unknown global or the global is not removed
    pub const INVALID_ACK_REMOVE: Self = Self(0);
}

impl Debug for WlFixesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_ACK_REMOVE => "INVALID_ACK_REMOVE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
