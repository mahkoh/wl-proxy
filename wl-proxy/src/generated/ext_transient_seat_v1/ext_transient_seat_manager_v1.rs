//! transient seat manager
//!
//! The transient seat manager creates short-lived seats.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_transient_seat_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtTransientSeatManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtTransientSeatManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtTransientSeatManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaExtTransientSeatManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaExtTransientSeatManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtTransientSeatManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtTransientSeatManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtTransientSeatManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtTransientSeatManagerV1 {
    /// Since when the create message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE__SINCE: u32 = 1;

    /// create a transient seat
    ///
    /// Create a new seat that is removed when the client side transient seat
    /// object is destroyed.
    ///
    /// The actual seat may be removed sooner, in which case the transient seat
    /// object shall become inert.
    #[inline]
    pub fn send_create(
        &self,
        seat: &Rc<MetaExtTransientSeatV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
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
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the manager.
    ///
    /// All objects created by the manager will remain valid until they are
    /// destroyed themselves.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
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
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ExtTransientSeatManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaExtTransientSeatManagerV1MessageHandler {
    /// create a transient seat
    ///
    /// Create a new seat that is removed when the client side transient seat
    /// object is destroyed.
    ///
    /// The actual seat may be removed sooner, in which case the transient seat
    /// object shall become inert.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    #[inline]
    fn create(
        &mut self,
        _slf: &Rc<MetaExtTransientSeatManagerV1>,
        seat: &Rc<MetaExtTransientSeatV1>,
    ) {
        let res = _slf.send_create(
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_transient_seat_manager_v1.create message: {}", Report::new(e));
        }
    }

    /// destroy the manager
    ///
    /// Destroy the manager.
    ///
    /// All objects created by the manager will remain valid until they are
    /// destroyed themselves.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtTransientSeatManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_transient_seat_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtTransientSeatManagerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaExtTransientSeatV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create(&self, arg0);
                } else {
                    DefaultMessageHandler.create(&self, arg0);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

