//! weston touchscreen calibration interface
//!
//! This is the global interface for calibrating a touchscreen input
//! coordinate transformation. It is recommended to make this interface
//! privileged.
//!
//! This interface can be used by a client to show a calibration pattern and
//! receive uncalibrated touch coordinates, facilitating the computation of
//! a calibration transformation that will align actual touch positions
//! on screen with their expected coordinates.
//!
//! Immediately after being bound by a client, the compositor sends the
//! touch_device events.
//!
//! The client chooses a touch device from the touch_device events, creates a
//! wl_surface and then a weston_touch_calibrator for the wl_surface and the
//! chosen touch device. The client waits for the compositor to send a
//! configure event before it starts drawing the first calibration pattern.
//! After receiving the configure event, the client will iterate drawing a
//! pattern, getting touch input via weston_touch_calibrator, and converting
//! pixel coordinates to expected touch coordinates with
//! weston_touch_calibrator.convert until it has enough correspondences to
//! compute the calibration transformation or the compositor cancels the
//! calibration.
//!
//! Once the client has successfully computed a new calibration, it can use
//! weston_touch_calibration.save request to load the new calibration into
//! the compositor. The compositor may take this new calibration into use and
//! may write it into persistent storage.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A weston_touch_calibration object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WestonTouchCalibration {
    core: ObjectCore,
    handler: HandlerHolder<dyn WestonTouchCalibrationHandler>,
}

struct DefaultHandler;

impl WestonTouchCalibrationHandler for DefaultHandler { }

impl ConcreteObject for WestonTouchCalibration {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WestonTouchCalibration;
    const INTERFACE_NAME: &str = "weston_touch_calibration";
}

impl WestonTouchCalibration {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WestonTouchCalibrationHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WestonTouchCalibrationHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WestonTouchCalibration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WestonTouchCalibration")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WestonTouchCalibration {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// unbind
    ///
    /// Destroy the binding to the global interface, does not affect any
    /// objects already created through this interface.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_touch_calibration#{}.destroy()\n", id);
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

