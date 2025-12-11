//! system bell
//!
//! This global interface enables clients to ring the system bell.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_system_bell_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgSystemBellV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn XdgSystemBellV1Handler>,
}

struct DefaultHandler;

impl XdgSystemBellV1Handler for DefaultHandler { }

impl XdgSystemBellV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "xdg_system_bell_v1";
}

impl XdgSystemBellV1 {
    pub fn set_handler(&self, handler: impl XdgSystemBellV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn XdgSystemBellV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgSystemBellV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgSystemBellV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgSystemBellV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the system bell object
    ///
    /// Notify that the object will no longer be used.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_system_bell_v1#{}.destroy()\n", id);
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

    /// Since when the ring message is available.
    pub const MSG__RING__SINCE: u32 = 1;

    /// ring the system bell
    ///
    /// This requests rings the system bell on behalf of a client. How ringing
    /// the bell is implemented is up to the compositor. It may be an audible
    /// sound, a visual feedback of some kind, or any other thing including
    /// nothing.
    ///
    ///         The passed surface should correspond to a toplevel like surface role,
    ///         or be null, meaning the client doesn't have a particular toplevel it
    ///         wants to associate the bell ringing with. See the xdg-shell protocol
    ///         extension for a toplevel like surface role.
    ///
    /// # Arguments
    ///
    /// - `surface`: associated surface
    #[inline]
    pub fn send_ring(
        &self,
        surface: Option<&Rc<WlSurface>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            surface,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("surface")),
                Some(id) => id,
            },
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_system_bell_v1#{}.ring(surface: wl_surface#{})\n", id, arg0_id);
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
        ]);
        Ok(())
    }
}

/// A message handler for [XdgSystemBellV1] proxies.
pub trait XdgSystemBellV1Handler: Any {
    /// destroy the system bell object
    ///
    /// Notify that the object will no longer be used.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<XdgSystemBellV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_system_bell_v1.destroy message: {}", Report::new(e));
        }
    }

    /// ring the system bell
    ///
    /// This requests rings the system bell on behalf of a client. How ringing
    /// the bell is implemented is up to the compositor. It may be an audible
    /// sound, a visual feedback of some kind, or any other thing including
    /// nothing.
    ///
    ///         The passed surface should correspond to a toplevel like surface role,
    ///         or be null, meaning the client doesn't have a particular toplevel it
    ///         wants to associate the bell ringing with. See the xdg-shell protocol
    ///         extension for a toplevel like surface role.
    ///
    /// # Arguments
    ///
    /// - `surface`: associated surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn ring(
        &mut self,
        _slf: &Rc<XdgSystemBellV1>,
        surface: Option<&Rc<WlSurface>>,
    ) {
        let res = _slf.send_ring(
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_system_bell_v1.ring message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for XdgSystemBellV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::XdgSystemBellV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_system_bell_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_system_bell_v1#{}.ring(surface: wl_surface#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).ring(&self, arg0);
                } else {
                    DefaultHandler.ring(&self, arg0);
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
            1 => "ring",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Proxy for XdgSystemBellV1 {
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

