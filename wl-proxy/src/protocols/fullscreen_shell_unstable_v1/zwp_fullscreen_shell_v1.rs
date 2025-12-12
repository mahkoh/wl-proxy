//! displays a single surface per output
//!
//! Displays a single surface per output.
//!
//! This interface provides a mechanism for a single client to display
//! simple full-screen surfaces.  While there technically may be multiple
//! clients bound to this interface, only one of those clients should be
//! shown at a time.
//!
//! To present a surface, the client uses either the present_surface or
//! present_surface_for_mode requests.  Presenting a surface takes effect
//! on the next wl_surface.commit.  See the individual requests for
//! details about scaling and mode switches.
//!
//! The client can have at most one surface per output at any time.
//! Requesting a surface to be presented on an output that already has a
//! surface replaces the previously presented surface.  Presenting a null
//! surface removes its content and effectively disables the output.
//! Exactly what happens when an output is "disabled" is
//! compositor-specific.  The same surface may be presented on multiple
//! outputs simultaneously.
//!
//! Once a surface is presented on an output, it stays on that output
//! until either the client removes it or the compositor destroys the
//! output.  This way, the client can update the output's contents by
//! simply attaching a new buffer.
//!
//! Warning! The protocol described in this file is experimental and
//! backward incompatible changes may be made. Backward compatible changes
//! may be added together with the corresponding interface version bump.
//! Backward incompatible changes are done by bumping the version number in
//! the protocol and interface names and resetting the interface version.
//! Once the protocol is to be declared stable, the 'z' prefix and the
//! version number in the protocol and interface names are removed and the
//! interface version number is reset.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwp_fullscreen_shell_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwpFullscreenShellV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwpFullscreenShellV1Handler>,
}

struct DefaultHandler;

impl ZwpFullscreenShellV1Handler for DefaultHandler { }

impl ZwpFullscreenShellV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::ZwpFullscreenShellV1;
    pub const INTERFACE_NAME: &str = "zwp_fullscreen_shell_v1";
}

