//! extension protocol to wlr-output-management
//!
//! This protocol serves as an extension to wlr-output-management.
//!
//! It primarily adds explicit output mirroring,
//! while upstream is figuring out how to best support that.
//!
//! It was designed against version 4 of wlr-output-management, but tries
//! it's best to be forward compatible.

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

pub mod zcosmic_output_configuration_head_v1;
pub mod zcosmic_output_configuration_v1;
pub mod zcosmic_output_head_v1;
pub mod zcosmic_output_manager_v1;