    /// unbind
    ///
    /// Destroy the binding to the global interface, does not affect any
    /// objects already created through this interface.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("weston_touch_calibration.destroy", &e);
        }
    }

    /// Since when the create_calibrator message is available.
    pub const MSG__CREATE_CALIBRATOR__SINCE: u32 = 1;

    /// give the calibrator role to a surface
    ///
    /// This gives the calibrator role to the surface and ties it with the
    /// given touch input device.
    ///
    /// If the surface already has a role, then invalid_surface error is raised.
    ///
    /// If the device string is not one advertised with touch_device event's
    /// device argument, then invalid_device error is raised.
    ///
    /// If a weston_touch_calibrator protocol object exists in the compositor
    /// already, then already_exists error is raised. This limitation is
    /// compositor-wide and not specific to any particular client.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface to give the role to
    /// - `device`: the touch device to calibrate
    /// - `cal`: a new calibrator object
    #[inline]
    pub fn try_send_create_calibrator(
        &self,
        surface: &Rc<WlSurface>,
        device: &str,
        cal: &Rc<WestonTouchCalibrator>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            surface,
            device,
            cal,
        );
        let arg0 = arg0.core();
        let arg2_obj = arg2;
        let arg2 = arg2_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        arg2.generate_server_id(arg2_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("cal", e)))?;
        let arg2_id = arg2.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: &str, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_touch_calibration#{}.create_calibrator(surface: wl_surface#{}, device: {:?}, cal: weston_touch_calibrator#{})\n", id, arg0, arg1, arg2);
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
        ]);
        fmt.string(arg1);
        fmt.words([
            arg2_id,
        ]);
        Ok(())
    }

    /// give the calibrator role to a surface
    ///
    /// This gives the calibrator role to the surface and ties it with the
    /// given touch input device.
    ///
    /// If the surface already has a role, then invalid_surface error is raised.
    ///
    /// If the device string is not one advertised with touch_device event's
    /// device argument, then invalid_device error is raised.
    ///
    /// If a weston_touch_calibrator protocol object exists in the compositor
    /// already, then already_exists error is raised. This limitation is
    /// compositor-wide and not specific to any particular client.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface to give the role to
    /// - `device`: the touch device to calibrate
    /// - `cal`: a new calibrator object
    #[inline]
    pub fn send_create_calibrator(
        &self,
        surface: &Rc<WlSurface>,
        device: &str,
        cal: &Rc<WestonTouchCalibrator>,
    ) {
        let res = self.try_send_create_calibrator(
            surface,
            device,
            cal,
        );
        if let Err(e) = res {
            log_send("weston_touch_calibration.create_calibrator", &e);
        }
    }

    /// give the calibrator role to a surface
    ///
    /// This gives the calibrator role to the surface and ties it with the
    /// given touch input device.
    ///
    /// If the surface already has a role, then invalid_surface error is raised.
    ///
    /// If the device string is not one advertised with touch_device event's
    /// device argument, then invalid_device error is raised.
    ///
    /// If a weston_touch_calibrator protocol object exists in the compositor
    /// already, then already_exists error is raised. This limitation is
    /// compositor-wide and not specific to any particular client.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface to give the role to
    /// - `device`: the touch device to calibrate
    #[inline]
    pub fn new_try_send_create_calibrator(
        &self,
        surface: &Rc<WlSurface>,
        device: &str,
    ) -> Result<Rc<WestonTouchCalibrator>, ObjectError> {
        let cal = self.core.create_child();
        self.try_send_create_calibrator(
            surface,
            device,
            &cal,
        )?;
        Ok(cal)
    }

    /// give the calibrator role to a surface
    ///
    /// This gives the calibrator role to the surface and ties it with the
    /// given touch input device.
    ///
    /// If the surface already has a role, then invalid_surface error is raised.
    ///
    /// If the device string is not one advertised with touch_device event's
    /// device argument, then invalid_device error is raised.
    ///
    /// If a weston_touch_calibrator protocol object exists in the compositor
    /// already, then already_exists error is raised. This limitation is
    /// compositor-wide and not specific to any particular client.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface to give the role to
    /// - `device`: the touch device to calibrate
    #[inline]
    pub fn new_send_create_calibrator(
        &self,
        surface: &Rc<WlSurface>,
        device: &str,
    ) -> Rc<WestonTouchCalibrator> {
        let cal = self.core.create_child();
        self.send_create_calibrator(
            surface,
            device,
            &cal,
        );
        cal
    }

    /// Since when the save message is available.
    pub const MSG__SAVE__SINCE: u32 = 1;

    /// save calibration for a touch device
    ///
    /// This request asks the compositor to save the calibration data for the
    /// given touch input device. The compositor may ignore this request.
    ///
    /// If the device string is not one advertised with touch_device event's
    /// device argument, then invalid_device error is raised.
    ///
    /// The array must contain exactly six 'float' (the 32-bit floating
    /// point format used by the C language on the system) numbers. For a 3x3
    /// calibration matrix in the form
    /// @code
    ///         ( a b c )
    ///         ( d e f )
    ///         ( 0 0 1 )
    /// @endcode
    /// the array must contain the values { a, b, c, d, e, f }. For the
    /// definition of the coordinate spaces, see
    /// libinput_device_config_calibration_set_matrix().
    ///
    /// # Arguments
    ///
    /// - `device`: the target touch device
    /// - `matrix`: the new calibration matrix
    #[inline]
    pub fn try_send_save(
        &self,
        device: &str,
        matrix: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            device,
            matrix,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= weston_touch_calibration#{}.save(device: {:?}, matrix: {})\n", id, arg0, debug_array(arg1));
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
        fmt.string(arg0);
        fmt.array(arg1);
        Ok(())
    }

    /// save calibration for a touch device
    ///
    /// This request asks the compositor to save the calibration data for the
    /// given touch input device. The compositor may ignore this request.
    ///
    /// If the device string is not one advertised with touch_device event's
    /// device argument, then invalid_device error is raised.
    ///
    /// The array must contain exactly six 'float' (the 32-bit floating
    /// point format used by the C language on the system) numbers. For a 3x3
    /// calibration matrix in the form
    /// @code
    ///         ( a b c )
    ///         ( d e f )
    ///         ( 0 0 1 )
    /// @endcode
    /// the array must contain the values { a, b, c, d, e, f }. For the
    /// definition of the coordinate spaces, see
    /// libinput_device_config_calibration_set_matrix().
    ///
    /// # Arguments
    ///
    /// - `device`: the target touch device
    /// - `matrix`: the new calibration matrix
    #[inline]
    pub fn send_save(
        &self,
        device: &str,
        matrix: &[u8],
    ) {
        let res = self.try_send_save(
            device,
            matrix,
        );
        if let Err(e) = res {
            log_send("weston_touch_calibration.save", &e);
        }
    }

    /// Since when the touch_device message is available.
    pub const MSG__TOUCH_DEVICE__SINCE: u32 = 1;

    /// advertise a touchscreen input device
    ///
    /// When a client binds to weston_touch_calibration, one touch_device event
    /// is sent for each touchscreen that is available to be calibrated. This
    /// is the only time the event is sent. Touch devices added in the
    /// compositor will not generate events for existing
    /// weston_touch_calibration objects.
    ///
    /// An event carries the touch device identification and the associated
    /// output or head (display connector) name.
    ///
    /// On platforms using udev, the device identification is the udev sys
    /// path. It is an absolute path and starts with the sys mount point.
    ///
    /// # Arguments
    ///
    /// - `device`: the touch device identification
    /// - `head`: name of the head or display connector
    #[inline]
    pub fn try_send_touch_device(
        &self,
        device: &str,
        head: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            device,
            head,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= weston_touch_calibration#{}.touch_device(device: {:?}, head: {:?})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
        ]);
        fmt.string(arg0);
        fmt.string(arg1);
        Ok(())
    }

    /// advertise a touchscreen input device
    ///
    /// When a client binds to weston_touch_calibration, one touch_device event
    /// is sent for each touchscreen that is available to be calibrated. This
    /// is the only time the event is sent. Touch devices added in the
    /// compositor will not generate events for existing
    /// weston_touch_calibration objects.
    ///
    /// An event carries the touch device identification and the associated
    /// output or head (display connector) name.
    ///
    /// On platforms using udev, the device identification is the udev sys
    /// path. It is an absolute path and starts with the sys mount point.
    ///
    /// # Arguments
    ///
    /// - `device`: the touch device identification
    /// - `head`: name of the head or display connector
    #[inline]
    pub fn send_touch_device(
        &self,
        device: &str,
        head: &str,
    ) {
        let res = self.try_send_touch_device(
            device,
            head,
        );
        if let Err(e) = res {
            log_send("weston_touch_calibration.touch_device", &e);
        }
    }
}

