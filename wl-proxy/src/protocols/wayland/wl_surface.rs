//! an onscreen surface
//!
//! A surface is a rectangular area that may be displayed on zero
//! or more outputs, and shown any number of times at the compositor's
//! discretion. They can present wl_buffers, receive user input, and
//! define a local coordinate system.
//!
//! The size of a surface (and relative positions on it) is described
//! in surface-local coordinates, which may differ from the buffer
//! coordinates of the pixel content, in case a buffer_transform
//! or a buffer_scale is used.
//!
//! A surface without a "role" is fairly useless: a compositor does
//! not know where, when or how to present it. The role is the
//! purpose of a wl_surface. Examples of roles are a cursor for a
//! pointer (as set by wl_pointer.set_cursor), a drag icon
//! (wl_data_device.start_drag), a sub-surface
//! (wl_subcompositor.get_subsurface), and a window as defined by a
//! shell protocol (e.g. wl_shell.get_shell_surface).
//!
//! A surface can have only one role at a time. Initially a
//! wl_surface does not have a role. Once a wl_surface is given a
//! role, it is set permanently for the whole lifetime of the
//! wl_surface object. Giving the current role again is allowed,
//! unless explicitly forbidden by the relevant interface
//! specification.
//!
//! Surface roles are given by requests in other interfaces such as
//! wl_pointer.set_cursor. The request should explicitly mention
//! that this request gives a role to a wl_surface. Often, this
//! request also creates a new protocol object that represents the
//! role and adds additional functionality to wl_surface. When a
//! client wants to destroy a wl_surface, they must destroy this role
//! object before the wl_surface, otherwise a defunct_role_object error is
//! sent.
//!
//! Destroying the role object does not remove the role from the
//! wl_surface, but it may stop the wl_surface from "playing the role".
//! For instance, if a wl_subsurface object is destroyed, the wl_surface
//! it was created for will be unmapped and forget its position and
//! z-order. It is allowed to create a wl_subsurface for the same
//! wl_surface again, but it is not allowed to use the wl_surface as
//! a cursor (cursor is a different role than sub-surface, and role
//! switching is not allowed).

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_surface object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlSurface {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlSurfaceHandler>,
}

struct DefaultHandler;

impl WlSurfaceHandler for DefaultHandler { }

impl ConcreteObject for WlSurface {
    const XML_VERSION: u32 = 6;
    const INTERFACE: ObjectInterface = ObjectInterface::WlSurface;
    const INTERFACE_NAME: &str = "wl_surface";
}

impl WlSurface {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlSurfaceHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlSurfaceHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlSurface")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlSurface {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// delete surface
    ///
    /// Deletes the surface and invalidates its object ID.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.destroy()\n", id);
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

