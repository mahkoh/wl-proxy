//! per-surface explicit synchronization
//!
//! This object is an add-on interface for wl_surface to enable explicit
//! synchronization.
//!
//! Each surface can be associated with only one object of this interface at
//! any time.
//!
//! Explicit synchronization is guaranteed to be supported for buffers
//! created with any version of the linux-dmabuf protocol. Compositors are
//! free to support explicit synchronization for additional buffer types.
//! If at surface commit time the attached buffer does not support explicit
//! synchronization, an unsupported_buffer error is raised.
//!
//! As long as the wp_linux_drm_syncobj_surface_v1 object is alive, the
//! compositor may ignore implicit synchronization for buffers attached and
//! committed to the wl_surface. The delivery of wl_buffer.release events
//! for buffers attached to the surface becomes undefined.
//!
//! Clients must set both acquire and release points if and only if a
//! non-null buffer is attached in the same surface commit. See the
//! no_buffer, no_acquire_point and no_release_point protocol errors.
//!
//! If at surface commit time the acquire and release DRM syncobj timelines
//! are identical, the acquire point value must be strictly less than the
//! release point value, or else the conflicting_points protocol error is
//! raised.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_linux_drm_syncobj_surface_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpLinuxDrmSyncobjSurfaceV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpLinuxDrmSyncobjSurfaceV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpLinuxDrmSyncobjSurfaceV1MessageHandler for DefaultMessageHandler { }

impl MetaWpLinuxDrmSyncobjSurfaceV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpLinuxDrmSyncobjSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpLinuxDrmSyncobjSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpLinuxDrmSyncobjSurfaceV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the surface synchronization object
    ///
    /// Destroy this surface synchronization object.
    ///
    /// Any timeline point set by this object with set_acquire_point or
    /// set_release_point since the last commit may be discarded by the
    /// compositor. Any timeline point set by this object before the last
    /// commit will not be affected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        Ok(())
    }

    /// Since when the set_acquire_point message is available.
    #[allow(dead_code)]
    pub const MSG__SET_ACQUIRE_POINT__SINCE: u32 = 1;

    /// set the acquire timeline point
    ///
    /// Set the timeline point that must be signalled before the compositor may
    /// sample from the buffer attached with wl_surface.attach.
    ///
    /// The 64-bit unsigned value combined from point_hi and point_lo is the
    /// point value.
    ///
    /// The acquire point is double-buffered state, and will be applied on the
    /// next wl_surface.commit request for the associated surface. Thus, it
    /// applies only to the buffer that is attached to the surface at commit
    /// time.
    ///
    /// If an acquire point has already been attached during the same commit
    /// cycle, the new point replaces the old one.
    ///
    /// If the associated wl_surface was destroyed, a no_surface error is
    /// raised.
    ///
    /// If at surface commit time there is a pending acquire timeline point set
    /// but no pending buffer attached, a no_buffer error is raised. If at
    /// surface commit time there is a pending buffer attached but no pending
    /// acquire timeline point set, the no_acquire_point protocol error is
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `timeline`:
    /// - `point_hi`: high 32 bits of the point value
    /// - `point_lo`: low 32 bits of the point value
    #[inline]
    pub fn send_set_acquire_point(
        &self,
        timeline: &Rc<MetaWpLinuxDrmSyncobjTimelineV1>,
        point_hi: u32,
        point_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            timeline,
            point_hi,
            point_lo,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the set_release_point message is available.
    #[allow(dead_code)]
    pub const MSG__SET_RELEASE_POINT__SINCE: u32 = 1;

    /// set the release timeline point
    ///
    /// Set the timeline point that must be signalled by the compositor when it
    /// has finished its usage of the buffer attached with wl_surface.attach
    /// for the relevant commit.
    ///
    /// Once the timeline point is signaled, and assuming the associated buffer
    /// is not pending release from other wl_surface.commit requests, no
    /// additional explicit or implicit synchronization with the compositor is
    /// required to safely re-use the buffer.
    ///
    /// Note that clients cannot rely on the release point being always
    /// signaled after the acquire point: compositors may release buffers
    /// without ever reading from them. In addition, the compositor may use
    /// different presentation paths for different commits, which may have
    /// different release behavior. As a result, the compositor may signal the
    /// release points in a different order than the client committed them.
    ///
    /// Because signaling a timeline point also signals every previous point,
    /// it is generally not safe to use the same timeline object for the
    /// release points of multiple buffers. The out-of-order signaling
    /// described above may lead to a release point being signaled before the
    /// compositor has finished reading. To avoid this, it is strongly
    /// recommended that each buffer should use a separate timeline for its
    /// release points.
    ///
    /// The 64-bit unsigned value combined from point_hi and point_lo is the
    /// point value.
    ///
    /// The release point is double-buffered state, and will be applied on the
    /// next wl_surface.commit request for the associated surface. Thus, it
    /// applies only to the buffer that is attached to the surface at commit
    /// time.
    ///
    /// If a release point has already been attached during the same commit
    /// cycle, the new point replaces the old one.
    ///
    /// If the associated wl_surface was destroyed, a no_surface error is
    /// raised.
    ///
    /// If at surface commit time there is a pending release timeline point set
    /// but no pending buffer attached, a no_buffer error is raised. If at
    /// surface commit time there is a pending buffer attached but no pending
    /// release timeline point set, the no_release_point protocol error is
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `timeline`:
    /// - `point_hi`: high 32 bits of the point value
    /// - `point_lo`: low 32 bits of the point value
    #[inline]
    pub fn send_set_release_point(
        &self,
        timeline: &Rc<MetaWpLinuxDrmSyncobjTimelineV1>,
        point_hi: u32,
        point_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            timeline,
            point_hi,
            point_lo,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0,
            arg1,
            arg2,
        ]);
        Ok(())
    }
}

