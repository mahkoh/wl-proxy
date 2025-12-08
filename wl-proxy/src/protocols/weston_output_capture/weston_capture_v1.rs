//! image capture factory
//!
//! The global interface exposing Weston screenshooting functionality
//! intended for single shots.
//!
//! This is a privileged inteface.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_capture_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonCaptureV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonCaptureV1Handler>,
}

struct DefaultHandler;

impl WestonCaptureV1Handler for DefaultHandler { }

impl ConcreteObject for WestonCaptureV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonCaptureV1;
    const INTERFACE_NAME: &str = "weston_capture_v1";
}

impl WestonCaptureV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonCaptureV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonCaptureV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonCaptureV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonCaptureV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonCaptureV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unbind image capture factory
    ///
    /// Affects no other protocol objects in any way.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_capture_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
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

    /// unbind image capture factory
    ///
    /// Affects no other protocol objects in any way.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("weston_capture_v1.destroy", &e);
        }
    }

    /// Since when the create message is available.
    pub const MSG__CREATE__SINCE: u32 = 1;

    /// create an object for capturing output images
    ///
    /// This creates a weston_capture_source_v1 object corresponding to the
    /// given wl_output. The object delivers information for allocating
    /// suitable buffers, and exposes the capture function.
    ///
    /// The object will be using the given pixel source for capturing images.
    /// If the source is not available, all attempts to capture will fail
    /// gracefully.
    ///
    /// 'writeback' source will use hardware writeback feature of DRM KMS for
    /// capturing. This may allow hardware planes to remain used
    /// during the capture. This source is often not available.
    ///
    /// 'framebuffer' source copies the contents of the final framebuffer.
    /// Using this source temporarily disables all use of hardware planes and
    /// DRM KMS color pipeline features. This source is always available.
    ///
    /// 'full_framebuffer' is otherwise the same as 'framebuffer' except it
    /// will include also any borders (decorations) that the framebuffer may
    /// contain.
    ///
    /// 'blending' source copies the contents of the intermediate blending
    /// buffer, which should be in linear-light format.  Using this source
    /// temporarily disables all use of hardware planes. This source is only
    /// available when a blending buffer exists, e.g. when color management
    /// is active on the output.
    ///
    /// If the pixel source is not one of the defined enumeration values,
    /// 'invalid_source' protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `output`: output to shoot
    /// - `source`: pixel source
    /// - `capture_source_new_id`: new object
    #[inline]
    pub fn try_send_create(
        &self,
        output: &Rc<WlOutput>,
        source: WestonCaptureV1Source,
        capture_source_new_id: &Rc<WestonCaptureSourceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            output,
            source,
            capture_source_new_id,
        );
        let arg0 = arg0.core();
        let arg2_obj = arg2;
        let arg2 = arg2_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
            Some(id) => id,
        };
        arg2.generate_server_id(arg2_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("capture_source_new_id", e)))?;
        let arg2_id = arg2.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: WestonCaptureV1Source, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_capture_v1#{}.create(output: wl_output#{}, source: {:?}, capture_source_new_id: weston_capture_source_v1#{})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2_id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0_id,
            arg1.0,
            arg2_id,
        ]);
        Ok(())
    }

    /// create an object for capturing output images
    ///
    /// This creates a weston_capture_source_v1 object corresponding to the
    /// given wl_output. The object delivers information for allocating
    /// suitable buffers, and exposes the capture function.
    ///
    /// The object will be using the given pixel source for capturing images.
    /// If the source is not available, all attempts to capture will fail
    /// gracefully.
    ///
    /// 'writeback' source will use hardware writeback feature of DRM KMS for
    /// capturing. This may allow hardware planes to remain used
    /// during the capture. This source is often not available.
    ///
    /// 'framebuffer' source copies the contents of the final framebuffer.
    /// Using this source temporarily disables all use of hardware planes and
    /// DRM KMS color pipeline features. This source is always available.
    ///
    /// 'full_framebuffer' is otherwise the same as 'framebuffer' except it
    /// will include also any borders (decorations) that the framebuffer may
    /// contain.
    ///
    /// 'blending' source copies the contents of the intermediate blending
    /// buffer, which should be in linear-light format.  Using this source
    /// temporarily disables all use of hardware planes. This source is only
    /// available when a blending buffer exists, e.g. when color management
    /// is active on the output.
    ///
    /// If the pixel source is not one of the defined enumeration values,
    /// 'invalid_source' protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `output`: output to shoot
    /// - `source`: pixel source
    /// - `capture_source_new_id`: new object
    #[inline]
    pub fn send_create(
        &self,
        output: &Rc<WlOutput>,
        source: WestonCaptureV1Source,
        capture_source_new_id: &Rc<WestonCaptureSourceV1>,
    ) {
        let res = self.try_send_create(
            output,
            source,
            capture_source_new_id,
        );
        if let Err(e) = res {
            log_send("weston_capture_v1.create", &e);
        }
    }

    /// create an object for capturing output images
    ///
    /// This creates a weston_capture_source_v1 object corresponding to the
    /// given wl_output. The object delivers information for allocating
    /// suitable buffers, and exposes the capture function.
    ///
    /// The object will be using the given pixel source for capturing images.
    /// If the source is not available, all attempts to capture will fail
    /// gracefully.
    ///
    /// 'writeback' source will use hardware writeback feature of DRM KMS for
    /// capturing. This may allow hardware planes to remain used
    /// during the capture. This source is often not available.
    ///
    /// 'framebuffer' source copies the contents of the final framebuffer.
    /// Using this source temporarily disables all use of hardware planes and
    /// DRM KMS color pipeline features. This source is always available.
    ///
    /// 'full_framebuffer' is otherwise the same as 'framebuffer' except it
    /// will include also any borders (decorations) that the framebuffer may
    /// contain.
    ///
    /// 'blending' source copies the contents of the intermediate blending
    /// buffer, which should be in linear-light format.  Using this source
    /// temporarily disables all use of hardware planes. This source is only
    /// available when a blending buffer exists, e.g. when color management
    /// is active on the output.
    ///
    /// If the pixel source is not one of the defined enumeration values,
    /// 'invalid_source' protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `output`: output to shoot
    /// - `source`: pixel source
    #[inline]
    pub fn new_try_send_create(
        &self,
        output: &Rc<WlOutput>,
        source: WestonCaptureV1Source,
    ) -> Result<Rc<WestonCaptureSourceV1>, ObjectError> {
        let capture_source_new_id = self.core.create_child();
        self.try_send_create(
            output,
            source,
            &capture_source_new_id,
        )?;
        Ok(capture_source_new_id)
    }

    /// create an object for capturing output images
    ///
    /// This creates a weston_capture_source_v1 object corresponding to the
    /// given wl_output. The object delivers information for allocating
    /// suitable buffers, and exposes the capture function.
    ///
    /// The object will be using the given pixel source for capturing images.
    /// If the source is not available, all attempts to capture will fail
    /// gracefully.
    ///
    /// 'writeback' source will use hardware writeback feature of DRM KMS for
    /// capturing. This may allow hardware planes to remain used
    /// during the capture. This source is often not available.
    ///
    /// 'framebuffer' source copies the contents of the final framebuffer.
    /// Using this source temporarily disables all use of hardware planes and
    /// DRM KMS color pipeline features. This source is always available.
    ///
    /// 'full_framebuffer' is otherwise the same as 'framebuffer' except it
    /// will include also any borders (decorations) that the framebuffer may
    /// contain.
    ///
    /// 'blending' source copies the contents of the intermediate blending
    /// buffer, which should be in linear-light format.  Using this source
    /// temporarily disables all use of hardware planes. This source is only
    /// available when a blending buffer exists, e.g. when color management
    /// is active on the output.
    ///
    /// If the pixel source is not one of the defined enumeration values,
    /// 'invalid_source' protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `output`: output to shoot
    /// - `source`: pixel source
    #[inline]
    pub fn new_send_create(
        &self,
        output: &Rc<WlOutput>,
        source: WestonCaptureV1Source,
    ) -> Rc<WestonCaptureSourceV1> {
        let capture_source_new_id = self.core.create_child();
        self.send_create(
            output,
            source,
            &capture_source_new_id,
        );
        capture_source_new_id
    }
}

