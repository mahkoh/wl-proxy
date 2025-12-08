//! create desktop-style surfaces
//!
//! The xdg_wm_base interface is exposed as a global object enabling clients
//! to turn their wl_surfaces into windows in a desktop environment. It
//! defines the basic functionality needed for clients and the compositor to
//! create windows that can be dragged, resized, maximized, etc, as well as
//! creating transient windows such as popup menus.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_wm_base object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgWmBase {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgWmBaseHandler>,
}

struct DefaultHandler;

impl XdgWmBaseHandler for DefaultHandler { }

impl ConcreteObject for XdgWmBase {
    const XML_VERSION: u32 = 7;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgWmBase;
    const INTERFACE_NAME: &str = "xdg_wm_base";
}

impl XdgWmBase {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgWmBaseHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgWmBaseHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgWmBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgWmBase")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgWmBase {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy xdg_wm_base
    ///
    /// Destroy this xdg_wm_base object.
    ///
    /// Destroying a bound xdg_wm_base object while there are surfaces
    /// still alive created by this xdg_wm_base object instance is illegal
    /// and will result in a defunct_surfaces error.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_wm_base#{}.destroy()\n", id);
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

    /// destroy xdg_wm_base
    ///
    /// Destroy this xdg_wm_base object.
    ///
    /// Destroying a bound xdg_wm_base object while there are surfaces
    /// still alive created by this xdg_wm_base object instance is illegal
    /// and will result in a defunct_surfaces error.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_wm_base.destroy", &e);
        }
    }

    /// Since when the create_positioner message is available.
    pub const MSG__CREATE_POSITIONER__SINCE: u32 = 1;

    /// create a positioner object
    ///
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_create_positioner(
        &self,
        id: &Rc<XdgPositioner>,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_wm_base#{}.create_positioner(id: xdg_positioner#{})\n", id, arg0);
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
            1,
            arg0_id,
        ]);
        Ok(())
    }

    /// create a positioner object
    ///
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_create_positioner(
        &self,
        id: &Rc<XdgPositioner>,
    ) {
        let res = self.try_send_create_positioner(
            id,
        );
        if let Err(e) = res {
            log_send("xdg_wm_base.create_positioner", &e);
        }
    }

    /// create a positioner object
    ///
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    #[inline]
    pub fn new_try_send_create_positioner(
        &self,
    ) -> Result<Rc<XdgPositioner>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_positioner(
            &id,
        )?;
        Ok(id)
    }

    /// create a positioner object
    ///
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    #[inline]
    pub fn new_send_create_positioner(
        &self,
    ) -> Rc<XdgPositioner> {
        let id = self.core.create_child();
        self.send_create_positioner(
            &id,
        );
        id
    }

    /// Since when the get_xdg_surface message is available.
    pub const MSG__GET_XDG_SURFACE__SINCE: u32 = 1;

    /// create a shell surface from a surface
    ///
    /// This creates an xdg_surface for the given surface. While xdg_surface
    /// itself is not a role, the corresponding surface may only be assigned
    /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is
    /// illegal to create an xdg_surface for a wl_surface which already has an
    /// assigned role and this will result in a role error.
    ///
    /// This creates an xdg_surface for the given surface. An xdg_surface is
    /// used as basis to define a role to a given surface, such as xdg_toplevel
    /// or xdg_popup. It also manages functionality shared between xdg_surface
    /// based surface roles.
    ///
    /// See the documentation of xdg_surface for more details about what an
    /// xdg_surface is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn try_send_get_xdg_surface(
        &self,
        id: &Rc<XdgSurface>,
        surface: &Rc<WlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_wm_base#{}.get_xdg_surface(id: xdg_surface#{}, surface: wl_surface#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg1_id,
        ]);
        Ok(())
    }

    /// create a shell surface from a surface
    ///
    /// This creates an xdg_surface for the given surface. While xdg_surface
    /// itself is not a role, the corresponding surface may only be assigned
    /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is
    /// illegal to create an xdg_surface for a wl_surface which already has an
    /// assigned role and this will result in a role error.
    ///
    /// This creates an xdg_surface for the given surface. An xdg_surface is
    /// used as basis to define a role to a given surface, such as xdg_toplevel
    /// or xdg_popup. It also manages functionality shared between xdg_surface
    /// based surface roles.
    ///
    /// See the documentation of xdg_surface for more details about what an
    /// xdg_surface is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    #[inline]
    pub fn send_get_xdg_surface(
        &self,
        id: &Rc<XdgSurface>,
        surface: &Rc<WlSurface>,
    ) {
        let res = self.try_send_get_xdg_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log_send("xdg_wm_base.get_xdg_surface", &e);
        }
    }

    /// create a shell surface from a surface
    ///
    /// This creates an xdg_surface for the given surface. While xdg_surface
    /// itself is not a role, the corresponding surface may only be assigned
    /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is
    /// illegal to create an xdg_surface for a wl_surface which already has an
    /// assigned role and this will result in a role error.
    ///
    /// This creates an xdg_surface for the given surface. An xdg_surface is
    /// used as basis to define a role to a given surface, such as xdg_toplevel
    /// or xdg_popup. It also manages functionality shared between xdg_surface
    /// based surface roles.
    ///
    /// See the documentation of xdg_surface for more details about what an
    /// xdg_surface is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_try_send_get_xdg_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Result<Rc<XdgSurface>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_xdg_surface(
            &id,
            surface,
        )?;
        Ok(id)
    }

    /// create a shell surface from a surface
    ///
    /// This creates an xdg_surface for the given surface. While xdg_surface
    /// itself is not a role, the corresponding surface may only be assigned
    /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is
    /// illegal to create an xdg_surface for a wl_surface which already has an
    /// assigned role and this will result in a role error.
    ///
    /// This creates an xdg_surface for the given surface. An xdg_surface is
    /// used as basis to define a role to a given surface, such as xdg_toplevel
    /// or xdg_popup. It also manages functionality shared between xdg_surface
    /// based surface roles.
    ///
    /// See the documentation of xdg_surface for more details about what an
    /// xdg_surface is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn new_send_get_xdg_surface(
        &self,
        surface: &Rc<WlSurface>,
    ) -> Rc<XdgSurface> {
        let id = self.core.create_child();
        self.send_get_xdg_surface(
            &id,
            surface,
        );
        id
    }

    /// Since when the pong message is available.
    pub const MSG__PONG__SINCE: u32 = 1;

    /// respond to a ping event
    ///
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive. See xdg_wm_base.ping
    /// and xdg_wm_base.error.unresponsive.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the ping event
    #[inline]
    pub fn try_send_pong(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_wm_base#{}.pong(serial: {})\n", id, arg0);
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
            3,
            arg0,
        ]);
        Ok(())
    }

    /// respond to a ping event
    ///
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive. See xdg_wm_base.ping
    /// and xdg_wm_base.error.unresponsive.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the ping event
    #[inline]
    pub fn send_pong(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_pong(
            serial,
        );
        if let Err(e) = res {
            log_send("xdg_wm_base.pong", &e);
        }
    }

    /// Since when the ping message is available.
    pub const MSG__PING__SINCE: u32 = 1;

    /// check if the client is alive
    ///
    /// The ping event asks the client if it's still alive. Pass the
    /// serial specified in the event back to the compositor by sending
    /// a "pong" request back with the specified serial. See xdg_wm_base.pong.
    ///
    /// Compositors can use this to determine if the client is still
    /// alive. It's unspecified what will happen if the client doesn't
    /// respond to the ping request, or in what timeframe. Clients should
    /// try to respond in a reasonable amount of time. The “unresponsive”
    /// error is provided for compositors that wish to disconnect unresponsive
    /// clients.
    ///
    /// A compositor is free to ping in any way it wants, but a client must
    /// always respond to any xdg_wm_base object it created.
    ///
    /// # Arguments
    ///
    /// - `serial`: pass this to the pong request
    #[inline]
    pub fn try_send_ping(
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_wm_base#{}.ping(serial: {})\n", client_id, id, arg0);
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
            0,
            arg0,
        ]);
        Ok(())
    }

    /// check if the client is alive
    ///
    /// The ping event asks the client if it's still alive. Pass the
    /// serial specified in the event back to the compositor by sending
    /// a "pong" request back with the specified serial. See xdg_wm_base.pong.
    ///
    /// Compositors can use this to determine if the client is still
    /// alive. It's unspecified what will happen if the client doesn't
    /// respond to the ping request, or in what timeframe. Clients should
    /// try to respond in a reasonable amount of time. The “unresponsive”
    /// error is provided for compositors that wish to disconnect unresponsive
    /// clients.
    ///
    /// A compositor is free to ping in any way it wants, but a client must
    /// always respond to any xdg_wm_base object it created.
    ///
    /// # Arguments
    ///
    /// - `serial`: pass this to the pong request
    #[inline]
    pub fn send_ping(
        &self,
        serial: u32,
    ) {
        let res = self.try_send_ping(
            serial,
        );
        if let Err(e) = res {
            log_send("xdg_wm_base.ping", &e);
        }
    }
}

