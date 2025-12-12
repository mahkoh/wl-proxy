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

/// A wp_content_type_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpContentTypeV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn WpContentTypeV1Handler>,
}

struct DefaultHandler;

impl WpContentTypeV1Handler for DefaultHandler { }

impl WpContentTypeV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::WpContentTypeV1;
    pub const INTERFACE_NAME: &str = "wp_content_type_v1";
}

impl WpContentTypeV1 {
    pub fn set_handler(&self, handler: impl WpContentTypeV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_content_type_v1#{}.destroy()\n", id);
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
    pub fn send_set_content_type(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_content_type_v1#{}.set_content_type(content_type: {:?})\n", id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }
}

/// A message handler for [WpContentTypeV1] proxies.
pub trait WpContentTypeV1Handler: Any {
    /// destroy the content type object
    ///
    /// Switch back to not specifying the content type of this surface. This is
    /// equivalent to setting the content type to none, including double
    /// buffering semantics. See set_content_type for details.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<WpContentTypeV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_content_type_v1.destroy message: {}", Report::new(e));
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
    fn set_content_type(
        &mut self,
        _slf: &Rc<WpContentTypeV1>,
        content_type: WpContentTypeV1Type,
    ) {
        let res = _slf.send_set_content_type(
            content_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_content_type_v1.set_content_type message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for WpContentTypeV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::WpContentTypeV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_content_type_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                let arg0 = WpContentTypeV1Type(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_content_type_v1#{}.set_content_type(content_type: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).set_content_type(&self, arg0);
                } else {
                    DefaultHandler.set_content_type(&self, arg0);
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

impl Proxy for WpContentTypeV1 {
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
