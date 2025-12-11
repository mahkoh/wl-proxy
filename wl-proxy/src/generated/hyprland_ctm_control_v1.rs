//! controlling outputs' color transform matrix
//!
//! This protocol allows a client to control outputs' color transform matrix (CTM).
//!
//! This protocol is privileged and should not be exposed to unprivileged clients.

#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::manual_map)]
#![allow(clippy::module_inception)]
#![allow(unused_imports)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_rust_codeblocks)]

pub mod hyprland_ctm_control_manager_v1;
