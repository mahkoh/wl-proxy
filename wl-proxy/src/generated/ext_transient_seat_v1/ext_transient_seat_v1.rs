//! transient seat handle
//!
//! When the transient seat handle is destroyed, the seat itself will also be
//! destroyed.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_transient_seat_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtTransientSeatV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ExtTransientSeatV1Handler>,
}

struct DefaultHandler;

impl ExtTransientSeatV1Handler for DefaultHandler { }

impl ExtTransientSeatV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "ext_transient_seat_v1";
}

impl ExtTransientSeatV1 {
    pub fn set_handler(&self, handler: impl ExtTransientSeatV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ExtTransientSeatV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtTransientSeatV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtTransientSeatV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtTransientSeatV1 {
    /// Since when the ready message is available.
    #[allow(dead_code)]
    pub const MSG__READY__SINCE: u32 = 1;

    /// transient seat is ready
    ///
    /// This event advertises the global name for the wl_seat to be used with
    /// wl_registry_bind.
    ///
    /// It is sent exactly once, immediately after the transient seat is created
    /// and the new "wl_seat" global is advertised, if and only if the creation
    /// of the transient seat was allowed.
    ///
    /// # Arguments
    ///
    /// - `global_name`:
    #[inline]
    pub fn send_ready(
        &self,
        global_name: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            global_name,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_transient_seat_v1#{}.ready(global_name: {})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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
        Ok(())
    }

    /// Since when the denied message is available.
    #[allow(dead_code)]
    pub const MSG__DENIED__SINCE: u32 = 1;

    /// transient seat creation denied
    ///
    /// The event informs the client that the compositor denied its request to
    /// create a transient seat.
    ///
    /// It is sent exactly once, immediately after the transient seat object is
    /// created, if and only if the creation of the transient seat was denied.
    ///
    /// After receiving this event, the client should destroy the object.
    #[inline]
    pub fn send_denied(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_transient_seat_v1#{}.denied()\n", client.endpoint.id, id);
            self.core.state.log(args);
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
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy transient seat
    ///
    /// When the transient seat object is destroyed by the client, the
    /// associated seat created by the compositor is also destroyed.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_transient_seat_v1#{}.destroy()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
}

/// A message handler for [ExtTransientSeatV1] proxies.
#[allow(dead_code)]
pub trait ExtTransientSeatV1Handler: Any {
    /// transient seat is ready
    ///
    /// This event advertises the global name for the wl_seat to be used with
    /// wl_registry_bind.
    ///
    /// It is sent exactly once, immediately after the transient seat is created
    /// and the new "wl_seat" global is advertised, if and only if the creation
    /// of the transient seat was allowed.
    ///
    /// # Arguments
    ///
    /// - `global_name`:
    #[inline]
    fn ready(
        &mut self,
        _slf: &Rc<ExtTransientSeatV1>,
        global_name: u32,
    ) {
        let res = _slf.send_ready(
            global_name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_transient_seat_v1.ready message: {}", Report::new(e));
        }
    }

    /// transient seat creation denied
    ///
    /// The event informs the client that the compositor denied its request to
    /// create a transient seat.
    ///
    /// It is sent exactly once, immediately after the transient seat object is
    /// created, if and only if the creation of the transient seat was denied.
    ///
    /// After receiving this event, the client should destroy the object.
    #[inline]
    fn denied(
        &mut self,
        _slf: &Rc<ExtTransientSeatV1>,
    ) {
        let res = _slf.send_denied(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_transient_seat_v1.denied message: {}", Report::new(e));
        }
    }

    /// destroy transient seat
    ///
    /// When the transient seat object is destroyed by the client, the
    /// associated seat created by the compositor is also destroyed.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ExtTransientSeatV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_transient_seat_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ExtTransientSeatV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ExtTransientSeatV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_transient_seat_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_transient_seat_v1#{}.ready(global_name: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).ready(&self, arg0);
                } else {
                    DefaultHandler.ready(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_transient_seat_v1#{}.denied()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).denied(&self);
                } else {
                    DefaultHandler.denied(&self);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
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
            0 => "ready",
            1 => "denied",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for ExtTransientSeatV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