    /// delete surface
    ///
    /// Deletes the surface and invalidates its object ID.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wl_surface.destroy", &e);
        }
    }

    /// Since when the attach message is available.
    pub const MSG__ATTACH__SINCE: u32 = 1;

    /// set the surface contents
    ///
    /// Set a buffer as the content of this surface.
    ///
    /// The new size of the surface is calculated based on the buffer
    /// size transformed by the inverse buffer_transform and the
    /// inverse buffer_scale. This means that at commit time the supplied
    /// buffer size must be an integer multiple of the buffer_scale. If
    /// that's not the case, an invalid_size error is sent.
    ///
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes. Setting anything other than 0
    /// as x and y arguments is discouraged, and should instead be replaced
    /// with using the separate wl_surface.offset request.
    ///
    /// When the bound wl_surface version is 5 or higher, passing any
    /// non-zero x or y is a protocol violation, and will result in an
    /// 'invalid_offset' error being raised. The x and y arguments are ignored
    /// and do not change the pending state. To achieve equivalent semantics,
    /// use wl_surface.offset.
    ///
    /// Surface contents are double-buffered state, see wl_surface.commit.
    ///
    /// The initial surface contents are void; there is no content.
    /// wl_surface.attach assigns the given wl_buffer as the pending
    /// wl_buffer. wl_surface.commit makes the pending wl_buffer the new
    /// surface contents, and the size of the surface becomes the size
    /// calculated from the wl_buffer, as described above. After commit,
    /// there is no pending buffer until the next attach.
    ///
    /// Committing a pending wl_buffer allows the compositor to read the
    /// pixels in the wl_buffer. The compositor may access the pixels at
    /// any time after the wl_surface.commit request. When the compositor
    /// will not access the pixels anymore, it will send the
    /// wl_buffer.release event. Only after receiving wl_buffer.release,
    /// the client may reuse the wl_buffer. A wl_buffer that has been
    /// attached and then replaced by another attach instead of committed
    /// will not receive a release event, and is not used by the
    /// compositor.
    ///
    /// If a pending wl_buffer has been committed to more than one wl_surface,
    /// the delivery of wl_buffer.release events becomes undefined. A well
    /// behaved client should not rely on wl_buffer.release events in this
    /// case. Alternatively, a client could create multiple wl_buffer objects
    /// from the same backing storage or use a protocol extension providing
    /// per-commit release notifications.
    ///
    /// Destroying the wl_buffer after wl_buffer.release does not change
    /// the surface contents. Destroying the wl_buffer before wl_buffer.release
    /// is allowed as long as the underlying buffer storage isn't re-used (this
    /// can happen e.g. on client process termination). However, if the client
    /// destroys the wl_buffer before receiving the wl_buffer.release event and
    /// mutates the underlying buffer storage, the surface contents become
    /// undefined immediately.
    ///
    /// If wl_surface.attach is sent with a NULL wl_buffer, the
    /// following wl_surface.commit will remove the surface content.
    ///
    /// If a pending wl_buffer has been destroyed, the result is not specified.
    /// Many compositors are known to remove the surface content on the following
    /// wl_surface.commit, but this behaviour is not universal. Clients seeking to
    /// maximise compatibility should not destroy pending buffers and should
    /// ensure that they explicitly remove content from surfaces, even after
    /// destroying buffers.
    ///
    /// # Arguments
    ///
    /// - `buffer`: buffer of surface contents
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn try_send_attach(
        &self,
        buffer: Option<&Rc<WlBuffer>>,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            buffer,
            x,
            y,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("buffer"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.attach(buffer: wl_buffer#{}, x: {}, y: {})\n", id, arg0, arg1, arg2);
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
            arg1 as u32,
            arg2 as u32,
        ]);
        Ok(())
    }

    /// set the surface contents
    ///
    /// Set a buffer as the content of this surface.
    ///
    /// The new size of the surface is calculated based on the buffer
    /// size transformed by the inverse buffer_transform and the
    /// inverse buffer_scale. This means that at commit time the supplied
    /// buffer size must be an integer multiple of the buffer_scale. If
    /// that's not the case, an invalid_size error is sent.
    ///
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes. Setting anything other than 0
    /// as x and y arguments is discouraged, and should instead be replaced
    /// with using the separate wl_surface.offset request.
    ///
    /// When the bound wl_surface version is 5 or higher, passing any
    /// non-zero x or y is a protocol violation, and will result in an
    /// 'invalid_offset' error being raised. The x and y arguments are ignored
    /// and do not change the pending state. To achieve equivalent semantics,
    /// use wl_surface.offset.
    ///
    /// Surface contents are double-buffered state, see wl_surface.commit.
    ///
    /// The initial surface contents are void; there is no content.
    /// wl_surface.attach assigns the given wl_buffer as the pending
    /// wl_buffer. wl_surface.commit makes the pending wl_buffer the new
    /// surface contents, and the size of the surface becomes the size
    /// calculated from the wl_buffer, as described above. After commit,
    /// there is no pending buffer until the next attach.
    ///
    /// Committing a pending wl_buffer allows the compositor to read the
    /// pixels in the wl_buffer. The compositor may access the pixels at
    /// any time after the wl_surface.commit request. When the compositor
    /// will not access the pixels anymore, it will send the
    /// wl_buffer.release event. Only after receiving wl_buffer.release,
    /// the client may reuse the wl_buffer. A wl_buffer that has been
    /// attached and then replaced by another attach instead of committed
    /// will not receive a release event, and is not used by the
    /// compositor.
    ///
    /// If a pending wl_buffer has been committed to more than one wl_surface,
    /// the delivery of wl_buffer.release events becomes undefined. A well
    /// behaved client should not rely on wl_buffer.release events in this
    /// case. Alternatively, a client could create multiple wl_buffer objects
    /// from the same backing storage or use a protocol extension providing
    /// per-commit release notifications.
    ///
    /// Destroying the wl_buffer after wl_buffer.release does not change
    /// the surface contents. Destroying the wl_buffer before wl_buffer.release
    /// is allowed as long as the underlying buffer storage isn't re-used (this
    /// can happen e.g. on client process termination). However, if the client
    /// destroys the wl_buffer before receiving the wl_buffer.release event and
    /// mutates the underlying buffer storage, the surface contents become
    /// undefined immediately.
    ///
    /// If wl_surface.attach is sent with a NULL wl_buffer, the
    /// following wl_surface.commit will remove the surface content.
    ///
    /// If a pending wl_buffer has been destroyed, the result is not specified.
    /// Many compositors are known to remove the surface content on the following
    /// wl_surface.commit, but this behaviour is not universal. Clients seeking to
    /// maximise compatibility should not destroy pending buffers and should
    /// ensure that they explicitly remove content from surfaces, even after
    /// destroying buffers.
    ///
    /// # Arguments
    ///
    /// - `buffer`: buffer of surface contents
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn send_attach(
        &self,
        buffer: Option<&Rc<WlBuffer>>,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_attach(
            buffer,
            x,
            y,
        );
        if let Err(e) = res {
            log_send("wl_surface.attach", &e);
        }
    }

    /// Since when the damage message is available.
    pub const MSG__DAMAGE__SINCE: u32 = 1;

    /// mark part of the surface damaged
    ///
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The compositor
    /// ignores the parts of the damage that fall outside of the surface.
    ///
    /// Damage is double-buffered state, see wl_surface.commit.
    ///
    /// The damage rectangle is specified in surface-local coordinates,
    /// where x and y specify the upper left corner of the damage rectangle.
    ///
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage adds pending damage: the new pending damage
    /// is the union of old pending damage and the given rectangle.
    ///
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    ///
    /// Note! New clients should not use this request. Instead damage can be
    /// posted with wl_surface.damage_buffer which uses buffer coordinates
    /// instead of surface coordinates.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `width`: width of damage rectangle
    /// - `height`: height of damage rectangle
    #[inline]
    pub fn try_send_damage(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            x,
            y,
            width,
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.damage(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3);
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
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// mark part of the surface damaged
    ///
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The compositor
    /// ignores the parts of the damage that fall outside of the surface.
    ///
    /// Damage is double-buffered state, see wl_surface.commit.
    ///
    /// The damage rectangle is specified in surface-local coordinates,
    /// where x and y specify the upper left corner of the damage rectangle.
    ///
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage adds pending damage: the new pending damage
    /// is the union of old pending damage and the given rectangle.
    ///
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    ///
    /// Note! New clients should not use this request. Instead damage can be
    /// posted with wl_surface.damage_buffer which uses buffer coordinates
    /// instead of surface coordinates.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `width`: width of damage rectangle
    /// - `height`: height of damage rectangle
    #[inline]
    pub fn send_damage(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_damage(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("wl_surface.damage", &e);
        }
    }

    /// Since when the frame message is available.
    pub const MSG__FRAME__SINCE: u32 = 1;

    /// request a frame throttling hint
    ///
    /// Request a notification when it is a good time to start drawing a new
    /// frame, by creating a frame callback. This is useful for throttling
    /// redrawing operations, and driving animations.
    ///
    /// When a client is animating on a wl_surface, it can use the 'frame'
    /// request to get notified when it is a good time to draw and commit the
    /// next frame of animation. If the client commits an update earlier than
    /// that, it is likely that some updates will not make it to the display,
    /// and the client is wasting resources by drawing too often.
    ///
    /// The frame request will take effect on the next wl_surface.commit.
    /// The notification will only be posted for one frame unless
    /// requested again. For a wl_surface, the notifications are posted in
    /// the order the frame requests were committed.
    ///
    /// The server must send the notifications so that a client
    /// will not send excessive updates, while still allowing
    /// the highest possible update rate for clients that wait for the reply
    /// before drawing again. The server should give some time for the client
    /// to draw and commit after sending the frame callback events to let it
    /// hit the next output refresh.
    ///
    /// A server should avoid signaling the frame callbacks if the
    /// surface is not visible in any way, e.g. the surface is off-screen,
    /// or completely obscured by other opaque surfaces.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is the current time, in
    /// milliseconds, with an undefined base.
    ///
    /// # Arguments
    ///
    /// - `callback`: callback object for the frame request
    #[inline]
    pub fn try_send_frame(
        &self,
        callback: &Rc<WlCallback>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            callback,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("callback", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.frame(callback: wl_callback#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            3,
            arg0_id,
        ]);
        Ok(())
    }

    /// request a frame throttling hint
    ///
    /// Request a notification when it is a good time to start drawing a new
    /// frame, by creating a frame callback. This is useful for throttling
    /// redrawing operations, and driving animations.
    ///
    /// When a client is animating on a wl_surface, it can use the 'frame'
    /// request to get notified when it is a good time to draw and commit the
    /// next frame of animation. If the client commits an update earlier than
    /// that, it is likely that some updates will not make it to the display,
    /// and the client is wasting resources by drawing too often.
    ///
    /// The frame request will take effect on the next wl_surface.commit.
    /// The notification will only be posted for one frame unless
    /// requested again. For a wl_surface, the notifications are posted in
    /// the order the frame requests were committed.
    ///
    /// The server must send the notifications so that a client
    /// will not send excessive updates, while still allowing
    /// the highest possible update rate for clients that wait for the reply
    /// before drawing again. The server should give some time for the client
    /// to draw and commit after sending the frame callback events to let it
    /// hit the next output refresh.
    ///
    /// A server should avoid signaling the frame callbacks if the
    /// surface is not visible in any way, e.g. the surface is off-screen,
    /// or completely obscured by other opaque surfaces.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is the current time, in
    /// milliseconds, with an undefined base.
    ///
    /// # Arguments
    ///
    /// - `callback`: callback object for the frame request
    #[inline]
    pub fn send_frame(
        &self,
        callback: &Rc<WlCallback>,
    ) {
        let res = self.try_send_frame(
            callback,
        );
        if let Err(e) = res {
            log_send("wl_surface.frame", &e);
        }
    }

    /// request a frame throttling hint
    ///
    /// Request a notification when it is a good time to start drawing a new
    /// frame, by creating a frame callback. This is useful for throttling
    /// redrawing operations, and driving animations.
    ///
    /// When a client is animating on a wl_surface, it can use the 'frame'
    /// request to get notified when it is a good time to draw and commit the
    /// next frame of animation. If the client commits an update earlier than
    /// that, it is likely that some updates will not make it to the display,
    /// and the client is wasting resources by drawing too often.
    ///
    /// The frame request will take effect on the next wl_surface.commit.
    /// The notification will only be posted for one frame unless
    /// requested again. For a wl_surface, the notifications are posted in
    /// the order the frame requests were committed.
    ///
    /// The server must send the notifications so that a client
    /// will not send excessive updates, while still allowing
    /// the highest possible update rate for clients that wait for the reply
    /// before drawing again. The server should give some time for the client
    /// to draw and commit after sending the frame callback events to let it
    /// hit the next output refresh.
    ///
    /// A server should avoid signaling the frame callbacks if the
    /// surface is not visible in any way, e.g. the surface is off-screen,
    /// or completely obscured by other opaque surfaces.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is the current time, in
    /// milliseconds, with an undefined base.
    #[inline]
    pub fn new_try_send_frame(
        &self,
    ) -> Result<Rc<WlCallback>, ObjectError> {
        let callback = self.core.create_child();
        self.try_send_frame(
            &callback,
        )?;
        Ok(callback)
    }

    /// request a frame throttling hint
    ///
    /// Request a notification when it is a good time to start drawing a new
    /// frame, by creating a frame callback. This is useful for throttling
    /// redrawing operations, and driving animations.
    ///
    /// When a client is animating on a wl_surface, it can use the 'frame'
    /// request to get notified when it is a good time to draw and commit the
    /// next frame of animation. If the client commits an update earlier than
    /// that, it is likely that some updates will not make it to the display,
    /// and the client is wasting resources by drawing too often.
    ///
    /// The frame request will take effect on the next wl_surface.commit.
    /// The notification will only be posted for one frame unless
    /// requested again. For a wl_surface, the notifications are posted in
    /// the order the frame requests were committed.
    ///
    /// The server must send the notifications so that a client
    /// will not send excessive updates, while still allowing
    /// the highest possible update rate for clients that wait for the reply
    /// before drawing again. The server should give some time for the client
    /// to draw and commit after sending the frame callback events to let it
    /// hit the next output refresh.
    ///
    /// A server should avoid signaling the frame callbacks if the
    /// surface is not visible in any way, e.g. the surface is off-screen,
    /// or completely obscured by other opaque surfaces.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is the current time, in
    /// milliseconds, with an undefined base.
    #[inline]
    pub fn new_send_frame(
        &self,
    ) -> Rc<WlCallback> {
        let callback = self.core.create_child();
        self.send_frame(
            &callback,
        );
        callback
    }

    /// Since when the set_opaque_region message is available.
    pub const MSG__SET_OPAQUE_REGION__SINCE: u32 = 1;

    /// set opaque region
    ///
    /// This request sets the region of the surface that contains
    /// opaque content.
    ///
    /// The opaque region is an optimization hint for the compositor
    /// that lets it optimize the redrawing of content behind opaque
    /// regions.  Setting an opaque region is not required for correct
    /// behaviour, but marking transparent content as opaque will result
    /// in repaint artifacts.
    ///
    /// The opaque region is specified in surface-local coordinates.
    ///
    /// The compositor ignores the parts of the opaque region that fall
    /// outside of the surface.
    ///
    /// Opaque region is double-buffered state, see wl_surface.commit.
    ///
    /// wl_surface.set_opaque_region changes the pending opaque region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise, the pending and current regions are never changed.
    ///
    /// The initial value for an opaque region is empty. Setting the pending
    /// opaque region has copy semantics, and the wl_region object can be
    /// destroyed immediately. A NULL wl_region causes the pending opaque
    /// region to be set to empty.
    ///
    /// # Arguments
    ///
    /// - `region`: opaque region of the surface
    #[inline]
    pub fn try_send_set_opaque_region(
        &self,
        region: Option<&Rc<WlRegion>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            region,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("region"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.set_opaque_region(region: wl_region#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            4,
            arg0_id,
        ]);
        Ok(())
    }

    /// set opaque region
    ///
    /// This request sets the region of the surface that contains
    /// opaque content.
    ///
    /// The opaque region is an optimization hint for the compositor
    /// that lets it optimize the redrawing of content behind opaque
    /// regions.  Setting an opaque region is not required for correct
    /// behaviour, but marking transparent content as opaque will result
    /// in repaint artifacts.
    ///
    /// The opaque region is specified in surface-local coordinates.
    ///
    /// The compositor ignores the parts of the opaque region that fall
    /// outside of the surface.
    ///
    /// Opaque region is double-buffered state, see wl_surface.commit.
    ///
    /// wl_surface.set_opaque_region changes the pending opaque region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise, the pending and current regions are never changed.
    ///
    /// The initial value for an opaque region is empty. Setting the pending
    /// opaque region has copy semantics, and the wl_region object can be
    /// destroyed immediately. A NULL wl_region causes the pending opaque
    /// region to be set to empty.
    ///
    /// # Arguments
    ///
    /// - `region`: opaque region of the surface
    #[inline]
    pub fn send_set_opaque_region(
        &self,
        region: Option<&Rc<WlRegion>>,
    ) {
        let res = self.try_send_set_opaque_region(
            region,
        );
        if let Err(e) = res {
            log_send("wl_surface.set_opaque_region", &e);
        }
    }

    /// Since when the set_input_region message is available.
    pub const MSG__SET_INPUT_REGION__SINCE: u32 = 1;

    /// set input region
    ///
    /// This request sets the region of the surface that can receive
    /// pointer and touch events.
    ///
    /// Input events happening outside of this region will try the next
    /// surface in the server surface stack. The compositor ignores the
    /// parts of the input region that fall outside of the surface.
    ///
    /// The input region is specified in surface-local coordinates.
    ///
    /// Input region is double-buffered state, see wl_surface.commit.
    ///
    /// wl_surface.set_input_region changes the pending input region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise the pending and current regions are never changed,
    /// except cursor and icon surfaces are special cases, see
    /// wl_pointer.set_cursor and wl_data_device.start_drag.
    ///
    /// The initial value for an input region is infinite. That means the
    /// whole surface will accept input. Setting the pending input region
    /// has copy semantics, and the wl_region object can be destroyed
    /// immediately. A NULL wl_region causes the input region to be set
    /// to infinite.
    ///
    /// # Arguments
    ///
    /// - `region`: input region of the surface
    #[inline]
    pub fn try_send_set_input_region(
        &self,
        region: Option<&Rc<WlRegion>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            region,
        );
        let arg0 = arg0.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0 {
            None => 0,
            Some(arg0) => match arg0.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("region"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.set_input_region(region: wl_region#{})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id);
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
            5,
            arg0_id,
        ]);
        Ok(())
    }

    /// set input region
    ///
    /// This request sets the region of the surface that can receive
    /// pointer and touch events.
    ///
    /// Input events happening outside of this region will try the next
    /// surface in the server surface stack. The compositor ignores the
    /// parts of the input region that fall outside of the surface.
    ///
    /// The input region is specified in surface-local coordinates.
    ///
    /// Input region is double-buffered state, see wl_surface.commit.
    ///
    /// wl_surface.set_input_region changes the pending input region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise the pending and current regions are never changed,
    /// except cursor and icon surfaces are special cases, see
    /// wl_pointer.set_cursor and wl_data_device.start_drag.
    ///
    /// The initial value for an input region is infinite. That means the
    /// whole surface will accept input. Setting the pending input region
    /// has copy semantics, and the wl_region object can be destroyed
    /// immediately. A NULL wl_region causes the input region to be set
    /// to infinite.
    ///
    /// # Arguments
    ///
    /// - `region`: input region of the surface
    #[inline]
    pub fn send_set_input_region(
        &self,
        region: Option<&Rc<WlRegion>>,
    ) {
        let res = self.try_send_set_input_region(
            region,
        );
        if let Err(e) = res {
            log_send("wl_surface.set_input_region", &e);
        }
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// commit pending surface state
    ///
    /// Surface state (input, opaque, and damage regions, attached buffers,
    /// etc.) is double-buffered. Protocol requests modify the pending state,
    /// as opposed to the active state in use by the compositor.
    ///
    /// A commit request atomically creates a content update from the pending
    /// state, even if the pending state has not been touched. The content
    /// update is placed in a queue until it becomes active. After commit, the
    /// new pending state is as documented for each related request.
    ///
    /// When the content update is applied, the wl_buffer is applied before all
    /// other state. This means that all coordinates in double-buffered state
    /// are relative to the newly attached wl_buffers, except for
    /// wl_surface.attach itself. If there is no newly attached wl_buffer, the
    /// coordinates are relative to the previous content update.
    ///
    /// All requests that need a commit to become effective are documented
    /// to affect double-buffered state.
    ///
    /// Other interfaces may add further double-buffered surface state.
    #[inline]
    pub fn try_send_commit(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.commit()\n", id);
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
            6,
        ]);
        Ok(())
    }

    /// commit pending surface state
    ///
    /// Surface state (input, opaque, and damage regions, attached buffers,
    /// etc.) is double-buffered. Protocol requests modify the pending state,
    /// as opposed to the active state in use by the compositor.
    ///
    /// A commit request atomically creates a content update from the pending
    /// state, even if the pending state has not been touched. The content
    /// update is placed in a queue until it becomes active. After commit, the
    /// new pending state is as documented for each related request.
    ///
    /// When the content update is applied, the wl_buffer is applied before all
    /// other state. This means that all coordinates in double-buffered state
    /// are relative to the newly attached wl_buffers, except for
    /// wl_surface.attach itself. If there is no newly attached wl_buffer, the
    /// coordinates are relative to the previous content update.
    ///
    /// All requests that need a commit to become effective are documented
    /// to affect double-buffered state.
    ///
    /// Other interfaces may add further double-buffered surface state.
    #[inline]
    pub fn send_commit(
        &self,
    ) {
        let res = self.try_send_commit(
        );
        if let Err(e) = res {
            log_send("wl_surface.commit", &e);
        }
    }

    /// Since when the enter message is available.
    pub const MSG__ENTER__SINCE: u32 = 1;

    /// surface enters an output
    ///
    /// This is emitted whenever a surface's creation, movement, or resizing
    /// results in some part of it being within the scanout region of an
    /// output.
    ///
    /// Note that a surface may be overlapping with zero or more outputs.
    ///
    /// # Arguments
    ///
    /// - `output`: output entered by the surface
    #[inline]
    pub fn try_send_enter(
        &self,
        output: &Rc<WlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("output", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_surface#{}.enter(output: wl_output#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0_id,
        ]);
        Ok(())
    }

    /// surface enters an output
    ///
    /// This is emitted whenever a surface's creation, movement, or resizing
    /// results in some part of it being within the scanout region of an
    /// output.
    ///
    /// Note that a surface may be overlapping with zero or more outputs.
    ///
    /// # Arguments
    ///
    /// - `output`: output entered by the surface
    #[inline]
    pub fn send_enter(
        &self,
        output: &Rc<WlOutput>,
    ) {
        let res = self.try_send_enter(
            output,
        );
        if let Err(e) = res {
            log_send("wl_surface.enter", &e);
        }
    }

    /// Since when the leave message is available.
    pub const MSG__LEAVE__SINCE: u32 = 1;

    /// surface leaves an output
    ///
    /// This is emitted whenever a surface's creation, movement, or resizing
    /// results in it no longer having any part of it within the scanout region
    /// of an output.
    ///
    /// Clients should not use the number of outputs the surface is on for frame
    /// throttling purposes. The surface might be hidden even if no leave event
    /// has been sent, and the compositor might expect new surface content
    /// updates even if no enter event has been sent. The frame event should be
    /// used instead.
    ///
    /// # Arguments
    ///
    /// - `output`: output left by the surface
    #[inline]
    pub fn try_send_leave(
        &self,
        output: &Rc<WlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg0.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("output", client.endpoint.id)));
        }
        let arg0_id = arg0.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_surface#{}.leave(output: wl_output#{})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0_id);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// surface leaves an output
    ///
    /// This is emitted whenever a surface's creation, movement, or resizing
    /// results in it no longer having any part of it within the scanout region
    /// of an output.
    ///
    /// Clients should not use the number of outputs the surface is on for frame
    /// throttling purposes. The surface might be hidden even if no leave event
    /// has been sent, and the compositor might expect new surface content
    /// updates even if no enter event has been sent. The frame event should be
    /// used instead.
    ///
    /// # Arguments
    ///
    /// - `output`: output left by the surface
    #[inline]
    pub fn send_leave(
        &self,
        output: &Rc<WlOutput>,
    ) {
        let res = self.try_send_leave(
            output,
        );
        if let Err(e) = res {
            log_send("wl_surface.leave", &e);
        }
    }

    /// Since when the set_buffer_transform message is available.
    pub const MSG__SET_BUFFER_TRANSFORM__SINCE: u32 = 2;

    /// sets the buffer transformation
    ///
    /// This request sets the transformation that the client has already applied
    /// to the content of the buffer. The accepted values for the transform
    /// parameter are the values for wl_output.transform.
    ///
    /// The compositor applies the inverse of this transformation whenever it
    /// uses the buffer contents.
    ///
    /// Buffer transform is double-buffered state, see wl_surface.commit.
    ///
    /// A newly created surface has its buffer transformation set to normal.
    ///
    /// wl_surface.set_buffer_transform changes the pending buffer
    /// transformation. wl_surface.commit copies the pending buffer
    /// transformation to the current one. Otherwise, the pending and current
    /// values are never changed.
    ///
    /// The purpose of this request is to allow clients to render content
    /// according to the output transform, thus permitting the compositor to
    /// use certain optimizations even if the display is rotated. Using
    /// hardware overlays and scanning out a client buffer for fullscreen
    /// surfaces are examples of such optimizations. Those optimizations are
    /// highly dependent on the compositor implementation, so the use of this
    /// request should be considered on a case-by-case basis.
    ///
    /// Note that if the transform value includes 90 or 270 degree rotation,
    /// the width of the buffer will become the surface height and the height
    /// of the buffer will become the surface width.
    ///
    /// If transform is not one of the values from the
    /// wl_output.transform enum the invalid_transform protocol error
    /// is raised.
    ///
    /// # Arguments
    ///
    /// - `transform`: transform for interpreting buffer contents
    #[inline]
    pub fn try_send_set_buffer_transform(
        &self,
        transform: WlOutputTransform,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            transform,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WlOutputTransform) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.set_buffer_transform(transform: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            7,
            arg0.0,
        ]);
        Ok(())
    }

    /// sets the buffer transformation
    ///
    /// This request sets the transformation that the client has already applied
    /// to the content of the buffer. The accepted values for the transform
    /// parameter are the values for wl_output.transform.
    ///
    /// The compositor applies the inverse of this transformation whenever it
    /// uses the buffer contents.
    ///
    /// Buffer transform is double-buffered state, see wl_surface.commit.
    ///
    /// A newly created surface has its buffer transformation set to normal.
    ///
    /// wl_surface.set_buffer_transform changes the pending buffer
    /// transformation. wl_surface.commit copies the pending buffer
    /// transformation to the current one. Otherwise, the pending and current
    /// values are never changed.
    ///
    /// The purpose of this request is to allow clients to render content
    /// according to the output transform, thus permitting the compositor to
    /// use certain optimizations even if the display is rotated. Using
    /// hardware overlays and scanning out a client buffer for fullscreen
    /// surfaces are examples of such optimizations. Those optimizations are
    /// highly dependent on the compositor implementation, so the use of this
    /// request should be considered on a case-by-case basis.
    ///
    /// Note that if the transform value includes 90 or 270 degree rotation,
    /// the width of the buffer will become the surface height and the height
    /// of the buffer will become the surface width.
    ///
    /// If transform is not one of the values from the
    /// wl_output.transform enum the invalid_transform protocol error
    /// is raised.
    ///
    /// # Arguments
    ///
    /// - `transform`: transform for interpreting buffer contents
    #[inline]
    pub fn send_set_buffer_transform(
        &self,
        transform: WlOutputTransform,
    ) {
        let res = self.try_send_set_buffer_transform(
            transform,
        );
        if let Err(e) = res {
            log_send("wl_surface.set_buffer_transform", &e);
        }
    }

    /// Since when the set_buffer_scale message is available.
    pub const MSG__SET_BUFFER_SCALE__SINCE: u32 = 3;

    /// sets the buffer scaling factor
    ///
    /// This request sets an optional scaling factor on how the compositor
    /// interprets the contents of the buffer attached to the window.
    ///
    /// Buffer scale is double-buffered state, see wl_surface.commit.
    ///
    /// A newly created surface has its buffer scale set to 1.
    ///
    /// wl_surface.set_buffer_scale changes the pending buffer scale.
    /// wl_surface.commit copies the pending buffer scale to the current one.
    /// Otherwise, the pending and current values are never changed.
    ///
    /// The purpose of this request is to allow clients to supply higher
    /// resolution buffer data for use on high resolution outputs. It is
    /// intended that you pick the same buffer scale as the scale of the
    /// output that the surface is displayed on. This means the compositor
    /// can avoid scaling when rendering the surface on that output.
    ///
    /// Note that if the scale is larger than 1, then you have to attach
    /// a buffer that is larger (by a factor of scale in each dimension)
    /// than the desired surface size.
    ///
    /// If scale is not greater than 0 the invalid_scale protocol error is
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `scale`: scale for interpreting buffer contents
    #[inline]
    pub fn try_send_set_buffer_scale(
        &self,
        scale: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            scale,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.set_buffer_scale(scale: {})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            8,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// sets the buffer scaling factor
    ///
    /// This request sets an optional scaling factor on how the compositor
    /// interprets the contents of the buffer attached to the window.
    ///
    /// Buffer scale is double-buffered state, see wl_surface.commit.
    ///
    /// A newly created surface has its buffer scale set to 1.
    ///
    /// wl_surface.set_buffer_scale changes the pending buffer scale.
    /// wl_surface.commit copies the pending buffer scale to the current one.
    /// Otherwise, the pending and current values are never changed.
    ///
    /// The purpose of this request is to allow clients to supply higher
    /// resolution buffer data for use on high resolution outputs. It is
    /// intended that you pick the same buffer scale as the scale of the
    /// output that the surface is displayed on. This means the compositor
    /// can avoid scaling when rendering the surface on that output.
    ///
    /// Note that if the scale is larger than 1, then you have to attach
    /// a buffer that is larger (by a factor of scale in each dimension)
    /// than the desired surface size.
    ///
    /// If scale is not greater than 0 the invalid_scale protocol error is
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `scale`: scale for interpreting buffer contents
    #[inline]
    pub fn send_set_buffer_scale(
        &self,
        scale: i32,
    ) {
        let res = self.try_send_set_buffer_scale(
            scale,
        );
        if let Err(e) = res {
            log_send("wl_surface.set_buffer_scale", &e);
        }
    }

    /// Since when the damage_buffer message is available.
    pub const MSG__DAMAGE_BUFFER__SINCE: u32 = 4;

    /// mark part of the surface damaged using buffer coordinates
    ///
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The compositor
    /// ignores the parts of the damage that fall outside of the surface.
    ///
    /// Damage is double-buffered state, see wl_surface.commit.
    ///
    /// The damage rectangle is specified in buffer coordinates,
    /// where x and y specify the upper left corner of the damage rectangle.
    ///
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage_buffer adds pending damage: the new pending
    /// damage is the union of old pending damage and the given rectangle.
    ///
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    ///
    /// This request differs from wl_surface.damage in only one way - it
    /// takes damage in buffer coordinates instead of surface-local
    /// coordinates. While this generally is more intuitive than surface
    /// coordinates, it is especially desirable when using wp_viewport
    /// or when a drawing library (like EGL) is unaware of buffer scale
    /// and buffer transform.
    ///
    /// Note: Because buffer transformation changes and damage requests may
    /// be interleaved in the protocol stream, it is impossible to determine
    /// the actual mapping between surface and buffer damage until
    /// wl_surface.commit time. Therefore, compositors wishing to take both
    /// kinds of damage into account will have to accumulate damage from the
    /// two requests separately and only transform from one to the other
    /// after receiving the wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `x`: buffer-local x coordinate
    /// - `y`: buffer-local y coordinate
    /// - `width`: width of damage rectangle
    /// - `height`: height of damage rectangle
    #[inline]
    pub fn try_send_damage_buffer(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            x,
            y,
            width,
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.damage_buffer(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1, arg2, arg3);
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
            9,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// mark part of the surface damaged using buffer coordinates
    ///
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The compositor
    /// ignores the parts of the damage that fall outside of the surface.
    ///
    /// Damage is double-buffered state, see wl_surface.commit.
    ///
    /// The damage rectangle is specified in buffer coordinates,
    /// where x and y specify the upper left corner of the damage rectangle.
    ///
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage_buffer adds pending damage: the new pending
    /// damage is the union of old pending damage and the given rectangle.
    ///
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    ///
    /// This request differs from wl_surface.damage in only one way - it
    /// takes damage in buffer coordinates instead of surface-local
    /// coordinates. While this generally is more intuitive than surface
    /// coordinates, it is especially desirable when using wp_viewport
    /// or when a drawing library (like EGL) is unaware of buffer scale
    /// and buffer transform.
    ///
    /// Note: Because buffer transformation changes and damage requests may
    /// be interleaved in the protocol stream, it is impossible to determine
    /// the actual mapping between surface and buffer damage until
    /// wl_surface.commit time. Therefore, compositors wishing to take both
    /// kinds of damage into account will have to accumulate damage from the
    /// two requests separately and only transform from one to the other
    /// after receiving the wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `x`: buffer-local x coordinate
    /// - `y`: buffer-local y coordinate
    /// - `width`: width of damage rectangle
    /// - `height`: height of damage rectangle
    #[inline]
    pub fn send_damage_buffer(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_damage_buffer(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("wl_surface.damage_buffer", &e);
        }
    }

    /// Since when the offset message is available.
    pub const MSG__OFFSET__SINCE: u32 = 5;

    /// set the surface contents offset
    ///
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes.
    ///
    /// The exact semantics of wl_surface.offset are role-specific. Refer to
    /// the documentation of specific roles for more information.
    ///
    /// Surface location offset is double-buffered state, see
    /// wl_surface.commit.
    ///
    /// This request is semantically equivalent to and the replaces the x and y
    /// arguments in the wl_surface.attach request in wl_surface versions prior
    /// to 5. See wl_surface.attach for details.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn try_send_offset(
        &self,
        x: i32,
        y: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            x,
            y,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_surface#{}.offset(x: {}, y: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
            10,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// set the surface contents offset
    ///
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes.
    ///
    /// The exact semantics of wl_surface.offset are role-specific. Refer to
    /// the documentation of specific roles for more information.
    ///
    /// Surface location offset is double-buffered state, see
    /// wl_surface.commit.
    ///
    /// This request is semantically equivalent to and the replaces the x and y
    /// arguments in the wl_surface.attach request in wl_surface versions prior
    /// to 5. See wl_surface.attach for details.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn send_offset(
        &self,
        x: i32,
        y: i32,
    ) {
        let res = self.try_send_offset(
            x,
            y,
        );
        if let Err(e) = res {
            log_send("wl_surface.offset", &e);
        }
    }

    /// Since when the preferred_buffer_scale message is available.
    pub const MSG__PREFERRED_BUFFER_SCALE__SINCE: u32 = 6;

    /// preferred buffer scale for the surface
    ///
    /// This event indicates the preferred buffer scale for this surface. It is
    /// sent whenever the compositor's preference changes.
    ///
    /// Before receiving this event the preferred buffer scale for this surface
    /// is 1.
    ///
    /// It is intended that scaling aware clients use this event to scale their
    /// content and use wl_surface.set_buffer_scale to indicate the scale they
    /// have rendered with. This allows clients to supply a higher detail
    /// buffer.
    ///
    /// The compositor shall emit a scale value greater than 0.
    ///
    /// # Arguments
    ///
    /// - `factor`: preferred scaling factor
    #[inline]
    pub fn try_send_preferred_buffer_scale(
        &self,
        factor: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            factor,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_surface#{}.preferred_buffer_scale(factor: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// preferred buffer scale for the surface
    ///
    /// This event indicates the preferred buffer scale for this surface. It is
    /// sent whenever the compositor's preference changes.
    ///
    /// Before receiving this event the preferred buffer scale for this surface
    /// is 1.
    ///
    /// It is intended that scaling aware clients use this event to scale their
    /// content and use wl_surface.set_buffer_scale to indicate the scale they
    /// have rendered with. This allows clients to supply a higher detail
    /// buffer.
    ///
    /// The compositor shall emit a scale value greater than 0.
    ///
    /// # Arguments
    ///
    /// - `factor`: preferred scaling factor
    #[inline]
    pub fn send_preferred_buffer_scale(
        &self,
        factor: i32,
    ) {
        let res = self.try_send_preferred_buffer_scale(
            factor,
        );
        if let Err(e) = res {
            log_send("wl_surface.preferred_buffer_scale", &e);
        }
    }

    /// Since when the preferred_buffer_transform message is available.
    pub const MSG__PREFERRED_BUFFER_TRANSFORM__SINCE: u32 = 6;

    /// preferred buffer transform for the surface
    ///
    /// This event indicates the preferred buffer transform for this surface.
    /// It is sent whenever the compositor's preference changes.
    ///
    /// Before receiving this event the preferred buffer transform for this
    /// surface is normal.
    ///
    /// Applying this transformation to the surface buffer contents and using
    /// wl_surface.set_buffer_transform might allow the compositor to use the
    /// surface buffer more efficiently.
    ///
    /// # Arguments
    ///
    /// - `transform`: preferred transform
    #[inline]
    pub fn try_send_preferred_buffer_transform(
        &self,
        transform: WlOutputTransform,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            transform,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: WlOutputTransform) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_surface#{}.preferred_buffer_transform(transform: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            3,
            arg0.0,
        ]);
        Ok(())
    }

    /// preferred buffer transform for the surface
    ///
    /// This event indicates the preferred buffer transform for this surface.
    /// It is sent whenever the compositor's preference changes.
    ///
    /// Before receiving this event the preferred buffer transform for this
    /// surface is normal.
    ///
    /// Applying this transformation to the surface buffer contents and using
    /// wl_surface.set_buffer_transform might allow the compositor to use the
    /// surface buffer more efficiently.
    ///
    /// # Arguments
    ///
    /// - `transform`: preferred transform
    #[inline]
    pub fn send_preferred_buffer_transform(
        &self,
        transform: WlOutputTransform,
    ) {
        let res = self.try_send_preferred_buffer_transform(
            transform,
        );
        if let Err(e) = res {
            log_send("wl_surface.preferred_buffer_transform", &e);
        }
    }
}

