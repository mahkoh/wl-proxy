//! manager to control data devices
//!
//! This interface is a manager that allows creating per-seat data device
//! controls.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_data_control_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrDataControlManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrDataControlManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrDataControlManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrDataControlManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrDataControlManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrDataControlManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrDataControlManagerV1 {
    /// Since when the create_data_source message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_DATA_SOURCE__SINCE: u32 = 1;

    /// create a new data source
    ///
    /// Create a new data source.
    #[inline]
    pub fn send_create_data_source(
        &self,
        id: &Rc<MetaZwlrDataControlSourceV1>,
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
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.server_obj_id.get().unwrap_or(0),
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
        id: &Rc<MetaZwlrDataControlDeviceV1>,
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwlrDataControlManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrDataControlManagerV1MessageHandler {
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
        _slf: &Rc<MetaZwlrDataControlManagerV1>,
        id: &Rc<MetaZwlrDataControlSourceV1>,
    ) {
        let res = _slf.send_create_data_source(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_manager_v1.create_data_source message: {}", Report::new(e));
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
        _slf: &Rc<MetaZwlrDataControlManagerV1>,
        id: &Rc<MetaZwlrDataControlDeviceV1>,
        seat: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_get_data_device(
            id,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_manager_v1.get_data_device message: {}", Report::new(e));
        }
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrDataControlManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_data_control_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrDataControlManagerV1 {
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
                let arg0 = MetaZwlrDataControlSourceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
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
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwlrDataControlDeviceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    return Err(ObjectError);
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
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
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

