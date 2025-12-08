//! weston internal debugging
//!
//! This is a generic debugging interface for Weston internals, the global
//! object advertized through wl_registry.
//!
//! WARNING: This interface by design allows a denial-of-service attack. It
//! should not be offered in production, or proper authorization mechanisms
//! must be enforced.
//!
//! The idea is for a client to provide a file descriptor that the server
//! uses for printing debug information. The server uses the file
//! descriptor in blocking writes mode, which exposes the denial-of-service
//! risk. The blocking mode is necessary to ensure all debug messages can
//! be easily printed in place. It also ensures message ordering if a
//! client subscribes to more than one debug stream.
//!
//! The available debugging features depend on the server.
//!
//! A debug stream can be one-shot where the server prints the requested
//! information and then closes it, or continuous where server keeps on
//! printing until the client stops it. Or anything in between.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_debug_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonDebugV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonDebugV1Handler>,
}

struct DefaultHandler;

impl WestonDebugV1Handler for DefaultHandler { }

impl ConcreteObject for WestonDebugV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonDebugV1;
    const INTERFACE_NAME: &str = "weston_debug_v1";
}

impl WestonDebugV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonDebugV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonDebugV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonDebugV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonDebugV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonDebugV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy factory object
    ///
    /// Destroys the factory object, but does not affect any other objects.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_debug_v1#{}.destroy()\n", id);
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

    /// destroy factory object
    ///
    /// Destroys the factory object, but does not affect any other objects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("weston_debug_v1.destroy", &e);
        }
    }

    /// Since when the available message is available.
    pub const MSG__AVAILABLE__SINCE: u32 = 1;

    /// advertise available debug scope
    ///
    /// Advertises an available debug scope which the client may be able to
    /// bind to. No information is provided by the server about the content
    /// contained within the debug streams provided by the scope, once a
    /// client has subscribed.
    ///
    /// # Arguments
    ///
    /// - `name`: debug stream name
    /// - `description`: human-readable description of the debug scope
    #[inline]
    pub fn try_send_available(
        &self,
        name: &str,
        description: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            name,
            description,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_debug_v1#{}.available(name: {:?}, description: {:?})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
        fmt.string(arg0);
        if let Some(arg1) = arg1 {
            fmt.string(arg1);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// advertise available debug scope
    ///
    /// Advertises an available debug scope which the client may be able to
    /// bind to. No information is provided by the server about the content
    /// contained within the debug streams provided by the scope, once a
    /// client has subscribed.
    ///
    /// # Arguments
    ///
    /// - `name`: debug stream name
    /// - `description`: human-readable description of the debug scope
    #[inline]
    pub fn send_available(
        &self,
        name: &str,
        description: Option<&str>,
    ) {
        let res = self.try_send_available(
            name,
            description,
        );
        if let Err(e) = res {
            log_send("weston_debug_v1.available", &e);
        }
    }

    /// Since when the subscribe message is available.
    pub const MSG__SUBSCRIBE__SINCE: u32 = 1;

    /// subscribe to a debug stream
    ///
    /// Subscribe to a named debug stream. The server will start printing
    /// to the given file descriptor.
    ///
    /// If the named debug stream is a one-shot dump, the server will send
    /// weston_debug_stream_v1.complete event once all requested data has
    /// been printed. Otherwise, the server will continue streaming debug
    /// prints until the subscription object is destroyed.
    ///
    /// If the debug stream name is unknown to the server, the server will
    /// immediately respond with weston_debug_stream_v1.failure event.
    ///
    /// # Arguments
    ///
    /// - `name`: debug stream name
    /// - `streamfd`: write stream file descriptor
    /// - `stream`: created debug stream object
    #[inline]
    pub fn try_send_subscribe(
        &self,
        name: &str,
        streamfd: &Rc<OwnedFd>,
        stream: &Rc<WestonDebugStreamV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            name,
            streamfd,
            stream,
        );
        let arg2_obj = arg2;
        let arg2 = arg2_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg2.generate_server_id(arg2_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("stream", e)))?;
        let arg2_id = arg2.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: i32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_debug_v1#{}.subscribe(name: {:?}, streamfd: {}, stream: weston_debug_stream_v1#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1.as_raw_fd(), arg2_id);
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
        ]);
        fmt.string(arg0);
        fmt.fds.push_back(arg1.clone());
        fmt.words([
            arg2_id,
        ]);
        Ok(())
    }

    /// subscribe to a debug stream
    ///
    /// Subscribe to a named debug stream. The server will start printing
    /// to the given file descriptor.
    ///
    /// If the named debug stream is a one-shot dump, the server will send
    /// weston_debug_stream_v1.complete event once all requested data has
    /// been printed. Otherwise, the server will continue streaming debug
    /// prints until the subscription object is destroyed.
    ///
    /// If the debug stream name is unknown to the server, the server will
    /// immediately respond with weston_debug_stream_v1.failure event.
    ///
    /// # Arguments
    ///
    /// - `name`: debug stream name
    /// - `streamfd`: write stream file descriptor
    /// - `stream`: created debug stream object
    #[inline]
    pub fn send_subscribe(
        &self,
        name: &str,
        streamfd: &Rc<OwnedFd>,
        stream: &Rc<WestonDebugStreamV1>,
    ) {
        let res = self.try_send_subscribe(
            name,
            streamfd,
            stream,
        );
        if let Err(e) = res {
            log_send("weston_debug_v1.subscribe", &e);
        }
    }

    /// subscribe to a debug stream
    ///
    /// Subscribe to a named debug stream. The server will start printing
    /// to the given file descriptor.
    ///
    /// If the named debug stream is a one-shot dump, the server will send
    /// weston_debug_stream_v1.complete event once all requested data has
    /// been printed. Otherwise, the server will continue streaming debug
    /// prints until the subscription object is destroyed.
    ///
    /// If the debug stream name is unknown to the server, the server will
    /// immediately respond with weston_debug_stream_v1.failure event.
    ///
    /// # Arguments
    ///
    /// - `name`: debug stream name
    /// - `streamfd`: write stream file descriptor
    #[inline]
    pub fn new_try_send_subscribe(
        &self,
        name: &str,
        streamfd: &Rc<OwnedFd>,
    ) -> Result<Rc<WestonDebugStreamV1>, ObjectError> {
        let stream = self.core.create_child();
        self.try_send_subscribe(
            name,
            streamfd,
            &stream,
        )?;
        Ok(stream)
    }

    /// subscribe to a debug stream
    ///
    /// Subscribe to a named debug stream. The server will start printing
    /// to the given file descriptor.
    ///
    /// If the named debug stream is a one-shot dump, the server will send
    /// weston_debug_stream_v1.complete event once all requested data has
    /// been printed. Otherwise, the server will continue streaming debug
    /// prints until the subscription object is destroyed.
    ///
    /// If the debug stream name is unknown to the server, the server will
    /// immediately respond with weston_debug_stream_v1.failure event.
    ///
    /// # Arguments
    ///
    /// - `name`: debug stream name
    /// - `streamfd`: write stream file descriptor
    #[inline]
    pub fn new_send_subscribe(
        &self,
        name: &str,
        streamfd: &Rc<OwnedFd>,
    ) -> Rc<WestonDebugStreamV1> {
        let stream = self.core.create_child();
        self.send_subscribe(
            name,
            streamfd,
            &stream,
        );
        stream
    }
}