/// A message handler for [WpLinuxDrmSyncobjSurfaceV1] proxies.
#[allow(dead_code)]
pub trait MetaWpLinuxDrmSyncobjSurfaceV1MessageHandler {
    /// destroy the surface synchronization object
    ///
    /// Destroy this surface synchronization object.
    ///
    /// Any timeline point set by this object with set_acquire_point or
    /// set_release_point since the last commit may be discarded by the
    /// compositor. Any timeline point set by this object before the last
    /// commit will not be affected.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpLinuxDrmSyncobjSurfaceV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_linux_drm_syncobj_surface_v1.destroy message: {}", Report::new(e));
        }
    }

    /// set the acquire timeline point
    ///
    /// Set the timeline point that must be signalled before the compositor may
    /// sample from the buffer attached with wl_surface.attach.
    ///
    /// The 64-bit unsigned value combined from point_hi and point_lo is the
    /// point value.
    ///
    /// The acquire point is double-buffered state, and will be applied on the
    /// next wl_surface.commit request for the associated surface. Thus, it
    /// applies only to the buffer that is attached to the surface at commit
    /// time.
    ///
    /// If an acquire point has already been attached during the same commit
    /// cycle, the new point replaces the old one.
    ///
    /// If the associated wl_surface was destroyed, a no_surface error is
    /// raised.
    ///
    /// If at surface commit time there is a pending acquire timeline point set
    /// but no pending buffer attached, a no_buffer error is raised. If at
    /// surface commit time there is a pending buffer attached but no pending
    /// acquire timeline point set, the no_acquire_point protocol error is
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `timeline`:
    /// - `point_hi`: high 32 bits of the point value
    /// - `point_lo`: low 32 bits of the point value
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_acquire_point(
        &mut self,
        _slf: &Rc<MetaWpLinuxDrmSyncobjSurfaceV1>,
        timeline: &Rc<MetaWpLinuxDrmSyncobjTimelineV1>,
        point_hi: u32,
        point_lo: u32,
    ) {
        let res = _slf.send_set_acquire_point(
            timeline,
            point_hi,
            point_lo,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_linux_drm_syncobj_surface_v1.set_acquire_point message: {}", Report::new(e));
        }
    }

    /// set the release timeline point
    ///
    /// Set the timeline point that must be signalled by the compositor when it
    /// has finished its usage of the buffer attached with wl_surface.attach
    /// for the relevant commit.
    ///
    /// Once the timeline point is signaled, and assuming the associated buffer
    /// is not pending release from other wl_surface.commit requests, no
    /// additional explicit or implicit synchronization with the compositor is
    /// required to safely re-use the buffer.
    ///
    /// Note that clients cannot rely on the release point being always
    /// signaled after the acquire point: compositors may release buffers
    /// without ever reading from them. In addition, the compositor may use
    /// different presentation paths for different commits, which may have
    /// different release behavior. As a result, the compositor may signal the
    /// release points in a different order than the client committed them.
    ///
    /// Because signaling a timeline point also signals every previous point,
    /// it is generally not safe to use the same timeline object for the
    /// release points of multiple buffers. The out-of-order signaling
    /// described above may lead to a release point being signaled before the
    /// compositor has finished reading. To avoid this, it is strongly
    /// recommended that each buffer should use a separate timeline for its
    /// release points.
    ///
    /// The 64-bit unsigned value combined from point_hi and point_lo is the
    /// point value.
    ///
    /// The release point is double-buffered state, and will be applied on the
    /// next wl_surface.commit request for the associated surface. Thus, it
    /// applies only to the buffer that is attached to the surface at commit
    /// time.
    ///
    /// If a release point has already been attached during the same commit
    /// cycle, the new point replaces the old one.
    ///
    /// If the associated wl_surface was destroyed, a no_surface error is
    /// raised.
    ///
    /// If at surface commit time there is a pending release timeline point set
    /// but no pending buffer attached, a no_buffer error is raised. If at
    /// surface commit time there is a pending buffer attached but no pending
    /// release timeline point set, the no_release_point protocol error is
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `timeline`:
    /// - `point_hi`: high 32 bits of the point value
    /// - `point_lo`: low 32 bits of the point value
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_release_point(
        &mut self,
        _slf: &Rc<MetaWpLinuxDrmSyncobjSurfaceV1>,
        timeline: &Rc<MetaWpLinuxDrmSyncobjTimelineV1>,
        point_hi: u32,
        point_lo: u32,
    ) {
        let res = _slf.send_set_release_point(
            timeline,
            point_hi,
            point_lo,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_linux_drm_syncobj_surface_v1.set_release_point message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpLinuxDrmSyncobjSurfaceV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWpLinuxDrmSyncobjTimelineV1>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_acquire_point(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.set_acquire_point(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWpLinuxDrmSyncobjTimelineV1>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_release_point(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.set_release_point(&self, arg0, arg1, arg2);
                }
            }
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

impl MetaWpLinuxDrmSyncobjSurfaceV1 {
    /// Since when the error.no_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_SURFACE__SINCE: u32 = 1;
    /// Since when the error.unsupported_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_UNSUPPORTED_BUFFER__SINCE: u32 = 1;
    /// Since when the error.no_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_BUFFER__SINCE: u32 = 1;
    /// Since when the error.no_acquire_point enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_ACQUIRE_POINT__SINCE: u32 = 1;
    /// Since when the error.no_release_point enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_RELEASE_POINT__SINCE: u32 = 1;
    /// Since when the error.conflicting_points enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_CONFLICTING_POINTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpLinuxDrmSyncobjSurfaceV1Error(pub u32);

impl MetaWpLinuxDrmSyncobjSurfaceV1Error {
    /// the associated wl_surface was destroyed
    #[allow(dead_code)]
    pub const NO_SURFACE: Self = Self(1);

    /// the buffer does not support explicit synchronization
    #[allow(dead_code)]
    pub const UNSUPPORTED_BUFFER: Self = Self(2);

    /// no buffer was attached
    #[allow(dead_code)]
    pub const NO_BUFFER: Self = Self(3);

    /// no acquire timeline point was set
    #[allow(dead_code)]
    pub const NO_ACQUIRE_POINT: Self = Self(4);

    /// no release timeline point was set
    #[allow(dead_code)]
    pub const NO_RELEASE_POINT: Self = Self(5);

    /// acquire and release timeline points are in conflict
    #[allow(dead_code)]
    pub const CONFLICTING_POINTS: Self = Self(6);
}

impl Debug for MetaWpLinuxDrmSyncobjSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NO_SURFACE => "NO_SURFACE",
            Self::UNSUPPORTED_BUFFER => "UNSUPPORTED_BUFFER",
            Self::NO_BUFFER => "NO_BUFFER",
            Self::NO_ACQUIRE_POINT => "NO_ACQUIRE_POINT",
            Self::NO_RELEASE_POINT => "NO_RELEASE_POINT",
            Self::CONFLICTING_POINTS => "CONFLICTING_POINTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
