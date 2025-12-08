//! manager to inform clients and begin capturing
//!
//! This object is a manager which offers requests to start capturing from a
//! source.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_screencopy_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrScreencopyManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrScreencopyManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrScreencopyManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrScreencopyManagerV1 {
    pub const XML_VERSION: u32 = 3;
}

impl MetaZwlrScreencopyManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrScreencopyManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrScreencopyManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrScreencopyManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrScreencopyManagerV1 {
    /// Since when the capture_output message is available.
    #[allow(dead_code)]
    pub const MSG__CAPTURE_OUTPUT__SINCE: u32 = 1;

    /// capture an output
    ///
    /// Capture the next frame of an entire output.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `output`:
    #[inline]
    pub fn send_capture_output(
        &self,
        frame: &Rc<MetaZwlrScreencopyFrameV1>,
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
            return Err(ObjectError);
        };
        let arg2 = match arg2.server_obj_id.get() {
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
            arg1 as u32,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the capture_output_region message is available.
    #[allow(dead_code)]
    pub const MSG__CAPTURE_OUTPUT_REGION__SINCE: u32 = 1;

    /// capture an output's region
    ///
    /// Capture the next frame of an output's region.
    ///
    /// The region is given in output logical coordinates, see
    /// xdg_output.logical_size. The region will be clipped to the output's
    /// extents.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `output`:
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_capture_output_region(
        &self,
        frame: &Rc<MetaZwlrScreencopyFrameV1>,
        overlay_cursor: i32,
        output: &Rc<MetaWlOutput>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
        ) = (
            frame,
            overlay_cursor,
            output,
            x,
            y,
            width,
            height,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg2 = match arg2.server_obj_id.get() {
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
            1,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1 as u32,
            arg2,
            arg3 as u32,
            arg4 as u32,
            arg5 as u32,
            arg6 as u32,
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ZwlrScreencopyManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrScreencopyManagerV1MessageHandler {
    /// capture an output
    ///
    /// Capture the next frame of an entire output.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn capture_output(
        &mut self,
        _slf: &Rc<MetaZwlrScreencopyManagerV1>,
        frame: &Rc<MetaZwlrScreencopyFrameV1>,
        overlay_cursor: i32,
        output: &Rc<MetaWlOutput>,
    ) {
        let res = _slf.send_capture_output(
            frame,
            overlay_cursor,
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_manager_v1.capture_output message: {}", Report::new(e));
        }
    }

    /// capture an output's region
    ///
    /// Capture the next frame of an output's region.
    ///
    /// The region is given in output logical coordinates, see
    /// xdg_output.logical_size. The region will be clipped to the output's
    /// extents.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `output`:
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn capture_output_region(
        &mut self,
        _slf: &Rc<MetaZwlrScreencopyManagerV1>,
        frame: &Rc<MetaZwlrScreencopyFrameV1>,
        overlay_cursor: i32,
        output: &Rc<MetaWlOutput>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_capture_output_region(
            frame,
            overlay_cursor,
            output,
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_manager_v1.capture_output_region message: {}", Report::new(e));
        }
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrScreencopyManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_screencopy_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrScreencopyManagerV1 {
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
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwlrScreencopyFrameV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg2) = client.endpoint.lookup(arg2) else {
                    return Err(ObjectError);
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = arg1 as i32;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).capture_output(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.capture_output(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwlrScreencopyFrameV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg2) = client.endpoint.lookup(arg2) else {
                    return Err(ObjectError);
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = arg1 as i32;
                let arg2 = &arg2;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                let arg5 = arg5 as i32;
                let arg6 = arg6 as i32;
                if let Some(handler) = handler {
                    (**handler).capture_output_region(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                } else {
                    DefaultMessageHandler.capture_output_region(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                }
            }
            2 => {
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