impl ZwpFullscreenShellV1 {
    pub fn set_handler(&self, handler: impl ZwpFullscreenShellV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ZwpFullscreenShellV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwpFullscreenShellV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwpFullscreenShellV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwpFullscreenShellV1 {
    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// release the wl_fullscreen_shell interface
    ///
    /// Release the binding from the wl_fullscreen_shell interface.
    ///
    /// This destroys the server-side object and frees this binding.  If
    /// the client binds to wl_fullscreen_shell multiple times, it may wish
    /// to free some of those bindings.
    #[inline]
    pub fn send_release(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_fullscreen_shell_v1#{}.release()\n", id);
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

    /// Since when the capability message is available.
    pub const MSG__CAPABILITY__SINCE: u32 = 1;

    /// advertises a capability of the compositor
    ///
    /// Advertises a single capability of the compositor.
    ///
    /// When the wl_fullscreen_shell interface is bound, this event is emitted
    /// once for each capability advertised.  Valid capabilities are given by
    /// the wl_fullscreen_shell.capability enum.  If clients want to take
    /// advantage of any of these capabilities, they should use a
    /// wl_display.sync request immediately after binding to ensure that they
    /// receive all the capability events.
    ///
    /// # Arguments
    ///
    /// - `capability`:
    #[inline]
    pub fn send_capability(
        &self,
        capability: ZwpFullscreenShellV1Capability,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            capability,
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= zwp_fullscreen_shell_v1#{}.capability(capability: {:?})\n", client.endpoint.id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the present_surface message is available.
    pub const MSG__PRESENT_SURFACE__SINCE: u32 = 1;

    /// present surface for display
    ///
    /// Present a surface on the given output.
    ///
    /// If the output is null, the compositor will present the surface on
    /// whatever display (or displays) it thinks best.  In particular, this
    /// may replace any or all surfaces currently presented so it should
    /// not be used in combination with placing surfaces on specific
    /// outputs.
    ///
    /// The method parameter is a hint to the compositor for how the surface
    /// is to be presented.  In particular, it tells the compositor how to
    /// handle a size mismatch between the presented surface and the
    /// output.  The compositor is free to ignore this parameter.
    ///
    /// The "zoom", "zoom_crop", and "stretch" methods imply a scaling
    /// operation on the surface.  This will override any kind of output
    /// scaling, so the buffer_scale property of the surface is effectively
    /// ignored.
    ///
    /// This request gives the surface the role of a fullscreen shell surface.
    /// If the surface already has another role, it raises a role protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    /// - `method`:
    /// - `output`:
    #[inline]
    pub fn send_present_surface(
        &self,
        surface: Option<&Rc<WlSurface>>,
        method: ZwpFullscreenShellV1PresentMethod,
        output: Option<&Rc<WlOutput>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            surface,
            method,
            output,
        );
        let arg0 = arg0.map(|a| a.core());
        let arg2 = arg2.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("surface")),
                Some(id) => id,
            },
        };
        let arg2_id = match arg2 {
            None => 0,
            Some(arg2) => match arg2.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("output")),
                Some(id) => id,
            },
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_fullscreen_shell_v1#{}.present_surface(surface: wl_surface#{}, method: {:?}, output: wl_output#{})\n", id, arg0_id, arg1, arg2_id);
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
            arg0_id,
            arg1.0,
            arg2_id,
        ]);
        Ok(())
    }

    /// Since when the present_surface_for_mode message is available.
    pub const MSG__PRESENT_SURFACE_FOR_MODE__SINCE: u32 = 1;

    /// present surface for display at a particular mode
    ///
    /// Presents a surface on the given output for a particular mode.
    ///
    /// If the current size of the output differs from that of the surface,
    /// the compositor will attempt to change the size of the output to
    /// match the surface.  The result of the mode-switch operation will be
    /// returned via the provided wl_fullscreen_shell_mode_feedback object.
    ///
    /// If the current output mode matches the one requested or if the
    /// compositor successfully switches the mode to match the surface,
    /// then the mode_successful event will be sent and the output will
    /// contain the contents of the given surface.  If the compositor
    /// cannot match the output size to the surface size, the mode_failed
    /// will be sent and the output will contain the contents of the
    /// previously presented surface (if any).  If another surface is
    /// presented on the given output before either of these has a chance
    /// to happen, the present_cancelled event will be sent.
    ///
    /// Due to race conditions and other issues unknown to the client, no
    /// mode-switch operation is guaranteed to succeed.  However, if the
    /// mode is one advertised by wl_output.mode or if the compositor
    /// advertises the ARBITRARY_MODES capability, then the client should
    /// expect that the mode-switch operation will usually succeed.
    ///
    /// If the size of the presented surface changes, the resulting output
    /// is undefined.  The compositor may attempt to change the output mode
    /// to compensate.  However, there is no guarantee that a suitable mode
    /// will be found and the client has no way to be notified of success
    /// or failure.
    ///
    /// The framerate parameter specifies the desired framerate for the
    /// output in mHz.  The compositor is free to ignore this parameter.  A
    /// value of 0 indicates that the client has no preference.
    ///
    /// If the value of wl_output.scale differs from wl_surface.buffer_scale,
    /// then the compositor may choose a mode that matches either the buffer
    /// size or the surface size.  In either case, the surface will fill the
    /// output.
    ///
    /// This request gives the surface the role of a fullscreen shell surface.
    /// If the surface already has another role, it raises a role protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    /// - `output`:
    /// - `framerate`:
    /// - `feedback`:
    #[inline]
    pub fn send_present_surface_for_mode(
        &self,
        surface: &Rc<WlSurface>,
        output: &Rc<WlOutput>,
        framerate: i32,
        feedback: &Rc<ZwpFullscreenShellModeFeedbackV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            surface,
            output,
            framerate,
            feedback,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.core();
        let arg3_obj = arg3;
        let arg3 = arg3_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("output")),
            Some(id) => id,
        };
        arg3.generate_server_id(arg3_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("feedback", e))?;
        let arg3_id = arg3.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwp_fullscreen_shell_v1#{}.present_surface_for_mode(surface: wl_surface#{}, output: wl_output#{}, framerate: {}, feedback: zwp_fullscreen_shell_mode_feedback_v1#{})\n", id, arg0_id, arg1_id, arg2, arg3_id);
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
            arg1_id,
            arg2 as u32,
            arg3_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpFullscreenShellV1] proxies.
