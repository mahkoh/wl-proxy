//! communicate corner radius of windows
//!
//! This protocol provides a way for clients to communicate the
//! corner radius of their toplevels, should they use rounded corners.
//!
//! This hint can then be used by the compositor to draw fitting outlines
//! or prevent overdrawing of other server-side drawn interfaces.

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

pub mod cosmic_corner_radius_manager_v1;
pub mod cosmic_corner_radius_toplevel_v1;
