//! reposition the pointer to a location on a surface
//!
//! This global interface allows applications to request the pointer to be
//! moved to a position relative to a wl_surface.
//!
//! Note that if the desired behavior is to constrain the pointer to an area
//! or lock it to a position, this protocol does not provide a reliable way
//! to do that. The pointer constraint and pointer lock protocols should be
//! used for those use cases instead.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_pointer_warp_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpPointerWarpV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpPointerWarpV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpPointerWarpV1MessageHandler for DefaultMessageHandler { }

impl MetaWpPointerWarpV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpPointerWarpV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpPointerWarpV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaWpPointerWarpV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaWpPointerWarpV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpPointerWarpV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpPointerWarpV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the warp manager
    ///
    /// Destroy the pointer warp manager.
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

    /// Since when the warp_pointer message is available.
    #[allow(dead_code)]
    pub const MSG__WARP_POINTER__SINCE: u32 = 1;

    /// reposition the pointer
    ///
    /// Request the compositor to move the pointer to a surface-local position.
    /// Whether or not the compositor honors the request is implementation defined,
    /// but it should
    /// - honor it if the surface has pointer focus, including
    ///   when it has an implicit pointer grab
    /// - reject it if the enter serial is incorrect
    /// - reject it if the requested position is outside of the surface
    ///
    /// Note that the enter serial is valid for any surface of the client,
    /// and does not have to be from the surface the pointer is warped to.
    ///
    /// # Arguments
    ///
    /// - `surface`: surface to position the pointer on
    /// - `pointer`: the pointer that should be repositioned
    /// - `x`:
    /// - `y`:
    /// - `serial`: serial number of the enter event
    #[inline]
    pub fn send_warp_pointer(
        &self,
        surface: &Rc<MetaWlSurface>,
        pointer: &Rc<MetaWlPointer>,
        x: Fixed,
        y: Fixed,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            surface,
            pointer,
            x,
            y,
            serial,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("pointer")),
            Some(id) => id,
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
            arg1_id,
            arg2.to_wire() as u32,
            arg3.to_wire() as u32,
            arg4,
        ]);
        Ok(())
    }
}

/// A message handler for [WpPointerWarpV1] proxies.
#[allow(dead_code)]
pub trait MetaWpPointerWarpV1MessageHandler {
    /// destroy the warp manager
    ///
    /// Destroy the pointer warp manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpPointerWarpV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_pointer_warp_v1.destroy message: {}", Report::new(e));
        }
    }

    /// reposition the pointer
    ///
    /// Request the compositor to move the pointer to a surface-local position.
    /// Whether or not the compositor honors the request is implementation defined,
    /// but it should
    /// - honor it if the surface has pointer focus, including
    ///   when it has an implicit pointer grab
    /// - reject it if the enter serial is incorrect
    /// - reject it if the requested position is outside of the surface
    ///
    /// Note that the enter serial is valid for any surface of the client,
    /// and does not have to be from the surface the pointer is warped to.
    ///
    /// # Arguments
    ///
    /// - `surface`: surface to position the pointer on
    /// - `pointer`: the pointer that should be repositioned
    /// - `x`:
    /// - `y`:
    /// - `serial`: serial number of the enter event
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn warp_pointer(
        &mut self,
        _slf: &Rc<MetaWpPointerWarpV1>,
        surface: &Rc<MetaWlSurface>,
        pointer: &Rc<MetaWlPointer>,
        x: Fixed,
        y: Fixed,
        serial: u32,
    ) {
        let res = _slf.send_warp_pointer(
            surface,
            pointer,
            x,
            y,
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_pointer_warp_v1.warp_pointer message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpPointerWarpV1 {
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
                    arg2,
                    arg3,
                    arg4,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = Fixed::from_wire(arg3 as i32);
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlPointer>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("pointer", o.core().interface, ProxyInterface::WlPointer));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).warp_pointer(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultMessageHandler.warp_pointer(&self, arg0, arg1, arg2, arg3, arg4);
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
            1 => "warp_pointer",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

