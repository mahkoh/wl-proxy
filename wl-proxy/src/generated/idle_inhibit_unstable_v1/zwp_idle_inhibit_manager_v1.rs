//! control behavior when display idles
//!
//! This interface permits inhibiting the idle behavior such as screen
//! blanking, locking, and screensaving.  The client binds the idle manager
//! globally, then creates idle-inhibitor objects for each surface.
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

/// A zwp_idle_inhibit_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpIdleInhibitManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpIdleInhibitManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpIdleInhibitManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpIdleInhibitManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpIdleInhibitManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpIdleInhibitManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpIdleInhibitManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpIdleInhibitManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpIdleInhibitManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpIdleInhibitManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the idle inhibitor object
    ///
    /// Destroy the inhibit manager.
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

    /// Since when the create_inhibitor message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_INHIBITOR__SINCE: u32 = 1;

    /// create a new inhibitor object
    ///
    /// Create a new inhibitor object associated with the given surface.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: the surface that inhibits the idle behavior
    #[inline]
    pub fn send_create_inhibitor(
        &self,
        id: &Rc<MetaZwpIdleInhibitorV1>,
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

/// A message handler for [ZwpIdleInhibitManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpIdleInhibitManagerV1MessageHandler {
    /// destroy the idle inhibitor object
    ///
    /// Destroy the inhibit manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpIdleInhibitManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_idle_inhibit_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a new inhibitor object
    ///
    /// Create a new inhibitor object associated with the given surface.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: the surface that inhibits the idle behavior
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn create_inhibitor(
        &mut self,
        _slf: &Rc<MetaZwpIdleInhibitManagerV1>,
        id: &Rc<MetaZwpIdleInhibitorV1>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_create_inhibitor(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_idle_inhibit_manager_v1.create_inhibitor message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpIdleInhibitManagerV1 {
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
                let arg0 = MetaZwpIdleInhibitorV1::new(&self.core.state, self.core.version);
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
                    (**handler).create_inhibitor(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.create_inhibitor(&self, arg0, arg1);
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
            1 => "create_inhibitor",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

