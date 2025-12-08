//! a surface displayed while the session is locked
//!
//! The client may use lock surfaces to display a screensaver, render a
//! dialog to enter a password and unlock the session, or however else it
//! sees fit.
//!
//! On binding this interface the compositor will immediately send the
//! first configure event. After making the ack_configure request in
//! response to this event the client should attach and commit the first
//! buffer. Committing the surface before acking the first configure is a
//! protocol error. Committing the surface with a null buffer at any time
//! is a protocol error.
//!
//! The compositor is free to handle keyboard/pointer focus for lock
//! surfaces however it chooses. A reasonable way to do this would be to
//! give the first lock surface created keyboard focus and change keyboard
//! focus if the user clicks on other surfaces.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_session_lock_surface_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtSessionLockSurfaceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtSessionLockSurfaceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtSessionLockSurfaceV1MessageHandler for DefaultMessageHandler { }

impl MetaExtSessionLockSurfaceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtSessionLockSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtSessionLockSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtSessionLockSurfaceV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the lock surface object
    ///
    /// This informs the compositor that the lock surface object will no
    /// longer be used.
    ///
    /// It is recommended for a lock client to destroy lock surfaces if
    /// their corresponding wl_output global is removed.
    ///
    /// If a lock surface on an active output is destroyed before the
    /// ext_session_lock_v1.unlock_and_destroy event is sent, the compositor
    /// must fall back to rendering a solid color.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the ack_configure message is available.
    #[allow(dead_code)]
    pub const MSG__ACK_CONFIGURE__SINCE: u32 = 1;

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the surface
    /// in response to the configure event, then the client must make an
    /// ack_configure request sometime before the commit request, passing
    /// along the serial of the configure event.
    ///
    /// If the client receives multiple configure events before it can
    /// respond to one, it only has to ack the last configure event.
    ///
    /// A client is not required to commit immediately after sending an
    /// ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing,
    /// but only the last request sent before a commit indicates which
    /// configure event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the configure event
    /// referenced by the given serial, as well as all older configure events
    /// sent on this object.
    ///
    /// It is a protocol error to issue multiple ack_configure requests
    /// referencing the same configure event or to issue an ack_configure
    /// request referencing a configure event older than the last configure
    /// event acked for a given lock surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from the configure event
    #[inline]
    pub fn send_ack_configure(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the configure message is available.
    #[allow(dead_code)]
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// the client should resize its surface
    ///
    /// This event is sent once on binding the interface and may be sent again
    /// at the compositor's discretion, for example if output geometry changes.
    ///
    /// The width and height are in surface-local coordinates and are exact
    /// requirements. Failing to match these surface dimensions in the next
    /// commit after acking a configure is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial for use in ack_configure
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_configure(
        &self,
        serial: u32,
        width: u32,
        height: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            serial,
            width,
            height,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtSessionLockSurfaceV1] proxies.
#[allow(dead_code)]
pub trait MetaExtSessionLockSurfaceV1MessageHandler {
    /// destroy the lock surface object
    ///
    /// This informs the compositor that the lock surface object will no
    /// longer be used.
    ///
    /// It is recommended for a lock client to destroy lock surfaces if
    /// their corresponding wl_output global is removed.
    ///
    /// If a lock surface on an active output is destroyed before the
    /// ext_session_lock_v1.unlock_and_destroy event is sent, the compositor
    /// must fall back to rendering a solid color.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtSessionLockSurfaceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_session_lock_surface_v1.destroy message: {}", Report::new(e));
        }
    }

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the surface
    /// in response to the configure event, then the client must make an
    /// ack_configure request sometime before the commit request, passing
    /// along the serial of the configure event.
    ///
    /// If the client receives multiple configure events before it can
    /// respond to one, it only has to ack the last configure event.
    ///
    /// A client is not required to commit immediately after sending an
    /// ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing,
    /// but only the last request sent before a commit indicates which
    /// configure event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the configure event
    /// referenced by the given serial, as well as all older configure events
    /// sent on this object.
    ///
    /// It is a protocol error to issue multiple ack_configure requests
    /// referencing the same configure event or to issue an ack_configure
    /// request referencing a configure event older than the last configure
    /// event acked for a given lock surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial from the configure event
    #[inline]
    fn ack_configure(
        &mut self,
        _slf: &Rc<MetaExtSessionLockSurfaceV1>,
        serial: u32,
    ) {
        let res = _slf.send_ack_configure(
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_session_lock_surface_v1.ack_configure message: {}", Report::new(e));
        }
    }

    /// the client should resize its surface
    ///
    /// This event is sent once on binding the interface and may be sent again
    /// at the compositor's discretion, for example if output geometry changes.
    ///
    /// The width and height are in surface-local coordinates and are exact
    /// requirements. Failing to match these surface dimensions in the next
    /// commit after acking a configure is a protocol error.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial for use in ack_configure
    /// - `width`:
    /// - `height`:
    #[inline]
    fn configure(
        &mut self,
        _slf: &Rc<MetaExtSessionLockSurfaceV1>,
        serial: u32,
        width: u32,
        height: u32,
    ) {
        let res = _slf.send_configure(
            serial,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_session_lock_surface_v1.configure message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtSessionLockSurfaceV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).ack_configure(&self, arg0);
                } else {
                    DefaultMessageHandler.ack_configure(&self, arg0);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).configure(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.configure(&self, arg0, arg1, arg2);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

impl MetaExtSessionLockSurfaceV1 {
    /// Since when the error.commit_before_first_ack enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_COMMIT_BEFORE_FIRST_ACK__SINCE: u32 = 1;
    /// Since when the error.null_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NULL_BUFFER__SINCE: u32 = 1;
    /// Since when the error.dimensions_mismatch enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DIMENSIONS_MISMATCH__SINCE: u32 = 1;
    /// Since when the error.invalid_serial enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SERIAL__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaExtSessionLockSurfaceV1Error(pub u32);

impl MetaExtSessionLockSurfaceV1Error {
    /// surface committed before first ack_configure request
    #[allow(dead_code)]
    pub const COMMIT_BEFORE_FIRST_ACK: Self = Self(0);

    /// surface committed with a null buffer
    #[allow(dead_code)]
    pub const NULL_BUFFER: Self = Self(1);

    /// failed to match ack'd width/height
    #[allow(dead_code)]
    pub const DIMENSIONS_MISMATCH: Self = Self(2);

    /// serial provided in ack_configure is invalid
    #[allow(dead_code)]
    pub const INVALID_SERIAL: Self = Self(3);
}

impl Debug for MetaExtSessionLockSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::COMMIT_BEFORE_FIRST_ACK => "COMMIT_BEFORE_FIRST_ACK",
            Self::NULL_BUFFER => "NULL_BUFFER",
            Self::DIMENSIONS_MISMATCH => "DIMENSIONS_MISMATCH",
            Self::INVALID_SERIAL => "INVALID_SERIAL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
