//! Colorimetric image description information
//!
//! Sends all matching events describing an image description object exactly
//! once and finally sends the 'done' event.
//!
//! This means
//! - if the image description is parametric, it must send
//!   - primaries
//!   - named_primaries, if applicable
//!   - at least one of tf_power and tf_named, as applicable
//!   - luminances
//!   - target_primaries
//!   - target_luminance
//! - if the image description is parametric, it may send, if applicable,
//!   - target_max_cll
//!   - target_max_fall
//! - if the image description contains an ICC profile, it must send the
//!   icc_file event
//!
//! Once a wp_image_description_info_v1 object has delivered a 'done' event it
//! is automatically destroyed.
//!
//! Every wp_image_description_info_v1 created from the same
//! wp_image_description_v1 shall always return the exact same data.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_image_description_info_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpImageDescriptionInfoV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpImageDescriptionInfoV1Handler>,
}

struct DefaultHandler;

impl WpImageDescriptionInfoV1Handler for DefaultHandler { }

impl ConcreteObject for WpImageDescriptionInfoV1 {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WpImageDescriptionInfoV1;
    const INTERFACE_NAME: &str = "wp_image_description_info_v1";
}

impl WpImageDescriptionInfoV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpImageDescriptionInfoV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpImageDescriptionInfoV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpImageDescriptionInfoV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpImageDescriptionInfoV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpImageDescriptionInfoV1 {
    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// end of information
    ///
    /// Signals the end of information events and destroys the object.
    #[inline]
    pub fn try_send_done(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.done()\n", client_id, id);
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
            0,
        ]);
        drop(fmt);
        drop(outgoing_ref);
        drop(client_ref);
        self.core.handle_client_destroy();
        Ok(())
    }

    /// end of information
    ///
    /// Signals the end of information events and destroys the object.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.done", &e);
        }
    }

    /// Since when the icc_file message is available.
    pub const MSG__ICC_FILE__SINCE: u32 = 1;

    /// ICC profile matching the image description
    ///
    /// The icc argument provides a file descriptor to the client which may be
    /// memory-mapped to provide the ICC profile matching the image description.
    /// The fd is read-only, and if mapped then it must be mapped with
    /// MAP_PRIVATE by the client.
    ///
    /// The ICC profile version and other details are determined by the
    /// compositor. There is no provision for a client to ask for a specific
    /// kind of a profile.
    ///
    /// # Arguments
    ///
    /// - `icc`: ICC profile file descriptor
    /// - `icc_size`: ICC profile size, in bytes
    #[inline]
    pub fn try_send_icc_file(
        &self,
        icc: &Rc<OwnedFd>,
        icc_size: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            icc,
            icc_size,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.icc_file(icc: {}, icc_size: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0.as_raw_fd(), arg1);
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
            1,
            arg1,
        ]);
        Ok(())
    }

    /// ICC profile matching the image description
    ///
    /// The icc argument provides a file descriptor to the client which may be
    /// memory-mapped to provide the ICC profile matching the image description.
    /// The fd is read-only, and if mapped then it must be mapped with
    /// MAP_PRIVATE by the client.
    ///
    /// The ICC profile version and other details are determined by the
    /// compositor. There is no provision for a client to ask for a specific
    /// kind of a profile.
    ///
    /// # Arguments
    ///
    /// - `icc`: ICC profile file descriptor
    /// - `icc_size`: ICC profile size, in bytes
    #[inline]
    pub fn send_icc_file(
        &self,
        icc: &Rc<OwnedFd>,
        icc_size: u32,
    ) {
        let res = self.try_send_icc_file(
            icc,
            icc_size,
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.icc_file", &e);
        }
    }

    /// Since when the primaries message is available.
    pub const MSG__PRIMARIES__SINCE: u32 = 1;

    /// primaries as chromaticity coordinates
    ///
    /// Delivers the primary color volume primaries and white point using CIE
    /// 1931 xy chromaticity coordinates.
    ///
    /// Each coordinate value is multiplied by 1 million to get the argument
    /// value to carry precision of 6 decimals.
    ///
    /// # Arguments
    ///
    /// - `r_x`: Red x * 1M
    /// - `r_y`: Red y * 1M
    /// - `g_x`: Green x * 1M
    /// - `g_y`: Green y * 1M
    /// - `b_x`: Blue x * 1M
    /// - `b_y`: Blue y * 1M
    /// - `w_x`: White x * 1M
    /// - `w_y`: White y * 1M
    #[inline]
    pub fn try_send_primaries(
        &self,
        r_x: i32,
        r_y: i32,
        g_x: i32,
        g_y: i32,
        b_x: i32,
        b_y: i32,
        w_x: i32,
        w_y: i32,
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
        ) = (
            r_x,
            r_y,
            g_x,
            g_y,
            b_x,
            b_y,
            w_x,
            w_y,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.primaries(r_x: {}, r_y: {}, g_x: {}, g_y: {}, b_x: {}, b_y: {}, w_x: {}, w_y: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
            arg4 as u32,
            arg5 as u32,
            arg6 as u32,
            arg7 as u32,
        ]);
        Ok(())
    }

    /// primaries as chromaticity coordinates
    ///
    /// Delivers the primary color volume primaries and white point using CIE
    /// 1931 xy chromaticity coordinates.
    ///
    /// Each coordinate value is multiplied by 1 million to get the argument
    /// value to carry precision of 6 decimals.
    ///
    /// # Arguments
    ///
    /// - `r_x`: Red x * 1M
    /// - `r_y`: Red y * 1M
    /// - `g_x`: Green x * 1M
    /// - `g_y`: Green y * 1M
    /// - `b_x`: Blue x * 1M
    /// - `b_y`: Blue y * 1M
    /// - `w_x`: White x * 1M
    /// - `w_y`: White y * 1M
    #[inline]
    pub fn send_primaries(
        &self,
        r_x: i32,
        r_y: i32,
        g_x: i32,
        g_y: i32,
        b_x: i32,
        b_y: i32,
        w_x: i32,
        w_y: i32,
    ) {
        let res = self.try_send_primaries(
            r_x,
            r_y,
            g_x,
            g_y,
            b_x,
            b_y,
            w_x,
            w_y,
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.primaries", &e);
        }
    }

    /// Since when the primaries_named message is available.
    pub const MSG__PRIMARIES_NAMED__SINCE: u32 = 1;

    /// named primaries
    ///
    /// Delivers the primary color volume primaries and white point using an
    /// explicitly enumerated named set.
    ///
    /// # Arguments
    ///
    /// - `primaries`: named primaries
    #[inline]
    pub fn try_send_primaries_named(
        &self,
        primaries: WpColorManagerV1Primaries,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            primaries,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WpColorManagerV1Primaries) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.primaries_named(primaries: {:?})\n", client_id, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// named primaries
    ///
    /// Delivers the primary color volume primaries and white point using an
    /// explicitly enumerated named set.
    ///
    /// # Arguments
    ///
    /// - `primaries`: named primaries
    #[inline]
    pub fn send_primaries_named(
        &self,
        primaries: WpColorManagerV1Primaries,
    ) {
        let res = self.try_send_primaries_named(
            primaries,
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.primaries_named", &e);
        }
    }

    /// Since when the tf_power message is available.
    pub const MSG__TF_POWER__SINCE: u32 = 1;

    /// transfer characteristic as a power curve
    ///
    /// The color component transfer characteristic of this image description is
    /// a pure power curve. This event provides the exponent of the power
    /// function. This curve represents the conversion from electrical to
    /// optical pixel or color values.
    ///
    /// The curve exponent has been multiplied by 10000 to get the argument eexp
    /// value to carry the precision of 4 decimals.
    ///
    /// # Arguments
    ///
    /// - `eexp`: the exponent * 10000
    #[inline]
    pub fn try_send_tf_power(
        &self,
        eexp: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            eexp,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.tf_power(eexp: {})\n", client_id, id, arg0);
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
            4,
            arg0,
        ]);
        Ok(())
    }

    /// transfer characteristic as a power curve
    ///
    /// The color component transfer characteristic of this image description is
    /// a pure power curve. This event provides the exponent of the power
    /// function. This curve represents the conversion from electrical to
    /// optical pixel or color values.
    ///
    /// The curve exponent has been multiplied by 10000 to get the argument eexp
    /// value to carry the precision of 4 decimals.
    ///
    /// # Arguments
    ///
    /// - `eexp`: the exponent * 10000
    #[inline]
    pub fn send_tf_power(
        &self,
        eexp: u32,
    ) {
        let res = self.try_send_tf_power(
            eexp,
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.tf_power", &e);
        }
    }

    /// Since when the tf_named message is available.
    pub const MSG__TF_NAMED__SINCE: u32 = 1;

    /// named transfer characteristic
    ///
    /// Delivers the transfer characteristic using an explicitly enumerated
    /// named function.
    ///
    /// # Arguments
    ///
    /// - `tf`: named transfer function
    #[inline]
    pub fn try_send_tf_named(
        &self,
        tf: WpColorManagerV1TransferFunction,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            tf,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WpColorManagerV1TransferFunction) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.tf_named(tf: {:?})\n", client_id, id, arg0);
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
            5,
            arg0.0,
        ]);
        Ok(())
    }

    /// named transfer characteristic
    ///
    /// Delivers the transfer characteristic using an explicitly enumerated
    /// named function.
    ///
    /// # Arguments
    ///
    /// - `tf`: named transfer function
    #[inline]
    pub fn send_tf_named(
        &self,
        tf: WpColorManagerV1TransferFunction,
    ) {
        let res = self.try_send_tf_named(
            tf,
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.tf_named", &e);
        }
    }

    /// Since when the luminances message is available.
    pub const MSG__LUMINANCES__SINCE: u32 = 1;

    /// primary color volume luminance range and reference white
    ///
    /// Delivers the primary color volume luminance range and the reference
    /// white luminance level. These values include the minimum display emission
    /// and ambient flare luminances, assumed to be optically additive and have
    /// the chromaticity of the primary color volume white point.
    ///
    /// The minimum luminance is multiplied by 10000 to get the argument
    /// 'min_lum' value and carries precision of 4 decimals. The maximum
    /// luminance and reference white luminance values are unscaled.
    ///
    /// # Arguments
    ///
    /// - `min_lum`: minimum luminance (cd/m²) * 10000
    /// - `max_lum`: maximum luminance (cd/m²)
    /// - `reference_lum`: reference white luminance (cd/m²)
    #[inline]
    pub fn try_send_luminances(
        &self,
        min_lum: u32,
        max_lum: u32,
        reference_lum: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            min_lum,
            max_lum,
            reference_lum,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.luminances(min_lum: {}, max_lum: {}, reference_lum: {})\n", client_id, id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2);
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
            6,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// primary color volume luminance range and reference white
    ///
    /// Delivers the primary color volume luminance range and the reference
    /// white luminance level. These values include the minimum display emission
    /// and ambient flare luminances, assumed to be optically additive and have
    /// the chromaticity of the primary color volume white point.
    ///
    /// The minimum luminance is multiplied by 10000 to get the argument
    /// 'min_lum' value and carries precision of 4 decimals. The maximum
    /// luminance and reference white luminance values are unscaled.
    ///
    /// # Arguments
    ///
    /// - `min_lum`: minimum luminance (cd/m²) * 10000
    /// - `max_lum`: maximum luminance (cd/m²)
    /// - `reference_lum`: reference white luminance (cd/m²)
    #[inline]
    pub fn send_luminances(
        &self,
        min_lum: u32,
        max_lum: u32,
        reference_lum: u32,
    ) {
        let res = self.try_send_luminances(
            min_lum,
            max_lum,
            reference_lum,
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.luminances", &e);
        }
    }

    /// Since when the target_primaries message is available.
    pub const MSG__TARGET_PRIMARIES__SINCE: u32 = 1;

    /// target primaries as chromaticity coordinates
    ///
    /// Provides the color primaries and white point of the target color volume
    /// using CIE 1931 xy chromaticity coordinates. This is compatible with the
    /// SMPTE ST 2086 definition of HDR static metadata for mastering displays.
    ///
    /// While primary color volume is about how color is encoded, the target
    /// color volume is the actually displayable color volume. If target color
    /// volume is equal to the primary color volume, then this event is not
    /// sent.
    ///
    /// Each coordinate value is multiplied by 1 million to get the argument
    /// value to carry precision of 6 decimals.
    ///
    /// # Arguments
    ///
    /// - `r_x`: Red x * 1M
    /// - `r_y`: Red y * 1M
    /// - `g_x`: Green x * 1M
    /// - `g_y`: Green y * 1M
    /// - `b_x`: Blue x * 1M
    /// - `b_y`: Blue y * 1M
    /// - `w_x`: White x * 1M
    /// - `w_y`: White y * 1M
    #[inline]
    pub fn try_send_target_primaries(
        &self,
        r_x: i32,
        r_y: i32,
        g_x: i32,
        g_y: i32,
        b_x: i32,
        b_y: i32,
        w_x: i32,
        w_y: i32,
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
        ) = (
            r_x,
            r_y,
            g_x,
            g_y,
            b_x,
            b_y,
            w_x,
            w_y,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.target_primaries(r_x: {}, r_y: {}, g_x: {}, g_y: {}, b_x: {}, b_y: {}, w_x: {}, w_y: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
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
            7,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
            arg4 as u32,
            arg5 as u32,
            arg6 as u32,
            arg7 as u32,
        ]);
        Ok(())
    }

    /// target primaries as chromaticity coordinates
    ///
    /// Provides the color primaries and white point of the target color volume
    /// using CIE 1931 xy chromaticity coordinates. This is compatible with the
    /// SMPTE ST 2086 definition of HDR static metadata for mastering displays.
    ///
    /// While primary color volume is about how color is encoded, the target
    /// color volume is the actually displayable color volume. If target color
    /// volume is equal to the primary color volume, then this event is not
    /// sent.
    ///
    /// Each coordinate value is multiplied by 1 million to get the argument
    /// value to carry precision of 6 decimals.
    ///
    /// # Arguments
    ///
    /// - `r_x`: Red x * 1M
    /// - `r_y`: Red y * 1M
    /// - `g_x`: Green x * 1M
    /// - `g_y`: Green y * 1M
    /// - `b_x`: Blue x * 1M
    /// - `b_y`: Blue y * 1M
    /// - `w_x`: White x * 1M
    /// - `w_y`: White y * 1M
    #[inline]
    pub fn send_target_primaries(
        &self,
        r_x: i32,
        r_y: i32,
        g_x: i32,
        g_y: i32,
        b_x: i32,
        b_y: i32,
        w_x: i32,
        w_y: i32,
    ) {
        let res = self.try_send_target_primaries(
            r_x,
            r_y,
            g_x,
            g_y,
            b_x,
            b_y,
            w_x,
            w_y,
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.target_primaries", &e);
        }
    }

    /// Since when the target_luminance message is available.
    pub const MSG__TARGET_LUMINANCE__SINCE: u32 = 1;

    /// target luminance range
    ///
    /// Provides the luminance range that the image description is targeting as
    /// the minimum and maximum absolute luminance L. These values include the
    /// minimum display emission and ambient flare luminances, assumed to be
    /// optically additive and have the chromaticity of the primary color
    /// volume white point. This should be compatible with the SMPTE ST 2086
    /// definition of HDR static metadata.
    ///
    /// This luminance range is only theoretical and may not correspond to the
    /// luminance of light emitted on an actual display.
    ///
    /// Min L value is multiplied by 10000 to get the argument min_lum value and
    /// carry precision of 4 decimals. Max L value is unscaled for max_lum.
    ///
    /// # Arguments
    ///
    /// - `min_lum`: min L (cd/m²) * 10000
    /// - `max_lum`: max L (cd/m²)
    #[inline]
    pub fn try_send_target_luminance(
        &self,
        min_lum: u32,
        max_lum: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            min_lum,
            max_lum,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.target_luminance(min_lum: {}, max_lum: {})\n", client_id, id, arg0, arg1);
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
            8,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// target luminance range
    ///
    /// Provides the luminance range that the image description is targeting as
    /// the minimum and maximum absolute luminance L. These values include the
    /// minimum display emission and ambient flare luminances, assumed to be
    /// optically additive and have the chromaticity of the primary color
    /// volume white point. This should be compatible with the SMPTE ST 2086
    /// definition of HDR static metadata.
    ///
    /// This luminance range is only theoretical and may not correspond to the
    /// luminance of light emitted on an actual display.
    ///
    /// Min L value is multiplied by 10000 to get the argument min_lum value and
    /// carry precision of 4 decimals. Max L value is unscaled for max_lum.
    ///
    /// # Arguments
    ///
    /// - `min_lum`: min L (cd/m²) * 10000
    /// - `max_lum`: max L (cd/m²)
    #[inline]
    pub fn send_target_luminance(
        &self,
        min_lum: u32,
        max_lum: u32,
    ) {
        let res = self.try_send_target_luminance(
            min_lum,
            max_lum,
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.target_luminance", &e);
        }
    }

    /// Since when the target_max_cll message is available.
    pub const MSG__TARGET_MAX_CLL__SINCE: u32 = 1;

    /// target maximum content light level
    ///
    /// Provides the targeted max_cll of the image description. max_cll is
    /// defined by CTA-861-H.
    ///
    /// This luminance is only theoretical and may not correspond to the
    /// luminance of light emitted on an actual display.
    ///
    /// # Arguments
    ///
    /// - `max_cll`: Maximum content light-level (cd/m²)
    #[inline]
    pub fn try_send_target_max_cll(
        &self,
        max_cll: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            max_cll,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.target_max_cll(max_cll: {})\n", client_id, id, arg0);
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
            9,
            arg0,
        ]);
        Ok(())
    }

    /// target maximum content light level
    ///
    /// Provides the targeted max_cll of the image description. max_cll is
    /// defined by CTA-861-H.
    ///
    /// This luminance is only theoretical and may not correspond to the
    /// luminance of light emitted on an actual display.
    ///
    /// # Arguments
    ///
    /// - `max_cll`: Maximum content light-level (cd/m²)
    #[inline]
    pub fn send_target_max_cll(
        &self,
        max_cll: u32,
    ) {
        let res = self.try_send_target_max_cll(
            max_cll,
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.target_max_cll", &e);
        }
    }

    /// Since when the target_max_fall message is available.
    pub const MSG__TARGET_MAX_FALL__SINCE: u32 = 1;

    /// target maximum frame-average light level
    ///
    /// Provides the targeted max_fall of the image description. max_fall is
    /// defined by CTA-861-H.
    ///
    /// This luminance is only theoretical and may not correspond to the
    /// luminance of light emitted on an actual display.
    ///
    /// # Arguments
    ///
    /// - `max_fall`: Maximum frame-average light level (cd/m²)
    #[inline]
    pub fn try_send_target_max_fall(
        &self,
        max_fall: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            max_fall,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wp_image_description_info_v1#{}.target_max_fall(max_fall: {})\n", client_id, id, arg0);
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
            10,
            arg0,
        ]);
        Ok(())
    }

    /// target maximum frame-average light level
    ///
    /// Provides the targeted max_fall of the image description. max_fall is
    /// defined by CTA-861-H.
    ///
    /// This luminance is only theoretical and may not correspond to the
    /// luminance of light emitted on an actual display.
    ///
    /// # Arguments
    ///
    /// - `max_fall`: Maximum frame-average light level (cd/m²)
    #[inline]
    pub fn send_target_max_fall(
        &self,
        max_fall: u32,
    ) {
        let res = self.try_send_target_max_fall(
            max_fall,
        );
        if let Err(e) = res {
            log_send("wp_image_description_info_v1.target_max_fall", &e);
        }
    }
}

