//! Protocol for grabbing the keyboard from Xwayland
//!
//! This protocol is application-specific to meet the needs of the X11
//! protocol through Xwayland. It provides a way for Xwayland to request
//! all keyboard events to be forwarded to a surface even when the
//! surface does not have keyboard focus.
//!
//! In the X11 protocol, a client may request an "active grab" on the
//! keyboard. On success, all key events are reported only to the
//! grabbing X11 client. For details, see XGrabKeyboard(3).
//!
//! The core Wayland protocol does not have a notion of an active
//! keyboard grab. When running in Xwayland, X11 applications may
//! acquire an active grab inside Xwayland but that cannot be translated
//! to the Wayland compositor who may set the input focus to some other
//! surface. In doing so, it breaks the X11 client assumption that all
//! key events are reported to the grabbing client.
//!
//! This protocol specifies a way for Xwayland to request all keyboard
//! be directed to the given surface. The protocol does not guarantee
//! that the compositor will honor this request and it does not
//! prescribe user interfaces on how to handle the respond. For example,
//! a compositor may inform the user that all key events are now
//! forwarded to the given client surface, or it may ask the user for
//! permission to do so.
//!
//! Compositors are required to restrict access to this application
//! specific protocol to Xwayland alone.
//!
//! Warning! The protocol described in this file is experimental and
//! backward incompatible changes may be made. Backward compatible
//! changes may be added together with the corresponding interface
//! version bump.
//! Backward incompatible changes are done by bumping the version
//! number in the protocol and interface names and resetting the
//! interface version. Once the protocol is to be declared stable,
//! the 'z' prefix and the version number in the protocol and
//! interface names are removed and the interface version number is
//! reset.

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

pub mod zwp_xwayland_keyboard_grab_manager_v1;
pub mod zwp_xwayland_keyboard_grab_v1;