/// A message handler for [`WlSurface`] proxies.
pub trait WlSurfaceHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlSurface>) {
        slf.core.delete_id();
    }

    /// delete surface
    ///
    /// Deletes the surface and invalidates its object ID.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wl_surface.destroy", &e);
        }
    }

    /// set the surface contents
    ///
    /// Set a buffer as the content of this surface.
    ///
    /// The new size of the surface is calculated based on the buffer
    /// size transformed by the inverse buffer_transform and the
    /// inverse buffer_scale. This means that at commit time the supplied
    /// buffer size must be an integer multiple of the buffer_scale. If
    /// that's not the case, an invalid_size error is sent.
    ///
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes. Setting anything other than 0
    /// as x and y arguments is discouraged, and should instead be replaced
    /// with using the separate wl_surface.offset request.
    ///
    /// When the bound wl_surface version is 5 or higher, passing any
    /// non-zero x or y is a protocol violation, and will result in an
    /// 'invalid_offset' error being raised. The x and y arguments are ignored
    /// and do not change the pending state. To achieve equivalent semantics,
    /// use wl_surface.offset.
    ///
    /// Surface contents are double-buffered state, see wl_surface.commit.
    ///
    /// The initial surface contents are void; there is no content.
    /// wl_surface.attach assigns the given wl_buffer as the pending
    /// wl_buffer. wl_surface.commit makes the pending wl_buffer the new
    /// surface contents, and the size of the surface becomes the size
    /// calculated from the wl_buffer, as described above. After commit,
    /// there is no pending buffer until the next attach.
    ///
    /// Committing a pending wl_buffer allows the compositor to read the
    /// pixels in the wl_buffer. The compositor may access the pixels at
    /// any time after the wl_surface.commit request. When the compositor
    /// will not access the pixels anymore, it will send the
    /// wl_buffer.release event. Only after receiving wl_buffer.release,
    /// the client may reuse the wl_buffer. A wl_buffer that has been
    /// attached and then replaced by another attach instead of committed
    /// will not receive a release event, and is not used by the
    /// compositor.
    ///
    /// If a pending wl_buffer has been committed to more than one wl_surface,
    /// the delivery of wl_buffer.release events becomes undefined. A well
    /// behaved client should not rely on wl_buffer.release events in this
    /// case. Alternatively, a client could create multiple wl_buffer objects
    /// from the same backing storage or use a protocol extension providing
    /// per-commit release notifications.
    ///
    /// Destroying the wl_buffer after wl_buffer.release does not change
    /// the surface contents. Destroying the wl_buffer before wl_buffer.release
    /// is allowed as long as the underlying buffer storage isn't re-used (this
    /// can happen e.g. on client process termination). However, if the client
    /// destroys the wl_buffer before receiving the wl_buffer.release event and
    /// mutates the underlying buffer storage, the surface contents become
    /// undefined immediately.
    ///
    /// If wl_surface.attach is sent with a NULL wl_buffer, the
    /// following wl_surface.commit will remove the surface content.
    ///
    /// If a pending wl_buffer has been destroyed, the result is not specified.
    /// Many compositors are known to remove the surface content on the following
    /// wl_surface.commit, but this behaviour is not universal. Clients seeking to
    /// maximise compatibility should not destroy pending buffers and should
    /// ensure that they explicitly remove content from surfaces, even after
    /// destroying buffers.
    ///
    /// # Arguments
    ///
    /// - `buffer`: buffer of surface contents
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_attach(
        &mut self,
        slf: &Rc<WlSurface>,
        buffer: Option<&Rc<WlBuffer>>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_attach(
            buffer,
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("wl_surface.attach", &e);
        }
    }

    /// mark part of the surface damaged
    ///
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The compositor
    /// ignores the parts of the damage that fall outside of the surface.
    ///
    /// Damage is double-buffered state, see wl_surface.commit.
    ///
    /// The damage rectangle is specified in surface-local coordinates,
    /// where x and y specify the upper left corner of the damage rectangle.
    ///
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage adds pending damage: the new pending damage
    /// is the union of old pending damage and the given rectangle.
    ///
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    ///
    /// Note! New clients should not use this request. Instead damage can be
    /// posted with wl_surface.damage_buffer which uses buffer coordinates
    /// instead of surface coordinates.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `width`: width of damage rectangle
    /// - `height`: height of damage rectangle
    #[inline]
    fn handle_damage(
        &mut self,
        slf: &Rc<WlSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_damage(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("wl_surface.damage", &e);
        }
    }

    /// request a frame throttling hint
    ///
    /// Request a notification when it is a good time to start drawing a new
    /// frame, by creating a frame callback. This is useful for throttling
    /// redrawing operations, and driving animations.
    ///
    /// When a client is animating on a wl_surface, it can use the 'frame'
    /// request to get notified when it is a good time to draw and commit the
    /// next frame of animation. If the client commits an update earlier than
    /// that, it is likely that some updates will not make it to the display,
    /// and the client is wasting resources by drawing too often.
    ///
    /// The frame request will take effect on the next wl_surface.commit.
    /// The notification will only be posted for one frame unless
    /// requested again. For a wl_surface, the notifications are posted in
    /// the order the frame requests were committed.
    ///
    /// The server must send the notifications so that a client
    /// will not send excessive updates, while still allowing
    /// the highest possible update rate for clients that wait for the reply
    /// before drawing again. The server should give some time for the client
    /// to draw and commit after sending the frame callback events to let it
    /// hit the next output refresh.
    ///
    /// A server should avoid signaling the frame callbacks if the
    /// surface is not visible in any way, e.g. the surface is off-screen,
    /// or completely obscured by other opaque surfaces.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is the current time, in
    /// milliseconds, with an undefined base.
    ///
    /// # Arguments
    ///
    /// - `callback`: callback object for the frame request
    #[inline]
    fn handle_frame(
        &mut self,
        slf: &Rc<WlSurface>,
        callback: &Rc<WlCallback>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_frame(
            callback,
        );
        if let Err(e) = res {
            log_forward("wl_surface.frame", &e);
        }
    }

    /// set opaque region
    ///
    /// This request sets the region of the surface that contains
    /// opaque content.
    ///
    /// The opaque region is an optimization hint for the compositor
    /// that lets it optimize the redrawing of content behind opaque
    /// regions.  Setting an opaque region is not required for correct
    /// behaviour, but marking transparent content as opaque will result
    /// in repaint artifacts.
    ///
    /// The opaque region is specified in surface-local coordinates.
    ///
    /// The compositor ignores the parts of the opaque region that fall
    /// outside of the surface.
    ///
    /// Opaque region is double-buffered state, see wl_surface.commit.
    ///
    /// wl_surface.set_opaque_region changes the pending opaque region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise, the pending and current regions are never changed.
    ///
    /// The initial value for an opaque region is empty. Setting the pending
    /// opaque region has copy semantics, and the wl_region object can be
    /// destroyed immediately. A NULL wl_region causes the pending opaque
    /// region to be set to empty.
    ///
    /// # Arguments
    ///
    /// - `region`: opaque region of the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_opaque_region(
        &mut self,
        slf: &Rc<WlSurface>,
        region: Option<&Rc<WlRegion>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_opaque_region(
            region,
        );
        if let Err(e) = res {
            log_forward("wl_surface.set_opaque_region", &e);
        }
    }

    /// set input region
    ///
    /// This request sets the region of the surface that can receive
    /// pointer and touch events.
    ///
    /// Input events happening outside of this region will try the next
    /// surface in the server surface stack. The compositor ignores the
    /// parts of the input region that fall outside of the surface.
    ///
    /// The input region is specified in surface-local coordinates.
    ///
    /// Input region is double-buffered state, see wl_surface.commit.
    ///
    /// wl_surface.set_input_region changes the pending input region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise the pending and current regions are never changed,
    /// except cursor and icon surfaces are special cases, see
    /// wl_pointer.set_cursor and wl_data_device.start_drag.
    ///
    /// The initial value for an input region is infinite. That means the
    /// whole surface will accept input. Setting the pending input region
    /// has copy semantics, and the wl_region object can be destroyed
    /// immediately. A NULL wl_region causes the input region to be set
    /// to infinite.
    ///
    /// # Arguments
    ///
    /// - `region`: input region of the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_input_region(
        &mut self,
        slf: &Rc<WlSurface>,
        region: Option<&Rc<WlRegion>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_input_region(
            region,
        );
        if let Err(e) = res {
            log_forward("wl_surface.set_input_region", &e);
        }
    }

    /// commit pending surface state
    ///
    /// Surface state (input, opaque, and damage regions, attached buffers,
    /// etc.) is double-buffered. Protocol requests modify the pending state,
    /// as opposed to the active state in use by the compositor.
    ///
    /// A commit request atomically creates a content update from the pending
    /// state, even if the pending state has not been touched. The content
    /// update is placed in a queue until it becomes active. After commit, the
    /// new pending state is as documented for each related request.
    ///
    /// When the content update is applied, the wl_buffer is applied before all
    /// other state. This means that all coordinates in double-buffered state
    /// are relative to the newly attached wl_buffers, except for
    /// wl_surface.attach itself. If there is no newly attached wl_buffer, the
    /// coordinates are relative to the previous content update.
    ///
    /// All requests that need a commit to become effective are documented
    /// to affect double-buffered state.
    ///
    /// Other interfaces may add further double-buffered surface state.
    #[inline]
    fn handle_commit(
        &mut self,
        slf: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_commit(
        );
        if let Err(e) = res {
            log_forward("wl_surface.commit", &e);
        }
    }

    /// surface enters an output
    ///
    /// This is emitted whenever a surface's creation, movement, or resizing
    /// results in some part of it being within the scanout region of an
    /// output.
    ///
    /// Note that a surface may be overlapping with zero or more outputs.
    ///
    /// # Arguments
    ///
    /// - `output`: output entered by the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_enter(
        &mut self,
        slf: &Rc<WlSurface>,
        output: &Rc<WlOutput>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = output.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_enter(
            output,
        );
        if let Err(e) = res {
            log_forward("wl_surface.enter", &e);
        }
    }

    /// surface leaves an output
    ///
    /// This is emitted whenever a surface's creation, movement, or resizing
    /// results in it no longer having any part of it within the scanout region
    /// of an output.
    ///
    /// Clients should not use the number of outputs the surface is on for frame
    /// throttling purposes. The surface might be hidden even if no leave event
    /// has been sent, and the compositor might expect new surface content
    /// updates even if no enter event has been sent. The frame event should be
    /// used instead.
    ///
    /// # Arguments
    ///
    /// - `output`: output left by the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_leave(
        &mut self,
        slf: &Rc<WlSurface>,
        output: &Rc<WlOutput>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = output.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_leave(
            output,
        );
        if let Err(e) = res {
            log_forward("wl_surface.leave", &e);
        }
    }

    /// sets the buffer transformation
    ///
    /// This request sets the transformation that the client has already applied
    /// to the content of the buffer. The accepted values for the transform
    /// parameter are the values for wl_output.transform.
    ///
    /// The compositor applies the inverse of this transformation whenever it
    /// uses the buffer contents.
    ///
    /// Buffer transform is double-buffered state, see wl_surface.commit.
    ///
    /// A newly created surface has its buffer transformation set to normal.
    ///
    /// wl_surface.set_buffer_transform changes the pending buffer
    /// transformation. wl_surface.commit copies the pending buffer
    /// transformation to the current one. Otherwise, the pending and current
    /// values are never changed.
    ///
    /// The purpose of this request is to allow clients to render content
    /// according to the output transform, thus permitting the compositor to
    /// use certain optimizations even if the display is rotated. Using
    /// hardware overlays and scanning out a client buffer for fullscreen
    /// surfaces are examples of such optimizations. Those optimizations are
    /// highly dependent on the compositor implementation, so the use of this
    /// request should be considered on a case-by-case basis.
    ///
    /// Note that if the transform value includes 90 or 270 degree rotation,
    /// the width of the buffer will become the surface height and the height
    /// of the buffer will become the surface width.
    ///
    /// If transform is not one of the values from the
    /// wl_output.transform enum the invalid_transform protocol error
    /// is raised.
    ///
    /// # Arguments
    ///
    /// - `transform`: transform for interpreting buffer contents
    #[inline]
    fn handle_set_buffer_transform(
        &mut self,
        slf: &Rc<WlSurface>,
        transform: WlOutputTransform,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_buffer_transform(
            transform,
        );
        if let Err(e) = res {
            log_forward("wl_surface.set_buffer_transform", &e);
        }
    }

    /// sets the buffer scaling factor
    ///
    /// This request sets an optional scaling factor on how the compositor
    /// interprets the contents of the buffer attached to the window.
    ///
    /// Buffer scale is double-buffered state, see wl_surface.commit.
    ///
    /// A newly created surface has its buffer scale set to 1.
    ///
    /// wl_surface.set_buffer_scale changes the pending buffer scale.
    /// wl_surface.commit copies the pending buffer scale to the current one.
    /// Otherwise, the pending and current values are never changed.
    ///
    /// The purpose of this request is to allow clients to supply higher
    /// resolution buffer data for use on high resolution outputs. It is
    /// intended that you pick the same buffer scale as the scale of the
    /// output that the surface is displayed on. This means the compositor
    /// can avoid scaling when rendering the surface on that output.
    ///
    /// Note that if the scale is larger than 1, then you have to attach
    /// a buffer that is larger (by a factor of scale in each dimension)
    /// than the desired surface size.
    ///
    /// If scale is not greater than 0 the invalid_scale protocol error is
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `scale`: scale for interpreting buffer contents
    #[inline]
    fn handle_set_buffer_scale(
        &mut self,
        slf: &Rc<WlSurface>,
        scale: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_buffer_scale(
            scale,
        );
        if let Err(e) = res {
            log_forward("wl_surface.set_buffer_scale", &e);
        }
    }

    /// mark part of the surface damaged using buffer coordinates
    ///
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The compositor
    /// ignores the parts of the damage that fall outside of the surface.
    ///
    /// Damage is double-buffered state, see wl_surface.commit.
    ///
    /// The damage rectangle is specified in buffer coordinates,
    /// where x and y specify the upper left corner of the damage rectangle.
    ///
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage_buffer adds pending damage: the new pending
    /// damage is the union of old pending damage and the given rectangle.
    ///
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    ///
    /// This request differs from wl_surface.damage in only one way - it
    /// takes damage in buffer coordinates instead of surface-local
    /// coordinates. While this generally is more intuitive than surface
    /// coordinates, it is especially desirable when using wp_viewport
    /// or when a drawing library (like EGL) is unaware of buffer scale
    /// and buffer transform.
    ///
    /// Note: Because buffer transformation changes and damage requests may
    /// be interleaved in the protocol stream, it is impossible to determine
    /// the actual mapping between surface and buffer damage until
    /// wl_surface.commit time. Therefore, compositors wishing to take both
    /// kinds of damage into account will have to accumulate damage from the
    /// two requests separately and only transform from one to the other
    /// after receiving the wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `x`: buffer-local x coordinate
    /// - `y`: buffer-local y coordinate
    /// - `width`: width of damage rectangle
    /// - `height`: height of damage rectangle
    #[inline]
    fn handle_damage_buffer(
        &mut self,
        slf: &Rc<WlSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_damage_buffer(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("wl_surface.damage_buffer", &e);
        }
    }

    /// set the surface contents offset
    ///
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes.
    ///
    /// The exact semantics of wl_surface.offset are role-specific. Refer to
    /// the documentation of specific roles for more information.
    ///
    /// Surface location offset is double-buffered state, see
    /// wl_surface.commit.
    ///
    /// This request is semantically equivalent to and the replaces the x and y
    /// arguments in the wl_surface.attach request in wl_surface versions prior
    /// to 5. See wl_surface.attach for details.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    fn handle_offset(
        &mut self,
        slf: &Rc<WlSurface>,
        x: i32,
        y: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_offset(
            x,
            y,
        );
        if let Err(e) = res {
            log_forward("wl_surface.offset", &e);
        }
    }

    /// preferred buffer scale for the surface
    ///
    /// This event indicates the preferred buffer scale for this surface. It is
    /// sent whenever the compositor's preference changes.
    ///
    /// Before receiving this event the preferred buffer scale for this surface
    /// is 1.
    ///
    /// It is intended that scaling aware clients use this event to scale their
    /// content and use wl_surface.set_buffer_scale to indicate the scale they
    /// have rendered with. This allows clients to supply a higher detail
    /// buffer.
    ///
    /// The compositor shall emit a scale value greater than 0.
    ///
    /// # Arguments
    ///
    /// - `factor`: preferred scaling factor
    #[inline]
    fn handle_preferred_buffer_scale(
        &mut self,
        slf: &Rc<WlSurface>,
        factor: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preferred_buffer_scale(
            factor,
        );
        if let Err(e) = res {
            log_forward("wl_surface.preferred_buffer_scale", &e);
        }
    }

    /// preferred buffer transform for the surface
    ///
    /// This event indicates the preferred buffer transform for this surface.
    /// It is sent whenever the compositor's preference changes.
    ///
    /// Before receiving this event the preferred buffer transform for this
    /// surface is normal.
    ///
    /// Applying this transformation to the surface buffer contents and using
    /// wl_surface.set_buffer_transform might allow the compositor to use the
    /// surface buffer more efficiently.
    ///
    /// # Arguments
    ///
    /// - `transform`: preferred transform
    #[inline]
    fn handle_preferred_buffer_transform(
        &mut self,
        slf: &Rc<WlSurface>,
        transform: WlOutputTransform,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_preferred_buffer_transform(
            transform,
        );
        if let Err(e) = res {
            log_forward("wl_surface.preferred_buffer_transform", &e);
        }
    }
}

