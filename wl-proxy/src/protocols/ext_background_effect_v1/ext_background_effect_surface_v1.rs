//! background effects for a surface
//!
//! The background effect object provides a way to specify a region behind
//! a surface that should have background effects like blur applied.
//!
//! If the wl_surface associated with the ext_background_effect_surface_v1
//! object has been destroyed, this object becomes inert.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_background_effect_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtBackgroundEffectSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtBackgroundEffectSurfaceV1Handler>,
}

struct DefaultHandler;

impl ExtBackgroundEffectSurfaceV1Handler for DefaultHandler { }

impl ExtBackgroundEffectSurfaceV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ExtBackgroundEffectSurfaceV1;
    pub const INTERFACE_NAME: &str = "ext_background_effect_surface_v1";
}

impl ExtBackgroundEffectSurfaceV1 {
    pub fn set_handler(&self, handler: impl ExtBackgroundEffectSurfaceV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ExtBackgroundEffectSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtBackgroundEffectSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtBackgroundEffectSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtBackgroundEffectSurfaceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// release the blur object
    ///
    /// Informs the server that the client will no longer be using this protocol
    /// object. The effect regions will be removed on the next commit.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_background_effect_surface_v1#{}.destroy()\n", id);
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

    /// Since when the set_blur_region message is available.
    pub const MSG__SET_BLUR_REGION__SINCE: u32 = 1;

    /// set blur region
    ///
    /// This request sets the region of the surface that will have its
    /// background blurred.
    ///
    /// The blur region is specified in the surface-local coordinates, and
    /// clipped by the compositor to the surface size.
    ///
    /// The initial value for the blur region is empty. Setting the pending
    /// blur region has copy semantics, and the wl_region object can be
    /// destroyed immediately. A NULL wl_region removes the effect.
    ///
    /// The blur region is double-buffered state, and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The blur algorithm is subject to compositor policies.
    ///
    /// If the associated surface has been destroyed, the surface_destroyed
    /// error will be raised.
    ///
    /// # Arguments
    ///
    /// - `region`: blur region of the surface
    #[inline]
    pub fn send_set_blur_region(
        &self,
        region: Option<&Rc<WlRegion>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            region,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("region")),
                Some(id) => id,
            },
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_background_effect_surface_v1#{}.set_blur_region(region: wl_region#{})\n", id, arg0_id);
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

/// A message handler for [ExtBackgroundEffectSurfaceV1] proxies.
pub trait ExtBackgroundEffectSurfaceV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtBackgroundEffectSurfaceV1>) {
        let _ = slf.core.delete_id();
    }

    /// release the blur object
    ///
    /// Informs the server that the client will no longer be using this protocol
    /// object. The effect regions will be removed on the next commit.
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<ExtBackgroundEffectSurfaceV1>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_background_effect_surface_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set blur region
    ///
    /// This request sets the region of the surface that will have its
    /// background blurred.
    ///
    /// The blur region is specified in the surface-local coordinates, and
    /// clipped by the compositor to the surface size.
    ///
    /// The initial value for the blur region is empty. Setting the pending
    /// blur region has copy semantics, and the wl_region object can be
    /// destroyed immediately. A NULL wl_region removes the effect.
    ///
    /// The blur region is double-buffered state, and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The blur algorithm is subject to compositor policies.
    ///
    /// If the associated surface has been destroyed, the surface_destroyed
    /// error will be raised.
    ///
    /// # Arguments
    ///
    /// - `region`: blur region of the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_blur_region(
        &mut self,
        _slf: &Rc<ExtBackgroundEffectSurfaceV1>,
        region: Option<&Rc<WlRegion>>,
    ) {
        if !_slf.core.forward_to_server.get() {
            return;
        }
        let res = _slf.send_set_blur_region(
            region,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_background_effect_surface_v1.set_blur_region message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ExtBackgroundEffectSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtBackgroundEffectSurfaceV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
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
        let Some(mut handler) = self.handler.try_borrow_mut() else {
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_background_effect_surface_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_background_effect_surface_v1#{}.set_blur_region(region: wl_region#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlRegion>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("region", o.core().interface, ObjectInterface::WlRegion));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_blur_region(&self, arg0);
                } else {
                    DefaultHandler.handle_set_blur_region(&self, arg0);
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
        let Some(mut handler) = self.handler.try_borrow_mut() else {
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
            1 => "set_blur_region",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ExtBackgroundEffectSurfaceV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl ExtBackgroundEffectSurfaceV1 {
    /// Since when the error.surface_destroyed enum variant is available.
    pub const ENM__ERROR_SURFACE_DESTROYED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtBackgroundEffectSurfaceV1Error(pub u32);

impl ExtBackgroundEffectSurfaceV1Error {
    /// the associated surface has been destroyed
    pub const SURFACE_DESTROYED: Self = Self(0);
}

impl Debug for ExtBackgroundEffectSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SURFACE_DESTROYED => "SURFACE_DESTROYED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
