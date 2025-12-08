//! color manager singleton
//!
//! A singleton global interface used for getting color management extensions
//! for wl_surface and wl_output objects, and for creating client defined
//! image description objects. The extension interfaces allow
//! getting the image description of outputs and setting the image
//! description of surfaces.
//!
//! Compositors should never remove this global.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_color_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpColorManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpColorManagerV1Handler>,
}

struct DefaultHandler;

impl WpColorManagerV1Handler for DefaultHandler { }

impl ConcreteObject for WpColorManagerV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WpColorManagerV1;
    const INTERFACE_NAME: &str = "wp_color_manager_v1";
}

impl WpColorManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpColorManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpColorManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpColorManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpColorManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpColorManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the color manager
    ///
    /// Destroy the wp_color_manager_v1 object. This does not affect any other
    /// objects in any way.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_manager_v1#{}.destroy()\n", id);
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

    /// destroy the color manager
    ///
    /// Destroy the wp_color_manager_v1 object. This does not affect any other
    /// objects in any way.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.destroy", &e);
        }
    }

    /// Since when the get_output message is available.
    pub const MSG__GET_OUTPUT__SINCE: u32 = 1;

    /// create a color management interface for a wl_output
    ///
    /// This creates a new wp_color_management_output_v1 object for the
    /// given wl_output.
    ///
    /// See the wp_color_management_output_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    #[inline]
    pub fn try_send_get_output(
        &self,
        id: &Rc<WpColorManagementOutputV1>,
        output: &Rc<WlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            output,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_manager_v1#{}.get_output(id: wp_color_management_output_v1#{}, output: wl_output#{})\n", id, arg0, arg1);
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

    /// create a color management interface for a wl_output
    ///
    /// This creates a new wp_color_management_output_v1 object for the
    /// given wl_output.
    ///
    /// See the wp_color_management_output_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    #[inline]
    pub fn send_get_output(
        &self,
        id: &Rc<WpColorManagementOutputV1>,
        output: &Rc<WlOutput>,
    ) {
        let res = self.try_send_get_output(
            id,
            output,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.get_output", &e);
        }
    }

    /// create a color management interface for a wl_output
    ///
    /// This creates a new wp_color_management_output_v1 object for the
    /// given wl_output.
    ///
    /// See the wp_color_management_output_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn new_try_send_get_output(
        &self,
        output: &Rc<WlOutput>,
    ) -> Result<Rc<WpColorManagementOutputV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_output(
            &id,
            output,
        )?;
        Ok(id)
    }

    /// create a color management interface for a wl_output
    ///
    /// This creates a new wp_color_management_output_v1 object for the
    /// given wl_output.
    ///
    /// See the wp_color_management_output_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn new_send_get_output(
        &self,
        output: &Rc<WlOutput>,
    ) -> Rc<WpColorManagementOutputV1> {
        let id = self.core.create_child();
        self.send_get_output(
            &id,
            output,
        );
        id
    }

    /// Since when the get_surface message is available.
    pub const MSG__GET_SURFACE__SINCE: u32 = 1;

    /// create a color management interface for a wl_surface
    ///
    /// If a wp_color_management_surface_v1 object already exists for the given
    /// wl_surface, the protocol error surface_exists is raised.
    ///
    /// This creates a new color wp_color_management_surface_v1 object for the
    /// given wl_surface.
    ///
    /// See the wp_color_management_surface_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn try_send_get_surface(
        &self,
        id: &Rc<WpColorManagementSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_manager_v1#{}.get_surface(id: wp_color_management_surface_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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

    /// create a color management interface for a wl_surface
    ///
    /// If a wp_color_management_surface_v1 object already exists for the given
    /// wl_surface, the protocol error surface_exists is raised.
    ///
    /// This creates a new color wp_color_management_surface_v1 object for the
    /// given wl_surface.
    ///
    /// See the wp_color_management_surface_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_surface(
        &self,
        id: &Rc<WpColorManagementSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.get_surface", &e);
        }
    }

    /// create a color management interface for a wl_surface
    ///
    /// If a wp_color_management_surface_v1 object already exists for the given
    /// wl_surface, the protocol error surface_exists is raised.
    ///
    /// This creates a new color wp_color_management_surface_v1 object for the
    /// given wl_surface.
    ///
    /// See the wp_color_management_surface_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_try_send_get_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<WpColorManagementSurfaceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_surface(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// create a color management interface for a wl_surface
    ///
    /// If a wp_color_management_surface_v1 object already exists for the given
    /// wl_surface, the protocol error surface_exists is raised.
    ///
    /// This creates a new color wp_color_management_surface_v1 object for the
    /// given wl_surface.
    ///
    /// See the wp_color_management_surface_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_send_get_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<WpColorManagementSurfaceV1> {
        let id = self.core.create_child();
        self.send_get_surface(
            &id,
            surface,
        );
        id
    }

    /// Since when the get_surface_feedback message is available.
    pub const MSG__GET_SURFACE_FEEDBACK__SINCE: u32 = 1;

    /// create a color management feedback interface
    ///
    /// This creates a new color wp_color_management_surface_feedback_v1 object
    /// for the given wl_surface.
    ///
    /// See the wp_color_management_surface_feedback_v1 interface for more
    /// details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn try_send_get_surface_feedback(
        &self,
        id: &Rc<WpColorManagementSurfaceFeedbackV1>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_manager_v1#{}.get_surface_feedback(id: wp_color_management_surface_feedback_v1#{}, surface: wl_surface#{})\n", id, arg0, arg1);
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
            3,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// create a color management feedback interface
    ///
    /// This creates a new color wp_color_management_surface_feedback_v1 object
    /// for the given wl_surface.
    ///
    /// See the wp_color_management_surface_feedback_v1 interface for more
    /// details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_surface_feedback(
        &self,
        id: &Rc<WpColorManagementSurfaceFeedbackV1>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_surface_feedback(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.get_surface_feedback", &e);
        }
    }

    /// create a color management feedback interface
    ///
    /// This creates a new color wp_color_management_surface_feedback_v1 object
    /// for the given wl_surface.
    ///
    /// See the wp_color_management_surface_feedback_v1 interface for more
    /// details.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_try_send_get_surface_feedback(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<WpColorManagementSurfaceFeedbackV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_surface_feedback(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// create a color management feedback interface
    ///
    /// This creates a new color wp_color_management_surface_feedback_v1 object
    /// for the given wl_surface.
    ///
    /// See the wp_color_management_surface_feedback_v1 interface for more
    /// details.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_send_get_surface_feedback(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<WpColorManagementSurfaceFeedbackV1> {
        let id = self.core.create_child();
        self.send_get_surface_feedback(
            &id,
            surface,
        );
        id
    }

    /// Since when the create_icc_creator message is available.
    pub const MSG__CREATE_ICC_CREATOR__SINCE: u32 = 1;

    /// make a new ICC-based image description creator object
    ///
    /// Makes a new ICC-based image description creator object with all
    /// properties initially unset. The client can then use the object's
    /// interface to define all the required properties for an image description
    /// and finally create a wp_image_description_v1 object.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.icc_v2_v4.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// # Arguments
    ///
    /// - `obj`: the new creator object
    #[inline]
    pub fn try_send_create_icc_creator(
        &self,
        obj: &Rc<WpImageDescriptionCreatorIccV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            obj,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("obj", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_manager_v1#{}.create_icc_creator(obj: wp_image_description_creator_icc_v1#{})\n", id, arg0);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// make a new ICC-based image description creator object
    ///
    /// Makes a new ICC-based image description creator object with all
    /// properties initially unset. The client can then use the object's
    /// interface to define all the required properties for an image description
    /// and finally create a wp_image_description_v1 object.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.icc_v2_v4.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// # Arguments
    ///
    /// - `obj`: the new creator object
    #[inline]
    pub fn send_create_icc_creator(
        &self,
        obj: &Rc<WpImageDescriptionCreatorIccV1>,
    ) {
        let res = self.try_send_create_icc_creator(
            obj,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.create_icc_creator", &e);
        }
    }

    /// make a new ICC-based image description creator object
    ///
    /// Makes a new ICC-based image description creator object with all
    /// properties initially unset. The client can then use the object's
    /// interface to define all the required properties for an image description
    /// and finally create a wp_image_description_v1 object.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.icc_v2_v4.
    /// Otherwise this request raises the protocol error unsupported_feature.
    #[inline]
    pub fn new_try_send_create_icc_creator(
        &self,
    ) -> Result<Rc<WpImageDescriptionCreatorIccV1>, ObjectError> {
        let obj = self.core.create_child();
        self.try_send_create_icc_creator(
            &obj,
        )?;
        Ok(obj)
    }

    /// make a new ICC-based image description creator object
    ///
    /// Makes a new ICC-based image description creator object with all
    /// properties initially unset. The client can then use the object's
    /// interface to define all the required properties for an image description
    /// and finally create a wp_image_description_v1 object.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.icc_v2_v4.
    /// Otherwise this request raises the protocol error unsupported_feature.
    #[inline]
    pub fn new_send_create_icc_creator(
        &self,
    ) -> Rc<WpImageDescriptionCreatorIccV1> {
        let obj = self.core.create_child();
        self.send_create_icc_creator(
            &obj,
        );
        obj
    }

    /// Since when the create_parametric_creator message is available.
    pub const MSG__CREATE_PARAMETRIC_CREATOR__SINCE: u32 = 1;

    /// make a new parametric image description creator object
    ///
    /// Makes a new parametric image description creator object with all
    /// properties initially unset. The client can then use the object's
    /// interface to define all the required properties for an image description
    /// and finally create a wp_image_description_v1 object.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.parametric.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// # Arguments
    ///
    /// - `obj`: the new creator object
    #[inline]
    pub fn try_send_create_parametric_creator(
        &self,
        obj: &Rc<WpImageDescriptionCreatorParamsV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            obj,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("obj", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_manager_v1#{}.create_parametric_creator(obj: wp_image_description_creator_params_v1#{})\n", id, arg0);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// make a new parametric image description creator object
    ///
    /// Makes a new parametric image description creator object with all
    /// properties initially unset. The client can then use the object's
    /// interface to define all the required properties for an image description
    /// and finally create a wp_image_description_v1 object.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.parametric.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// # Arguments
    ///
    /// - `obj`: the new creator object
    #[inline]
    pub fn send_create_parametric_creator(
        &self,
        obj: &Rc<WpImageDescriptionCreatorParamsV1>,
    ) {
        let res = self.try_send_create_parametric_creator(
            obj,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.create_parametric_creator", &e);
        }
    }

    /// make a new parametric image description creator object
    ///
    /// Makes a new parametric image description creator object with all
    /// properties initially unset. The client can then use the object's
    /// interface to define all the required properties for an image description
    /// and finally create a wp_image_description_v1 object.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.parametric.
    /// Otherwise this request raises the protocol error unsupported_feature.
    #[inline]
    pub fn new_try_send_create_parametric_creator(
        &self,
    ) -> Result<Rc<WpImageDescriptionCreatorParamsV1>, ObjectError> {
        let obj = self.core.create_child();
        self.try_send_create_parametric_creator(
            &obj,
        )?;
        Ok(obj)
    }

    /// make a new parametric image description creator object
    ///
    /// Makes a new parametric image description creator object with all
    /// properties initially unset. The client can then use the object's
    /// interface to define all the required properties for an image description
    /// and finally create a wp_image_description_v1 object.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.parametric.
    /// Otherwise this request raises the protocol error unsupported_feature.
    #[inline]
    pub fn new_send_create_parametric_creator(
        &self,
    ) -> Rc<WpImageDescriptionCreatorParamsV1> {
        let obj = self.core.create_child();
        self.send_create_parametric_creator(
            &obj,
        );
        obj
    }

    /// Since when the create_windows_scrgb message is available.
    pub const MSG__CREATE_WINDOWS_SCRGB__SINCE: u32 = 1;

    /// create Windows-scRGB image description object
    ///
    /// This creates a pre-defined image description for the so-called
    /// Windows-scRGB stimulus encoding. This comes from the Windows 10 handling
    /// of its own definition of an scRGB color space for an HDR screen
    /// driven in BT.2100/PQ signalling mode.
    ///
    /// Windows-scRGB uses sRGB (BT.709) color primaries and white point.
    /// The transfer characteristic is extended linear.
    ///
    /// The nominal color channel value range is extended, meaning it includes
    /// negative and greater than 1.0 values. Negative values are used to
    /// escape the sRGB color gamut boundaries. To make use of the extended
    /// range, the client needs to use a pixel format that can represent those
    /// values, e.g. floating-point 16 bits per channel.
    ///
    /// Nominal color value R=G=B=0.0 corresponds to BT.2100/PQ system
    /// 0 cd/m², and R=G=B=1.0 corresponds to BT.2100/PQ system 80 cd/m².
    /// The maximum is R=G=B=125.0 corresponding to 10k cd/m².
    ///
    /// Windows-scRGB is displayed by Windows 10 by converting it to
    /// BT.2100/PQ, maintaining the CIE 1931 chromaticity and mapping the
    /// luminance as above. No adjustment is made to the signal to account
    /// for the viewing conditions.
    ///
    /// The reference white level of Windows-scRGB is unknown. If a
    /// reference white level must be assumed for compositor processing, it
    /// should be R=G=B=2.5375 corresponding to 203 cd/m² of Report ITU-R
    /// BT.2408-7.
    ///
    /// The target color volume of Windows-scRGB is unknown. The color gamut
    /// may be anything between sRGB and BT.2100.
    ///
    /// Note: EGL_EXT_gl_colorspace_scrgb_linear definition differs from
    /// Windows-scRGB by using R=G=B=1.0 as the reference white level, while
    /// Windows-scRGB reference white level is unknown or varies. However,
    /// it seems probable that Windows implements both
    /// EGL_EXT_gl_colorspace_scrgb_linear and Vulkan
    /// VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT as Windows-scRGB.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.windows_scrgb.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// The resulting image description object does not allow get_information
    /// request. The wp_image_description_v1.ready event shall be sent.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    pub fn try_send_create_windows_scrgb(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_manager_v1#{}.create_windows_scrgb(image_description: wp_image_description_v1#{})\n", id, arg0);
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
            6,
            arg0_id,
        ]);
        Ok(())
    }

    /// create Windows-scRGB image description object
    ///
    /// This creates a pre-defined image description for the so-called
    /// Windows-scRGB stimulus encoding. This comes from the Windows 10 handling
    /// of its own definition of an scRGB color space for an HDR screen
    /// driven in BT.2100/PQ signalling mode.
    ///
    /// Windows-scRGB uses sRGB (BT.709) color primaries and white point.
    /// The transfer characteristic is extended linear.
    ///
    /// The nominal color channel value range is extended, meaning it includes
    /// negative and greater than 1.0 values. Negative values are used to
    /// escape the sRGB color gamut boundaries. To make use of the extended
    /// range, the client needs to use a pixel format that can represent those
    /// values, e.g. floating-point 16 bits per channel.
    ///
    /// Nominal color value R=G=B=0.0 corresponds to BT.2100/PQ system
    /// 0 cd/m², and R=G=B=1.0 corresponds to BT.2100/PQ system 80 cd/m².
    /// The maximum is R=G=B=125.0 corresponding to 10k cd/m².
    ///
    /// Windows-scRGB is displayed by Windows 10 by converting it to
    /// BT.2100/PQ, maintaining the CIE 1931 chromaticity and mapping the
    /// luminance as above. No adjustment is made to the signal to account
    /// for the viewing conditions.
    ///
    /// The reference white level of Windows-scRGB is unknown. If a
    /// reference white level must be assumed for compositor processing, it
    /// should be R=G=B=2.5375 corresponding to 203 cd/m² of Report ITU-R
    /// BT.2408-7.
    ///
    /// The target color volume of Windows-scRGB is unknown. The color gamut
    /// may be anything between sRGB and BT.2100.
    ///
    /// Note: EGL_EXT_gl_colorspace_scrgb_linear definition differs from
    /// Windows-scRGB by using R=G=B=1.0 as the reference white level, while
    /// Windows-scRGB reference white level is unknown or varies. However,
    /// it seems probable that Windows implements both
    /// EGL_EXT_gl_colorspace_scrgb_linear and Vulkan
    /// VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT as Windows-scRGB.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.windows_scrgb.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// The resulting image description object does not allow get_information
    /// request. The wp_image_description_v1.ready event shall be sent.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    pub fn send_create_windows_scrgb(
        &self,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        let res = self.try_send_create_windows_scrgb(
            image_description,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.create_windows_scrgb", &e);
        }
    }

    /// create Windows-scRGB image description object
    ///
    /// This creates a pre-defined image description for the so-called
    /// Windows-scRGB stimulus encoding. This comes from the Windows 10 handling
    /// of its own definition of an scRGB color space for an HDR screen
    /// driven in BT.2100/PQ signalling mode.
    ///
    /// Windows-scRGB uses sRGB (BT.709) color primaries and white point.
    /// The transfer characteristic is extended linear.
    ///
    /// The nominal color channel value range is extended, meaning it includes
    /// negative and greater than 1.0 values. Negative values are used to
    /// escape the sRGB color gamut boundaries. To make use of the extended
    /// range, the client needs to use a pixel format that can represent those
    /// values, e.g. floating-point 16 bits per channel.
    ///
    /// Nominal color value R=G=B=0.0 corresponds to BT.2100/PQ system
    /// 0 cd/m², and R=G=B=1.0 corresponds to BT.2100/PQ system 80 cd/m².
    /// The maximum is R=G=B=125.0 corresponding to 10k cd/m².
    ///
    /// Windows-scRGB is displayed by Windows 10 by converting it to
    /// BT.2100/PQ, maintaining the CIE 1931 chromaticity and mapping the
    /// luminance as above. No adjustment is made to the signal to account
    /// for the viewing conditions.
    ///
    /// The reference white level of Windows-scRGB is unknown. If a
    /// reference white level must be assumed for compositor processing, it
    /// should be R=G=B=2.5375 corresponding to 203 cd/m² of Report ITU-R
    /// BT.2408-7.
    ///
    /// The target color volume of Windows-scRGB is unknown. The color gamut
    /// may be anything between sRGB and BT.2100.
    ///
    /// Note: EGL_EXT_gl_colorspace_scrgb_linear definition differs from
    /// Windows-scRGB by using R=G=B=1.0 as the reference white level, while
    /// Windows-scRGB reference white level is unknown or varies. However,
    /// it seems probable that Windows implements both
    /// EGL_EXT_gl_colorspace_scrgb_linear and Vulkan
    /// VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT as Windows-scRGB.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.windows_scrgb.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// The resulting image description object does not allow get_information
    /// request. The wp_image_description_v1.ready event shall be sent.
    #[inline]
    pub fn new_try_send_create_windows_scrgb(
        &self,
    ) -> Result<Rc<WpImageDescriptionV1>, ObjectError> {
        let image_description = self.core.create_child();
        self.try_send_create_windows_scrgb(
            &image_description,
        )?;
        Ok(image_description)
    }

    /// create Windows-scRGB image description object
    ///
    /// This creates a pre-defined image description for the so-called
    /// Windows-scRGB stimulus encoding. This comes from the Windows 10 handling
    /// of its own definition of an scRGB color space for an HDR screen
    /// driven in BT.2100/PQ signalling mode.
    ///
    /// Windows-scRGB uses sRGB (BT.709) color primaries and white point.
    /// The transfer characteristic is extended linear.
    ///
    /// The nominal color channel value range is extended, meaning it includes
    /// negative and greater than 1.0 values. Negative values are used to
    /// escape the sRGB color gamut boundaries. To make use of the extended
    /// range, the client needs to use a pixel format that can represent those
    /// values, e.g. floating-point 16 bits per channel.
    ///
    /// Nominal color value R=G=B=0.0 corresponds to BT.2100/PQ system
    /// 0 cd/m², and R=G=B=1.0 corresponds to BT.2100/PQ system 80 cd/m².
    /// The maximum is R=G=B=125.0 corresponding to 10k cd/m².
    ///
    /// Windows-scRGB is displayed by Windows 10 by converting it to
    /// BT.2100/PQ, maintaining the CIE 1931 chromaticity and mapping the
    /// luminance as above. No adjustment is made to the signal to account
    /// for the viewing conditions.
    ///
    /// The reference white level of Windows-scRGB is unknown. If a
    /// reference white level must be assumed for compositor processing, it
    /// should be R=G=B=2.5375 corresponding to 203 cd/m² of Report ITU-R
    /// BT.2408-7.
    ///
    /// The target color volume of Windows-scRGB is unknown. The color gamut
    /// may be anything between sRGB and BT.2100.
    ///
    /// Note: EGL_EXT_gl_colorspace_scrgb_linear definition differs from
    /// Windows-scRGB by using R=G=B=1.0 as the reference white level, while
    /// Windows-scRGB reference white level is unknown or varies. However,
    /// it seems probable that Windows implements both
    /// EGL_EXT_gl_colorspace_scrgb_linear and Vulkan
    /// VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT as Windows-scRGB.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.windows_scrgb.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// The resulting image description object does not allow get_information
    /// request. The wp_image_description_v1.ready event shall be sent.
    #[inline]
    pub fn new_send_create_windows_scrgb(
        &self,
    ) -> Rc<WpImageDescriptionV1> {
        let image_description = self.core.create_child();
        self.send_create_windows_scrgb(
            &image_description,
        );
        image_description
    }

    /// Since when the supported_intent message is available.
    pub const MSG__SUPPORTED_INTENT__SINCE: u32 = 1;

    /// supported rendering intent
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each rendering intent the compositor supports.
    ///
    /// A compositor must not advertise intents that are deprecated in the
    /// bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `render_intent`: rendering intent
    #[inline]
    pub fn try_send_supported_intent(
        &self,
        render_intent: WpColorManagerV1RenderIntent,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            render_intent,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WpColorManagerV1RenderIntent) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_manager_v1#{}.supported_intent(render_intent: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// supported rendering intent
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each rendering intent the compositor supports.
    ///
    /// A compositor must not advertise intents that are deprecated in the
    /// bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `render_intent`: rendering intent
    #[inline]
    pub fn send_supported_intent(
        &self,
        render_intent: WpColorManagerV1RenderIntent,
    ) {
        let res = self.try_send_supported_intent(
            render_intent,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.supported_intent", &e);
        }
    }

    /// Since when the supported_feature message is available.
    pub const MSG__SUPPORTED_FEATURE__SINCE: u32 = 1;

    /// supported features
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each compositor supported feature listed in the enumeration.
    ///
    /// A compositor must not advertise features that are deprecated in the
    /// bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `feature`: supported feature
    #[inline]
    pub fn try_send_supported_feature(
        &self,
        feature: WpColorManagerV1Feature,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            feature,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WpColorManagerV1Feature) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_manager_v1#{}.supported_feature(feature: {:?})\n", client_id, id, arg0);
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
            1,
            arg0.0,
        ]);
        Ok(())
    }

    /// supported features
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each compositor supported feature listed in the enumeration.
    ///
    /// A compositor must not advertise features that are deprecated in the
    /// bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `feature`: supported feature
    #[inline]
    pub fn send_supported_feature(
        &self,
        feature: WpColorManagerV1Feature,
    ) {
        let res = self.try_send_supported_feature(
            feature,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.supported_feature", &e);
        }
    }

    /// Since when the supported_tf_named message is available.
    pub const MSG__SUPPORTED_TF_NAMED__SINCE: u32 = 1;

    /// supported named transfer characteristic
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each named transfer function the compositor supports with the
    /// parametric image description creator.
    ///
    /// A compositor must not advertise transfer functions that are deprecated
    /// in the bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `tf`: Named transfer function
    #[inline]
    pub fn try_send_supported_tf_named(
        &self,
        tf: WpColorManagerV1TransferFunction,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            tf,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WpColorManagerV1TransferFunction) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_manager_v1#{}.supported_tf_named(tf: {:?})\n", client_id, id, arg0);
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
            2,
            arg0.0,
        ]);
        Ok(())
    }

    /// supported named transfer characteristic
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each named transfer function the compositor supports with the
    /// parametric image description creator.
    ///
    /// A compositor must not advertise transfer functions that are deprecated
    /// in the bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `tf`: Named transfer function
    #[inline]
    pub fn send_supported_tf_named(
        &self,
        tf: WpColorManagerV1TransferFunction,
    ) {
        let res = self.try_send_supported_tf_named(
            tf,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.supported_tf_named", &e);
        }
    }

    /// Since when the supported_primaries_named message is available.
    pub const MSG__SUPPORTED_PRIMARIES_NAMED__SINCE: u32 = 1;

    /// supported named primaries
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each named set of primaries the compositor supports with the
    /// parametric image description creator.
    ///
    /// A compositor must not advertise names that are deprecated in the
    /// bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `primaries`: Named color primaries
    #[inline]
    pub fn try_send_supported_primaries_named(
        &self,
        primaries: WpColorManagerV1Primaries,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            primaries,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WpColorManagerV1Primaries) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_manager_v1#{}.supported_primaries_named(primaries: {:?})\n", client_id, id, arg0);
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
            3,
            arg0.0,
        ]);
        Ok(())
    }

    /// supported named primaries
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each named set of primaries the compositor supports with the
    /// parametric image description creator.
    ///
    /// A compositor must not advertise names that are deprecated in the
    /// bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `primaries`: Named color primaries
    #[inline]
    pub fn send_supported_primaries_named(
        &self,
        primaries: WpColorManagerV1Primaries,
    ) {
        let res = self.try_send_supported_primaries_named(
            primaries,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.supported_primaries_named", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all features have been sent
    ///
    /// This event is sent when all supported rendering intents, features,
    /// transfer functions and named primaries have been sent.
    #[inline]
    pub fn try_send_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_manager_v1#{}.done()\n", client_id, id);
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
            4,
        ]);
        Ok(())
    }

    /// all features have been sent
    ///
    /// This event is sent when all supported rendering intents, features,
    /// transfer functions and named primaries have been sent.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.done", &e);
        }
    }

    /// Since when the get_image_description message is available.
    pub const MSG__GET_IMAGE_DESCRIPTION__SINCE: u32 = 2;

    /// create an image description from a reference
    ///
    /// This request retrieves the image description backing a reference.
    ///
    /// The get_information request can be used if and only if the request that
    /// creates the reference allows it.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    /// - `reference`:
    #[inline]
    pub fn try_send_get_image_description(
        &self,
        image_description: &Rc<WpImageDescriptionV1>,
        reference: &Rc<WpImageDescriptionReferenceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            image_description,
            reference,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("reference"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("image_description", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_manager_v1#{}.get_image_description(image_description: wp_image_description_v1#{}, reference: wp_image_description_reference_v1#{})\n", id, arg0, arg1);
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
            7,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// create an image description from a reference
    ///
    /// This request retrieves the image description backing a reference.
    ///
    /// The get_information request can be used if and only if the request that
    /// creates the reference allows it.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    /// - `reference`:
    #[inline]
    pub fn send_get_image_description(
        &self,
        image_description: &Rc<WpImageDescriptionV1>,
        reference: &Rc<WpImageDescriptionReferenceV1>,
    ) {
        let res = self.try_send_get_image_description(
            image_description,
            reference,
        );
        if let Err(e) = res {
            log_send("wp_color_manager_v1.get_image_description", &e);
        }
    }

    /// create an image description from a reference
    ///
    /// This request retrieves the image description backing a reference.
    ///
    /// The get_information request can be used if and only if the request that
    /// creates the reference allows it.
    ///
    /// # Arguments
    ///
    /// - `reference`:
    #[inline]
    pub fn new_try_send_get_image_description(
        &self,
        reference: &Rc<WpImageDescriptionReferenceV1>,
    ) -> Result<Rc<WpImageDescriptionV1>, ObjectError> {
        let image_description = self.core.create_child();
        self.try_send_get_image_description(
            &image_description,
            reference,
        )?;
        Ok(image_description)
    }

    /// create an image description from a reference
    ///
    /// This request retrieves the image description backing a reference.
    ///
    /// The get_information request can be used if and only if the request that
    /// creates the reference allows it.
    ///
    /// # Arguments
    ///
    /// - `reference`:
    #[inline]
    pub fn new_send_get_image_description(
        &self,
        reference: &Rc<WpImageDescriptionReferenceV1>,
    ) -> Rc<WpImageDescriptionV1> {
        let image_description = self.core.create_child();
        self.send_get_image_description(
            &image_description,
            reference,
        );
        image_description
    }
}

/// A message handler for [`WpColorManagerV1`] proxies.
pub trait WpColorManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpColorManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the color manager
    ///
    /// Destroy the wp_color_manager_v1 object. This does not affect any other
    /// objects in any way.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.destroy", &e);
        }
    }

    /// create a color management interface for a wl_output
    ///
    /// This creates a new wp_color_management_output_v1 object for the
    /// given wl_output.
    ///
    /// See the wp_color_management_output_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_output(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        id: &Rc<WpColorManagementOutputV1>,
        output: &Rc<WlOutput>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_output(
            id,
            output,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.get_output", &e);
        }
    }

    /// create a color management interface for a wl_surface
    ///
    /// If a wp_color_management_surface_v1 object already exists for the given
    /// wl_surface, the protocol error surface_exists is raised.
    ///
    /// This creates a new color wp_color_management_surface_v1 object for the
    /// given wl_surface.
    ///
    /// See the wp_color_management_surface_v1 interface for more details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_surface(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        id: &Rc<WpColorManagementSurfaceV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.get_surface", &e);
        }
    }

    /// create a color management feedback interface
    ///
    /// This creates a new color wp_color_management_surface_feedback_v1 object
    /// for the given wl_surface.
    ///
    /// See the wp_color_management_surface_feedback_v1 interface for more
    /// details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_surface_feedback(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        id: &Rc<WpColorManagementSurfaceFeedbackV1>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_surface_feedback(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.get_surface_feedback", &e);
        }
    }

    /// make a new ICC-based image description creator object
    ///
    /// Makes a new ICC-based image description creator object with all
    /// properties initially unset. The client can then use the object's
    /// interface to define all the required properties for an image description
    /// and finally create a wp_image_description_v1 object.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.icc_v2_v4.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// # Arguments
    ///
    /// - `obj`: the new creator object
    #[inline]
    fn handle_create_icc_creator(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        obj: &Rc<WpImageDescriptionCreatorIccV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_icc_creator(
            obj,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.create_icc_creator", &e);
        }
    }

    /// make a new parametric image description creator object
    ///
    /// Makes a new parametric image description creator object with all
    /// properties initially unset. The client can then use the object's
    /// interface to define all the required properties for an image description
    /// and finally create a wp_image_description_v1 object.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.parametric.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// # Arguments
    ///
    /// - `obj`: the new creator object
    #[inline]
    fn handle_create_parametric_creator(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        obj: &Rc<WpImageDescriptionCreatorParamsV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_parametric_creator(
            obj,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.create_parametric_creator", &e);
        }
    }

    /// create Windows-scRGB image description object
    ///
    /// This creates a pre-defined image description for the so-called
    /// Windows-scRGB stimulus encoding. This comes from the Windows 10 handling
    /// of its own definition of an scRGB color space for an HDR screen
    /// driven in BT.2100/PQ signalling mode.
    ///
    /// Windows-scRGB uses sRGB (BT.709) color primaries and white point.
    /// The transfer characteristic is extended linear.
    ///
    /// The nominal color channel value range is extended, meaning it includes
    /// negative and greater than 1.0 values. Negative values are used to
    /// escape the sRGB color gamut boundaries. To make use of the extended
    /// range, the client needs to use a pixel format that can represent those
    /// values, e.g. floating-point 16 bits per channel.
    ///
    /// Nominal color value R=G=B=0.0 corresponds to BT.2100/PQ system
    /// 0 cd/m², and R=G=B=1.0 corresponds to BT.2100/PQ system 80 cd/m².
    /// The maximum is R=G=B=125.0 corresponding to 10k cd/m².
    ///
    /// Windows-scRGB is displayed by Windows 10 by converting it to
    /// BT.2100/PQ, maintaining the CIE 1931 chromaticity and mapping the
    /// luminance as above. No adjustment is made to the signal to account
    /// for the viewing conditions.
    ///
    /// The reference white level of Windows-scRGB is unknown. If a
    /// reference white level must be assumed for compositor processing, it
    /// should be R=G=B=2.5375 corresponding to 203 cd/m² of Report ITU-R
    /// BT.2408-7.
    ///
    /// The target color volume of Windows-scRGB is unknown. The color gamut
    /// may be anything between sRGB and BT.2100.
    ///
    /// Note: EGL_EXT_gl_colorspace_scrgb_linear definition differs from
    /// Windows-scRGB by using R=G=B=1.0 as the reference white level, while
    /// Windows-scRGB reference white level is unknown or varies. However,
    /// it seems probable that Windows implements both
    /// EGL_EXT_gl_colorspace_scrgb_linear and Vulkan
    /// VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT as Windows-scRGB.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.windows_scrgb.
    /// Otherwise this request raises the protocol error unsupported_feature.
    ///
    /// The resulting image description object does not allow get_information
    /// request. The wp_image_description_v1.ready event shall be sent.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    fn handle_create_windows_scrgb(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_windows_scrgb(
            image_description,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.create_windows_scrgb", &e);
        }
    }

    /// supported rendering intent
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each rendering intent the compositor supports.
    ///
    /// A compositor must not advertise intents that are deprecated in the
    /// bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `render_intent`: rendering intent
    #[inline]
    fn handle_supported_intent(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        render_intent: WpColorManagerV1RenderIntent,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_supported_intent(
            render_intent,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.supported_intent", &e);
        }
    }

    /// supported features
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each compositor supported feature listed in the enumeration.
    ///
    /// A compositor must not advertise features that are deprecated in the
    /// bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `feature`: supported feature
    #[inline]
    fn handle_supported_feature(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        feature: WpColorManagerV1Feature,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_supported_feature(
            feature,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.supported_feature", &e);
        }
    }

    /// supported named transfer characteristic
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each named transfer function the compositor supports with the
    /// parametric image description creator.
    ///
    /// A compositor must not advertise transfer functions that are deprecated
    /// in the bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `tf`: Named transfer function
    #[inline]
    fn handle_supported_tf_named(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        tf: WpColorManagerV1TransferFunction,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_supported_tf_named(
            tf,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.supported_tf_named", &e);
        }
    }

    /// supported named primaries
    ///
    /// When this object is created, it shall immediately send this event once
    /// for each named set of primaries the compositor supports with the
    /// parametric image description creator.
    ///
    /// A compositor must not advertise names that are deprecated in the
    /// bound version of the interface.
    ///
    /// # Arguments
    ///
    /// - `primaries`: Named color primaries
    #[inline]
    fn handle_supported_primaries_named(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        primaries: WpColorManagerV1Primaries,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_supported_primaries_named(
            primaries,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.supported_primaries_named", &e);
        }
    }

    /// all features have been sent
    ///
    /// This event is sent when all supported rendering intents, features,
    /// transfer functions and named primaries have been sent.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.done", &e);
        }
    }

    /// create an image description from a reference
    ///
    /// This request retrieves the image description backing a reference.
    ///
    /// The get_information request can be used if and only if the request that
    /// creates the reference allows it.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    /// - `reference`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_image_description(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        image_description: &Rc<WpImageDescriptionV1>,
        reference: &Rc<WpImageDescriptionReferenceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_image_description(
            image_description,
            reference,
        );
        if let Err(e) = res {
            log_forward("wp_color_manager_v1.get_image_description", &e);
        }
    }
}

