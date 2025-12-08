//! protocol for message synchronization
//!
//! This protocol can be used by wl-proxy applications to synchronize state
//! between their own wayland client and the proxy.
//!
//! Compositors should not implement this interface directly.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wlproxy_sync_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlproxySyncV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlproxySyncV1Handler>,
}

struct DefaultHandler;

impl WlproxySyncV1Handler for DefaultHandler { }

impl ConcreteObject for WlproxySyncV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WlproxySyncV1;
    const INTERFACE_NAME: &str = "wlproxy_sync_v1";
}

impl WlproxySyncV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlproxySyncV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlproxySyncV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlproxySyncV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlproxySyncV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlproxySyncV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this object
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_sync_v1#{}.destroy()\n", id);
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

    /// destroy this object
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wlproxy_sync_v1.destroy", &e);
        }
    }

    /// Since when the sync_with_proxy message is available.
    pub const MSG__SYNC_WITH_PROXY__SINCE: u32 = 1;

    /// synchronize from the client to the proxy
    ///
    /// This request can be used to synchronize messages between the client
    /// and the proxy.
    ///
    /// The id_hi and id_lo arguments form an opaque 64-bit number.
    ///
    /// # Arguments
    ///
    /// - `id_hi`: upper 32 bits of the id
    /// - `id_lo`: lower 32 bits of the id
    #[inline]
    pub fn try_send_sync_with_proxy(
        &self,
        id_hi: u32,
        id_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id_hi,
            id_lo,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wlproxy_sync_v1#{}.sync_with_proxy(id_hi: {}, id_lo: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
            arg1,
        ]);
        Ok(())
    }

    /// synchronize from the client to the proxy
    ///
    /// This request can be used to synchronize messages between the client
    /// and the proxy.
    ///
    /// The id_hi and id_lo arguments form an opaque 64-bit number.
    ///
    /// # Arguments
    ///
    /// - `id_hi`: upper 32 bits of the id
    /// - `id_lo`: lower 32 bits of the id
    #[inline]
    pub fn send_sync_with_proxy(
        &self,
        id_hi: u32,
        id_lo: u32,
    ) {
        let res = self.try_send_sync_with_proxy(
            id_hi,
            id_lo,
        );
        if let Err(e) = res {
            log_send("wlproxy_sync_v1.sync_with_proxy", &e);
        }
    }

    /// Since when the sync_with_client message is available.
    pub const MSG__SYNC_WITH_CLIENT__SINCE: u32 = 1;

    /// synchronize from the proxy to the client
    ///
    /// This event can be used to synchronize messages between the client
    /// and the proxy.
    ///
    /// The id_hi and id_lo arguments form an opaque 64-bit number.
    ///
    /// # Arguments
    ///
    /// - `id_hi`: upper 32 bits of the id
    /// - `id_lo`: lower 32 bits of the id
    #[inline]
    pub fn try_send_sync_with_client(
        &self,
        id_hi: u32,
        id_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id_hi,
            id_lo,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wlproxy_sync_v1#{}.sync_with_client(id_hi: {}, id_lo: {})\n", client_id, id, arg0, arg1);
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
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// synchronize from the proxy to the client
    ///
    /// This event can be used to synchronize messages between the client
    /// and the proxy.
    ///
    /// The id_hi and id_lo arguments form an opaque 64-bit number.
    ///
    /// # Arguments
    ///
    /// - `id_hi`: upper 32 bits of the id
    /// - `id_lo`: lower 32 bits of the id
    #[inline]
    pub fn send_sync_with_client(
        &self,
        id_hi: u32,
        id_lo: u32,
    ) {
        let res = self.try_send_sync_with_client(
            id_hi,
            id_lo,
        );
        if let Err(e) = res {
            log_send("wlproxy_sync_v1.sync_with_client", &e);
        }
    }
}

/// A message handler for [`WlproxySyncV1`] proxies.
pub trait WlproxySyncV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlproxySyncV1>) {
        slf.core.delete_id();
    }

    /// destroy this object
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WlproxySyncV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wlproxy_sync_v1.destroy", &e);
        }
    }

    /// synchronize from the client to the proxy
    ///
    /// This request can be used to synchronize messages between the client
    /// and the proxy.
    ///
    /// The id_hi and id_lo arguments form an opaque 64-bit number.
    ///
    /// # Arguments
    ///
    /// - `id_hi`: upper 32 bits of the id
    /// - `id_lo`: lower 32 bits of the id
    #[inline]
    fn handle_sync_with_proxy(
        &mut self,
        slf: &Rc<WlproxySyncV1>,
        id_hi: u32,
        id_lo: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_sync_with_proxy(
            id_hi,
            id_lo,
        );
        if let Err(e) = res {
            log_forward("wlproxy_sync_v1.sync_with_proxy", &e);
        }
    }

    /// synchronize from the proxy to the client
    ///
    /// This event can be used to synchronize messages between the client
    /// and the proxy.
    ///
    /// The id_hi and id_lo arguments form an opaque 64-bit number.
    ///
    /// # Arguments
    ///
    /// - `id_hi`: upper 32 bits of the id
    /// - `id_lo`: lower 32 bits of the id
    #[inline]
    fn handle_sync_with_client(
        &mut self,
        slf: &Rc<WlproxySyncV1>,
        id_hi: u32,
        id_lo: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_sync_with_client(
            id_hi,
            id_lo,
        );
        if let Err(e) = res {
            log_forward("wlproxy_sync_v1.sync_with_client", &e);
        }
    }
}

impl ObjectPrivate for WlproxySyncV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlproxySyncV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_sync_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wlproxy_sync_v1#{}.sync_with_proxy(id_hi: {}, id_lo: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_sync_with_proxy(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_sync_with_proxy(&self, arg0, arg1);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wlproxy_sync_v1#{}.sync_with_client(id_hi: {}, id_lo: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_sync_with_client(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_sync_with_client(&self, arg0, arg1);
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
            1 => "sync_with_proxy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "sync_with_client",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlproxySyncV1 {
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