/// A message handler for [`WpImageDescriptionInfoV1`] proxies.
pub trait WpImageDescriptionInfoV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpImageDescriptionInfoV1>) {
        slf.core.delete_id();
    }

    /// end of information
    ///
    /// Signals the end of information events and destroys the object.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.done", &e);
        }
    }

    /// ICC profile matching the image description
    ///
    /// The icc argument provides a file descriptor to the client which may be
    /// memory-mapped to provide the ICC profile matching the image description.
    /// The fd is read-only, and if mapped then it must be mapped with
    /// MAP_PRIVATE by the client.
    ///
    /// The ICC profile version and other details are determined by the
    /// compositor. There is no provision for a client to ask for a specific
    /// kind of a profile.
    ///
    /// # Arguments
    ///
    /// - `icc`: ICC profile file descriptor
    /// - `icc_size`: ICC profile size, in bytes
    #[inline]
    fn handle_icc_file(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
        icc: &Rc<OwnedFd>,
        icc_size: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_icc_file(
            icc,
            icc_size,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.icc_file", &e);
        }
    }

    /// primaries as chromaticity coordinates
    ///
    /// Delivers the primary color volume primaries and white point using CIE
    /// 1931 xy chromaticity coordinates.
    ///
    /// Each coordinate value is multiplied by 1 million to get the argument
    /// value to carry precision of 6 decimals.
    ///
    /// # Arguments
    ///
    /// - `r_x`: Red x * 1M
    /// - `r_y`: Red y * 1M
    /// - `g_x`: Green x * 1M
    /// - `g_y`: Green y * 1M
    /// - `b_x`: Blue x * 1M
    /// - `b_y`: Blue y * 1M
    /// - `w_x`: White x * 1M
    /// - `w_y`: White y * 1M
    #[inline]
    fn handle_primaries(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
        r_x: i32,
        r_y: i32,
        g_x: i32,
        g_y: i32,
        b_x: i32,
        b_y: i32,
        w_x: i32,
        w_y: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_primaries(
            r_x,
            r_y,
            g_x,
            g_y,
            b_x,
            b_y,
            w_x,
            w_y,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.primaries", &e);
        }
    }

    /// named primaries
    ///
    /// Delivers the primary color volume primaries and white point using an
    /// explicitly enumerated named set.
    ///
    /// # Arguments
    ///
    /// - `primaries`: named primaries
    #[inline]
    fn handle_primaries_named(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
        primaries: WpColorManagerV1Primaries,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_primaries_named(
            primaries,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.primaries_named", &e);
        }
    }

    /// transfer characteristic as a power curve
    ///
    /// The color component transfer characteristic of this image description is
    /// a pure power curve. This event provides the exponent of the power
    /// function. This curve represents the conversion from electrical to
    /// optical pixel or color values.
    ///
    /// The curve exponent has been multiplied by 10000 to get the argument eexp
    /// value to carry the precision of 4 decimals.
    ///
    /// # Arguments
    ///
    /// - `eexp`: the exponent * 10000
    #[inline]
    fn handle_tf_power(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
        eexp: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tf_power(
            eexp,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.tf_power", &e);
        }
    }

    /// named transfer characteristic
    ///
    /// Delivers the transfer characteristic using an explicitly enumerated
    /// named function.
    ///
    /// # Arguments
    ///
    /// - `tf`: named transfer function
    #[inline]
    fn handle_tf_named(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
        tf: WpColorManagerV1TransferFunction,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_tf_named(
            tf,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.tf_named", &e);
        }
    }

    /// primary color volume luminance range and reference white
    ///
    /// Delivers the primary color volume luminance range and the reference
    /// white luminance level. These values include the minimum display emission
    /// and ambient flare luminances, assumed to be optically additive and have
    /// the chromaticity of the primary color volume white point.
    ///
    /// The minimum luminance is multiplied by 10000 to get the argument
    /// 'min_lum' value and carries precision of 4 decimals. The maximum
    /// luminance and reference white luminance values are unscaled.
    ///
    /// # Arguments
    ///
    /// - `min_lum`: minimum luminance (cd/m²) * 10000
    /// - `max_lum`: maximum luminance (cd/m²)
    /// - `reference_lum`: reference white luminance (cd/m²)
    #[inline]
    fn handle_luminances(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
        min_lum: u32,
        max_lum: u32,
        reference_lum: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_luminances(
            min_lum,
            max_lum,
            reference_lum,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.luminances", &e);
        }
    }

    /// target primaries as chromaticity coordinates
    ///
    /// Provides the color primaries and white point of the target color volume
    /// using CIE 1931 xy chromaticity coordinates. This is compatible with the
    /// SMPTE ST 2086 definition of HDR static metadata for mastering displays.
    ///
    /// While primary color volume is about how color is encoded, the target
    /// color volume is the actually displayable color volume. If target color
    /// volume is equal to the primary color volume, then this event is not
    /// sent.
    ///
    /// Each coordinate value is multiplied by 1 million to get the argument
    /// value to carry precision of 6 decimals.
    ///
    /// # Arguments
    ///
    /// - `r_x`: Red x * 1M
    /// - `r_y`: Red y * 1M
    /// - `g_x`: Green x * 1M
    /// - `g_y`: Green y * 1M
    /// - `b_x`: Blue x * 1M
    /// - `b_y`: Blue y * 1M
    /// - `w_x`: White x * 1M
    /// - `w_y`: White y * 1M
    #[inline]
    fn handle_target_primaries(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
        r_x: i32,
        r_y: i32,
        g_x: i32,
        g_y: i32,
        b_x: i32,
        b_y: i32,
        w_x: i32,
        w_y: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_target_primaries(
            r_x,
            r_y,
            g_x,
            g_y,
            b_x,
            b_y,
            w_x,
            w_y,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.target_primaries", &e);
        }
    }

    /// target luminance range
    ///
    /// Provides the luminance range that the image description is targeting as
    /// the minimum and maximum absolute luminance L. These values include the
    /// minimum display emission and ambient flare luminances, assumed to be
    /// optically additive and have the chromaticity of the primary color
    /// volume white point. This should be compatible with the SMPTE ST 2086
    /// definition of HDR static metadata.
    ///
    /// This luminance range is only theoretical and may not correspond to the
    /// luminance of light emitted on an actual display.
    ///
    /// Min L value is multiplied by 10000 to get the argument min_lum value and
    /// carry precision of 4 decimals. Max L value is unscaled for max_lum.
    ///
    /// # Arguments
    ///
    /// - `min_lum`: min L (cd/m²) * 10000
    /// - `max_lum`: max L (cd/m²)
    #[inline]
    fn handle_target_luminance(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
        min_lum: u32,
        max_lum: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_target_luminance(
            min_lum,
            max_lum,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.target_luminance", &e);
        }
    }

    /// target maximum content light level
    ///
    /// Provides the targeted max_cll of the image description. max_cll is
    /// defined by CTA-861-H.
    ///
    /// This luminance is only theoretical and may not correspond to the
    /// luminance of light emitted on an actual display.
    ///
    /// # Arguments
    ///
    /// - `max_cll`: Maximum content light-level (cd/m²)
    #[inline]
    fn handle_target_max_cll(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
        max_cll: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_target_max_cll(
            max_cll,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.target_max_cll", &e);
        }
    }

    /// target maximum frame-average light level
    ///
    /// Provides the targeted max_fall of the image description. max_fall is
    /// defined by CTA-861-H.
    ///
    /// This luminance is only theoretical and may not correspond to the
    /// luminance of light emitted on an actual display.
    ///
    /// # Arguments
    ///
    /// - `max_fall`: Maximum frame-average light level (cd/m²)
    #[inline]
    fn handle_target_max_fall(
        &mut self,
        slf: &Rc<WpImageDescriptionInfoV1>,
        max_fall: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_target_max_fall(
            max_fall,
        );
        if let Err(e) = res {
            log_forward("wp_image_description_info_v1.target_max_fall", &e);
        }
    }
}

