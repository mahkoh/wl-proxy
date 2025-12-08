//! holder of image description parameters
//!
//! This type of object is used for collecting all the parameters required
//! to create a wp_image_description_v1 object. A complete set of required
//! parameters consists of these properties:
//! - transfer characteristic function (tf)
//! - chromaticities of primaries and white point (primary color volume)
//!
//! The following properties are optional and have a well-defined default
//! if not explicitly set:
//! - primary color volume luminance range
//! - reference white luminance level
//! - mastering display primaries and white point (target color volume)
//! - mastering luminance range
//!
//! The following properties are optional and will be ignored
//! if not explicitly set:
//! - maximum content light level
//! - maximum frame-average light level
//!
//! Each required property must be set exactly once if the client is to create
//! an image description. The set requests verify that a property was not
//! already set. The create request verifies that all required properties are
//! set. There may be several alternative requests for setting each property,
//! and in that case the client must choose one of them.
//!
//! Once all properties have been set, the create request must be used to
//! create the image description object, destroying the creator in the
//! process.
//!
//! A viewer, who is viewing the display defined by the resulting image
//! description (the viewing environment included), is assumed to be fully
//! adapted to the primary color volume's white point.
//!
//! Any of the following conditions will cause the colorimetry of a pixel
//! to become undefined:
//! - Values outside of the defined range of the transfer characteristic.
//! - Tristimulus that exceeds the target color volume.
//! - If extended_target_volume is not supported: tristimulus that exceeds
//! the primary color volume.
//!
//! The closest correspondence to an image description created through this
//! interface is the Display class of profiles in ICC.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_image_description_creator_params_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpImageDescriptionCreatorParamsV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpImageDescriptionCreatorParamsV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpImageDescriptionCreatorParamsV1MessageHandler for DefaultMessageHandler { }

impl MetaWpImageDescriptionCreatorParamsV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpImageDescriptionCreatorParamsV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpImageDescriptionCreatorParamsV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpImageDescriptionCreatorParamsV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpImageDescriptionCreatorParamsV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpImageDescriptionCreatorParamsV1 {
    /// Since when the create message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE__SINCE: u32 = 1;