pub trait ZwpFullscreenShellV1Handler: Any {
    /// release the wl_fullscreen_shell interface
    ///
    /// Release the binding from the wl_fullscreen_shell interface.
    ///
    /// This destroys the server-side object and frees this binding.  If
    /// the client binds to wl_fullscreen_shell multiple times, it may wish
    /// to free some of those bindings.
    #[inline]
    fn release(
        &mut self,
        _slf: &Rc<ZwpFullscreenShellV1>,
    ) {
        let res = _slf.send_release(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_fullscreen_shell_v1.release message: {}", Report::new(e));
        }
    }

    /// advertises a capability of the compositor
    ///
    /// Advertises a single capability of the compositor.
    ///
    /// When the wl_fullscreen_shell interface is bound, this event is emitted
    /// once for each capability advertised.  Valid capabilities are given by
    /// the wl_fullscreen_shell.capability enum.  If clients want to take
    /// advantage of any of these capabilities, they should use a
    /// wl_display.sync request immediately after binding to ensure that they
    /// receive all the capability events.
    ///
    /// # Arguments
    ///
    /// - `capability`:
    #[inline]
    fn capability(
        &mut self,
        _slf: &Rc<ZwpFullscreenShellV1>,
        capability: ZwpFullscreenShellV1Capability,
    ) {
        if _slf.core.zombie.get() {
            return;
        }
        let res = _slf.send_capability(
            capability,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_fullscreen_shell_v1.capability message: {}", Report::new(e));
        }
    }

