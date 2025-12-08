//! commit timing
//!
//! When a compositor latches on to new content updates it will check for
//! any number of requirements of the available content updates (such as
//! fences of all buffers being signalled) to consider the update ready.
//!
//! This protocol provides a method for adding a time constraint to surface
//! content. This constraint indicates to the compositor that a content
//! update should be presented as closely as possible to, but not before,
//! a specified time.
//!
//! This protocol does not change the Wayland property that content
//! updates are applied in the order they are received, even when some
//! content updates contain timestamps and others do not.
//!
//! To provide timestamps, this global factory interface must be used to
//! acquire a wp_commit_timing_v1 object for a surface, which may then be
//! used to provide timestamp information for commits.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_commit_timing_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpCommitTimingManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpCommitTimingManagerV1Handler>,
}

struct DefaultHandler;

impl WpCommitTimingManagerV1Handler for DefaultHandler { }

impl ConcreteObject for WpCommitTimingManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpCommitTimingManagerV1;
    const INTERFACE_NAME: &str = "wp_commit_timing_manager_v1";
}

impl WpCommitTimingManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpCommitTimingManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpCommitTimingManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpCommitTimingManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpCommitTimingManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpCommitTimingManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unbind from the commit timing interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object. Existing objects created by this object
    /// are not affected.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_commit_timing_manager_v1#{}.destroy()\n", id);
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

    /// unbind from the commit timing interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object. Existing objects created by this object
    /// are not affected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_commit_timing_manager_v1.destroy", &e);
        }
    }

    /// Since when the get_timer message is available.
    pub const MSG__GET_TIMER__SINCE: u32 = 1;

    /// request commit timer interface for surface
    ///
    /// Establish a timing controller for a surface.
    ///
    /// Only one commit timer can be created for a surface, or a
    /// commit_timer_exists protocol error will be generated.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn try_send_get_timer(
        &self,
        id: &Rc<WpCommitTimerV1>,
        surface: &Rc<WlSurface>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_commit_timing_manager_v1#{}.get_timer(id: wp_commit_timer_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// request commit timer interface for surface
    ///
    /// Establish a timing controller for a surface.
    ///
    /// Only one commit timer can be created for a surface, or a
    /// commit_timer_exists protocol error will be generated.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_timer(
        &self,
        id: &Rc<WpCommitTimerV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_timer(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("wp_commit_timing_manager_v1.get_timer", &e);
        }
    }

    /// request commit timer interface for surface
    ///
    /// Establish a timing controller for a surface.
    ///
    /// Only one commit timer can be created for a surface, or a
    /// commit_timer_exists protocol error will be generated.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_try_send_get_timer(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<WpCommitTimerV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_timer(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// request commit timer interface for surface
    ///
    /// Establish a timing controller for a surface.
    ///
    /// Only one commit timer can be created for a surface, or a
    /// commit_timer_exists protocol error will be generated.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_send_get_timer(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<WpCommitTimerV1> {
        let id = self.core.create_child();
        self.send_get_timer(
            &id,
            surface,
        );
        id
    }
}

/// A message handler for [`WpCommitTimingManagerV1`] proxies.
pub trait WpCommitTimingManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpCommitTimingManagerV1>) {
        slf.core.delete_id();
    }

    /// unbind from the commit timing interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object. Existing objects created by this object
    /// are not affected.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpCommitTimingManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_commit_timing_manager_v1.destroy", &e);
        }
    }

    /// request commit timer interface for surface
    ///
    /// Establish a timing controller for a surface.
    ///
    /// Only one commit timer can be created for a surface, or a
    /// commit_timer_exists protocol error will be generated.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_timer(
        &mut self,
        slf: &Rc<WpCommitTimingManagerV1>,
        id: &Rc<WpCommitTimerV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_timer(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("wp_commit_timing_manager_v1.get_timer", &e);
        }
    }
}

impl ObjectPrivate for WpCommitTimingManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpCommitTimingManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_commit_timing_manager_v1#{}.destroy()\n", client_id, id);
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
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_commit_timing_manager_v1#{}.get_timer(id: wp_commit_timer_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WpCommitTimerV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_timer(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_timer(&self, arg0, arg1);
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
            1 => "get_timer",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpCommitTimingManagerV1 {
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

impl WpCommitTimingManagerV1 {
    /// Since when the error.commit_timer_exists enum variant is available.
    pub const ENM__ERROR_COMMIT_TIMER_EXISTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpCommitTimingManagerV1Error(pub u32);

impl WpCommitTimingManagerV1Error {
    /// commit timer already exists for surface
    pub const COMMIT_TIMER_EXISTS: Self = Self(0);
}

impl Debug for WpCommitTimingManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::COMMIT_TIMER_EXISTS => "COMMIT_TIMER_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
