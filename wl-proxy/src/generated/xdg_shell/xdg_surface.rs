//! desktop user interface surface base interface
//!
//! An interface that may be implemented by a wl_surface, for
//! implementations that provide a desktop-style user interface.
//!
//! It provides a base set of functionality required to construct user
//! interface elements requiring management by the compositor, such as
//! toplevel windows, menus, etc. The types of functionality are split into
//! xdg_surface roles.
//!
//! Creating an xdg_surface does not set the role for a wl_surface. In order
//! to map an xdg_surface, the client must create a role-specific object
//! using, e.g., get_toplevel, get_popup. The wl_surface for any given
//! xdg_surface can have at most one role, and may not be assigned any role
//! not based on xdg_surface.
//!
//! A role must be assigned before any other requests are made to the
//! xdg_surface object.
//!
//! The client must call wl_surface.commit on the corresponding wl_surface
//! for the xdg_surface state to take effect.
//!
//! Creating an xdg_surface from a wl_surface which has a buffer attached or
//! committed is a client error, and any attempts by a client to attach or
//! manipulate a buffer prior to the first xdg_surface.configure call must
//! also be treated as errors.
//!
//! After creating a role-specific object and setting it up (e.g. by sending
//! the title, app ID, size constraints, parent, etc), the client must
//! perform an initial commit without any buffer attached. The compositor
//! will reply with initial wl_surface state such as
//! wl_surface.preferred_buffer_scale followed by an xdg_surface.configure
//! event. The client must acknowledge it and is then allowed to attach a
//! buffer to map the surface.
//!
//! Mapping an xdg_surface-based role surface is defined as making it
//! possible for the surface to be shown by the compositor. Note that
//! a mapped surface is not guaranteed to be visible once it is mapped.
//!
//! For an xdg_surface to be mapped by the compositor, the following
//! conditions must be met:
//! (1) the client has assigned an xdg_surface-based role to the surface
//! (2) the client has set and committed the xdg_surface state and the
//! 	  role-dependent state to the surface
//! (3) the client has committed a buffer to the surface
//!
//! A newly-unmapped surface is considered to have met condition (1) out
//! of the 3 required conditions for mapping a surface if its role surface
//! has not been destroyed, i.e. the client must perform the initial commit
//! again before attaching a buffer.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_surface proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgSurface {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgSurfaceMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgSurfaceMessageHandler for DefaultMessageHandler { }

impl MetaXdgSurface {
    pub const XML_VERSION: u32 = 7;
}

impl MetaXdgSurface {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgSurface, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaXdgSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgSurface")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgSurface {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_surface
    ///
    /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
    /// after its role object has been destroyed, otherwise
    /// a defunct_role_object error is raised.
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

    /// Since when the get_toplevel message is available.
    #[allow(dead_code)]
    pub const MSG__GET_TOPLEVEL__SINCE: u32 = 1;

