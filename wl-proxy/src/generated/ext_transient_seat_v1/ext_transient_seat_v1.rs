//! transient seat handle
//!
//! When the transient seat handle is destroyed, the seat itself will also be
//! destroyed.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_transient_seat_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtTransientSeatV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtTransientSeatV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtTransientSeatV1MessageHandler for DefaultMessageHandler { }

impl MetaExtTransientSeatV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaExtTransientSeatV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtTransientSeatV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtTransientSeatV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtTransientSeatV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtTransientSeatV1 {
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
        eprintln!("client#{:04} <= ext_transient_seat_v1#{}.ready(global_name: {})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
        eprintln!("client#{:04} <= ext_transient_seat_v1#{}.denied()", client.endpoint.id, id);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
        eprintln!("server      <= ext_transient_seat_v1#{}.destroy()", id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
pub trait MetaExtTransientSeatV1MessageHandler {
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
        _slf: &Rc<MetaExtTransientSeatV1>,
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
        _slf: &Rc<MetaExtTransientSeatV1>,
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
        _slf: &Rc<MetaExtTransientSeatV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_transient_seat_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtTransientSeatV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> ext_transient_seat_v1#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("server      -> ext_transient_seat_v1#{}.ready(global_name: {})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).ready(&self, arg0);
                } else {
                    DefaultMessageHandler.ready(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> ext_transient_seat_v1#{}.denied()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).denied(&self);
                } else {
                    DefaultMessageHandler.denied(&self);
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

