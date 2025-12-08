//! close reproduction of the xdg input capture portal
//!
//! Interface that is used to create barrier, and trigger capture and release of the pointer.
//! The inputs are sent through an EIS socket, when the cursor hit a barrier.
//! Barriers can only be placed on screen edges and need to be a straight line that cover one corner to another.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_input_capture_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandInputCaptureV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn HyprlandInputCaptureV1Handler>,
}

struct DefaultHandler;

impl HyprlandInputCaptureV1Handler for DefaultHandler { }

impl ConcreteObject for HyprlandInputCaptureV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::HyprlandInputCaptureV1;
    const INTERFACE_NAME: &str = "hyprland_input_capture_v1";
}

impl HyprlandInputCaptureV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl HyprlandInputCaptureV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandInputCaptureV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandInputCaptureV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandInputCaptureV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandInputCaptureV1 {
    /// Since when the clear_barriers message is available.
    pub const MSG__CLEAR_BARRIERS__SINCE: u32 = 1;

    /// clear every barriers registered
    ///
    /// Remove every barriers from the session, new barriers need to be send before calling enable again.
    #[inline]
    pub fn try_send_clear_barriers(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_input_capture_v1#{}.clear_barriers()\n", id);
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

    /// clear every barriers registered
    ///
    /// Remove every barriers from the session, new barriers need to be send before calling enable again.
    #[inline]
    pub fn send_clear_barriers(
        &self,
    ) {
        let res = self.try_send_clear_barriers(
        );
        if let Err(e) = res {
            log_send("hyprland_input_capture_v1.clear_barriers", &e);
        }
    }

    /// Since when the add_barrier message is available.
    pub const MSG__ADD_BARRIER__SINCE: u32 = 1;

    /// add one barrier
    ///
    /// Add one barrier to the current session, the barrier need to a line placed on the edge of the screen, and is a straight line from one corner to another.
    ///
    /// # Arguments
    ///
    /// - `zone_set`: The current zone_set
    /// - `id`: The zone id
    /// - `x1`:
    /// - `y1`:
    /// - `x2`:
    /// - `y2`:
    #[inline]
    pub fn try_send_add_barrier(
        &self,
        zone_set: u32,
        id: u32,
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            zone_set,
            id,
            x1,
            y1,
            x2,
            y2,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_input_capture_v1#{}.add_barrier(zone_set: {}, id: {}, x1: {}, y1: {}, x2: {}, y2: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3, arg4, arg5);
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
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ]);
        Ok(())
    }

    /// add one barrier
    ///
    /// Add one barrier to the current session, the barrier need to a line placed on the edge of the screen, and is a straight line from one corner to another.
    ///
    /// # Arguments
    ///
    /// - `zone_set`: The current zone_set
    /// - `id`: The zone id
    /// - `x1`:
    /// - `y1`:
    /// - `x2`:
    /// - `y2`:
    #[inline]
    pub fn send_add_barrier(
        &self,
        zone_set: u32,
        id: u32,
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
    ) {
        let res = self.try_send_add_barrier(
            zone_set,
            id,
            x1,
            y1,
            x2,
            y2,
        );
        if let Err(e) = res {
            log_send("hyprland_input_capture_v1.add_barrier", &e);
        }
    }

    /// Since when the enable message is available.
    pub const MSG__ENABLE__SINCE: u32 = 1;

    /// enable input capturing
    ///
    /// Enable the input capturing to be triggered by the cursor crossing a barrier.
    #[inline]
    pub fn try_send_enable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_input_capture_v1#{}.enable()\n", id);
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
            2,
        ]);
        Ok(())
    }

    /// enable input capturing
    ///
    /// Enable the input capturing to be triggered by the cursor crossing a barrier.
    #[inline]
    pub fn send_enable(
        &self,
    ) {
        let res = self.try_send_enable(
        );
        if let Err(e) = res {
            log_send("hyprland_input_capture_v1.enable", &e);
        }
    }

    /// Since when the disable message is available.
    pub const MSG__DISABLE__SINCE: u32 = 1;

    /// disable input capturing
    ///
    /// Disable input capturing, the crossing of a barrier will not trigger anymore input capture.
    #[inline]
    pub fn try_send_disable(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_input_capture_v1#{}.disable()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// disable input capturing
    ///
    /// Disable input capturing, the crossing of a barrier will not trigger anymore input capture.
    #[inline]
    pub fn send_disable(
        &self,
    ) {
        let res = self.try_send_disable(
        );
        if let Err(e) = res {
            log_send("hyprland_input_capture_v1.disable", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// release input capturing
    ///
    /// Release input capturing, the input are not intercepted anymore and barrier crossing will activate it again.
    ///     If x != -1 and y != -1 then the cursor is warped to the x and y coordinates.
    ///
    /// # Arguments
    ///
    /// - `activation_id`: The activation id provided when activated is called
    /// - `x`: the x position of the cursor
    /// - `y`: the y position of the cursor
    #[inline]
    pub fn try_send_release(
        &self,
        activation_id: u32,
        x: Fixed,
        y: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            activation_id,
            x,
            y,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: Fixed, arg2: Fixed) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_input_capture_v1#{}.release(activation_id: {}, x: {}, y: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2);
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
            4,
            arg0,
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
        ]);
        Ok(())
    }

    /// release input capturing
    ///
    /// Release input capturing, the input are not intercepted anymore and barrier crossing will activate it again.
    ///     If x != -1 and y != -1 then the cursor is warped to the x and y coordinates.
    ///
    /// # Arguments
    ///
    /// - `activation_id`: The activation id provided when activated is called
    /// - `x`: the x position of the cursor
    /// - `y`: the y position of the cursor
    #[inline]
    pub fn send_release(
        &self,
        activation_id: u32,
        x: Fixed,
        y: Fixed,
    ) {
        let res = self.try_send_release(
            activation_id,
            x,
            y,
        );
        if let Err(e) = res {
            log_send("hyprland_input_capture_v1.release", &e);
        }
    }

    /// Since when the eis_fd message is available.
    pub const MSG__EIS_FD__SINCE: u32 = 1;

    /// eis file descriptor
    ///
    /// This event provide the file descriptor of an eis socket where inputs will be sent when input capturing is active
    ///
    /// # Arguments
    ///
    /// - `fd`: eis socket file descriptor
    #[inline]
    pub fn try_send_eis_fd(
        &self,
        fd: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            fd,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_input_capture_v1#{}.eis_fd(fd: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0.as_raw_fd());
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// eis file descriptor
    ///
    /// This event provide the file descriptor of an eis socket where inputs will be sent when input capturing is active
    ///
    /// # Arguments
    ///
    /// - `fd`: eis socket file descriptor
    #[inline]
    pub fn send_eis_fd(
        &self,
        fd: &Rc<OwnedFd>,
    ) {
        let res = self.try_send_eis_fd(
            fd,
        );
        if let Err(e) = res {
            log_send("hyprland_input_capture_v1.eis_fd", &e);
        }
    }

    /// Since when the disabled message is available.
    pub const MSG__DISABLED__SINCE: u32 = 1;

    /// disable the session
    ///
    /// Called when the application will not receive captured input. The application can call enable to request future input capturing
    #[inline]
    pub fn try_send_disabled(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_input_capture_v1#{}.disabled()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
        ]);
        Ok(())
    }

    /// disable the session
    ///
    /// Called when the application will not receive captured input. The application can call enable to request future input capturing
    #[inline]
    pub fn send_disabled(
        &self,
    ) {
        let res = self.try_send_disabled(
        );
        if let Err(e) = res {
            log_send("hyprland_input_capture_v1.disabled", &e);
        }
    }

    /// Since when the activated message is available.
    pub const MSG__ACTIVATED__SINCE: u32 = 1;

    /// inputs has been captured
    ///
    /// Called when the application is about to receive inputs
    ///
    /// # Arguments
    ///
    /// - `activation_id`: Same number used in eis start_emulating to allow synchronisation
    /// - `x`: the x position of the cursor
    /// - `y`: the y position of the cursor
    /// - `barrier_id`: the is of the barrier that have been triggered
    #[inline]
    pub fn try_send_activated(
        &self,
        activation_id: u32,
        x: Fixed,
        y: Fixed,
        barrier_id: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            activation_id,
            x,
            y,
            barrier_id,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: Fixed, arg2: Fixed, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_input_capture_v1#{}.activated(activation_id: {}, x: {}, y: {}, barrier_id: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3);
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
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
            arg3,
        ]);
        Ok(())
    }

    /// inputs has been captured
    ///
    /// Called when the application is about to receive inputs
    ///
    /// # Arguments
    ///
    /// - `activation_id`: Same number used in eis start_emulating to allow synchronisation
    /// - `x`: the x position of the cursor
    /// - `y`: the y position of the cursor
    /// - `barrier_id`: the is of the barrier that have been triggered
    #[inline]
    pub fn send_activated(
        &self,
        activation_id: u32,
        x: Fixed,
        y: Fixed,
        barrier_id: u32,
    ) {
        let res = self.try_send_activated(
            activation_id,
            x,
            y,
            barrier_id,
        );
        if let Err(e) = res {
            log_send("hyprland_input_capture_v1.activated", &e);
        }
    }

    /// Since when the deactivated message is available.
    pub const MSG__DEACTIVATED__SINCE: u32 = 1;

    /// pointer motion
    ///
    /// Called when input capture is stopped, and inputs are no longer sent to the application
    ///
    /// # Arguments
    ///
    /// - `activation_id`: same activation id of the latest activated event
    #[inline]
    pub fn try_send_deactivated(
        &self,
        activation_id: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            activation_id,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_input_capture_v1#{}.deactivated(activation_id: {})\n", client_id, id, arg0);
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
            3,
            arg0,
        ]);
        Ok(())
    }

    /// pointer motion
    ///
    /// Called when input capture is stopped, and inputs are no longer sent to the application
    ///
    /// # Arguments
    ///
    /// - `activation_id`: same activation id of the latest activated event
    #[inline]
    pub fn send_deactivated(
        &self,
        activation_id: u32,
    ) {
        let res = self.try_send_deactivated(
            activation_id,
        );
        if let Err(e) = res {
            log_send("hyprland_input_capture_v1.deactivated", &e);
        }
    }
}

