//! image capture source manager for outputs
//!
//! A manager for creating image capture source objects for wl_output objects.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_output_image_capture_source_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtOutputImageCaptureSourceManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtOutputImageCaptureSourceManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtOutputImageCaptureSourceManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaExtOutputImageCaptureSourceManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtOutputImageCaptureSourceManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtOutputImageCaptureSourceManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtOutputImageCaptureSourceManagerV1 {
    /// Since when the create_source message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_SOURCE__SINCE: u32 = 1;

    /// create source object for output
    ///
    /// Creates a source object for an output. Images captured from this source
    /// will show the same content as the output. Some elements may be omitted,
    /// such as cursors and overlays that have been marked as transparent to
    /// capturing.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `output`:
    #[inline]
    pub fn send_create_source(
        &self,
        source: &Rc<MetaExtImageCaptureSourceV1>,
        output: &Rc<MetaWlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            source,
            output,
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
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        Ok(())
    }
}

/// A message handler for [ExtOutputImageCaptureSourceManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaExtOutputImageCaptureSourceManagerV1MessageHandler {
    /// create source object for output
    ///
    /// Creates a source object for an output. Images captured from this source
    /// will show the same content as the output. Some elements may be omitted,
    /// such as cursors and overlays that have been marked as transparent to
    /// capturing.
    ///
    /// # Arguments
    ///
    /// - `source`:
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn create_source(
        &mut self,
        _slf: &Rc<MetaExtOutputImageCaptureSourceManagerV1>,
        source: &Rc<MetaExtImageCaptureSourceV1>,
        output: &Rc<MetaWlOutput>,
    ) {
        let res = _slf.send_create_source(
            source,
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_output_image_capture_source_manager_v1.create_source message: {}", Report::new(e));
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
        _slf: &Rc<MetaExtOutputImageCaptureSourceManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_output_image_capture_source_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtOutputImageCaptureSourceManagerV1 {
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
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
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

