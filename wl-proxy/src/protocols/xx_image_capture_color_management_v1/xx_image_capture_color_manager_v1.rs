//! global to add color-management support for image captures
//!
//! This global allows clients to retrieve information about the preferred
//! image description of a capture source and to define the image description
//! of a capture frame.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xx_image_capture_color_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XxImageCaptureColorManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XxImageCaptureColorManagerV1Handler>,
}

struct DefaultHandler;

impl XxImageCaptureColorManagerV1Handler for DefaultHandler { }

impl ConcreteObject for XxImageCaptureColorManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XxImageCaptureColorManagerV1;
    const INTERFACE_NAME: &str = "xx_image_capture_color_manager_v1";
}

impl XxImageCaptureColorManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XxImageCaptureColorManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XxImageCaptureColorManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XxImageCaptureColorManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XxImageCaptureColorManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XxImageCaptureColorManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this object
    ///
    /// Destroys the object. This request can be sent at any time by the
    /// client.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_image_capture_color_manager_v1#{}.destroy()\n", id);
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

    /// destroy this object
    ///
    /// Destroys the object. This request can be sent at any time by the
    /// client.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xx_image_capture_color_manager_v1.destroy", &e);
        }
    }

    /// Since when the get_capture_source_colors message is available.
    pub const MSG__GET_CAPTURE_SOURCE_COLORS__SINCE: u32 = 1;

    /// create a color management interface for an ext_image_capture_source_v1
    ///
    /// This creates a new xx_image_capture_source_colors_v1 for a given
    /// ext_image_capture_source_v1.
    ///
    /// See xx_image_capture_source_colors_v1 for more details.
    ///
    /// # Arguments
    ///
    /// - `colors`:
    /// - `source`:
    #[inline]
    pub fn try_send_get_capture_source_colors(
        &self,
        colors: &Rc<XxImageCaptureSourceColorsV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            colors,
            source,
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
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("colors", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_image_capture_color_manager_v1#{}.get_capture_source_colors(colors: xx_image_capture_source_colors_v1#{}, source: ext_image_capture_source_v1#{})\n", id, arg0, arg1);
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

    /// create a color management interface for an ext_image_capture_source_v1
    ///
    /// This creates a new xx_image_capture_source_colors_v1 for a given
    /// ext_image_capture_source_v1.
    ///
    /// See xx_image_capture_source_colors_v1 for more details.
    ///
    /// # Arguments
    ///
    /// - `colors`:
    /// - `source`:
    #[inline]
    pub fn send_get_capture_source_colors(
        &self,
        colors: &Rc<XxImageCaptureSourceColorsV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
    ) {
        let res = self.try_send_get_capture_source_colors(
            colors,
            source,
        );
        if let Err(e) = res {
            log_send("xx_image_capture_color_manager_v1.get_capture_source_colors", &e);
        }
    }

    /// create a color management interface for an ext_image_capture_source_v1
    ///
    /// This creates a new xx_image_capture_source_colors_v1 for a given
    /// ext_image_capture_source_v1.
    ///
    /// See xx_image_capture_source_colors_v1 for more details.
    ///
    /// # Arguments
    ///
    /// - `source`:
    #[inline]
    pub fn new_try_send_get_capture_source_colors(
        &self,
        source: &Rc<ExtImageCaptureSourceV1>,
    ) -> Result<Rc<XxImageCaptureSourceColorsV1>, ObjectError> {
        let colors = self.core.create_child();
        self.try_send_get_capture_source_colors(
            &colors,
            source,
        )?;
        Ok(colors)
    }

    /// create a color management interface for an ext_image_capture_source_v1
    ///
    /// This creates a new xx_image_capture_source_colors_v1 for a given
    /// ext_image_capture_source_v1.
    ///
    /// See xx_image_capture_source_colors_v1 for more details.
    ///
    /// # Arguments
    ///
    /// - `source`:
    #[inline]
    pub fn new_send_get_capture_source_colors(
        &self,
        source: &Rc<ExtImageCaptureSourceV1>,
    ) -> Rc<XxImageCaptureSourceColorsV1> {
        let colors = self.core.create_child();
        self.send_get_capture_source_colors(
            &colors,
            source,
        );
        colors
    }

    /// Since when the set_frame_image_description message is available.
    pub const MSG__SET_FRAME_IMAGE_DESCRIPTION__SINCE: u32 = 1;

    /// set the image description of a frame
    ///
    /// If the frame is successfully captured, it will be encoded according to
    /// this image description.
    ///
    /// This request must not be used if the image description is not ready.
    /// Otherwise the image_description_not_ready error is emitted.
    ///
    /// This request must not be used if the frame has already been captured.
    /// Otherwise the already_captured error is emitted.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `image_description`:
    #[inline]
    pub fn try_send_set_frame_image_description(
        &self,
        frame: &Rc<ExtImageCopyCaptureFrameV1>,
        image_description: &Rc<WpImageDescriptionV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            frame,
            image_description,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("frame"))),
            Some(id) => id,
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("image_description"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_image_capture_color_manager_v1#{}.set_frame_image_description(frame: ext_image_copy_capture_frame_v1#{}, image_description: wp_image_description_v1#{})\n", id, arg0, arg1);
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
            2,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// set the image description of a frame
    ///
    /// If the frame is successfully captured, it will be encoded according to
    /// this image description.
    ///
    /// This request must not be used if the image description is not ready.
    /// Otherwise the image_description_not_ready error is emitted.
    ///
    /// This request must not be used if the frame has already been captured.
    /// Otherwise the already_captured error is emitted.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `image_description`:
    #[inline]
    pub fn send_set_frame_image_description(
        &self,
        frame: &Rc<ExtImageCopyCaptureFrameV1>,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        let res = self.try_send_set_frame_image_description(
            frame,
            image_description,
        );
        if let Err(e) = res {
            log_send("xx_image_capture_color_manager_v1.set_frame_image_description", &e);
        }
    }
}

/// A message handler for [`XxImageCaptureColorManagerV1`] proxies.
pub trait XxImageCaptureColorManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XxImageCaptureColorManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy this object
    ///
    /// Destroys the object. This request can be sent at any time by the
    /// client.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XxImageCaptureColorManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xx_image_capture_color_manager_v1.destroy", &e);
        }
    }

    /// create a color management interface for an ext_image_capture_source_v1
    ///
    /// This creates a new xx_image_capture_source_colors_v1 for a given
    /// ext_image_capture_source_v1.
    ///
    /// See xx_image_capture_source_colors_v1 for more details.
    ///
    /// # Arguments
    ///
    /// - `colors`:
    /// - `source`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_capture_source_colors(
        &mut self,
        slf: &Rc<XxImageCaptureColorManagerV1>,
        colors: &Rc<XxImageCaptureSourceColorsV1>,
        source: &Rc<ExtImageCaptureSourceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_capture_source_colors(
            colors,
            source,
        );
        if let Err(e) = res {
            log_forward("xx_image_capture_color_manager_v1.get_capture_source_colors", &e);
        }
    }

    /// set the image description of a frame
    ///
    /// If the frame is successfully captured, it will be encoded according to
    /// this image description.
    ///
    /// This request must not be used if the image description is not ready.
    /// Otherwise the image_description_not_ready error is emitted.
    ///
    /// This request must not be used if the frame has already been captured.
    /// Otherwise the already_captured error is emitted.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `image_description`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_frame_image_description(
        &mut self,
        slf: &Rc<XxImageCaptureColorManagerV1>,
        frame: &Rc<ExtImageCopyCaptureFrameV1>,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_frame_image_description(
            frame,
            image_description,
        );
        if let Err(e) = res {
            log_forward("xx_image_capture_color_manager_v1.set_frame_image_description", &e);
        }
    }
}

