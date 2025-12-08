//! content type object for a surface
//!
//! The content type object allows the compositor to optimize for the kind
//! of content shown on the surface. A compositor may for example use it to
//! set relevant drm properties like "content type".
//!
//! The client may request to switch to another content type at any time.
//! When the associated surface gets destroyed, this object becomes inert and
//! the client should destroy it.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_content_type_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpContentTypeV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpContentTypeV1Handler>,
}

struct DefaultHandler;

impl WpContentTypeV1Handler for DefaultHandler { }

impl ConcreteObject for WpContentTypeV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpContentTypeV1;
    const INTERFACE_NAME: &str = "wp_content_type_v1";
}

impl WpContentTypeV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpContentTypeV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpContentTypeV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpContentTypeV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpContentTypeV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpContentTypeV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the content type object
    ///
    /// Switch back to not specifying the content type of this surface. This is
    /// equivalent to setting the content type to none, including double
    /// buffering semantics. See set_content_type for details.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_content_type_v1#{}.destroy()\n", id);
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

    /// destroy the content type object
    ///
    /// Switch back to not specifying the content type of this surface. This is
    /// equivalent to setting the content type to none, including double
    /// buffering semantics. See set_content_type for details.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_content_type_v1.destroy", &e);
        }
    }

    /// Since when the set_content_type message is available.
    pub const MSG__SET_CONTENT_TYPE__SINCE: u32 = 1;

    /// specify the content type
    ///
    /// Set the surface content type. This informs the compositor that the
    /// client believes it is displaying buffers matching this content type.
    ///
    /// This is purely a hint for the compositor, which can be used to adjust
    /// its behavior or hardware settings to fit the presented content best.
    ///
    /// The content type is double-buffered state, see wl_surface.commit for
    /// details.
    ///
    /// # Arguments
    ///
    /// - `content_type`: the content type
    #[inline]
    pub fn try_send_set_content_type(
        &self,
        content_type: WpContentTypeV1Type,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            content_type,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WpContentTypeV1Type) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_content_type_v1#{}.set_content_type(content_type: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// specify the content type
    ///
    /// Set the surface content type. This informs the compositor that the
    /// client believes it is displaying buffers matching this content type.
    ///
    /// This is purely a hint for the compositor, which can be used to adjust
    /// its behavior or hardware settings to fit the presented content best.
    ///
    /// The content type is double-buffered state, see wl_surface.commit for
    /// details.
    ///
    /// # Arguments
    ///
    /// - `content_type`: the content type
    #[inline]
    pub fn send_set_content_type(
        &self,
        content_type: WpContentTypeV1Type,
    ) {
        let res = self.try_send_set_content_type(
            content_type,
        );
        if let Err(e) = res {
            log_send("wp_content_type_v1.set_content_type", &e);
        }
    }
}

/// A message handler for [`WpContentTypeV1`] proxies.
pub trait WpContentTypeV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpContentTypeV1>) {
        slf.core.delete_id();
    }

    /// destroy the content type object
    ///
    /// Switch back to not specifying the content type of this surface. This is
    /// equivalent to setting the content type to none, including double
    /// buffering semantics. See set_content_type for details.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpContentTypeV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_content_type_v1.destroy", &e);
        }
    }

    /// specify the content type
    ///
    /// Set the surface content type. This informs the compositor that the
    /// client believes it is displaying buffers matching this content type.
    ///
    /// This is purely a hint for the compositor, which can be used to adjust
    /// its behavior or hardware settings to fit the presented content best.
    ///
    /// The content type is double-buffered state, see wl_surface.commit for
    /// details.
    ///
    /// # Arguments
    ///
    /// - `content_type`: the content type
    #[inline]
    fn handle_set_content_type(
        &mut self,
        slf: &Rc<WpContentTypeV1>,
        content_type: WpContentTypeV1Type,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_content_type(
            content_type,
        );
        if let Err(e) = res {
            log_forward("wp_content_type_v1.set_content_type", &e);
        }
    }
}

impl ObjectPrivate for WpContentTypeV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpContentTypeV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_content_type_v1#{}.destroy()\n", client_id, id);
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
                let arg0 = WpContentTypeV1Type(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WpContentTypeV1Type) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_content_type_v1#{}.set_content_type(content_type: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_content_type(&self, arg0);
                } else {
                    DefaultHandler.handle_set_content_type(&self, arg0);
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
            1 => "set_content_type",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpContentTypeV1 {
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

impl WpContentTypeV1 {
    /// Since when the type.none enum variant is available.
    pub const ENM__TYPE_NONE__SINCE: u32 = 1;
    /// Since when the type.photo enum variant is available.
    pub const ENM__TYPE_PHOTO__SINCE: u32 = 1;
    /// Since when the type.video enum variant is available.
    pub const ENM__TYPE_VIDEO__SINCE: u32 = 1;
    /// Since when the type.game enum variant is available.
    pub const ENM__TYPE_GAME__SINCE: u32 = 1;
}

/// possible content types
///
/// These values describe the available content types for a surface.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpContentTypeV1Type(pub u32);

impl WpContentTypeV1Type {
    /// no content type applies
    ///
    /// The content type none means that either the application has no data
    /// about the content type, or that the content doesn't fit into one of
    /// the other categories.
    pub const NONE: Self = Self(0);

    /// photo content type
    ///
    /// The content type photo describes content derived from digital still
    /// pictures and may be presented with minimal processing.
    pub const PHOTO: Self = Self(1);

    /// video content type
    ///
    /// The content type video describes a video or animation and may be
    /// presented with more accurate timing to avoid stutter. Where scaling
    /// is needed, scaling methods more appropriate for video may be used.
    pub const VIDEO: Self = Self(2);

    /// game content type
    ///
    /// The content type game describes a running game. Its content may be
    /// presented with reduced latency.
    pub const GAME: Self = Self(3);
}

impl Debug for WpContentTypeV1Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::PHOTO => "PHOTO",
            Self::VIDEO => "VIDEO",
            Self::GAME => "GAME",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
