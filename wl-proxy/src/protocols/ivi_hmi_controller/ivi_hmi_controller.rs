//! set up and control IVI style UI

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A ivi_hmi_controller object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct IviHmiController {
    core: ObjectCore,
    handler: HandlerHolder<dyn IviHmiControllerHandler>,
}

struct DefaultHandler;

impl IviHmiControllerHandler for DefaultHandler { }

impl ConcreteObject for IviHmiController {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::IviHmiController;
    const INTERFACE_NAME: &str = "ivi_hmi_controller";
}

impl IviHmiController {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl IviHmiControllerHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn IviHmiControllerHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for IviHmiController {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IviHmiController")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl IviHmiController {
    /// Since when the UI_ready message is available.
    pub const MSG__UI_READY__SINCE: u32 = 1;

    /// inform the ready for drawing desktop.
    #[inline]
    pub fn try_send_UI_ready(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ivi_hmi_controller#{}.UI_ready()\n", id);
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
        Ok(())
    }

    /// inform the ready for drawing desktop.
    #[inline]
    pub fn send_UI_ready(
        &self,
    ) {
        let res = self.try_send_UI_ready(
        );
        if let Err(e) = res {
            log_send("ivi_hmi_controller.UI_ready", &e);
        }
    }

    /// Since when the workspace_control message is available.
    pub const MSG__WORKSPACE_CONTROL__SINCE: u32 = 1;

    /// start controlling a surface by server
    ///
    /// Reference protocol to control a surface by server.
    /// To control a surface by server, it gives seat to the server
    /// to e.g. control Home screen. Home screen has several workspaces
    /// to group launchers of wayland application. These workspaces
    /// are drawn on a horizontally long surface to be controlled
    /// by motion of input device. E.g. A motion from right to left
    /// happens, the viewport of surface is controlled in the ivi-shell
    /// by using ivi-layout. client can recognizes the end of controlling
    /// by event "workspace_end_control".
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `serial`:
    #[inline]
    pub fn try_send_workspace_control(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            seat,
            serial,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("seat"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ivi_hmi_controller#{}.workspace_control(seat: wl_seat#{}, serial: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg1,
        ]);
        Ok(())
    }

    /// start controlling a surface by server
    ///
    /// Reference protocol to control a surface by server.
    /// To control a surface by server, it gives seat to the server
    /// to e.g. control Home screen. Home screen has several workspaces
    /// to group launchers of wayland application. These workspaces
    /// are drawn on a horizontally long surface to be controlled
    /// by motion of input device. E.g. A motion from right to left
    /// happens, the viewport of surface is controlled in the ivi-shell
    /// by using ivi-layout. client can recognizes the end of controlling
    /// by event "workspace_end_control".
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `serial`:
    #[inline]
    pub fn send_workspace_control(
        &self,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) {
        let res = self.try_send_workspace_control(
            seat,
            serial,
        );
        if let Err(e) = res {
            log_send("ivi_hmi_controller.workspace_control", &e);
        }
    }

    /// Since when the switch_mode message is available.
    pub const MSG__SWITCH_MODE__SINCE: u32 = 1;

