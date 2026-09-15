//! protocol for compositor-created wl_surface objects
//!
//! This protocol allows the compositor to create wl_surface objects and send
//! them to the client.
//!
//! This protocol is frozen, meaning that its API and semantics will never
//! change.

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

pub mod jay_wl_surface_factory_manager_v1;
pub mod jay_wl_surface_factory_v1;
