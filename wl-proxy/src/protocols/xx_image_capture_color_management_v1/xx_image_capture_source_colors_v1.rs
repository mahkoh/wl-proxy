//! capture source color properties
//!
//! This object describes the color properties of a capture source.
//!
//! The xx_image_capture_source_colors_v1 is associated with the compositor
//! object backing the source. Therefore the client destroying the
//! ext_image_capture_source_v1 object has no impact. The object becomes inert
//! when the compositor object backing the source is destroyed. How the client
//! can detect this depends on the source.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xx_image_capture_source_colors_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XxImageCaptureSourceColorsV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XxImageCaptureSourceColorsV1Handler>,
}

struct DefaultHandler;

impl XxImageCaptureSourceColorsV1Handler for DefaultHandler { }

impl ConcreteObject for XxImageCaptureSourceColorsV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XxImageCaptureSourceColorsV1;
    const INTERFACE_NAME: &str = "xx_image_capture_source_colors_v1";
}

impl XxImageCaptureSourceColorsV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XxImageCaptureSourceColorsV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XxImageCaptureSourceColorsV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XxImageCaptureSourceColorsV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XxImageCaptureSourceColorsV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XxImageCaptureSourceColorsV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroys this object
    ///
    /// Destroys this object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_image_capture_source_colors_v1#{}.destroy()\n", id);
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

    /// destroys this object
    ///
    /// Destroys this object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xx_image_capture_source_colors_v1.destroy", &e);
        }
    }

    /// Since when the get_preferred message is available.
    pub const MSG__GET_PREFERRED__SINCE: u32 = 1;

    /// retrieve the preferred description
    ///
    /// The preferred image description represents the compositor's preferred
    /// color encoding for this capture source at the current time. There might
    /// be performance and power advantages, as well as improved color
    /// reproduction, if the image description of a capture frame matches the
    /// preferred image description.
    ///
    /// This creates a new wp_image_description_reference_v1 object for the
    /// currently preferred image description for the capture source. The client
    /// should stop using and destroy the image descriptions created by earlier
    /// invocations of this request for the associated capture source. This
    /// request is usually sent as a reaction to the
    /// xx_image_capture_source_colors_v1.preferred_changed event or when
    /// creating a xx_image_capture_source_colors_v1 object if the client is
    /// capable of adapting to image descriptions.
    ///
    /// The created wp_image_description_reference_v1 object preserves the
    /// preferred image description of the capture source from the time the
    /// object was created.
    ///
    /// The resulting image description object allows the
    /// wp_image_description_v1.get_information request.
    ///
    /// # Arguments
    ///
    /// - `id`: the preferred description
    #[inline]
    pub fn try_send_get_preferred(
        &self,
        id: &Rc<WpImageDescriptionReferenceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xx_image_capture_source_colors_v1#{}.get_preferred(id: wp_image_description_reference_v1#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
        ]);
        Ok(())
    }

    /// retrieve the preferred description
    ///
    /// The preferred image description represents the compositor's preferred
    /// color encoding for this capture source at the current time. There might
    /// be performance and power advantages, as well as improved color
    /// reproduction, if the image description of a capture frame matches the
    /// preferred image description.
    ///
    /// This creates a new wp_image_description_reference_v1 object for the
    /// currently preferred image description for the capture source. The client
    /// should stop using and destroy the image descriptions created by earlier
    /// invocations of this request for the associated capture source. This
    /// request is usually sent as a reaction to the
    /// xx_image_capture_source_colors_v1.preferred_changed event or when
    /// creating a xx_image_capture_source_colors_v1 object if the client is
    /// capable of adapting to image descriptions.
    ///
    /// The created wp_image_description_reference_v1 object preserves the
    /// preferred image description of the capture source from the time the
    /// object was created.
    ///
    /// The resulting image description object allows the
    /// wp_image_description_v1.get_information request.
    ///
    /// # Arguments
    ///
    /// - `id`: the preferred description
    #[inline]
    pub fn send_get_preferred(
        &self,
        id: &Rc<WpImageDescriptionReferenceV1>,
    ) {
        let res = self.try_send_get_preferred(
            id,
        );
        if let Err(e) = res {
            log_send("xx_image_capture_source_colors_v1.get_preferred", &e);
        }
    }

    /// retrieve the preferred description
    ///
    /// The preferred image description represents the compositor's preferred
    /// color encoding for this capture source at the current time. There might
    /// be performance and power advantages, as well as improved color
    /// reproduction, if the image description of a capture frame matches the
    /// preferred image description.
    ///
    /// This creates a new wp_image_description_reference_v1 object for the
    /// currently preferred image description for the capture source. The client
    /// should stop using and destroy the image descriptions created by earlier
    /// invocations of this request for the associated capture source. This
    /// request is usually sent as a reaction to the
    /// xx_image_capture_source_colors_v1.preferred_changed event or when
    /// creating a xx_image_capture_source_colors_v1 object if the client is
    /// capable of adapting to image descriptions.
    ///
    /// The created wp_image_description_reference_v1 object preserves the
    /// preferred image description of the capture source from the time the
    /// object was created.
    ///
    /// The resulting image description object allows the
    /// wp_image_description_v1.get_information request.
    #[inline]
    pub fn new_try_send_get_preferred(
        &self,
    ) -> Result<Rc<WpImageDescriptionReferenceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_preferred(
            &id,
        )?;
        Ok(id)
    }

    /// retrieve the preferred description
    ///
    /// The preferred image description represents the compositor's preferred
    /// color encoding for this capture source at the current time. There might
    /// be performance and power advantages, as well as improved color
    /// reproduction, if the image description of a capture frame matches the
    /// preferred image description.
    ///
    /// This creates a new wp_image_description_reference_v1 object for the
    /// currently preferred image description for the capture source. The client
    /// should stop using and destroy the image descriptions created by earlier
    /// invocations of this request for the associated capture source. This
    /// request is usually sent as a reaction to the
    /// xx_image_capture_source_colors_v1.preferred_changed event or when
    /// creating a xx_image_capture_source_colors_v1 object if the client is
    /// capable of adapting to image descriptions.
    ///
    /// The created wp_image_description_reference_v1 object preserves the
    /// preferred image description of the capture source from the time the
    /// object was created.
    ///
    /// The resulting image description object allows the
    /// wp_image_description_v1.get_information request.
    #[inline]
    pub fn new_send_get_preferred(
        &self,
    ) -> Rc<WpImageDescriptionReferenceV1> {
        let id = self.core.create_child();
        self.send_get_preferred(
            &id,
        );
        id
    }

    /// Since when the preferred_changed message is available.
    pub const MSG__PREFERRED_CHANGED__SINCE: u32 = 1;

    /// the preferred image description changed
    ///
    /// The preferred image description is the one which likely has the most
    /// performance and/or quality benefits for the compositor if used by the
    /// client for its capture frames. This event is sent whenever
    /// the compositor changes the capture source's preferred image description.
    ///
    /// This event sends the identity of the new preferred state as the
    /// argument, so clients who are aware of the image description already can
    /// reuse it. Otherwise, if the client wants to know what the preferred
    /// image description is, it shall use the
    /// xx_image_capture_source_colors_v1.get_preferred request.
    ///
    /// The preferred image description is not automatically used for anything.
    /// It is only a hint, and clients may set any valid image description with
    /// xx_image_capture_color_manager_v1.set_frame_image_description, but there
    /// might be performance and color accuracy improvements by providing the
    /// capture frame's contents in the preferred image description. Therefore
    /// clients that can, should capture frames in the preferred image
    /// description.
    ///
    /// # Arguments
    ///
    /// - `identity_hi`: image description id number (high 32 bits)
    /// - `identity_lo`: image description id number (low 32 bits)
    #[inline]
    pub fn try_send_preferred_changed(
        &self,
        identity_hi: u32,
        identity_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            identity_hi,
            identity_lo,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xx_image_capture_source_colors_v1#{}.preferred_changed(identity_hi: {}, identity_lo: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            arg1,
        ]);
        Ok(())
    }

    /// the preferred image description changed
    ///
    /// The preferred image description is the one which likely has the most
    /// performance and/or quality benefits for the compositor if used by the
    /// client for its capture frames. This event is sent whenever
    /// the compositor changes the capture source's preferred image description.
    ///
    /// This event sends the identity of the new preferred state as the
    /// argument, so clients who are aware of the image description already can
    /// reuse it. Otherwise, if the client wants to know what the preferred
    /// image description is, it shall use the
    /// xx_image_capture_source_colors_v1.get_preferred request.
    ///
    /// The preferred image description is not automatically used for anything.
    /// It is only a hint, and clients may set any valid image description with
    /// xx_image_capture_color_manager_v1.set_frame_image_description, but there
    /// might be performance and color accuracy improvements by providing the
    /// capture frame's contents in the preferred image description. Therefore
    /// clients that can, should capture frames in the preferred image
    /// description.
    ///
    /// # Arguments
    ///
    /// - `identity_hi`: image description id number (high 32 bits)
    /// - `identity_lo`: image description id number (low 32 bits)
    #[inline]
    pub fn send_preferred_changed(
        &self,
        identity_hi: u32,
        identity_lo: u32,
    ) {
        let res = self.try_send_preferred_changed(
            identity_hi,
            identity_lo,
        );
        if let Err(e) = res {
            log_send("xx_image_capture_source_colors_v1.preferred_changed", &e);
        }
    }
}

