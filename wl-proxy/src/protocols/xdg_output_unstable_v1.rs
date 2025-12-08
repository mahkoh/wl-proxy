//! Protocol to describe output regions
//!
//! This protocol aims at describing outputs in a way which is more in line
//! with the concept of an output on desktop oriented systems.
//!
//! Some information are more specific to the concept of an output for
//! a desktop oriented system and may not make sense in other applications,
//! such as IVI systems for example.
//!
//! Typically, the global compositor space on a desktop system is made of
//! a contiguous or overlapping set of rectangular regions.
//!
//! The logical_position and logical_size events defined in this protocol
//! might provide information identical to their counterparts already
//! available from wl_output, in which case the information provided by this
//! protocol should be preferred to their equivalent in wl_output. The goal is
//! to move the desktop specific concepts (such as output location within the
//! global compositor space, etc.) out of the core wl_output protocol.
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

pub mod zxdg_output_manager_v1;
pub mod zxdg_output_v1;
