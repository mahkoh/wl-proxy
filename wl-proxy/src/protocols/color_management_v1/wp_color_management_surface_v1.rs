//! color management extension to a surface
//!
//! A wp_color_management_surface_v1 allows the client to set the color
//! space and HDR properties of a surface.
//!
//! If the wl_surface associated with the wp_color_management_surface_v1 is
//! destroyed, the wp_color_management_surface_v1 object becomes inert.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_color_management_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpColorManagementSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpColorManagementSurfaceV1Handler>,
}

struct DefaultHandler;

impl WpColorManagementSurfaceV1Handler for DefaultHandler { }

impl WpColorManagementSurfaceV1 {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: ObjectInterface = ObjectInterface::WpColorManagementSurfaceV1;
    pub const INTERFACE_NAME: &str = "wp_color_management_surface_v1";
}

impl WpColorManagementSurfaceV1 {
    pub fn set_handler(&self, handler: impl WpColorManagementSurfaceV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn WpColorManagementSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpColorManagementSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpColorManagementSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpColorManagementSurfaceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the color management interface for a surface
    ///
    /// Destroy the wp_color_management_surface_v1 object and do the same as
    /// unset_image_description.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_surface_v1#{}.destroy()\n", id);
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

    /// Since when the set_image_description message is available.
    pub const MSG__SET_IMAGE_DESCRIPTION__SINCE: u32 = 1;

    /// set the surface image description
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Set the image description of the underlying surface. The image
    /// description and rendering intent are double-buffered state, see
    /// wl_surface.commit.
    ///
    /// It is the client's responsibility to understand the image description
    /// it sets on a surface, and to provide content that matches that image
    /// description. Compositors might convert images to match their own or any
    /// other image descriptions.
    ///
    /// Image descriptions which are not ready (see wp_image_description_v1)
    /// are forbidden in this request, and in such case the protocol error
    /// image_description is raised.
    ///
    /// All image descriptions which are ready (see wp_image_description_v1)
    /// are allowed and must always be accepted by the compositor.
    ///
    /// When an image description is set on a surface, it establishes an
    /// explicit link between surface pixel values and surface colorimetry.
    /// This link may be undefined for some pixel values, see the image
    /// description creator interfaces for the conditions. Non-finite
    /// floating-point values (NaN, Inf) always have an undefined colorimetry.
    ///
    /// A rendering intent provides the client's preference on how surface
    /// colorimetry should be mapped to each output. The render_intent value
    /// must be one advertised by the compositor with
    /// wp_color_manager_v1.render_intent event, otherwise the protocol error
    /// render_intent is raised.
    ///
    /// By default, a surface does not have an associated image description
    /// nor a rendering intent. The handling of color on such surfaces is
    /// compositor implementation defined. Compositors should handle such
    /// surfaces as sRGB, but may handle them differently if they have specific
    /// requirements.
    ///
    /// Setting the image description has copy semantics; after this request,
    /// the image description can be immediately destroyed without affecting
    /// the pending state of the surface.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    /// - `render_intent`: rendering intent
    #[inline]
    pub fn send_set_image_description(
        &self,
        image_description: &Rc<WpImageDescriptionV1>,
        render_intent: WpColorManagerV1RenderIntent,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            image_description,
            render_intent,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("image_description")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_surface_v1#{}.set_image_description(image_description: wp_image_description_v1#{}, render_intent: {:?})\n", id, arg0_id, arg1);
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
            arg1.0,
        ]);
        Ok(())
    }

    /// Since when the unset_image_description message is available.
    pub const MSG__UNSET_IMAGE_DESCRIPTION__SINCE: u32 = 1;

    /// remove the surface image description
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// This request removes any image description from the surface. See
    /// set_image_description for how a compositor handles a surface without
    /// an image description. This is double-buffered state, see
    /// wl_surface.commit.
    #[inline]
    pub fn send_unset_image_description(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_surface_v1#{}.unset_image_description()\n", id);
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
        ]);
        Ok(())
    }
}

