//! output color properties
//!
//! A wp_color_management_output_v1 describes the color properties of an
//! output.
//!
//! The wp_color_management_output_v1 is associated with the wl_output global
//! underlying the wl_output object. Therefore the client destroying the
//! wl_output object has no impact, but the compositor removing the output
//! global makes the wp_color_management_output_v1 object inert.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_color_management_output_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpColorManagementOutputV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpColorManagementOutputV1Handler>,
}

struct DefaultHandler;

impl WpColorManagementOutputV1Handler for DefaultHandler { }

impl ConcreteObject for WpColorManagementOutputV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WpColorManagementOutputV1;
    const INTERFACE_NAME: &str = "wp_color_management_output_v1";
}

impl WpColorManagementOutputV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpColorManagementOutputV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpColorManagementOutputV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpColorManagementOutputV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpColorManagementOutputV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpColorManagementOutputV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the color management output
    ///
    /// Destroy the color wp_color_management_output_v1 object. This does not
    /// affect any remaining protocol objects.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_output_v1#{}.destroy()\n", id);
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

    /// destroy the color management output
    ///
    /// Destroy the color wp_color_management_output_v1 object. This does not
    /// affect any remaining protocol objects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_color_management_output_v1.destroy", &e);
        }
    }

    /// Since when the image_description_changed message is available.
    pub const MSG__IMAGE_DESCRIPTION_CHANGED__SINCE: u32 = 1;

    /// image description changed
    ///
    /// This event is sent whenever the image description of the output changed,
    /// followed by one wl_output.done event common to output events across all
    /// extensions.
    ///
    /// If the client wants to use the updated image description, it needs to do
    /// get_image_description again, because image description objects are
    /// immutable.
    #[inline]
    pub fn try_send_image_description_changed(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_management_output_v1#{}.image_description_changed()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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

    /// image description changed
    ///
    /// This event is sent whenever the image description of the output changed,
    /// followed by one wl_output.done event common to output events across all
    /// extensions.
    ///
    /// If the client wants to use the updated image description, it needs to do
    /// get_image_description again, because image description objects are
    /// immutable.
    #[inline]
    pub fn send_image_description_changed(
        &self,
    ) {
        let res = self.try_send_image_description_changed(
        );
        if let Err(e) = res {
            log_send("wp_color_management_output_v1.image_description_changed", &e);
        }
    }

    /// Since when the get_image_description message is available.
    pub const MSG__GET_IMAGE_DESCRIPTION__SINCE: u32 = 1;

    /// get the image description of the output
    ///
    /// This creates a new wp_image_description_v1 object for the current image
    /// description of the output. There always is exactly one image description
    /// active for an output so the client should destroy the image description
    /// created by earlier invocations of this request. This request is usually
    /// sent as a reaction to the image_description_changed event or when
    /// creating a wp_color_management_output_v1 object.
    ///
    /// The image description of an output represents the color encoding the
    /// output expects. There might be performance and power advantages, as well
    /// as improved color reproduction, if a content update matches the image
    /// description of the output it is being shown on. If a content update is
    /// shown on any other output than the one it matches the image description
    /// of, then the color reproduction on those outputs might be considerably
    /// worse.
    ///
    /// The created wp_image_description_v1 object preserves the image
    /// description of the output from the time the object was created.
    ///
    /// The resulting image description object allows get_information request.
    ///
    /// If this protocol object is inert, the resulting image description object
    /// shall immediately deliver the wp_image_description_v1.failed event with
    /// the no_output cause.
    ///
    /// If the interface version is inadequate for the output's image
    /// description, meaning that the client does not support all the events
    /// needed to deliver the crucial information, the resulting image
    /// description object shall immediately deliver the
    /// wp_image_description_v1.failed event with the low_version cause.
    ///
    /// Otherwise the object shall immediately deliver the ready event.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    pub fn try_send_get_image_description(
        &self,
        image_description: &Rc<WpImageDescriptionV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            image_description,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("image_description", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_output_v1#{}.get_image_description(image_description: wp_image_description_v1#{})\n", id, arg0);
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

    /// get the image description of the output
    ///
    /// This creates a new wp_image_description_v1 object for the current image
    /// description of the output. There always is exactly one image description
    /// active for an output so the client should destroy the image description
    /// created by earlier invocations of this request. This request is usually
    /// sent as a reaction to the image_description_changed event or when
    /// creating a wp_color_management_output_v1 object.
    ///
    /// The image description of an output represents the color encoding the
    /// output expects. There might be performance and power advantages, as well
    /// as improved color reproduction, if a content update matches the image
    /// description of the output it is being shown on. If a content update is
    /// shown on any other output than the one it matches the image description
    /// of, then the color reproduction on those outputs might be considerably
    /// worse.
    ///
    /// The created wp_image_description_v1 object preserves the image
    /// description of the output from the time the object was created.
    ///
    /// The resulting image description object allows get_information request.
    ///
    /// If this protocol object is inert, the resulting image description object
    /// shall immediately deliver the wp_image_description_v1.failed event with
    /// the no_output cause.
    ///
    /// If the interface version is inadequate for the output's image
    /// description, meaning that the client does not support all the events
    /// needed to deliver the crucial information, the resulting image
    /// description object shall immediately deliver the
    /// wp_image_description_v1.failed event with the low_version cause.
    ///
    /// Otherwise the object shall immediately deliver the ready event.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    pub fn send_get_image_description(
        &self,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        let res = self.try_send_get_image_description(
            image_description,
        );
        if let Err(e) = res {
            log_send("wp_color_management_output_v1.get_image_description", &e);
        }
    }

    /// get the image description of the output
    ///
    /// This creates a new wp_image_description_v1 object for the current image
    /// description of the output. There always is exactly one image description
    /// active for an output so the client should destroy the image description
    /// created by earlier invocations of this request. This request is usually
    /// sent as a reaction to the image_description_changed event or when
    /// creating a wp_color_management_output_v1 object.
    ///
    /// The image description of an output represents the color encoding the
    /// output expects. There might be performance and power advantages, as well
    /// as improved color reproduction, if a content update matches the image
    /// description of the output it is being shown on. If a content update is
    /// shown on any other output than the one it matches the image description
    /// of, then the color reproduction on those outputs might be considerably
    /// worse.
    ///
    /// The created wp_image_description_v1 object preserves the image
    /// description of the output from the time the object was created.
    ///
    /// The resulting image description object allows get_information request.
    ///
    /// If this protocol object is inert, the resulting image description object
    /// shall immediately deliver the wp_image_description_v1.failed event with
    /// the no_output cause.
    ///
    /// If the interface version is inadequate for the output's image
    /// description, meaning that the client does not support all the events
    /// needed to deliver the crucial information, the resulting image
    /// description object shall immediately deliver the
    /// wp_image_description_v1.failed event with the low_version cause.
    ///
    /// Otherwise the object shall immediately deliver the ready event.
    #[inline]
    pub fn new_try_send_get_image_description(
        &self,
    ) -> Result<Rc<WpImageDescriptionV1>, ObjectError> {
        let image_description = self.core.create_child();
        self.try_send_get_image_description(
            &image_description,
        )?;
        Ok(image_description)
    }

    /// get the image description of the output
    ///
    /// This creates a new wp_image_description_v1 object for the current image
    /// description of the output. There always is exactly one image description
    /// active for an output so the client should destroy the image description
    /// created by earlier invocations of this request. This request is usually
    /// sent as a reaction to the image_description_changed event or when
    /// creating a wp_color_management_output_v1 object.
    ///
    /// The image description of an output represents the color encoding the
    /// output expects. There might be performance and power advantages, as well
    /// as improved color reproduction, if a content update matches the image
    /// description of the output it is being shown on. If a content update is
    /// shown on any other output than the one it matches the image description
    /// of, then the color reproduction on those outputs might be considerably
    /// worse.
    ///
    /// The created wp_image_description_v1 object preserves the image
    /// description of the output from the time the object was created.
    ///
    /// The resulting image description object allows get_information request.
    ///
    /// If this protocol object is inert, the resulting image description object
    /// shall immediately deliver the wp_image_description_v1.failed event with
    /// the no_output cause.
    ///
    /// If the interface version is inadequate for the output's image
    /// description, meaning that the client does not support all the events
    /// needed to deliver the crucial information, the resulting image
    /// description object shall immediately deliver the
    /// wp_image_description_v1.failed event with the low_version cause.
    ///
    /// Otherwise the object shall immediately deliver the ready event.
    #[inline]
    pub fn new_send_get_image_description(
        &self,
    ) -> Rc<WpImageDescriptionV1> {
        let image_description = self.core.create_child();
        self.send_get_image_description(
            &image_description,
        );
        image_description
    }
}

