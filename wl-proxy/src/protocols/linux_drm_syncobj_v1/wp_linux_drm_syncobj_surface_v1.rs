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

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wp_linux_drm_syncobj_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WpLinuxDrmSyncobjSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn WpLinuxDrmSyncobjSurfaceV1Handler>,
}

struct DefaultHandler;

impl WpLinuxDrmSyncobjSurfaceV1Handler for DefaultHandler { }

impl ConcreteObject for WpLinuxDrmSyncobjSurfaceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WpLinuxDrmSyncobjSurfaceV1;
    const INTERFACE_NAME: &str = "wp_linux_drm_syncobj_surface_v1";
}

impl WpLinuxDrmSyncobjSurfaceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WpLinuxDrmSyncobjSurfaceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WpLinuxDrmSyncobjSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WpLinuxDrmSyncobjSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WpLinuxDrmSyncobjSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WpLinuxDrmSyncobjSurfaceV1 {
    /// Since when the destroy message is available.
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
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_linux_drm_syncobj_surface_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

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
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wp_linux_drm_syncobj_surface_v1.destroy", &e);
        }
    }

    /// Since when the set_acquire_point message is available.
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
    pub fn try_send_set_acquire_point(
        &self,
        timeline: &Rc<WpLinuxDrmSyncobjTimelineV1>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("timeline"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_linux_drm_syncobj_surface_v1#{}.set_acquire_point(timeline: wp_linux_drm_syncobj_timeline_v1#{}, point_hi: {}, point_lo: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0_id,
            arg1,
            arg2,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_set_acquire_point(
        &self,
        timeline: &Rc<WpLinuxDrmSyncobjTimelineV1>,
        point_hi: u32,
        point_lo: u32,
    ) {
        let res = self.try_send_set_acquire_point(
            timeline,
            point_hi,
            point_lo,
        );
        if let Err(e) = res {
            log_send("wp_linux_drm_syncobj_surface_v1.set_acquire_point", &e);
        }
    }

    /// Since when the set_release_point message is available.
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
    pub fn try_send_set_release_point(
        &self,
        timeline: &Rc<WpLinuxDrmSyncobjTimelineV1>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("timeline"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wp_linux_drm_syncobj_surface_v1#{}.set_release_point(timeline: wp_linux_drm_syncobj_timeline_v1#{}, point_hi: {}, point_lo: {})\n", id, arg0, arg1, arg2);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0_id,
            arg1,
            arg2,
        ]);
        Ok(())
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
    #[inline]
    pub fn send_set_release_point(
        &self,
        timeline: &Rc<WpLinuxDrmSyncobjTimelineV1>,
        point_hi: u32,
        point_lo: u32,
    ) {
        let res = self.try_send_set_release_point(
            timeline,
            point_hi,
            point_lo,
        );
        if let Err(e) = res {
            log_send("wp_linux_drm_syncobj_surface_v1.set_release_point", &e);
        }
    }
}

/// A message handler for [`WpLinuxDrmSyncobjSurfaceV1`] proxies.
pub trait WpLinuxDrmSyncobjSurfaceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WpLinuxDrmSyncobjSurfaceV1>) {
        slf.core.delete_id();
    }

    /// destroy the surface synchronization object
    ///
    /// Destroy this surface synchronization object.
    ///
    /// Any timeline point set by this object with set_acquire_point or
    /// set_release_point since the last commit may be discarded by the
    /// compositor. Any timeline point set by this object before the last
    /// commit will not be affected.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WpLinuxDrmSyncobjSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wp_linux_drm_syncobj_surface_v1.destroy", &e);
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
    fn handle_set_acquire_point(
        &mut self,
        slf: &Rc<WpLinuxDrmSyncobjSurfaceV1>,
        timeline: &Rc<WpLinuxDrmSyncobjTimelineV1>,
        point_hi: u32,
        point_lo: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_acquire_point(
            timeline,
            point_hi,
            point_lo,
        );
        if let Err(e) = res {
            log_forward("wp_linux_drm_syncobj_surface_v1.set_acquire_point", &e);
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
    fn handle_set_release_point(
        &mut self,
        slf: &Rc<WpLinuxDrmSyncobjSurfaceV1>,
        timeline: &Rc<WpLinuxDrmSyncobjTimelineV1>,
        point_hi: u32,
        point_lo: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_release_point(
            timeline,
            point_hi,
            point_lo,
        );
        if let Err(e) = res {
            log_forward("wp_linux_drm_syncobj_surface_v1.set_release_point", &e);
        }
    }
}

impl ObjectPrivate for WpLinuxDrmSyncobjSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WpLinuxDrmSyncobjSurfaceV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_linux_drm_syncobj_surface_v1#{}.destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_linux_drm_syncobj_surface_v1#{}.set_acquire_point(timeline: wp_linux_drm_syncobj_timeline_v1#{}, point_hi: {}, point_lo: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WpLinuxDrmSyncobjTimelineV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("timeline", o.core().interface, ObjectInterface::WpLinuxDrmSyncobjTimelineV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_acquire_point(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_set_acquire_point(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 20)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wp_linux_drm_syncobj_surface_v1#{}.set_release_point(timeline: wp_linux_drm_syncobj_timeline_v1#{}, point_hi: {}, point_lo: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WpLinuxDrmSyncobjTimelineV1>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("timeline", o.core().interface, ObjectInterface::WpLinuxDrmSyncobjTimelineV1)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_set_release_point(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_set_release_point(&self, arg0, arg1, arg2);
                }
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "set_acquire_point",
            2 => "set_release_point",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WpLinuxDrmSyncobjSurfaceV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl WpLinuxDrmSyncobjSurfaceV1 {
    /// Since when the error.no_surface enum variant is available.
    pub const ENM__ERROR_NO_SURFACE__SINCE: u32 = 1;
    /// Since when the error.unsupported_buffer enum variant is available.
    pub const ENM__ERROR_UNSUPPORTED_BUFFER__SINCE: u32 = 1;
    /// Since when the error.no_buffer enum variant is available.
    pub const ENM__ERROR_NO_BUFFER__SINCE: u32 = 1;
    /// Since when the error.no_acquire_point enum variant is available.
    pub const ENM__ERROR_NO_ACQUIRE_POINT__SINCE: u32 = 1;
    /// Since when the error.no_release_point enum variant is available.
    pub const ENM__ERROR_NO_RELEASE_POINT__SINCE: u32 = 1;
    /// Since when the error.conflicting_points enum variant is available.
    pub const ENM__ERROR_CONFLICTING_POINTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WpLinuxDrmSyncobjSurfaceV1Error(pub u32);

impl WpLinuxDrmSyncobjSurfaceV1Error {
    /// the associated wl_surface was destroyed
    pub const NO_SURFACE: Self = Self(1);

    /// the buffer does not support explicit synchronization
    pub const UNSUPPORTED_BUFFER: Self = Self(2);

    /// no buffer was attached
    pub const NO_BUFFER: Self = Self(3);

    /// no acquire timeline point was set
    pub const NO_ACQUIRE_POINT: Self = Self(4);

    /// no release timeline point was set
    pub const NO_RELEASE_POINT: Self = Self(5);

    /// acquire and release timeline points are in conflict
    pub const CONFLICTING_POINTS: Self = Self(6);
}

impl Debug for WpLinuxDrmSyncobjSurfaceV1Error {
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