/// A message handler for [`HyprlandInputCaptureV1`] proxies.
pub trait HyprlandInputCaptureV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<HyprlandInputCaptureV1>) {
        slf.core.delete_id();
    }

    /// clear every barriers registered
    ///
    /// Remove every barriers from the session, new barriers need to be send before calling enable again.
    #[inline]
    fn handle_clear_barriers(
        &mut self,
        slf: &Rc<HyprlandInputCaptureV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_clear_barriers(
        );
        if let Err(e) = res {
            log_forward("hyprland_input_capture_v1.clear_barriers", &e);
        }
    }

    /// add one barrier
    ///
    /// Add one barrier to the current session, the barrier need to a line placed on the edge of the screen, and is a straight line from one corner to another.
    ///
    /// # Arguments
    ///
    /// - `zone_set`: The current zone_set
    /// - `id`: The zone id
    /// - `x1`:
    /// - `y1`:
    /// - `x2`:
    /// - `y2`:
    #[inline]
    fn handle_add_barrier(
        &mut self,
        slf: &Rc<HyprlandInputCaptureV1>,
        zone_set: u32,
        id: u32,
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_add_barrier(
            zone_set,
            id,
            x1,
            y1,
            x2,
            y2,
        );
        if let Err(e) = res {
            log_forward("hyprland_input_capture_v1.add_barrier", &e);
        }
    }

    /// enable input capturing
    ///
    /// Enable the input capturing to be triggered by the cursor crossing a barrier.
    #[inline]
    fn handle_enable(
        &mut self,
        slf: &Rc<HyprlandInputCaptureV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_enable(
        );
        if let Err(e) = res {
            log_forward("hyprland_input_capture_v1.enable", &e);
        }
    }

    /// disable input capturing
    ///
    /// Disable input capturing, the crossing of a barrier will not trigger anymore input capture.
    #[inline]
    fn handle_disable(
        &mut self,
        slf: &Rc<HyprlandInputCaptureV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_disable(
        );
        if let Err(e) = res {
            log_forward("hyprland_input_capture_v1.disable", &e);
        }
    }

    /// release input capturing
    ///
    /// Release input capturing, the input are not intercepted anymore and barrier crossing will activate it again.
    ///     If x != -1 and y != -1 then the cursor is warped to the x and y coordinates.
    ///
    /// # Arguments
    ///
    /// - `activation_id`: The activation id provided when activated is called
    /// - `x`: the x position of the cursor
    /// - `y`: the y position of the cursor
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<HyprlandInputCaptureV1>,
        activation_id: u32,
        x: Fixed,
        y: Fixed,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
            activation_id,
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("hyprland_input_capture_v1.release", &e);
        }
    }

    /// eis file descriptor
    ///
    /// This event provide the file descriptor of an eis socket where inputs will be sent when input capturing is active
    ///
    /// # Arguments
    ///
    /// - `fd`: eis socket file descriptor
    #[inline]
    fn handle_eis_fd(
        &mut self,
        slf: &Rc<HyprlandInputCaptureV1>,
        fd: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_eis_fd(
            fd,
        );
        if let Err(e) = res {
            log_forward("hyprland_input_capture_v1.eis_fd", &e);
        }
    }

    /// disable the session
    ///
    /// Called when the application will not receive captured input. The application can call enable to request future input capturing
    #[inline]
    fn handle_disabled(
        &mut self,
        slf: &Rc<HyprlandInputCaptureV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_disabled(
        );
        if let Err(e) = res {
            log_forward("hyprland_input_capture_v1.disabled", &e);
        }
    }

    /// inputs has been captured
    ///
    /// Called when the application is about to receive inputs
    ///
    /// # Arguments
    ///
    /// - `activation_id`: Same number used in eis start_emulating to allow synchronisation
    /// - `x`: the x position of the cursor
    /// - `y`: the y position of the cursor
    /// - `barrier_id`: the is of the barrier that have been triggered
    #[inline]
    fn handle_activated(
        &mut self,
        slf: &Rc<HyprlandInputCaptureV1>,
        activation_id: u32,
        x: Fixed,
        y: Fixed,
        barrier_id: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_activated(
            activation_id,
            x,
            y,
            barrier_id,
        );
        if let Err(e) = res {
            log_forward("hyprland_input_capture_v1.activated", &e);
        }
    }

    /// pointer motion
    ///
    /// Called when input capture is stopped, and inputs are no longer sent to the application
    ///
    /// # Arguments
    ///
    /// - `activation_id`: same activation id of the latest activated event
    #[inline]
    fn handle_deactivated(
        &mut self,
        slf: &Rc<HyprlandInputCaptureV1>,
        activation_id: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_deactivated(
            activation_id,
        );
        if let Err(e) = res {
            log_forward("hyprland_input_capture_v1.deactivated", &e);
        }
    }
}

