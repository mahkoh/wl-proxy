//! protocol to assign icons to toplevels
//!
//! This protocol allows clients to set icons for their toplevel surfaces
//! either via the XDG icon stock (using an icon name), or from pixel data.
//!
//! A toplevel icon represents the individual toplevel (unlike the application
//! or launcher icon, which represents the application as a whole), and may be
//! shown in window switchers, window overviews and taskbars that list
//! individual windows.
//!
//! This document adheres to RFC 2119 when using words like "must",
//! "should", "may", etc.
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

pub mod xdg_toplevel_icon_manager_v1;
pub mod xdg_toplevel_icon_v1;
