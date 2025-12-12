//! virtual pointer manager
//!
//! This object allows clients to create individual virtual pointer objects.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_virtual_pointer_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrVirtualPointerManagerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZwlrVirtualPointerManagerV1Handler>,
}

struct DefaultHandler;

impl ZwlrVirtualPointerManagerV1Handler for DefaultHandler { }

impl ZwlrVirtualPointerManagerV1 {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: ProxyInterface = ProxyInterface::ZwlrVirtualPointerManagerV1;
    pub const INTERFACE_NAME: &str = "zwlr_virtual_pointer_manager_v1";
}

impl ZwlrVirtualPointerManagerV1 {
    pub fn set_handler(&self, handler: impl ZwlrVirtualPointerManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrVirtualPointerManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrVirtualPointerManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrVirtualPointerManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrVirtualPointerManagerV1 {
    /// Since when the create_virtual_pointer message is available.
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
        seat: Option<&Rc<WlSeat>>,
        id: &Rc<ZwlrVirtualPointerV1>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_manager_v1#{}.create_virtual_pointer(seat: wl_seat#{}, id: zwlr_virtual_pointer_v1#{})\n", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_manager_v1#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the create_virtual_pointer_with_output message is available.
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
        seat: Option<&Rc<WlSeat>>,
        output: Option<&Rc<WlOutput>>,
        id: &Rc<ZwlrVirtualPointerV1>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_virtual_pointer_manager_v1#{}.create_virtual_pointer_with_output(seat: wl_seat#{}, output: wl_output#{}, id: zwlr_virtual_pointer_v1#{})\n", id, arg0_id, arg1_id, arg2_id);
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
            2,
            arg0_id,
            arg1_id,
            arg2_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwlrVirtualPointerManagerV1] proxies.
pub trait ZwlrVirtualPointerManagerV1Handler: Any {
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
        _slf: &Rc<ZwlrVirtualPointerManagerV1>,
        seat: Option<&Rc<WlSeat>>,
        id: &Rc<ZwlrVirtualPointerV1>,
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
        _slf: &Rc<ZwlrVirtualPointerManagerV1>,
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
        _slf: &Rc<ZwlrVirtualPointerManagerV1>,
        seat: Option<&Rc<WlSeat>>,
        output: Option<&Rc<WlOutput>>,
        id: &Rc<ZwlrVirtualPointerV1>,
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

impl ProxyPrivate for ZwlrVirtualPointerManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZwlrVirtualPointerManagerV1, version),
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
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_manager_v1#{}.create_virtual_pointer(seat: wl_seat#{}, id: zwlr_virtual_pointer_v1#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                    };
                    Some(arg0)
                };
                let arg1_id = arg1;
                let arg1 = ZwlrVirtualPointerV1::new(&self.core.state, self.core.version);
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError::SetClientId(arg1_id, "id", e))?;
                let arg0 = arg0.as_ref();
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).create_virtual_pointer(&self, arg0, arg1);
                } else {
                    DefaultHandler.create_virtual_pointer(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_virtual_pointer_manager_v1#{}.create_virtual_pointer_with_output(seat: wl_seat#{}, output: wl_output#{}, id: zwlr_virtual_pointer_v1#{})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
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
                    let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlOutput>() else {
                        let o = client.endpoint.lookup(arg1_id).unwrap();
                        return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                    };
                    Some(arg1)
                };
                let arg2_id = arg2;
                let arg2 = ZwlrVirtualPointerV1::new(&self.core.state, self.core.version);
                arg2.core().set_client_id(client, arg2_id, arg2.clone())
                    .map_err(|e| ObjectError::SetClientId(arg2_id, "id", e))?;
                let arg0 = arg0.as_ref();
                let arg1 = arg1.as_ref();
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).create_virtual_pointer_with_output(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.create_virtual_pointer_with_output(&self, arg0, arg1, arg2);
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
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

impl Proxy for ZwlrVirtualPointerManagerV1 {
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