impl ObjectPrivate for XxImageCaptureColorManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XxImageCaptureColorManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_image_capture_color_manager_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_image_capture_color_manager_v1#{}.get_capture_source_colors(colors: xx_image_capture_source_colors_v1#{}, source: ext_image_capture_source_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = XxImageCaptureSourceColorsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "colors", e)))?;
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
                    (**handler).handle_get_capture_source_colors(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_capture_source_colors(&self, arg0, arg1);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_image_capture_color_manager_v1#{}.set_frame_image_description(frame: ext_image_copy_capture_frame_v1#{}, image_description: wp_image_description_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<ExtImageCopyCaptureFrameV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("frame", o.core().interface, ObjectInterface::ExtImageCopyCaptureFrameV1)));
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WpImageDescriptionV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("image_description", o.core().interface, ObjectInterface::WpImageDescriptionV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_set_frame_image_description(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_frame_image_description(&self, arg0, arg1);
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
            1 => "get_capture_source_colors",
            2 => "set_frame_image_description",
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

impl Object for XxImageCaptureColorManagerV1 {
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

impl XxImageCaptureColorManagerV1 {
    /// Since when the error.image_description_not_ready enum variant is available.
    pub const ENM__ERROR_IMAGE_DESCRIPTION_NOT_READY__SINCE: u32 = 1;
    /// Since when the error.already_captured enum variant is available.
    pub const ENM__ERROR_ALREADY_CAPTURED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XxImageCaptureColorManagerV1Error(pub u32);

impl XxImageCaptureColorManagerV1Error {
    /// the image description is not ready
    pub const IMAGE_DESCRIPTION_NOT_READY: Self = Self(1);

    /// capture request has been sent
    pub const ALREADY_CAPTURED: Self = Self(2);
}

impl Debug for XxImageCaptureColorManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::IMAGE_DESCRIPTION_NOT_READY => "IMAGE_DESCRIPTION_NOT_READY",
            Self::ALREADY_CAPTURED => "ALREADY_CAPTURED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