impl ObjectPrivate for WpImageDescriptionInfoV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpImageDescriptionInfoV1, version),
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
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                self.core.handle_server_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
                }
            }
            1 => {
                let [
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("icc")));
                };
                let arg0 = &arg0;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.icc_file(icc: {}, icc_size: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0.as_raw_fd(), arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_icc_file(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_icc_file(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                    arg7,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 40)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                let arg5 = arg5 as i32;
                let arg6 = arg6 as i32;
                let arg7 = arg7 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.primaries(r_x: {}, r_y: {}, g_x: {}, g_y: {}, b_x: {}, b_y: {}, w_x: {}, w_y: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                }
                if let Some(handler) = handler {
                    (**handler).handle_primaries(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                } else {
                    DefaultHandler.handle_primaries(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WpColorManagerV1Primaries(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WpColorManagerV1Primaries) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.primaries_named(primaries: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_primaries_named(&self, arg0);
                } else {
                    DefaultHandler.handle_primaries_named(&self, arg0);
                }
            }
            4 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.tf_power(eexp: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tf_power(&self, arg0);
                } else {
                    DefaultHandler.handle_tf_power(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WpColorManagerV1TransferFunction(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WpColorManagerV1TransferFunction) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.tf_named(tf: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_tf_named(&self, arg0);
                } else {
                    DefaultHandler.handle_tf_named(&self, arg0);
                }
            }
            6 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.luminances(min_lum: {}, max_lum: {}, reference_lum: {})\n", id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2);
                }
                if let Some(handler) = handler {
                    (**handler).handle_luminances(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_luminances(&self, arg0, arg1, arg2);
                }
            }
            7 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                    arg7,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 40)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                let arg5 = arg5 as i32;
                let arg6 = arg6 as i32;
                let arg7 = arg7 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.target_primaries(r_x: {}, r_y: {}, g_x: {}, g_y: {}, b_x: {}, b_y: {}, w_x: {}, w_y: {})\n", id, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                }
                if let Some(handler) = handler {
                    (**handler).handle_target_primaries(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                } else {
                    DefaultHandler.handle_target_primaries(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                }
            }
            8 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.target_luminance(min_lum: {}, max_lum: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_target_luminance(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_target_luminance(&self, arg0, arg1);
                }
            }
            9 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.target_max_cll(max_cll: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_target_max_cll(&self, arg0);
                } else {
                    DefaultHandler.handle_target_max_cll(&self, arg0);
                }
            }
            10 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wp_image_description_info_v1#{}.target_max_fall(max_fall: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_target_max_fall(&self, arg0);
                } else {
                    DefaultHandler.handle_target_max_fall(&self, arg0);
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
        let _ = id;
        None
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "done",
            1 => "icc_file",
            2 => "primaries",
            3 => "primaries_named",
            4 => "tf_power",
            5 => "tf_named",
            6 => "luminances",
            7 => "target_primaries",
            8 => "target_luminance",
            9 => "target_max_cll",
            10 => "target_max_fall",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WpImageDescriptionInfoV1 {
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

