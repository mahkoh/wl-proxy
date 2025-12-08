//! color representation extension to a surface
//!
//! A wp_color_representation_surface_v1 allows the client to set the color
//! representation metadata of a surface.
//!
//! By default, a surface does not have any color representation metadata set.
//! The reconstruction of R, G, B signals on such surfaces is compositor
//! implementation defined. The alpha mode is assumed to be
//! premultiplied_electrical when the alpha mode is unset.
//!
//! If the wl_surface associated with the wp_color_representation_surface_v1
//! is destroyed, the wp_color_representation_surface_v1 object becomes inert.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_color_representation_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpColorRepresentationSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpColorRepresentationSurfaceV1Handler>,
}

struct DefaultHandler;

impl WpColorRepresentationSurfaceV1Handler for DefaultHandler { }

impl ConcreteObject for WpColorRepresentationSurfaceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpColorRepresentationSurfaceV1;
    const INTERFACE_NAME: &str = "wp_color_representation_surface_v1";
}

impl WpColorRepresentationSurfaceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpColorRepresentationSurfaceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpColorRepresentationSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpColorRepresentationSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpColorRepresentationSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpColorRepresentationSurfaceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the color representation
    ///
    /// Destroy the wp_color_representation_surface_v1 object.
    ///
    /// Destroying this object unsets all the color representation metadata from
    /// the surface. See the wp_color_representation_surface_v1 interface
    /// description for how a compositor handles a surface without color
    /// representation metadata. Unsetting is double-buffered state, see
    /// wl_surface.commit.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_representation_surface_v1#{}.destroy()\n", id);
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

    /// destroy the color representation
    ///
    /// Destroy the wp_color_representation_surface_v1 object.
    ///
    /// Destroying this object unsets all the color representation metadata from
    /// the surface. See the wp_color_representation_surface_v1 interface
    /// description for how a compositor handles a surface without color
    /// representation metadata. Unsetting is double-buffered state, see
    /// wl_surface.commit.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_color_representation_surface_v1.destroy", &e);
        }
    }

    /// Since when the set_alpha_mode message is available.
    pub const MSG__SET_ALPHA_MODE__SINCE: u32 = 1;

    /// set the surface alpha mode
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Assuming an alpha channel exists, it is always linear. The alpha mode
    /// determines whether and how the color channels include pre-multiplied
    /// alpha. Using straight alpha might have performance benefits.
    ///
    /// Only alpha modes advertised by the compositor are allowed to be used as
    /// argument for this request. The "alpha_mode" protocol error is raised
    /// otherwise.
    ///
    /// Alpha mode is double buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `alpha_mode`: alpha mode
    #[inline]
    pub fn try_send_set_alpha_mode(
        &self,
        alpha_mode: WpColorRepresentationSurfaceV1AlphaMode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            alpha_mode,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WpColorRepresentationSurfaceV1AlphaMode) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_representation_surface_v1#{}.set_alpha_mode(alpha_mode: {:?})\n", id, arg0);
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
            1,
            arg0.0,
        ]);
        Ok(())
    }

    /// set the surface alpha mode
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Assuming an alpha channel exists, it is always linear. The alpha mode
    /// determines whether and how the color channels include pre-multiplied
    /// alpha. Using straight alpha might have performance benefits.
    ///
    /// Only alpha modes advertised by the compositor are allowed to be used as
    /// argument for this request. The "alpha_mode" protocol error is raised
    /// otherwise.
    ///
    /// Alpha mode is double buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `alpha_mode`: alpha mode
    #[inline]
    pub fn send_set_alpha_mode(
        &self,
        alpha_mode: WpColorRepresentationSurfaceV1AlphaMode,
    ) {
        let res = self.try_send_set_alpha_mode(
            alpha_mode,
        );
        if let Err(e) = res {
            log_send("wp_color_representation_surface_v1.set_alpha_mode", &e);
        }
    }

    /// Since when the set_coefficients_and_range message is available.
    pub const MSG__SET_COEFFICIENTS_AND_RANGE__SINCE: u32 = 1;

    /// set the matrix coefficients and range
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Set the matrix coefficients and video range which defines the formula
    /// and the related constants used to derive red, green and blue signals.
    /// Usually coefficients correspond to MatrixCoefficients code points in
    /// H.273.
    ///
    /// Only combinations advertised by the compositor are allowed to be used as
    /// argument for this request. The "coefficients" protocol error is raised
    /// otherwise.
    ///
    /// A call to wl_surface.commit verifies that the pixel format and the
    /// coefficients-range combination in the committed surface contents are
    /// compatible, if contents exist. The "pixel_format" protocol error is
    /// raised otherwise.
    ///
    /// A pixel format is compatible with the coefficients-range combination if
    /// the related equations and conventions as defined in H.273 can produce
    /// the color channels (RGB or YCbCr) of the pixel format.
    ///
    /// For the definition of the supported combination, see the
    /// wp_color_representation_surface_v1::coefficients and
    /// wp_color_representation_surface_v1::range enums.
    ///
    /// The coefficients-range combination is double-buffered, see
    /// wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `coefficients`: matrix coefficients
    /// - `range`: range
    #[inline]
    pub fn try_send_set_coefficients_and_range(
        &self,
        coefficients: WpColorRepresentationSurfaceV1Coefficients,
        range: WpColorRepresentationSurfaceV1Range,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            coefficients,
            range,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WpColorRepresentationSurfaceV1Coefficients, arg1: WpColorRepresentationSurfaceV1Range) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_representation_surface_v1#{}.set_coefficients_and_range(coefficients: {:?}, range: {:?})\n", id, arg0, arg1);
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
            arg0.0,
            arg1.0,
        ]);
        Ok(())
    }

    /// set the matrix coefficients and range
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Set the matrix coefficients and video range which defines the formula
    /// and the related constants used to derive red, green and blue signals.
    /// Usually coefficients correspond to MatrixCoefficients code points in
    /// H.273.
    ///
    /// Only combinations advertised by the compositor are allowed to be used as
    /// argument for this request. The "coefficients" protocol error is raised
    /// otherwise.
    ///
    /// A call to wl_surface.commit verifies that the pixel format and the
    /// coefficients-range combination in the committed surface contents are
    /// compatible, if contents exist. The "pixel_format" protocol error is
    /// raised otherwise.
    ///
    /// A pixel format is compatible with the coefficients-range combination if
    /// the related equations and conventions as defined in H.273 can produce
    /// the color channels (RGB or YCbCr) of the pixel format.
    ///
    /// For the definition of the supported combination, see the
    /// wp_color_representation_surface_v1::coefficients and
    /// wp_color_representation_surface_v1::range enums.
    ///
    /// The coefficients-range combination is double-buffered, see
    /// wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `coefficients`: matrix coefficients
    /// - `range`: range
    #[inline]
    pub fn send_set_coefficients_and_range(
        &self,
        coefficients: WpColorRepresentationSurfaceV1Coefficients,
        range: WpColorRepresentationSurfaceV1Range,
    ) {
        let res = self.try_send_set_coefficients_and_range(
            coefficients,
            range,
        );
        if let Err(e) = res {
            log_send("wp_color_representation_surface_v1.set_coefficients_and_range", &e);
        }
    }

    /// Since when the set_chroma_location message is available.
    pub const MSG__SET_CHROMA_LOCATION__SINCE: u32 = 1;

    /// set the chroma location
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Set the chroma location type which defines the position of downsampled
    /// chroma samples, corresponding to Chroma420SampleLocType code points in
    /// H.273.
    ///
    /// An invalid chroma location enum value raises the "chroma_location"
    /// protocol error.
    ///
    /// A call to wl_surface.commit verifies that the pixel format and chroma
    /// location type in the committed surface contents are compatible, if
    /// contents exist. The "pixel_format" protocol error is raised otherwise.
    ///
    /// For the definition of the supported chroma location types, see the
    /// wp_color_representation_surface_v1::chroma_location enum.
    ///
    /// The chroma location type is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `chroma_location`: chroma sample location
    #[inline]
    pub fn try_send_set_chroma_location(
        &self,
        chroma_location: WpColorRepresentationSurfaceV1ChromaLocation,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            chroma_location,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WpColorRepresentationSurfaceV1ChromaLocation) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_color_representation_surface_v1#{}.set_chroma_location(chroma_location: {:?})\n", id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// set the chroma location
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Set the chroma location type which defines the position of downsampled
    /// chroma samples, corresponding to Chroma420SampleLocType code points in
    /// H.273.
    ///
    /// An invalid chroma location enum value raises the "chroma_location"
    /// protocol error.
    ///
    /// A call to wl_surface.commit verifies that the pixel format and chroma
    /// location type in the committed surface contents are compatible, if
    /// contents exist. The "pixel_format" protocol error is raised otherwise.
    ///
    /// For the definition of the supported chroma location types, see the
    /// wp_color_representation_surface_v1::chroma_location enum.
    ///
    /// The chroma location type is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `chroma_location`: chroma sample location
    #[inline]
    pub fn send_set_chroma_location(
        &self,
        chroma_location: WpColorRepresentationSurfaceV1ChromaLocation,
    ) {
        let res = self.try_send_set_chroma_location(
            chroma_location,
        );
        if let Err(e) = res {
            log_send("wp_color_representation_surface_v1.set_chroma_location", &e);
        }
    }
}