impl ObjectPrivate for HyprlandInputCaptureV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::HyprlandInputCaptureV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_input_capture_v1#{}.clear_barriers()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_clear_barriers(&self);
                } else {
                    DefaultHandler.handle_clear_barriers(&self);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 32)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_input_capture_v1#{}.add_barrier(zone_set: {}, id: {}, x1: {}, y1: {}, x2: {}, y2: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4, arg5);
                }
                if let Some(handler) = handler {
                    (**handler).handle_add_barrier(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.handle_add_barrier(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_input_capture_v1#{}.enable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_enable(&self);
                } else {
                    DefaultHandler.handle_enable(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_input_capture_v1#{}.disable()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_disable(&self);
                } else {
                    DefaultHandler.handle_disable(&self);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: Fixed, arg2: Fixed) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_input_capture_v1#{}.release(activation_id: {}, x: {}, y: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_release(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_release(&self, arg0, arg1, arg2);
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
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd")));
                };
                let arg0 = &arg0;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_input_capture_v1#{}.eis_fd(fd: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0.as_raw_fd());
                }
                if let Some(handler) = handler {
                    (**handler).handle_eis_fd(&self, arg0);
                } else {
                    DefaultHandler.handle_eis_fd(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_input_capture_v1#{}.disabled()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_disabled(&self);
                } else {
                    DefaultHandler.handle_disabled(&self);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: Fixed, arg2: Fixed, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_input_capture_v1#{}.activated(activation_id: {}, x: {}, y: {}, barrier_id: {})\n", id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_activated(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_activated(&self, arg0, arg1, arg2, arg3);
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
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_input_capture_v1#{}.deactivated(activation_id: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_deactivated(&self, arg0);
                } else {
                    DefaultHandler.handle_deactivated(&self, arg0);
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
            0 => "clear_barriers",
            1 => "add_barrier",
            2 => "enable",
            3 => "disable",
            4 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "eis_fd",
            1 => "disabled",
            2 => "activated",
            3 => "deactivated",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for HyprlandInputCaptureV1 {
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

impl HyprlandInputCaptureV1 {
    /// Since when the error.invalid_barrier_id enum variant is available.
    pub const ENM__ERROR_INVALID_BARRIER_ID__SINCE: u32 = 1;
    /// Since when the error.invalid_barrier enum variant is available.
    pub const ENM__ERROR_INVALID_BARRIER__SINCE: u32 = 1;
    /// Since when the error.invalid_activation_id enum variant is available.
    pub const ENM__ERROR_INVALID_ACTIVATION_ID__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HyprlandInputCaptureV1Error(pub u32);

impl HyprlandInputCaptureV1Error {
    /// The barrier id already exist
    pub const INVALID_BARRIER_ID: Self = Self(0);

    /// The barrier coordinates are invalid
    pub const INVALID_BARRIER: Self = Self(1);

    /// The activation id provided is invalid
    pub const INVALID_ACTIVATION_ID: Self = Self(2);
}

impl Debug for HyprlandInputCaptureV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_BARRIER_ID => "INVALID_BARRIER_ID",
            Self::INVALID_BARRIER => "INVALID_BARRIER",
            Self::INVALID_ACTIVATION_ID => "INVALID_ACTIVATION_ID",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