/// A message handler for [`XdgWmBase`] proxies.
pub trait XdgWmBaseHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgWmBase>) {
        slf.core.delete_id();
    }

    /// destroy xdg_wm_base
    ///
    /// Destroy this xdg_wm_base object.
    ///
    /// Destroying a bound xdg_wm_base object while there are surfaces
    /// still alive created by this xdg_wm_base object instance is illegal
    /// and will result in a defunct_surfaces error.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgWmBase>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_wm_base.destroy", &e);
        }
    }

    /// create a positioner object
    ///
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_create_positioner(
        &mut self,
        slf: &Rc<XdgWmBase>,
        id: &Rc<XdgPositioner>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_positioner(
            id,
        );
        if let Err(e) = res {
            log_forward("xdg_wm_base.create_positioner", &e);
        }
    }

    /// create a shell surface from a surface
    ///
    /// This creates an xdg_surface for the given surface. While xdg_surface
    /// itself is not a role, the corresponding surface may only be assigned
    /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is
    /// illegal to create an xdg_surface for a wl_surface which already has an
    /// assigned role and this will result in a role error.
    ///
    /// This creates an xdg_surface for the given surface. An xdg_surface is
    /// used as basis to define a role to a given surface, such as xdg_toplevel
    /// or xdg_popup. It also manages functionality shared between xdg_surface
    /// based surface roles.
    ///
    /// See the documentation of xdg_surface for more details about what an
    /// xdg_surface is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_get_xdg_surface(
        &mut self,
        slf: &Rc<XdgWmBase>,
        id: &Rc<XdgSurface>,
        surface: &Rc<WlSurface>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_xdg_surface(
            id,
            surface,
        );
        if let Err(e) = res {
            log_forward("xdg_wm_base.get_xdg_surface", &e);
        }
    }

    /// respond to a ping event
    ///
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive. See xdg_wm_base.ping
    /// and xdg_wm_base.error.unresponsive.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the ping event
    #[inline]
    fn handle_pong(
        &mut self,
        slf: &Rc<XdgWmBase>,
        serial: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_pong(
            serial,
        );
        if let Err(e) = res {
            log_forward("xdg_wm_base.pong", &e);
        }
    }

    /// check if the client is alive
    ///
    /// The ping event asks the client if it's still alive. Pass the
    /// serial specified in the event back to the compositor by sending
    /// a "pong" request back with the specified serial. See xdg_wm_base.pong.
    ///
    /// Compositors can use this to determine if the client is still
    /// alive. It's unspecified what will happen if the client doesn't
    /// respond to the ping request, or in what timeframe. Clients should
    /// try to respond in a reasonable amount of time. The “unresponsive”
    /// error is provided for compositors that wish to disconnect unresponsive
    /// clients.
    ///
    /// A compositor is free to ping in any way it wants, but a client must
    /// always respond to any xdg_wm_base object it created.
    ///
    /// # Arguments
    ///
    /// - `serial`: pass this to the pong request
    #[inline]
    fn handle_ping(
        &mut self,
        slf: &Rc<XdgWmBase>,
        serial: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_ping(
            serial,
        );
        if let Err(e) = res {
            log_forward("xdg_wm_base.ping", &e);
        }
    }
}

