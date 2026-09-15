//! protocol for assigning icons to subjects
//!
//! This protocol allows clients to assign icons to subjects.
//!
//! Each icon has a subject that determines what the icon describes. A subject
//! might, for example, be an xdg_toplevel, and the compositor might request the
//! client to draw three icons for the server-side decorations, the task bar,
//! and a tab-switcher respectively.
//!
//! Since icons are often displayed in multiple locations with different sizes
//! and scales, clients only create a factory object. It is up to the compositor
//! to use this factory to request the client to draw a concrete icon. This
//! process is dynamic: the compositor can request the client to draw a new icon
//! at any time and can inform the client that a concrete icon is no longer
//! required, whenever such a location appears or disappears.

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

pub mod jay_icon_surface_factory_v1;
pub mod jay_icon_surface_manager_v1;
pub mod jay_icon_surface_subject_v1;
pub mod jay_icon_surface_v1;
