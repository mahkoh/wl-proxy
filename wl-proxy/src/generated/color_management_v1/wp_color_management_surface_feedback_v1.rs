//! color management extension to a surface
//!
//! A wp_color_management_surface_feedback_v1 allows the client to get the
//! preferred image description of a surface.
//!
//! If the wl_surface associated with this object is destroyed, the
//! wp_color_management_surface_feedback_v1 object becomes inert.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_color_management_surface_feedback_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpColorManagementSurfaceFeedbackV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpColorManagementSurfaceFeedbackV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpColorManagementSurfaceFeedbackV1MessageHandler for DefaultMessageHandler { }

impl MetaWpColorManagementSurfaceFeedbackV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpColorManagementSurfaceFeedbackV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpColorManagementSurfaceFeedbackV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpColorManagementSurfaceFeedbackV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpColorManagementSurfaceFeedbackV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpColorManagementSurfaceFeedbackV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the color management interface for a surface
    ///
    /// Destroy the wp_color_management_surface_feedback_v1 object.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
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

    /// Since when the preferred_changed message is available.
    #[allow(dead_code)]
    pub const MSG__PREFERRED_CHANGED__SINCE: u32 = 1;

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
    /// - `identity`: image description id number
    #[inline]
    pub fn send_preferred_changed(
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
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the get_preferred message is available.
    #[allow(dead_code)]
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
    #[inline]
    pub fn send_get_preferred(
        &self,
        image_description: &Rc<MetaWpImageDescriptionV1>,
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
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
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
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the get_preferred_parametric message is available.
    #[allow(dead_code)]
    pub const MSG__GET_PREFERRED_PARAMETRIC__SINCE: u32 = 1;

    /// get the preferred image description
    ///
    /// The same description as for get_preferred applies, except the returned
    /// image description is guaranteed to be parametric. This is meant for
    /// clients that can only deal with parametric image descriptions.
    ///
    /// If the compositor doesn't support parametric image descriptions, the
    /// unsupported_feature error is emitted.
    #[inline]
    pub fn send_get_preferred_parametric(
        &self,
        image_description: &Rc<MetaWpImageDescriptionV1>,
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
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }
}

/// A message handler for [WpColorManagementSurfaceFeedbackV1] proxies.
#[allow(dead_code)]
pub trait MetaWpColorManagementSurfaceFeedbackV1MessageHandler {
    /// destroy the color management interface for a surface
    ///
    /// Destroy the wp_color_management_surface_feedback_v1 object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpColorManagementSurfaceFeedbackV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_surface_feedback_v1.destroy message: {}", Report::new(e));
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
    /// - `identity`: image description id number
    #[inline]
    fn preferred_changed(
        &mut self,
        _slf: &Rc<MetaWpColorManagementSurfaceFeedbackV1>,
        identity: u32,
    ) {
        let res = _slf.send_preferred_changed(
            identity,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_surface_feedback_v1.preferred_changed message: {}", Report::new(e));
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
    fn get_preferred(
        &mut self,
        _slf: &Rc<MetaWpColorManagementSurfaceFeedbackV1>,
        image_description: &Rc<MetaWpImageDescriptionV1>,
    ) {
        let res = _slf.send_get_preferred(
            image_description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_surface_feedback_v1.get_preferred message: {}", Report::new(e));
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
    fn get_preferred_parametric(
        &mut self,
        _slf: &Rc<MetaWpColorManagementSurfaceFeedbackV1>,
        image_description: &Rc<MetaWpImageDescriptionV1>,
    ) {
        let res = _slf.send_get_preferred_parametric(
            image_description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_surface_feedback_v1.get_preferred_parametric message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpColorManagementSurfaceFeedbackV1 {
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
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_preferred(&self, arg0);
                } else {
                    DefaultMessageHandler.get_preferred(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_preferred_parametric(&self, arg0);
                } else {
                    DefaultMessageHandler.get_preferred_parametric(&self, arg0);
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
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).preferred_changed(&self, arg0);
                } else {
                    DefaultMessageHandler.preferred_changed(&self, arg0);
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

impl MetaWpColorManagementSurfaceFeedbackV1 {
    /// Since when the error.inert enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INERT__SINCE: u32 = 1;
    /// Since when the error.unsupported_feature enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_UNSUPPORTED_FEATURE__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpColorManagementSurfaceFeedbackV1Error(pub u32);

impl MetaWpColorManagementSurfaceFeedbackV1Error {
    /// forbidden request on inert object
    #[allow(dead_code)]
    pub const INERT: Self = Self(0);

    /// attempted to use an unsupported feature
    #[allow(dead_code)]
    pub const UNSUPPORTED_FEATURE: Self = Self(1);
}

impl Debug for MetaWpColorManagementSurfaceFeedbackV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INERT => "INERT",
            Self::UNSUPPORTED_FEATURE => "UNSUPPORTED_FEATURE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
