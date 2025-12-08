//! protocol for relative pointer motion events
//!
//! This protocol specifies a set of interfaces used for making clients able to
//! receive relative pointer events not obstructed by barriers (such as the
//! monitor edge or other pointer barriers).
//!
//! To start receiving relative pointer events, a client must first bind the
//! global interface "wp_relative_pointer_manager" which, if a compositor
//! supports relative pointer motion events, is exposed by the registry. After
//! having created the relative pointer manager proxy object, the client uses
//! it to create the actual relative pointer object using the
//! "get_relative_pointer" request given a wl_pointer. The relative pointer
//! motion events will then, when applicable, be transmitted via the proxy of
//! the newly created relative pointer object. See the documentation of the
//! relative pointer interface for more details.
//!
//! Warning! The protocol described in this file is experimental and backward
//! incompatible changes may be made. Backward compatible changes may be added
//! together with the corresponding interface version bump. Backward
//! incompatible changes are done by bumping the version number in the protocol
//! and interface names and resetting the interface version. Once the protocol
//! is to be declared stable, the 'z' prefix and the version number in the
//! protocol and interface names are removed and the interface version number is
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

pub mod zwp_relative_pointer_manager_v1;
pub mod zwp_relative_pointer_v1;
