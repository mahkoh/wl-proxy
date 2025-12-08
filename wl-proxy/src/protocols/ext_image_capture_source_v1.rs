//! opaque image capture source objects
//!
//! This protocol serves as an intermediary between capturing protocols and
//! potential image capture sources such as outputs and toplevels.
//!
//! This protocol may be extended to support more image capture sources in the
//! future, thereby adding those image capture sources to other protocols that
//! use the image capture source object without having to modify those
//! protocols.
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

pub mod ext_foreign_toplevel_image_capture_source_manager_v1;
pub mod ext_image_capture_source_v1;
pub mod ext_output_image_capture_source_manager_v1;
