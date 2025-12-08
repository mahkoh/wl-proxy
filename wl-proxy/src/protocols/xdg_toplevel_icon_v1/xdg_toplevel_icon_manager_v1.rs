//! interface to manage toplevel icons
//!
//! This interface allows clients to create toplevel window icons and set
//! them on toplevel windows to be displayed to the user.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_icon_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgToplevelIconManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgToplevelIconManagerV1Handler>,
}

struct DefaultHandler;

impl XdgToplevelIconManagerV1Handler for DefaultHandler { }

impl ConcreteObject for XdgToplevelIconManagerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgToplevelIconManagerV1;
    const INTERFACE_NAME: &str = "xdg_toplevel_icon_manager_v1";
}

impl XdgToplevelIconManagerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgToplevelIconManagerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgToplevelIconManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgToplevelIconManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgToplevelIconManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgToplevelIconManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the toplevel icon manager
    ///
    /// Destroy the toplevel icon manager.
    /// This does not destroy objects created with the manager.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_icon_manager_v1#{}.destroy()\n", id);
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

    /// destroy the toplevel icon manager
    ///
    /// Destroy the toplevel icon manager.
    /// This does not destroy objects created with the manager.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_icon_manager_v1.destroy", &e);
        }
    }

    /// Since when the create_icon message is available.
    pub const MSG__CREATE_ICON__SINCE: u32 = 1;

    /// create a new icon instance
    ///
    /// Creates a new icon object. This icon can then be attached to a
    /// xdg_toplevel via the 'set_icon' request.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn try_send_create_icon(
        &self,
        id: &Rc<XdgToplevelIconV1>,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_icon_manager_v1#{}.create_icon(id: xdg_toplevel_icon_v1#{})\n", id, arg0);
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

    /// create a new icon instance
    ///
    /// Creates a new icon object. This icon can then be attached to a
    /// xdg_toplevel via the 'set_icon' request.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn send_create_icon(
        &self,
        id: &Rc<XdgToplevelIconV1>,
    ) {
        let res = self.try_send_create_icon(
            id,
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_icon_manager_v1.create_icon", &e);
        }
    }

    /// create a new icon instance
    ///
    /// Creates a new icon object. This icon can then be attached to a
    /// xdg_toplevel via the 'set_icon' request.
    #[inline]
    pub fn new_try_send_create_icon(
        &self,
    ) -> Result<Rc<XdgToplevelIconV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_icon(
            &id,
        )?;
        Ok(id)
    }

    /// create a new icon instance
    ///
    /// Creates a new icon object. This icon can then be attached to a
    /// xdg_toplevel via the 'set_icon' request.
    #[inline]
    pub fn new_send_create_icon(
        &self,
    ) -> Rc<XdgToplevelIconV1> {
        let id = self.core.create_child();
        self.send_create_icon(
            &id,
        );
        id
    }

    /// Since when the set_icon message is available.
    pub const MSG__SET_ICON__SINCE: u32 = 1;

    /// set an icon on a toplevel window
    ///
    /// This request assigns the icon 'icon' to 'toplevel', or clears the
    /// toplevel icon if 'icon' was null.
    /// This state is double-buffered and is applied on the next
    /// wl_surface.commit of the toplevel.
    ///
    /// After making this call, the xdg_toplevel_icon_v1 provided as 'icon'
    /// can be destroyed by the client without 'toplevel' losing its icon.
    /// The xdg_toplevel_icon_v1 is immutable from this point, and any
    /// future attempts to change it must raise the
    /// 'xdg_toplevel_icon_v1.immutable' protocol error.
    ///
    /// The compositor must set the toplevel icon from either the pixel data
    /// the icon provides, or by loading a stock icon using the icon name.
    /// See the description of 'xdg_toplevel_icon_v1' for details.
    ///
    /// If 'icon' is set to null, the icon of the respective toplevel is reset
    /// to its default icon (usually the icon of the application, derived from
    /// its desktop-entry file, or a placeholder icon).
    /// If this request is passed an icon with no pixel buffers or icon name
    /// assigned, the icon must be reset just like if 'icon' was null.
    ///
    /// # Arguments
    ///
    /// - `toplevel`: the toplevel to act on
    /// - `icon`:
    #[inline]
    pub fn try_send_set_icon(
        &self,
        toplevel: &Rc<XdgToplevel>,
        icon: Option<&Rc<XdgToplevelIconV1>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            toplevel,
            icon,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("toplevel"))),
            Some(id) => id,
        };
        let arg1_id = match arg1 {
            None => 0,
            Some(arg1) => match arg1.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("icon"))),
                Some(id) => id,
            },
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_icon_manager_v1#{}.set_icon(toplevel: xdg_toplevel#{}, icon: xdg_toplevel_icon_v1#{})\n", id, arg0, arg1);
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

    /// set an icon on a toplevel window
    ///
    /// This request assigns the icon 'icon' to 'toplevel', or clears the
    /// toplevel icon if 'icon' was null.
    /// This state is double-buffered and is applied on the next
    /// wl_surface.commit of the toplevel.
    ///
    /// After making this call, the xdg_toplevel_icon_v1 provided as 'icon'
    /// can be destroyed by the client without 'toplevel' losing its icon.
    /// The xdg_toplevel_icon_v1 is immutable from this point, and any
    /// future attempts to change it must raise the
    /// 'xdg_toplevel_icon_v1.immutable' protocol error.
    ///
    /// The compositor must set the toplevel icon from either the pixel data
    /// the icon provides, or by loading a stock icon using the icon name.
    /// See the description of 'xdg_toplevel_icon_v1' for details.
    ///
    /// If 'icon' is set to null, the icon of the respective toplevel is reset
    /// to its default icon (usually the icon of the application, derived from
    /// its desktop-entry file, or a placeholder icon).
    /// If this request is passed an icon with no pixel buffers or icon name
    /// assigned, the icon must be reset just like if 'icon' was null.
    ///
    /// # Arguments
    ///
    /// - `toplevel`: the toplevel to act on
    /// - `icon`:
    #[inline]
    pub fn send_set_icon(
        &self,
        toplevel: &Rc<XdgToplevel>,
        icon: Option<&Rc<XdgToplevelIconV1>>,
    ) {
        let res = self.try_send_set_icon(
            toplevel,
            icon,
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_icon_manager_v1.set_icon", &e);
        }
    }

    /// Since when the icon_size message is available.
    pub const MSG__ICON_SIZE__SINCE: u32 = 1;

    /// describes a supported & preferred icon size
    ///
    /// This event indicates an icon size the compositor prefers to be
    /// available if the client has scalable icons and can render to any size.
    ///
    /// When the 'xdg_toplevel_icon_manager_v1' object is created, the
    /// compositor may send one or more 'icon_size' events to describe the list
    /// of preferred icon sizes. If the compositor has no size preference, it
    /// may not send any 'icon_size' event, and it is up to the client to
    /// decide a suitable icon size.
    ///
    /// A sequence of 'icon_size' events must be finished with a 'done' event.
    /// If the compositor has no size preferences, it must still send the
    /// 'done' event, without any preceding 'icon_size' events.
    ///
    /// # Arguments
    ///
    /// - `size`: the edge size of the square icon in surface-local coordinates, e.g. 64
    #[inline]
    pub fn try_send_icon_size(
        &self,
        size: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            size,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_toplevel_icon_manager_v1#{}.icon_size(size: {})\n", client_id, id, arg0);
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
            arg0 as u32,
        ]);
        Ok(())
    }

    /// describes a supported & preferred icon size
    ///
    /// This event indicates an icon size the compositor prefers to be
    /// available if the client has scalable icons and can render to any size.
    ///
    /// When the 'xdg_toplevel_icon_manager_v1' object is created, the
    /// compositor may send one or more 'icon_size' events to describe the list
    /// of preferred icon sizes. If the compositor has no size preference, it
    /// may not send any 'icon_size' event, and it is up to the client to
    /// decide a suitable icon size.
    ///
    /// A sequence of 'icon_size' events must be finished with a 'done' event.
    /// If the compositor has no size preferences, it must still send the
    /// 'done' event, without any preceding 'icon_size' events.
    ///
    /// # Arguments
    ///
    /// - `size`: the edge size of the square icon in surface-local coordinates, e.g. 64
    #[inline]
    pub fn send_icon_size(
        &self,
        size: i32,
    ) {
        let res = self.try_send_icon_size(
            size,
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_icon_manager_v1.icon_size", &e);
        }
    }

    /// Since when the done message is available.
    pub const MSG__DONE__SINCE: u32 = 1;

    /// all information has been sent
    ///
    /// This event is sent after all 'icon_size' events have been sent.
    #[inline]
    pub fn try_send_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= xdg_toplevel_icon_manager_v1#{}.done()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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
        ]);
        Ok(())
    }

    /// all information has been sent
    ///
    /// This event is sent after all 'icon_size' events have been sent.
    #[inline]
    pub fn send_done(
        &self,
    ) {
        let res = self.try_send_done(
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_icon_manager_v1.done", &e);
        }
    }
}

