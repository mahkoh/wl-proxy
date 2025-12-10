//! window decoration manager
//!
//! This interface allows a compositor to announce support for server-side
//! decorations.
//!
//! A window decoration is a set of window controls as deemed appropriate by
//! the party managing them, such as user interface components used to move,
//! resize and change a window's state.
//!
//! A client can use this protocol to request being decorated by a supporting
//! compositor.
//!
//! If compositor and client do not negotiate the use of a server-side
//! decoration using this protocol, clients continue to self-decorate as they
//! see fit.
//!
//! Warning! The protocol described in this file is experimental and
//! backward incompatible changes may be made. Backward compatible changes
//! may be added together with the corresponding interface version bump.
//! Backward incompatible changes are done by bumping the version number in
//! the protocol and interface names and resetting the interface version.
//! Once the protocol is to be declared stable, the 'z' prefix and the
//! version number in the protocol and interface names are removed and the
//! interface version number is reset.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zxdg_decoration_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZxdgDecorationManagerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ZxdgDecorationManagerV1Handler>,
}

struct DefaultHandler;

impl ZxdgDecorationManagerV1Handler for DefaultHandler { }

impl ZxdgDecorationManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "zxdg_decoration_manager_v1";
}

impl ZxdgDecorationManagerV1 {
    pub fn set_handler(&self, handler: impl ZxdgDecorationManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZxdgDecorationManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZxdgDecorationManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZxdgDecorationManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZxdgDecorationManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the decoration manager object
    ///
    /// Destroy the decoration manager. This doesn't destroy objects created
    /// with the manager.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_decoration_manager_v1#{}.destroy()\n", id);
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

    /// Since when the get_toplevel_decoration message is available.
    #[allow(dead_code)]
    pub const MSG__GET_TOPLEVEL_DECORATION__SINCE: u32 = 1;

    /// create a new toplevel decoration object
    ///
    /// Create a new decoration object associated with the given toplevel.
    ///
    /// Creating an xdg_toplevel_decoration from an xdg_toplevel which has a
    /// buffer attached or committed is a client error, and any attempts by a
    /// client to attach or manipulate a buffer prior to the first
    /// xdg_toplevel_decoration.configure event must also be treated as
    /// errors.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    #[inline]
    pub fn send_get_toplevel_decoration(
        &self,
        id: &Rc<ZxdgToplevelDecorationV1>,
        toplevel: &Rc<XdgToplevel>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            toplevel,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("toplevel")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zxdg_decoration_manager_v1#{}.get_toplevel_decoration(id: zxdg_toplevel_decoration_v1#{}, toplevel: xdg_toplevel#{})\n", id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZxdgDecorationManagerV1] proxies.
#[allow(dead_code)]
pub trait ZxdgDecorationManagerV1Handler: Any {
    /// destroy the decoration manager object
    ///
    /// Destroy the decoration manager. This doesn't destroy objects created
    /// with the manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<ZxdgDecorationManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_decoration_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a new toplevel decoration object
    ///
    /// Create a new decoration object associated with the given toplevel.
    ///
    /// Creating an xdg_toplevel_decoration from an xdg_toplevel which has a
    /// buffer attached or committed is a client error, and any attempts by a
    /// client to attach or manipulate a buffer prior to the first
    /// xdg_toplevel_decoration.configure event must also be treated as
    /// errors.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `toplevel`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_toplevel_decoration(
        &mut self,
        _slf: &Rc<ZxdgDecorationManagerV1>,
        id: &Rc<ZxdgToplevelDecorationV1>,
        toplevel: &Rc<XdgToplevel>,
    ) {
        let res = _slf.send_get_toplevel_decoration(
            id,
            toplevel,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zxdg_decoration_manager_v1.get_toplevel_decoration message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for ZxdgDecorationManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ZxdgDecorationManagerV1, version),
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_decoration_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zxdg_decoration_manager_v1#{}.get_toplevel_decoration(id: zxdg_toplevel_decoration_v1#{}, toplevel: xdg_toplevel#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ZxdgToplevelDecorationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<XdgToplevel>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("toplevel", o.core().interface, ProxyInterface::XdgToplevel));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_toplevel_decoration(&self, arg0, arg1);
                } else {
                    DefaultHandler.get_toplevel_decoration(&self, arg0, arg1);
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
            0 => "destroy",
            1 => "get_toplevel_decoration",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for ZxdgDecorationManagerV1 {
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

