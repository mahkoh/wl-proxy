use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A treeland_capture_context_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandCaptureContextV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandCaptureContextV1Handler>,
}

struct DefaultHandler;

impl TreelandCaptureContextV1Handler for DefaultHandler { }

impl TreelandCaptureContextV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: &str = "treeland_capture_context_v1";
}

impl TreelandCaptureContextV1 {
    pub fn set_handler(&self, handler: impl TreelandCaptureContextV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandCaptureContextV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandCaptureContextV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandCaptureContextV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandCaptureContextV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete this object, used or not
    ///
    /// Destroys the context. This request can be sent at any time by the client.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_capture_context_v1#{}.destroy()\n", id);
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

    /// Since when the select_source message is available.
    pub const MSG__SELECT_SOURCE__SINCE: u32 = 1;

    /// select source interactively
    ///
    /// Selector is provided by compositor. Client can provide source hint to hint compositor
    /// to provide certain kinds of source.
    ///
    /// # Arguments
    ///
    /// - `source_hint`:
    /// - `freeze`: freeze compositing or not
    /// - `with_cursor`: whether source content contains cursor or not
    /// - `mask`: this mask is guaranteed to be at the top most
    #[inline]
    pub fn send_select_source(
        &self,
        source_hint: TreelandCaptureContextV1SourceType,
        freeze: u32,
        with_cursor: u32,
        mask: Option<&Rc<WlSurface>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            source_hint,
            freeze,
            with_cursor,
            mask,
        );
        let arg3 = arg3.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg3_id = match arg3 {
            None => 0,
            Some(arg3) => match arg3.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("mask")),
                Some(id) => id,
            },
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_capture_context_v1#{}.select_source(source_hint: {:?}, freeze: {}, with_cursor: {}, mask: wl_surface#{})\n", id, arg0, arg1, arg2, arg3_id);
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
            arg1,
            arg2,
            arg3_id,
        ]);
        Ok(())
    }

    /// Since when the source_ready message is available.
    pub const MSG__SOURCE_READY__SINCE: u32 = 1;

    /// notify client that source is ready
    ///
    /// This event supplies the client some information about the capture source, including
    /// the capture region relative to mask and source type.
    ///
    /// # Arguments
    ///
    /// - `region_x`: offset x of capture region relative to mask for capture contents
    /// - `region_y`: offset y of capture region relative to mask for capture contents
    /// - `region_width`: width of capture region
    /// - `region_height`: height of capture region
    /// - `source_type`: final capture source type
    #[inline]
    pub fn send_source_ready(
        &self,
        region_x: i32,
        region_y: i32,
        region_width: u32,
        region_height: u32,
        source_type: TreelandCaptureContextV1SourceType,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            region_x,
            region_y,
            region_width,
            region_height,
            source_type,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_capture_context_v1#{}.source_ready(region_x: {}, region_y: {}, region_width: {}, region_height: {}, source_type: {:?})\n", client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4);
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
            arg4.0,
        ]);
        Ok(())
    }

    /// Since when the source_failed message is available.
    pub const MSG__SOURCE_FAILED__SINCE: u32 = 1;

    /// notify client that source selection is failed
    ///
    /// There could a lot of reasons but the most common one is that selector is busy
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    pub fn send_source_failed(
        &self,
        reason: TreelandCaptureContextV1SourceFailure,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_capture_context_v1#{}.source_failed(reason: {:?})\n", client.endpoint.id, id, arg0);
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
            1,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the capture message is available.
    pub const MSG__CAPTURE__SINCE: u32 = 1;

    /// capture one frame
    ///
    /// This event can be called just once. A second call might result in a protocol error cause
    /// we just provide transient
    #[inline]
    pub fn send_capture(
        &self,
        frame: &Rc<TreelandCaptureFrameV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            frame,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("frame", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_capture_context_v1#{}.capture(frame: treeland_capture_frame_v1#{})\n", id, arg0_id);
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
            arg0_id,
        ]);
        Ok(())
    }

    /// Since when the create_session message is available.
    pub const MSG__CREATE_SESSION__SINCE: u32 = 1;

    /// create a persistent session for capturing
    ///
    /// Often used by a screen recorder.
    #[inline]
    pub fn send_create_session(
        &self,
        session: &Rc<TreelandCaptureSessionV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            session,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("session", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_capture_context_v1#{}.create_session(session: treeland_capture_session_v1#{})\n", id, arg0_id);
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
            3,
            arg0_id,
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandCaptureContextV1] proxies.
pub trait TreelandCaptureContextV1Handler: Any {
    /// delete this object, used or not
    ///
    /// Destroys the context. This request can be sent at any time by the client.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandCaptureContextV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_context_v1.destroy message: {}", Report::new(e));
        }
    }

    /// select source interactively
    ///
    /// Selector is provided by compositor. Client can provide source hint to hint compositor
    /// to provide certain kinds of source.
    ///
    /// # Arguments
    ///
    /// - `source_hint`:
    /// - `freeze`: freeze compositing or not
    /// - `with_cursor`: whether source content contains cursor or not
    /// - `mask`: this mask is guaranteed to be at the top most
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn select_source(
        &mut self,
        _slf: &Rc<TreelandCaptureContextV1>,
        source_hint: TreelandCaptureContextV1SourceType,
        freeze: u32,
        with_cursor: u32,
        mask: Option<&Rc<WlSurface>>,
    ) {
        let res = _slf.send_select_source(
            source_hint,
            freeze,
            with_cursor,
            mask,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_context_v1.select_source message: {}", Report::new(e));
        }
    }

    /// notify client that source is ready
    ///
    /// This event supplies the client some information about the capture source, including
    /// the capture region relative to mask and source type.
    ///
    /// # Arguments
    ///
    /// - `region_x`: offset x of capture region relative to mask for capture contents
    /// - `region_y`: offset y of capture region relative to mask for capture contents
    /// - `region_width`: width of capture region
    /// - `region_height`: height of capture region
    /// - `source_type`: final capture source type
    #[inline]
    fn source_ready(
        &mut self,
        _slf: &Rc<TreelandCaptureContextV1>,
        region_x: i32,
        region_y: i32,
        region_width: u32,
        region_height: u32,
        source_type: TreelandCaptureContextV1SourceType,
    ) {
        let res = _slf.send_source_ready(
            region_x,
            region_y,
            region_width,
            region_height,
            source_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_context_v1.source_ready message: {}", Report::new(e));
        }
    }

    /// notify client that source selection is failed
    ///
    /// There could a lot of reasons but the most common one is that selector is busy
    ///
    /// # Arguments
    ///
    /// - `reason`:
    #[inline]
    fn source_failed(
        &mut self,
        _slf: &Rc<TreelandCaptureContextV1>,
        reason: TreelandCaptureContextV1SourceFailure,
    ) {
        let res = _slf.send_source_failed(
            reason,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_context_v1.source_failed message: {}", Report::new(e));
        }
    }

    /// capture one frame
    ///
    /// This event can be called just once. A second call might result in a protocol error cause
    /// we just provide transient
    ///
    /// # Arguments
    ///
    /// - `frame`:
    #[inline]
    fn capture(
        &mut self,
        _slf: &Rc<TreelandCaptureContextV1>,
        frame: &Rc<TreelandCaptureFrameV1>,
    ) {
        let res = _slf.send_capture(
            frame,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_context_v1.capture message: {}", Report::new(e));
        }
    }

    /// create a persistent session for capturing
    ///
    /// Often used by a screen recorder.
    ///
    /// # Arguments
    ///
    /// - `session`:
    #[inline]
    fn create_session(
        &mut self,
        _slf: &Rc<TreelandCaptureContextV1>,
        session: &Rc<TreelandCaptureSessionV1>,
    ) {
        let res = _slf.send_create_session(
            session,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_capture_context_v1.create_session message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandCaptureContextV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandCaptureContextV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_capture_context_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
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
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                let arg0 = TreelandCaptureContextV1SourceType(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_capture_context_v1#{}.select_source(source_hint: {:?}, freeze: {}, with_cursor: {}, mask: wl_surface#{})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                let arg3 = if arg3 == 0 {
                    None
                } else {
                    let arg3_id = arg3;
                    let Some(arg3) = client.endpoint.lookup(arg3_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg3_id));
                    };
                    let Ok(arg3) = (arg3 as Rc<dyn Any>).downcast::<WlSurface>() else {
                        let o = client.endpoint.lookup(arg3_id).unwrap();
                        return Err(ObjectError::WrongObjectType("mask", o.core().interface, ProxyInterface::WlSurface));
                    };
                    Some(arg3)
                };
                let arg3 = arg3.as_ref();
                if let Some(handler) = handler {
                    (**handler).select_source(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.select_source(&self, arg0, arg1, arg2, arg3);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_capture_context_v1#{}.capture(frame: treeland_capture_frame_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = TreelandCaptureFrameV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "frame", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).capture(&self, arg0);
                } else {
                    DefaultHandler.capture(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_capture_context_v1#{}.create_session(session: treeland_capture_session_v1#{})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let arg0 = TreelandCaptureSessionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "session", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_session(&self, arg0);
                } else {
                    DefaultHandler.create_session(&self, arg0);
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
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 28));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg4 = TreelandCaptureContextV1SourceType(arg4);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_capture_context_v1#{}.source_ready(region_x: {}, region_y: {}, region_width: {}, region_height: {}, source_type: {:?})\n", msg[0], arg0, arg1, arg2, arg3, arg4);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).source_ready(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.source_ready(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = TreelandCaptureContextV1SourceFailure(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_capture_context_v1#{}.source_failed(reason: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).source_failed(&self, arg0);
                } else {
                    DefaultHandler.source_failed(&self, arg0);
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
            1 => "select_source",
            2 => "capture",
            3 => "create_session",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "source_ready",
            1 => "source_failed",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for TreelandCaptureContextV1 {
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

impl TreelandCaptureContextV1 {
    /// Since when the source_type.output enum variant is available.
    pub const ENM__SOURCE_TYPE_OUTPUT__SINCE: u32 = 1;
    /// Since when the source_type.window enum variant is available.
    pub const ENM__SOURCE_TYPE_WINDOW__SINCE: u32 = 1;
    /// Since when the source_type.region enum variant is available.
    pub const ENM__SOURCE_TYPE_REGION__SINCE: u32 = 1;

    /// Since when the source_failure.selector_busy enum variant is available.
    pub const ENM__SOURCE_FAILURE_SELECTOR_BUSY__SINCE: u32 = 1;
    /// Since when the source_failure.user_cancel enum variant is available.
    pub const ENM__SOURCE_FAILURE_USER_CANCEL__SINCE: u32 = 1;
    /// Since when the source_failure.source_destroyed enum variant is available.
    pub const ENM__SOURCE_FAILURE_SOURCE_DESTROYED__SINCE: u32 = 1;
    /// Since when the source_failure.other enum variant is available.
    pub const ENM__SOURCE_FAILURE_OTHER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
pub struct TreelandCaptureContextV1SourceType(pub u32);

/// An iterator over the set bits in a [TreelandCaptureContextV1SourceType].
///
/// You can construct this with the `IntoIterator` implementation of `TreelandCaptureContextV1SourceType`.
#[derive(Clone, Debug)]
pub struct TreelandCaptureContextV1SourceTypeIter(pub u32);

impl TreelandCaptureContextV1SourceType {
    /// output source type
    pub const OUTPUT: Self = Self(0x1);

    /// window source type
    pub const WINDOW: Self = Self(0x2);

    /// region source type
    pub const REGION: Self = Self(0x4);
}

impl TreelandCaptureContextV1SourceType {
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
        Self(0 | 0x1 | 0x2 | 0x4)
    }
}

impl Iterator for TreelandCaptureContextV1SourceTypeIter {
    type Item = TreelandCaptureContextV1SourceType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(TreelandCaptureContextV1SourceType(bit))
    }
}

impl IntoIterator for TreelandCaptureContextV1SourceType {
    type Item = TreelandCaptureContextV1SourceType;
    type IntoIter = TreelandCaptureContextV1SourceTypeIter;

    fn into_iter(self) -> Self::IntoIter {
        TreelandCaptureContextV1SourceTypeIter(self.0)
    }
}

impl BitAnd for TreelandCaptureContextV1SourceType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for TreelandCaptureContextV1SourceType {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for TreelandCaptureContextV1SourceType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for TreelandCaptureContextV1SourceType {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for TreelandCaptureContextV1SourceType {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for TreelandCaptureContextV1SourceType {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for TreelandCaptureContextV1SourceType {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for TreelandCaptureContextV1SourceType {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for TreelandCaptureContextV1SourceType {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for TreelandCaptureContextV1SourceType {
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
            f.write_str("OUTPUT")?;
        }
        if v & 0x2 == 0x2 {
            v &= !0x2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("WINDOW")?;
        }
        if v & 0x4 == 0x4 {
            v &= !0x4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("REGION")?;
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

/// source failure reason
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandCaptureContextV1SourceFailure(pub u32);

impl TreelandCaptureContextV1SourceFailure {
    /// selector is occupied by other context
    pub const SELECTOR_BUSY: Self = Self(1);

    /// User cancel this context from compositor
    pub const USER_CANCEL: Self = Self(2);

    /// Source has been destroyed
    pub const SOURCE_DESTROYED: Self = Self(3);

    /// other failure
    pub const OTHER: Self = Self(4);
}

impl Debug for TreelandCaptureContextV1SourceFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::SELECTOR_BUSY => "SELECTOR_BUSY",
            Self::USER_CANCEL => "USER_CANCEL",
            Self::SOURCE_DESTROYED => "SOURCE_DESTROYED",
            Self::OTHER => "OTHER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
