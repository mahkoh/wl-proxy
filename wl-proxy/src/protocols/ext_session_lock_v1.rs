//! secure session locking with arbitrary graphics
//!
//! This protocol allows for a privileged Wayland client to lock the session
//! and display arbitrary graphics while the session is locked.
//!
//! The compositor may choose to restrict this protocol to a special client
//! launched by the compositor itself or expose it to all privileged clients,
//! this is compositor policy.
//!
//! The client is responsible for performing authentication and informing the
//! compositor when the session should be unlocked. If the client dies while
//! the session is locked the session remains locked, possibly permanently
//! depending on compositor policy.
//!
//! The key words "must", "must not", "required", "shall", "shall not",
//! "should", "should not", "recommended",  "may", and "optional" in this
//! document are to be interpreted as described in IETF RFC 2119.
//!
//! Warning! The protocol described in this file is currently in the
//! testing phase. Backward compatible changes may be added together with
//! the corresponding interface version bump. Backward incompatible changes
//! can only be done by creating a new major version of the extension.

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

pub mod ext_session_lock_manager_v1;
pub mod ext_session_lock_surface_v1;
pub mod ext_session_lock_v1;