impl ObjectPrivate for XdgWmBase {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgWmBase, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_wm_base#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_wm_base#{}.create_positioner(id: xdg_positioner#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = XdgPositioner::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_positioner(&self, arg0);
                } else {
                    DefaultHandler.handle_create_positioner(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_wm_base#{}.get_xdg_surface(id: xdg_surface#{}, surface: wl_surface#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = XdgSurface::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_get_xdg_surface(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_get_xdg_surface(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_wm_base#{}.pong(serial: {})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_pong(&self, arg0);
                } else {
                    DefaultHandler.handle_pong(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_wm_base#{}.ping(serial: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ping(&self, arg0);
                } else {
                    DefaultHandler.handle_ping(&self, arg0);
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
            1 => "create_positioner",
            2 => "get_xdg_surface",
            3 => "pong",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "ping",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for XdgWmBase {
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

impl XdgWmBase {
    /// Since when the error.role enum variant is available.
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.defunct_surfaces enum variant is available.
    pub const ENM__ERROR_DEFUNCT_SURFACES__SINCE: u32 = 1;
    /// Since when the error.not_the_topmost_popup enum variant is available.
    pub const ENM__ERROR_NOT_THE_TOPMOST_POPUP__SINCE: u32 = 1;
    /// Since when the error.invalid_popup_parent enum variant is available.
    pub const ENM__ERROR_INVALID_POPUP_PARENT__SINCE: u32 = 1;
    /// Since when the error.invalid_surface_state enum variant is available.
    pub const ENM__ERROR_INVALID_SURFACE_STATE__SINCE: u32 = 1;
    /// Since when the error.invalid_positioner enum variant is available.
    pub const ENM__ERROR_INVALID_POSITIONER__SINCE: u32 = 1;
    /// Since when the error.unresponsive enum variant is available.
    pub const ENM__ERROR_UNRESPONSIVE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgWmBaseError(pub u32);

impl XdgWmBaseError {
    /// given wl_surface has another role
    pub const ROLE: Self = Self(0);

    /// xdg_wm_base was destroyed before children
    pub const DEFUNCT_SURFACES: Self = Self(1);

    /// the client tried to map or destroy a non-topmost popup
    pub const NOT_THE_TOPMOST_POPUP: Self = Self(2);

    /// the client specified an invalid popup parent surface
    pub const INVALID_POPUP_PARENT: Self = Self(3);

    /// the client provided an invalid surface state
    pub const INVALID_SURFACE_STATE: Self = Self(4);

    /// the client provided an invalid positioner
    pub const INVALID_POSITIONER: Self = Self(5);

    /// the client didn’t respond to a ping event in time
    pub const UNRESPONSIVE: Self = Self(6);
}

impl Debug for XdgWmBaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            Self::DEFUNCT_SURFACES => "DEFUNCT_SURFACES",
            Self::NOT_THE_TOPMOST_POPUP => "NOT_THE_TOPMOST_POPUP",
            Self::INVALID_POPUP_PARENT => "INVALID_POPUP_PARENT",
            Self::INVALID_SURFACE_STATE => "INVALID_SURFACE_STATE",
            Self::INVALID_POSITIONER => "INVALID_POSITIONER",
            Self::UNRESPONSIVE => "UNRESPONSIVE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
