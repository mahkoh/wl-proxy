//! Surface commit timer
//!
//! An object to set a time constraint for a content update on a surface.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_commit_timer_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpCommitTimerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpCommitTimerV1Handler>,
}

struct DefaultHandler;

impl WpCommitTimerV1Handler for DefaultHandler { }

impl ConcreteObject for WpCommitTimerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpCommitTimerV1;
    const INTERFACE_NAME: &str = "wp_commit_timer_v1";
}

impl WpCommitTimerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpCommitTimerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpCommitTimerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpCommitTimerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpCommitTimerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpCommitTimerV1 {
    /// Since when the set_timestamp message is available.
    pub const MSG__SET_TIMESTAMP__SINCE: u32 = 1;

    /// Specify time the following commit takes effect
    ///
    /// Provide a timing constraint for a surface content update.
    ///
    /// A set_timestamp request may be made before a wl_surface.commit to
    /// tell the compositor that the content is intended to be presented
    /// as closely as possible to, but not before, the specified time.
    /// The time is in the domain of the compositor's presentation clock.
    ///
    /// An invalid_timestamp error will be generated for invalid tv_nsec.
    ///
    /// If a timestamp already exists on the surface, a timestamp_exists
    /// error is generated.
    ///
    /// Requesting set_timestamp after the commit_timer object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of target time
    /// - `tv_sec_lo`: low 32 bits of the seconds part of target time
    /// - `tv_nsec`: nanoseconds part of target time
    #[inline]
    pub fn try_send_set_timestamp(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_commit_timer_v1#{}.set_timestamp(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2);
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
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Specify time the following commit takes effect
    ///
    /// Provide a timing constraint for a surface content update.
    ///
    /// A set_timestamp request may be made before a wl_surface.commit to
    /// tell the compositor that the content is intended to be presented
    /// as closely as possible to, but not before, the specified time.
    /// The time is in the domain of the compositor's presentation clock.
    ///
    /// An invalid_timestamp error will be generated for invalid tv_nsec.
    ///
    /// If a timestamp already exists on the surface, a timestamp_exists
    /// error is generated.
    ///
    /// Requesting set_timestamp after the commit_timer object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of target time
    /// - `tv_sec_lo`: low 32 bits of the seconds part of target time
    /// - `tv_nsec`: nanoseconds part of target time
    #[inline]
    pub fn send_set_timestamp(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) {
        let res = self.try_send_set_timestamp(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log_send("wp_commit_timer_v1.set_timestamp", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// Destroy the timer
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object.
    ///
    /// Existing timing constraints are not affected by the destruction.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_commit_timer_v1#{}.destroy()\n", id);
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

    /// Destroy the timer
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object.
    ///
    /// Existing timing constraints are not affected by the destruction.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_commit_timer_v1.destroy", &e);
        }
    }
}

/// A message handler for [`WpCommitTimerV1`] proxies.
pub trait WpCommitTimerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpCommitTimerV1>) {
        slf.core.delete_id();
    }

    /// Specify time the following commit takes effect
    ///
    /// Provide a timing constraint for a surface content update.
    ///
    /// A set_timestamp request may be made before a wl_surface.commit to
    /// tell the compositor that the content is intended to be presented
    /// as closely as possible to, but not before, the specified time.
    /// The time is in the domain of the compositor's presentation clock.
    ///
    /// An invalid_timestamp error will be generated for invalid tv_nsec.
    ///
    /// If a timestamp already exists on the surface, a timestamp_exists
    /// error is generated.
    ///
    /// Requesting set_timestamp after the commit_timer object's surface is
    /// destroyed will generate a "surface_destroyed" error.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of target time
    /// - `tv_sec_lo`: low 32 bits of the seconds part of target time
    /// - `tv_nsec`: nanoseconds part of target time
    #[inline]
    fn handle_set_timestamp(
        &mut self,
        slf: &Rc<WpCommitTimerV1>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_timestamp(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log_forward("wp_commit_timer_v1.set_timestamp", &e);
        }
    }

    /// Destroy the timer
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object.
    ///
    /// Existing timing constraints are not affected by the destruction.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpCommitTimerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_commit_timer_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for WpCommitTimerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpCommitTimerV1, version),
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
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_commit_timer_v1#{}.set_timestamp(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_timestamp(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_set_timestamp(&self, arg0, arg1, arg2);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_commit_timer_v1#{}.destroy()\n", client_id, id);
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
            0 => "set_timestamp",
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

impl Object for WpCommitTimerV1 {
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

impl WpCommitTimerV1 {
    /// Since when the error.invalid_timestamp enum variant is available.
    pub const ENM__ERROR_INVALID_TIMESTAMP__SINCE: u32 = 1;
    /// Since when the error.timestamp_exists enum variant is available.
    pub const ENM__ERROR_TIMESTAMP_EXISTS__SINCE: u32 = 1;
    /// Since when the error.surface_destroyed enum variant is available.
    pub const ENM__ERROR_SURFACE_DESTROYED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpCommitTimerV1Error(pub u32);

impl WpCommitTimerV1Error {
    /// timestamp contains an invalid value
    pub const INVALID_TIMESTAMP: Self = Self(0);

    /// timestamp exists
    pub const TIMESTAMP_EXISTS: Self = Self(1);

    /// the associated surface no longer exists
    pub const SURFACE_DESTROYED: Self = Self(2);
}

impl Debug for WpCommitTimerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_TIMESTAMP => "INVALID_TIMESTAMP",
            Self::TIMESTAMP_EXISTS => "TIMESTAMP_EXISTS",
            Self::SURFACE_DESTROYED => "SURFACE_DESTROYED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
