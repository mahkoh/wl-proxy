//! color management extension to a surface
//!
//! A wp_color_management_surface_feedback_v1 allows the client to get the
//! preferred image description of a surface.
//!
//! If the wl_surface associated with this object is destroyed, the
//! wp_color_management_surface_feedback_v1 object becomes inert.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_color_management_surface_feedback_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpColorManagementSurfaceFeedbackV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpColorManagementSurfaceFeedbackV1Handler>,
}

struct DefaultHandler;

impl WpColorManagementSurfaceFeedbackV1Handler for DefaultHandler { }

impl ConcreteObject for WpColorManagementSurfaceFeedbackV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WpColorManagementSurfaceFeedbackV1;
    const INTERFACE_NAME: &str = "wp_color_management_surface_feedback_v1";
}

impl WpColorManagementSurfaceFeedbackV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpColorManagementSurfaceFeedbackV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpColorManagementSurfaceFeedbackV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpColorManagementSurfaceFeedbackV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpColorManagementSurfaceFeedbackV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpColorManagementSurfaceFeedbackV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the color management interface for a surface
    ///
    /// Destroy the wp_color_management_surface_feedback_v1 object.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_surface_feedback_v1#{}.destroy()\n", id);
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

    /// destroy the color management interface for a surface
    ///
    /// Destroy the wp_color_management_surface_feedback_v1 object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_color_management_surface_feedback_v1.destroy", &e);
        }
    }

    /// Since when the preferred_changed message is available.
    pub const MSG__PREFERRED_CHANGED__SINCE: u32 = 1;

    /// Since when the preferred_changed message is deprecated.
    pub const MSG__PREFERRED_CHANGED__DEPRECATED_SINCE: u32 = 2;

    /// the preferred image description changed (32-bit)
    ///
    /// Starting from interface version 2, 'preferred_changed2' is sent instead
    /// of this event. See the 'preferred_changed2' event for the definition.
    ///
    /// # Arguments
    ///
    /// - `identity`: the 32-bit image description id number
    #[inline]
    pub fn try_send_preferred_changed(
        &self,
        identity: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            identity,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_management_surface_feedback_v1#{}.preferred_changed(identity: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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

    /// the preferred image description changed (32-bit)
    ///
    /// Starting from interface version 2, 'preferred_changed2' is sent instead
    /// of this event. See the 'preferred_changed2' event for the definition.
    ///
    /// # Arguments
    ///
    /// - `identity`: the 32-bit image description id number
    #[inline]
    pub fn send_preferred_changed(
        &self,
        identity: u32,
    ) {
        let res = self.try_send_preferred_changed(
            identity,
        );
        if let Err(e) = res {
            log_send("wp_color_management_surface_feedback_v1.preferred_changed", &e);
        }
    }

    /// Since when the get_preferred message is available.
    pub const MSG__GET_PREFERRED__SINCE: u32 = 1;

    /// get the preferred image description
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// The preferred image description represents the compositor's preferred
    /// color encoding for this wl_surface at the current time. There might be
    /// performance and power advantages, as well as improved color
    /// reproduction, if the image description of a content update matches the
    /// preferred image description.
    ///
    /// This creates a new wp_image_description_v1 object for the currently
    /// preferred image description for the wl_surface. The client should
    /// stop using and destroy the image descriptions created by earlier
    /// invocations of this request for the associated wl_surface.
    /// This request is usually sent as a reaction to the preferred_changed
    /// event or when creating a wp_color_management_surface_feedback_v1 object
    /// if the client is capable of adapting to image descriptions.
    ///
    /// The created wp_image_description_v1 object preserves the preferred image
    /// description of the wl_surface from the time the object was created.
    ///
    /// The resulting image description object allows get_information request.
    ///
    /// If the image description is parametric, the client should set it on its
    /// wl_surface only if the image description is an exact match with the
    /// client content. Particularly if everything else matches, but the target
    /// color volume is greater than what the client needs, the client should
    /// create its own parameric image description with its exact parameters.
    ///
    /// If the interface version is inadequate for the preferred image
    /// description, meaning that the client does not support all the
    /// events needed to deliver the crucial information, the resulting image
    /// description object shall immediately deliver the
    /// wp_image_description_v1.failed event with the low_version cause,
    /// otherwise the object shall immediately deliver the ready event.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    pub fn try_send_get_preferred(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_surface_feedback_v1#{}.get_preferred(image_description: wp_image_description_v1#{})\n", id, arg0);
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

    /// get the preferred image description
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// The preferred image description represents the compositor's preferred
    /// color encoding for this wl_surface at the current time. There might be
    /// performance and power advantages, as well as improved color
    /// reproduction, if the image description of a content update matches the
    /// preferred image description.
    ///
    /// This creates a new wp_image_description_v1 object for the currently
    /// preferred image description for the wl_surface. The client should
    /// stop using and destroy the image descriptions created by earlier
    /// invocations of this request for the associated wl_surface.
    /// This request is usually sent as a reaction to the preferred_changed
    /// event or when creating a wp_color_management_surface_feedback_v1 object
    /// if the client is capable of adapting to image descriptions.
    ///
    /// The created wp_image_description_v1 object preserves the preferred image
    /// description of the wl_surface from the time the object was created.
    ///
    /// The resulting image description object allows get_information request.
    ///
    /// If the image description is parametric, the client should set it on its
    /// wl_surface only if the image description is an exact match with the
    /// client content. Particularly if everything else matches, but the target
    /// color volume is greater than what the client needs, the client should
    /// create its own parameric image description with its exact parameters.
    ///
    /// If the interface version is inadequate for the preferred image
    /// description, meaning that the client does not support all the
    /// events needed to deliver the crucial information, the resulting image
    /// description object shall immediately deliver the
    /// wp_image_description_v1.failed event with the low_version cause,
    /// otherwise the object shall immediately deliver the ready event.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    pub fn send_get_preferred(
        &self,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        let res = self.try_send_get_preferred(
            image_description,
        );
        if let Err(e) = res {
            log_send("wp_color_management_surface_feedback_v1.get_preferred", &e);
        }
    }

    /// get the preferred image description
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// The preferred image description represents the compositor's preferred
    /// color encoding for this wl_surface at the current time. There might be
    /// performance and power advantages, as well as improved color
    /// reproduction, if the image description of a content update matches the
    /// preferred image description.
    ///
    /// This creates a new wp_image_description_v1 object for the currently
    /// preferred image description for the wl_surface. The client should
    /// stop using and destroy the image descriptions created by earlier
    /// invocations of this request for the associated wl_surface.
    /// This request is usually sent as a reaction to the preferred_changed
    /// event or when creating a wp_color_management_surface_feedback_v1 object
    /// if the client is capable of adapting to image descriptions.
    ///
    /// The created wp_image_description_v1 object preserves the preferred image
    /// description of the wl_surface from the time the object was created.
    ///
    /// The resulting image description object allows get_information request.
    ///
    /// If the image description is parametric, the client should set it on its
    /// wl_surface only if the image description is an exact match with the
    /// client content. Particularly if everything else matches, but the target
    /// color volume is greater than what the client needs, the client should
    /// create its own parameric image description with its exact parameters.
    ///
    /// If the interface version is inadequate for the preferred image
    /// description, meaning that the client does not support all the
    /// events needed to deliver the crucial information, the resulting image
    /// description object shall immediately deliver the
    /// wp_image_description_v1.failed event with the low_version cause,
    /// otherwise the object shall immediately deliver the ready event.
    #[inline]
    pub fn new_try_send_get_preferred(
        &self,
    ) -> Result<Rc<WpImageDescriptionV1>, ObjectError> {
        let image_description = self.core.create_child();
        self.try_send_get_preferred(
            &image_description,
        )?;
        Ok(image_description)
    }

    /// get the preferred image description
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// The preferred image description represents the compositor's preferred
    /// color encoding for this wl_surface at the current time. There might be
    /// performance and power advantages, as well as improved color
    /// reproduction, if the image description of a content update matches the
    /// preferred image description.
    ///
    /// This creates a new wp_image_description_v1 object for the currently
    /// preferred image description for the wl_surface. The client should
    /// stop using and destroy the image descriptions created by earlier
    /// invocations of this request for the associated wl_surface.
    /// This request is usually sent as a reaction to the preferred_changed
    /// event or when creating a wp_color_management_surface_feedback_v1 object
    /// if the client is capable of adapting to image descriptions.
    ///
    /// The created wp_image_description_v1 object preserves the preferred image
    /// description of the wl_surface from the time the object was created.
    ///
    /// The resulting image description object allows get_information request.
    ///
    /// If the image description is parametric, the client should set it on its
    /// wl_surface only if the image description is an exact match with the
    /// client content. Particularly if everything else matches, but the target
    /// color volume is greater than what the client needs, the client should
    /// create its own parameric image description with its exact parameters.
    ///
    /// If the interface version is inadequate for the preferred image
    /// description, meaning that the client does not support all the
    /// events needed to deliver the crucial information, the resulting image
    /// description object shall immediately deliver the
    /// wp_image_description_v1.failed event with the low_version cause,
    /// otherwise the object shall immediately deliver the ready event.
    #[inline]
    pub fn new_send_get_preferred(
        &self,
    ) -> Rc<WpImageDescriptionV1> {
        let image_description = self.core.create_child();
        self.send_get_preferred(
            &image_description,
        );
        image_description
    }

    /// Since when the get_preferred_parametric message is available.
    pub const MSG__GET_PREFERRED_PARAMETRIC__SINCE: u32 = 1;

    /// get the preferred image description
    ///
    /// The same description as for get_preferred applies, except the returned
    /// image description is guaranteed to be parametric. This is meant for
    /// clients that can only deal with parametric image descriptions.
    ///
    /// If the compositor doesn't support parametric image descriptions, the
    /// unsupported_feature error is emitted.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    pub fn try_send_get_preferred_parametric(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_surface_feedback_v1#{}.get_preferred_parametric(image_description: wp_image_description_v1#{})\n", id, arg0);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }

    /// get the preferred image description
    ///
    /// The same description as for get_preferred applies, except the returned
    /// image description is guaranteed to be parametric. This is meant for
    /// clients that can only deal with parametric image descriptions.
    ///
    /// If the compositor doesn't support parametric image descriptions, the
    /// unsupported_feature error is emitted.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    pub fn send_get_preferred_parametric(
        &self,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        let res = self.try_send_get_preferred_parametric(
            image_description,
        );
        if let Err(e) = res {
            log_send("wp_color_management_surface_feedback_v1.get_preferred_parametric", &e);
        }
    }

    /// get the preferred image description
    ///
    /// The same description as for get_preferred applies, except the returned
    /// image description is guaranteed to be parametric. This is meant for
    /// clients that can only deal with parametric image descriptions.
    ///
    /// If the compositor doesn't support parametric image descriptions, the
    /// unsupported_feature error is emitted.
    #[inline]
    pub fn new_try_send_get_preferred_parametric(
        &self,
    ) -> Result<Rc<WpImageDescriptionV1>, ObjectError> {
        let image_description = self.core.create_child();
        self.try_send_get_preferred_parametric(
            &image_description,
        )?;
        Ok(image_description)
    }

    /// get the preferred image description
    ///
    /// The same description as for get_preferred applies, except the returned
    /// image description is guaranteed to be parametric. This is meant for
    /// clients that can only deal with parametric image descriptions.
    ///
    /// If the compositor doesn't support parametric image descriptions, the
    /// unsupported_feature error is emitted.
    #[inline]
    pub fn new_send_get_preferred_parametric(
        &self,
    ) -> Rc<WpImageDescriptionV1> {
        let image_description = self.core.create_child();
        self.send_get_preferred_parametric(
            &image_description,
        );
        image_description
    }

    /// Since when the preferred_changed2 message is available.
    pub const MSG__PREFERRED_CHANGED2__SINCE: u32 = 2;

    /// the preferred image description changed
    ///
    /// The preferred image description is the one which likely has the most
    /// performance and/or quality benefits for the compositor if used by the
    /// client for its wl_surface contents. This event is sent whenever the
    /// compositor changes the wl_surface's preferred image description.
    ///
    /// This event sends the identity of the new preferred state as the argument,
    /// so clients who are aware of the image description already can reuse it.
    /// Otherwise, if the client client wants to know what the preferred image
    /// description is, it shall use the get_preferred request.
    ///
    /// The preferred image description is not automatically used for anything.
    /// It is only a hint, and clients may set any valid image description with
    /// set_image_description, but there might be performance and color accuracy
    /// improvements by providing the wl_surface contents in the preferred
    /// image description. Therefore clients that can, should render according
    /// to the preferred image description
    ///
    /// # Arguments
    ///
    /// - `identity_hi`: high 32 bits of the 64-bit image description id number
    /// - `identity_lo`: low 32 bits of the 64-bit image description id number
    #[inline]
    pub fn try_send_preferred_changed2(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_management_surface_feedback_v1#{}.preferred_changed2(identity_hi: {}, identity_lo: {})\n", client_id, id, arg0, arg1);
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
            1,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// the preferred image description changed
    ///
    /// The preferred image description is the one which likely has the most
    /// performance and/or quality benefits for the compositor if used by the
    /// client for its wl_surface contents. This event is sent whenever the
    /// compositor changes the wl_surface's preferred image description.
    ///
    /// This event sends the identity of the new preferred state as the argument,
    /// so clients who are aware of the image description already can reuse it.
    /// Otherwise, if the client client wants to know what the preferred image
    /// description is, it shall use the get_preferred request.
    ///
    /// The preferred image description is not automatically used for anything.
    /// It is only a hint, and clients may set any valid image description with
    /// set_image_description, but there might be performance and color accuracy
    /// improvements by providing the wl_surface contents in the preferred
    /// image description. Therefore clients that can, should render according
    /// to the preferred image description
    ///
    /// # Arguments
    ///
    /// - `identity_hi`: high 32 bits of the 64-bit image description id number
    /// - `identity_lo`: low 32 bits of the 64-bit image description id number
    #[inline]
    pub fn send_preferred_changed2(
        &self,
        identity_hi: u32,
        identity_lo: u32,
    ) {
        let res = self.try_send_preferred_changed2(
            identity_hi,
            identity_lo,
        );
        if let Err(e) = res {
            log_send("wp_color_management_surface_feedback_v1.preferred_changed2", &e);
        }
    }
}

