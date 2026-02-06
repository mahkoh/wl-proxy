//! a toplevel window icon
//!
//! This interface defines a toplevel icon.
//! An icon can have a name, and multiple buffers.
//! In order to be applied, the icon must have either a name, or at least
//! one buffer assigned. Applying an empty icon (with no buffer or name) to
//! a toplevel should reset its icon to the default icon.
//!
//! It is up to compositor policy whether to prefer using a buffer or loading
//! an icon via its name. See 'set_name' and 'add_buffer' for details.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A xdg_toplevel_icon_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct XdgToplevelIconV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn XdgToplevelIconV1Handler>,
}

struct DefaultHandler;

impl XdgToplevelIconV1Handler for DefaultHandler { }

impl ConcreteObject for XdgToplevelIconV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::XdgToplevelIconV1;
    const INTERFACE_NAME: &str = "xdg_toplevel_icon_v1";
}

impl XdgToplevelIconV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl XdgToplevelIconV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn XdgToplevelIconV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for XdgToplevelIconV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XdgToplevelIconV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl XdgToplevelIconV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the icon object
    ///
    /// Destroys the 'xdg_toplevel_icon_v1' object.
    /// The icon must still remain set on every toplevel it was assigned to,
    /// until the toplevel icon is reset explicitly.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_icon_v1#{}.destroy()\n", id);
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

    /// destroy the icon object
    ///
    /// Destroys the 'xdg_toplevel_icon_v1' object.
    /// The icon must still remain set on every toplevel it was assigned to,
    /// until the toplevel icon is reset explicitly.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_icon_v1.destroy", &e);
        }
    }

    /// Since when the set_name message is available.
    pub const MSG__SET_NAME__SINCE: u32 = 1;

    /// set an icon name
    ///
    /// This request assigns an icon name to this icon.
    /// Any previously set name is overridden.
    ///
    /// The compositor must resolve 'icon_name' according to the lookup rules
    /// described in the XDG icon theme specification[1] using the
    /// environment's current icon theme.
    ///
    /// If the compositor does not support icon names or cannot resolve
    /// 'icon_name' according to the XDG icon theme specification it must
    /// fall back to using pixel buffer data instead.
    ///
    /// If this request is made after the icon has been assigned to a toplevel
    /// via 'set_icon', an 'immutable' error must be raised.
    ///
    /// [1]: https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
    ///
    /// # Arguments
    ///
    /// - `icon_name`:
    #[inline]
    pub fn try_send_set_name(
        &self,
        icon_name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            icon_name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_icon_v1#{}.set_name(icon_name: {:?})\n", id, arg0);
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
            1,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// set an icon name
    ///
    /// This request assigns an icon name to this icon.
    /// Any previously set name is overridden.
    ///
    /// The compositor must resolve 'icon_name' according to the lookup rules
    /// described in the XDG icon theme specification[1] using the
    /// environment's current icon theme.
    ///
    /// If the compositor does not support icon names or cannot resolve
    /// 'icon_name' according to the XDG icon theme specification it must
    /// fall back to using pixel buffer data instead.
    ///
    /// If this request is made after the icon has been assigned to a toplevel
    /// via 'set_icon', an 'immutable' error must be raised.
    ///
    /// [1]: https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
    ///
    /// # Arguments
    ///
    /// - `icon_name`:
    #[inline]
    pub fn send_set_name(
        &self,
        icon_name: &str,
    ) {
        let res = self.try_send_set_name(
            icon_name,
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_icon_v1.set_name", &e);
        }
    }

    /// Since when the add_buffer message is available.
    pub const MSG__ADD_BUFFER__SINCE: u32 = 1;

    /// add icon data from a pixel buffer
    ///
    /// This request adds pixel data supplied as wl_buffer to the icon.
    ///
    /// The client should add pixel data for all icon sizes and scales that
    /// it can provide, or which are explicitly requested by the compositor
    /// via 'icon_size' events on xdg_toplevel_icon_manager_v1.
    ///
    /// The wl_buffer supplying pixel data as 'buffer' must be backed by wl_shm
    /// and must be a square (width and height being equal).
    /// If any of these buffer requirements are not fulfilled, a 'invalid_buffer'
    /// error must be raised.
    ///
    /// If this icon instance already has a buffer of the same size and scale
    /// from a previous 'add_buffer' request, data from the last request
    /// overrides the preexisting pixel data.
    ///
    /// The wl_buffer must be kept alive for as long as the xdg_toplevel_icon
    /// it is associated with is not destroyed, otherwise a 'no_buffer' error
    /// is raised. The buffer contents must not be modified after it was
    /// assigned to the icon. As a result, the region of the wl_shm_pool's
    /// backing storage used for the wl_buffer must not be modified after this
    /// request is sent. The wl_buffer.release event is unused.
    ///
    /// If this request is made after the icon has been assigned to a toplevel
    /// via 'set_icon', an 'immutable' error must be raised.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    /// - `scale`: the scaling factor of the icon, e.g. 1
    #[inline]
    pub fn try_send_add_buffer(
        &self,
        buffer: &Rc<WlBuffer>,
        scale: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            buffer,
            scale,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("buffer"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= xdg_toplevel_icon_v1#{}.add_buffer(buffer: wl_buffer#{}, scale: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg1 as u32,
        ]);
        Ok(())
    }

    /// add icon data from a pixel buffer
    ///
    /// This request adds pixel data supplied as wl_buffer to the icon.
    ///
    /// The client should add pixel data for all icon sizes and scales that
    /// it can provide, or which are explicitly requested by the compositor
    /// via 'icon_size' events on xdg_toplevel_icon_manager_v1.
    ///
    /// The wl_buffer supplying pixel data as 'buffer' must be backed by wl_shm
    /// and must be a square (width and height being equal).
    /// If any of these buffer requirements are not fulfilled, a 'invalid_buffer'
    /// error must be raised.
    ///
    /// If this icon instance already has a buffer of the same size and scale
    /// from a previous 'add_buffer' request, data from the last request
    /// overrides the preexisting pixel data.
    ///
    /// The wl_buffer must be kept alive for as long as the xdg_toplevel_icon
    /// it is associated with is not destroyed, otherwise a 'no_buffer' error
    /// is raised. The buffer contents must not be modified after it was
    /// assigned to the icon. As a result, the region of the wl_shm_pool's
    /// backing storage used for the wl_buffer must not be modified after this
    /// request is sent. The wl_buffer.release event is unused.
    ///
    /// If this request is made after the icon has been assigned to a toplevel
    /// via 'set_icon', an 'immutable' error must be raised.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    /// - `scale`: the scaling factor of the icon, e.g. 1
    #[inline]
    pub fn send_add_buffer(
        &self,
        buffer: &Rc<WlBuffer>,
        scale: i32,
    ) {
        let res = self.try_send_add_buffer(
            buffer,
            scale,
        );
        if let Err(e) = res {
            log_send("xdg_toplevel_icon_v1.add_buffer", &e);
        }
    }
}