    /// present surface for display
    ///
    /// Present a surface on the given output.
    ///
    /// If the output is null, the compositor will present the surface on
    /// whatever display (or displays) it thinks best.  In particular, this
    /// may replace any or all surfaces currently presented so it should
    /// not be used in combination with placing surfaces on specific
    /// outputs.
    ///
    /// The method parameter is a hint to the compositor for how the surface
    /// is to be presented.  In particular, it tells the compositor how to
    /// handle a size mismatch between the presented surface and the
    /// output.  The compositor is free to ignore this parameter.
    ///
    /// The "zoom", "zoom_crop", and "stretch" methods imply a scaling
    /// operation on the surface.  This will override any kind of output
    /// scaling, so the buffer_scale property of the surface is effectively
    /// ignored.
    ///
    /// This request gives the surface the role of a fullscreen shell surface.
    /// If the surface already has another role, it raises a role protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    /// - `method`:
    /// - `output`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn present_surface(
        &mut self,
        _slf: &Rc<ZwpFullscreenShellV1>,
        surface: Option<&Rc<WlSurface>>,
        method: ZwpFullscreenShellV1PresentMethod,
        output: Option<&Rc<WlOutput>>,
    ) {
        let res = _slf.send_present_surface(
            surface,
            method,
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_fullscreen_shell_v1.present_surface message: {}", Report::new(e));
        }
    }

    /// present surface for display at a particular mode
    ///
    /// Presents a surface on the given output for a particular mode.
    ///
    /// If the current size of the output differs from that of the surface,
    /// the compositor will attempt to change the size of the output to
    /// match the surface.  The result of the mode-switch operation will be
    /// returned via the provided wl_fullscreen_shell_mode_feedback object.
    ///
    /// If the current output mode matches the one requested or if the
    /// compositor successfully switches the mode to match the surface,
    /// then the mode_successful event will be sent and the output will
    /// contain the contents of the given surface.  If the compositor
    /// cannot match the output size to the surface size, the mode_failed
    /// will be sent and the output will contain the contents of the
    /// previously presented surface (if any).  If another surface is
    /// presented on the given output before either of these has a chance
    /// to happen, the present_cancelled event will be sent.
    ///
    /// Due to race conditions and other issues unknown to the client, no
    /// mode-switch operation is guaranteed to succeed.  However, if the
    /// mode is one advertised by wl_output.mode or if the compositor
    /// advertises the ARBITRARY_MODES capability, then the client should
    /// expect that the mode-switch operation will usually succeed.
    ///
    /// If the size of the presented surface changes, the resulting output
    /// is undefined.  The compositor may attempt to change the output mode
    /// to compensate.  However, there is no guarantee that a suitable mode
    /// will be found and the client has no way to be notified of success
    /// or failure.
    ///
    /// The framerate parameter specifies the desired framerate for the
    /// output in mHz.  The compositor is free to ignore this parameter.  A
    /// value of 0 indicates that the client has no preference.
    ///
    /// If the value of wl_output.scale differs from wl_surface.buffer_scale,
    /// then the compositor may choose a mode that matches either the buffer
    /// size or the surface size.  In either case, the surface will fill the
    /// output.
    ///
    /// This request gives the surface the role of a fullscreen shell surface.
    /// If the surface already has another role, it raises a role protocol
    /// error.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    /// - `output`:
    /// - `framerate`:
    /// - `feedback`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn present_surface_for_mode(
        &mut self,
        _slf: &Rc<ZwpFullscreenShellV1>,
        surface: &Rc<WlSurface>,
        output: &Rc<WlOutput>,
        framerate: i32,
        feedback: &Rc<ZwpFullscreenShellModeFeedbackV1>,
    ) {
        let res = _slf.send_present_surface_for_mode(
            surface,
            output,
            framerate,
            feedback,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_fullscreen_shell_v1.present_surface_for_mode message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for ZwpFullscreenShellV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwpFullscreenShellV1, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_fullscreen_shell_v1#{}.release()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).release(&self);
                } else {
                    DefaultHandler.release(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg1 = ZwpFullscreenShellV1PresentMethod(arg1);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_fullscreen_shell_v1#{}.present_surface(surface: wl_surface#{}, method: {:?}, output: wl_output#{})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface));
                    };
                    Some(arg0)
                };
                let arg2 = if arg2 == 0 {
                    None
                } else {
                    let arg2_id = arg2;
                    let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                    };
                    let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlOutput>() else {
                        let o = client.endpoint.lookup(arg2_id).unwrap();
                        return Err(ObjectError::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput));
                    };
                    Some(arg2)
                };
                let arg0 = arg0.as_ref();
                let arg2 = arg2.as_ref();
                if let Some(handler) = handler {
                    (**handler).present_surface(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.present_surface(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 24));
                };
                let arg2 = arg2 as i32;
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwp_fullscreen_shell_v1#{}.present_surface_for_mode(surface: wl_surface#{}, output: wl_output#{}, framerate: {}, feedback: zwp_fullscreen_shell_mode_feedback_v1#{})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface));
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput));
                };
                let arg3_id = arg3;
                let arg3 = ZwpFullscreenShellModeFeedbackV1::new(&self.core.state, self.core.version);
                arg3.core().set_client_id(client, arg3_id, arg3.clone())
                    .map_err(|e| ObjectError::SetClientId(arg3_id, "feedback", e))?;
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg3 = &arg3;
                if let Some(handler) = handler {
                    (**handler).present_surface_for_mode(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.present_surface_for_mode(&self, arg0, arg1, arg2, arg3);
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
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = ZwpFullscreenShellV1Capability(arg0);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> zwp_fullscreen_shell_v1#{}.capability(capability: {:?})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).capability(&self, arg0);
                } else {
                    DefaultHandler.capability(&self, arg0);
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
            0 => "release",
            1 => "present_surface",
            2 => "present_surface_for_mode",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "capability",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for ZwpFullscreenShellV1 {
    fn core(&self) -> &ObjectCore {
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

impl ZwpFullscreenShellV1 {
    /// Since when the capability.arbitrary_modes enum variant is available.
    pub const ENM__CAPABILITY_ARBITRARY_MODES__SINCE: u32 = 1;
    /// Since when the capability.cursor_plane enum variant is available.
    pub const ENM__CAPABILITY_CURSOR_PLANE__SINCE: u32 = 1;

    /// Since when the present_method.default enum variant is available.
    pub const ENM__PRESENT_METHOD_DEFAULT__SINCE: u32 = 1;
    /// Since when the present_method.center enum variant is available.
    pub const ENM__PRESENT_METHOD_CENTER__SINCE: u32 = 1;
    /// Since when the present_method.zoom enum variant is available.
    pub const ENM__PRESENT_METHOD_ZOOM__SINCE: u32 = 1;
    /// Since when the present_method.zoom_crop enum variant is available.
    pub const ENM__PRESENT_METHOD_ZOOM_CROP__SINCE: u32 = 1;
    /// Since when the present_method.stretch enum variant is available.
    pub const ENM__PRESENT_METHOD_STRETCH__SINCE: u32 = 1;

    /// Since when the error.invalid_method enum variant is available.
    pub const ENM__ERROR_INVALID_METHOD__SINCE: u32 = 1;
    /// Since when the error.role enum variant is available.
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
}

/// capabilities advertised by the compositor
///
/// Various capabilities that can be advertised by the compositor.  They
/// are advertised one-at-a-time when the wl_fullscreen_shell interface is
/// bound.  See the wl_fullscreen_shell.capability event for more details.
///
/// ARBITRARY_MODES:
/// This is a hint to the client that indicates that the compositor is
/// capable of setting practically any mode on its outputs.  If this
/// capability is provided, wl_fullscreen_shell.present_surface_for_mode
/// will almost never fail and clients should feel free to set whatever
/// mode they like.  If the compositor does not advertise this, it may
/// still support some modes that are not advertised through wl_global.mode
/// but it is less likely.
///
/// CURSOR_PLANE:
/// This is a hint to the client that indicates that the compositor can
/// handle a cursor surface from the client without actually compositing.
/// This may be because of a hardware cursor plane or some other mechanism.
/// If the compositor does not advertise this capability then setting
/// wl_pointer.cursor may degrade performance or be ignored entirely.  If
/// CURSOR_PLANE is not advertised, it is recommended that the client draw
/// its own cursor and set wl_pointer.cursor(NULL).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpFullscreenShellV1Capability(pub u32);

impl ZwpFullscreenShellV1Capability {
    /// compositor is capable of almost any output mode
    pub const ARBITRARY_MODES: Self = Self(1);

    /// compositor has a separate cursor plane
    pub const CURSOR_PLANE: Self = Self(2);
}

impl Debug for ZwpFullscreenShellV1Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ARBITRARY_MODES => "ARBITRARY_MODES",
            Self::CURSOR_PLANE => "CURSOR_PLANE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// different method to set the surface fullscreen
///
/// Hints to indicate to the compositor how to deal with a conflict
/// between the dimensions of the surface and the dimensions of the
/// output. The compositor is free to ignore this parameter.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpFullscreenShellV1PresentMethod(pub u32);

impl ZwpFullscreenShellV1PresentMethod {
    /// no preference, apply default policy
    pub const DEFAULT: Self = Self(0);

    /// center the surface on the output
    pub const CENTER: Self = Self(1);

    /// scale the surface, preserving aspect ratio, to the largest size that will fit on the output
    pub const ZOOM: Self = Self(2);

    /// scale the surface, preserving aspect ratio, to fully fill the output cropping if needed
    pub const ZOOM_CROP: Self = Self(3);

    /// scale the surface to the size of the output ignoring aspect ratio
    pub const STRETCH: Self = Self(4);
}

impl Debug for ZwpFullscreenShellV1PresentMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DEFAULT => "DEFAULT",
            Self::CENTER => "CENTER",
            Self::ZOOM => "ZOOM",
            Self::ZOOM_CROP => "ZOOM_CROP",
            Self::STRETCH => "STRETCH",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// wl_fullscreen_shell error values
///
/// These errors can be emitted in response to wl_fullscreen_shell requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwpFullscreenShellV1Error(pub u32);

impl ZwpFullscreenShellV1Error {
    /// present_method is not known
    pub const INVALID_METHOD: Self = Self(0);

    /// given wl_surface has another role
    pub const ROLE: Self = Self(1);
}

impl Debug for ZwpFullscreenShellV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_METHOD => "INVALID_METHOD",
            Self::ROLE => "ROLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
