//! cursor capture session
//!
//! This object represents a cursor capture session. It extends the base
//! capture session with cursor-specific metadata.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_image_copy_capture_cursor_session_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtImageCopyCaptureCursorSessionV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn ExtImageCopyCaptureCursorSessionV1Handler>,
}

struct DefaultHandler;

impl ExtImageCopyCaptureCursorSessionV1Handler for DefaultHandler { }

impl ExtImageCopyCaptureCursorSessionV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "ext_image_copy_capture_cursor_session_v1";
}

impl ExtImageCopyCaptureCursorSessionV1 {
    pub fn set_handler(&self, handler: impl ExtImageCopyCaptureCursorSessionV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ExtImageCopyCaptureCursorSessionV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtImageCopyCaptureCursorSessionV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtImageCopyCaptureCursorSessionV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtImageCopyCaptureCursorSessionV1 {
    /// Since when the destroy message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_cursor_session_v1#{}.destroy()\n", id);
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

    /// Since when the get_capture_session message is available.
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
        session: &Rc<ExtImageCopyCaptureSessionV1>,
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_cursor_session_v1#{}.get_capture_session(session: ext_image_copy_capture_session_v1#{})\n", id, arg0_id);
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

    /// Since when the enter message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_cursor_session_v1#{}.enter()\n", client.endpoint.id, id);
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
        ]);
        Ok(())
    }

    /// Since when the leave message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_cursor_session_v1#{}.leave()\n", client.endpoint.id, id);
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
            1,
        ]);
        Ok(())
    }

    /// Since when the position message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_cursor_session_v1#{}.position(x: {}, y: {})\n", client.endpoint.id, id, arg0, arg1);
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
            2,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// Since when the hotspot message is available.
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
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ext_image_copy_capture_cursor_session_v1#{}.hotspot(x: {}, y: {})\n", client.endpoint.id, id, arg0, arg1);
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
            3,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtImageCopyCaptureCursorSessionV1] proxies.
pub trait ExtImageCopyCaptureCursorSessionV1Handler: Any {
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
        _slf: &Rc<ExtImageCopyCaptureCursorSessionV1>,
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
        _slf: &Rc<ExtImageCopyCaptureCursorSessionV1>,
        session: &Rc<ExtImageCopyCaptureSessionV1>,
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
        _slf: &Rc<ExtImageCopyCaptureCursorSessionV1>,
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
        _slf: &Rc<ExtImageCopyCaptureCursorSessionV1>,
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
        _slf: &Rc<ExtImageCopyCaptureCursorSessionV1>,
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
        _slf: &Rc<ExtImageCopyCaptureCursorSessionV1>,
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

impl ProxyPrivate for ExtImageCopyCaptureCursorSessionV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::ExtImageCopyCaptureCursorSessionV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_cursor_session_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_cursor_session_v1#{}.get_capture_session(session: ext_image_copy_capture_session_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = ExtImageCopyCaptureSessionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "session", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_capture_session(&self, arg0);
                } else {
                    DefaultHandler.get_capture_session(&self, arg0);
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_cursor_session_v1#{}.enter()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).enter(&self);
                } else {
                    DefaultHandler.enter(&self);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_cursor_session_v1#{}.leave()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).leave(&self);
                } else {
                    DefaultHandler.leave(&self);
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_cursor_session_v1#{}.position(x: {}, y: {})\n", msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).position(&self, arg0, arg1);
                } else {
                    DefaultHandler.position(&self, arg0, arg1);
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
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ext_image_copy_capture_cursor_session_v1#{}.hotspot(x: {}, y: {})\n", msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).hotspot(&self, arg0, arg1);
                } else {
                    DefaultHandler.hotspot(&self, arg0, arg1);
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

impl Proxy for ExtImageCopyCaptureCursorSessionV1 {
    fn core(&self) -> &ProxyCore {
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

impl ExtImageCopyCaptureCursorSessionV1 {
    /// Since when the error.duplicate_session enum variant is available.
    pub const ENM__ERROR_DUPLICATE_SESSION__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtImageCopyCaptureCursorSessionV1Error(pub u32);

impl ExtImageCopyCaptureCursorSessionV1Error {
    /// get_capture_session sent twice
    pub const DUPLICATE_SESSION: Self = Self(1);
}

impl Debug for ExtImageCopyCaptureCursorSessionV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DUPLICATE_SESSION => "DUPLICATE_SESSION",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