/// A message handler for [`XdgToplevelIconManagerV1`] proxies.
pub trait XdgToplevelIconManagerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgToplevelIconManagerV1>) {
        slf.core.delete_id();
    }

    /// destroy the toplevel icon manager
    ///
    /// Destroy the toplevel icon manager.
    /// This does not destroy objects created with the manager.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgToplevelIconManagerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_icon_manager_v1.destroy", &e);
        }
    }

    /// create a new icon instance
    ///
    /// Creates a new icon object. This icon can then be attached to a
    /// xdg_toplevel via the 'set_icon' request.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn handle_create_icon(
        &mut self,
        slf: &Rc<XdgToplevelIconManagerV1>,
        id: &Rc<XdgToplevelIconV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_icon(
            id,
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_icon_manager_v1.create_icon", &e);
        }
    }

    /// set an icon on a toplevel window
    ///
    /// This request assigns the icon 'icon' to 'toplevel', or clears the
    /// toplevel icon if 'icon' was null.
    /// This state is double-buffered and is applied on the next
    /// wl_surface.commit of the toplevel.
    ///
    /// After making this call, the xdg_toplevel_icon_v1 provided as 'icon'
    /// can be destroyed by the client without 'toplevel' losing its icon.
    /// The xdg_toplevel_icon_v1 is immutable from this point, and any
    /// future attempts to change it must raise the
    /// 'xdg_toplevel_icon_v1.immutable' protocol error.
    ///
    /// The compositor must set the toplevel icon from either the pixel data
    /// the icon provides, or by loading a stock icon using the icon name.
    /// See the description of 'xdg_toplevel_icon_v1' for details.
    ///
    /// If 'icon' is set to null, the icon of the respective toplevel is reset
    /// to its default icon (usually the icon of the application, derived from
    /// its desktop-entry file, or a placeholder icon).
    /// If this request is passed an icon with no pixel buffers or icon name
    /// assigned, the icon must be reset just like if 'icon' was null.
    ///
    /// # Arguments
    ///
    /// - `toplevel`: the toplevel to act on
    /// - `icon`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_icon(
        &mut self,
        slf: &Rc<XdgToplevelIconManagerV1>,
        toplevel: &Rc<XdgToplevel>,
        icon: Option<&Rc<XdgToplevelIconV1>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_icon(
            toplevel,
            icon,
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_icon_manager_v1.set_icon", &e);
        }
    }

    /// describes a supported & preferred icon size
    ///
    /// This event indicates an icon size the compositor prefers to be
    /// available if the client has scalable icons and can render to any size.
    ///
    /// When the 'xdg_toplevel_icon_manager_v1' object is created, the
    /// compositor may send one or more 'icon_size' events to describe the list
    /// of preferred icon sizes. If the compositor has no size preference, it
    /// may not send any 'icon_size' event, and it is up to the client to
    /// decide a suitable icon size.
    ///
    /// A sequence of 'icon_size' events must be finished with a 'done' event.
    /// If the compositor has no size preferences, it must still send the
    /// 'done' event, without any preceding 'icon_size' events.
    ///
    /// # Arguments
    ///
    /// - `size`: the edge size of the square icon in surface-local coordinates, e.g. 64
    #[inline]
    fn handle_icon_size(
        &mut self,
        slf: &Rc<XdgToplevelIconManagerV1>,
        size: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_icon_size(
            size,
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_icon_manager_v1.icon_size", &e);
        }
    }

    /// all information has been sent
    ///
    /// This event is sent after all 'icon_size' events have been sent.
    #[inline]
    fn handle_done(
        &mut self,
        slf: &Rc<XdgToplevelIconManagerV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_done(
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_icon_manager_v1.done", &e);
        }
    }
}

