//! protocol for tearing control
//!
//! For some use cases like games or drawing tablets it can make sense to
//! reduce latency by accepting tearing with the use of asynchronous page
//! flips. This global is a factory interface, allowing clients to inform
//! which type of presentation the content of their surfaces is suitable for.
//!
//! Graphics APIs like EGL or Vulkan, that manage the buffer queue and commits
//! of a wl_surface themselves, are likely to be using this extension
//! internally. If a client is using such an API for a wl_surface, it should
//! not directly use this extension on that surface, to avoid raising a
//! tearing_control_exists protocol error.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_tearing_control_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpTearingControlManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpTearingControlManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpTearingControlManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaWpTearingControlManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpTearingControlManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpTearingControlManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaWpTearingControlManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaWpTearingControlManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpTearingControlManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpTearingControlManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy tearing control factory object
    ///
    /// Destroy this tearing control factory object. Other objects, including
    /// wp_tearing_control_v1 objects created by this factory, are not affected
    /// by this request.
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

    /// Since when the get_tearing_control message is available.
    #[allow(dead_code)]
    pub const MSG__GET_TEARING_CONTROL__SINCE: u32 = 1;

    /// extend surface interface for tearing control
    ///
    /// Instantiate an interface extension for the given wl_surface to request
    /// asynchronous page flips for presentation.
    ///
    /// If the given wl_surface already has a wp_tearing_control_v1 object
    /// associated, the tearing_control_exists protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_tearing_control(
        &self,
        id: &Rc<MetaWpTearingControlV1>,
        surface: &Rc<MetaWlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
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
}

/// A message handler for [WpTearingControlManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaWpTearingControlManagerV1MessageHandler {
    /// destroy tearing control factory object
    ///
    /// Destroy this tearing control factory object. Other objects, including
    /// wp_tearing_control_v1 objects created by this factory, are not affected
    /// by this request.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpTearingControlManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_tearing_control_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// extend surface interface for tearing control
    ///
    /// Instantiate an interface extension for the given wl_surface to request
    /// asynchronous page flips for presentation.
    ///
    /// If the given wl_surface already has a wp_tearing_control_v1 object
    /// associated, the tearing_control_exists protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_tearing_control(
        &mut self,
        _slf: &Rc<MetaWpTearingControlManagerV1>,
        id: &Rc<MetaWpTearingControlV1>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_tearing_control(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_tearing_control_manager_v1.get_tearing_control message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpTearingControlManagerV1 {
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0_id = arg0;
                let arg0 = MetaWpTearingControlV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_tearing_control(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_tearing_control(&self, arg0, arg1);
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
            1 => "get_tearing_control",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaWpTearingControlManagerV1 {
    /// Since when the error.tearing_control_exists enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_TEARING_CONTROL_EXISTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpTearingControlManagerV1Error(pub u32);

impl MetaWpTearingControlManagerV1Error {
    /// the surface already has a tearing object associated
    #[allow(dead_code)]
    pub const TEARING_CONTROL_EXISTS: Self = Self(0);
}

impl Debug for MetaWpTearingControlManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TEARING_CONTROL_EXISTS => "TEARING_CONTROL_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