    /// request mode switch of application layout
    ///
    /// hmi-controller loaded to ivi-shall implements 4 types of layout
    /// as a reference; tiling, side by side, full_screen, and random.
    ///
    /// # Arguments
    ///
    /// - `layout_mode`:
    #[inline]
    pub fn try_send_switch_mode(
        &self,
        layout_mode: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            layout_mode,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ivi_hmi_controller#{}.switch_mode(layout_mode: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0,
        ]);
        Ok(())
    }

    /// request mode switch of application layout
    ///
    /// hmi-controller loaded to ivi-shall implements 4 types of layout
    /// as a reference; tiling, side by side, full_screen, and random.
    ///
    /// # Arguments
    ///
    /// - `layout_mode`:
    #[inline]
    pub fn send_switch_mode(
        &self,
        layout_mode: u32,
    ) {
        let res = self.try_send_switch_mode(
            layout_mode,
        );
        if let Err(e) = res {
            log_send("ivi_hmi_controller.switch_mode", &e);
        }
    }

    /// Since when the home message is available.
    pub const MSG__HOME__SINCE: u32 = 1;

    /// request displaying/undisplaying home screen
    ///
    /// home screen is a reference implementation of launcher to launch
    /// wayland applications. The home screen has several workspaces to
    /// group wayland applications. By defining the following keys in
    /// weston.ini, user can add launcher icon to launch a wayland application
    /// to a workspace.
    /// [ivi-launcher]
    /// workspace-id=0
    ///         : id of workspace to add a launcher
    /// icon-id=4001
    ///         : ivi id of ivi_surface to draw an icon
    /// icon=/home/user/review/build-ivi-shell/data/icon_ivi_flower.png
    ///         : path to icon image
    /// path=/home/user/review/build-ivi-shell/weston-dnd
    ///         : path to wayland application
    ///
    /// # Arguments
    ///
    /// - `home`:
    #[inline]
    pub fn try_send_home(
        &self,
        home: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            home,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= ivi_hmi_controller#{}.home(home: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            3,
            arg0,
        ]);
        Ok(())
    }

    /// request displaying/undisplaying home screen
    ///
    /// home screen is a reference implementation of launcher to launch
    /// wayland applications. The home screen has several workspaces to
    /// group wayland applications. By defining the following keys in
    /// weston.ini, user can add launcher icon to launch a wayland application
    /// to a workspace.
    /// [ivi-launcher]
    /// workspace-id=0
    ///         : id of workspace to add a launcher
    /// icon-id=4001
    ///         : ivi id of ivi_surface to draw an icon
    /// icon=/home/user/review/build-ivi-shell/data/icon_ivi_flower.png
    ///         : path to icon image
    /// path=/home/user/review/build-ivi-shell/weston-dnd
    ///         : path to wayland application
    ///
    /// # Arguments
    ///
    /// - `home`:
    #[inline]
    pub fn send_home(
        &self,
        home: u32,
    ) {
        let res = self.try_send_home(
            home,
        );
        if let Err(e) = res {
            log_send("ivi_hmi_controller.home", &e);
        }
    }

    /// Since when the workspace_end_control message is available.
    pub const MSG__WORKSPACE_END_CONTROL__SINCE: u32 = 1;

    /// notify controlling workspace end
    ///
    /// # Arguments
    ///
    /// - `is_controlled`:
    #[inline]
    pub fn try_send_workspace_end_control(
        &self,
        is_controlled: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            is_controlled,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= ivi_hmi_controller#{}.workspace_end_control(is_controlled: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
        ]);
        Ok(())
    }

    /// notify controlling workspace end
    ///
    /// # Arguments
    ///
    /// - `is_controlled`:
    #[inline]
    pub fn send_workspace_end_control(
        &self,
        is_controlled: i32,
    ) {
        let res = self.try_send_workspace_end_control(
            is_controlled,
        );
        if let Err(e) = res {
            log_send("ivi_hmi_controller.workspace_end_control", &e);
        }
    }
}

/// A message handler for [`IviHmiController`] proxies.
pub trait IviHmiControllerHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<IviHmiController>) {
        slf.core.delete_id();
    }

    /// inform the ready for drawing desktop.
    #[inline]
    fn handle_UI_ready(
        &mut self,
        slf: &Rc<IviHmiController>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_UI_ready(
        );
        if let Err(e) = res {
            log_forward("ivi_hmi_controller.UI_ready", &e);
        }
    }

    /// start controlling a surface by server
    ///
    /// Reference protocol to control a surface by server.
    /// To control a surface by server, it gives seat to the server
    /// to e.g. control Home screen. Home screen has several workspaces
    /// to group launchers of wayland application. These workspaces
    /// are drawn on a horizontally long surface to be controlled
    /// by motion of input device. E.g. A motion from right to left
    /// happens, the viewport of surface is controlled in the ivi-shell
    /// by using ivi-layout. client can recognizes the end of controlling
    /// by event "workspace_end_control".
    ///
    /// # Arguments
    ///
    /// - `seat`:
    /// - `serial`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_workspace_control(
        &mut self,
        slf: &Rc<IviHmiController>,
        seat: &Rc<WlSeat>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_workspace_control(
            seat,
            serial,
        );
        if let Err(e) = res {
            log_forward("ivi_hmi_controller.workspace_control", &e);
        }
    }

    /// request mode switch of application layout
    ///
    /// hmi-controller loaded to ivi-shall implements 4 types of layout
    /// as a reference; tiling, side by side, full_screen, and random.
    ///
    /// # Arguments
    ///
    /// - `layout_mode`:
    #[inline]
    fn handle_switch_mode(
        &mut self,
        slf: &Rc<IviHmiController>,
        layout_mode: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_switch_mode(
            layout_mode,
        );
        if let Err(e) = res {
            log_forward("ivi_hmi_controller.switch_mode", &e);
        }
    }

    /// request displaying/undisplaying home screen
    ///
    /// home screen is a reference implementation of launcher to launch
    /// wayland applications. The home screen has several workspaces to
    /// group wayland applications. By defining the following keys in
    /// weston.ini, user can add launcher icon to launch a wayland application
    /// to a workspace.
    /// [ivi-launcher]
    /// workspace-id=0
    ///         : id of workspace to add a launcher
    /// icon-id=4001
    ///         : ivi id of ivi_surface to draw an icon
    /// icon=/home/user/review/build-ivi-shell/data/icon_ivi_flower.png
    ///         : path to icon image
    /// path=/home/user/review/build-ivi-shell/weston-dnd
    ///         : path to wayland application
    ///
    /// # Arguments
    ///
    /// - `home`:
    #[inline]
    fn handle_home(
        &mut self,
        slf: &Rc<IviHmiController>,
        home: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_home(
            home,
        );
        if let Err(e) = res {
            log_forward("ivi_hmi_controller.home", &e);
        }
    }

    /// notify controlling workspace end
    ///
    /// # Arguments
    ///
    /// - `is_controlled`:
    #[inline]
    fn handle_workspace_end_control(
        &mut self,
        slf: &Rc<IviHmiController>,
        is_controlled: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_workspace_end_control(
            is_controlled,
        );
        if let Err(e) = res {
            log_forward("ivi_hmi_controller.workspace_end_control", &e);
        }
    }
}

