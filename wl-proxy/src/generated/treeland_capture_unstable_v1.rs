//! protocol for capturing output contents or window contents
//!
//! This protocol allows authorized application to capture output contents or window
//! contents(useful for window streaming).

#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::manual_map)]
#![allow(clippy::module_inception)]
#![allow(unused_imports)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_rust_codeblocks)]

pub mod treeland_capture_context_v1;
pub mod treeland_capture_frame_v1;
pub mod treeland_capture_manager_v1;
pub mod treeland_capture_session_v1;
