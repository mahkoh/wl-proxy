//! X primary selection emulation
//!
//! The primary selection device manager is a singleton global object that
//! provides access to the primary selection. It allows to create
//! wp_primary_selection_source objects, as well as retrieving the per-seat
//! wp_primary_selection_device objects.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_primary_selection_device_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpPrimarySelectionDeviceManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpPrimarySelectionDeviceManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpPrimarySelectionDeviceManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpPrimarySelectionDeviceManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpPrimarySelectionDeviceManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpPrimarySelectionDeviceManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpPrimarySelectionDeviceManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpPrimarySelectionDeviceManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpPrimarySelectionDeviceManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpPrimarySelectionDeviceManagerV1 {
    /// Since when the create_source message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_SOURCE__SINCE: u32 = 1;

    /// create a new primary selection source
    ///
    /// Create a new primary selection source.
    #[inline]
    pub fn send_create_source(
        &self,
        id: &Rc<MetaZwpPrimarySelectionSourceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the get_device message is available.
    #[allow(dead_code)]
    pub const MSG__GET_DEVICE__SINCE: u32 = 1;

    /// create a new primary selection device
    ///
    /// Create a new data device for a given seat.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`:
    #[inline]
    pub fn send_get_device(
        &self,
        id: &Rc<MetaZwpPrimarySelectionDeviceV1>,
        seat: &Rc<MetaWlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the primary selection device manager
    ///
    /// Destroy the primary selection device manager.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ZwpPrimarySelectionDeviceManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpPrimarySelectionDeviceManagerV1MessageHandler {
    /// create a new primary selection source
    ///
    /// Create a new primary selection source.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn create_source(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionDeviceManagerV1>,
        id: &Rc<MetaZwpPrimarySelectionSourceV1>,
    ) {
        let res = _slf.send_create_source(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_device_manager_v1.create_source message: {}", Report::new(e));
        }
    }

    /// create a new primary selection device
    ///
    /// Create a new data device for a given seat.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_device(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionDeviceManagerV1>,
        id: &Rc<MetaZwpPrimarySelectionDeviceV1>,
        seat: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_get_device(
            id,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_device_manager_v1.get_device message: {}", Report::new(e));
        }
    }

    /// destroy the primary selection device manager
    ///
    /// Destroy the primary selection device manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpPrimarySelectionDeviceManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_primary_selection_device_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpPrimarySelectionDeviceManagerV1 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpPrimarySelectionSourceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_source(&self, arg0);
                } else {
                    DefaultMessageHandler.create_source(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpPrimarySelectionDeviceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_device(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_device(&self, arg0, arg1);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
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
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "create_source",
            1 => "get_device",
            2 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

