//! fast_forward interface
//!
//! A fast_forward object to ignore content update constraints.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_fast_forward_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonFastForwardV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonFastForwardV1Handler>,
}

struct DefaultHandler;

impl WestonFastForwardV1Handler for DefaultHandler { }

impl ConcreteObject for WestonFastForwardV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonFastForwardV1;
    const INTERFACE_NAME: &str = "weston_fast_forward_v1";
}

impl WestonFastForwardV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonFastForwardV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonFastForwardV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonFastForwardV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonFastForwardV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonFastForwardV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the fast_forward object
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object.
    ///
    /// Surface state changes previously made by this protocol are
    /// unaffected by this object's destruction.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_fast_forward_v1#{}.destroy()\n", id);
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

    /// destroy the fast_forward object
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object.
    ///
    /// Surface state changes previously made by this protocol are
    /// unaffected by this object's destruction.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("weston_fast_forward_v1.destroy", &e);
        }
    }

    /// Since when the fast_forward message is available.
    pub const MSG__FAST_FORWARD__SINCE: u32 = 1;

    /// fast forwards through upcoming constraints
    ///
    /// Upon wl_surface.commit of a content update containing a fast forward
    /// request, the compositor must immediately ignore all previously
    /// committed constraints on that surface.
    ///
    /// The content update containing the fast forward request also has
    /// its constraints ignored. Normal constraint handling resumes for
    /// subsequent commits.
    ///
    /// surface_destroyed error will be emitted if associated surface no
    /// longer exists.
    ///
    /// "fast_forward" is double-buffered state, see wl_surface.commit.
    #[inline]
    pub fn try_send_fast_forward(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_fast_forward_v1#{}.fast_forward()\n", id);
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
        Ok(())
    }

    /// fast forwards through upcoming constraints
    ///
    /// Upon wl_surface.commit of a content update containing a fast forward
    /// request, the compositor must immediately ignore all previously
    /// committed constraints on that surface.
    ///
    /// The content update containing the fast forward request also has
    /// its constraints ignored. Normal constraint handling resumes for
    /// subsequent commits.
    ///
    /// surface_destroyed error will be emitted if associated surface no
    /// longer exists.
    ///
    /// "fast_forward" is double-buffered state, see wl_surface.commit.
    #[inline]
    pub fn send_fast_forward(
        &self,
    ) {
        let res = self.try_send_fast_forward(
        );
        if let Err(e) = res {
            log_send("weston_fast_forward_v1.fast_forward", &e);
        }
    }
}

/// A message handler for [`WestonFastForwardV1`] proxies.
pub trait WestonFastForwardV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonFastForwardV1>) {
        slf.core.delete_id();
    }

    /// destroy the fast_forward object
    ///
    /// Informs the server that the client will no longer be using this
    /// protocol object.
    ///
    /// Surface state changes previously made by this protocol are
    /// unaffected by this object's destruction.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WestonFastForwardV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("weston_fast_forward_v1.destroy", &e);
        }
    }

    /// fast forwards through upcoming constraints
    ///
    /// Upon wl_surface.commit of a content update containing a fast forward
    /// request, the compositor must immediately ignore all previously
    /// committed constraints on that surface.
    ///
    /// The content update containing the fast forward request also has
    /// its constraints ignored. Normal constraint handling resumes for
    /// subsequent commits.
    ///
    /// surface_destroyed error will be emitted if associated surface no
    /// longer exists.
    ///
    /// "fast_forward" is double-buffered state, see wl_surface.commit.
    #[inline]
    fn handle_fast_forward(
        &mut self,
        slf: &Rc<WestonFastForwardV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_fast_forward(
        );
        if let Err(e) = res {
            log_forward("weston_fast_forward_v1.fast_forward", &e);
        }
    }
}

impl ObjectPrivate for WestonFastForwardV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonFastForwardV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_fast_forward_v1#{}.destroy()\n", client_id, id);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_fast_forward_v1#{}.fast_forward()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_fast_forward(&self);
                } else {
                    DefaultHandler.handle_fast_forward(&self);
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
            1 => "fast_forward",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for WestonFastForwardV1 {
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

impl WestonFastForwardV1 {
    /// Since when the error.surface_destroyed enum variant is available.
    pub const ENM__ERROR_SURFACE_DESTROYED__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to illegal
/// requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonFastForwardV1Error(pub u32);

impl WestonFastForwardV1Error {
    /// the associated surface no longer exists
    pub const SURFACE_DESTROYED: Self = Self(0);
}

impl Debug for WestonFastForwardV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SURFACE_DESTROYED => "SURFACE_DESTROYED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
