//! Protocol for managing application sessions
//!
//! This description provides a high-level overview of the interplay between
//! the interfaces defined this protocol. For details, see the protocol
//! specification.
//!
//! The xdg_session_manager protocol declares interfaces necessary to
//! allow clients to restore toplevel state from previous executions. The
//! xdg_session_manager_v1.get_session request can be used to obtain a
//! xdg_session_v1 resource representing the state of a set of toplevels.
//!
//! Clients may obtain the session string to use in future calls through
//! the xdg_session_v1.created event. Compositors will use this string
//! as an identifiable token for future runs, possibly storing data about
//! the related toplevels in persistent storage. Clients that wish to
//! track sessions in multiple environments may use the $XDG_CURRENT_DESKTOP
//! environment variable.
//!
//! Toplevels are managed through the xdg_session_v1.add_toplevel and
//! xdg_session_v1.remove_toplevel pair of requests. Clients will explicitly
//! request a toplevel to be restored according to prior state through the
//! xdg_session_v1.restore_toplevel request before the toplevel is mapped.
//!
//! Compositors may store session information up to any arbitrary level, and
//! apply any limits and policies to the amount of data stored and its lifetime.
//! Clients must account for missing sessions and partial session restoration.
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

pub mod xdg_session_manager_v1;
pub mod xdg_session_v1;
pub mod xdg_toplevel_session_v1;