    /// assign the xdg_toplevel surface role
    ///
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    #[inline]
    pub fn send_get_toplevel(
        &self,
        id: &Rc<MetaXdgToplevel>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
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
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the get_popup message is available.
    #[allow(dead_code)]
    pub const MSG__GET_POPUP__SINCE: u32 = 1;

    /// assign the xdg_popup surface role
    ///
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `parent`:
    /// - `positioner`:
    #[inline]
    pub fn send_get_popup(
        &self,
        id: &Rc<MetaXdgPopup>,
        parent: Option<&Rc<MetaXdgSurface>>,
        positioner: &Rc<MetaXdgPositioner>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            parent,
            positioner,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.map(|a| a.core());
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1 {
            None => 0,
            Some(arg1) => match arg1.server_obj_id.get() {
                None => return Err(ObjectError),
                Some(id) => id,
            },
        };
        let arg2 = match arg2.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
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
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the set_window_geometry message is available.
    #[allow(dead_code)]
    pub const MSG__SET_WINDOW_GEOMETRY__SINCE: u32 = 1;

    /// set the new window geometry
    ///
    /// The window geometry of a surface is its "visible bounds" from the
    /// user's perspective. Client-side decorations often have invisible
    /// portions like drop-shadows which should be ignored for the
    /// purposes of aligning, placing and constraining windows. Note that
    /// in some situations, compositors may clip rendering to the window
    /// geometry, so the client should avoid putting functional elements
    /// outside of it.
    ///
    /// The window geometry is double-buffered state, see wl_surface.commit.
    ///
    /// When maintaining a position, the compositor should treat the (x, y)
    /// coordinate of the window geometry as the top left corner of the window.
    /// A client changing the (x, y) window geometry coordinate should in
    /// general not alter the position of the window.
    ///
    /// Once the window geometry of the surface is set, it is not possible to
    /// unset it, and it will remain the same until set_window_geometry is
    /// called again, even if a new subsurface or buffer is attached.
    ///
    /// If never set, the value is the full bounds of the surface,
    /// including any subsurfaces. This updates dynamically on every
    /// commit. This unset is meant for extremely simple clients.
    ///
    /// The arguments are given in the surface-local coordinate space of
    /// the wl_surface associated with this xdg_surface, and may extend outside
    /// of the wl_surface itself to mark parts of the subsurface tree as part of
    /// the window geometry.
    ///
    /// When applied, the effective window geometry will be the set window
    /// geometry clamped to the bounding rectangle of the combined
    /// geometry of the surface of the xdg_surface and the associated
    /// subsurfaces.
    ///
    /// The effective geometry will not be recalculated unless a new call to
    /// set_window_geometry is done and the new pending surface state is
    /// subsequently applied.
    ///
    /// The width and height of the effective window geometry must be
    /// greater than zero. Setting an invalid size will raise an
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn send_set_window_geometry(
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
            3,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// Since when the ack_configure message is available.
    #[allow(dead_code)]
    pub const MSG__ACK_CONFIGURE__SINCE: u32 = 1;

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the
    /// surface in response to the configure event, then the client
    /// must make an ack_configure request sometime before the commit
    /// request, passing along the serial of the configure event.
    ///
    /// For instance, for toplevel surfaces the compositor might use this
    /// information to move a surface to the top left only when the client has
    /// drawn itself for the maximized or fullscreen state.
    ///
    /// If the client receives multiple configure events before it
    /// can respond to one, it only has to ack the last configure event.
    /// Acking a configure event that was never sent raises an invalid_serial
    /// error.
    ///
    /// A client is not required to commit immediately after sending
    /// an ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing, but
    /// only the last request sent before a commit indicates which configure
    /// event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the serial number sent with
    /// the request, as well as serial numbers sent by all configure events
    /// sent on this xdg_surface prior to the configure event referenced by
    /// the committed serial.
    ///
    /// It is an error to issue multiple ack_configure requests referencing a
    /// serial from the same configure event, or to issue an ack_configure
    /// request referencing a serial from a configure event issued before the
    /// event identified by the last ack_configure request for the same
    /// xdg_surface. Doing so will raise an invalid_serial error.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial from the configure event
    #[inline]
    pub fn send_ack_configure(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
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
            4,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the configure message is available.
    #[allow(dead_code)]
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// suggest a surface change
    ///
    /// The configure event marks the end of a configure sequence. A configure
    /// sequence is a set of one or more events configuring the state of the
    /// xdg_surface, including the final xdg_surface.configure event.
    ///
    /// Where applicable, xdg_surface surface roles will during a configure
    /// sequence extend this event as a latched state sent as events before the
    /// xdg_surface.configure event. Such events should be considered to make up
    /// a set of atomically applied configuration states, where the
    /// xdg_surface.configure commits the accumulated state.
    ///
    /// Clients should arrange their surface for the new states, and then send
    /// an ack_configure request with the serial sent in this configure event at
    /// some point before committing the new surface.
    ///
    /// If the client receives multiple configure events before it can respond
    /// to one, it is free to discard all but the last event it received.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the configure event
    #[inline]
    pub fn send_configure(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgSurface] proxies.
#[allow(dead_code)]
pub trait MetaXdgSurfaceMessageHandler {
    /// destroy the xdg_surface
    ///
    /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
    /// after its role object has been destroyed, otherwise
    /// a defunct_role_object error is raised.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgSurface>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_surface.destroy message: {}", Report::new(e));
        }
    }

    /// assign the xdg_toplevel surface role
    ///
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn get_toplevel(
        &mut self,
        _slf: &Rc<MetaXdgSurface>,
        id: &Rc<MetaXdgToplevel>,
    ) {
        let res = _slf.send_get_toplevel(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_surface.get_toplevel message: {}", Report::new(e));
        }
    }

    /// assign the xdg_popup surface role
    ///
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `parent`:
    /// - `positioner`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_popup(
        &mut self,
        _slf: &Rc<MetaXdgSurface>,
        id: &Rc<MetaXdgPopup>,
        parent: Option<&Rc<MetaXdgSurface>>,
        positioner: &Rc<MetaXdgPositioner>,
    ) {
        let res = _slf.send_get_popup(
            id,
            parent,
            positioner,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_surface.get_popup message: {}", Report::new(e));
        }
    }

    /// set the new window geometry
    ///
    /// The window geometry of a surface is its "visible bounds" from the
    /// user's perspective. Client-side decorations often have invisible
    /// portions like drop-shadows which should be ignored for the
    /// purposes of aligning, placing and constraining windows. Note that
    /// in some situations, compositors may clip rendering to the window
    /// geometry, so the client should avoid putting functional elements
    /// outside of it.
    ///
    /// The window geometry is double-buffered state, see wl_surface.commit.
    ///
    /// When maintaining a position, the compositor should treat the (x, y)
    /// coordinate of the window geometry as the top left corner of the window.
    /// A client changing the (x, y) window geometry coordinate should in
    /// general not alter the position of the window.
    ///
    /// Once the window geometry of the surface is set, it is not possible to
    /// unset it, and it will remain the same until set_window_geometry is
    /// called again, even if a new subsurface or buffer is attached.
    ///
    /// If never set, the value is the full bounds of the surface,
    /// including any subsurfaces. This updates dynamically on every
    /// commit. This unset is meant for extremely simple clients.
    ///
    /// The arguments are given in the surface-local coordinate space of
    /// the wl_surface associated with this xdg_surface, and may extend outside
    /// of the wl_surface itself to mark parts of the subsurface tree as part of
    /// the window geometry.
    ///
    /// When applied, the effective window geometry will be the set window
    /// geometry clamped to the bounding rectangle of the combined
    /// geometry of the surface of the xdg_surface and the associated
    /// subsurfaces.
    ///
    /// The effective geometry will not be recalculated unless a new call to
    /// set_window_geometry is done and the new pending surface state is
    /// subsequently applied.
    ///
    /// The width and height of the effective window geometry must be
    /// greater than zero. Setting an invalid size will raise an
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    fn set_window_geometry(
        &mut self,
        _slf: &Rc<MetaXdgSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_set_window_geometry(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_surface.set_window_geometry message: {}", Report::new(e));
        }
    }

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the
    /// surface in response to the configure event, then the client
    /// must make an ack_configure request sometime before the commit
    /// request, passing along the serial of the configure event.
    ///
    /// For instance, for toplevel surfaces the compositor might use this
    /// information to move a surface to the top left only when the client has
    /// drawn itself for the maximized or fullscreen state.
    ///
    /// If the client receives multiple configure events before it
    /// can respond to one, it only has to ack the last configure event.
    /// Acking a configure event that was never sent raises an invalid_serial
    /// error.
    ///
    /// A client is not required to commit immediately after sending
    /// an ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing, but
    /// only the last request sent before a commit indicates which configure
    /// event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the serial number sent with
    /// the request, as well as serial numbers sent by all configure events
    /// sent on this xdg_surface prior to the configure event referenced by
    /// the committed serial.
    ///
    /// It is an error to issue multiple ack_configure requests referencing a
    /// serial from the same configure event, or to issue an ack_configure
    /// request referencing a serial from a configure event issued before the
    /// event identified by the last ack_configure request for the same
    /// xdg_surface. Doing so will raise an invalid_serial error.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial from the configure event
    #[inline]
    fn ack_configure(
        &mut self,
        _slf: &Rc<MetaXdgSurface>,
        serial: u32,
    ) {
        let res = _slf.send_ack_configure(
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_surface.ack_configure message: {}", Report::new(e));
        }
    }

    /// suggest a surface change
    ///
    /// The configure event marks the end of a configure sequence. A configure
    /// sequence is a set of one or more events configuring the state of the
    /// xdg_surface, including the final xdg_surface.configure event.
    ///
    /// Where applicable, xdg_surface surface roles will during a configure
    /// sequence extend this event as a latched state sent as events before the
    /// xdg_surface.configure event. Such events should be considered to make up
    /// a set of atomically applied configuration states, where the
    /// xdg_surface.configure commits the accumulated state.
    ///
    /// Clients should arrange their surface for the new states, and then send
    /// an ack_configure request with the serial sent in this configure event at
    /// some point before committing the new surface.
    ///
    /// If the client receives multiple configure events before it can respond
    /// to one, it is free to discard all but the last event it received.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the configure event
    #[inline]
    fn configure(
        &mut self,
        _slf: &Rc<MetaXdgSurface>,
        serial: u32,
    ) {
        let res = _slf.send_configure(
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_surface.configure message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgSurface {
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
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaXdgToplevel::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_toplevel(&self, arg0);
                } else {
                    DefaultMessageHandler.get_toplevel(&self, arg0);
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
                let arg0_id = arg0;
                let arg0 = MetaXdgPopup::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg1 = if arg1 == 0 {
                    None
                } else {
                    let Some(arg1) = client.endpoint.lookup(arg1) else {
                        return Err(ObjectError);
                    };
                    let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaXdgSurface>() else {
                        return Err(ObjectError);
                    };
                    Some(arg1)
                };
                let Some(arg2) = client.endpoint.lookup(arg2) else {
                    return Err(ObjectError);
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaXdgPositioner>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = arg1.as_ref();
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).get_popup(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.get_popup(&self, arg0, arg1, arg2);
                }
            }
            3 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                if let Some(handler) = handler {
                    (**handler).set_window_geometry(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.set_window_geometry(&self, arg0, arg1, arg2, arg3);
                }
            }
            4 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).ack_configure(&self, arg0);
                } else {
                    DefaultMessageHandler.ack_configure(&self, arg0);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).configure(&self, arg0);
                } else {
                    DefaultMessageHandler.configure(&self, arg0);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

impl MetaXdgSurface {
    /// Since when the error.not_constructed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NOT_CONSTRUCTED__SINCE: u32 = 1;
    /// Since when the error.already_constructed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_CONSTRUCTED__SINCE: u32 = 1;
    /// Since when the error.unconfigured_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_UNCONFIGURED_BUFFER__SINCE: u32 = 1;
    /// Since when the error.invalid_serial enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SERIAL__SINCE: u32 = 1;
    /// Since when the error.invalid_size enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SIZE__SINCE: u32 = 1;
    /// Since when the error.defunct_role_object enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DEFUNCT_ROLE_OBJECT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgSurfaceError(pub u32);

impl MetaXdgSurfaceError {
    /// Surface was not fully constructed
    #[allow(dead_code)]
    pub const NOT_CONSTRUCTED: Self = Self(1);

    /// Surface was already constructed
    #[allow(dead_code)]
    pub const ALREADY_CONSTRUCTED: Self = Self(2);

    /// Attaching a buffer to an unconfigured surface
    #[allow(dead_code)]
    pub const UNCONFIGURED_BUFFER: Self = Self(3);

    /// Invalid serial number when acking a configure event
    #[allow(dead_code)]
    pub const INVALID_SERIAL: Self = Self(4);

    /// Width or height was zero or negative
    #[allow(dead_code)]
    pub const INVALID_SIZE: Self = Self(5);

    /// Surface was destroyed before its role object
    #[allow(dead_code)]
    pub const DEFUNCT_ROLE_OBJECT: Self = Self(6);
}

impl Debug for MetaXdgSurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NOT_CONSTRUCTED => "NOT_CONSTRUCTED",
            Self::ALREADY_CONSTRUCTED => "ALREADY_CONSTRUCTED",
            Self::UNCONFIGURED_BUFFER => "UNCONFIGURED_BUFFER",
            Self::INVALID_SERIAL => "INVALID_SERIAL",
            Self::INVALID_SIZE => "INVALID_SIZE",
            Self::DEFUNCT_ROLE_OBJECT => "DEFUNCT_ROLE_OBJECT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
