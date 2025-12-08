//! Protocol for inhibiting the compositor keyboard shortcuts
//!
//! This protocol specifies a way for a client to request the compositor
//! to ignore its own keyboard shortcuts for a given seat, so that all
//! key events from that seat get forwarded to a surface.
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

pub mod zwp_keyboard_shortcuts_inhibit_manager_v1;
pub mod zwp_keyboard_shortcuts_inhibitor_v1;
