//! manager to control data devices
//!
//! This interface is a manager that allows creating per-seat data device
//! controls.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_data_control_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtDataControlManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtDataControlManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtDataControlManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaExtDataControlManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaExtDataControlManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtDataControlManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaExtDataControlManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaExtDataControlManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtDataControlManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtDataControlManagerV1 {
    /// Since when the create_data_source message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_DATA_SOURCE__SINCE: u32 = 1;

    /// create a new data source
    ///
    /// Create a new data source.
    #[inline]
    pub fn send_create_data_source(
        &self,
        id: &Rc<MetaExtDataControlSourceV1>,
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

    /// Since when the get_data_device message is available.
    #[allow(dead_code)]
    pub const MSG__GET_DATA_DEVICE__SINCE: u32 = 1;

    /// get a data device for a seat
    ///
    /// Create a data device that can be used to manage a seat's selection.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`:
    #[inline]
    pub fn send_get_data_device(
        &self,
        id: &Rc<MetaExtDataControlDeviceV1>,
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

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
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

/// A message handler for [ExtDataControlManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaExtDataControlManagerV1MessageHandler {
    /// create a new data source
    ///
    /// Create a new data source.
    ///
    /// # Arguments
    ///
    /// - `id`: data source to create
    #[inline]
    fn create_data_source(
        &mut self,
        _slf: &Rc<MetaExtDataControlManagerV1>,
        id: &Rc<MetaExtDataControlSourceV1>,
    ) {
        let res = _slf.send_create_data_source(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_manager_v1.create_data_source message: {}", Report::new(e));
        }
    }

    /// get a data device for a seat
    ///
    /// Create a data device that can be used to manage a seat's selection.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `seat`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_data_device(
        &mut self,
        _slf: &Rc<MetaExtDataControlManagerV1>,
        id: &Rc<MetaExtDataControlDeviceV1>,
        seat: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_get_data_device(
            id,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_manager_v1.get_data_device message: {}", Report::new(e));
        }
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtDataControlManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_data_control_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtDataControlManagerV1 {
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
                let arg0 = MetaExtDataControlSourceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_data_source(&self, arg0);
                } else {
                    DefaultMessageHandler.create_data_source(&self, arg0);
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
                let arg0 = MetaExtDataControlDeviceV1::new(&self.core.state, self.core.version);
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
                    (**handler).get_data_device(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_data_device(&self, arg0, arg1);
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
            0 => "create_data_source",
            1 => "get_data_device",
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

