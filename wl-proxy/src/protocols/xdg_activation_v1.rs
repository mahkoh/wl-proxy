//! Protocol for requesting activation of surfaces
//!
//! The way for a client to pass focus to another toplevel is as follows.
//!
//! The client that intends to activate another toplevel uses the
//! xdg_activation_v1.get_activation_token request to get an activation token.
//! This token is then forwarded to the client, which is supposed to activate
//! one of its surfaces, through a separate band of communication.
//!
//! One established way of doing this is through the XDG_ACTIVATION_TOKEN
//! environment variable of a newly launched child process. The child process
//! should unset the environment variable again right after reading it out in
//! order to avoid propagating it to other child processes.
//!
//! Another established way exists for Applications implementing the D-Bus
//! interface org.freedesktop.Application, which should get their token under
//! activation-token on their platform_data.
//!
//! In general activation tokens may be transferred across clients through
//! means not described in this protocol.
//!
//! The client to be activated will then pass the token
//! it received to the xdg_activation_v1.activate request. The compositor can
//! then use this token to decide how to react to the activation request.
//!
//! The token the activating client gets may be ineffective either already at
//! the time it receives it, for example if it was not focused, for focus
//! stealing prevention. The activating client will have no way to discover
//! the validity of the token, and may still forward it to the to be activated
//! client.
//!
//! The created activation token may optionally get information attached to it
//! that can be used by the compositor to identify the application that we
//! intend to activate. This can for example be used to display a visual hint
//! about what application is being started.
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

pub mod xdg_activation_token_v1;
pub mod xdg_activation_v1;
