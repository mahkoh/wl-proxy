//! Protocol for providing secure output
//!
//! This protocol specifies a set of interfaces used to provide
//! content-protection for e.g. HDCP, and protect surface contents on the
//! secured outputs and prevent from appearing in screenshots or from being
//! visible on non-secure outputs.
//!
//! A secure-output is defined as an output that is secured by some
//! content-protection mechanism e.g. HDCP, and meets at least the minimum
//! required content-protection level requested by a client.
//!
//! The term content-protection is defined in terms of HDCP type 0 and
//! HDCP type 1, but this may be extended in future.
//!
//! This protocol is not intended for implementing Digital Rights Management on
//! general (e.g. Desktop) systems, and would only be useful for closed systems.
//! As the server is the one responsible for implementing
//! the content-protection, the client can only trust the content-protection as
//! much they can trust the server.
//!
//! In order to protect the content and prevent surface contents from appearing
//! in screenshots or from being visible on non-secure outputs, a client must
//! first bind the global interface "weston_content_protection" which, if a
//! compositor supports secure output, is exposed by the registry.
//! Using the bound global object, the client uses the "get_protection" request
//! to instantiate an interface extension for a wl_surface object.
//! This extended interface will then allow surfaces to request for
//! content-protection, and also to censor the visibility of the surface on
//! non-secure outputs. Client applications should not wait for the protection
//! to change, as it might never change in case the content-protection cannot be
//! achieved. Alternatively, clients can use a timeout and start showing the
//! content in lower quality.
//!
//! Censored visibility is defined as the compositor censoring the protected
//! content on non-secure outputs. Censoring may include artificially reducing
//! image quality or replacing the protected content completely with
//! placeholder graphics.
//!
//! Censored visibility is controlled by protection mode, set by the client.
//! In "relax" mode, the compositor may show protected content on non-secure
//! outputs. It will be up to the client to adapt to secure and non-secure
//! presentation. In "enforce" mode, the compositor will censor the parts of
//! protected content that would otherwise show on non-secure outputs.

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

pub mod weston_content_protection;
pub mod weston_protected_surface;
