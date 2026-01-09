//! protocol for providing explicit synchronization
//!
//! This protocol allows clients to request explicit synchronization for
//! buffers. It is tied to the Linux DRM synchronization object framework.
//!
//! Synchronization refers to co-ordination of pipelined operations performed
//! on buffers. Most GPU clients will schedule an asynchronous operation to
//! render to the buffer, then immediately send the buffer to the compositor
//! to be attached to a surface.
//!
//! With implicit synchronization, ensuring that the rendering operation is
//! complete before the compositor displays the buffer is an implementation
//! detail handled by either the kernel or userspace graphics driver.
//!
//! By contrast, with explicit synchronization, DRM synchronization object
//! timeline points mark when the asynchronous operations are complete. When
//! submitting a buffer, the client provides a timeline point which will be
//! waited on before the compositor accesses the buffer, and another timeline
//! point that the compositor will signal when it no longer needs to access the
//! buffer contents for the purposes of the surface commit.
//!
//! Linux DRM synchronization objects are documented at:
//! https://dri.freedesktop.org/docs/drm/gpu/drm-mm.html#drm-sync-objects
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

pub mod wp_linux_drm_syncobj_manager_v1;
pub mod wp_linux_drm_syncobj_surface_v1;
pub mod wp_linux_drm_syncobj_timeline_v1;
