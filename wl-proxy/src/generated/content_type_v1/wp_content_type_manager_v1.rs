//! surface content type manager
//!
//! This interface allows a client to describe the kind of content a surface
//! will display, to allow the compositor to optimize its behavior for it.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_content_type_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpContentTypeManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpContentTypeManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpContentTypeManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaWpContentTypeManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpContentTypeManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpContentTypeManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpContentTypeManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the content type manager object
    ///
    /// Destroy the content type manager. This doesn't destroy objects created
    /// with the manager.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the get_surface_content_type message is available.
    #[allow(dead_code)]
    pub const MSG__GET_SURFACE_CONTENT_TYPE__SINCE: u32 = 1;

    /// create a new content type object
    ///
    /// Create a new content type object associated with the given surface.
    ///
    /// Creating a wp_content_type_v1 from a wl_surface which already has one
    /// attached is a client error: already_constructed.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_surface_content_type(
        &self,
        id: &Rc<MetaWpContentTypeV1>,
        surface: &Rc<MetaWlSurface>,
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
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }
}

/// A message handler for [WpContentTypeManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaWpContentTypeManagerV1MessageHandler {
    /// destroy the content type manager object
    ///
    /// Destroy the content type manager. This doesn't destroy objects created
    /// with the manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpContentTypeManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_content_type_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a new content type object
    ///
    /// Create a new content type object associated with the given surface.
    ///
    /// Creating a wp_content_type_v1 from a wl_surface which already has one
    /// attached is a client error: already_constructed.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_surface_content_type(
        &mut self,
        _slf: &Rc<MetaWpContentTypeManagerV1>,
        id: &Rc<MetaWpContentTypeV1>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_get_surface_content_type(
            id,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_content_type_manager_v1.get_surface_content_type message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpContentTypeManagerV1 {
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
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpContentTypeV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_surface_content_type(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_surface_content_type(&self, arg0, arg1);
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
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

impl MetaWpContentTypeManagerV1 {
    /// Since when the error.already_constructed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_CONSTRUCTED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpContentTypeManagerV1Error(pub u32);

impl MetaWpContentTypeManagerV1Error {
    /// wl_surface already has a content type object
    #[allow(dead_code)]
    pub const ALREADY_CONSTRUCTED: Self = Self(0);
}

impl Debug for MetaWpContentTypeManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALREADY_CONSTRUCTED => "ALREADY_CONSTRUCTED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
