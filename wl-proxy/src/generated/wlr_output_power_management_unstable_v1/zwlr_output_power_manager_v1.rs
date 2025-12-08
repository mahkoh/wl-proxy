//! manager to create per-output power management
//!
//! This interface is a manager that allows creating per-output power
//! management mode controls.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_power_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrOutputPowerManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrOutputPowerManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrOutputPowerManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrOutputPowerManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwlrOutputPowerManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrOutputPowerManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrOutputPowerManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrOutputPowerManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrOutputPowerManagerV1 {
    /// Since when the get_output_power message is available.
    #[allow(dead_code)]
    pub const MSG__GET_OUTPUT_POWER__SINCE: u32 = 1;

    /// get a power management for an output
    ///
    /// Create an output power management mode control that can be used to
    /// adjust the power management mode for a given output.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    #[inline]
    pub fn send_get_output_power(
        &self,
        id: &Rc<MetaZwlrOutputPowerV1>,
        output: &Rc<MetaWlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            output,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("output")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        eprintln!("server      <= zwlr_output_power_manager_v1#{}.get_output_power(id: zwlr_output_power_v1#{}, output: wl_output#{})", id, arg0_id, arg1_id);
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
        eprintln!("server      <= zwlr_output_power_manager_v1#{}.destroy()", id);
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

/// A message handler for [ZwlrOutputPowerManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrOutputPowerManagerV1MessageHandler {
    /// get a power management for an output
    ///
    /// Create an output power management mode control that can be used to
    /// adjust the power management mode for a given output.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_output_power(
        &mut self,
        _slf: &Rc<MetaZwlrOutputPowerManagerV1>,
        id: &Rc<MetaZwlrOutputPowerV1>,
        output: &Rc<MetaWlOutput>,
    ) {
        let res = _slf.send_get_output_power(
            id,
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_power_manager_v1.get_output_power message: {}", Report::new(e));
        }
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrOutputPowerManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_power_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrOutputPowerManagerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                eprintln!("client#{:04} -> zwlr_output_power_manager_v1#{}.get_output_power(id: zwlr_output_power_v1#{}, output: wl_output#{})", client.endpoint.id, msg[0], arg0, arg1);
                let arg0_id = arg0;
                let arg0 = MetaZwlrOutputPowerV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_output_power(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_output_power(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> zwlr_output_power_manager_v1#{}.destroy()", client.endpoint.id, msg[0]);
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
            0 => "get_output_power",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