/// A message handler for [`WpColorManagementOutputV1`] proxies.
pub trait WpColorManagementOutputV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpColorManagementOutputV1>) {
        slf.core.delete_id();
    }

    /// destroy the color management output
    ///
    /// Destroy the color wp_color_management_output_v1 object. This does not
    /// affect any remaining protocol objects.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpColorManagementOutputV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_color_management_output_v1.destroy", &e);
        }
    }

    /// image description changed
    ///
    /// This event is sent whenever the image description of the output changed,
    /// followed by one wl_output.done event common to output events across all
    /// extensions.
    ///
    /// If the client wants to use the updated image description, it needs to do
    /// get_image_description again, because image description objects are
    /// immutable.
    #[inline]
    fn handle_image_description_changed(
        &mut self,
        slf: &Rc<WpColorManagementOutputV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_image_description_changed(
        );
        if let Err(e) = res {
            log_forward("wp_color_management_output_v1.image_description_changed", &e);
        }
    }

    /// get the image description of the output
    ///
    /// This creates a new wp_image_description_v1 object for the current image
    /// description of the output. There always is exactly one image description
    /// active for an output so the client should destroy the image description
    /// created by earlier invocations of this request. This request is usually
    /// sent as a reaction to the image_description_changed event or when
    /// creating a wp_color_management_output_v1 object.
    ///
    /// The image description of an output represents the color encoding the
    /// output expects. There might be performance and power advantages, as well
    /// as improved color reproduction, if a content update matches the image
    /// description of the output it is being shown on. If a content update is
    /// shown on any other output than the one it matches the image description
    /// of, then the color reproduction on those outputs might be considerably
    /// worse.
    ///
    /// The created wp_image_description_v1 object preserves the image
    /// description of the output from the time the object was created.
    ///
    /// The resulting image description object allows get_information request.
    ///
    /// If this protocol object is inert, the resulting image description object
    /// shall immediately deliver the wp_image_description_v1.failed event with
    /// the no_output cause.
    ///
    /// If the interface version is inadequate for the output's image
    /// description, meaning that the client does not support all the events
    /// needed to deliver the crucial information, the resulting image
    /// description object shall immediately deliver the
    /// wp_image_description_v1.failed event with the low_version cause.
    ///
    /// Otherwise the object shall immediately deliver the ready event.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    fn handle_get_image_description(
        &mut self,
        slf: &Rc<WpColorManagementOutputV1>,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_image_description(
            image_description,
        );
        if let Err(e) = res {
            log_forward("wp_color_management_output_v1.get_image_description", &e);
        }
    }
}

impl ObjectPrivate for WpColorManagementOutputV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpColorManagementOutputV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_output_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_output_v1#{}.get_image_description(image_description: wp_image_description_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "image_description", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_get_image_description(&self, arg0);
                } else {
                    DefaultHandler.handle_get_image_description(&self, arg0);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_management_output_v1#{}.image_description_changed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_image_description_changed(&self);
                } else {
                    DefaultHandler.handle_image_description_changed(&self);
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
            1 => "get_image_description",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "image_description_changed",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WpColorManagementOutputV1 {
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

