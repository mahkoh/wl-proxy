//! limit input focus to a set of surfaces
//!
//! This protocol allows clients to limit input focus to a specific set
//! of surfaces and receive a notification when the limiter is removed as
//! detailed below.

#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::manual_map)]
#![allow(clippy::module_inception)]
#![allow(unused_imports)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_rust_codeblocks)]

pub mod hyprland_focus_grab_manager_v1;
pub mod hyprland_focus_grab_v1;
