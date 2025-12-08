//! global registry object
//!
//! The singleton global registry object.  The server has a number of
//! global objects that are available to all clients.  These objects
//! typically represent an actual object in the server (for example,
//! an input device) or they are singleton objects that provide
//! extension functionality.
//!
//! When a client creates a registry object, the registry object
//! will emit a global event for each global currently in the
//! registry.  Globals come and go as a result of device or
//! monitor hotplugs, reconfiguration or other events, and the
//! registry will send out global and global_remove events to
//! keep the client up to date with the changes.  To mark the end
//! of the initial burst of events, the client can use the
//! wl_display.sync request immediately after calling
//! wl_display.get_registry.
//!
//! A client can bind to a global object by using the bind
//! request.  This creates a client-side handle that lets the object
//! emit events to the client and lets the client invoke requests on
//! the object.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_registry object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlRegistry {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlRegistryHandler>,
    names: RefCell<HashSet<u32>>,
}

struct DefaultHandler;

impl WlRegistryHandler for DefaultHandler { }

impl ConcreteObject for WlRegistry {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WlRegistry;
    const INTERFACE_NAME: &str = "wl_registry";
}

impl WlRegistry {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlRegistryHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlRegistryHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlRegistry")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlRegistry {
    /// Since when the bind message is available.
    pub const MSG__BIND__SINCE: u32 = 1;

    /// bind an object to the display
    ///
    /// Binds a new, client-created object to the server using the
    /// specified name as the identifier.
    ///
    /// # Arguments
    ///
    /// - `name`: unique numeric name of the object
    /// - `id`: bounded object
    #[inline]
    pub fn try_send_bind(
        &self,
        name: u32,
        id: Rc<dyn Object>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            name,
            id,
        );
        let arg1_obj = arg1;
        let arg1 = arg1_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg1.generate_server_id(arg1_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg1_id = arg1.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1_interface: &str, arg1_id: u32, arg1_version: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_registry#{}.bind(name: {}, id: {}#{} (version: {}))\n", id, arg0, arg1_interface, arg1_id, arg1_version);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1.interface.name(), arg1_id, arg1.version);
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
            arg0,
        ]);
        fmt.string(arg1.interface.name());
        fmt.words([
            arg1.version,
            arg1_id,
        ]);
        Ok(())
    }

    /// bind an object to the display
    ///
    /// Binds a new, client-created object to the server using the
    /// specified name as the identifier.
    ///
    /// # Arguments
    ///
    /// - `name`: unique numeric name of the object
    /// - `id`: bounded object
    #[inline]
    pub fn send_bind(
        &self,
        name: u32,
        id: Rc<dyn Object>,
    ) {
        let res = self.try_send_bind(
            name,
            id,
        );
        if let Err(e) = res {
            log_send("wl_registry.bind", &e);
        }
    }

    /// Since when the global message is available.
    pub const MSG__GLOBAL__SINCE: u32 = 1;

    /// announce global object
    ///
    /// Notify the client of global objects.
    ///
    /// The event notifies the client that a global object with
    /// the given name is now available, and it implements the
    /// given version of the given interface.
    ///
    /// # Arguments
    ///
    /// - `name`: numeric name of the global object
    /// - `interface`: interface implemented by the object
    /// - `version`: interface version
    #[inline]
    pub fn try_send_global(
        &self,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<(), ObjectError> {
        let interface = interface.name();
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            name,
            interface,
            version,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_registry#{}.global(name: {}, interface: {:?}, version: {})\n", client_id, id, arg0, arg1, arg2);
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
        ]);
        fmt.string(arg1);
        fmt.words([
            arg2,
        ]);
        Ok(())
    }

    /// announce global object
    ///
    /// Notify the client of global objects.
    ///
    /// The event notifies the client that a global object with
    /// the given name is now available, and it implements the
    /// given version of the given interface.
    ///
    /// # Arguments
    ///
    /// - `name`: numeric name of the global object
    /// - `interface`: interface implemented by the object
    /// - `version`: interface version
    #[inline]
    pub fn send_global(
        &self,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        let res = self.try_send_global(
            name,
            interface,
            version,
        );
        if let Err(e) = res {
            log_send("wl_registry.global", &e);
        }
    }

    /// Since when the global_remove message is available.
    pub const MSG__GLOBAL_REMOVE__SINCE: u32 = 1;

    /// announce removal of global object
    ///
    /// Notify the client of removed global objects.
    ///
    /// This event notifies the client that the global identified
    /// by name is no longer available.  If the client bound to
    /// the global using the bind request, the client should now
    /// destroy that object.
    ///
    /// The object remains valid and requests to the object will be
    /// ignored until the client destroys it, to avoid races between
    /// the global going away and a client sending a request to it.
    ///
    /// # Arguments
    ///
    /// - `name`: numeric name of the global object
    #[inline]
    pub fn try_send_global_remove(
        &self,
        name: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_registry#{}.global_remove(name: {})\n", client_id, id, arg0);
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
            1,
            arg0,
        ]);
        Ok(())
    }

    /// announce removal of global object
    ///
    /// Notify the client of removed global objects.
    ///
    /// This event notifies the client that the global identified
    /// by name is no longer available.  If the client bound to
    /// the global using the bind request, the client should now
    /// destroy that object.
    ///
    /// The object remains valid and requests to the object will be
    /// ignored until the client destroys it, to avoid races between
    /// the global going away and a client sending a request to it.
    ///
    /// # Arguments
    ///
    /// - `name`: numeric name of the global object
    #[inline]
    pub fn send_global_remove(
        &self,
        name: u32,
    ) {
        let res = self.try_send_global_remove(
            name,
        );
        if let Err(e) = res {
            log_send("wl_registry.global_remove", &e);
        }
    }
}