/// A message handler for [`WpColorRepresentationSurfaceV1`] proxies.
pub trait WpColorRepresentationSurfaceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpColorRepresentationSurfaceV1>) {
        slf.core.delete_id();
    }

    /// destroy the color representation
    ///
    /// Destroy the wp_color_representation_surface_v1 object.
    ///
    /// Destroying this object unsets all the color representation metadata from
    /// the surface. See the wp_color_representation_surface_v1 interface
    /// description for how a compositor handles a surface without color
    /// representation metadata. Unsetting is double-buffered state, see
    /// wl_surface.commit.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpColorRepresentationSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_color_representation_surface_v1.destroy", &e);
        }
    }

    /// set the surface alpha mode
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Assuming an alpha channel exists, it is always linear. The alpha mode
    /// determines whether and how the color channels include pre-multiplied
    /// alpha. Using straight alpha might have performance benefits.
    ///
    /// Only alpha modes advertised by the compositor are allowed to be used as
    /// argument for this request. The "alpha_mode" protocol error is raised
    /// otherwise.
    ///
    /// Alpha mode is double buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `alpha_mode`: alpha mode
    #[inline]
    fn handle_set_alpha_mode(
        &mut self,
        slf: &Rc<WpColorRepresentationSurfaceV1>,
        alpha_mode: WpColorRepresentationSurfaceV1AlphaMode,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_alpha_mode(
            alpha_mode,
        );
        if let Err(e) = res {
            log_forward("wp_color_representation_surface_v1.set_alpha_mode", &e);
        }
    }

    /// set the matrix coefficients and range
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Set the matrix coefficients and video range which defines the formula
    /// and the related constants used to derive red, green and blue signals.
    /// Usually coefficients correspond to MatrixCoefficients code points in
    /// H.273.
    ///
    /// Only combinations advertised by the compositor are allowed to be used as
    /// argument for this request. The "coefficients" protocol error is raised
    /// otherwise.
    ///
    /// A call to wl_surface.commit verifies that the pixel format and the
    /// coefficients-range combination in the committed surface contents are
    /// compatible, if contents exist. The "pixel_format" protocol error is
    /// raised otherwise.
    ///
    /// A pixel format is compatible with the coefficients-range combination if
    /// the related equations and conventions as defined in H.273 can produce
    /// the color channels (RGB or YCbCr) of the pixel format.
    ///
    /// For the definition of the supported combination, see the
    /// wp_color_representation_surface_v1::coefficients and
    /// wp_color_representation_surface_v1::range enums.
    ///
    /// The coefficients-range combination is double-buffered, see
    /// wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `coefficients`: matrix coefficients
    /// - `range`: range
    #[inline]
    fn handle_set_coefficients_and_range(
        &mut self,
        slf: &Rc<WpColorRepresentationSurfaceV1>,
        coefficients: WpColorRepresentationSurfaceV1Coefficients,
        range: WpColorRepresentationSurfaceV1Range,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_coefficients_and_range(
            coefficients,
            range,
        );
        if let Err(e) = res {
            log_forward("wp_color_representation_surface_v1.set_coefficients_and_range", &e);
        }
    }

    /// set the chroma location
    ///
    /// If this protocol object is inert, the protocol error inert is raised.
    ///
    /// Set the chroma location type which defines the position of downsampled
    /// chroma samples, corresponding to Chroma420SampleLocType code points in
    /// H.273.
    ///
    /// An invalid chroma location enum value raises the "chroma_location"
    /// protocol error.
    ///
    /// A call to wl_surface.commit verifies that the pixel format and chroma
    /// location type in the committed surface contents are compatible, if
    /// contents exist. The "pixel_format" protocol error is raised otherwise.
    ///
    /// For the definition of the supported chroma location types, see the
    /// wp_color_representation_surface_v1::chroma_location enum.
    ///
    /// The chroma location type is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `chroma_location`: chroma sample location
    #[inline]
    fn handle_set_chroma_location(
        &mut self,
        slf: &Rc<WpColorRepresentationSurfaceV1>,
        chroma_location: WpColorRepresentationSurfaceV1ChromaLocation,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_chroma_location(
            chroma_location,
        );
        if let Err(e) = res {
            log_forward("wp_color_representation_surface_v1.set_chroma_location", &e);
        }
    }
}

