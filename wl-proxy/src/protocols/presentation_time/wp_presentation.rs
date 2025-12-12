//! timed presentation related wl_surface requests
//!
//! The main feature of this interface is accurate presentation
//! timing feedback to ensure smooth video playback while maintaining
//! audio/video synchronization. Some features use the concept of a
//! presentation clock, which is defined in the
//! presentation.clock_id event.
//!
//! A content update for a wl_surface is submitted by a
//! wl_surface.commit request. Request 'feedback' associates with
//! the wl_surface.commit and provides feedback on the content
//! update, particularly the final realized presentation time.
//!
//!
//!
//! When the final realized presentation time is available, e.g.
//! after a framebuffer flip completes, the requested
//! presentation_feedback.presented events are sent. The final
//! presentation time can differ from the compositor's predicted
//! display update time and the update's target time, especially
//! when the compositor misses its target vertical blanking period.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_presentation object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpPresentation {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpPresentationHandler>,
}

struct DefaultHandler;

impl WpPresentationHandler for DefaultHandler { }

impl WpPresentation {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: ObjectInterface = ObjectInterface::WpPresentation;
    pub const INTERFACE_NAME: &str = "wp_presentation";
}

impl WpPresentation {
    pub fn set_handler(&self, handler: impl WpPresentationHandler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpPresentationHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpPresentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpPresentation")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpPresentation {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unbind from the presentation interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object. Existing objects created by this object
    /// are not affected.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_presentation#{}.destroy()\n", id);
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

    /// Since when the feedback message is available.
    pub const MSG__FEEDBACK__SINCE: u32 = 1;

    /// request presentation feedback information
    ///
    /// Request presentation feedback for the current content submission
    /// on the given surface. This creates a new presentation_feedback
    /// object, which will deliver the feedback information once. If
    /// multiple presentation_feedback objects are created for the same
    /// submission, they will all deliver the same information.
    ///
    /// For details on what information is returned, see the
    /// presentation_feedback interface.
    ///
    /// # Arguments
    ///
    /// - `surface`: target surface
    /// - `callback`: new feedback object
    #[inline]
    pub fn send_feedback(
        &self,
        surface: &Rc<WlSurface>,
        callback: &Rc<WpPresentationFeedback>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            surface,
            callback,
        );
        let arg0 = arg0.core();
        let arg1_obj = arg1;
        let arg1 = arg1_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        arg1.generate_server_id(arg1_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("callback", e))?;
        let arg1_id = arg1.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_presentation#{}.feedback(surface: wl_surface#{}, callback: wp_presentation_feedback#{})\n", id, arg0_id, arg1_id);
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
            arg1_id,
        ]);
        Ok(())
    }

    /// Since when the clock_id message is available.
    pub const MSG__CLOCK_ID__SINCE: u32 = 1;

    /// clock ID for timestamps
    ///
    /// This event tells the client in which clock domain the
    /// compositor interprets the timestamps used by the presentation
    /// extension. This clock is called the presentation clock.
    ///
    /// The compositor sends this event when the client binds to the
    /// presentation interface. The presentation clock does not change
    /// during the lifetime of the client connection.
    ///
    /// The clock identifier is platform dependent. On POSIX platforms, the
    /// identifier value is one of the clockid_t values accepted by
    /// clock_gettime(). clock_gettime() is defined by POSIX.1-2001.
    ///
    /// Timestamps in this clock domain are expressed as tv_sec_hi,
    /// tv_sec_lo, tv_nsec triples, each component being an unsigned
    /// 32-bit value. Whole seconds are in tv_sec which is a 64-bit
    /// value combined from tv_sec_hi and tv_sec_lo, and the
    /// additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999].
    ///
    /// Note that clock_id applies only to the presentation clock,
    /// and implies nothing about e.g. the timestamps used in the
    /// Wayland core protocol input events.
    ///
    /// Compositors should prefer a clock which does not jump and is
    /// not slewed e.g. by NTP. The absolute value of the clock is
    /// irrelevant. Precision of one millisecond or better is
    /// recommended. Clients must be able to query the current clock
    /// value directly, not by asking the compositor.
    ///
    /// # Arguments
    ///
    /// - `clk_id`: platform clock identifier
    #[inline]
    pub fn send_clock_id(
        &self,
        clk_id: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            clk_id,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_presentation#{}.clock_id(clk_id: {})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [WpPresentation] proxies.
pub trait WpPresentationHandler: Any {
    /// unbind from the presentation interface
    ///
    /// Informs the server that the client will no longer be using
    /// this protocol object. Existing objects created by this object
    /// are not affected.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WpPresentation>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_presentation.destroy message: {}", Report::new(e));
        }
    }

    /// request presentation feedback information
    ///
    /// Request presentation feedback for the current content submission
    /// on the given surface. This creates a new presentation_feedback
    /// object, which will deliver the feedback information once. If
    /// multiple presentation_feedback objects are created for the same
    /// submission, they will all deliver the same information.
    ///
    /// For details on what information is returned, see the
    /// presentation_feedback interface.
    ///
    /// # Arguments
    ///
    /// - `surface`: target surface
    /// - `callback`: new feedback object
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn feedback(
        &mut self,
        _slf: &Rc<WpPresentation>,
        surface: &Rc<WlSurface>,
        callback: &Rc<WpPresentationFeedback>,
    ) {
        let res = _slf.send_feedback(
            surface,
            callback,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_presentation.feedback message: {}", Report::new(e));
        }
    }

    /// clock ID for timestamps
    ///
    /// This event tells the client in which clock domain the
    /// compositor interprets the timestamps used by the presentation
    /// extension. This clock is called the presentation clock.
    ///
    /// The compositor sends this event when the client binds to the
    /// presentation interface. The presentation clock does not change
    /// during the lifetime of the client connection.
    ///
    /// The clock identifier is platform dependent. On POSIX platforms, the
    /// identifier value is one of the clockid_t values accepted by
    /// clock_gettime(). clock_gettime() is defined by POSIX.1-2001.
    ///
    /// Timestamps in this clock domain are expressed as tv_sec_hi,
    /// tv_sec_lo, tv_nsec triples, each component being an unsigned
    /// 32-bit value. Whole seconds are in tv_sec which is a 64-bit
    /// value combined from tv_sec_hi and tv_sec_lo, and the
    /// additional fractional part in tv_nsec as nanoseconds. Hence,
    /// for valid timestamps tv_nsec must be in [0, 999999999].
    ///
    /// Note that clock_id applies only to the presentation clock,
    /// and implies nothing about e.g. the timestamps used in the
    /// Wayland core protocol input events.
    ///
    /// Compositors should prefer a clock which does not jump and is
    /// not slewed e.g. by NTP. The absolute value of the clock is
    /// irrelevant. Precision of one millisecond or better is
    /// recommended. Clients must be able to query the current clock
    /// value directly, not by asking the compositor.
    ///
    /// # Arguments
    ///
    /// - `clk_id`: platform clock identifier
    #[inline]
    fn clock_id(
        &mut self,
        _slf: &Rc<WpPresentation>,
        clk_id: u32,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_clock_id(
            clk_id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_presentation.clock_id message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for WpPresentation {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpPresentation, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_presentation#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_presentation#{}.feedback(surface: wl_surface#{}, callback: wp_presentation_feedback#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface));
                };
                let arg1_id = arg1;
                let arg1 = WpPresentationFeedback::new(&self.core.state, self.core.version);
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError::SetClientId(arg1_id, "callback", e))?;
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).feedback(&self, arg0, arg1);
                } else {
                    DefaultHandler.feedback(&self, arg0, arg1);
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_presentation#{}.clock_id(clk_id: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).clock_id(&self, arg0);
                } else {
                    DefaultHandler.clock_id(&self, arg0);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "feedback",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "clock_id",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WpPresentation {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl WpPresentation {
    /// Since when the error.invalid_timestamp enum variant is available.
    pub const ENM__ERROR_INVALID_TIMESTAMP__SINCE: u32 = 1;
    /// Since when the error.invalid_flag enum variant is available.
    pub const ENM__ERROR_INVALID_FLAG__SINCE: u32 = 1;
}

/// fatal presentation errors
///
/// These fatal protocol errors may be emitted in response to
/// illegal presentation requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpPresentationError(pub u32);

impl WpPresentationError {
    /// invalid value in tv_nsec
    pub const INVALID_TIMESTAMP: Self = Self(0);

    /// invalid flag
    pub const INVALID_FLAG: Self = Self(1);
}

impl Debug for WpPresentationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_TIMESTAMP => "INVALID_TIMESTAMP",
            Self::INVALID_FLAG => "INVALID_FLAG",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
