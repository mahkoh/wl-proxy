//! Protocol for associating X11 windows to wl_surfaces
//!
//! This protocol adds a xwayland_surface role which allows an Xwayland
//! server to associate an X11 window to a wl_surface.
//!
//! Before this protocol, this would be done via the Xwayland server
//! providing the wl_surface's resource id via the a client message with
//! the WL_SURFACE_ID atom on the X window.
//! This was problematic as a race could occur if the wl_surface
//! associated with a WL_SURFACE_ID for a window was destroyed before the
//! client message was processed by the compositor and another surface
//! (or other object) had taken its id due to recycling.
//!
//! This protocol solves the problem by moving the X11 window to wl_surface
//! association step to the Wayland side, which means that the association
//! cannot happen out-of-sync with the resource lifetime of the wl_surface.
//!
//! This protocol avoids duplicating the race on the other side by adding a
//! non-zero monotonic serial number which is entirely unique that is set on
//! both the wl_surface (via. xwayland_surface_v1's set_serial method) and
//! the X11 window (via. the `WL_SURFACE_SERIAL` client message) that can be
//! used to associate them, and synchronize the two timelines.
//!
//! The key words "must", "must not", "required", "shall", "shall not",
//! "should", "should not", "recommended",  "may", and "optional" in this
//! document are to be interpreted as described in IETF RFC 2119.
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

pub mod xwayland_shell_v1;
pub mod xwayland_surface_v1;