/// A message handler for [WpColorManagementSurfaceV1] proxies.
pub trait WpColorManagementSurfaceV1Handler: Any {
    /// destroy the color management interface for a surface
    ///
    /// Destroy the wp_color_management_surface_v1 object and do the same as
    /// unset_image_description.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WpColorManagementSurfaceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_surface_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set the surface image description
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Set the image description of the underlying surface. The image
    /// description and rendering intent are double-buffered state, see
    /// wl_surface.commit.
    ///
    /// It is the client's responsibility to understand the image description
    /// it sets on a surface, and to provide content that matches that image
    /// description. Compositors might convert images to match their own or any
    /// other image descriptions.
    ///
    /// Image descriptions which are not ready (see wp_image_description_v1)
    /// are forbidden in this request, and in such case the protocol error
    /// image_description is raised.
    ///
    /// All image descriptions which are ready (see wp_image_description_v1)
    /// are allowed and must always be accepted by the compositor.
    ///
    /// When an image description is set on a surface, it establishes an
    /// explicit link between surface pixel values and surface colorimetry.
    /// This link may be undefined for some pixel values, see the image
    /// description creator interfaces for the conditions. Non-finite
    /// floating-point values (NaN, Inf) always have an undefined colorimetry.
    ///
    /// A rendering intent provides the client's preference on how surface
    /// colorimetry should be mapped to each output. The render_intent value
    /// must be one advertised by the compositor with
    /// wp_color_manager_v1.render_intent event, otherwise the protocol error
    /// render_intent is raised.
    ///
    /// By default, a surface does not have an associated image description
    /// nor a rendering intent. The handling of color on such surfaces is
    /// compositor implementation defined. Compositors should handle such
    /// surfaces as sRGB, but may handle them differently if they have specific
    /// requirements.
    ///
    /// Setting the image description has copy semantics; after this request,
    /// the image description can be immediately destroyed without affecting
    /// the pending state of the surface.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    /// - `render_intent`: rendering intent
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_image_description(
        &mut self,
        _slf: &Rc<WpColorManagementSurfaceV1>,
        image_description: &Rc<WpImageDescriptionV1>,
        render_intent: WpColorManagerV1RenderIntent,
    ) {
        let res = _slf.send_set_image_description(
            image_description,
            render_intent,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_surface_v1.set_image_description message: {}", Report::new(e));
        }
    }

    /// remove the surface image description
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// This request removes any image description from the surface. See
    /// set_image_description for how a compositor handles a surface without
    /// an image description. This is double-buffered state, see
    /// wl_surface.commit.
    #[inline]
    fn unset_image_description(
        &mut self,
        _slf: &Rc<WpColorManagementSurfaceV1>,
    ) {
        let res = _slf.send_unset_image_description(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_surface_v1.unset_image_description message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for WpColorManagementSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpColorManagementSurfaceV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_surface_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg1 = WpColorManagerV1RenderIntent(arg1);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_surface_v1#{}.set_image_description(image_description: wp_image_description_v1#{}, render_intent: {:?})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WpImageDescriptionV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("image_description", o.core().interface, ObjectInterface::WpImageDescriptionV1));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_image_description(&self, arg0, arg1);
                } else {
                    DefaultHandler.set_image_description(&self, arg0, arg1);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_surface_v1#{}.unset_image_description()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).unset_image_description(&self);
                } else {
                    DefaultHandler.unset_image_description(&self);
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
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "set_image_description",
            2 => "unset_image_description",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpColorManagementSurfaceV1 {
    fn core(&self) -> &ObjectCore {
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

impl WpColorManagementSurfaceV1 {
    /// Since when the error.render_intent enum variant is available.
    pub const ENM__ERROR_RENDER_INTENT__SINCE: u32 = 1;
    /// Since when the error.image_description enum variant is available.
    pub const ENM__ERROR_IMAGE_DESCRIPTION__SINCE: u32 = 1;
    /// Since when the error.inert enum variant is available.
    pub const ENM__ERROR_INERT__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorManagementSurfaceV1Error(pub u32);

impl WpColorManagementSurfaceV1Error {
    /// unsupported rendering intent
    pub const RENDER_INTENT: Self = Self(0);

    /// invalid image description
    pub const IMAGE_DESCRIPTION: Self = Self(1);

    /// forbidden request on inert object
    pub const INERT: Self = Self(2);
}

impl Debug for WpColorManagementSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RENDER_INTENT => "RENDER_INTENT",
            Self::IMAGE_DESCRIPTION => "IMAGE_DESCRIPTION",
            Self::INERT => "INERT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