impl ObjectPrivate for WpColorManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpColorManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_manager_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_manager_v1#{}.get_output(id: wp_color_management_output_v1#{}, output: wl_output#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WpColorManagementOutputV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_output(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_output(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_manager_v1#{}.get_surface(id: wp_color_management_surface_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WpColorManagementSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_surface(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_surface(&self, arg0, arg1);
                }
            }
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_manager_v1#{}.get_surface_feedback(id: wp_color_management_surface_feedback_v1#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WpColorManagementSurfaceFeedbackV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_surface_feedback(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_surface_feedback(&self, arg0, arg1);
                }
            }
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_manager_v1#{}.create_icc_creator(obj: wp_image_description_creator_icc_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WpImageDescriptionCreatorIccV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "obj", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_icc_creator(&self, arg0);
                } else {
                    DefaultHandler.handle_create_icc_creator(&self, arg0);
                }
            }
            5 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_manager_v1#{}.create_parametric_creator(obj: wp_image_description_creator_params_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WpImageDescriptionCreatorParamsV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "obj", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_parametric_creator(&self, arg0);
                } else {
                    DefaultHandler.handle_create_parametric_creator(&self, arg0);
                }
            }
            6 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_manager_v1#{}.create_windows_scrgb(image_description: wp_image_description_v1#{})\n", client_id, id, arg0);
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
                    (**handler).handle_create_windows_scrgb(&self, arg0);
                } else {
                    DefaultHandler.handle_create_windows_scrgb(&self, arg0);
                }
            }
            7 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_manager_v1#{}.get_image_description(image_description: wp_image_description_v1#{}, reference: wp_image_description_reference_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = WpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "image_description", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WpImageDescriptionReferenceV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("reference", o.core().interface, ObjectInterface::WpImageDescriptionReferenceV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_image_description(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_image_description(&self, arg0, arg1);
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
                let arg0 = WpColorManagerV1RenderIntent(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WpColorManagerV1RenderIntent) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_manager_v1#{}.supported_intent(render_intent: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_supported_intent(&self, arg0);
                } else {
                    DefaultHandler.handle_supported_intent(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WpColorManagerV1Feature(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WpColorManagerV1Feature) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_manager_v1#{}.supported_feature(feature: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_supported_feature(&self, arg0);
                } else {
                    DefaultHandler.handle_supported_feature(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WpColorManagerV1TransferFunction(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WpColorManagerV1TransferFunction) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_manager_v1#{}.supported_tf_named(tf: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_supported_tf_named(&self, arg0);
                } else {
                    DefaultHandler.handle_supported_tf_named(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WpColorManagerV1Primaries(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WpColorManagerV1Primaries) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_manager_v1#{}.supported_primaries_named(primaries: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_supported_primaries_named(&self, arg0);
                } else {
                    DefaultHandler.handle_supported_primaries_named(&self, arg0);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_manager_v1#{}.done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
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
            1 => "get_output",
            2 => "get_surface",
            3 => "get_surface_feedback",
            4 => "create_icc_creator",
            5 => "create_parametric_creator",
            6 => "create_windows_scrgb",
            7 => "get_image_description",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "supported_intent",
            1 => "supported_feature",
            2 => "supported_tf_named",
            3 => "supported_primaries_named",
            4 => "done",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WpColorManagerV1 {
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

impl WpColorManagerV1 {
    /// Since when the error.unsupported_feature enum variant is available.
    pub const ENM__ERROR_UNSUPPORTED_FEATURE__SINCE: u32 = 1;
    /// Since when the error.surface_exists enum variant is available.
    pub const ENM__ERROR_SURFACE_EXISTS__SINCE: u32 = 1;

    /// Since when the render_intent.perceptual enum variant is available.
    pub const ENM__RENDER_INTENT_PERCEPTUAL__SINCE: u32 = 1;
    /// Since when the render_intent.relative enum variant is available.
    pub const ENM__RENDER_INTENT_RELATIVE__SINCE: u32 = 1;
    /// Since when the render_intent.saturation enum variant is available.
    pub const ENM__RENDER_INTENT_SATURATION__SINCE: u32 = 1;
    /// Since when the render_intent.absolute enum variant is available.
    pub const ENM__RENDER_INTENT_ABSOLUTE__SINCE: u32 = 1;
    /// Since when the render_intent.relative_bpc enum variant is available.
    pub const ENM__RENDER_INTENT_RELATIVE_BPC__SINCE: u32 = 1;
    /// Since when the render_intent.absolute_no_adaptation enum variant is available.
    pub const ENM__RENDER_INTENT_ABSOLUTE_NO_ADAPTATION__SINCE: u32 = 2;

    /// Since when the feature.icc_v2_v4 enum variant is available.
    pub const ENM__FEATURE_ICC_V2_V4__SINCE: u32 = 1;
    /// Since when the feature.parametric enum variant is available.
    pub const ENM__FEATURE_PARAMETRIC__SINCE: u32 = 1;
    /// Since when the feature.set_primaries enum variant is available.
    pub const ENM__FEATURE_SET_PRIMARIES__SINCE: u32 = 1;
    /// Since when the feature.set_tf_power enum variant is available.
    pub const ENM__FEATURE_SET_TF_POWER__SINCE: u32 = 1;
    /// Since when the feature.set_luminances enum variant is available.
    pub const ENM__FEATURE_SET_LUMINANCES__SINCE: u32 = 1;
    /// Since when the feature.set_mastering_display_primaries enum variant is available.
    pub const ENM__FEATURE_SET_MASTERING_DISPLAY_PRIMARIES__SINCE: u32 = 1;
    /// Since when the feature.extended_target_volume enum variant is available.
    pub const ENM__FEATURE_EXTENDED_TARGET_VOLUME__SINCE: u32 = 1;
    /// Since when the feature.windows_scrgb enum variant is available.
    pub const ENM__FEATURE_WINDOWS_SCRGB__SINCE: u32 = 1;

    /// Since when the primaries.srgb enum variant is available.
    pub const ENM__PRIMARIES_SRGB__SINCE: u32 = 1;
    /// Since when the primaries.pal_m enum variant is available.
    pub const ENM__PRIMARIES_PAL_M__SINCE: u32 = 1;
    /// Since when the primaries.pal enum variant is available.
    pub const ENM__PRIMARIES_PAL__SINCE: u32 = 1;
    /// Since when the primaries.ntsc enum variant is available.
    pub const ENM__PRIMARIES_NTSC__SINCE: u32 = 1;
    /// Since when the primaries.generic_film enum variant is available.
    pub const ENM__PRIMARIES_GENERIC_FILM__SINCE: u32 = 1;
    /// Since when the primaries.bt2020 enum variant is available.
    pub const ENM__PRIMARIES_BT2020__SINCE: u32 = 1;
    /// Since when the primaries.cie1931_xyz enum variant is available.
    pub const ENM__PRIMARIES_CIE1931_XYZ__SINCE: u32 = 1;
    /// Since when the primaries.dci_p3 enum variant is available.
    pub const ENM__PRIMARIES_DCI_P3__SINCE: u32 = 1;
    /// Since when the primaries.display_p3 enum variant is available.
    pub const ENM__PRIMARIES_DISPLAY_P3__SINCE: u32 = 1;
    /// Since when the primaries.adobe_rgb enum variant is available.
    pub const ENM__PRIMARIES_ADOBE_RGB__SINCE: u32 = 1;

    /// Since when the transfer_function.bt1886 enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_BT1886__SINCE: u32 = 1;
    /// Since when the transfer_function.gamma22 enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_GAMMA22__SINCE: u32 = 1;
    /// Since when the transfer_function.gamma28 enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_GAMMA28__SINCE: u32 = 1;
    /// Since when the transfer_function.st240 enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_ST240__SINCE: u32 = 1;
    /// Since when the transfer_function.ext_linear enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_EXT_LINEAR__SINCE: u32 = 1;
    /// Since when the transfer_function.log_100 enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_LOG_100__SINCE: u32 = 1;
    /// Since when the transfer_function.log_316 enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_LOG_316__SINCE: u32 = 1;
    /// Since when the transfer_function.xvycc enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_XVYCC__SINCE: u32 = 1;
    /// Since when the transfer_function.srgb enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_SRGB__SINCE: u32 = 1;

    /// Since when the transfer_function.srgb enum variant is deprecated.
    pub const ENM__TRANSFER_FUNCTION_SRGB__DEPRECATED_SINCE: u32 = 2;
    /// Since when the transfer_function.ext_srgb enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_EXT_SRGB__SINCE: u32 = 1;

    /// Since when the transfer_function.ext_srgb enum variant is deprecated.
    pub const ENM__TRANSFER_FUNCTION_EXT_SRGB__DEPRECATED_SINCE: u32 = 2;
    /// Since when the transfer_function.st2084_pq enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_ST2084_PQ__SINCE: u32 = 1;
    /// Since when the transfer_function.st428 enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_ST428__SINCE: u32 = 1;
    /// Since when the transfer_function.hlg enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_HLG__SINCE: u32 = 1;
    /// Since when the transfer_function.compound_power_2_4 enum variant is available.
    pub const ENM__TRANSFER_FUNCTION_COMPOUND_POWER_2_4__SINCE: u32 = 2;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorManagerV1Error(pub u32);

impl WpColorManagerV1Error {
    /// request not supported
    pub const UNSUPPORTED_FEATURE: Self = Self(0);

    /// color management surface exists already
    pub const SURFACE_EXISTS: Self = Self(1);
}

impl Debug for WpColorManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::UNSUPPORTED_FEATURE => "UNSUPPORTED_FEATURE",
            Self::SURFACE_EXISTS => "SURFACE_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// rendering intents
///
/// See the ICC.1:2022 specification from the International Color Consortium
/// for more details about rendering intents.
///
/// The principles of ICC defined rendering intents apply with all types of
/// image descriptions, not only those with ICC file profiles.
///
/// Compositors must support the perceptual rendering intent. Other
/// rendering intents are optional.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorManagerV1RenderIntent(pub u32);

impl WpColorManagerV1RenderIntent {
    /// perceptual
    pub const PERCEPTUAL: Self = Self(0);

    /// media-relative colorimetric
    pub const RELATIVE: Self = Self(1);

    /// saturation
    pub const SATURATION: Self = Self(2);

    /// ICC-absolute colorimetric
    pub const ABSOLUTE: Self = Self(3);

    /// media-relative colorimetric + black point compensation
    pub const RELATIVE_BPC: Self = Self(4);

    /// ICC-absolute colorimetric without adaptation
    ///
    /// This rendering intent is a modified absolute rendering intent that
    /// assumes the viewer is not adapted to the display white point, so no
    /// chromatic adaptation between surface and display is done.
    /// This can be useful for color proofing applications.
    pub const ABSOLUTE_NO_ADAPTATION: Self = Self(5);
}

impl Debug for WpColorManagerV1RenderIntent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::PERCEPTUAL => "PERCEPTUAL",
            Self::RELATIVE => "RELATIVE",
            Self::SATURATION => "SATURATION",
            Self::ABSOLUTE => "ABSOLUTE",
            Self::RELATIVE_BPC => "RELATIVE_BPC",
            Self::ABSOLUTE_NO_ADAPTATION => "ABSOLUTE_NO_ADAPTATION",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// compositor supported features
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorManagerV1Feature(pub u32);

impl WpColorManagerV1Feature {
    /// create_icc_creator request
    pub const ICC_V2_V4: Self = Self(0);

    /// create_parametric_creator request
    pub const PARAMETRIC: Self = Self(1);

    /// parametric set_primaries request
    pub const SET_PRIMARIES: Self = Self(2);

    /// parametric set_tf_power request
    pub const SET_TF_POWER: Self = Self(3);

    /// parametric set_luminances request
    pub const SET_LUMINANCES: Self = Self(4);

    /// parametric set_mastering_display_primaries request
    ///
    /// The compositor supports set_mastering_display_primaries request with a
    /// target color volume fully contained inside the primary color volume.
    pub const SET_MASTERING_DISPLAY_PRIMARIES: Self = Self(5);

    /// parametric target exceeds primary color volume
    ///
    /// The compositor additionally supports target color volumes that
    /// extend outside of the primary color volume.
    ///
    /// This can only be advertised if feature set_mastering_display_primaries
    /// is supported as well.
    pub const EXTENDED_TARGET_VOLUME: Self = Self(6);

    /// create_windows_scrgb request
    pub const WINDOWS_SCRGB: Self = Self(7);
}

impl Debug for WpColorManagerV1Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ICC_V2_V4 => "ICC_V2_V4",
            Self::PARAMETRIC => "PARAMETRIC",
            Self::SET_PRIMARIES => "SET_PRIMARIES",
            Self::SET_TF_POWER => "SET_TF_POWER",
            Self::SET_LUMINANCES => "SET_LUMINANCES",
            Self::SET_MASTERING_DISPLAY_PRIMARIES => "SET_MASTERING_DISPLAY_PRIMARIES",
            Self::EXTENDED_TARGET_VOLUME => "EXTENDED_TARGET_VOLUME",
            Self::WINDOWS_SCRGB => "WINDOWS_SCRGB",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// named color primaries
///
/// Named color primaries used to encode well-known sets of primaries.
///
/// A value of 0 is invalid and will never be present in the list of enums.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorManagerV1Primaries(pub u32);

impl WpColorManagerV1Primaries {
    /// Color primaries for the sRGB color space as defined by the BT.709 standard
    ///
    /// Color primaries as defined by
    /// - Rec. ITU-R BT.709-6
    /// - Rec. ITU-R BT.1361-0 conventional colour gamut system and extended
    ///   colour gamut system (historical)
    /// - IEC 61966-2-1 sRGB or sYCC
    /// - IEC 61966-2-4
    /// - Society of Motion Picture and Television Engineers (SMPTE) RP 177
    ///   (1993) Annex B
    pub const SRGB: Self = Self(1);

    /// Color primaries for PAL-M as defined by the BT.470 standard
    ///
    /// Color primaries as defined by
    /// - Rec. ITU-R BT.470-6 System M (historical)
    /// - United States National Television System Committee 1953
    ///   Recommendation for transmission standards for color television
    /// - United States Federal Communications Commission (2003) Title 47 Code
    ///   of Federal Regulations 73.682 (a)(20)
    pub const PAL_M: Self = Self(2);

    /// Color primaries for PAL as defined by the BT.601 standard
    ///
    /// Color primaries as defined by
    /// - Rec. ITU-R BT.470-6 System B, G (historical)
    /// - Rec. ITU-R BT.601-7 625
    /// - Rec. ITU-R BT.1358-0 625 (historical)
    /// - Rec. ITU-R BT.1700-0 625 PAL and 625 SECAM
    pub const PAL: Self = Self(3);

    /// Color primaries for NTSC as defined by the BT.601 standard
    ///
    /// Color primaries as defined by
    /// - Rec. ITU-R BT.601-7 525
    /// - Rec. ITU-R BT.1358-1 525 or 625 (historical)
    /// - Rec. ITU-R BT.1700-0 NTSC
    /// - SMPTE 170M (2004)
    /// - SMPTE 240M (1999) (historical)
    pub const NTSC: Self = Self(4);

    /// Generic film with colour filters using Illuminant C
    ///
    /// Color primaries as defined by Recommendation ITU-T H.273
    /// "Coding-independent code points for video signal type identification"
    /// for "generic film".
    pub const GENERIC_FILM: Self = Self(5);

    /// Color primaries as defined by the BT.2020 and BT.2100 standard
    ///
    /// Color primaries as defined by
    /// - Rec. ITU-R BT.2020-2
    /// - Rec. ITU-R BT.2100-0
    pub const BT2020: Self = Self(6);

    /// Color primaries of the full CIE 1931 XYZ color space
    ///
    /// Color primaries as defined as the maximum of the CIE 1931 XYZ color
    /// space by
    /// - SMPTE ST 428-1
    /// - (CIE 1931 XYZ as in ISO 11664-1)
    pub const CIE1931_XYZ: Self = Self(7);

    /// Color primaries of the DCI P3 color space as defined by the SMPTE RP 431 standard
    ///
    /// Color primaries as defined by Digital Cinema System and published in
    /// SMPTE RP 431-2 (2011).
    pub const DCI_P3: Self = Self(8);

    /// Color primaries of Display P3 variant of the DCI-P3 color space as defined by the SMPTE EG 432 standard
    ///
    /// Color primaries as defined by Digital Cinema System and published in
    /// SMPTE EG 432-1 (2010).
    pub const DISPLAY_P3: Self = Self(9);

    /// Color primaries of the Adobe RGB color space as defined by the ISO 12640 standard
    ///
    /// Color primaries as defined by Adobe as "Adobe RGB" and later published
    /// by ISO 12640-4 (2011).
    pub const ADOBE_RGB: Self = Self(10);
}

impl Debug for WpColorManagerV1Primaries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SRGB => "SRGB",
            Self::PAL_M => "PAL_M",
            Self::PAL => "PAL",
            Self::NTSC => "NTSC",
            Self::GENERIC_FILM => "GENERIC_FILM",
            Self::BT2020 => "BT2020",
            Self::CIE1931_XYZ => "CIE1931_XYZ",
            Self::DCI_P3 => "DCI_P3",
            Self::DISPLAY_P3 => "DISPLAY_P3",
            Self::ADOBE_RGB => "ADOBE_RGB",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// named transfer functions
///
/// Named transfer functions used to represent well-known transfer
/// characteristics of displays.
///
/// A value of 0 is invalid and will never be present in the list of enums.
///
/// See appendix.md for the formulae.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorManagerV1TransferFunction(pub u32);

impl WpColorManagerV1TransferFunction {
    /// BT.1886 display transfer characteristic
    ///
    /// Rec. ITU-R BT.1886 is the display transfer characteristic assumed by
    /// - Rec. ITU-R BT.601-7 525 and 625
    /// - Rec. ITU-R BT.709-6
    /// - Rec. ITU-R BT.2020-2
    ///
    /// This TF implies these default luminances from Rec. ITU-R BT.2035:
    /// - primary color volume minimum: 0.01 cd/m²
    /// - primary color volume maximum: 100 cd/m²
    /// - reference white: 100 cd/m²
    pub const BT1886: Self = Self(1);

    /// Assumed display gamma 2.2 transfer function
    ///
    /// Transfer characteristics as defined by
    /// - Rec. ITU-R BT.470-6 System M (historical)
    /// - United States National Television System Committee 1953
    ///   Recommendation for transmission standards for color television
    /// - United States Federal Communications Commission (2003) Title 47 Code
    ///   of Federal Regulations 73.682 (a) (20)
    /// - Rec. ITU-R BT.1700-0 625 PAL and 625 SECAM
    /// - IEC 61966-2-1 (reference display)
    pub const GAMMA22: Self = Self(2);

    /// Assumed display gamma 2.8 transfer function
    ///
    /// Transfer characteristics as defined by
    /// - Rec. ITU-R BT.470-6 System B, G (historical)
    pub const GAMMA28: Self = Self(3);

    /// SMPTE ST 240 transfer function
    ///
    /// Transfer characteristics as defined by
    /// - SMPTE ST 240 (1999)
    pub const ST240: Self = Self(4);

    /// extended linear transfer function
    ///
    /// Linear transfer function defined over all real numbers.
    /// Normalised electrical values are equal the normalised optical values.
    pub const EXT_LINEAR: Self = Self(5);

    /// logarithmic 100:1 transfer function
    ///
    /// Logarithmic transfer characteristic (100:1 range).
    pub const LOG_100: Self = Self(6);

    /// logarithmic (100*Sqrt(10) : 1) transfer function
    ///
    /// Logarithmic transfer characteristic (100 * Sqrt(10) : 1 range).
    pub const LOG_316: Self = Self(7);

    /// IEC 61966-2-4 transfer function
    ///
    /// Transfer characteristics as defined by
    /// - IEC 61966-2-4
    pub const XVYCC: Self = Self(8);

    /// Deprecated (ambiguous sRGB transfer function)
    ///
    /// Transfer characteristics as defined by
    /// - IEC 61966-2-1 sRGB
    ///
    /// As a rule of thumb, use gamma22 for video, motion picture and
    /// computer graphics, or compound_power_2_4 for ICC calibrated print
    /// workflows.
    pub const SRGB: Self = Self(9);

    /// Deprecated (Extended sRGB piece-wise transfer function)
    ///
    /// Transfer characteristics as defined by
    /// - IEC 61966-2-1 sYCC
    pub const EXT_SRGB: Self = Self(10);

    /// perceptual quantizer transfer function
    ///
    /// Transfer characteristics as defined by
    /// - SMPTE ST 2084 (2014) for 10-, 12-, 14- and 16-bit systems
    /// - Rec. ITU-R BT.2100-2 perceptual quantization (PQ) system
    ///
    /// This TF implies these default luminances
    /// - primary color volume minimum: 0.005 cd/m²
    /// - primary color volume maximum: 10000 cd/m²
    /// - reference white: 203 cd/m²
    ///
    /// The difference between the primary color volume minimum and maximum
    /// must be approximately 10000 cd/m² as that is the swing of the EOTF
    /// defined by ST 2084 and BT.2100. The default value for the
    /// reference white is a protocol addition: it is suggested by
    /// Report ITU-R BT.2408-7 and is not part of ST 2084 or BT.2100.
    pub const ST2084_PQ: Self = Self(11);

    /// SMPTE ST 428 transfer function
    ///
    /// Transfer characteristics as defined by
    /// - SMPTE ST 428-1 (2019)
    pub const ST428: Self = Self(12);

    /// hybrid log-gamma transfer function
    ///
    /// Transfer characteristics as defined by
    /// - ARIB STD-B67 (2015)
    /// - Rec. ITU-R BT.2100-2 hybrid log-gamma (HLG) system
    ///
    /// This TF implies these default luminances
    /// - primary color volume minimum: 0.005 cd/m²
    /// - primary color volume maximum: 1000 cd/m²
    /// - reference white: 203 cd/m²
    ///
    /// HLG is a relative display-referred signal with a specified
    /// non-linear mapping to the display peak luminance (the HLG OOTF).
    /// All absolute luminance values used here for HLG assume a 1000 cd/m²
    /// peak display.
    ///
    /// The default value for the reference white is a protocol addition:
    /// it is suggested by Report ITU-R BT.2408-7 and is not part of
    /// ARIB STD-B67 or BT.2100.
    pub const HLG: Self = Self(13);

    /// IEC 61966-2-1 encoding function
    ///
    /// Encoding characteristics as defined by IEC 61966-2-1, for displays
    /// that invert the encoding function.
    pub const COMPOUND_POWER_2_4: Self = Self(14);
}

impl Debug for WpColorManagerV1TransferFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BT1886 => "BT1886",
            Self::GAMMA22 => "GAMMA22",
            Self::GAMMA28 => "GAMMA28",
            Self::ST240 => "ST240",
            Self::EXT_LINEAR => "EXT_LINEAR",
            Self::LOG_100 => "LOG_100",
            Self::LOG_316 => "LOG_316",
            Self::XVYCC => "XVYCC",
            Self::SRGB => "SRGB",
            Self::EXT_SRGB => "EXT_SRGB",
            Self::ST2084_PQ => "ST2084_PQ",
            Self::ST428 => "ST428",
            Self::HLG => "HLG",
            Self::COMPOUND_POWER_2_4 => "COMPOUND_POWER_2_4",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
