use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A treeland_capture_session_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandCaptureSessionV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandCaptureSessionV1Handler>,
}

struct DefaultHandler;

impl TreelandCaptureSessionV1Handler for DefaultHandler { }

impl TreelandCaptureSessionV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "treeland_capture_session_v1";
}

impl TreelandCaptureSessionV1 {
    pub fn set_handler(&self, handler: impl TreelandCaptureSessionV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandCaptureSessionV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandCaptureSessionV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandCaptureSessionV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandCaptureSessionV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object
    ///
    /// Unreferences the frame. This request must be called as soon as it's no longer valid.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_capture_session_v1#{}.destroy()\n", id);
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

    /// Since when the start message is available.
    pub const MSG__START__SINCE: u32 = 1;

    /// start session
    ///
    /// Start session and keeps sending frame.
    #[inline]
    pub fn send_start(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_capture_session_v1#{}.start()\n", id);
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
        ]);
        Ok(())
    }

    /// Since when the frame_done message is available.
    pub const MSG__FRAME_DONE__SINCE: u32 = 1;

    /// Indicates current frame is done.
    ///
    /// This is the ACK to the current "ready" event. The next "frame" event will be sent only when current
    /// "ready" event is acknowledged. The timestamp should be the same as the one sent in "ready" event.
    /// If the frame has the "transient" flag, all objects sent before become invalid after this event.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_usec`:
    #[inline]
    pub fn send_frame_done(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_usec: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_usec,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_capture_session_v1#{}.frame_done(tv_sec_hi: {}, tv_sec_lo: {}, tv_usec: {})\n", id, arg0, arg1, arg2);
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
            2,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the frame message is available.
    pub const MSG__FRAME__SINCE: u32 = 1;

    /// supply the client with information about the frame
    ///
    /// Main event supplying the client with information about the frame. If the capture didn't fail, this event is always
    /// emitted first before any other events.
    /// When mask is provided, x and y should be offset relative to mask surface origin. Otherwise offset_x and offset_y should always
    /// be zero.
    ///
    /// # Arguments
    ///
    /// - `offset_x`: crop offset x
    /// - `offset_y`: crop offset y
    /// - `width`:
    /// - `height`:
    /// - `buffer_flags`:
    /// - `flags`:
    /// - `format`:
    /// - `mod_high`:
    /// - `mod_low`:
    /// - `num_objects`:
    #[inline]
    pub fn send_frame(
        &self,
        offset_x: i32,
        offset_y: i32,
        width: u32,
        height: u32,
        buffer_flags: u32,
        flags: TreelandCaptureSessionV1Flags,
        format: u32,
        mod_high: u32,
        mod_low: u32,
        num_objects: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
            arg7,
            arg8,
            arg9,
        ) = (
            offset_x,
            offset_y,
            width,
            height,
            buffer_flags,
            flags,
            format,
            mod_high,
            mod_low,
            num_objects,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_capture_session_v1#{}.frame(offset_x: {}, offset_y: {}, width: {}, height: {}, buffer_flags: {}, flags: {:?}, format: {}, mod_high: {}, mod_low: {}, num_objects: {})\n", client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
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
            arg0 as u32,
            arg1 as u32,
            arg2,
            arg3,
            arg4,
            arg5.0,
            arg6,
            arg7,
            arg8,
            arg9,
        ]);
        Ok(())
    }

    /// Since when the object message is available.
    pub const MSG__OBJECT__SINCE: u32 = 1;

    /// supply the client with object fd
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `fd`:
    /// - `size`:
    /// - `offset`:
    /// - `stride`:
    /// - `plane_index`:
    #[inline]
    pub fn send_object(
        &self,
        index: u32,
        fd: &Rc<OwnedFd>,
        size: u32,
        offset: u32,
        stride: u32,
        plane_index: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            index,
            fd,
            size,
            offset,
            stride,
            plane_index,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_capture_session_v1#{}.object(index: {}, fd: {}, size: {}, offset: {}, stride: {}, plane_index: {})\n", client.endpoint.id, id, arg0, arg1.as_raw_fd(), arg2, arg3, arg4, arg5);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg1.clone());
        fmt.words([
            id,
            1,
            arg0,
            arg2,
            arg3,
            arg4,
            arg5,
        ]);
        Ok(())
    }

    /// Since when the ready message is available.
    pub const MSG__READY__SINCE: u32 = 1;

    /// indicates frame is available for reading
    ///
    /// This event is sent as soon as the frame is presented, indicating it is available for reading. This event
    /// includes the time at which presentation happened at.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    #[inline]
    pub fn send_ready(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_capture_session_v1#{}.ready(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", client.endpoint.id, id, arg0, arg1, arg2);
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
            2,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the cancel message is available.
    pub const MSG__CANCEL__SINCE: u32 = 1;

    /// Notifies current frame is no longer valid.
    ///
    /// If the capture failed or if the frame is no longer valid after the "frame" event has been emitted, this
    /// event will be used to inform the client to scrap the frame.
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    pub fn send_cancel(
        &self,
        reason: TreelandCaptureSessionV1CancelReason,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            reason,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_capture_session_v1#{}.cancel(reason: {:?})\n", client.endpoint.id, id, arg0);
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
            3,
            arg0.0,
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandCaptureSessionV1] proxies.
pub trait TreelandCaptureSessionV1Handler: Any {
    /// delete this object
    ///
    /// Unreferences the frame. This request must be called as soon as it's no longer valid.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandCaptureSessionV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_session_v1.destroy message: {}", Report::new(e));
        }
    }

    /// start session
    ///
    /// Start session and keeps sending frame.
    #[inline]
    fn start(
        &mut self,
        _slf: &Rc<TreelandCaptureSessionV1>,
    ) {
        let res = _slf.send_start(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_session_v1.start message: {}", Report::new(e));
        }
    }

    /// Indicates current frame is done.
    ///
    /// This is the ACK to the current "ready" event. The next "frame" event will be sent only when current
    /// "ready" event is acknowledged. The timestamp should be the same as the one sent in "ready" event.
    /// If the frame has the "transient" flag, all objects sent before become invalid after this event.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_usec`:
    #[inline]
    fn frame_done(
        &mut self,
        _slf: &Rc<TreelandCaptureSessionV1>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_usec: u32,
    ) {
        let res = _slf.send_frame_done(
            tv_sec_hi,
            tv_sec_lo,
            tv_usec,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_session_v1.frame_done message: {}", Report::new(e));
        }
    }

    /// supply the client with information about the frame
    ///
    /// Main event supplying the client with information about the frame. If the capture didn't fail, this event is always
    /// emitted first before any other events.
    /// When mask is provided, x and y should be offset relative to mask surface origin. Otherwise offset_x and offset_y should always
    /// be zero.
    ///
    /// # Arguments
    ///
    /// - `offset_x`: crop offset x
    /// - `offset_y`: crop offset y
    /// - `width`:
    /// - `height`:
    /// - `buffer_flags`:
    /// - `flags`:
    /// - `format`:
    /// - `mod_high`:
    /// - `mod_low`:
    /// - `num_objects`:
    #[inline]
    fn frame(
        &mut self,
        _slf: &Rc<TreelandCaptureSessionV1>,
        offset_x: i32,
        offset_y: i32,
        width: u32,
        height: u32,
        buffer_flags: u32,
        flags: TreelandCaptureSessionV1Flags,
        format: u32,
        mod_high: u32,
        mod_low: u32,
        num_objects: u32,
    ) {
        let res = _slf.send_frame(
            offset_x,
            offset_y,
            width,
            height,
            buffer_flags,
            flags,
            format,
            mod_high,
            mod_low,
            num_objects,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_session_v1.frame message: {}", Report::new(e));
        }
    }

    /// supply the client with object fd
    ///
    /// # Arguments
    ///
    /// - `index`:
    /// - `fd`:
    /// - `size`:
    /// - `offset`:
    /// - `stride`:
    /// - `plane_index`:
    #[inline]
    fn object(
        &mut self,
        _slf: &Rc<TreelandCaptureSessionV1>,
        index: u32,
        fd: &Rc<OwnedFd>,
        size: u32,
        offset: u32,
        stride: u32,
        plane_index: u32,
    ) {
        let res = _slf.send_object(
            index,
            fd,
            size,
            offset,
            stride,
            plane_index,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_session_v1.object message: {}", Report::new(e));
        }
    }

    /// indicates frame is available for reading
    ///
    /// This event is sent as soon as the frame is presented, indicating it is available for reading. This event
    /// includes the time at which presentation happened at.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`:
    /// - `tv_sec_lo`:
    /// - `tv_nsec`:
    #[inline]
    fn ready(
        &mut self,
        _slf: &Rc<TreelandCaptureSessionV1>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
    ) {
        let res = _slf.send_ready(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_session_v1.ready message: {}", Report::new(e));
        }
    }

    /// Notifies current frame is no longer valid.
    ///
    /// If the capture failed or if the frame is no longer valid after the "frame" event has been emitted, this
    /// event will be used to inform the client to scrap the frame.
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    fn cancel(
        &mut self,
        _slf: &Rc<TreelandCaptureSessionV1>,
        reason: TreelandCaptureSessionV1CancelReason,
    ) {
        let res = _slf.send_cancel(
            reason,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_session_v1.cancel message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandCaptureSessionV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandCaptureSessionV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_capture_session_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_capture_session_v1#{}.start()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).start(&self);
                } else {
                    DefaultHandler.start(&self);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_capture_session_v1#{}.frame_done(tv_sec_hi: {}, tv_sec_lo: {}, tv_usec: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).frame_done(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.frame_done(&self, arg0, arg1, arg2);
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
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                    arg7,
                    arg8,
                    arg9,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 48));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg5 = TreelandCaptureSessionV1Flags(arg5);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_capture_session_v1#{}.frame(offset_x: {}, offset_y: {}, width: {}, height: {}, buffer_flags: {}, flags: {:?}, format: {}, mod_high: {}, mod_low: {}, num_objects: {})\n", msg[0], arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).frame(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                } else {
                    DefaultHandler.frame(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                }
            }
            1 => {
                let [
                    arg0,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg1 = &arg1;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_capture_session_v1#{}.object(index: {}, fd: {}, size: {}, offset: {}, stride: {}, plane_index: {})\n", msg[0], arg0, arg1.as_raw_fd(), arg2, arg3, arg4, arg5);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).object(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.object(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_capture_session_v1#{}.ready(tv_sec_hi: {}, tv_sec_lo: {}, tv_nsec: {})\n", msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).ready(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.ready(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = TreelandCaptureSessionV1CancelReason(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_capture_session_v1#{}.cancel(reason: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).cancel(&self, arg0);
                } else {
                    DefaultHandler.cancel(&self, arg0);
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
            1 => "start",
            2 => "frame_done",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "frame",
            1 => "object",
            2 => "ready",
            3 => "cancel",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for TreelandCaptureSessionV1 {
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

impl TreelandCaptureSessionV1 {
    /// Since when the cancel_reason.temporary enum variant is available.
    pub const ENM__CANCEL_REASON_TEMPORARY__SINCE: u32 = 1;
    /// Since when the cancel_reason.permanent enum variant is available.
    pub const ENM__CANCEL_REASON_PERMANENT__SINCE: u32 = 1;
    /// Since when the cancel_reason.resizing enum variant is available.
    pub const ENM__CANCEL_REASON_RESIZING__SINCE: u32 = 1;

    /// Since when the flags.transient enum variant is available.
    pub const ENM__FLAGS_TRANSIENT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandCaptureSessionV1CancelReason(pub u32);

impl TreelandCaptureSessionV1CancelReason {
    /// temporary error, source will produce more frames
    pub const TEMPORARY: Self = Self(0);

    /// fatal error, source will not produce frames
    pub const PERMANENT: Self = Self(1);

    /// temporary error, source will produce more frames
    pub const RESIZING: Self = Self(2);
}

impl Debug for TreelandCaptureSessionV1CancelReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TEMPORARY => "TEMPORARY",
            Self::PERMANENT => "PERMANENT",
            Self::RESIZING => "RESIZING",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct TreelandCaptureSessionV1Flags(pub u32);

/// An iterator over the set bits in a [TreelandCaptureSessionV1Flags].
///
/// You can construct this with the `IntoIterator` implementation of `TreelandCaptureSessionV1Flags`.
#[derive(Clone, Debug)]
pub struct TreelandCaptureSessionV1FlagsIter(pub u32);

impl TreelandCaptureSessionV1Flags {
    /// clients should copy frame before processing
    pub const TRANSIENT: Self = Self(0x1);
}

impl TreelandCaptureSessionV1Flags {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 0x1)
    }
}

impl Iterator for TreelandCaptureSessionV1FlagsIter {
    type Item = TreelandCaptureSessionV1Flags;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(TreelandCaptureSessionV1Flags(bit))
    }
}

impl IntoIterator for TreelandCaptureSessionV1Flags {
    type Item = TreelandCaptureSessionV1Flags;
    type IntoIter = TreelandCaptureSessionV1FlagsIter;

    fn into_iter(self) -> Self::IntoIter {
        TreelandCaptureSessionV1FlagsIter(self.0)
    }
}

impl BitAnd for TreelandCaptureSessionV1Flags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for TreelandCaptureSessionV1Flags {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for TreelandCaptureSessionV1Flags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for TreelandCaptureSessionV1Flags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for TreelandCaptureSessionV1Flags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for TreelandCaptureSessionV1Flags {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for TreelandCaptureSessionV1Flags {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for TreelandCaptureSessionV1Flags {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for TreelandCaptureSessionV1Flags {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for TreelandCaptureSessionV1Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 0x1 == 0x1 {
            v &= !0x1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("TRANSIENT")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}
