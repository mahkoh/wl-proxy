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

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_color_representation_surface_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpColorRepresentationSurfaceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpColorRepresentationSurfaceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpColorRepresentationSurfaceV1MessageHandler for DefaultMessageHandler { }

impl MetaWpColorRepresentationSurfaceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpColorRepresentationSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpColorRepresentationSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpColorRepresentationSurfaceV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
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
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the set_alpha_mode message is available.
    #[allow(dead_code)]
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
    pub fn send_set_alpha_mode(
        &self,
        alpha_mode: MetaWpColorRepresentationSurfaceV1AlphaMode,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            alpha_mode,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the set_coefficients_and_range message is available.
    #[allow(dead_code)]
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
    pub fn send_set_coefficients_and_range(
        &self,
        coefficients: MetaWpColorRepresentationSurfaceV1Coefficients,
        range: MetaWpColorRepresentationSurfaceV1Range,
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
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0.0,
            arg1.0,
        ]);
        Ok(())
    }

    /// Since when the set_chroma_location message is available.
    #[allow(dead_code)]
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
    pub fn send_set_chroma_location(
        &self,
        chroma_location: MetaWpColorRepresentationSurfaceV1ChromaLocation,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            chroma_location,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
            arg0.0,
        ]);
        Ok(())
    }
}

/// A message handler for [WpColorRepresentationSurfaceV1] proxies.
#[allow(dead_code)]
pub trait MetaWpColorRepresentationSurfaceV1MessageHandler {
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
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpColorRepresentationSurfaceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_representation_surface_v1.destroy message: {}", Report::new(e));
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
    fn set_alpha_mode(
        &mut self,
        _slf: &Rc<MetaWpColorRepresentationSurfaceV1>,
        alpha_mode: MetaWpColorRepresentationSurfaceV1AlphaMode,
    ) {
        let res = _slf.send_set_alpha_mode(
            alpha_mode,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_representation_surface_v1.set_alpha_mode message: {}", Report::new(e));
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
    fn set_coefficients_and_range(
        &mut self,
        _slf: &Rc<MetaWpColorRepresentationSurfaceV1>,
        coefficients: MetaWpColorRepresentationSurfaceV1Coefficients,
        range: MetaWpColorRepresentationSurfaceV1Range,
    ) {
        let res = _slf.send_set_coefficients_and_range(
            coefficients,
            range,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_representation_surface_v1.set_coefficients_and_range message: {}", Report::new(e));
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
    fn set_chroma_location(
        &mut self,
        _slf: &Rc<MetaWpColorRepresentationSurfaceV1>,
        chroma_location: MetaWpColorRepresentationSurfaceV1ChromaLocation,
    ) {
        let res = _slf.send_set_chroma_location(
            chroma_location,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_color_representation_surface_v1.set_chroma_location message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpColorRepresentationSurfaceV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWpColorRepresentationSurfaceV1AlphaMode(arg0);
                if let Some(handler) = handler {
                    (**handler).set_alpha_mode(&self, arg0);
                } else {
                    DefaultMessageHandler.set_alpha_mode(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWpColorRepresentationSurfaceV1Coefficients(arg0);
                let arg1 = MetaWpColorRepresentationSurfaceV1Range(arg1);
                if let Some(handler) = handler {
                    (**handler).set_coefficients_and_range(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_coefficients_and_range(&self, arg0, arg1);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = MetaWpColorRepresentationSurfaceV1ChromaLocation(arg0);
                if let Some(handler) = handler {
                    (**handler).set_chroma_location(&self, arg0);
                } else {
                    DefaultMessageHandler.set_chroma_location(&self, arg0);
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

impl MetaWpColorRepresentationSurfaceV1 {
    /// Since when the error.alpha_mode enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALPHA_MODE__SINCE: u32 = 1;
    /// Since when the error.coefficients enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_COEFFICIENTS__SINCE: u32 = 1;
    /// Since when the error.pixel_format enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_PIXEL_FORMAT__SINCE: u32 = 1;
    /// Since when the error.inert enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INERT__SINCE: u32 = 1;
    /// Since when the error.chroma_location enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_CHROMA_LOCATION__SINCE: u32 = 1;

    /// Since when the alpha_mode.premultiplied_electrical enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ALPHA_MODE_PREMULTIPLIED_ELECTRICAL__SINCE: u32 = 1;
    /// Since when the alpha_mode.premultiplied_optical enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ALPHA_MODE_PREMULTIPLIED_OPTICAL__SINCE: u32 = 1;
    /// Since when the alpha_mode.straight enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ALPHA_MODE_STRAIGHT__SINCE: u32 = 1;

    /// Since when the coefficients.identity enum variant is available.
    #[allow(dead_code)]
    pub const ENM__COEFFICIENTS_IDENTITY__SINCE: u32 = 1;
    /// Since when the coefficients.bt709 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__COEFFICIENTS_BT709__SINCE: u32 = 1;
    /// Since when the coefficients.fcc enum variant is available.
    #[allow(dead_code)]
    pub const ENM__COEFFICIENTS_FCC__SINCE: u32 = 1;
    /// Since when the coefficients.bt601 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__COEFFICIENTS_BT601__SINCE: u32 = 1;
    /// Since when the coefficients.smpte240 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__COEFFICIENTS_SMPTE240__SINCE: u32 = 1;
    /// Since when the coefficients.bt2020 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__COEFFICIENTS_BT2020__SINCE: u32 = 1;
    /// Since when the coefficients.bt2020_cl enum variant is available.
    #[allow(dead_code)]
    pub const ENM__COEFFICIENTS_BT2020_CL__SINCE: u32 = 1;
    /// Since when the coefficients.ictcp enum variant is available.
    #[allow(dead_code)]
    pub const ENM__COEFFICIENTS_ICTCP__SINCE: u32 = 1;

    /// Since when the range.full enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RANGE_FULL__SINCE: u32 = 1;
    /// Since when the range.limited enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RANGE_LIMITED__SINCE: u32 = 1;

    /// Since when the chroma_location.type_0 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CHROMA_LOCATION_TYPE_0__SINCE: u32 = 1;
    /// Since when the chroma_location.type_1 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CHROMA_LOCATION_TYPE_1__SINCE: u32 = 1;
    /// Since when the chroma_location.type_2 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CHROMA_LOCATION_TYPE_2__SINCE: u32 = 1;
    /// Since when the chroma_location.type_3 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CHROMA_LOCATION_TYPE_3__SINCE: u32 = 1;
    /// Since when the chroma_location.type_4 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CHROMA_LOCATION_TYPE_4__SINCE: u32 = 1;
    /// Since when the chroma_location.type_5 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CHROMA_LOCATION_TYPE_5__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpColorRepresentationSurfaceV1Error(pub u32);

impl MetaWpColorRepresentationSurfaceV1Error {
    /// unsupported alpha mode
    #[allow(dead_code)]
    pub const ALPHA_MODE: Self = Self(1);

    /// unsupported coefficients
    #[allow(dead_code)]
    pub const COEFFICIENTS: Self = Self(2);

    /// the pixel format and a set value are incompatible
    #[allow(dead_code)]
    pub const PIXEL_FORMAT: Self = Self(3);

    /// forbidden request on inert object
    #[allow(dead_code)]
    pub const INERT: Self = Self(4);

    /// invalid chroma location
    #[allow(dead_code)]
    pub const CHROMA_LOCATION: Self = Self(5);
}

impl Debug for MetaWpColorRepresentationSurfaceV1Error {
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
#[allow(dead_code)]
pub struct MetaWpColorRepresentationSurfaceV1AlphaMode(pub u32);

impl MetaWpColorRepresentationSurfaceV1AlphaMode {
    /// premultiplied alpha in electrical values
    ///
    /// Electrical color channel values (after transfer function encoding)
    /// are already multiplied with the alpha channel value.
    #[allow(dead_code)]
    pub const PREMULTIPLIED_ELECTRICAL: Self = Self(0);

    /// premultiplied alpha in optical values
    ///
    /// Optical color channel values (before transfer function encoding)
    /// are already multiplied with the alpha channel value.
    #[allow(dead_code)]
    pub const PREMULTIPLIED_OPTICAL: Self = Self(1);

    /// straight alpha
    ///
    /// Alpha channel has not been pre-multiplied into color channels.
    #[allow(dead_code)]
    pub const STRAIGHT: Self = Self(2);
}

impl Debug for MetaWpColorRepresentationSurfaceV1AlphaMode {
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
#[allow(dead_code)]
pub struct MetaWpColorRepresentationSurfaceV1Coefficients(pub u32);

impl MetaWpColorRepresentationSurfaceV1Coefficients {
    /// The identity matrix
    ///
    /// Coefficients as defined by
    /// - IEC 61966-2-1 sRGB
    /// - SMPTE ST 428-1 (2019)
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 0.
    /// Compatible with pixel formats of the RGB family.
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub const BT709: Self = Self(2);

    /// FCC matrix coefficients
    ///
    /// Coefficients as defined by
    /// - United States Federal Communications Commission (2003) Title 47
    ///   Code of Federal Regulations 73.682 (a) (20)
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 4.
    /// Compatible with pixel formats of the YCbCr family.
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub const BT601: Self = Self(4);

    /// SMPTE ST 240 matrix coefficients
    ///
    /// Coefficients as defined by
    /// - SMPTE ST 240 (1999)
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 7.
    /// Compatible with pixel formats of the YCbCr family.
    #[allow(dead_code)]
    pub const SMPTE240: Self = Self(5);

    /// BT.2020 and BT.2100 YCbCr matrix coefficients
    ///
    /// Coefficients as defined by
    /// - Rec. ITU-R BT.2020-2 (non-constant luminance)
    /// - Rec. ITU-R BT.2100-2 Y′CbCr
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 9.
    /// Compatible with pixel formats of the YCbCr family.
    #[allow(dead_code)]
    pub const BT2020: Self = Self(6);

    /// BT.2020 matrix coefficients for constant luminance
    ///
    /// Coefficients as defined by
    /// - Rec. ITU-R BT.2020-2 (constant luminance)
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 10.
    /// Compatible with pixel formats of the YCbCr family.
    #[allow(dead_code)]
    pub const BT2020_CL: Self = Self(7);

    /// BT.2100 ICtCp matrix coefficients
    ///
    /// Coefficients as defined by
    /// - Rec. ITU-R BT.2100-2 ICTCP
    ///
    /// Equivalent to H.273 MatrixCoefficients code point 14.
    /// Compatible with pixel formats of the YCbCr family.
    #[allow(dead_code)]
    pub const ICTCP: Self = Self(8);
}

impl Debug for MetaWpColorRepresentationSurfaceV1Coefficients {
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
#[allow(dead_code)]
pub struct MetaWpColorRepresentationSurfaceV1Range(pub u32);

impl MetaWpColorRepresentationSurfaceV1Range {
    /// Full color range
    #[allow(dead_code)]
    pub const FULL: Self = Self(1);

    /// Limited color range
    #[allow(dead_code)]
    pub const LIMITED: Self = Self(2);
}

impl Debug for MetaWpColorRepresentationSurfaceV1Range {
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
#[allow(dead_code)]
pub struct MetaWpColorRepresentationSurfaceV1ChromaLocation(pub u32);

impl MetaWpColorRepresentationSurfaceV1ChromaLocation {
    /// Horizontal offset of 0, vertical offset of 0.5
    ///
    /// Corresponding to VkChromaLocations:
    /// - xChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
    /// - yChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 0.
    #[allow(dead_code)]
    pub const TYPE_0: Self = Self(1);

    /// Horizontal offset of 0.5, vertical offset of 0.5
    ///
    /// Corresponding to VkChromaLocations:
    /// - xChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
    /// - yChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 1.
    #[allow(dead_code)]
    pub const TYPE_1: Self = Self(2);

    /// Horizontal offset of 0, vertical offset of 0
    ///
    /// Corresponding to VkChromaLocations:
    /// - xChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
    /// - yChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 2.
    #[allow(dead_code)]
    pub const TYPE_2: Self = Self(3);

    /// Horizontal offset of 0.5, vertical offset of 0
    ///
    /// Corresponding to VkChromaLocations:
    /// - xChromaOffset: VK_CHROMA_LOCATION_MIDPOINT
    /// - yChromaOffset: VK_CHROMA_LOCATION_COSITED_EVEN
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 3.
    #[allow(dead_code)]
    pub const TYPE_3: Self = Self(4);

    /// Horizontal offset of 0, vertical offset of 1
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 4.
    #[allow(dead_code)]
    pub const TYPE_4: Self = Self(5);

    /// Horizontal offset of 0.5, vertical offset of 1
    ///
    /// Equivalent to H.273 Chroma420SampleLocType 5.
    #[allow(dead_code)]
    pub const TYPE_5: Self = Self(6);
}

impl Debug for MetaWpColorRepresentationSurfaceV1ChromaLocation {
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