/// A message handler for [`WpColorManagementSurfaceFeedbackV1`] proxies.
pub trait WpColorManagementSurfaceFeedbackV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpColorManagementSurfaceFeedbackV1>) {
        slf.core.delete_id();
    }

    /// destroy the color management interface for a surface
    ///
    /// Destroy the wp_color_management_surface_feedback_v1 object.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpColorManagementSurfaceFeedbackV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_color_management_surface_feedback_v1.destroy", &e);
        }
    }

    /// the preferred image description changed (32-bit)
    ///
    /// Starting from interface version 2, 'preferred_changed2' is sent instead
    /// of this event. See the 'preferred_changed2' event for the definition.
    ///
    /// # Arguments
    ///
    /// - `identity`: the 32-bit image description id number
    #[inline]
    fn handle_preferred_changed(
        &mut self,
        slf: &Rc<WpColorManagementSurfaceFeedbackV1>,
        identity: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preferred_changed(
            identity,
        );
        if let Err(e) = res {
            log_forward("wp_color_management_surface_feedback_v1.preferred_changed", &e);
        }
    }

    /// get the preferred image description
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// The preferred image description represents the compositor's preferred
    /// color encoding for this wl_surface at the current time. There might be
    /// performance and power advantages, as well as improved color
    /// reproduction, if the image description of a content update matches the
    /// preferred image description.
    ///
    /// This creates a new wp_image_description_v1 object for the currently
    /// preferred image description for the wl_surface. The client should
    /// stop using and destroy the image descriptions created by earlier
    /// invocations of this request for the associated wl_surface.
    /// This request is usually sent as a reaction to the preferred_changed
    /// event or when creating a wp_color_management_surface_feedback_v1 object
    /// if the client is capable of adapting to image descriptions.
    ///
    /// The created wp_image_description_v1 object preserves the preferred image
    /// description of the wl_surface from the time the object was created.
    ///
    /// The resulting image description object allows get_information request.
    ///
    /// If the image description is parametric, the client should set it on its
    /// wl_surface only if the image description is an exact match with the
    /// client content. Particularly if everything else matches, but the target
    /// color volume is greater than what the client needs, the client should
    /// create its own parameric image description with its exact parameters.
    ///
    /// If the interface version is inadequate for the preferred image
    /// description, meaning that the client does not support all the
    /// events needed to deliver the crucial information, the resulting image
    /// description object shall immediately deliver the
    /// wp_image_description_v1.failed event with the low_version cause,
    /// otherwise the object shall immediately deliver the ready event.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    fn handle_get_preferred(
        &mut self,
        slf: &Rc<WpColorManagementSurfaceFeedbackV1>,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_preferred(
            image_description,
        );
        if let Err(e) = res {
            log_forward("wp_color_management_surface_feedback_v1.get_preferred", &e);
        }
    }

    /// get the preferred image description
    ///
    /// The same description as for get_preferred applies, except the returned
    /// image description is guaranteed to be parametric. This is meant for
    /// clients that can only deal with parametric image descriptions.
    ///
    /// If the compositor doesn't support parametric image descriptions, the
    /// unsupported_feature error is emitted.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    fn handle_get_preferred_parametric(
        &mut self,
        slf: &Rc<WpColorManagementSurfaceFeedbackV1>,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_preferred_parametric(
            image_description,
        );
        if let Err(e) = res {
            log_forward("wp_color_management_surface_feedback_v1.get_preferred_parametric", &e);
        }
    }

    /// the preferred image description changed
    ///
    /// The preferred image description is the one which likely has the most
    /// performance and/or quality benefits for the compositor if used by the
    /// client for its wl_surface contents. This event is sent whenever the
    /// compositor changes the wl_surface's preferred image description.
    ///
    /// This event sends the identity of the new preferred state as the argument,
    /// so clients who are aware of the image description already can reuse it.
    /// Otherwise, if the client client wants to know what the preferred image
    /// description is, it shall use the get_preferred request.
    ///
    /// The preferred image description is not automatically used for anything.
    /// It is only a hint, and clients may set any valid image description with
    /// set_image_description, but there might be performance and color accuracy
    /// improvements by providing the wl_surface contents in the preferred
    /// image description. Therefore clients that can, should render according
    /// to the preferred image description
    ///
    /// # Arguments
    ///
    /// - `identity_hi`: high 32 bits of the 64-bit image description id number
    /// - `identity_lo`: low 32 bits of the 64-bit image description id number
    #[inline]
    fn handle_preferred_changed2(
        &mut self,
        slf: &Rc<WpColorManagementSurfaceFeedbackV1>,
        identity_hi: u32,
        identity_lo: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preferred_changed2(
            identity_hi,
            identity_lo,
        );
        if let Err(e) = res {
            log_forward("wp_color_management_surface_feedback_v1.preferred_changed2", &e);
        }
    }
}