    /// Create the image description object using params
    ///
    /// Create an image description object based on the parameters previously
    /// set on this object.
    ///
    /// The completeness of the parameter set is verified. If the set is not
    /// complete, the protocol error incomplete_set is raised. For the
    /// definition of a complete set, see the description of this interface.
    ///
    /// The protocol error invalid_luminance is raised if any of the following
    /// requirements is not met:
    /// - When max_cll is set, it must be greater than min L and less or equal
    ///   to max L of the mastering luminance range.
    /// - When max_fall is set, it must be greater than min L and less or equal
    ///   to max L of the mastering luminance range.
    /// - When both max_cll and max_fall are set, max_fall must be less or equal
    ///   to max_cll.
    ///
    /// If the particular combination of the parameter set is not supported
    /// by the compositor, the resulting image description object shall
    /// immediately deliver the wp_image_description_v1.failed event with the
    /// 'unsupported' cause. If a valid image description was created from the
    /// parameter set, the wp_image_description_v1.ready event will eventually
    /// be sent instead.
    ///
    /// This request destroys the wp_image_description_creator_params_v1
    /// object.
    ///
    /// The resulting image description object does not allow get_information
    /// request.
    #[inline]
    pub fn send_create(
        &self,
        image_description: &Rc<MetaWpImageDescriptionV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            image_description,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the set_tf_named message is available.
    #[allow(dead_code)]
    pub const MSG__SET_TF_NAMED__SINCE: u32 = 1;

    /// named transfer characteristic
    ///
    /// Sets the transfer characteristic using explicitly enumerated named
    /// functions.
    ///
    /// When the resulting image description is attached to an image, the
    /// content should be decoded according to the industry standard
    /// practices for the transfer characteristic.
    ///
    /// Only names advertised with wp_color_manager_v1 event supported_tf_named
    /// are allowed. Other values shall raise the protocol error invalid_tf.
    ///
    /// If transfer characteristic has already been set on this object, the
    /// protocol error already_set is raised.
    ///
    /// # Arguments
    ///
    /// - `tf`: named transfer function
    #[inline]
    pub fn send_set_tf_named(
        &self,
        tf: MetaWpColorManagerV1TransferFunction,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            tf,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_tf_power message is available.
    #[allow(dead_code)]
    pub const MSG__SET_TF_POWER__SINCE: u32 = 1;

    /// transfer characteristic as a power curve
    ///
    /// Sets the color component transfer characteristic to a power curve with
    /// the given exponent. Negative values are handled by mirroring the
    /// positive half of the curve through the origin. The valid domain and
    /// range of the curve are all finite real numbers. This curve represents
    /// the conversion from electrical to optical color channel values.
    ///
    /// The curve exponent shall be multiplied by 10000 to get the argument eexp
    /// value to carry the precision of 4 decimals.
    ///
    /// The curve exponent must be at least 1.0 and at most 10.0. Otherwise the
    /// protocol error invalid_tf is raised.
    ///
    /// If transfer characteristic has already been set on this object, the
    /// protocol error already_set is raised.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.set_tf_power. Otherwise this request raises
    /// the protocol error unsupported_feature.
    ///
    /// # Arguments
    ///
    /// - `eexp`: the exponent * 10000
    #[inline]
    pub fn send_set_tf_power(
        &self,
        eexp: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            eexp,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_primaries_named message is available.
    #[allow(dead_code)]
    pub const MSG__SET_PRIMARIES_NAMED__SINCE: u32 = 1;

    /// named primaries
    ///
    /// Sets the color primaries and white point using explicitly named sets.
    /// This describes the primary color volume which is the basis for color
    /// value encoding.
    ///
    /// Only names advertised with wp_color_manager_v1 event
    /// supported_primaries_named are allowed. Other values shall raise the
    /// protocol error invalid_primaries_named.
    ///
    /// If primaries have already been set on this object, the protocol error
    /// already_set is raised.
    ///
    /// # Arguments
    ///
    /// - `primaries`: named primaries
    #[inline]
    pub fn send_set_primaries_named(
        &self,
        primaries: MetaWpColorManagerV1Primaries,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            primaries,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_primaries message is available.
    #[allow(dead_code)]
    pub const MSG__SET_PRIMARIES__SINCE: u32 = 1;

    /// primaries as chromaticity coordinates
    ///
    /// Sets the color primaries and white point using CIE 1931 xy chromaticity
    /// coordinates. This describes the primary color volume which is the basis
    /// for color value encoding.
    ///
    /// Each coordinate value is multiplied by 1 million to get the argument
    /// value to carry precision of 6 decimals.
    ///
    /// If primaries have already been set on this object, the protocol error
    /// already_set is raised.
    ///
    /// This request can be used if the compositor advertises
    /// wp_color_manager_v1.feature.set_primaries. Otherwise this request raises
    /// the protocol error unsupported_feature.
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
    pub fn send_set_primaries(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            4,
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

    /// Since when the set_luminances message is available.
    #[allow(dead_code)]
    pub const MSG__SET_LUMINANCES__SINCE: u32 = 1;

    /// primary color volume luminance range and reference white
    ///
    /// Sets the primary color volume luminance range and the reference white
    /// luminance level. These values include the minimum display emission, but
    /// not external flare. The minimum display emission is assumed to have
    /// the chromaticity of the primary color volume white point.
    ///
    /// The default luminances from
    /// https://www.color.org/chardata/rgb/srgb.xalter are
    /// - primary color volume minimum: 0.2 cd/m²
    /// - primary color volume maximum: 80 cd/m²
    /// - reference white: 80 cd/m²
    ///
    /// Setting a named transfer characteristic can imply other default
    /// luminances.
    ///
    /// The default luminances get overwritten when this request is used.
    /// With transfer_function.st2084_pq the given 'max_lum' value is ignored,
    /// and 'max_lum' is taken as 'min_lum' + 10000 cd/m².
    ///
    /// 'min_lum' and 'max_lum' specify the minimum and maximum luminances of
    /// the primary color volume as reproduced by the targeted display.
    ///
    /// 'reference_lum' specifies the luminance of the reference white as
    /// reproduced by the targeted display, and reflects the targeted viewing
    /// environment.
    ///
    /// Compositors should make sure that all content is anchored, meaning that
    /// an input signal level of 'reference_lum' on one image description and
    /// another input signal level of 'reference_lum' on another image
    /// description should produce the same output level, even though the
    /// 'reference_lum' on both image representations can be different.
    ///
    /// 'reference_lum' may be higher than 'max_lum'. In that case reaching
    /// the reference white output level in image content requires the
    /// 'extended_target_volume' feature support.
    ///
    /// If 'max_lum' or 'reference_lum' are less than or equal to 'min_lum',
    /// the protocol error invalid_luminance is raised.
    ///
    /// The minimum luminance is multiplied by 10000 to get the argument
    /// 'min_lum' value and carries precision of 4 decimals. The maximum
    /// luminance and reference white luminance values are unscaled.
    ///
    /// If the primary color volume luminance range and the reference white
    /// luminance level have already been set on this object, the protocol error
    /// already_set is raised.
    ///
    /// This request can be used if the compositor advertises
    /// wp_color_manager_v1.feature.set_luminances. Otherwise this request
    /// raises the protocol error unsupported_feature.
    ///
    /// # Arguments
    ///
    /// - `min_lum`: minimum luminance (cd/m²) * 10000
    /// - `max_lum`: maximum luminance (cd/m²)
    /// - `reference_lum`: reference white luminance (cd/m²)
    #[inline]
    pub fn send_set_luminances(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            5,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the set_mastering_display_primaries message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MASTERING_DISPLAY_PRIMARIES__SINCE: u32 = 1;

    /// mastering display primaries as chromaticity coordinates
    ///
    /// Provides the color primaries and white point of the mastering display
    /// using CIE 1931 xy chromaticity coordinates. This is compatible with the
    /// SMPTE ST 2086 definition of HDR static metadata.
    ///
    /// The mastering display primaries and mastering display luminances define
    /// the target color volume.
    ///
    /// If mastering display primaries are not explicitly set, the target color
    /// volume is assumed to have the same primaries as the primary color volume.
    ///
    /// The target color volume is defined by all tristimulus values between 0.0
    /// and 1.0 (inclusive) of the color space defined by the given mastering
    /// display primaries and white point. The colorimetry is identical between
    /// the container color space and the mastering display color space,
    /// including that no chromatic adaptation is applied even if the white
    /// points differ.
    ///
    /// The target color volume can exceed the primary color volume to allow for
    /// a greater color volume with an existing color space definition (for
    /// example scRGB). It can be smaller than the primary color volume to
    /// minimize gamut and tone mapping distances for big color spaces (HDR
    /// metadata).
    ///
    /// To make use of the entire target color volume a suitable pixel format
    /// has to be chosen (e.g. floating point to exceed the primary color
    /// volume, or abusing limited quantization range as with xvYCC).
    ///
    /// Each coordinate value is multiplied by 1 million to get the argument
    /// value to carry precision of 6 decimals.
    ///
    /// If mastering display primaries have already been set on this object, the
    /// protocol error already_set is raised.
    ///
    /// This request can be used if the compositor advertises
    /// wp_color_manager_v1.feature.set_mastering_display_primaries. Otherwise
    /// this request raises the protocol error unsupported_feature. The
    /// advertisement implies support only for target color volumes fully
    /// contained within the primary color volume.
    ///
    /// If a compositor additionally supports target color volume exceeding the
    /// primary color volume, it must advertise
    /// wp_color_manager_v1.feature.extended_target_volume. If a client uses
    /// target color volume exceeding the primary color volume and the
    /// compositor does not support it, the result is implementation defined.
    /// Compositors are recommended to detect this case and fail the image
    /// description gracefully, but it may as well result in color artifacts.
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
    pub fn send_set_mastering_display_primaries(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            6,
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

    /// Since when the set_mastering_luminance message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MASTERING_LUMINANCE__SINCE: u32 = 1;

    /// display mastering luminance range
    ///
    /// Sets the luminance range that was used during the content mastering
    /// process as the minimum and maximum absolute luminance L. These values
    /// include the minimum display emission and ambient flare luminances,
    /// assumed to be optically additive and have the chromaticity of the
    /// primary color volume white point. This should be
    /// compatible with the SMPTE ST 2086 definition of HDR static metadata.
    ///
    /// The mastering display primaries and mastering display luminances define
    /// the target color volume.
    ///
    /// If mastering luminances are not explicitly set, the target color volume
    /// is assumed to have the same min and max luminances as the primary color
    /// volume.
    ///
    /// If max L is less than or equal to min L, the protocol error
    /// invalid_luminance is raised.
    ///
    /// Min L value is multiplied by 10000 to get the argument min_lum value
    /// and carry precision of 4 decimals. Max L value is unscaled for max_lum.
    ///
    /// This request can be used if the compositor advertises
    /// wp_color_manager_v1.feature.set_mastering_display_primaries. Otherwise
    /// this request raises the protocol error unsupported_feature. The
    /// advertisement implies support only for target color volumes fully
    /// contained within the primary color volume.
    ///
    /// If a compositor additionally supports target color volume exceeding the
    /// primary color volume, it must advertise
    /// wp_color_manager_v1.feature.extended_target_volume. If a client uses
    /// target color volume exceeding the primary color volume and the
    /// compositor does not support it, the result is implementation defined.
    /// Compositors are recommended to detect this case and fail the image
    /// description gracefully, but it may as well result in color artifacts.
    ///
    /// # Arguments
    ///
    /// - `min_lum`: min L (cd/m²) * 10000
    /// - `max_lum`: max L (cd/m²)
    #[inline]
    pub fn send_set_mastering_luminance(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            7,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the set_max_cll message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MAX_CLL__SINCE: u32 = 1;

    /// maximum content light level
    ///
    /// Sets the maximum content light level (max_cll) as defined by CTA-861-H.
    ///
    /// max_cll is undefined by default.
    ///
    /// # Arguments
    ///
    /// - `max_cll`: Maximum content light level (cd/m²)
    #[inline]
    pub fn send_set_max_cll(
        &self,
        max_cll: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            max_cll,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            8,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the set_max_fall message is available.
    #[allow(dead_code)]
    pub const MSG__SET_MAX_FALL__SINCE: u32 = 1;

    /// maximum frame-average light level
    ///
    /// Sets the maximum frame-average light level (max_fall) as defined by
    /// CTA-861-H.
    ///
    /// max_fall is undefined by default.
    ///
    /// # Arguments
    ///
    /// - `max_fall`: Maximum frame-average light level (cd/m²)
    #[inline]
    pub fn send_set_max_fall(
        &self,
        max_fall: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            max_fall,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
}

/// A message handler for [WpImageDescriptionCreatorParamsV1] proxies.
#[allow(dead_code)]
pub trait MetaWpImageDescriptionCreatorParamsV1MessageHandler {
    /// Create the image description object using params
    ///
    /// Create an image description object based on the parameters previously
    /// set on this object.
    ///
    /// The completeness of the parameter set is verified. If the set is not
    /// complete, the protocol error incomplete_set is raised. For the
    /// definition of a complete set, see the description of this interface.
    ///
    /// The protocol error invalid_luminance is raised if any of the following
    /// requirements is not met:
    /// - When max_cll is set, it must be greater than min L and less or equal
    ///   to max L of the mastering luminance range.
    /// - When max_fall is set, it must be greater than min L and less or equal
    ///   to max L of the mastering luminance range.
    /// - When both max_cll and max_fall are set, max_fall must be less or equal
    ///   to max_cll.
    ///
    /// If the particular combination of the parameter set is not supported
    /// by the compositor, the resulting image description object shall
    /// immediately deliver the wp_image_description_v1.failed event with the
    /// 'unsupported' cause. If a valid image description was created from the
    /// parameter set, the wp_image_description_v1.ready event will eventually
    /// be sent instead.
    ///
    /// This request destroys the wp_image_description_creator_params_v1
    /// object.
    ///
    /// The resulting image description object does not allow get_information
    /// request.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    fn create(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorParamsV1>,
        image_description: &Rc<MetaWpImageDescriptionV1>,
    ) {
        let res = _slf.send_create(
            image_description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_params_v1.create message: {}", Report::new(e));
        }
    }

    /// named transfer characteristic
    ///
    /// Sets the transfer characteristic using explicitly enumerated named
    /// functions.
    ///
    /// When the resulting image description is attached to an image, the
    /// content should be decoded according to the industry standard
    /// practices for the transfer characteristic.
    ///
    /// Only names advertised with wp_color_manager_v1 event supported_tf_named
    /// are allowed. Other values shall raise the protocol error invalid_tf.
    ///
    /// If transfer characteristic has already been set on this object, the
    /// protocol error already_set is raised.
    ///
    /// # Arguments
    ///
    /// - `tf`: named transfer function
    #[inline]
    fn set_tf_named(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorParamsV1>,
        tf: MetaWpColorManagerV1TransferFunction,
    ) {
        let res = _slf.send_set_tf_named(
            tf,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_params_v1.set_tf_named message: {}", Report::new(e));
        }
    }

    /// transfer characteristic as a power curve
    ///
    /// Sets the color component transfer characteristic to a power curve with
    /// the given exponent. Negative values are handled by mirroring the
    /// positive half of the curve through the origin. The valid domain and
    /// range of the curve are all finite real numbers. This curve represents
    /// the conversion from electrical to optical color channel values.
    ///
    /// The curve exponent shall be multiplied by 10000 to get the argument eexp
    /// value to carry the precision of 4 decimals.
    ///
    /// The curve exponent must be at least 1.0 and at most 10.0. Otherwise the
    /// protocol error invalid_tf is raised.
    ///
    /// If transfer characteristic has already been set on this object, the
    /// protocol error already_set is raised.
    ///
    /// This request can be used when the compositor advertises
    /// wp_color_manager_v1.feature.set_tf_power. Otherwise this request raises
    /// the protocol error unsupported_feature.
    ///
    /// # Arguments
    ///
    /// - `eexp`: the exponent * 10000
    #[inline]
    fn set_tf_power(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorParamsV1>,
        eexp: u32,
    ) {
        let res = _slf.send_set_tf_power(
            eexp,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_params_v1.set_tf_power message: {}", Report::new(e));
        }
    }

    /// named primaries
    ///
    /// Sets the color primaries and white point using explicitly named sets.
    /// This describes the primary color volume which is the basis for color
    /// value encoding.
    ///
    /// Only names advertised with wp_color_manager_v1 event
    /// supported_primaries_named are allowed. Other values shall raise the
    /// protocol error invalid_primaries_named.
    ///
    /// If primaries have already been set on this object, the protocol error
    /// already_set is raised.
    ///
    /// # Arguments
    ///
    /// - `primaries`: named primaries
    #[inline]
    fn set_primaries_named(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorParamsV1>,
        primaries: MetaWpColorManagerV1Primaries,
    ) {
        let res = _slf.send_set_primaries_named(
            primaries,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_params_v1.set_primaries_named message: {}", Report::new(e));
        }
    }

    /// primaries as chromaticity coordinates
    ///
    /// Sets the color primaries and white point using CIE 1931 xy chromaticity
    /// coordinates. This describes the primary color volume which is the basis
    /// for color value encoding.
    ///
    /// Each coordinate value is multiplied by 1 million to get the argument
    /// value to carry precision of 6 decimals.
    ///
    /// If primaries have already been set on this object, the protocol error
    /// already_set is raised.
    ///
    /// This request can be used if the compositor advertises
    /// wp_color_manager_v1.feature.set_primaries. Otherwise this request raises
    /// the protocol error unsupported_feature.
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
    fn set_primaries(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorParamsV1>,
        r_x: i32,
        r_y: i32,
        g_x: i32,
        g_y: i32,
        b_x: i32,
        b_y: i32,
        w_x: i32,
        w_y: i32,
    ) {
        let res = _slf.send_set_primaries(
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
            log::warn!("Could not forward a wp_image_description_creator_params_v1.set_primaries message: {}", Report::new(e));
        }
    }

    /// primary color volume luminance range and reference white
    ///
    /// Sets the primary color volume luminance range and the reference white
    /// luminance level. These values include the minimum display emission, but
    /// not external flare. The minimum display emission is assumed to have
    /// the chromaticity of the primary color volume white point.
    ///
    /// The default luminances from
    /// https://www.color.org/chardata/rgb/srgb.xalter are
    /// - primary color volume minimum: 0.2 cd/m²
    /// - primary color volume maximum: 80 cd/m²
    /// - reference white: 80 cd/m²
    ///
    /// Setting a named transfer characteristic can imply other default
    /// luminances.
    ///
    /// The default luminances get overwritten when this request is used.
    /// With transfer_function.st2084_pq the given 'max_lum' value is ignored,
    /// and 'max_lum' is taken as 'min_lum' + 10000 cd/m².
    ///
    /// 'min_lum' and 'max_lum' specify the minimum and maximum luminances of
    /// the primary color volume as reproduced by the targeted display.
    ///
    /// 'reference_lum' specifies the luminance of the reference white as
    /// reproduced by the targeted display, and reflects the targeted viewing
    /// environment.
    ///
    /// Compositors should make sure that all content is anchored, meaning that
    /// an input signal level of 'reference_lum' on one image description and
    /// another input signal level of 'reference_lum' on another image
    /// description should produce the same output level, even though the
    /// 'reference_lum' on both image representations can be different.
    ///
    /// 'reference_lum' may be higher than 'max_lum'. In that case reaching
    /// the reference white output level in image content requires the
    /// 'extended_target_volume' feature support.
    ///
    /// If 'max_lum' or 'reference_lum' are less than or equal to 'min_lum',
    /// the protocol error invalid_luminance is raised.
    ///
    /// The minimum luminance is multiplied by 10000 to get the argument
    /// 'min_lum' value and carries precision of 4 decimals. The maximum
    /// luminance and reference white luminance values are unscaled.
    ///
    /// If the primary color volume luminance range and the reference white
    /// luminance level have already been set on this object, the protocol error
    /// already_set is raised.
    ///
    /// This request can be used if the compositor advertises
    /// wp_color_manager_v1.feature.set_luminances. Otherwise this request
    /// raises the protocol error unsupported_feature.
    ///
    /// # Arguments
    ///
    /// - `min_lum`: minimum luminance (cd/m²) * 10000
    /// - `max_lum`: maximum luminance (cd/m²)
    /// - `reference_lum`: reference white luminance (cd/m²)
    #[inline]
    fn set_luminances(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorParamsV1>,
        min_lum: u32,
        max_lum: u32,
        reference_lum: u32,
    ) {
        let res = _slf.send_set_luminances(
            min_lum,
            max_lum,
            reference_lum,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_params_v1.set_luminances message: {}", Report::new(e));
        }
    }

    /// mastering display primaries as chromaticity coordinates
    ///
    /// Provides the color primaries and white point of the mastering display
    /// using CIE 1931 xy chromaticity coordinates. This is compatible with the
    /// SMPTE ST 2086 definition of HDR static metadata.
    ///
    /// The mastering display primaries and mastering display luminances define
    /// the target color volume.
    ///
    /// If mastering display primaries are not explicitly set, the target color
    /// volume is assumed to have the same primaries as the primary color volume.
    ///
    /// The target color volume is defined by all tristimulus values between 0.0
    /// and 1.0 (inclusive) of the color space defined by the given mastering
    /// display primaries and white point. The colorimetry is identical between
    /// the container color space and the mastering display color space,
    /// including that no chromatic adaptation is applied even if the white
    /// points differ.
    ///
    /// The target color volume can exceed the primary color volume to allow for
    /// a greater color volume with an existing color space definition (for
    /// example scRGB). It can be smaller than the primary color volume to
    /// minimize gamut and tone mapping distances for big color spaces (HDR
    /// metadata).
    ///
    /// To make use of the entire target color volume a suitable pixel format
    /// has to be chosen (e.g. floating point to exceed the primary color
    /// volume, or abusing limited quantization range as with xvYCC).
    ///
    /// Each coordinate value is multiplied by 1 million to get the argument
    /// value to carry precision of 6 decimals.
    ///
    /// If mastering display primaries have already been set on this object, the
    /// protocol error already_set is raised.
    ///
    /// This request can be used if the compositor advertises
    /// wp_color_manager_v1.feature.set_mastering_display_primaries. Otherwise
    /// this request raises the protocol error unsupported_feature. The
    /// advertisement implies support only for target color volumes fully
    /// contained within the primary color volume.
    ///
    /// If a compositor additionally supports target color volume exceeding the
    /// primary color volume, it must advertise
    /// wp_color_manager_v1.feature.extended_target_volume. If a client uses
    /// target color volume exceeding the primary color volume and the
    /// compositor does not support it, the result is implementation defined.
    /// Compositors are recommended to detect this case and fail the image
    /// description gracefully, but it may as well result in color artifacts.
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
    fn set_mastering_display_primaries(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorParamsV1>,
        r_x: i32,
        r_y: i32,
        g_x: i32,
        g_y: i32,
        b_x: i32,
        b_y: i32,
        w_x: i32,
        w_y: i32,
    ) {
        let res = _slf.send_set_mastering_display_primaries(
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
            log::warn!("Could not forward a wp_image_description_creator_params_v1.set_mastering_display_primaries message: {}", Report::new(e));
        }
    }

    /// display mastering luminance range
    ///
    /// Sets the luminance range that was used during the content mastering
    /// process as the minimum and maximum absolute luminance L. These values
    /// include the minimum display emission and ambient flare luminances,
    /// assumed to be optically additive and have the chromaticity of the
    /// primary color volume white point. This should be
    /// compatible with the SMPTE ST 2086 definition of HDR static metadata.
    ///
    /// The mastering display primaries and mastering display luminances define
    /// the target color volume.
    ///
    /// If mastering luminances are not explicitly set, the target color volume
    /// is assumed to have the same min and max luminances as the primary color
    /// volume.
    ///
    /// If max L is less than or equal to min L, the protocol error
    /// invalid_luminance is raised.
    ///
    /// Min L value is multiplied by 10000 to get the argument min_lum value
    /// and carry precision of 4 decimals. Max L value is unscaled for max_lum.
    ///
    /// This request can be used if the compositor advertises
    /// wp_color_manager_v1.feature.set_mastering_display_primaries. Otherwise
    /// this request raises the protocol error unsupported_feature. The
    /// advertisement implies support only for target color volumes fully
    /// contained within the primary color volume.
    ///
    /// If a compositor additionally supports target color volume exceeding the
    /// primary color volume, it must advertise
    /// wp_color_manager_v1.feature.extended_target_volume. If a client uses
    /// target color volume exceeding the primary color volume and the
    /// compositor does not support it, the result is implementation defined.
    /// Compositors are recommended to detect this case and fail the image
    /// description gracefully, but it may as well result in color artifacts.
    ///
    /// # Arguments
    ///
    /// - `min_lum`: min L (cd/m²) * 10000
    /// - `max_lum`: max L (cd/m²)
    #[inline]
    fn set_mastering_luminance(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorParamsV1>,
        min_lum: u32,
        max_lum: u32,
    ) {
        let res = _slf.send_set_mastering_luminance(
            min_lum,
            max_lum,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_params_v1.set_mastering_luminance message: {}", Report::new(e));
        }
    }

    /// maximum content light level
    ///
    /// Sets the maximum content light level (max_cll) as defined by CTA-861-H.
    ///
    /// max_cll is undefined by default.
    ///
    /// # Arguments
    ///
    /// - `max_cll`: Maximum content light level (cd/m²)
    #[inline]
    fn set_max_cll(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorParamsV1>,
        max_cll: u32,
    ) {
        let res = _slf.send_set_max_cll(
            max_cll,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_params_v1.set_max_cll message: {}", Report::new(e));
        }
    }

    /// maximum frame-average light level
    ///
    /// Sets the maximum frame-average light level (max_fall) as defined by
    /// CTA-861-H.
    ///
    /// max_fall is undefined by default.
    ///
    /// # Arguments
    ///
    /// - `max_fall`: Maximum frame-average light level (cd/m²)
    #[inline]
    fn set_max_fall(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorParamsV1>,
        max_fall: u32,
    ) {
        let res = _slf.send_set_max_fall(
            max_fall,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_params_v1.set_max_fall message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpImageDescriptionCreatorParamsV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaWpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create(&self, arg0);
                } else {
                    DefaultMessageHandler.create(&self, arg0);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWpColorManagerV1TransferFunction(arg0);
                if let Some(handler) = handler {
                    (**handler).set_tf_named(&self, arg0);
                } else {
                    DefaultMessageHandler.set_tf_named(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).set_tf_power(&self, arg0);
                } else {
                    DefaultMessageHandler.set_tf_power(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWpColorManagerV1Primaries(arg0);
                if let Some(handler) = handler {
                    (**handler).set_primaries_named(&self, arg0);
                } else {
                    DefaultMessageHandler.set_primaries_named(&self, arg0);
                }
            }
            4 => {
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
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                let arg5 = arg5 as i32;
                let arg6 = arg6 as i32;
                let arg7 = arg7 as i32;
                if let Some(handler) = handler {
                    (**handler).set_primaries(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                } else {
                    DefaultMessageHandler.set_primaries(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                }
            }
            5 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).set_luminances(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.set_luminances(&self, arg0, arg1, arg2);
                }
            }
            6 => {
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
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                let arg4 = arg4 as i32;
                let arg5 = arg5 as i32;
                let arg6 = arg6 as i32;
                let arg7 = arg7 as i32;
                if let Some(handler) = handler {
                    (**handler).set_mastering_display_primaries(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                } else {
                    DefaultMessageHandler.set_mastering_display_primaries(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
                }
            }
            7 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).set_mastering_luminance(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_mastering_luminance(&self, arg0, arg1);
                }
            }
            8 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).set_max_cll(&self, arg0);
                } else {
                    DefaultMessageHandler.set_max_cll(&self, arg0);
                }
            }
            9 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).set_max_fall(&self, arg0);
                } else {
                    DefaultMessageHandler.set_max_fall(&self, arg0);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

impl MetaWpImageDescriptionCreatorParamsV1 {
    /// Since when the error.incomplete_set enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INCOMPLETE_SET__SINCE: u32 = 1;
    /// Since when the error.already_set enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_SET__SINCE: u32 = 1;
    /// Since when the error.unsupported_feature enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_UNSUPPORTED_FEATURE__SINCE: u32 = 1;
    /// Since when the error.invalid_tf enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_TF__SINCE: u32 = 1;
    /// Since when the error.invalid_primaries_named enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_PRIMARIES_NAMED__SINCE: u32 = 1;
    /// Since when the error.invalid_luminance enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_LUMINANCE__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpImageDescriptionCreatorParamsV1Error(pub u32);

impl MetaWpImageDescriptionCreatorParamsV1Error {
    /// incomplete parameter set
    #[allow(dead_code)]
    pub const INCOMPLETE_SET: Self = Self(0);

    /// property already set
    #[allow(dead_code)]
    pub const ALREADY_SET: Self = Self(1);

    /// request not supported
    #[allow(dead_code)]
    pub const UNSUPPORTED_FEATURE: Self = Self(2);

    /// invalid transfer characteristic
    #[allow(dead_code)]
    pub const INVALID_TF: Self = Self(3);

    /// invalid primaries named
    #[allow(dead_code)]
    pub const INVALID_PRIMARIES_NAMED: Self = Self(4);

    /// invalid luminance value or range
    #[allow(dead_code)]
    pub const INVALID_LUMINANCE: Self = Self(5);
}

impl Debug for MetaWpImageDescriptionCreatorParamsV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INCOMPLETE_SET => "INCOMPLETE_SET",
            Self::ALREADY_SET => "ALREADY_SET",
            Self::UNSUPPORTED_FEATURE => "UNSUPPORTED_FEATURE",
            Self::INVALID_TF => "INVALID_TF",
            Self::INVALID_PRIMARIES_NAMED => "INVALID_PRIMARIES_NAMED",
            Self::INVALID_LUMINANCE => "INVALID_LUMINANCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