/// A message handler for [`XdgToplevelIconV1`] proxies.
pub trait XdgToplevelIconV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<XdgToplevelIconV1>) {
        slf.core.delete_id();
    }

    /// destroy the icon object
    ///
    /// Destroys the 'xdg_toplevel_icon_v1' object.
    /// The icon must still remain set on every toplevel it was assigned to,
    /// until the toplevel icon is reset explicitly.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<XdgToplevelIconV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_icon_v1.destroy", &e);
        }
    }

    /// set an icon name
    ///
    /// This request assigns an icon name to this icon.
    /// Any previously set name is overridden.
    ///
    /// The compositor must resolve 'icon_name' according to the lookup rules
    /// described in the XDG icon theme specification[1] using the
    /// environment's current icon theme.
    ///
    /// If the compositor does not support icon names or cannot resolve
    /// 'icon_name' according to the XDG icon theme specification it must
    /// fall back to using pixel buffer data instead.
    ///
    /// If this request is made after the icon has been assigned to a toplevel
    /// via 'set_icon', an 'immutable' error must be raised.
    ///
    /// [1]: https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
    ///
    /// # Arguments
    ///
    /// - `icon_name`:
    #[inline]
    fn handle_set_name(
        &mut self,
        slf: &Rc<XdgToplevelIconV1>,
        icon_name: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_name(
            icon_name,
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_icon_v1.set_name", &e);
        }
    }

    /// add icon data from a pixel buffer
    ///
    /// This request adds pixel data supplied as wl_buffer to the icon.
    ///
    /// The client should add pixel data for all icon sizes and scales that
    /// it can provide, or which are explicitly requested by the compositor
    /// via 'icon_size' events on xdg_toplevel_icon_manager_v1.
    ///
    /// The wl_buffer supplying pixel data as 'buffer' must be backed by wl_shm
    /// and must be a square (width and height being equal).
    /// If any of these buffer requirements are not fulfilled, a 'invalid_buffer'
    /// error must be raised.
    ///
    /// If this icon instance already has a buffer of the same size and scale
    /// from a previous 'add_buffer' request, data from the last request
    /// overrides the preexisting pixel data.
    ///
    /// The wl_buffer must be kept alive for as long as the xdg_toplevel_icon
    /// it is associated with is not destroyed, otherwise a 'no_buffer' error
    /// is raised. The buffer contents must not be modified after it was
    /// assigned to the icon. As a result, the region of the wl_shm_pool's
    /// backing storage used for the wl_buffer must not be modified after this
    /// request is sent. The wl_buffer.release event is unused.
    ///
    /// If this request is made after the icon has been assigned to a toplevel
    /// via 'set_icon', an 'immutable' error must be raised.
    ///
    /// # Arguments
    ///
    /// - `buffer`:
    /// - `scale`: the scaling factor of the icon, e.g. 1
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_add_buffer(
        &mut self,
        slf: &Rc<XdgToplevelIconV1>,
        buffer: &Rc<WlBuffer>,
        scale: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_add_buffer(
            buffer,
            scale,
        );
        if let Err(e) = res {
            log_forward("xdg_toplevel_icon_v1.add_buffer", &e);
        }
    }
}

