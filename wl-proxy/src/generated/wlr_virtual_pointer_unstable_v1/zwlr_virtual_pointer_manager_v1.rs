//! virtual pointer manager
//!
//! This object allows clients to create individual virtual pointer objects.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_virtual_pointer_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrVirtualPointerManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrVirtualPointerManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrVirtualPointerManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrVirtualPointerManagerV1 {
    pub const XML_VERSION: u32 = 2;
}

impl MetaZwlrVirtualPointerManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrVirtualPointerManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwlrVirtualPointerManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwlrVirtualPointerManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrVirtualPointerManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrVirtualPointerManagerV1 {
    /// Since when the create_virtual_pointer message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_VIRTUAL_POINTER__SINCE: u32 = 1;

    /// Create a new virtual pointer
    ///
    /// Creates a new virtual pointer. The optional seat is a suggestion to the
    /// compositor.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `id`:
    #[inline]
    pub fn send_create_virtual_pointer(
        &self,
        seat: Option<&Rc<MetaWlSeat>>,
        id: &Rc<MetaZwlrVirtualPointerV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            seat,
            id,
        );
        let arg0 = arg0.map(|a| a.core());
        let arg1_obj = arg1;
        let arg1 = arg1_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("seat")),
                Some(id) => id,
            },
        };
        arg1.generate_server_id(arg1_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg1_id = arg1.server_obj_id.get().unwrap_or(0);
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

    /// destroy the virtual pointer manager
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the create_virtual_pointer_with_output message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_VIRTUAL_POINTER_WITH_OUTPUT__SINCE: u32 = 2;

    /// Create a new virtual pointer
    ///
    /// Creates a new virtual pointer. The seat and the output arguments are
    /// optional. If the seat argument is set, the compositor should assign the
    /// input device to the requested seat. If the output argument is set, the
    /// compositor should map the input device to the requested output.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `output`:
    /// - `id`:
    #[inline]
    pub fn send_create_virtual_pointer_with_output(
        &self,
        seat: Option<&Rc<MetaWlSeat>>,
        output: Option<&Rc<MetaWlOutput>>,
        id: &Rc<MetaZwlrVirtualPointerV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            seat,
            output,
            id,
        );
        let arg0 = arg0.map(|a| a.core());
        let arg1 = arg1.map(|a| a.core());
        let arg2_obj = arg2;
        let arg2 = arg2_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("seat")),
                Some(id) => id,
            },
        };
        let arg1_id = match arg1 {
            None => 0,
            Some(arg1) => match arg1.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("output")),
                Some(id) => id,
            },
        };
        arg2.generate_server_id(arg2_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg2_id = arg2.server_obj_id.get().unwrap_or(0);
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
            arg0_id,
            arg1_id,
            arg2_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwlrVirtualPointerManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrVirtualPointerManagerV1MessageHandler {
    /// Create a new virtual pointer
    ///
    /// Creates a new virtual pointer. The optional seat is a suggestion to the
    /// compositor.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `id`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn create_virtual_pointer(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerManagerV1>,
        seat: Option<&Rc<MetaWlSeat>>,
        id: &Rc<MetaZwlrVirtualPointerV1>,
    ) {
        let res = _slf.send_create_virtual_pointer(
            seat,
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_manager_v1.create_virtual_pointer message: {}", Report::new(e));
        }
    }

    /// destroy the virtual pointer manager
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// Create a new virtual pointer
    ///
    /// Creates a new virtual pointer. The seat and the output arguments are
    /// optional. If the seat argument is set, the compositor should assign the
    /// input device to the requested seat. If the output argument is set, the
    /// compositor should map the input device to the requested output.
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `output`:
    /// - `id`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn create_virtual_pointer_with_output(
        &mut self,
        _slf: &Rc<MetaZwlrVirtualPointerManagerV1>,
        seat: Option<&Rc<MetaWlSeat>>,
        output: Option<&Rc<MetaWlOutput>>,
        id: &Rc<MetaZwlrVirtualPointerV1>,
    ) {
        let res = _slf.send_create_virtual_pointer_with_output(
            seat,
            output,
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_virtual_pointer_manager_v1.create_virtual_pointer_with_output message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrVirtualPointerManagerV1 {
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                    };
                    Some(arg0)
                };
                let arg1_id = arg1;
                let arg1 = MetaZwlrVirtualPointerV1::new(&self.core.state, self.core.version);
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError::SetClientId(arg1_id, "id", e))?;
                let arg0 = arg0.as_ref();
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).create_virtual_pointer(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.create_virtual_pointer(&self, arg0, arg1);
                }
            }
            1 => {
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
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                    };
                    Some(arg0)
                };
                let arg1 = if arg1 == 0 {
                    None
                } else {
                    let arg1_id = arg1;
                    let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                    };
                    let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                        let o = client.endpoint.lookup(arg1_id).unwrap();
                        return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                    };
                    Some(arg1)
                };
                let arg2_id = arg2;
                let arg2 = MetaZwlrVirtualPointerV1::new(&self.core.state, self.core.version);
                arg2.core().set_client_id(client, arg2_id, arg2.clone())
                    .map_err(|e| ObjectError::SetClientId(arg2_id, "id", e))?;
                let arg0 = arg0.as_ref();
                let arg1 = arg1.as_ref();
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).create_virtual_pointer_with_output(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.create_virtual_pointer_with_output(&self, arg0, arg1, arg2);
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
            0 => "create_virtual_pointer",
            1 => "destroy",
            2 => "create_virtual_pointer_with_output",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

