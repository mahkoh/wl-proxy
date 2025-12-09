//! opaque image capture source object
//!
//! The image capture source object is an opaque descriptor for a capturable
//! resource.  This resource may be any sort of entity from which an image
//! may be derived.
//!
//! Note, because ext_image_capture_source_v1 objects are created from multiple
//! independent factory interfaces, the ext_image_capture_source_v1 interface is
//! frozen at version 1.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_image_capture_source_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtImageCaptureSourceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtImageCaptureSourceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtImageCaptureSourceV1MessageHandler for DefaultMessageHandler { }

impl MetaExtImageCaptureSourceV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaExtImageCaptureSourceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtImageCaptureSourceV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaExtImageCaptureSourceV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaExtImageCaptureSourceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtImageCaptureSourceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtImageCaptureSourceV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object
    ///
    /// Destroys the image capture source. This request may be sent at any time
    /// by the client.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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
}

/// A message handler for [ExtImageCaptureSourceV1] proxies.
#[allow(dead_code)]
pub trait MetaExtImageCaptureSourceV1MessageHandler {
    /// delete this object
    ///
    /// Destroys the image capture source. This request may be sent at any time
    /// by the client.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtImageCaptureSourceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_capture_source_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtImageCaptureSourceV1 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

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
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