impl ObjectPrivate for XdgToplevelIconV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::XdgToplevelIconV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_icon_v1#{}.destroy()\n", client_id, id);
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "icon_name")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_icon_v1#{}.set_name(icon_name: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_name(&self, arg0);
                } else {
                    DefaultHandler.handle_set_name(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> xdg_toplevel_icon_v1#{}.add_buffer(buffer: wl_buffer#{}, scale: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("buffer", o.core().interface, ObjectInterface::WlBuffer)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_add_buffer(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_add_buffer(&self, arg0, arg1);
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
            1 => "set_name",
            2 => "add_buffer",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for XdgToplevelIconV1 {
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

impl XdgToplevelIconV1 {
    /// Since when the error.invalid_buffer enum variant is available.
    pub const ENM__ERROR_INVALID_BUFFER__SINCE: u32 = 1;
    /// Since when the error.immutable enum variant is available.
    pub const ENM__ERROR_IMMUTABLE__SINCE: u32 = 1;
    /// Since when the error.no_buffer enum variant is available.
    pub const ENM__ERROR_NO_BUFFER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct XdgToplevelIconV1Error(pub u32);

impl XdgToplevelIconV1Error {
    /// the provided buffer does not satisfy requirements
    pub const INVALID_BUFFER: Self = Self(1);

    /// the icon has already been assigned to a toplevel and must not be changed
    pub const IMMUTABLE: Self = Self(2);

    /// the provided buffer has been destroyed before the toplevel icon
    pub const NO_BUFFER: Self = Self(3);
}

impl Debug for XdgToplevelIconV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_BUFFER => "INVALID_BUFFER",
            Self::IMMUTABLE => "IMMUTABLE",
            Self::NO_BUFFER => "NO_BUFFER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
