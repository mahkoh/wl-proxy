//! capturing the contents of toplevel windows
//!
//! This protocol allows clients to ask for exporting another toplevel's
//! surface(s) to a buffer.
//!
//! Particularly useful for sharing a single window.

#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::manual_map)]
#![allow(clippy::module_inception)]
#![allow(unused_imports)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_rust_codeblocks)]

pub mod hyprland_toplevel_export_frame_v1;
pub mod hyprland_toplevel_export_manager_v1;
