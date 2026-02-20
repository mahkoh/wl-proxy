//! configure libinput devices
//!
//! This protocol exposes libinput device configuration APIs. The libinput
//! documentation should be referred to for detailed information on libinput's
//! behavior.
//!
//! Note that the compositor will not be able to expose libinput devices through
//! this protocol when it does not have access to the hardware, for example when
//! running nested in another Wayland compositor or X11 session.
//!
//! This protocol is designed so that (hopefully) any backwards compatible
//! change to libinput's API can be matched with a backwards compatible change
//! to this protocol.
//!
//! Note: the libinput API uses floating point types (float and double in C)
//! which are not (yet?) natively supported by the Wayland protocol. However,
//! the Wayland protocol does support sending arbitrary bytes through the array
//! argument type. This protocol uses e.g. type="array" summary="double" to
//! indicate a native-endian IEEE-754 64-bit double value.
//!
//! The key words "must", "must not", "required", "shall", "shall not",
//! "should", "should not", "recommended", "may", and "optional" in this
//! document are to be interpreted as described in IETF RFC 2119.

#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::manual_map)]
#![allow(clippy::module_inception)]
#![allow(clippy::needless_return)]
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::doc_overindented_list_items)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_rust_codeblocks)]

pub mod river_libinput_accel_config_v1;
pub mod river_libinput_config_v1;
pub mod river_libinput_device_v1;
pub mod river_libinput_result_v1;
