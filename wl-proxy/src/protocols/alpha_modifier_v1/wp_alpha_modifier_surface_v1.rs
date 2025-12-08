//! alpha modifier object for a surface
//!
//! This interface allows the client to set a factor for the alpha values on
//! a surface, which can be used to offload such operations to the compositor.
//! The default factor is UINT32_MAX.
//!
//! This object has to be destroyed before the associated wl_surface. Once the
//! wl_surface is destroyed, all request on this object will raise the
//! no_surface error.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_alpha_modifier_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpAlphaModifierSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpAlphaModifierSurfaceV1Handler>,
}

struct DefaultHandler;

impl WpAlphaModifierSurfaceV1Handler for DefaultHandler { }

impl ConcreteObject for WpAlphaModifierSurfaceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpAlphaModifierSurfaceV1;
    const INTERFACE_NAME: &str = "wp_alpha_modifier_surface_v1";
}

impl WpAlphaModifierSurfaceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpAlphaModifierSurfaceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpAlphaModifierSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpAlphaModifierSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpAlphaModifierSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpAlphaModifierSurfaceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the alpha modifier object
    ///
    /// This destroys the object, and is equivalent to set_multiplier with
    /// a value of UINT32_MAX, with the same double-buffered semantics as
    /// set_multiplier.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_alpha_modifier_surface_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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

    /// destroy the alpha modifier object
    ///
    /// This destroys the object, and is equivalent to set_multiplier with
    /// a value of UINT32_MAX, with the same double-buffered semantics as
    /// set_multiplier.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_alpha_modifier_surface_v1.destroy", &e);
        }
    }

    /// Since when the set_multiplier message is available.
    pub const MSG__SET_MULTIPLIER__SINCE: u32 = 1;

    /// specify the alpha multiplier
    ///
    /// Sets the alpha multiplier for the surface. The alpha multiplier is
    /// double-buffered state, see wl_surface.commit for details.
    ///
    /// This factor is applied in the compositor's blending space, as an
    /// additional step after the processing of per-pixel alpha values for the
    /// wl_surface. The exact meaning of the factor is thus undefined, unless
    /// the blending space is specified in a different extension.
    ///
    /// This multiplier is applied even if the buffer attached to the
    /// wl_surface doesn't have an alpha channel; in that case an alpha value
    /// of one is used instead.
    ///
    /// Zero means completely transparent, UINT32_MAX means completely opaque.
    ///
    /// # Arguments
    ///
    /// - `factor`:
    #[inline]
    pub fn try_send_set_multiplier(
        &self,
        factor: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            factor,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_alpha_modifier_surface_v1#{}.set_multiplier(factor: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
        ]);
        Ok(())
    }

    /// specify the alpha multiplier
    ///
    /// Sets the alpha multiplier for the surface. The alpha multiplier is
    /// double-buffered state, see wl_surface.commit for details.
    ///
    /// This factor is applied in the compositor's blending space, as an
    /// additional step after the processing of per-pixel alpha values for the
    /// wl_surface. The exact meaning of the factor is thus undefined, unless
    /// the blending space is specified in a different extension.
    ///
    /// This multiplier is applied even if the buffer attached to the
    /// wl_surface doesn't have an alpha channel; in that case an alpha value
    /// of one is used instead.
    ///
    /// Zero means completely transparent, UINT32_MAX means completely opaque.
    ///
    /// # Arguments
    ///
    /// - `factor`:
    #[inline]
    pub fn send_set_multiplier(
        &self,
        factor: u32,
    ) {
        let res = self.try_send_set_multiplier(
            factor,
        );
        if let Err(e) = res {
            log_send("wp_alpha_modifier_surface_v1.set_multiplier", &e);
        }
    }
}

/// A message handler for [`WpAlphaModifierSurfaceV1`] proxies.
pub trait WpAlphaModifierSurfaceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpAlphaModifierSurfaceV1>) {
        slf.core.delete_id();
    }

    /// destroy the alpha modifier object
    ///
    /// This destroys the object, and is equivalent to set_multiplier with
    /// a value of UINT32_MAX, with the same double-buffered semantics as
    /// set_multiplier.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpAlphaModifierSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_alpha_modifier_surface_v1.destroy", &e);
        }
    }

    /// specify the alpha multiplier
    ///
    /// Sets the alpha multiplier for the surface. The alpha multiplier is
    /// double-buffered state, see wl_surface.commit for details.
    ///
    /// This factor is applied in the compositor's blending space, as an
    /// additional step after the processing of per-pixel alpha values for the
    /// wl_surface. The exact meaning of the factor is thus undefined, unless
    /// the blending space is specified in a different extension.
    ///
    /// This multiplier is applied even if the buffer attached to the
    /// wl_surface doesn't have an alpha channel; in that case an alpha value
    /// of one is used instead.
    ///
    /// Zero means completely transparent, UINT32_MAX means completely opaque.
    ///
    /// # Arguments
    ///
    /// - `factor`:
    #[inline]
    fn handle_set_multiplier(
        &mut self,
        slf: &Rc<WpAlphaModifierSurfaceV1>,
        factor: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_multiplier(
            factor,
        );
        if let Err(e) = res {
            log_forward("wp_alpha_modifier_surface_v1.set_multiplier", &e);
        }
    }
}

impl ObjectPrivate for WpAlphaModifierSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpAlphaModifierSurfaceV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_alpha_modifier_surface_v1#{}.destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
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
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_alpha_modifier_surface_v1#{}.set_multiplier(factor: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_multiplier(&self, arg0);
                } else {
                    DefaultHandler.handle_set_multiplier(&self, arg0);
                }
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "set_multiplier",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpAlphaModifierSurfaceV1 {
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

impl WpAlphaModifierSurfaceV1 {
    /// Since when the error.no_surface enum variant is available.
    pub const ENM__ERROR_NO_SURFACE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpAlphaModifierSurfaceV1Error(pub u32);

impl WpAlphaModifierSurfaceV1Error {
    /// wl_surface was destroyed
    pub const NO_SURFACE: Self = Self(0);
}

impl Debug for WpAlphaModifierSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NO_SURFACE => "NO_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
