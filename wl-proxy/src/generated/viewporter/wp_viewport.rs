//! crop and scale interface to a wl_surface
//!
//! An additional interface to a wl_surface object, which allows the
//! client to specify the cropping and scaling of the surface
//! contents.
//!
//! This interface works with two concepts: the source rectangle (src_x,
//! src_y, src_width, src_height), and the destination size (dst_width,
//! dst_height). The contents of the source rectangle are scaled to the
//! destination size, and content outside the source rectangle is ignored.
//! This state is double-buffered, see wl_surface.commit.
//!
//! The two parts of crop and scale state are independent: the source
//! rectangle, and the destination size. Initially both are unset, that
//! is, no scaling is applied. The whole of the current wl_buffer is
//! used as the source, and the surface size is as defined in
//! wl_surface.attach.
//!
//! If the destination size is set, it causes the surface size to become
//! dst_width, dst_height. The source (rectangle) is scaled to exactly
//! this size. This overrides whatever the attached wl_buffer size is,
//! unless the wl_buffer is NULL. If the wl_buffer is NULL, the surface
//! has no content and therefore no size. Otherwise, the size is always
//! at least 1x1 in surface local coordinates.
//!
//! If the source rectangle is set, it defines what area of the wl_buffer is
//! taken as the source. If the source rectangle is set and the destination
//! size is not set, then src_width and src_height must be integers, and the
//! surface size becomes the source rectangle size. This results in cropping
//! without scaling. If src_width or src_height are not integers and
//! destination size is not set, the bad_size protocol error is raised when
//! the surface state is applied.
//!
//! The coordinate transformations from buffer pixel coordinates up to
//! the surface-local coordinates happen in the following order:
//!   1. buffer_transform (wl_surface.set_buffer_transform)
//!   2. buffer_scale (wl_surface.set_buffer_scale)
//!   3. crop and scale (wp_viewport.set*)
//! This means, that the source rectangle coordinates of crop and scale
//! are given in the coordinates after the buffer transform and scale,
//! i.e. in the coordinates that would be the surface-local coordinates
//! if the crop and scale was not applied.
//!
//! If src_x or src_y are negative, the bad_value protocol error is raised.
//! Otherwise, if the source rectangle is partially or completely outside of
//! the non-NULL wl_buffer, then the out_of_buffer protocol error is raised
//! when the surface state is applied. A NULL wl_buffer does not raise the
//! out_of_buffer error.
//!
//! If the wl_surface associated with the wp_viewport is destroyed,
//! all wp_viewport requests except 'destroy' raise the protocol error
//! no_surface.
//!
//! If the wp_viewport object is destroyed, the crop and scale
//! state is removed from the wl_surface. The change will be applied
//! on the next wl_surface.commit.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_viewport proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpViewport {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpViewportMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpViewportMessageHandler for DefaultMessageHandler { }

impl MetaWpViewport {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpViewport {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpViewport, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpViewport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpViewport")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpViewport {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// remove scaling and cropping from the surface
    ///
    /// The associated wl_surface's crop and scale state is removed.
    /// The change is applied on the next wl_surface.commit.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_source message is available.
    #[allow(dead_code)]
    pub const MSG__SET_SOURCE__SINCE: u32 = 1;

