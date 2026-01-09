//! color representation protocol extension
//!
//! This protocol extension delivers the metadata required to define alpha mode,
//! the color model, sub-sampling and quantization range used when interpreting
//! buffer contents. The main use case is defining how the YCbCr family of pixel
//! formats convert to RGB.
//!
//! Note that this protocol does not define the colorimetry of the resulting RGB
//! channels / tristimulus values. Without the help of other extensions the
//! resulting colorimetry is therefore implementation defined.
//!
//! If this extension is not used, the color representation used is compositor
//! implementation defined.
//!
//! Recommendation ITU-T H.273
//! "Coding-independent code points for video signal type identification"
//! shall be referred to as simply H.273 here.

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

pub mod wp_color_representation_manager_v1;
pub mod wp_color_representation_surface_v1;
