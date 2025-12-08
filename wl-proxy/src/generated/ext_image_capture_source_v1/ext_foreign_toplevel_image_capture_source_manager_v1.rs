//! image capture source manager for foreign toplevels
//!
//! A manager for creating image capture source objects for
//! ext_foreign_toplevel_handle_v1 objects.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_foreign_toplevel_image_capture_source_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtForeignToplevelImageCaptureSourceManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtForeignToplevelImageCaptureSourceManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtForeignToplevelImageCaptureSourceManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaExtForeignToplevelImageCaptureSourceManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaExtForeignToplevelImageCaptureSourceManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtForeignToplevelImageCaptureSourceManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtForeignToplevelImageCaptureSourceManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtForeignToplevelImageCaptureSourceManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtForeignToplevelImageCaptureSourceManagerV1 {
    /// Since when the create_source message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_SOURCE__SINCE: u32 = 1;

    /// create source object for foreign toplevel
    ///
    /// Creates a source object for a foreign toplevel handle. Images captured
    /// from this source will show the same content as the toplevel.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `toplevel_handle`:
    #[inline]
    pub fn send_create_source(
        &self,
        source: &Rc<MetaExtImageCaptureSourceV1>,
        toplevel_handle: &Rc<MetaExtForeignToplevelHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            source,
            toplevel_handle,
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
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object
    ///
    /// Destroys the manager. This request may be sent at any time by the client
    /// and objects created by the manager will remain valid after its
    /// destruction.
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ExtForeignToplevelImageCaptureSourceManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaExtForeignToplevelImageCaptureSourceManagerV1MessageHandler {
    /// create source object for foreign toplevel
    ///
    /// Creates a source object for a foreign toplevel handle. Images captured
    /// from this source will show the same content as the toplevel.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `toplevel_handle`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn create_source(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelImageCaptureSourceManagerV1>,
        source: &Rc<MetaExtImageCaptureSourceV1>,
        toplevel_handle: &Rc<MetaExtForeignToplevelHandleV1>,
    ) {
        let res = _slf.send_create_source(
            source,
            toplevel_handle,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_image_capture_source_manager_v1.create_source message: {}", Report::new(e));
        }
    }

    /// delete this object
    ///
    /// Destroys the manager. This request may be sent at any time by the client
    /// and objects created by the manager will remain valid after its
    /// destruction.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtForeignToplevelImageCaptureSourceManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_foreign_toplevel_image_capture_source_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtForeignToplevelImageCaptureSourceManagerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaExtImageCaptureSourceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaExtForeignToplevelHandleV1>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).create_source(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.create_source(&self, arg0, arg1);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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

