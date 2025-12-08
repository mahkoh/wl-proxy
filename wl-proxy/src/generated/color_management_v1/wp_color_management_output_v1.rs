//! output color properties
//!
//! A wp_color_management_output_v1 describes the color properties of an
//! output.
//!
//! The wp_color_management_output_v1 is associated with the wl_output global
//! underlying the wl_output object. Therefore the client destroying the
//! wl_output object has no impact, but the compositor removing the output
//! global makes the wp_color_management_output_v1 object inert.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_color_management_output_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpColorManagementOutputV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpColorManagementOutputV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpColorManagementOutputV1MessageHandler for DefaultMessageHandler { }

impl MetaWpColorManagementOutputV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpColorManagementOutputV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpColorManagementOutputV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpColorManagementOutputV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpColorManagementOutputV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpColorManagementOutputV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the color management output
    ///
    /// Destroy the color wp_color_management_output_v1 object. This does not
    /// affect any remaining protocol objects.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wp_color_management_output_v1#{}.destroy()", id);
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

    /// Since when the image_description_changed message is available.
    #[allow(dead_code)]
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
    pub fn send_image_description_changed(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wp_color_management_output_v1#{}.image_description_changed()", client.endpoint.id, id);
        let endpoint = &client.endpoint;
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
        Ok(())
    }

    /// Since when the get_image_description message is available.
    #[allow(dead_code)]
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
    #[inline]
    pub fn send_get_image_description(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("image_description", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        eprintln!("server      <= wp_color_management_output_v1#{}.get_image_description(image_description: wp_image_description_v1#{})", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [WpColorManagementOutputV1] proxies.
#[allow(dead_code)]
pub trait MetaWpColorManagementOutputV1MessageHandler {
    /// destroy the color management output
    ///
    /// Destroy the color wp_color_management_output_v1 object. This does not
    /// affect any remaining protocol objects.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpColorManagementOutputV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_output_v1.destroy message: {}", Report::new(e));
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
    fn image_description_changed(
        &mut self,
        _slf: &Rc<MetaWpColorManagementOutputV1>,
    ) {
        let res = _slf.send_image_description_changed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_output_v1.image_description_changed message: {}", Report::new(e));
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
    fn get_image_description(
        &mut self,
        _slf: &Rc<MetaWpColorManagementOutputV1>,
        image_description: &Rc<MetaWpImageDescriptionV1>,
    ) {
        let res = _slf.send_get_image_description(
            image_description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_output_v1.get_image_description message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpColorManagementOutputV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> wp_color_management_output_v1#{}.destroy()", client.endpoint.id, msg[0]);
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
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                eprintln!("client#{:04} -> wp_color_management_output_v1#{}.get_image_description(image_description: wp_image_description_v1#{})", client.endpoint.id, msg[0], arg0);
                let arg0_id = arg0;
                let arg0 = MetaWpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "image_description", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_image_description(&self, arg0);
                } else {
                    DefaultMessageHandler.get_image_description(&self, arg0);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("server      -> wp_color_management_output_v1#{}.image_description_changed()", msg[0]);
                if let Some(handler) = handler {
                    (**handler).image_description_changed(&self);
                } else {
                    DefaultMessageHandler.image_description_changed(&self);
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