    /// set the source rectangle for cropping
    ///
    /// Set the source rectangle of the associated wl_surface. See
    /// wp_viewport for the description, and relation to the wl_buffer
    /// size.
    ///
    /// If all of x, y, width and height are -1.0, the source rectangle is
    /// unset instead. Any other set of values where width or height are zero
    /// or negative, or x or y are negative, raise the bad_value protocol
    /// error.
    ///
    /// The crop and scale state is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `x`: source rectangle x
    /// - `y`: source rectangle y
    /// - `width`: source rectangle width
    /// - `height`: source rectangle height
    #[inline]
    pub fn send_set_source(
        &self,
        x: Fixed,
        y: Fixed,
        width: Fixed,
        height: Fixed,
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
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0.to_wire() as u32,
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
            arg3.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the set_destination message is available.
    #[allow(dead_code)]
    pub const MSG__SET_DESTINATION__SINCE: u32 = 1;

    /// set the surface size for scaling
    ///
    /// Set the destination size of the associated wl_surface. See
    /// wp_viewport for the description, and relation to the wl_buffer
    /// size.
    ///
    /// If width is -1 and height is -1, the destination size is unset
    /// instead. Any other pair of values for width and height that
    /// contains zero or negative values raises the bad_value protocol
    /// error.
    ///
    /// The crop and scale state is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `width`: surface width
    /// - `height`: surface height
    #[inline]
    pub fn send_set_destination(
        &self,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            width,
            height,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }
}

/// A message handler for [WpViewport] proxies.
#[allow(dead_code)]
pub trait MetaWpViewportMessageHandler {
    /// remove scaling and cropping from the surface
    ///
    /// The associated wl_surface's crop and scale state is removed.
    /// The change is applied on the next wl_surface.commit.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWpViewport>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_viewport.destroy message: {}", Report::new(e));
        }
    }

    /// set the source rectangle for cropping
    ///
    /// Set the source rectangle of the associated wl_surface. See
    /// wp_viewport for the description, and relation to the wl_buffer
    /// size.
    ///
    /// If all of x, y, width and height are -1.0, the source rectangle is
    /// unset instead. Any other set of values where width or height are zero
    /// or negative, or x or y are negative, raise the bad_value protocol
    /// error.
    ///
    /// The crop and scale state is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `x`: source rectangle x
    /// - `y`: source rectangle y
    /// - `width`: source rectangle width
    /// - `height`: source rectangle height
    #[inline]
    fn set_source(
        &mut self,
        _slf: &Rc<MetaWpViewport>,
        x: Fixed,
        y: Fixed,
        width: Fixed,
        height: Fixed,
    ) {
        let res = _slf.send_set_source(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_viewport.set_source message: {}", Report::new(e));
        }
    }

    /// set the surface size for scaling
    ///
    /// Set the destination size of the associated wl_surface. See
    /// wp_viewport for the description, and relation to the wl_buffer
    /// size.
    ///
    /// If width is -1 and height is -1, the destination size is unset
    /// instead. Any other pair of values for width and height that
    /// contains zero or negative values raises the bad_value protocol
    /// error.
    ///
    /// The crop and scale state is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `width`: surface width
    /// - `height`: surface height
    #[inline]
    fn set_destination(
        &mut self,
        _slf: &Rc<MetaWpViewport>,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_set_destination(
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_viewport.set_destination message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpViewport {
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
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = Fixed::from_wire(arg0 as i32);
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = Fixed::from_wire(arg3 as i32);
                if let Some(handler) = handler {
                    (**handler).set_source(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.set_source(&self, arg0, arg1, arg2, arg3);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                if let Some(handler) = handler {
                    (**handler).set_destination(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_destination(&self, arg0, arg1);
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

impl MetaWpViewport {
    /// Since when the error.bad_value enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_VALUE__SINCE: u32 = 1;
    /// Since when the error.bad_size enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_SIZE__SINCE: u32 = 1;
    /// Since when the error.out_of_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_OUT_OF_BUFFER__SINCE: u32 = 1;
    /// Since when the error.no_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_SURFACE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpViewportError(pub u32);

impl MetaWpViewportError {
    /// negative or zero values in width or height
    #[allow(dead_code)]
    pub const BAD_VALUE: Self = Self(0);

    /// destination size is not integer
    #[allow(dead_code)]
    pub const BAD_SIZE: Self = Self(1);

    /// source rectangle extends outside of the content area
    #[allow(dead_code)]
    pub const OUT_OF_BUFFER: Self = Self(2);

    /// the wl_surface was destroyed
    #[allow(dead_code)]
    pub const NO_SURFACE: Self = Self(3);
}

impl Debug for MetaWpViewportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BAD_VALUE => "BAD_VALUE",
            Self::BAD_SIZE => "BAD_SIZE",
            Self::OUT_OF_BUFFER => "OUT_OF_BUFFER",
            Self::NO_SURFACE => "NO_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