impl ObjectPrivate for WpColorRepresentationSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpColorRepresentationSurfaceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_representation_surface_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WpColorRepresentationSurfaceV1AlphaMode(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WpColorRepresentationSurfaceV1AlphaMode) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_representation_surface_v1#{}.set_alpha_mode(alpha_mode: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_alpha_mode(&self, arg0);
                } else {
                    DefaultHandler.handle_set_alpha_mode(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = WpColorRepresentationSurfaceV1Coefficients(arg0);
                let arg1 = WpColorRepresentationSurfaceV1Range(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WpColorRepresentationSurfaceV1Coefficients, arg1: WpColorRepresentationSurfaceV1Range) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_representation_surface_v1#{}.set_coefficients_and_range(coefficients: {:?}, range: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_coefficients_and_range(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_coefficients_and_range(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WpColorRepresentationSurfaceV1ChromaLocation(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WpColorRepresentationSurfaceV1ChromaLocation) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_color_representation_surface_v1#{}.set_chroma_location(chroma_location: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_chroma_location(&self, arg0);
                } else {
                    DefaultHandler.handle_set_chroma_location(&self, arg0);
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
            1 => "set_alpha_mode",
            2 => "set_coefficients_and_range",
            3 => "set_chroma_location",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpColorRepresentationSurfaceV1 {
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

impl WpColorRepresentationSurfaceV1 {
    /// Since when the error.alpha_mode enum variant is available.
    pub const ENM__ERROR_ALPHA_MODE__SINCE: u32 = 1;
    /// Since when the error.coefficients enum variant is available.
    pub const ENM__ERROR_COEFFICIENTS__SINCE: u32 = 1;
    /// Since when the error.pixel_format enum variant is available.
    pub const ENM__ERROR_PIXEL_FORMAT__SINCE: u32 = 1;
    /// Since when the error.inert enum variant is available.
    pub const ENM__ERROR_INERT__SINCE: u32 = 1;
    /// Since when the error.chroma_location enum variant is available.
    pub const ENM__ERROR_CHROMA_LOCATION__SINCE: u32 = 1;

    /// Since when the alpha_mode.premultiplied_electrical enum variant is available.
    pub const ENM__ALPHA_MODE_PREMULTIPLIED_ELECTRICAL__SINCE: u32 = 1;
    /// Since when the alpha_mode.premultiplied_optical enum variant is available.
    pub const ENM__ALPHA_MODE_PREMULTIPLIED_OPTICAL__SINCE: u32 = 1;
    /// Since when the alpha_mode.straight enum variant is available.
    pub const ENM__ALPHA_MODE_STRAIGHT__SINCE: u32 = 1;

    /// Since when the coefficients.identity enum variant is available.
    pub const ENM__COEFFICIENTS_IDENTITY__SINCE: u32 = 1;
    /// Since when the coefficients.bt709 enum variant is available.
    pub const ENM__COEFFICIENTS_BT709__SINCE: u32 = 1;
    /// Since when the coefficients.fcc enum variant is available.
    pub const ENM__COEFFICIENTS_FCC__SINCE: u32 = 1;
    /// Since when the coefficients.bt601 enum variant is available.
    pub const ENM__COEFFICIENTS_BT601__SINCE: u32 = 1;
    /// Since when the coefficients.smpte240 enum variant is available.
    pub const ENM__COEFFICIENTS_SMPTE240__SINCE: u32 = 1;
    /// Since when the coefficients.bt2020 enum variant is available.
    pub const ENM__COEFFICIENTS_BT2020__SINCE: u32 = 1;
    /// Since when the coefficients.bt2020_cl enum variant is available.
    pub const ENM__COEFFICIENTS_BT2020_CL__SINCE: u32 = 1;
    /// Since when the coefficients.ictcp enum variant is available.
    pub const ENM__COEFFICIENTS_ICTCP__SINCE: u32 = 1;

    /// Since when the range.full enum variant is available.
    pub const ENM__RANGE_FULL__SINCE: u32 = 1;
    /// Since when the range.limited enum variant is available.
    pub const ENM__RANGE_LIMITED__SINCE: u32 = 1;

    /// Since when the chroma_location.type_0 enum variant is available.
    pub const ENM__CHROMA_LOCATION_TYPE_0__SINCE: u32 = 1;
    /// Since when the chroma_location.type_1 enum variant is available.
    pub const ENM__CHROMA_LOCATION_TYPE_1__SINCE: u32 = 1;
    /// Since when the chroma_location.type_2 enum variant is available.
    pub const ENM__CHROMA_LOCATION_TYPE_2__SINCE: u32 = 1;
    /// Since when the chroma_location.type_3 enum variant is available.
    pub const ENM__CHROMA_LOCATION_TYPE_3__SINCE: u32 = 1;
    /// Since when the chroma_location.type_4 enum variant is available.
    pub const ENM__CHROMA_LOCATION_TYPE_4__SINCE: u32 = 1;
    /// Since when the chroma_location.type_5 enum variant is available.
    pub const ENM__CHROMA_LOCATION_TYPE_5__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorRepresentationSurfaceV1Error(pub u32);

impl WpColorRepresentationSurfaceV1Error {
    /// unsupported alpha mode
    pub const ALPHA_MODE: Self = Self(1);

    /// unsupported coefficients
    pub const COEFFICIENTS: Self = Self(2);

    /// the pixel format and a set value are incompatible
    pub const PIXEL_FORMAT: Self = Self(3);

    /// forbidden request on inert object
    pub const INERT: Self = Self(4);

    /// invalid chroma location
    pub const CHROMA_LOCATION: Self = Self(5);
}

impl Debug for WpColorRepresentationSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ALPHA_MODE => "ALPHA_MODE",
            Self::COEFFICIENTS => "COEFFICIENTS",
            Self::PIXEL_FORMAT => "PIXEL_FORMAT",
            Self::INERT => "INERT",
            Self::CHROMA_LOCATION => "CHROMA_LOCATION",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// alpha mode
///
/// Specifies how the alpha channel affects the color channels.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorRepresentationSurfaceV1AlphaMode(pub u32);

impl WpColorRepresentationSurfaceV1AlphaMode {
    /// premultiplied alpha in electrical values
    ///
    /// Electrical color channel values (after transfer function encoding)
    /// are already multiplied with the alpha channel value.
    pub const PREMULTIPLIED_ELECTRICAL: Self = Self(0);

    /// premultiplied alpha in optical values
    ///
    /// Optical color channel values (before transfer function encoding)
    /// are already multiplied with the alpha channel value.
    pub const PREMULTIPLIED_OPTICAL: Self = Self(1);

    /// straight alpha
    ///
    /// Alpha channel has not been pre-multiplied into color channels.
    pub const STRAIGHT: Self = Self(2);
}

impl Debug for WpColorRepresentationSurfaceV1AlphaMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::PREMULTIPLIED_ELECTRICAL => "PREMULTIPLIED_ELECTRICAL",
            Self::PREMULTIPLIED_OPTICAL => "PREMULTIPLIED_OPTICAL",
            Self::STRAIGHT => "STRAIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// named coefficients
///
/// Named matrix coefficients used to encode well-known sets of
/// coefficients. H.273 is the authority, when it comes to the exact values
/// of coefficients and authoritative specifications, where an equivalent
/// code point exists.
///
/// A value of 0 is invalid and will never be present in the list of enums.
///
/// Descriptions do list the specifications for convenience.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorRepresentationSurfaceV1Coefficients(pub u32);

impl WpColorRepresentationSurfaceV1Coefficients {
    /// The identity matrix
    ///
    /// Coefficients as defined by
    /// - IEC 61966-2-1 sRGB
    /// - SMPTE ST 428-1 (2019)
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 0.
    /// Compatible with pixel formats of the RGB family.
    pub const IDENTITY: Self = Self(1);

    /// BT.709 matrix coefficients
    ///
    /// Coefficients as defined by
    /// - Rec. ITU-R BT.709-6
    /// - Rec. ITU-R BT.1361-0 conventional colour gamut system (historical)
    /// - Rec. ITU-R BT.1361-0 conventional colour gamut system and extended
    ///   colour gamut system (historical)
    /// - IEC 61966-2-4 xvYCC709
    /// - SMPTE RP 177 (1993) Annex B
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 1.
    /// Compatible with pixel formats of the YCbCr family.
    pub const BT709: Self = Self(2);

    /// FCC matrix coefficients
    ///
    /// Coefficients as defined by
    /// - United States Federal Communications Commission (2003) Title 47
    ///   Code of Federal Regulations 73.682 (a) (20)
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 4.
    /// Compatible with pixel formats of the YCbCr family.
    pub const FCC: Self = Self(3);

    /// BT.601-7 matrix coefficients
    ///
    /// Coefficients as defined by
    /// - Rec. ITU-R BT.470-6 System B, G (historical)
    /// - Rec. ITU-R BT.601-7 625
    /// - Rec. ITU-R BT.601-7 525
    /// - Rec. ITU-R BT.1358-0 625 (historical)
    /// - Rec. ITU-R BT.1358-1 525 or 625 (historical)
    /// - Rec. ITU-R BT.1700-0 625 PAL and 625 SECAM
    /// - Rec. ITU-R BT.1700-0 NTSC
    /// - IEC 61966-2-1 sYCC
    /// - IEC 61966-2-4 xvYCC601
    /// - SMPTE ST 170 (2004)
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 5, 6.
    /// Compatible with pixel formats of the YCbCr family.
    pub const BT601: Self = Self(4);

    /// SMPTE ST 240 matrix coefficients
    ///
    /// Coefficients as defined by
    /// - SMPTE ST 240 (1999)
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 7.
    /// Compatible with pixel formats of the YCbCr family.
    pub const SMPTE240: Self = Self(5);

    /// BT.2020 and BT.2100 YCbCr matrix coefficients
    ///
    /// Coefficients as defined by
    /// - Rec. ITU-R BT.2020-2 (non-constant luminance)
    /// - Rec. ITU-R BT.2100-2 Y′CbCr
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 9.
    /// Compatible with pixel formats of the YCbCr family.
    pub const BT2020: Self = Self(6);

    /// BT.2020 matrix coefficients for constant luminance
    ///
    /// Coefficients as defined by
    /// - Rec. ITU-R BT.2020-2 (constant luminance)
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 10.
    /// Compatible with pixel formats of the YCbCr family.
    pub const BT2020_CL: Self = Self(7);

    /// BT.2100 ICtCp matrix coefficients
    ///
    /// Coefficients as defined by
    /// - Rec. ITU-R BT.2100-2 ICTCP
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 14.
    /// Compatible with pixel formats of the YCbCr family.
    pub const ICTCP: Self = Self(8);
}

impl Debug for WpColorRepresentationSurfaceV1Coefficients {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::IDENTITY => "IDENTITY",
            Self::BT709 => "BT709",
            Self::FCC => "FCC",
            Self::BT601 => "BT601",
            Self::SMPTE240 => "SMPTE240",
            Self::BT2020 => "BT2020",
            Self::BT2020_CL => "BT2020_CL",
            Self::ICTCP => "ICTCP",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Color range values
///
/// Possible color range values.
///
/// A value of 0 is invalid and will never be present in the list of enums.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorRepresentationSurfaceV1Range(pub u32);

impl WpColorRepresentationSurfaceV1Range {
    /// Full color range
    pub const FULL: Self = Self(1);

    /// Limited color range
    pub const LIMITED: Self = Self(2);
}

impl Debug for WpColorRepresentationSurfaceV1Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::FULL => "FULL",
            Self::LIMITED => "LIMITED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Chroma sample location for 4:2:0 YCbCr
///
/// Chroma sample location as defined by H.273 Chroma420SampleLocType.
///
/// A value of 0 is invalid and will never be present in the list of enums.
///
/// The descriptions list the matching Vulkan VkChromaLocation combinations
/// for convenience.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpColorRepresentationSurfaceV1ChromaLocation(pub u32);

impl WpColorRepresentationSurfaceV1ChromaLocation {
    /// Horizontal offset of 0, vertical offset of 0.5
    ///
    /// Corresponding to VkChromaLocations:
    /// - xChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
    /// - yChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 0.
    pub const TYPE_0: Self = Self(1);

    /// Horizontal offset of 0.5, vertical offset of 0.5
    ///
    /// Corresponding to VkChromaLocations:
    /// - xChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
    /// - yChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 1.
    pub const TYPE_1: Self = Self(2);

    /// Horizontal offset of 0, vertical offset of 0
    ///
    /// Corresponding to VkChromaLocations:
    /// - xChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
    /// - yChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 2.
    pub const TYPE_2: Self = Self(3);

    /// Horizontal offset of 0.5, vertical offset of 0
    ///
    /// Corresponding to VkChromaLocations:
    /// - xChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
    /// - yChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 3.
    pub const TYPE_3: Self = Self(4);

    /// Horizontal offset of 0, vertical offset of 1
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 4.
    pub const TYPE_4: Self = Self(5);

    /// Horizontal offset of 0.5, vertical offset of 1
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 5.
    pub const TYPE_5: Self = Self(6);
}

impl Debug for WpColorRepresentationSurfaceV1ChromaLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TYPE_0 => "TYPE_0",
            Self::TYPE_1 => "TYPE_1",
            Self::TYPE_2 => "TYPE_2",
            Self::TYPE_3 => "TYPE_3",
            Self::TYPE_4 => "TYPE_4",
            Self::TYPE_5 => "TYPE_5",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
