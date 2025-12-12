//! toplevel to window mapping
//!
//! This object represents a mapping of a (wlr) toplevel to a window address.
//!
//! Once created, the `window_address` event will be sent containing the address of the window
//! associated with the toplevel.
//! Should the mapping fail, the `failed` event will be sent.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_toplevel_window_mapping_handle_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandToplevelWindowMappingHandleV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandToplevelWindowMappingHandleV1Handler>,
}

struct DefaultHandler;

impl HyprlandToplevelWindowMappingHandleV1Handler for DefaultHandler { }

impl HyprlandToplevelWindowMappingHandleV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::HyprlandToplevelWindowMappingHandleV1;
    pub const INTERFACE_NAME: &str = "hyprland_toplevel_window_mapping_handle_v1";
}

impl HyprlandToplevelWindowMappingHandleV1 {
    pub fn set_handler(&self, handler: impl HyprlandToplevelWindowMappingHandleV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandToplevelWindowMappingHandleV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandToplevelWindowMappingHandleV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandToplevelWindowMappingHandleV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandToplevelWindowMappingHandleV1 {
    /// Since when the window_address message is available.
    pub const MSG__WINDOW_ADDRESS__SINCE: u32 = 1;

    /// address of the window
    ///
    /// The full 64bit window address. The `address` field contains the lower 32 bits whilst the
    /// `address_hi` contains the upper 32 bits
    ///
    /// # Arguments
    ///
    /// - `address_hi`: upper 32 bits of the window address
    /// - `address`: lower 32 bits of the window address
    #[inline]
    pub fn send_window_address(
        &self,
        address_hi: u32,
        address: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            address_hi,
            address,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_toplevel_window_mapping_handle_v1#{}.window_address(address_hi: {}, address: {})\n", client.endpoint.id, id, arg0, arg1);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the failed message is available.
    pub const MSG__FAILED__SINCE: u32 = 1;

    /// mapping failed
    ///
    /// The mapping of the toplevel to a window address failed. Most likely the window does not
    /// exist (anymore).
    #[inline]
    pub fn send_failed(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_toplevel_window_mapping_handle_v1#{}.failed()\n", client.endpoint.id, id);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
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
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy handle
    ///
    /// Destroy the handle. This request can be sent at any time by the client.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_toplevel_window_mapping_handle_v1#{}.destroy()\n", id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [HyprlandToplevelWindowMappingHandleV1] proxies.
pub trait HyprlandToplevelWindowMappingHandleV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandToplevelWindowMappingHandleV1>) {
        let _ = slf.core.delete_id();
    }

    /// address of the window
    ///
    /// The full 64bit window address. The `address` field contains the lower 32 bits whilst the
    /// `address_hi` contains the upper 32 bits
    ///
    /// # Arguments
    ///
    /// - `address_hi`: upper 32 bits of the window address
    /// - `address`: lower 32 bits of the window address
    #[inline]
    fn handle_window_address(
        &mut self,
        _slf: &Rc<HyprlandToplevelWindowMappingHandleV1>,
        address_hi: u32,
        address: u32,
    ) {
        let res = _slf.send_window_address(
            address_hi,
            address,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_toplevel_window_mapping_handle_v1.window_address message: {}", Report::new(e));
        }
    }

    /// mapping failed
    ///
    /// The mapping of the toplevel to a window address failed. Most likely the window does not
    /// exist (anymore).
    #[inline]
    fn handle_failed(
        &mut self,
        _slf: &Rc<HyprlandToplevelWindowMappingHandleV1>,
    ) {
        let res = _slf.send_failed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_toplevel_window_mapping_handle_v1.failed message: {}", Report::new(e));
        }
    }

    /// destroy handle
    ///
    /// Destroy the handle. This request can be sent at any time by the client.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<HyprlandToplevelWindowMappingHandleV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_toplevel_window_mapping_handle_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for HyprlandToplevelWindowMappingHandleV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandToplevelWindowMappingHandleV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_toplevel_window_mapping_handle_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_toplevel_window_mapping_handle_v1#{}.window_address(address_hi: {}, address: {})\n", msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_window_address(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_window_address(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_toplevel_window_mapping_handle_v1#{}.failed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_failed(&self);
                } else {
                    DefaultHandler.handle_failed(&self);
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
            0 => "window_address",
            1 => "failed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for HyprlandToplevelWindowMappingHandleV1 {
    fn core(&self) -> &ObjectCore {
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

