//! manager to inform clients and begin capturing
//!
//! This object is a manager with which to start capturing from sources.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_export_dmabuf_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrExportDmabufManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrExportDmabufManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrExportDmabufManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrExportDmabufManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwlrExportDmabufManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrExportDmabufManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwlrExportDmabufManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwlrExportDmabufManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrExportDmabufManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrExportDmabufManagerV1 {
    /// Since when the capture_output message is available.
    #[allow(dead_code)]
    pub const MSG__CAPTURE_OUTPUT__SINCE: u32 = 1;

    /// capture a frame from an output
    ///
    /// Capture the next frame of an entire output.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: include custom client hardware cursor on top of the frame
    /// - `output`:
    #[inline]
    pub fn send_capture_output(
        &self,
        frame: &Rc<MetaZwlrExportDmabufFrameV1>,
        overlay_cursor: i32,
        output: &Rc<MetaWlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            frame,
            overlay_cursor,
            output,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("output")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("frame", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
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
            arg0_id,
            arg1 as u32,
            arg2_id,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ZwlrExportDmabufManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrExportDmabufManagerV1MessageHandler {
    /// capture a frame from an output
    ///
    /// Capture the next frame of an entire output.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: include custom client hardware cursor on top of the frame
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn capture_output(
        &mut self,
        _slf: &Rc<MetaZwlrExportDmabufManagerV1>,
        frame: &Rc<MetaZwlrExportDmabufFrameV1>,
        overlay_cursor: i32,
        output: &Rc<MetaWlOutput>,
    ) {
        let res = _slf.send_capture_output(
            frame,
            overlay_cursor,
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_export_dmabuf_manager_v1.capture_output message: {}", Report::new(e));
        }
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrExportDmabufManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_export_dmabuf_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrExportDmabufManagerV1 {
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
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg1 = arg1 as i32;
                let arg0_id = arg0;
                let arg0 = MetaZwlrExportDmabufFrameV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "frame", e))?;
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                };
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).capture_output(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.capture_output(&self, arg0, arg1, arg2);
                }
            }
            1 => {
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
            0 => "capture_output",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

