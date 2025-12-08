//! core global object
//!
//! The core global object.  This is a special singleton object.  It
//! is used for internal Wayland protocol features.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_display object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlDisplay {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlDisplayHandler>,
}

struct DefaultHandler;

impl WlDisplayHandler for DefaultHandler { }

impl ConcreteObject for WlDisplay {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WlDisplay;
    const INTERFACE_NAME: &str = "wl_display";
}

impl WlDisplay {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlDisplayHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlDisplayHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlDisplay")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlDisplay {
    /// Since when the sync message is available.
    pub const MSG__SYNC__SINCE: u32 = 1;

    /// asynchronous roundtrip
    ///
    /// The sync request asks the server to emit the 'done' event
    /// on the returned wl_callback object.  Since requests are
    /// handled in-order and events are delivered in-order, this can
    /// be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is undefined and should be ignored.
    ///
    /// # Arguments
    ///
    /// - `callback`: callback object for the sync request
    #[inline]
    pub fn try_send_sync(
        &self,
        callback: &Rc<WlCallback>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            callback,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("callback", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_display#{}.sync(callback: wl_callback#{})\n", id, arg0);
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
            0,
            arg0_id,
        ]);
        Ok(())
    }

    /// asynchronous roundtrip
    ///
    /// The sync request asks the server to emit the 'done' event
    /// on the returned wl_callback object.  Since requests are
    /// handled in-order and events are delivered in-order, this can
    /// be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is undefined and should be ignored.
    ///
    /// # Arguments
    ///
    /// - `callback`: callback object for the sync request
    #[inline]
    pub fn send_sync(
        &self,
        callback: &Rc<WlCallback>,
    ) {
        let res = self.try_send_sync(
            callback,
        );
        if let Err(e) = res {
            log_send("wl_display.sync", &e);
        }
    }

    /// asynchronous roundtrip
    ///
    /// The sync request asks the server to emit the 'done' event
    /// on the returned wl_callback object.  Since requests are
    /// handled in-order and events are delivered in-order, this can
    /// be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is undefined and should be ignored.
    #[inline]
    pub fn new_try_send_sync(
        &self,
    ) -> Result<Rc<WlCallback>, ObjectError> {
        let callback = self.core.create_child();
        self.try_send_sync(
            &callback,
        )?;
        Ok(callback)
    }

    /// asynchronous roundtrip
    ///
    /// The sync request asks the server to emit the 'done' event
    /// on the returned wl_callback object.  Since requests are
    /// handled in-order and events are delivered in-order, this can
    /// be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is undefined and should be ignored.
    #[inline]
    pub fn new_send_sync(
        &self,
    ) -> Rc<WlCallback> {
        let callback = self.core.create_child();
        self.send_sync(
            &callback,
        );
        callback
    }

    /// Since when the get_registry message is available.
    pub const MSG__GET_REGISTRY__SINCE: u32 = 1;

    /// get global registry object
    ///
    /// This request creates a registry object that allows the client
    /// to list and bind the global objects available from the
    /// compositor.
    ///
    /// It should be noted that the server side resources consumed in
    /// response to a get_registry request can only be released when the
    /// client disconnects, not when the client side proxy is destroyed.
    /// Therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    ///
    /// # Arguments
    ///
    /// - `registry`: global registry object
    #[inline]
    pub fn try_send_get_registry(
        &self,
        registry: &Rc<WlRegistry>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            registry,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("registry", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_display#{}.get_registry(registry: wl_registry#{})\n", id, arg0);
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

    /// get global registry object
    ///
    /// This request creates a registry object that allows the client
    /// to list and bind the global objects available from the
    /// compositor.
    ///
    /// It should be noted that the server side resources consumed in
    /// response to a get_registry request can only be released when the
    /// client disconnects, not when the client side proxy is destroyed.
    /// Therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    ///
    /// # Arguments
    ///
    /// - `registry`: global registry object
    #[inline]
    pub fn send_get_registry(
        &self,
        registry: &Rc<WlRegistry>,
    ) {
        let res = self.try_send_get_registry(
            registry,
        );
        if let Err(e) = res {
            log_send("wl_display.get_registry", &e);
        }
    }

    /// get global registry object
    ///
    /// This request creates a registry object that allows the client
    /// to list and bind the global objects available from the
    /// compositor.
    ///
    /// It should be noted that the server side resources consumed in
    /// response to a get_registry request can only be released when the
    /// client disconnects, not when the client side proxy is destroyed.
    /// Therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    #[inline]
    pub fn new_try_send_get_registry(
        &self,
    ) -> Result<Rc<WlRegistry>, ObjectError> {
        let registry = self.core.create_child();
        self.try_send_get_registry(
            &registry,
        )?;
        Ok(registry)
    }

    /// get global registry object
    ///
    /// This request creates a registry object that allows the client
    /// to list and bind the global objects available from the
    /// compositor.
    ///
    /// It should be noted that the server side resources consumed in
    /// response to a get_registry request can only be released when the
    /// client disconnects, not when the client side proxy is destroyed.
    /// Therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    #[inline]
    pub fn new_send_get_registry(
        &self,
    ) -> Rc<WlRegistry> {
        let registry = self.core.create_child();
        self.send_get_registry(
            &registry,
        );
        registry
    }

    /// Since when the error message is available.
    pub const MSG__ERROR__SINCE: u32 = 1;

    /// fatal error event
    ///
    /// The error event is sent out when a fatal (non-recoverable)
    /// error has occurred.  The object_id argument is the object
    /// where the error occurred, most often in response to a request
    /// to that object.  The code identifies the error and is defined
    /// by the object interface.  As such, each interface defines its
    /// own set of error codes.  The message is a brief description
    /// of the error, for (debugging) convenience.
    ///
    /// # Arguments
    ///
    /// - `object_id`: object where the error occurred
    /// - `code`: error code
    /// - `message`: error description
    #[inline]
    pub fn try_send_error(
        &self,
        object_id: Rc<dyn Object>,
        code: u32,
        message: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            object_id,
            code,
            message,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("object_id", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_display#{}.error(object_id: unknown#{}, code: {}, message: {:?})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id, arg1, arg2);
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
            arg0_id,
            arg1,
        ]);
        fmt.string(arg2);
        Ok(())
    }

    /// fatal error event
    ///
    /// The error event is sent out when a fatal (non-recoverable)
    /// error has occurred.  The object_id argument is the object
    /// where the error occurred, most often in response to a request
    /// to that object.  The code identifies the error and is defined
    /// by the object interface.  As such, each interface defines its
    /// own set of error codes.  The message is a brief description
    /// of the error, for (debugging) convenience.
    ///
    /// # Arguments
    ///
    /// - `object_id`: object where the error occurred
    /// - `code`: error code
    /// - `message`: error description
    #[inline]
    pub fn send_error(
        &self,
        object_id: Rc<dyn Object>,
        code: u32,
        message: &str,
    ) {
        let res = self.try_send_error(
            object_id,
            code,
            message,
        );
        if let Err(e) = res {
            log_send("wl_display.error", &e);
        }
    }

    /// Since when the delete_id message is available.
    pub const MSG__DELETE_ID__SINCE: u32 = 1;

    /// acknowledge object ID deletion
    ///
    /// This event is used internally by the object ID management
    /// logic. When a client deletes an object that it had created,
    /// the server will send this event to acknowledge that it has
    /// seen the delete request. When the client receives this event,
    /// it will know that it can safely reuse the object ID.
    ///
    /// # Arguments
    ///
    /// - `id`: deleted object ID
    #[inline]
    pub fn try_send_delete_id(
        &self,
        id: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_display#{}.delete_id(id: {})\n", client_id, id, arg0);
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

    /// acknowledge object ID deletion
    ///
    /// This event is used internally by the object ID management
    /// logic. When a client deletes an object that it had created,
    /// the server will send this event to acknowledge that it has
    /// seen the delete request. When the client receives this event,
    /// it will know that it can safely reuse the object ID.
    ///
    /// # Arguments
    ///
    /// - `id`: deleted object ID
    #[inline]
    pub fn send_delete_id(
        &self,
        id: u32,
    ) {
        let res = self.try_send_delete_id(
            id,
        );
        if let Err(e) = res {
            log_send("wl_display.delete_id", &e);
        }
    }
}

/// A message handler for [`WlDisplay`] proxies.
pub trait WlDisplayHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlDisplay>) {
        slf.core.delete_id();
    }

    /// asynchronous roundtrip
    ///
    /// The sync request asks the server to emit the 'done' event
    /// on the returned wl_callback object.  Since requests are
    /// handled in-order and events are delivered in-order, this can
    /// be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is undefined and should be ignored.
    ///
    /// # Arguments
    ///
    /// - `callback`: callback object for the sync request
    #[inline]
    fn handle_sync(
        &mut self,
        slf: &Rc<WlDisplay>,
        callback: &Rc<WlCallback>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_sync(
            callback,
        );
        if let Err(e) = res {
            log_forward("wl_display.sync", &e);
        }
    }

    /// get global registry object
    ///
    /// This request creates a registry object that allows the client
    /// to list and bind the global objects available from the
    /// compositor.
    ///
    /// It should be noted that the server side resources consumed in
    /// response to a get_registry request can only be released when the
    /// client disconnects, not when the client side proxy is destroyed.
    /// Therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    ///
    /// # Arguments
    ///
    /// - `registry`: global registry object
    #[inline]
    fn handle_get_registry(
        &mut self,
        slf: &Rc<WlDisplay>,
        registry: &Rc<WlRegistry>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_registry(
            registry,
        );
        if let Err(e) = res {
            log_forward("wl_display.get_registry", &e);
        }
    }
}

