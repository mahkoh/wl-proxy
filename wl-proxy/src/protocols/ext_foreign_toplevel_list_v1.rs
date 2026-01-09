//! list toplevels
//!
//! The purpose of this protocol is to provide protocol object handles for
//! toplevels, possibly originating from another client.
//!
//! This protocol is intentionally minimalistic and expects additional
//! functionality (e.g. creating a screencopy source from a toplevel handle,
//! getting information about the state of the toplevel) to be implemented
//! in extension protocols.
//!
//! The compositor may choose to restrict this protocol to a special client
//! launched by the compositor itself or expose it to all clients,
//! this is compositor policy.
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

pub mod ext_foreign_toplevel_handle_v1;
pub mod ext_foreign_toplevel_list_v1;