/// A message handler for [`WestonDebugV1`] proxies.
pub trait WestonDebugV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonDebugV1>) {
        slf.core.delete_id();
    }

    /// destroy factory object
    ///
    /// Destroys the factory object, but does not affect any other objects.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WestonDebugV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("weston_debug_v1.destroy", &e);
        }
    }

    /// advertise available debug scope
    ///
    /// Advertises an available debug scope which the client may be able to
    /// bind to. No information is provided by the server about the content
    /// contained within the debug streams provided by the scope, once a
    /// client has subscribed.
    ///
    /// # Arguments
    ///
    /// - `name`: debug stream name
    /// - `description`: human-readable description of the debug scope
    #[inline]
    fn handle_available(
        &mut self,
        slf: &Rc<WestonDebugV1>,
        name: &str,
        description: Option<&str>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_available(
            name,
            description,
        );
        if let Err(e) = res {
            log_forward("weston_debug_v1.available", &e);
        }
    }

    /// subscribe to a debug stream
    ///
    /// Subscribe to a named debug stream. The server will start printing
    /// to the given file descriptor.
    ///
    /// If the named debug stream is a one-shot dump, the server will send
    /// weston_debug_stream_v1.complete event once all requested data has
    /// been printed. Otherwise, the server will continue streaming debug
    /// prints until the subscription object is destroyed.
    ///
    /// If the debug stream name is unknown to the server, the server will
    /// immediately respond with weston_debug_stream_v1.failure event.
    ///
    /// # Arguments
    ///
    /// - `name`: debug stream name
    /// - `streamfd`: write stream file descriptor
    /// - `stream`: created debug stream object
    #[inline]
    fn handle_subscribe(
        &mut self,
        slf: &Rc<WestonDebugV1>,
        name: &str,
        streamfd: &Rc<OwnedFd>,
        stream: &Rc<WestonDebugStreamV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_subscribe(
            name,
            streamfd,
            stream,
        );
        if let Err(e) = res {
            log_forward("weston_debug_v1.subscribe", &e);
        }
    }
}

impl ObjectPrivate for WestonDebugV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonDebugV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_debug_v1#{}.destroy()\n", client_id, id);
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
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("stream")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("streamfd")));
                };
                let arg1 = &arg1;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: i32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_debug_v1#{}.subscribe(name: {:?}, streamfd: {}, stream: weston_debug_stream_v1#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1.as_raw_fd(), arg2);
                }
                let arg2_id = arg2;
                let arg2 = WestonDebugStreamV1::new(&self.core.state, self.core.version);
                arg2.core().set_client_id(client, arg2_id, arg2.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg2_id, "stream", e)))?;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_subscribe(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_subscribe(&self, arg0, arg1, arg2);
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
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "name")?;
                let arg1;
                (arg1, offset) = parse_string::<NullableString>(msg, offset, "description")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str, arg1: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_debug_v1#{}.available(name: {:?}, description: {:?})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_available(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_available(&self, arg0, arg1);
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
            1 => "subscribe",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "available",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WestonDebugV1 {
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

