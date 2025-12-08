//! A subscribed debug stream
//!
//! Represents one subscribed debug stream, created with
//! weston_debug_v1.subscribe. When the object is created, it is associated
//! with a given file descriptor. The server will continue writing to the
//! file descriptor until the object is destroyed or the server sends an
//! event through the object.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_debug_stream_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonDebugStreamV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonDebugStreamV1Handler>,
}

struct DefaultHandler;

impl WestonDebugStreamV1Handler for DefaultHandler { }

impl ConcreteObject for WestonDebugStreamV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonDebugStreamV1;
    const INTERFACE_NAME: &str = "weston_debug_stream_v1";
}

impl WestonDebugStreamV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonDebugStreamV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonDebugStreamV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonDebugStreamV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonDebugStreamV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonDebugStreamV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// close a debug stream
    ///
    /// Destroys the object, which causes the server to stop writing into
    /// and closes the associated file descriptor if it was not closed
    /// already.
    ///
    /// Use a wl_display.sync if the clients needs to guarantee the file
    /// descriptor is closed before continuing.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_debug_stream_v1#{}.destroy()\n", id);
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

    /// close a debug stream
    ///
    /// Destroys the object, which causes the server to stop writing into
    /// and closes the associated file descriptor if it was not closed
    /// already.
    ///
    /// Use a wl_display.sync if the clients needs to guarantee the file
    /// descriptor is closed before continuing.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("weston_debug_stream_v1.destroy", &e);
        }
    }

    /// Since when the complete message is available.
    pub const MSG__COMPLETE__SINCE: u32 = 1;

    /// server completed the debug stream
    ///
    /// The server has successfully finished writing to and has closed the
    /// associated file descriptor.
    ///
    /// This event is delivered only for one-shot debug streams where the
    /// server dumps some data and stop. This is never delivered for
    /// continuous debbug streams because they by definition never complete.
    #[inline]
    pub fn try_send_complete(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_debug_stream_v1#{}.complete()\n", client_id, id);
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

    /// server completed the debug stream
    ///
    /// The server has successfully finished writing to and has closed the
    /// associated file descriptor.
    ///
    /// This event is delivered only for one-shot debug streams where the
    /// server dumps some data and stop. This is never delivered for
    /// continuous debbug streams because they by definition never complete.
    #[inline]
    pub fn send_complete(
        &self,
    ) {
        let res = self.try_send_complete(
        );
        if let Err(e) = res {
            log_send("weston_debug_stream_v1.complete", &e);
        }
    }

    /// Since when the failure message is available.
    pub const MSG__FAILURE__SINCE: u32 = 1;

    /// server cannot continue the debug stream
    ///
    /// The server has stopped writing to and has closed the
    /// associated file descriptor. The data already written to the file
    /// descriptor is correct, but it may be truncated.
    ///
    /// This event may be delivered at any time and for any kind of debug
    /// stream. It may be due to a failure in or shutdown of the server.
    /// The message argument may provide a hint of the reason.
    ///
    /// # Arguments
    ///
    /// - `message`: human readable reason
    #[inline]
    pub fn try_send_failure(
        &self,
        message: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            message,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_debug_stream_v1#{}.failure(message: {:?})\n", client_id, id, arg0);
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
        ]);
        if let Some(arg0) = arg0 {
            fmt.string(arg0);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// server cannot continue the debug stream
    ///
    /// The server has stopped writing to and has closed the
    /// associated file descriptor. The data already written to the file
    /// descriptor is correct, but it may be truncated.
    ///
    /// This event may be delivered at any time and for any kind of debug
    /// stream. It may be due to a failure in or shutdown of the server.
    /// The message argument may provide a hint of the reason.
    ///
    /// # Arguments
    ///
    /// - `message`: human readable reason
    #[inline]
    pub fn send_failure(
        &self,
        message: Option<&str>,
    ) {
        let res = self.try_send_failure(
            message,
        );
        if let Err(e) = res {
            log_send("weston_debug_stream_v1.failure", &e);
        }
    }
}

/// A message handler for [`WestonDebugStreamV1`] proxies.
pub trait WestonDebugStreamV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonDebugStreamV1>) {
        slf.core.delete_id();
    }

    /// close a debug stream
    ///
    /// Destroys the object, which causes the server to stop writing into
    /// and closes the associated file descriptor if it was not closed
    /// already.
    ///
    /// Use a wl_display.sync if the clients needs to guarantee the file
    /// descriptor is closed before continuing.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WestonDebugStreamV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("weston_debug_stream_v1.destroy", &e);
        }
    }

    /// server completed the debug stream
    ///
    /// The server has successfully finished writing to and has closed the
    /// associated file descriptor.
    ///
    /// This event is delivered only for one-shot debug streams where the
    /// server dumps some data and stop. This is never delivered for
    /// continuous debbug streams because they by definition never complete.
    #[inline]
    fn handle_complete(
        &mut self,
        slf: &Rc<WestonDebugStreamV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_complete(
        );
        if let Err(e) = res {
            log_forward("weston_debug_stream_v1.complete", &e);
        }
    }

    /// server cannot continue the debug stream
    ///
    /// The server has stopped writing to and has closed the
    /// associated file descriptor. The data already written to the file
    /// descriptor is correct, but it may be truncated.
    ///
    /// This event may be delivered at any time and for any kind of debug
    /// stream. It may be due to a failure in or shutdown of the server.
    /// The message argument may provide a hint of the reason.
    ///
    /// # Arguments
    ///
    /// - `message`: human readable reason
    #[inline]
    fn handle_failure(
        &mut self,
        slf: &Rc<WestonDebugStreamV1>,
        message: Option<&str>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_failure(
            message,
        );
        if let Err(e) = res {
            log_forward("weston_debug_stream_v1.failure", &e);
        }
    }
}

impl ObjectPrivate for WestonDebugStreamV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonDebugStreamV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_debug_stream_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_debug_stream_v1#{}.complete()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_complete(&self);
                } else {
                    DefaultHandler.handle_complete(&self);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NullableString>(msg, offset, "message")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_debug_stream_v1#{}.failure(message: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_failure(&self, arg0);
                } else {
                    DefaultHandler.handle_failure(&self, arg0);
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
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "complete",
            1 => "failure",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WestonDebugStreamV1 {
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