impl ObjectPrivate for WlSurface {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlSurface, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.destroy()\n", client_id, id);
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
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.attach(buffer: wl_buffer#{}, x: {}, y: {})\n", client_id, id, arg0, arg1, arg2);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("buffer", o.core().interface, ObjectInterface::WlBuffer)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_attach(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_attach(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.damage(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_damage(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_damage(&self, arg0, arg1, arg2, arg3);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.frame(callback: wl_callback#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = WlCallback::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "callback", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_frame(&self, arg0);
                } else {
                    DefaultHandler.handle_frame(&self, arg0);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.set_opaque_region(region: wl_region#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlRegion>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("region", o.core().interface, ObjectInterface::WlRegion)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_opaque_region(&self, arg0);
                } else {
                    DefaultHandler.handle_set_opaque_region(&self, arg0);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.set_input_region(region: wl_region#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0 = if arg0 == 0 {
                    None
                } else {
                    let arg0_id = arg0;
                    let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                    };
                    let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlRegion>() else {
                        let o = client.endpoint.lookup(arg0_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("region", o.core().interface, ObjectInterface::WlRegion)));
                    };
                    Some(arg0)
                };
                let arg0 = arg0.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_input_region(&self, arg0);
                } else {
                    DefaultHandler.handle_set_input_region(&self, arg0);
                }
            }
            6 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.commit()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_commit(&self);
                } else {
                    DefaultHandler.handle_commit(&self);
                }
            }
            7 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WlOutputTransform(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WlOutputTransform) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.set_buffer_transform(transform: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_buffer_transform(&self, arg0);
                } else {
                    DefaultHandler.handle_set_buffer_transform(&self, arg0);
                }
            }
            8 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.set_buffer_scale(scale: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_buffer_scale(&self, arg0);
                } else {
                    DefaultHandler.handle_set_buffer_scale(&self, arg0);
                }
            }
            9 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 24)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.damage_buffer(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_damage_buffer(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_damage_buffer(&self, arg0, arg1, arg2, arg3);
                }
            }
            10 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_surface#{}.offset(x: {}, y: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_offset(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_offset(&self, arg0, arg1);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_surface#{}.enter(output: wl_output#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_enter(&self, arg0);
                } else {
                    DefaultHandler.handle_enter(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_surface#{}.leave(output: wl_output#{})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                let arg0_id = arg0;
                let Some(arg0) = server.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = server.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_leave(&self, arg0);
                } else {
                    DefaultHandler.handle_leave(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_surface#{}.preferred_buffer_scale(factor: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preferred_buffer_scale(&self, arg0);
                } else {
                    DefaultHandler.handle_preferred_buffer_scale(&self, arg0);
                }
            }
            3 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WlOutputTransform(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WlOutputTransform) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_surface#{}.preferred_buffer_transform(transform: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_preferred_buffer_transform(&self, arg0);
                } else {
                    DefaultHandler.handle_preferred_buffer_transform(&self, arg0);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "attach",
            2 => "damage",
            3 => "frame",
            4 => "set_opaque_region",
            5 => "set_input_region",
            6 => "commit",
            7 => "set_buffer_transform",
            8 => "set_buffer_scale",
            9 => "damage_buffer",
            10 => "offset",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "enter",
            1 => "leave",
            2 => "preferred_buffer_scale",
            3 => "preferred_buffer_transform",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlSurface {
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

impl WlSurface {
    /// Since when the error.invalid_scale enum variant is available.
    pub const ENM__ERROR_INVALID_SCALE__SINCE: u32 = 1;
    /// Since when the error.invalid_transform enum variant is available.
    pub const ENM__ERROR_INVALID_TRANSFORM__SINCE: u32 = 1;
    /// Since when the error.invalid_size enum variant is available.
    pub const ENM__ERROR_INVALID_SIZE__SINCE: u32 = 1;
    /// Since when the error.invalid_offset enum variant is available.
    pub const ENM__ERROR_INVALID_OFFSET__SINCE: u32 = 1;
    /// Since when the error.defunct_role_object enum variant is available.
    pub const ENM__ERROR_DEFUNCT_ROLE_OBJECT__SINCE: u32 = 1;
}

/// wl_surface error values
///
/// These errors can be emitted in response to wl_surface requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlSurfaceError(pub u32);

impl WlSurfaceError {
    /// buffer scale value is invalid
    pub const INVALID_SCALE: Self = Self(0);

    /// buffer transform value is invalid
    pub const INVALID_TRANSFORM: Self = Self(1);

    /// buffer size is invalid
    pub const INVALID_SIZE: Self = Self(2);

    /// buffer offset is invalid
    pub const INVALID_OFFSET: Self = Self(3);

    /// surface was destroyed before its role object
    pub const DEFUNCT_ROLE_OBJECT: Self = Self(4);
}

impl Debug for WlSurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SCALE => "INVALID_SCALE",
            Self::INVALID_TRANSFORM => "INVALID_TRANSFORM",
            Self::INVALID_SIZE => "INVALID_SIZE",
            Self::INVALID_OFFSET => "INVALID_OFFSET",
            Self::DEFUNCT_ROLE_OBJECT => "DEFUNCT_ROLE_OBJECT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
