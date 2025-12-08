//! manager to inform clients and begin capturing
//!
//! This object is a manager which offers requests to start capturing from a
//! source.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_toplevel_export_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandToplevelExportManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandToplevelExportManagerV1Handler>,
}

struct DefaultHandler;

impl HyprlandToplevelExportManagerV1Handler for DefaultHandler { }

impl ConcreteObject for HyprlandToplevelExportManagerV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::HyprlandToplevelExportManagerV1;
    const INTERFACE_NAME: &str = "hyprland_toplevel_export_manager_v1";
}

impl HyprlandToplevelExportManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl HyprlandToplevelExportManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandToplevelExportManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandToplevelExportManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandToplevelExportManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandToplevelExportManagerV1 {
    /// Since when the capture_toplevel message is available.
    pub const MSG__CAPTURE_TOPLEVEL__SINCE: u32 = 1;

    /// capture a toplevel
    ///
    /// Capture the next frame of a toplevel. (window)
    ///
    /// The captured frame will not contain any server-side decorations and will
    /// ignore the compositor-set geometry, like e.g. rounded corners.
    ///
    /// It will contain all the subsurfaces and popups, however the latter will be clipped
    /// to the geometry of the base surface.
    ///
    /// The handle parameter refers to the address of the window as seen in `hyprctl clients`.
    /// For example, for d161e7b0 it would be 3512854448.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `handle`: the handle of the toplevel (window) to be captured
    #[inline]
    pub fn try_send_capture_toplevel(
        &self,
        frame: &Rc<HyprlandToplevelExportFrameV1>,
        overlay_cursor: i32,
        handle: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            frame,
            overlay_cursor,
            handle,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("frame", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_toplevel_export_manager_v1#{}.capture_toplevel(frame: hyprland_toplevel_export_frame_v1#{}, overlay_cursor: {}, handle: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2);
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
            arg0_id,
            arg1 as u32,
            arg2,
        ]);
        Ok(())
    }

    /// capture a toplevel
    ///
    /// Capture the next frame of a toplevel. (window)
    ///
    /// The captured frame will not contain any server-side decorations and will
    /// ignore the compositor-set geometry, like e.g. rounded corners.
    ///
    /// It will contain all the subsurfaces and popups, however the latter will be clipped
    /// to the geometry of the base surface.
    ///
    /// The handle parameter refers to the address of the window as seen in `hyprctl clients`.
    /// For example, for d161e7b0 it would be 3512854448.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `handle`: the handle of the toplevel (window) to be captured
    #[inline]
    pub fn send_capture_toplevel(
        &self,
        frame: &Rc<HyprlandToplevelExportFrameV1>,
        overlay_cursor: i32,
        handle: u32,
    ) {
        let res = self.try_send_capture_toplevel(
            frame,
            overlay_cursor,
            handle,
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_manager_v1.capture_toplevel", &e);
        }
    }

    /// capture a toplevel
    ///
    /// Capture the next frame of a toplevel. (window)
    ///
    /// The captured frame will not contain any server-side decorations and will
    /// ignore the compositor-set geometry, like e.g. rounded corners.
    ///
    /// It will contain all the subsurfaces and popups, however the latter will be clipped
    /// to the geometry of the base surface.
    ///
    /// The handle parameter refers to the address of the window as seen in `hyprctl clients`.
    /// For example, for d161e7b0 it would be 3512854448.
    ///
    /// # Arguments
    ///
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `handle`: the handle of the toplevel (window) to be captured
    #[inline]
    pub fn new_try_send_capture_toplevel(
        &self,
        overlay_cursor: i32,
        handle: u32,
    ) -> Result<Rc<HyprlandToplevelExportFrameV1>, ObjectError> {
        let frame = self.core.create_child();
        self.try_send_capture_toplevel(
            &frame,
            overlay_cursor,
            handle,
        )?;
        Ok(frame)
    }

    /// capture a toplevel
    ///
    /// Capture the next frame of a toplevel. (window)
    ///
    /// The captured frame will not contain any server-side decorations and will
    /// ignore the compositor-set geometry, like e.g. rounded corners.
    ///
    /// It will contain all the subsurfaces and popups, however the latter will be clipped
    /// to the geometry of the base surface.
    ///
    /// The handle parameter refers to the address of the window as seen in `hyprctl clients`.
    /// For example, for d161e7b0 it would be 3512854448.
    ///
    /// # Arguments
    ///
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `handle`: the handle of the toplevel (window) to be captured
    #[inline]
    pub fn new_send_capture_toplevel(
        &self,
        overlay_cursor: i32,
        handle: u32,
    ) -> Rc<HyprlandToplevelExportFrameV1> {
        let frame = self.core.create_child();
        self.send_capture_toplevel(
            &frame,
            overlay_cursor,
            handle,
        );
        frame
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_toplevel_export_manager_v1#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_manager_v1.destroy", &e);
        }
    }

    /// Since when the capture_toplevel_with_wlr_toplevel_handle message is available.
    pub const MSG__CAPTURE_TOPLEVEL_WITH_WLR_TOPLEVEL_HANDLE__SINCE: u32 = 2;

    /// capture a toplevel
    ///
    /// Same as capture_toplevel, but with a zwlr_foreign_toplevel_handle_v1 handle.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `handle`: the zwlr_foreign_toplevel_handle_v1 handle of the toplevel to be captured
    #[inline]
    pub fn try_send_capture_toplevel_with_wlr_toplevel_handle(
        &self,
        frame: &Rc<HyprlandToplevelExportFrameV1>,
        overlay_cursor: i32,
        handle: &Rc<ZwlrForeignToplevelHandleV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            frame,
            overlay_cursor,
            handle,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("handle"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("frame", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_toplevel_export_manager_v1#{}.capture_toplevel_with_wlr_toplevel_handle(frame: hyprland_toplevel_export_frame_v1#{}, overlay_cursor: {}, handle: zwlr_foreign_toplevel_handle_v1#{})\n", id, arg0, arg1, arg2);
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
            2,
            arg0_id,
            arg1 as u32,
            arg2_id,
        ]);
        Ok(())
    }

    /// capture a toplevel
    ///
    /// Same as capture_toplevel, but with a zwlr_foreign_toplevel_handle_v1 handle.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `handle`: the zwlr_foreign_toplevel_handle_v1 handle of the toplevel to be captured
    #[inline]
    pub fn send_capture_toplevel_with_wlr_toplevel_handle(
        &self,
        frame: &Rc<HyprlandToplevelExportFrameV1>,
        overlay_cursor: i32,
        handle: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        let res = self.try_send_capture_toplevel_with_wlr_toplevel_handle(
            frame,
            overlay_cursor,
            handle,
        );
        if let Err(e) = res {
            log_send("hyprland_toplevel_export_manager_v1.capture_toplevel_with_wlr_toplevel_handle", &e);
        }
    }

    /// capture a toplevel
    ///
    /// Same as capture_toplevel, but with a zwlr_foreign_toplevel_handle_v1 handle.
    ///
    /// # Arguments
    ///
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `handle`: the zwlr_foreign_toplevel_handle_v1 handle of the toplevel to be captured
    #[inline]
    pub fn new_try_send_capture_toplevel_with_wlr_toplevel_handle(
        &self,
        overlay_cursor: i32,
        handle: &Rc<ZwlrForeignToplevelHandleV1>,
    ) -> Result<Rc<HyprlandToplevelExportFrameV1>, ObjectError> {
        let frame = self.core.create_child();
        self.try_send_capture_toplevel_with_wlr_toplevel_handle(
            &frame,
            overlay_cursor,
            handle,
        )?;
        Ok(frame)
    }

    /// capture a toplevel
    ///
    /// Same as capture_toplevel, but with a zwlr_foreign_toplevel_handle_v1 handle.
    ///
    /// # Arguments
    ///
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `handle`: the zwlr_foreign_toplevel_handle_v1 handle of the toplevel to be captured
    #[inline]
    pub fn new_send_capture_toplevel_with_wlr_toplevel_handle(
        &self,
        overlay_cursor: i32,
        handle: &Rc<ZwlrForeignToplevelHandleV1>,
    ) -> Rc<HyprlandToplevelExportFrameV1> {
        let frame = self.core.create_child();
        self.send_capture_toplevel_with_wlr_toplevel_handle(
            &frame,
            overlay_cursor,
            handle,
        );
        frame
    }
}