/// A message handler for [`WestonTouchCalibration`] proxies.
pub trait WestonTouchCalibrationHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WestonTouchCalibration>) {
        slf.core.delete_id();
    }

    /// unbind
    ///
    /// Destroy the binding to the global interface, does not affect any
    /// objects already created through this interface.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WestonTouchCalibration>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibration.destroy", &e);
        }
    }

    /// give the calibrator role to a surface
    ///
    /// This gives the calibrator role to the surface and ties it with the
    /// given touch input device.
    ///
    /// If the surface already has a role, then invalid_surface error is raised.
    ///
    /// If the device string is not one advertised with touch_device event's
    /// device argument, then invalid_device error is raised.
    ///
    /// If a weston_touch_calibrator protocol object exists in the compositor
    /// already, then already_exists error is raised. This limitation is
    /// compositor-wide and not specific to any particular client.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface to give the role to
    /// - `device`: the touch device to calibrate
    /// - `cal`: a new calibrator object
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_create_calibrator(
        &mut self,
        slf: &Rc<WestonTouchCalibration>,
        surface: &Rc<WlSurface>,
        device: &str,
        cal: &Rc<WestonTouchCalibrator>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_calibrator(
            surface,
            device,
            cal,
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibration.create_calibrator", &e);
        }
    }

    /// save calibration for a touch device
    ///
    /// This request asks the compositor to save the calibration data for the
    /// given touch input device. The compositor may ignore this request.
    ///
    /// If the device string is not one advertised with touch_device event's
    /// device argument, then invalid_device error is raised.
    ///
    /// The array must contain exactly six 'float' (the 32-bit floating
    /// point format used by the C language on the system) numbers. For a 3x3
    /// calibration matrix in the form
    /// @code
    ///         ( a b c )
    ///         ( d e f )
    ///         ( 0 0 1 )
    /// @endcode
    /// the array must contain the values { a, b, c, d, e, f }. For the
    /// definition of the coordinate spaces, see
    /// libinput_device_config_calibration_set_matrix().
    ///
    /// # Arguments
    ///
    /// - `device`: the target touch device
    /// - `matrix`: the new calibration matrix
    #[inline]
    fn handle_save(
        &mut self,
        slf: &Rc<WestonTouchCalibration>,
        device: &str,
        matrix: &[u8],
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_save(
            device,
            matrix,
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibration.save", &e);
        }
    }

    /// advertise a touchscreen input device
    ///
    /// When a client binds to weston_touch_calibration, one touch_device event
    /// is sent for each touchscreen that is available to be calibrated. This
    /// is the only time the event is sent. Touch devices added in the
    /// compositor will not generate events for existing
    /// weston_touch_calibration objects.
    ///
    /// An event carries the touch device identification and the associated
    /// output or head (display connector) name.
    ///
    /// On platforms using udev, the device identification is the udev sys
    /// path. It is an absolute path and starts with the sys mount point.
    ///
    /// # Arguments
    ///
    /// - `device`: the touch device identification
    /// - `head`: name of the head or display connector
    #[inline]
    fn handle_touch_device(
        &mut self,
        slf: &Rc<WestonTouchCalibration>,
        device: &str,
        head: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_touch_device(
            device,
            head,
        );
        if let Err(e) = res {
            log_forward("weston_touch_calibration.touch_device", &e);
        }
    }
}