/// A message handler for [`XxImageCaptureSourceColorsV1`] proxies.
pub trait XxImageCaptureSourceColorsV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XxImageCaptureSourceColorsV1>) {
        slf.core.delete_id();
    }

    /// destroys this object
    ///
    /// Destroys this object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XxImageCaptureSourceColorsV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xx_image_capture_source_colors_v1.destroy", &e);
        }
    }

    /// retrieve the preferred description
    ///
    /// The preferred image description represents the compositor's preferred
    /// color encoding for this capture source at the current time. There might
    /// be performance and power advantages, as well as improved color
    /// reproduction, if the image description of a capture frame matches the
    /// preferred image description.
    ///
    /// This creates a new wp_image_description_reference_v1 object for the
    /// currently preferred image description for the capture source. The client
    /// should stop using and destroy the image descriptions created by earlier
    /// invocations of this request for the associated capture source. This
    /// request is usually sent as a reaction to the
    /// xx_image_capture_source_colors_v1.preferred_changed event or when
    /// creating a xx_image_capture_source_colors_v1 object if the client is
    /// capable of adapting to image descriptions.
    ///
    /// The created wp_image_description_reference_v1 object preserves the
    /// preferred image description of the capture source from the time the
    /// object was created.
    ///
    /// The resulting image description object allows the
    /// wp_image_description_v1.get_information request.
    ///
    /// # Arguments
    ///
    /// - `id`: the preferred description
    #[inline]
    fn handle_get_preferred(
        &mut self,
        slf: &Rc<XxImageCaptureSourceColorsV1>,
        id: &Rc<WpImageDescriptionReferenceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_preferred(
            id,
        );
        if let Err(e) = res {
            log_forward("xx_image_capture_source_colors_v1.get_preferred", &e);
        }
    }

    /// the preferred image description changed
    ///
    /// The preferred image description is the one which likely has the most
    /// performance and/or quality benefits for the compositor if used by the
    /// client for its capture frames. This event is sent whenever
    /// the compositor changes the capture source's preferred image description.
    ///
    /// This event sends the identity of the new preferred state as the
    /// argument, so clients who are aware of the image description already can
    /// reuse it. Otherwise, if the client wants to know what the preferred
    /// image description is, it shall use the
    /// xx_image_capture_source_colors_v1.get_preferred request.
    ///
    /// The preferred image description is not automatically used for anything.
    /// It is only a hint, and clients may set any valid image description with
    /// xx_image_capture_color_manager_v1.set_frame_image_description, but there
    /// might be performance and color accuracy improvements by providing the
    /// capture frame's contents in the preferred image description. Therefore
    /// clients that can, should capture frames in the preferred image
    /// description.
    ///
    /// # Arguments
    ///
    /// - `identity_hi`: image description id number (high 32 bits)
    /// - `identity_lo`: image description id number (low 32 bits)
    #[inline]
    fn handle_preferred_changed(
        &mut self,
        slf: &Rc<XxImageCaptureSourceColorsV1>,
        identity_hi: u32,
        identity_lo: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preferred_changed(
            identity_hi,
            identity_lo,
        );
        if let Err(e) = res {
            log_forward("xx_image_capture_source_colors_v1.preferred_changed", &e);
        }
    }
}

impl ObjectPrivate for XxImageCaptureSourceColorsV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XxImageCaptureSourceColorsV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_image_capture_source_colors_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xx_image_capture_source_colors_v1#{}.get_preferred(id: wp_image_description_reference_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WpImageDescriptionReferenceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_preferred(&self, arg0);
                } else {
                    DefaultHandler.handle_get_preferred(&self, arg0);
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
            0 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xx_image_capture_source_colors_v1#{}.preferred_changed(identity_hi: {}, identity_lo: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preferred_changed(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_preferred_changed(&self, arg0, arg1);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "get_preferred",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "preferred_changed",
            _ => return None,
        };
        Some(name)
    }

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for XxImageCaptureSourceColorsV1 {
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