/// A message handler for [`HyprlandToplevelExportManagerV1`] proxies.
pub trait HyprlandToplevelExportManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandToplevelExportManagerV1>) {
        slf.core.delete_id();
    }

    /// capture a toplevel
    ///
    /// Capture the next frame of a toplevel. (window)
    ///
    /// The captured frame will not contain any server-side decorations and will
    /// ignore the compositor-set geometry, like e.g. rounded corners.
    ///
    /// It will contain all the subsurfaces and popups, however the latter will be clipped
    /// to the geometry of the base surface.
    ///
    /// The handle parameter refers to the address of the window as seen in `hyprctl clients`.
    /// For example, for d161e7b0 it would be 3512854448.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `handle`: the handle of the toplevel (window) to be captured
    #[inline]
    fn handle_capture_toplevel(
        &mut self,
        slf: &Rc<HyprlandToplevelExportManagerV1>,
        frame: &Rc<HyprlandToplevelExportFrameV1>,
        overlay_cursor: i32,
        handle: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_capture_toplevel(
            frame,
            overlay_cursor,
            handle,
        );
        if let Err(e) = res {
            log_forward("hyprland_toplevel_export_manager_v1.capture_toplevel", &e);
        }
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<HyprlandToplevelExportManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("hyprland_toplevel_export_manager_v1.destroy", &e);
        }
    }

    /// capture a toplevel
    ///
    /// Same as capture_toplevel, but with a zwlr_foreign_toplevel_handle_v1 handle.
    ///
    /// # Arguments
    ///
    /// - `frame`:
    /// - `overlay_cursor`: composite cursor onto the frame
    /// - `handle`: the zwlr_foreign_toplevel_handle_v1 handle of the toplevel to be captured
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_capture_toplevel_with_wlr_toplevel_handle(
        &mut self,
        slf: &Rc<HyprlandToplevelExportManagerV1>,
        frame: &Rc<HyprlandToplevelExportFrameV1>,
        overlay_cursor: i32,
        handle: &Rc<ZwlrForeignToplevelHandleV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_capture_toplevel_with_wlr_toplevel_handle(
            frame,
            overlay_cursor,
            handle,
        );
        if let Err(e) = res {
            log_forward("hyprland_toplevel_export_manager_v1.capture_toplevel_with_wlr_toplevel_handle", &e);
        }
    }
}

