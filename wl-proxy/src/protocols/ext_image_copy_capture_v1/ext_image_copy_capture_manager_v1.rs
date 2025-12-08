//! manager to inform clients and begin capturing
//!
//! This object is a manager which offers requests to start capturing from a
//! source.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ext_image_copy_capture_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ExtImageCopyCaptureManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ExtImageCopyCaptureManagerV1Handler>,
}

struct DefaultHandler;

impl ExtImageCopyCaptureManagerV1Handler for DefaultHandler { }

impl ConcreteObject for ExtImageCopyCaptureManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ExtImageCopyCaptureManagerV1;
    const INTERFACE_NAME: &str = "ext_image_copy_capture_manager_v1";
}

impl ExtImageCopyCaptureManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ExtImageCopyCaptureManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ExtImageCopyCaptureManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ExtImageCopyCaptureManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtImageCopyCaptureManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ExtImageCopyCaptureManagerV1 {
    /// Since when the create_session message is available.
    pub const MSG__CREATE_SESSION__SINCE: u32 = 1;

    /// capture an image capture source
    ///
    /// Create a capturing session for an image capture source.
    ///
    /// If the paint_cursors option is set, cursors shall be composited onto
    /// the captured frame. The cursor must not be composited onto the frame
    /// if this flag is not set.
    ///
    /// If the options bitfield is invalid, the invalid_option protocol error
    /// is sent.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `source`:
    /// - `options`:
    #[inline]
    pub fn try_send_create_session(
        &self,
        session: &Rc<ExtImageCopyCaptureSessionV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
        options: ExtImageCopyCaptureManagerV1Options,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            session,
            source,
            options,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("source"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("session", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: ExtImageCopyCaptureManagerV1Options) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_manager_v1#{}.create_session(session: ext_image_copy_capture_session_v1#{}, source: ext_image_capture_source_v1#{}, options: {:?})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2);
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
            arg0_id,
            arg1_id,
            arg2.0,
        ]);
        Ok(())
    }

    /// capture an image capture source
    ///
    /// Create a capturing session for an image capture source.
    ///
    /// If the paint_cursors option is set, cursors shall be composited onto
    /// the captured frame. The cursor must not be composited onto the frame
    /// if this flag is not set.
    ///
    /// If the options bitfield is invalid, the invalid_option protocol error
    /// is sent.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `source`:
    /// - `options`:
    #[inline]
    pub fn send_create_session(
        &self,
        session: &Rc<ExtImageCopyCaptureSessionV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
        options: ExtImageCopyCaptureManagerV1Options,
    ) {
        let res = self.try_send_create_session(
            session,
            source,
            options,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_manager_v1.create_session", &e);
        }
    }

    /// capture an image capture source
    ///
    /// Create a capturing session for an image capture source.
    ///
    /// If the paint_cursors option is set, cursors shall be composited onto
    /// the captured frame. The cursor must not be composited onto the frame
    /// if this flag is not set.
    ///
    /// If the options bitfield is invalid, the invalid_option protocol error
    /// is sent.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `options`:
    #[inline]
    pub fn new_try_send_create_session(
        &self,
        source: &Rc<ExtImageCaptureSourceV1>,
        options: ExtImageCopyCaptureManagerV1Options,
    ) -> Result<Rc<ExtImageCopyCaptureSessionV1>, ObjectError> {
        let session = self.core.create_child();
        self.try_send_create_session(
            &session,
            source,
            options,
        )?;
        Ok(session)
    }

    /// capture an image capture source
    ///
    /// Create a capturing session for an image capture source.
    ///
    /// If the paint_cursors option is set, cursors shall be composited onto
    /// the captured frame. The cursor must not be composited onto the frame
    /// if this flag is not set.
    ///
    /// If the options bitfield is invalid, the invalid_option protocol error
    /// is sent.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `options`:
    #[inline]
    pub fn new_send_create_session(
        &self,
        source: &Rc<ExtImageCaptureSourceV1>,
        options: ExtImageCopyCaptureManagerV1Options,
    ) -> Rc<ExtImageCopyCaptureSessionV1> {
        let session = self.core.create_child();
        self.send_create_session(
            &session,
            source,
            options,
        );
        session
    }

    /// Since when the create_pointer_cursor_session message is available.
    pub const MSG__CREATE_POINTER_CURSOR_SESSION__SINCE: u32 = 1;

    /// capture the pointer cursor of an image capture source
    ///
    /// Create a cursor capturing session for the pointer of an image capture
    /// source.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `source`:
    /// - `pointer`:
    #[inline]
    pub fn try_send_create_pointer_cursor_session(
        &self,
        session: &Rc<ExtImageCopyCaptureCursorSessionV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
        pointer: &Rc<WlPointer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            session,
            source,
            pointer,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("source"))),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("pointer"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("session", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_manager_v1#{}.create_pointer_cursor_session(session: ext_image_copy_capture_cursor_session_v1#{}, source: ext_image_capture_source_v1#{}, pointer: wl_pointer#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2_id);
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
            arg2_id,
        ]);
        Ok(())
    }

    /// capture the pointer cursor of an image capture source
    ///
    /// Create a cursor capturing session for the pointer of an image capture
    /// source.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `source`:
    /// - `pointer`:
    #[inline]
    pub fn send_create_pointer_cursor_session(
        &self,
        session: &Rc<ExtImageCopyCaptureCursorSessionV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
        pointer: &Rc<WlPointer>,
    ) {
        let res = self.try_send_create_pointer_cursor_session(
            session,
            source,
            pointer,
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_manager_v1.create_pointer_cursor_session", &e);
        }
    }

    /// capture the pointer cursor of an image capture source
    ///
    /// Create a cursor capturing session for the pointer of an image capture
    /// source.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `pointer`:
    #[inline]
    pub fn new_try_send_create_pointer_cursor_session(
        &self,
        source: &Rc<ExtImageCaptureSourceV1>,
        pointer: &Rc<WlPointer>,
    ) -> Result<Rc<ExtImageCopyCaptureCursorSessionV1>, ObjectError> {
        let session = self.core.create_child();
        self.try_send_create_pointer_cursor_session(
            &session,
            source,
            pointer,
        )?;
        Ok(session)
    }

    /// capture the pointer cursor of an image capture source
    ///
    /// Create a cursor capturing session for the pointer of an image capture
    /// source.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `pointer`:
    #[inline]
    pub fn new_send_create_pointer_cursor_session(
        &self,
        source: &Rc<ExtImageCaptureSourceV1>,
        pointer: &Rc<WlPointer>,
    ) -> Rc<ExtImageCopyCaptureCursorSessionV1> {
        let session = self.core.create_child();
        self.send_create_pointer_cursor_session(
            &session,
            source,
            pointer,
        );
        session
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the manager object.
    ///
    /// Other objects created via this interface are unaffected.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ext_image_copy_capture_manager_v1#{}.destroy()\n", id);
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the manager
    ///
    /// Destroy the manager object.
    ///
    /// Other objects created via this interface are unaffected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("ext_image_copy_capture_manager_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ExtImageCopyCaptureManagerV1`] proxies.
pub trait ExtImageCopyCaptureManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ExtImageCopyCaptureManagerV1>) {
        slf.core.delete_id();
    }

    /// capture an image capture source
    ///
    /// Create a capturing session for an image capture source.
    ///
    /// If the paint_cursors option is set, cursors shall be composited onto
    /// the captured frame. The cursor must not be composited onto the frame
    /// if this flag is not set.
    ///
    /// If the options bitfield is invalid, the invalid_option protocol error
    /// is sent.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `source`:
    /// - `options`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_create_session(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureManagerV1>,
        session: &Rc<ExtImageCopyCaptureSessionV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
        options: ExtImageCopyCaptureManagerV1Options,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_session(
            session,
            source,
            options,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_manager_v1.create_session", &e);
        }
    }

    /// capture the pointer cursor of an image capture source
    ///
    /// Create a cursor capturing session for the pointer of an image capture
    /// source.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `source`:
    /// - `pointer`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_create_pointer_cursor_session(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureManagerV1>,
        session: &Rc<ExtImageCopyCaptureCursorSessionV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
        pointer: &Rc<WlPointer>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_pointer_cursor_session(
            session,
            source,
            pointer,
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_manager_v1.create_pointer_cursor_session", &e);
        }
    }

    /// destroy the manager
    ///
    /// Destroy the manager object.
    ///
    /// Other objects created via this interface are unaffected.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ExtImageCopyCaptureManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("ext_image_copy_capture_manager_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ExtImageCopyCaptureManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ExtImageCopyCaptureManagerV1, version),
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
                let arg2 = ExtImageCopyCaptureManagerV1Options(arg2);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: ExtImageCopyCaptureManagerV1Options) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_manager_v1#{}.create_session(session: ext_image_copy_capture_session_v1#{}, source: ext_image_capture_source_v1#{}, options: {:?})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = ExtImageCopyCaptureSessionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "session", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ExtImageCaptureSourceV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("source", o.core().interface, ObjectInterface::ExtImageCaptureSourceV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_create_session(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_create_session(&self, arg0, arg1, arg2);
                }
            }
            1 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_manager_v1#{}.create_pointer_cursor_session(session: ext_image_copy_capture_cursor_session_v1#{}, source: ext_image_capture_source_v1#{}, pointer: wl_pointer#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = ExtImageCopyCaptureCursorSessionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "session", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ExtImageCaptureSourceV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("source", o.core().interface, ObjectInterface::ExtImageCaptureSourceV1)));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlPointer>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("pointer", o.core().interface, ObjectInterface::WlPointer)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_create_pointer_cursor_session(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_create_pointer_cursor_session(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ext_image_copy_capture_manager_v1#{}.destroy()\n", client_id, id);
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
            0 => "create_session",
            1 => "create_pointer_cursor_session",
            2 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ExtImageCopyCaptureManagerV1 {
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

impl ExtImageCopyCaptureManagerV1 {
    /// Since when the error.invalid_option enum variant is available.
    pub const ENM__ERROR_INVALID_OPTION__SINCE: u32 = 1;

    /// Since when the options.paint_cursors enum variant is available.
    pub const ENM__OPTIONS_PAINT_CURSORS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtImageCopyCaptureManagerV1Error(pub u32);

impl ExtImageCopyCaptureManagerV1Error {
    /// invalid option flag
    pub const INVALID_OPTION: Self = Self(1);
}

impl Debug for ExtImageCopyCaptureManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_OPTION => "INVALID_OPTION",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct ExtImageCopyCaptureManagerV1Options(pub u32);

/// An iterator over the set bits in a [`ExtImageCopyCaptureManagerV1Options`].
///
/// You can construct this with the `IntoIterator` implementation of `ExtImageCopyCaptureManagerV1Options`.
#[derive(Clone, Debug)]
pub struct ExtImageCopyCaptureManagerV1OptionsIter(pub u32);

impl ExtImageCopyCaptureManagerV1Options {
    /// paint cursors onto captured frames
    pub const PAINT_CURSORS: Self = Self(1);
}

impl ExtImageCopyCaptureManagerV1Options {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 1)
    }
}

impl Iterator for ExtImageCopyCaptureManagerV1OptionsIter {
    type Item = ExtImageCopyCaptureManagerV1Options;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(ExtImageCopyCaptureManagerV1Options(bit))
    }
}

impl IntoIterator for ExtImageCopyCaptureManagerV1Options {
    type Item = ExtImageCopyCaptureManagerV1Options;
    type IntoIter = ExtImageCopyCaptureManagerV1OptionsIter;

    fn into_iter(self) -> Self::IntoIter {
        ExtImageCopyCaptureManagerV1OptionsIter(self.0)
    }
}

impl BitAnd for ExtImageCopyCaptureManagerV1Options {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for ExtImageCopyCaptureManagerV1Options {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for ExtImageCopyCaptureManagerV1Options {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for ExtImageCopyCaptureManagerV1Options {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for ExtImageCopyCaptureManagerV1Options {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for ExtImageCopyCaptureManagerV1Options {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for ExtImageCopyCaptureManagerV1Options {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for ExtImageCopyCaptureManagerV1Options {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for ExtImageCopyCaptureManagerV1Options {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for ExtImageCopyCaptureManagerV1Options {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("PAINT_CURSORS")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}
