//! manage xdg_output objects
//!
//! A global factory interface for xdg_output objects.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zxdg_output_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZxdgOutputManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZxdgOutputManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZxdgOutputManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZxdgOutputManagerV1 {
    pub const XML_VERSION: u32 = 3;
}

impl MetaZxdgOutputManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZxdgOutputManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZxdgOutputManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZxdgOutputManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZxdgOutputManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_output_manager object
    ///
    /// Using this request a client can tell the server that it is not
    /// going to use the xdg_output_manager object anymore.
    ///
    /// Any objects already created through this instance are not affected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= zxdg_output_manager_v1#{}.destroy()", id);
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

    /// Since when the get_xdg_output message is available.
    #[allow(dead_code)]
    pub const MSG__GET_XDG_OUTPUT__SINCE: u32 = 1;

    /// create an xdg output from a wl_output
    ///
    /// This creates a new xdg_output object for the given wl_output.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    #[inline]
    pub fn send_get_xdg_output(
        &self,
        id: &Rc<MetaZxdgOutputV1>,
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
        eprintln!("server      <= zxdg_output_manager_v1#{}.get_xdg_output(id: zxdg_output_v1#{}, output: wl_output#{})", id, arg0_id, arg1_id);
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
}

/// A message handler for [ZxdgOutputManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZxdgOutputManagerV1MessageHandler {
    /// destroy the xdg_output_manager object
    ///
    /// Using this request a client can tell the server that it is not
    /// going to use the xdg_output_manager object anymore.
    ///
    /// Any objects already created through this instance are not affected.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZxdgOutputManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_output_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create an xdg output from a wl_output
    ///
    /// This creates a new xdg_output object for the given wl_output.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_xdg_output(
        &mut self,
        _slf: &Rc<MetaZxdgOutputManagerV1>,
        id: &Rc<MetaZxdgOutputV1>,
        output: &Rc<MetaWlOutput>,
    ) {
        let res = _slf.send_get_xdg_output(
            id,
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_output_manager_v1.get_xdg_output message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZxdgOutputManagerV1 {
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
                eprintln!("client#{:04} -> zxdg_output_manager_v1#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                eprintln!("client#{:04} -> zxdg_output_manager_v1#{}.get_xdg_output(id: zxdg_output_v1#{}, output: wl_output#{})", client.endpoint.id, msg[0], arg0, arg1);
                let arg0_id = arg0;
                let arg0 = MetaZxdgOutputV1::new(&self.core.state, self.core.version);
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
                    (**handler).get_xdg_output(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_xdg_output(&self, arg0, arg1);
                }
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
            0 => "destroy",
            1 => "get_xdg_output",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

