//! protocol for creating temporary seats
//!
//! The transient seat protocol can be used by privileged clients to create
//! independent seats that will be removed from the compositor when the client
//! destroys its transient seat.
//!
//! This protocol is intended for use with virtual input protocols such as
//! "virtual_keyboard_unstable_v1" or "wlr_virtual_pointer_unstable_v1", both
//! of which allow the user to select a seat.
//!
//! The "wl_seat" global created by this protocol does not generate input events
//! on its own, or have any capabilities except those assigned to it by other
//! protocol extensions, such as the ones mentioned above.
//!
//! For example, a remote desktop server can create a seat with virtual inputs
//! for each remote user by following these steps for each new connection:
//!  * Create a transient seat
//!  * Wait for the transient seat to be created
//!  * Locate a "wl_seat" global with a matching name
//!  * Create virtual inputs using the resulting "wl_seat" global

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

pub mod ext_transient_seat_manager_v1;
pub mod ext_transient_seat_v1;
