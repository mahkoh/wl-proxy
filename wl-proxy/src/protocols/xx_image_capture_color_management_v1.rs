//! image-copy-capture extension to support color management
//!
//! This protocol provides extensions to the ext-image-capture-source-v1 and
//! ext-image-copy-capture-v1 protocols to allow clients to capture frames in
//! color spaces other than srgb/gamma22.
//!
//! Warning! The protocol described in this file is currently in the
//! experimental phase. Backwards incompatible major versions of the
//! protocol are to be expected. Exposing this protocol without an opt-in
//! mechanism is discouraged.

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

pub mod xx_image_capture_color_manager_v1;
pub mod xx_image_capture_source_colors_v1;
