//! context object for input timestamps
//!
//! Provides high-resolution timestamp events for a set of subscribed input
//! events. The set of subscribed input events is determined by the
//! zwp_input_timestamps_manager_v1 request used to create this object.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_input_timestamps_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpInputTimestampsV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpInputTimestampsV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpInputTimestampsV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpInputTimestampsV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpInputTimestampsV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpInputTimestampsV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpInputTimestampsV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpInputTimestampsV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpInputTimestampsV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the input timestamps object
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. After the server processes the request, no more
    /// timestamp events will be emitted.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= zwp_input_timestamps_v1#{}.destroy()", id);
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

    /// Since when the timestamp message is available.
    #[allow(dead_code)]
    pub const MSG__TIMESTAMP__SINCE: u32 = 1;

    /// high-resolution timestamp event
    ///
    /// The timestamp event is associated with the first subsequent input event
    /// carrying a timestamp which belongs to the set of input events this
    /// object is subscribed to.
    ///
    /// The timestamp provided by this event is a high-resolution version of
    /// the timestamp argument of the associated input event. The provided
    /// timestamp is in the same clock domain and is at least as accurate as
    /// the associated input event timestamp.
    ///
    /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
    /// each component being an unsigned 32-bit value. Whole seconds are in
    /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
    /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999].
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the timestamp
    /// - `tv_nsec`: nanoseconds part of the timestamp
    #[inline]
    pub fn send_timestamp(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= zwp_input_timestamps_v1#{}.timestamp(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})", client.endpoint.id, id, arg0, arg1, arg2);
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
            arg1,
            arg2,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpInputTimestampsV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpInputTimestampsV1MessageHandler {
    /// destroy the input timestamps object
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object. After the server processes the request, no more
    /// timestamp events will be emitted.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpInputTimestampsV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_timestamps_v1.destroy message: {}", Report::new(e));
        }
    }

    /// high-resolution timestamp event
    ///
    /// The timestamp event is associated with the first subsequent input event
    /// carrying a timestamp which belongs to the set of input events this
    /// object is subscribed to.
    ///
    /// The timestamp provided by this event is a high-resolution version of
    /// the timestamp argument of the associated input event. The provided
    /// timestamp is in the same clock domain and is at least as accurate as
    /// the associated input event timestamp.
    ///
    /// The timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,
    /// each component being an unsigned 32-bit value. Whole seconds are in
    /// tv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,
    /// and the additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999].
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the timestamp
    /// - `tv_nsec`: nanoseconds part of the timestamp
    #[inline]
    fn timestamp(
        &mut self,
        _slf: &Rc<MetaZwpInputTimestampsV1>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) {
        let res = _slf.send_timestamp(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_input_timestamps_v1.timestamp message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpInputTimestampsV1 {
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
                eprintln!("client#{:04} -> zwp_input_timestamps_v1#{}.destroy()", client.endpoint.id, msg[0]);
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
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                eprintln!("server      -> zwp_input_timestamps_v1#{}.timestamp(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})", msg[0], arg0, arg1, arg2);
                if let Some(handler) = handler {
                    (**handler).timestamp(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.timestamp(&self, arg0, arg1, arg2);
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
            0 => "timestamp",
            _ => return None,
        };
        Some(name)
    }
}

