//! per-surface tearing control interface
//!
//! An additional interface to a wl_surface object, which allows the client
//! to hint to the compositor if the content on the surface is suitable for
//! presentation with tearing.
//! The default presentation hint is vsync. See presentation_hint for more
//! details.
//!
//! If the associated wl_surface is destroyed, this object becomes inert and
//! should be destroyed.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_tearing_control_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpTearingControlV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpTearingControlV1Handler>,
}

struct DefaultHandler;

impl WpTearingControlV1Handler for DefaultHandler { }

impl ConcreteObject for WpTearingControlV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpTearingControlV1;
    const INTERFACE_NAME: &str = "wp_tearing_control_v1";
}

impl WpTearingControlV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpTearingControlV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpTearingControlV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpTearingControlV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpTearingControlV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpTearingControlV1 {
    /// Since when the set_presentation_hint message is available.
    pub const MSG__SET_PRESENTATION_HINT__SINCE: u32 = 1;

    /// set presentation hint
    ///
    /// Set the presentation hint for the associated wl_surface. This state is
    /// double-buffered, see wl_surface.commit.
    ///
    /// The compositor is free to dynamically respect or ignore this hint based
    /// on various conditions like hardware capabilities, surface state and
    /// user preferences.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    #[inline]
    pub fn try_send_set_presentation_hint(
        &self,
        hint: WpTearingControlV1PresentationHint,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            hint,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WpTearingControlV1PresentationHint) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_tearing_control_v1#{}.set_presentation_hint(hint: {:?})\n", id, arg0);
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
            0,
            arg0.0,
        ]);
        Ok(())
    }

    /// set presentation hint
    ///
    /// Set the presentation hint for the associated wl_surface. This state is
    /// double-buffered, see wl_surface.commit.
    ///
    /// The compositor is free to dynamically respect or ignore this hint based
    /// on various conditions like hardware capabilities, surface state and
    /// user preferences.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    #[inline]
    pub fn send_set_presentation_hint(
        &self,
        hint: WpTearingControlV1PresentationHint,
    ) {
        let res = self.try_send_set_presentation_hint(
            hint,
        );
        if let Err(e) = res {
            log_send("wp_tearing_control_v1.set_presentation_hint", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy tearing control object
    ///
    /// Destroy this surface tearing object and revert the presentation hint to
    /// vsync. The change will be applied on the next wl_surface.commit.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_tearing_control_v1#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy tearing control object
    ///
    /// Destroy this surface tearing object and revert the presentation hint to
    /// vsync. The change will be applied on the next wl_surface.commit.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_tearing_control_v1.destroy", &e);
        }
    }
}

/// A message handler for [`WpTearingControlV1`] proxies.
pub trait WpTearingControlV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpTearingControlV1>) {
        slf.core.delete_id();
    }

    /// set presentation hint
    ///
    /// Set the presentation hint for the associated wl_surface. This state is
    /// double-buffered, see wl_surface.commit.
    ///
    /// The compositor is free to dynamically respect or ignore this hint based
    /// on various conditions like hardware capabilities, surface state and
    /// user preferences.
    ///
    /// # Arguments
    ///
    /// - `hint`:
    #[inline]
    fn handle_set_presentation_hint(
        &mut self,
        slf: &Rc<WpTearingControlV1>,
        hint: WpTearingControlV1PresentationHint,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_presentation_hint(
            hint,
        );
        if let Err(e) = res {
            log_forward("wp_tearing_control_v1.set_presentation_hint", &e);
        }
    }

    /// destroy tearing control object
    ///
    /// Destroy this surface tearing object and revert the presentation hint to
    /// vsync. The change will be applied on the next wl_surface.commit.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpTearingControlV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_tearing_control_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for WpTearingControlV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpTearingControlV1, version),
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WpTearingControlV1PresentationHint(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WpTearingControlV1PresentationHint) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_tearing_control_v1#{}.set_presentation_hint(hint: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_presentation_hint(&self, arg0);
                } else {
                    DefaultHandler.handle_set_presentation_hint(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_tearing_control_v1#{}.destroy()\n", client_id, id);
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
            0 => "set_presentation_hint",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpTearingControlV1 {
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

impl WpTearingControlV1 {
    /// Since when the presentation_hint.vsync enum variant is available.
    pub const ENM__PRESENTATION_HINT_VSYNC__SINCE: u32 = 1;
    /// Since when the presentation_hint.async enum variant is available.
    pub const ENM__PRESENTATION_HINT_ASYNC__SINCE: u32 = 1;
}

/// presentation hint values
///
/// This enum provides information for if submitted frames from the client
/// may be presented with tearing.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpTearingControlV1PresentationHint(pub u32);

impl WpTearingControlV1PresentationHint {
    /// tearing-free presentation
    ///
    /// The content of this surface is meant to be synchronized to the
    /// vertical blanking period. This should not result in visible tearing
    /// and may result in a delay before a surface commit is presented.
    pub const VSYNC: Self = Self(0);

    /// asynchronous presentation
    ///
    /// The content of this surface is meant to be presented with minimal
    /// latency and tearing is acceptable.
    pub const ASYNC: Self = Self(1);
}

impl Debug for WpTearingControlV1PresentationHint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::VSYNC => "VSYNC",
            Self::ASYNC => "ASYNC",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
