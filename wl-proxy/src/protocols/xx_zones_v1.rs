//! protocol to manage client-specific zones for explicit window placement
//!
//! This protocol provides a way for clients to create and add toplevel windows
//! to "zones".
//!
//! A zone is an environment with its own coordinate space where clients can
//! add and arrange windows that logically belong and relate to each other.
//! It provides means for, among other things, requesting that windows are
//! placed at specific coordinates within the zone coordinate space.
//! See the description of "xx_zone_v1" for more details.
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

pub mod xx_zone_item_v1;
pub mod xx_zone_manager_v1;
pub mod xx_zone_v1;