impl ObjectPrivate for WpColorManagementSurfaceFeedbackV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpColorManagementSurfaceFeedbackV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_surface_feedback_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_surface_feedback_v1#{}.get_preferred(image_description: wp_image_description_v1#{})\n", client_id, id, arg0);
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
                    (**handler).handle_get_preferred(&self, arg0);
                } else {
                    DefaultHandler.handle_get_preferred(&self, arg0);
                }
            }
            2 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_surface_feedback_v1#{}.get_preferred_parametric(image_description: wp_image_description_v1#{})\n", client_id, id, arg0);
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
                    (**handler).handle_get_preferred_parametric(&self, arg0);
                } else {
                    DefaultHandler.handle_get_preferred_parametric(&self, arg0);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_management_surface_feedback_v1#{}.preferred_changed(identity: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preferred_changed(&self, arg0);
                } else {
                    DefaultHandler.handle_preferred_changed(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_management_surface_feedback_v1#{}.preferred_changed2(identity_hi: {}, identity_lo: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preferred_changed2(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_preferred_changed2(&self, arg0, arg1);
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
            2 => "get_preferred_parametric",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "preferred_changed",
            1 => "preferred_changed2",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WpColorManagementSurfaceFeedbackV1 {
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

impl WpColorManagementSurfaceFeedbackV1 {
    /// Since when the error.inert enum variant is available.
    pub const ENM__ERROR_INERT__SINCE: u32 = 1;
    /// Since when the error.unsupported_feature enum variant is available.
    pub const ENM__ERROR_UNSUPPORTED_FEATURE__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorManagementSurfaceFeedbackV1Error(pub u32);

impl WpColorManagementSurfaceFeedbackV1Error {
    /// forbidden request on inert object
    pub const INERT: Self = Self(0);

    /// attempted to use an unsupported feature
    pub const UNSUPPORTED_FEATURE: Self = Self(1);
}

impl Debug for WpColorManagementSurfaceFeedbackV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INERT => "INERT",
            Self::UNSUPPORTED_FEATURE => "UNSUPPORTED_FEATURE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
