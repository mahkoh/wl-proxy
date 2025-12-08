//! content type object for a surface
//!
//! The content type object allows the compositor to optimize for the kind
//! of content shown on the surface. A compositor may for example use it to
//! set relevant drm properties like "content type".
//!
//! The client may request to switch to another content type at any time.
//! When the associated surface gets destroyed, this object becomes inert and
//! the client should destroy it.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_content_type_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpContentTypeV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpContentTypeV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpContentTypeV1MessageHandler for DefaultMessageHandler { }

impl MetaWpContentTypeV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpContentTypeV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpContentTypeV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpContentTypeV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpContentTypeV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpContentTypeV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
        eprintln!("server      <= wp_content_type_v1#{}.destroy()", id);
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

    /// Since when the set_content_type message is available.
    #[allow(dead_code)]
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
        content_type: MetaWpContentTypeV1Type,
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
        eprintln!("server      <= wp_content_type_v1#{}.set_content_type(content_type: {:?})", id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }
}

/// A message handler for [WpContentTypeV1] proxies.
#[allow(dead_code)]
pub trait MetaWpContentTypeV1MessageHandler {
    /// destroy the content type object
    ///
    /// Switch back to not specifying the content type of this surface. This is
    /// equivalent to setting the content type to none, including double
    /// buffering semantics. See set_content_type for details.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpContentTypeV1>,
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
        _slf: &Rc<MetaWpContentTypeV1>,
        content_type: MetaWpContentTypeV1Type,
    ) {
        let res = _slf.send_set_content_type(
            content_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_content_type_v1.set_content_type message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpContentTypeV1 {
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
                eprintln!("client#{:04} -> wp_content_type_v1#{}.destroy()", client.endpoint.id, msg[0]);
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
                let arg0 = MetaWpContentTypeV1Type(arg0);
                eprintln!("client#{:04} -> wp_content_type_v1#{}.set_content_type(content_type: {:?})", client.endpoint.id, msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).set_content_type(&self, arg0);
                } else {
                    DefaultMessageHandler.set_content_type(&self, arg0);
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

impl MetaWpContentTypeV1 {
    /// Since when the type.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_NONE__SINCE: u32 = 1;
    /// Since when the type.photo enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_PHOTO__SINCE: u32 = 1;
    /// Since when the type.video enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_VIDEO__SINCE: u32 = 1;
    /// Since when the type.game enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_GAME__SINCE: u32 = 1;
}

/// possible content types
///
/// These values describe the available content types for a surface.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpContentTypeV1Type(pub u32);

impl MetaWpContentTypeV1Type {
    /// no content type applies
    ///
    /// The content type none means that either the application has no data
    /// about the content type, or that the content doesn't fit into one of
    /// the other categories.
    #[allow(dead_code)]
    pub const NONE: Self = Self(0);

    /// photo content type
    ///
    /// The content type photo describes content derived from digital still
    /// pictures and may be presented with minimal processing.
    #[allow(dead_code)]
    pub const PHOTO: Self = Self(1);

    /// video content type
    ///
    /// The content type video describes a video or animation and may be
    /// presented with more accurate timing to avoid stutter. Where scaling
    /// is needed, scaling methods more appropriate for video may be used.
    #[allow(dead_code)]
    pub const VIDEO: Self = Self(2);

    /// game content type
    ///
    /// The content type game describes a running game. Its content may be
    /// presented with reduced latency.
    #[allow(dead_code)]
    pub const GAME: Self = Self(3);
}

impl Debug for MetaWpContentTypeV1Type {
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
