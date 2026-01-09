//! Protocol for requesting fractional surface scales
//!
//! This protocol allows a compositor to suggest for surfaces to render at
//! fractional scales.
//!
//! A client can submit scaled content by utilizing wp_viewport. This is done by
//! creating a wp_viewport object for the surface and setting the destination
//! rectangle to the surface size before the scale factor is applied.
//!
//! The buffer size is calculated by multiplying the surface size by the
//! intended scale.
//!
//! The wl_surface buffer scale should remain set to 1.
//!
//! If a surface has a surface-local size of 100 px by 50 px and wishes to
//! submit buffers with a scale of 1.5, then a buffer of 150px by 75 px should
//! be used and the wp_viewport destination rectangle should be 100 px by 50 px.
//!
//! For toplevel surfaces, the size is rounded halfway away from zero. The
//! rounding algorithm for subsurface position and size is not defined.

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

pub mod wp_fractional_scale_manager_v1;
pub mod wp_fractional_scale_v1;