/// A message handler for [`WestonCaptureV1`] proxies.
pub trait WestonCaptureV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonCaptureV1>) {
        slf.core.delete_id();
    }

    /// unbind image capture factory
    ///
    /// Affects no other protocol objects in any way.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WestonCaptureV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("weston_capture_v1.destroy", &e);
        }
    }

    /// create an object for capturing output images
    ///
    /// This creates a weston_capture_source_v1 object corresponding to the
    /// given wl_output. The object delivers information for allocating
    /// suitable buffers, and exposes the capture function.
    ///
    /// The object will be using the given pixel source for capturing images.
    /// If the source is not available, all attempts to capture will fail
    /// gracefully.
    ///
    /// 'writeback' source will use hardware writeback feature of DRM KMS for
    /// capturing. This may allow hardware planes to remain used
    /// during the capture. This source is often not available.
    ///
    /// 'framebuffer' source copies the contents of the final framebuffer.
    /// Using this source temporarily disables all use of hardware planes and
    /// DRM KMS color pipeline features. This source is always available.
    ///
    /// 'full_framebuffer' is otherwise the same as 'framebuffer' except it
    /// will include also any borders (decorations) that the framebuffer may
    /// contain.
    ///
    /// 'blending' source copies the contents of the intermediate blending
    /// buffer, which should be in linear-light format.  Using this source
    /// temporarily disables all use of hardware planes. This source is only
    /// available when a blending buffer exists, e.g. when color management
    /// is active on the output.
    ///
    /// If the pixel source is not one of the defined enumeration values,
    /// 'invalid_source' protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `output`: output to shoot
    /// - `source`: pixel source
    /// - `capture_source_new_id`: new object
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_create(
        &mut self,
        slf: &Rc<WestonCaptureV1>,
        output: &Rc<WlOutput>,
        source: WestonCaptureV1Source,
        capture_source_new_id: &Rc<WestonCaptureSourceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create(
            output,
            source,
            capture_source_new_id,
        );
        if let Err(e) = res {
            log_forward("weston_capture_v1.create", &e);
        }
    }
}

