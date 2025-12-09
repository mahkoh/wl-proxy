//! cursor capture session
//!
//! This object represents a cursor capture session. It extends the base
//! capture session with cursor-specific metadata.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_image_copy_capture_cursor_session_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtImageCopyCaptureCursorSessionV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtImageCopyCaptureCursorSessionV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtImageCopyCaptureCursorSessionV1MessageHandler for DefaultMessageHandler { }

impl MetaExtImageCopyCaptureCursorSessionV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaExtImageCopyCaptureCursorSessionV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtImageCopyCaptureCursorSessionV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaExtImageCopyCaptureCursorSessionV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaExtImageCopyCaptureCursorSessionV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtImageCopyCaptureCursorSessionV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtImageCopyCaptureCursorSessionV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object
    ///
    /// Destroys the session. This request can be sent at any time by the
    /// client.
    ///
    /// This request doesn't affect ext_image_copy_capture_frame_v1 objects created by
    /// this object.
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

    /// Since when the get_capture_session message is available.
    #[allow(dead_code)]
    pub const MSG__GET_CAPTURE_SESSION__SINCE: u32 = 1;

    /// get image copy capturer session
    ///
    /// Gets the image copy capture session for this cursor session.
    ///
    /// The session will produce frames of the cursor image. The compositor may
    /// pause the session when the cursor leaves the captured area.
    ///
    /// This request must not be sent more than once, or else the
    /// duplicate_session protocol error is raised.
    #[inline]
    pub fn send_get_capture_session(
        &self,
        session: &Rc<MetaExtImageCopyCaptureSessionV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            session,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("session", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
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
        ]);
        Ok(())
    }

    /// Since when the enter message is available.
    #[allow(dead_code)]
    pub const MSG__ENTER__SINCE: u32 = 1;

    /// cursor entered captured area
    ///
    /// Sent when a cursor enters the captured area. It shall be generated
    /// before the "position" and "hotspot" events when and only when a cursor
    /// enters the area.
    ///
    /// The cursor enters the captured area when the cursor image intersects
    /// with the captured area. Note, this is different from e.g.
    /// wl_pointer.enter.
    #[inline]
    pub fn send_enter(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
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
        Ok(())
    }

    /// Since when the leave message is available.
    #[allow(dead_code)]
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// cursor left captured area
    ///
    /// Sent when a cursor leaves the captured area. No "position" or "hotspot"
    /// event is generated for the cursor until the cursor enters the captured
    /// area again.
    #[inline]
    pub fn send_leave(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the position message is available.
    #[allow(dead_code)]
    pub const MSG__POSITION__SINCE: u32 = 1;

    /// position changed
    ///
    /// Cursors outside the image capture source do not get captured and no
    /// event will be generated for them.
    ///
    /// The given position is the position of the cursor's hotspot and it is
    /// relative to the main buffer's top left corner in transformed buffer
    /// pixel coordinates. The coordinates may be negative or greater than the
    /// main buffer size.
    ///
    /// # Arguments
    ///
    /// - `x`: position x coordinates
    /// - `y`: position y coordinates
    #[inline]
    pub fn send_position(
        &self,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            x,
            y,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the hotspot message is available.
    #[allow(dead_code)]
    pub const MSG__HOTSPOT__SINCE: u32 = 1;

    /// hotspot changed
    ///
    /// The hotspot describes the offset between the cursor image and the
    /// position of the input device.
    ///
    /// The given coordinates are the hotspot's offset from the origin in
    /// buffer coordinates.
    ///
    /// Clients should not apply the hotspot immediately: the hotspot becomes
    /// effective when the next ext_image_copy_capture_frame_v1.ready event is received.
    ///
    /// Compositors may delay this event until the client captures a new frame.
    ///
    /// # Arguments
    ///
    /// - `x`: hotspot x coordinates
    /// - `y`: hotspot y coordinates
    #[inline]
    pub fn send_hotspot(
        &self,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            x,
            y,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtImageCopyCaptureCursorSessionV1] proxies.
#[allow(dead_code)]
pub trait MetaExtImageCopyCaptureCursorSessionV1MessageHandler {
    /// delete this object
    ///
    /// Destroys the session. This request can be sent at any time by the
    /// client.
    ///
    /// This request doesn't affect ext_image_copy_capture_frame_v1 objects created by
    /// this object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureCursorSessionV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_cursor_session_v1.destroy message: {}", Report::new(e));
        }
    }

    /// get image copy capturer session
    ///
    /// Gets the image copy capture session for this cursor session.
    ///
    /// The session will produce frames of the cursor image. The compositor may
    /// pause the session when the cursor leaves the captured area.
    ///
    /// This request must not be sent more than once, or else the
    /// duplicate_session protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `session`:
    #[inline]
    fn get_capture_session(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureCursorSessionV1>,
        session: &Rc<MetaExtImageCopyCaptureSessionV1>,
    ) {
        let res = _slf.send_get_capture_session(
            session,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_cursor_session_v1.get_capture_session message: {}", Report::new(e));
        }
    }

    /// cursor entered captured area
    ///
    /// Sent when a cursor enters the captured area. It shall be generated
    /// before the "position" and "hotspot" events when and only when a cursor
    /// enters the area.
    ///
    /// The cursor enters the captured area when the cursor image intersects
    /// with the captured area. Note, this is different from e.g.
    /// wl_pointer.enter.
    #[inline]
    fn enter(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureCursorSessionV1>,
    ) {
        let res = _slf.send_enter(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_cursor_session_v1.enter message: {}", Report::new(e));
        }
    }

    /// cursor left captured area
    ///
    /// Sent when a cursor leaves the captured area. No "position" or "hotspot"
    /// event is generated for the cursor until the cursor enters the captured
    /// area again.
    #[inline]
    fn leave(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureCursorSessionV1>,
    ) {
        let res = _slf.send_leave(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_cursor_session_v1.leave message: {}", Report::new(e));
        }
    }

    /// position changed
    ///
    /// Cursors outside the image capture source do not get captured and no
    /// event will be generated for them.
    ///
    /// The given position is the position of the cursor's hotspot and it is
    /// relative to the main buffer's top left corner in transformed buffer
    /// pixel coordinates. The coordinates may be negative or greater than the
    /// main buffer size.
    ///
    /// # Arguments
    ///
    /// - `x`: position x coordinates
    /// - `y`: position y coordinates
    #[inline]
    fn position(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureCursorSessionV1>,
        x: i32,
        y: i32,
    ) {
        let res = _slf.send_position(
            x,
            y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_cursor_session_v1.position message: {}", Report::new(e));
        }
    }

    /// hotspot changed
    ///
    /// The hotspot describes the offset between the cursor image and the
    /// position of the input device.
    ///
    /// The given coordinates are the hotspot's offset from the origin in
    /// buffer coordinates.
    ///
    /// Clients should not apply the hotspot immediately: the hotspot becomes
    /// effective when the next ext_image_copy_capture_frame_v1.ready event is received.
    ///
    /// Compositors may delay this event until the client captures a new frame.
    ///
    /// # Arguments
    ///
    /// - `x`: hotspot x coordinates
    /// - `y`: hotspot y coordinates
    #[inline]
    fn hotspot(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureCursorSessionV1>,
        x: i32,
        y: i32,
    ) {
        let res = _slf.send_hotspot(
            x,
            y,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_cursor_session_v1.hotspot message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtImageCopyCaptureCursorSessionV1 {
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
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaExtImageCopyCaptureSessionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "session", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_capture_session(&self, arg0);
                } else {
                    DefaultMessageHandler.get_capture_session(&self, arg0);
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
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).enter(&self);
                } else {
                    DefaultMessageHandler.enter(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).leave(&self);
                } else {
                    DefaultMessageHandler.leave(&self);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).position(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.position(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).hotspot(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.hotspot(&self, arg0, arg1);
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
            1 => "get_capture_session",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "enter",
            1 => "leave",
            2 => "position",
            3 => "hotspot",
            _ => return None,
        };
        Some(name)
    }
}

impl MetaExtImageCopyCaptureCursorSessionV1 {
    /// Since when the error.duplicate_session enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DUPLICATE_SESSION__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaExtImageCopyCaptureCursorSessionV1Error(pub u32);

impl MetaExtImageCopyCaptureCursorSessionV1Error {
    /// get_capture_session sent twice
    #[allow(dead_code)]
    pub const DUPLICATE_SESSION: Self = Self(1);
}

impl Debug for MetaExtImageCopyCaptureCursorSessionV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DUPLICATE_SESSION => "DUPLICATE_SESSION",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