impl ObjectPrivate for IviHmiController {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::IviHmiController, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ivi_hmi_controller#{}.UI_ready()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_UI_ready(&self);
                } else {
                    DefaultHandler.handle_UI_ready(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ivi_hmi_controller#{}.workspace_control(seat: wl_seat#{}, serial: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSeat>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("seat", o.core().interface, ObjectInterface::WlSeat)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_workspace_control(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_workspace_control(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ivi_hmi_controller#{}.switch_mode(layout_mode: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_switch_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_switch_mode(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> ivi_hmi_controller#{}.home(home: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_home(&self, arg0);
                } else {
                    DefaultHandler.handle_home(&self, arg0);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> ivi_hmi_controller#{}.workspace_end_control(is_controlled: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_workspace_end_control(&self, arg0);
                } else {
                    DefaultHandler.handle_workspace_end_control(&self, arg0);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "UI_ready",
            1 => "workspace_control",
            2 => "switch_mode",
            3 => "home",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "workspace_end_control",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for IviHmiController {
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

impl IviHmiController {
    /// Since when the layout_mode.tiling enum variant is available.
    pub const ENM__LAYOUT_MODE_TILING__SINCE: u32 = 1;
    /// Since when the layout_mode.side_by_side enum variant is available.
    pub const ENM__LAYOUT_MODE_SIDE_BY_SIDE__SINCE: u32 = 1;
    /// Since when the layout_mode.full_screen enum variant is available.
    pub const ENM__LAYOUT_MODE_FULL_SCREEN__SINCE: u32 = 1;
    /// Since when the layout_mode.random enum variant is available.
    pub const ENM__LAYOUT_MODE_RANDOM__SINCE: u32 = 1;

    /// Since when the home.off enum variant is available.
    pub const ENM__HOME_OFF__SINCE: u32 = 1;
    /// Since when the home.on enum variant is available.
    pub const ENM__HOME_ON__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IviHmiControllerLayoutMode(pub u32);

impl IviHmiControllerLayoutMode {
    pub const TILING: Self = Self(0);

    pub const SIDE_BY_SIDE: Self = Self(1);

    pub const FULL_SCREEN: Self = Self(2);

    pub const RANDOM: Self = Self(3);
}

impl Debug for IviHmiControllerLayoutMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TILING => "TILING",
            Self::SIDE_BY_SIDE => "SIDE_BY_SIDE",
            Self::FULL_SCREEN => "FULL_SCREEN",
            Self::RANDOM => "RANDOM",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IviHmiControllerHome(pub u32);

impl IviHmiControllerHome {
    pub const OFF: Self = Self(0);

    pub const ON: Self = Self(1);
}

impl Debug for IviHmiControllerHome {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::OFF => "OFF",
            Self::ON => "ON",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
