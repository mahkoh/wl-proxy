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

/// A wp_color_management_output_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpColorManagementOutputV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn WpColorManagementOutputV1Handler>,
}

struct DefaultHandler;

impl WpColorManagementOutputV1Handler for DefaultHandler { }

impl WpColorManagementOutputV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::WpColorManagementOutputV1;
    pub const INTERFACE_NAME: &str = "wp_color_management_output_v1";
}

impl WpColorManagementOutputV1 {
    pub fn set_handler(&self, handler: impl WpColorManagementOutputV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_output_v1#{}.destroy()\n", id);
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
    pub fn send_image_description_changed(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_color_management_output_v1#{}.image_description_changed()\n", client.endpoint.id, id);
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
        ]);
        Ok(())
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
    #[inline]
    pub fn send_get_image_description(
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_management_output_v1#{}.get_image_description(image_description: wp_image_description_v1#{})\n", id, arg0_id);
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
}

/// A message handler for [WpColorManagementOutputV1] proxies.
pub trait WpColorManagementOutputV1Handler: Any {
    /// destroy the color management output
    ///
    /// Destroy the color wp_color_management_output_v1 object. This does not
    /// affect any remaining protocol objects.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WpColorManagementOutputV1>,
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
        _slf: &Rc<WpColorManagementOutputV1>,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
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
        _slf: &Rc<WpColorManagementOutputV1>,
        image_description: &Rc<WpImageDescriptionV1>,
    ) {
        let res = _slf.send_get_image_description(
            image_description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_management_output_v1.get_image_description message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WpColorManagementOutputV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WpColorManagementOutputV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_output_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_management_output_v1#{}.get_image_description(image_description: wp_image_description_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = WpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "image_description", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_image_description(&self, arg0);
                } else {
                    DefaultHandler.get_image_description(&self, arg0);
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_color_management_output_v1#{}.image_description_changed()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).image_description_changed(&self);
                } else {
                    DefaultHandler.image_description_changed(&self);
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

impl Proxy for WpColorManagementOutputV1 {
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

