//! Protocol for exporting xdg surface handles
//!
//! This protocol specifies a way for making it possible to reference a surface
//! of a different client. With such a reference, a client can, by using the
//! interfaces provided by this protocol, manipulate the relationship between
//! its own surfaces and the surface of some other client. For example, stack
//! some of its own surface above the other clients surface.
//!
//! In order for a client A to get a reference of a surface of client B, client
//! B must first export its surface using xdg_exporter.export_toplevel. Upon
//! doing this, client B will receive a handle (a unique string) that it may
//! share with client A in some way (for example D-Bus). After client A has
//! received the handle from client B, it may use xdg_importer.import_toplevel
//! to create a reference to the surface client B just exported. See the
//! corresponding requests for details.
//!
//! A possible use case for this is out-of-process dialogs. For example when a
//! sandboxed client without file system access needs the user to select a file
//! on the file system, given sandbox environment support, it can export its
//! surface, passing the exported surface handle to an unsandboxed process that
//! can show a file browser dialog and stack it above the sandboxed client's
//! surface.
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

pub mod zxdg_exported_v2;
pub mod zxdg_exporter_v2;
pub mod zxdg_imported_v2;
pub mod zxdg_importer_v2;