impl ObjectPrivate for WestonCaptureV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonCaptureV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_capture_v1#{}.destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg1 = WestonCaptureV1Source(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: WestonCaptureV1Source, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_capture_v1#{}.create(output: wl_output#{}, source: {:?}, capture_source_new_id: weston_capture_source_v1#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg2_id = arg2;
                let arg2 = WestonCaptureSourceV1::new(&self.core.state, self.core.version);
                arg2.core().set_client_id(client, arg2_id, arg2.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg2_id, "capture_source_new_id", e)))?;
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_create(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_create(&self, arg0, arg1, arg2);
                }
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "create",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WestonCaptureV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl WestonCaptureV1 {
    /// Since when the error.invalid_source enum variant is available.
    pub const ENM__ERROR_INVALID_SOURCE__SINCE: u32 = 1;

    /// Since when the source.writeback enum variant is available.
    pub const ENM__SOURCE_WRITEBACK__SINCE: u32 = 1;
    /// Since when the source.framebuffer enum variant is available.
    pub const ENM__SOURCE_FRAMEBUFFER__SINCE: u32 = 1;
    /// Since when the source.full_framebuffer enum variant is available.
    pub const ENM__SOURCE_FULL_FRAMEBUFFER__SINCE: u32 = 1;
    /// Since when the source.blending enum variant is available.
    pub const ENM__SOURCE_BLENDING__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonCaptureV1Error(pub u32);

impl WestonCaptureV1Error {
    /// invalid source enum value
    pub const INVALID_SOURCE: Self = Self(0);
}

impl Debug for WestonCaptureV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SOURCE => "INVALID_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonCaptureV1Source(pub u32);

impl WestonCaptureV1Source {
    /// use hardware writeback
    pub const WRITEBACK: Self = Self(0);

    /// copy from framebuffer, desktop area
    pub const FRAMEBUFFER: Self = Self(1);

    /// copy whole framebuffer, including borders
    pub const FULL_FRAMEBUFFER: Self = Self(2);

    /// copy from blending space
    pub const BLENDING: Self = Self(3);
}

impl Debug for WestonCaptureV1Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::WRITEBACK => "WRITEBACK",
            Self::FRAMEBUFFER => "FRAMEBUFFER",
            Self::FULL_FRAMEBUFFER => "FULL_FRAMEBUFFER",
            Self::BLENDING => "BLENDING",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