impl ObjectPrivate for HyprlandToplevelExportManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandToplevelExportManagerV1, version),
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
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_toplevel_export_manager_v1#{}.capture_toplevel(frame: hyprland_toplevel_export_frame_v1#{}, overlay_cursor: {}, handle: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = HyprlandToplevelExportFrameV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "frame", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_capture_toplevel(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_capture_toplevel(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_toplevel_export_manager_v1#{}.destroy()\n", client_id, id);
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
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_toplevel_export_manager_v1#{}.capture_toplevel_with_wlr_toplevel_handle(frame: hyprland_toplevel_export_frame_v1#{}, overlay_cursor: {}, handle: zwlr_foreign_toplevel_handle_v1#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let arg0 = HyprlandToplevelExportFrameV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "frame", e)))?;
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<ZwlrForeignToplevelHandleV1>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("handle", o.core().interface, ObjectInterface::ZwlrForeignToplevelHandleV1)));
                };
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_capture_toplevel_with_wlr_toplevel_handle(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_capture_toplevel_with_wlr_toplevel_handle(&self, arg0, arg1, arg2);
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
            0 => "capture_toplevel",
            1 => "destroy",
            2 => "capture_toplevel_with_wlr_toplevel_handle",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for HyprlandToplevelExportManagerV1 {
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