/// A message handler for [`WlRegistry`] proxies.
pub trait WlRegistryHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlRegistry>) {
        slf.core.delete_id();
    }

    /// bind an object to the display
    ///
    /// Binds a new, client-created object to the server using the
    /// specified name as the identifier.
    ///
    /// # Arguments
    ///
    /// - `name`: unique numeric name of the object
    /// - `id`: bounded object
    #[inline]
    fn handle_bind(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        id: Rc<dyn Object>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_bind(
            name,
            id,
        );
        if let Err(e) = res {
            log_forward("wl_registry.bind", &e);
        }
    }

    /// announce global object
    ///
    /// Notify the client of global objects.
    ///
    /// The event notifies the client that a global object with
    /// the given name is now available, and it implements the
    /// given version of the given interface.
    ///
    /// # Arguments
    ///
    /// - `name`: numeric name of the global object
    /// - `interface`: interface implemented by the object
    /// - `version`: interface version
    #[inline]
    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_global(
            name,
            interface,
            version,
        );
        if let Err(e) = res {
            log_forward("wl_registry.global", &e);
        }
    }

    /// announce removal of global object
    ///
    /// Notify the client of removed global objects.
    ///
    /// This event notifies the client that the global identified
    /// by name is no longer available.  If the client bound to
    /// the global using the bind request, the client should now
    /// destroy that object.
    ///
    /// The object remains valid and requests to the object will be
    /// ignored until the client destroys it, to avoid races between
    /// the global going away and a client sending a request to it.
    ///
    /// # Arguments
    ///
    /// - `name`: numeric name of the global object
    #[inline]
    fn handle_global_remove(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_global_remove(
            name,
        );
        if let Err(e) = res {
            log_forward("wl_registry.global_remove", &e);
        }
    }
}

impl ObjectPrivate for WlRegistry {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlRegistry, version),
            handler: Default::default(),
            names: Default::default(),
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("name")));
                };
                offset += 1;
                let arg1_interface;
                (arg1_interface, offset) = parse_string::<NonNullString>(msg, offset, "id")?;
                let Some(&arg1_version) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1_interface: &str, arg1_id: u32, arg1_version: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_registry#{}.bind(name: {}, id: {}#{} (version: {}))\n", client_id, id, arg0, arg1_interface, arg1_id, arg1_version);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1_interface, arg1, arg1_version);
                }
                let arg1_id = arg1;
                let arg1 = create_object_for_interface(&self.core.state, arg1_interface, arg1_version)?;
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg1_id, "id", e)))?;
                if let Some(handler) = handler {
                    (**handler).handle_bind(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_bind(&self, arg0, arg1);
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("name")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "interface")?;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("version")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: &str, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_registry#{}.global(name: {}, interface: {:?}, version: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                let Some(arg1) = ObjectInterface::from_str(arg1) else {
                    return Ok(());
                };
                let max_version = self.core.state.baseline.1[arg1];
                if max_version == 0 {
                    return Ok(());
                }
                self.names.borrow_mut().insert(arg0);
                let arg2 = max_version.min(arg2);
                if let Some(handler) = handler {
                    (**handler).handle_global(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_global(&self, arg0, arg1, arg2);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_registry#{}.global_remove(name: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if !self.names.borrow_mut().remove(&arg0) {
                    return Ok(());
                }
                if let Some(handler) = handler {
                    (**handler).handle_global_remove(&self, arg0);
                } else {
                    DefaultHandler.handle_global_remove(&self, arg0);
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
            0 => "bind",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "global",
            1 => "global_remove",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlRegistry {
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

