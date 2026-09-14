//! define touchscreen gestures
//!
//! This protocol allows the river-window-management-v1 window manager to
//! define touchscreen gestures and the conditions under which they should be
//! triggered. It also provides continuous updates on e.g. motion and pinch
//! scale while a gesture is in progress.
//!
//! The key words "must", "must not", "required", "shall", "shall not",
//! "should", "should not", "recommended", "may", and "optional" in this
//! document are to be interpreted as described in IETF RFC 2119.

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

pub mod river_touch_gesture_v1;
pub mod river_touch_gestures_seat_v1;
pub mod river_touch_gestures_v1;
