//! color management protocol
//!
//! The aim of the color management extension is to allow clients to know
//! the color properties of outputs, and to tell the compositor about the color
//! properties of their content on surfaces. All surface contents must be
//! readily intended for some display, but not necessarily for the display at
//! hand. Doing this enables a compositor to perform automatic color management
//! of content for different outputs according to how content is intended to
//! look like.
//!
//! For an introduction, see the section "Color management" in the Wayland
//! documentation at https://wayland.freedesktop.org/docs/html/ .
//!
//! The color properties are represented as an image description object which
//! is immutable after it has been created. A wl_output always has an
//! associated image description that clients can observe. A wl_surface
//! always has an associated preferred image description as a hint chosen by
//! the compositor that clients can also observe. Clients can set an image
//! description on a wl_surface to denote the color characteristics of the
//! surface contents.
//!
//! An image description essentially defines a display and (indirectly) its
//! viewing environment. An image description includes SDR and HDR colorimetry
//! and encoding, HDR metadata, and some parameters related to the viewing
//! environment. An image description does not include the properties set
//! through color-representation extension. It is expected that the
//! color-representation extension is used in conjunction with the
//! color-management extension when necessary, particularly with the YUV family
//! of pixel formats.
//!
//! The normative appendix for this protocol is in the appendix.md file beside
//! this XML file.
//!
//! The color-and-hdr repository
//! (https://gitlab.freedesktop.org/pq/color-and-hdr) contains
//! background information on the protocol design and legacy color management.
//! It also contains a glossary, learning resources for digital color, tools,
//! samples and more.
//!
//! The terminology used in this protocol is based on common color science and
//! color encoding terminology where possible. The glossary in the color-and-hdr
//! repository shall be the authority on the definition of terms in this
//! protocol.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

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

pub mod wp_color_management_output_v1;
pub mod wp_color_management_surface_feedback_v1;
pub mod wp_color_management_surface_v1;
pub mod wp_color_manager_v1;
pub mod wp_image_description_creator_icc_v1;
pub mod wp_image_description_creator_params_v1;
pub mod wp_image_description_info_v1;
pub mod wp_image_description_reference_v1;
pub mod wp_image_description_v1;