impl ObjectPrivate for XdgToplevelIconManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgToplevelIconManagerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_icon_manager_v1#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_icon_manager_v1#{}.create_icon(id: xdg_toplevel_icon_v1#{})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                let arg0_id = arg0;
                let arg0 = XdgToplevelIconV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_icon(&self, arg0);
                } else {
                    DefaultHandler.handle_create_icon(&self, arg0);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_icon_manager_v1#{}.set_icon(toplevel: xdg_toplevel#{}, icon: xdg_toplevel_icon_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<XdgToplevel>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("toplevel", o.core().interface, ObjectInterface::XdgToplevel)));
                };
                let arg1 = if arg1 == 0 {
                    None
                } else {
                    let arg1_id = arg1;
                    let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                    };
                    let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<XdgToplevelIconV1>() else {
                        let o = client.endpoint.lookup(arg1_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("icon", o.core().interface, ObjectInterface::XdgToplevelIconV1)));
                    };
                    Some(arg1)
                };
                let arg0 = &arg0;
                let arg1 = arg1.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_set_icon(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_icon(&self, arg0, arg1);
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
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_toplevel_icon_manager_v1#{}.icon_size(size: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_icon_size(&self, arg0);
                } else {
                    DefaultHandler.handle_icon_size(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> xdg_toplevel_icon_manager_v1#{}.done()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_done(&self);
                } else {
                    DefaultHandler.handle_done(&self);
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
            1 => "create_icon",
            2 => "set_icon",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "icon_size",
            1 => "done",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for XdgToplevelIconManagerV1 {
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

