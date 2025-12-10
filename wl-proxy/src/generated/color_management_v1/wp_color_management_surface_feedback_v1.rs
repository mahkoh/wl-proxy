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
pub struct WpColorManagementSurfaceFeedbackV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn WpColorManagementSurfaceFeedbackV1Handler>,
}

struct DefaultHandler;

impl WpColorManagementSurfaceFeedbackV1Handler for DefaultHandler { }

impl WpColorManagementSurfaceFeedbackV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "wp_color_management_surface_feedback_v1";
}

impl WpColorManagementSurfaceFeedbackV1 {
    pub fn set_handler(&self, handler: impl WpColorManagementSurfaceFeedbackV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_surface_feedback_v1#{}.destroy()\n", id);
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_management_surface_feedback_v1#{}.preferred_changed(identity: {})\n", client.endpoint.id, id, arg0);
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("image_description", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_surface_feedback_v1#{}.get_preferred(image_description: wp_image_description_v1#{})\n", id, arg0_id);
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("image_description", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_surface_feedback_v1#{}.get_preferred_parametric(image_description: wp_image_description_v1#{})\n", id, arg0_id);
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
            2,
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [WpColorManagementSurfaceFeedbackV1] proxies.
#[allow(dead_code)]
pub trait WpColorManagementSurfaceFeedbackV1Handler: Any {
    /// destroy the color management interface for a surface
    ///
    /// Destroy the wp_color_management_surface_feedback_v1 object.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WpColorManagementSurfaceFeedbackV1>,
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
        _slf: &Rc<WpColorManagementSurfaceFeedbackV1>,
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
        _slf: &Rc<WpColorManagementSurfaceFeedbackV1>,
        image_description: &Rc<WpImageDescriptionV1>,
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
        _slf: &Rc<WpColorManagementSurfaceFeedbackV1>,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        let res = _slf.send_get_preferred_parametric(
            image_description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_surface_feedback_v1.get_preferred_parametric message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WpColorManagementSurfaceFeedbackV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WpColorManagementSurfaceFeedbackV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_surface_feedback_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_surface_feedback_v1#{}.get_preferred(image_description: wp_image_description_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "image_description", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_preferred(&self, arg0);
                } else {
                    DefaultHandler.get_preferred(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_surface_feedback_v1#{}.get_preferred_parametric(image_description: wp_image_description_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "image_description", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_preferred_parametric(&self, arg0);
                } else {
                    DefaultHandler.get_preferred_parametric(&self, arg0);
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_management_surface_feedback_v1#{}.preferred_changed(identity: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).preferred_changed(&self, arg0);
                } else {
                    DefaultHandler.preferred_changed(&self, arg0);
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
            1 => "get_preferred",
            2 => "get_preferred_parametric",
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
}

impl Proxy for WpColorManagementSurfaceFeedbackV1 {
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

impl WpColorManagementSurfaceFeedbackV1 {
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
pub struct WpColorManagementSurfaceFeedbackV1Error(pub u32);

impl WpColorManagementSurfaceFeedbackV1Error {
    /// forbidden request on inert object
    #[allow(dead_code)]
    pub const INERT: Self = Self(0);

    /// attempted to use an unsupported feature
    #[allow(dead_code)]
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
