//! fractional surface scale information
//!
//! A global interface for requesting surfaces to use fractional scales.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_fractional_scale_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpFractionalScaleManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpFractionalScaleManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpFractionalScaleManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaWpFractionalScaleManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpFractionalScaleManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpFractionalScaleManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpFractionalScaleManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpFractionalScaleManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpFractionalScaleManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unbind the fractional surface scale interface
    ///
    /// Informs the server that the client will not be using this protocol
    /// object anymore. This does not affect any other objects,
    /// wp_fractional_scale_v1 objects included.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wp_fractional_scale_manager_v1#{}.destroy()", id);
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

    /// Since when the get_fractional_scale message is available.
    #[allow(dead_code)]
    pub const MSG__GET_FRACTIONAL_SCALE__SINCE: u32 = 1;

    /// extend surface interface for scale information
    ///
    /// Create an add-on object for the the wl_surface to let the compositor
    /// request fractional scales. If the given wl_surface already has a
    /// wp_fractional_scale_v1 object associated, the fractional_scale_exists
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`: the new surface scale info interface id
    /// - `surface`: the surface
    #[inline]
    pub fn send_get_fractional_scale(
        &self,
        id: &Rc<MetaWpFractionalScaleV1>,
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
        eprintln!("server      <= wp_fractional_scale_manager_v1#{}.get_fractional_scale(id: wp_fractional_scale_v1#{}, surface: wl_surface#{})", id, arg0_id, arg1_id);
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

/// A message handler for [WpFractionalScaleManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaWpFractionalScaleManagerV1MessageHandler {
    /// unbind the fractional surface scale interface
    ///
    /// Informs the server that the client will not be using this protocol
    /// object anymore. This does not affect any other objects,
    /// wp_fractional_scale_v1 objects included.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpFractionalScaleManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_fractional_scale_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// extend surface interface for scale information
    ///
    /// Create an add-on object for the the wl_surface to let the compositor
    /// request fractional scales. If the given wl_surface already has a
    /// wp_fractional_scale_v1 object associated, the fractional_scale_exists
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `id`: the new surface scale info interface id
    /// - `surface`: the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_fractional_scale(
        &mut self,
        _slf: &Rc<MetaWpFractionalScaleManagerV1>,
        id: &Rc<MetaWpFractionalScaleV1>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_fractional_scale(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_fractional_scale_manager_v1.get_fractional_scale message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpFractionalScaleManagerV1 {
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
                eprintln!("client#{:04} -> wp_fractional_scale_manager_v1#{}.destroy()", client.endpoint.id, msg[0]);
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
                eprintln!("client#{:04} -> wp_fractional_scale_manager_v1#{}.get_fractional_scale(id: wp_fractional_scale_v1#{}, surface: wl_surface#{})", client.endpoint.id, msg[0], arg0, arg1);
                let arg0_id = arg0;
                let arg0 = MetaWpFractionalScaleV1::new(&self.core.state, self.core.version);
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
                    (**handler).get_fractional_scale(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_fractional_scale(&self, arg0, arg1);
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
            1 => "get_fractional_scale",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaWpFractionalScaleManagerV1 {
    /// Since when the error.fractional_scale_exists enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_FRACTIONAL_SCALE_EXISTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpFractionalScaleManagerV1Error(pub u32);

impl MetaWpFractionalScaleManagerV1Error {
    /// the surface already has a fractional_scale object associated
    #[allow(dead_code)]
    pub const FRACTIONAL_SCALE_EXISTS: Self = Self(0);
}

impl Debug for MetaWpFractionalScaleManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::FRACTIONAL_SCALE_EXISTS => "FRACTIONAL_SCALE_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