impl ObjectPrivate for WestonTouchCalibration {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WestonTouchCalibration, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_touch_calibration#{}.destroy()\n", client_id, id);
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("surface")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "device")?;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("cal")));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: &str, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_touch_calibration#{}.create_calibrator(surface: wl_surface#{}, device: {:?}, cal: weston_touch_calibrator#{})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg2_id = arg2;
                let arg2 = WestonTouchCalibrator::new(&self.core.state, self.core.version);
                arg2.core().set_client_id(client, arg2_id, arg2.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg2_id, "cal", e)))?;
                let arg0 = &arg0;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).handle_create_calibrator(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_create_calibrator(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "device")?;
                let arg1;
                (arg1, offset) = parse_array(msg, offset, "matrix")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> weston_touch_calibration#{}.save(device: {:?}, matrix: {})\n", client_id, id, arg0, debug_array(arg1));
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_save(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_save(&self, arg0, arg1);
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "device")?;
                let arg1;
                (arg1, offset) = parse_string::<NonNullString>(msg, offset, "head")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str, arg1: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> weston_touch_calibration#{}.touch_device(device: {:?}, head: {:?})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_touch_device(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_touch_device(&self, arg0, arg1);
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
            0 => "destroy",
            1 => "create_calibrator",
            2 => "save",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "touch_device",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WestonTouchCalibration {
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

impl WestonTouchCalibration {
    /// Since when the error.invalid_surface enum variant is available.
    pub const ENM__ERROR_INVALID_SURFACE__SINCE: u32 = 1;
    /// Since when the error.invalid_device enum variant is available.
    pub const ENM__ERROR_INVALID_DEVICE__SINCE: u32 = 1;
    /// Since when the error.already_exists enum variant is available.
    pub const ENM__ERROR_ALREADY_EXISTS__SINCE: u32 = 1;
}

/// global interface errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WestonTouchCalibrationError(pub u32);

impl WestonTouchCalibrationError {
    /// the given wl_surface already has a role
    pub const INVALID_SURFACE: Self = Self(0);

    /// the given device is not valid
    pub const INVALID_DEVICE: Self = Self(1);

    /// a calibrator has already been created
    pub const ALREADY_EXISTS: Self = Self(2);
}

impl Debug for WestonTouchCalibrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SURFACE => "INVALID_SURFACE",
            Self::INVALID_DEVICE => "INVALID_DEVICE",
            Self::ALREADY_EXISTS => "ALREADY_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
