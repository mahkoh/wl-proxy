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
pub struct MetaXdgSystemBellV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgSystemBellV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgSystemBellV1MessageHandler for DefaultMessageHandler { }

impl MetaXdgSystemBellV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaXdgSystemBellV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgSystemBellV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaXdgSystemBellV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaXdgSystemBellV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgSystemBellV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgSystemBellV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
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

    /// Since when the ring message is available.
    #[allow(dead_code)]
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
        surface: Option<&Rc<MetaWlSurface>>,
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
        ]);
        Ok(())
    }
}

/// A message handler for [XdgSystemBellV1] proxies.
#[allow(dead_code)]
pub trait MetaXdgSystemBellV1MessageHandler {
    /// destroy the system bell object
    ///
    /// Notify that the object will no longer be used.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgSystemBellV1>,
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
        _slf: &Rc<MetaXdgSystemBellV1>,
        surface: Option<&Rc<MetaWlSurface>>,
    ) {
        let res = _slf.send_ring(
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_system_bell_v1.ring message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgSystemBellV1 {
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
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).ring(&self, arg0);
                } else {
                    DefaultMessageHandler.ring(&self, arg0);
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