impl ObjectPrivate for WlDisplay {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlDisplay, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_display#{}.sync(callback: wl_callback#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlCallback::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "callback", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_sync(&self, arg0);
                } else {
                    DefaultHandler.handle_sync(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_display#{}.get_registry(registry: wl_registry#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlRegistry::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "registry", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_registry(&self, arg0);
                } else {
                    DefaultHandler.handle_get_registry(&self, arg0);
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
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("object_id")));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("code")));
                };
                offset += 1;
                let arg2;
                (arg2, offset) = parse_string::<NonNullString>(msg, offset, "message")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_display#{}.error(object_id: unknown#{}, code: {}, message: {:?})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                return Err(ObjectError(ObjectErrorKind::ServerError(arg0.core().interface, arg0_id, arg1, StringError(arg2.to_string()))));
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_display#{}.delete_id(id: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                self.core.state.handle_delete_id(server, arg0);
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
            0 => "sync",
            1 => "get_registry",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "error",
            1 => "delete_id",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlDisplay {
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

impl WlDisplay {
    /// Since when the error.invalid_object enum variant is available.
    pub const ENM__ERROR_INVALID_OBJECT__SINCE: u32 = 1;
    /// Since when the error.invalid_method enum variant is available.
    pub const ENM__ERROR_INVALID_METHOD__SINCE: u32 = 1;
    /// Since when the error.no_memory enum variant is available.
    pub const ENM__ERROR_NO_MEMORY__SINCE: u32 = 1;
    /// Since when the error.implementation enum variant is available.
    pub const ENM__ERROR_IMPLEMENTATION__SINCE: u32 = 1;
}

/// global error values
///
/// These errors are global and can be emitted in response to any
/// server request.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlDisplayError(pub u32);

impl WlDisplayError {
    /// server couldn't find object
    pub const INVALID_OBJECT: Self = Self(0);

    /// method doesn't exist on the specified interface or malformed request
    pub const INVALID_METHOD: Self = Self(1);

    /// server is out of memory
    pub const NO_MEMORY: Self = Self(2);

    /// implementation error in compositor
    pub const IMPLEMENTATION: Self = Self(3);
}

impl Debug for WlDisplayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_OBJECT => "INVALID_OBJECT",
            Self::INVALID_METHOD => "INVALID_METHOD",
            Self::NO_MEMORY => "NO_MEMORY",
            Self::IMPLEMENTATION => "IMPLEMENTATION",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
