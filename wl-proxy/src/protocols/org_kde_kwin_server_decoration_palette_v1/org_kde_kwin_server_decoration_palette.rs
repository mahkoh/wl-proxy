//! server side decoration palette interface
//!
//! This interface allows a client to alter the palette of a server side decoration.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A org_kde_kwin_server_decoration_palette object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct OrgKdeKwinServerDecorationPalette {
    core: ObjectCore,
    handler: HandlerHolder<dyn OrgKdeKwinServerDecorationPaletteHandler>,
}

struct DefaultHandler;

impl OrgKdeKwinServerDecorationPaletteHandler for DefaultHandler { }

impl ConcreteObject for OrgKdeKwinServerDecorationPalette {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::OrgKdeKwinServerDecorationPalette;
    const INTERFACE_NAME: &str = "org_kde_kwin_server_decoration_palette";
}

impl OrgKdeKwinServerDecorationPalette {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl OrgKdeKwinServerDecorationPaletteHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn OrgKdeKwinServerDecorationPaletteHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for OrgKdeKwinServerDecorationPalette {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrgKdeKwinServerDecorationPalette")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl OrgKdeKwinServerDecorationPalette {
    /// Since when the set_palette message is available.
    pub const MSG__SET_PALETTE__SINCE: u32 = 1;

    /// Set a on the server side window decoration
    ///
    /// Color scheme that should be applied to the window decoration.
    /// Absolute file path, or name of palette in the user's config directory.
    /// The server may choose not to follow the requested style.
    ///
    /// # Arguments
    ///
    /// - `palette`: Absolute file path, or name of palette in the user's config directory
    #[inline]
    pub fn try_send_set_palette(
        &self,
        palette: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            palette,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= org_kde_kwin_server_decoration_palette#{}.set_palette(palette: {:?})\n", id, arg0);
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
            0,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Set a on the server side window decoration
    ///
    /// Color scheme that should be applied to the window decoration.
    /// Absolute file path, or name of palette in the user's config directory.
    /// The server may choose not to follow the requested style.
    ///
    /// # Arguments
    ///
    /// - `palette`: Absolute file path, or name of palette in the user's config directory
    #[inline]
    pub fn send_set_palette(
        &self,
        palette: &str,
    ) {
        let res = self.try_send_set_palette(
            palette,
        );
        if let Err(e) = res {
            log_send("org_kde_kwin_server_decoration_palette.set_palette", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// release the palette object
    #[inline]
    pub fn try_send_release(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= org_kde_kwin_server_decoration_palette#{}.release()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// release the palette object
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("org_kde_kwin_server_decoration_palette.release", &e);
        }
    }
}

/// A message handler for [`OrgKdeKwinServerDecorationPalette`] proxies.
pub trait OrgKdeKwinServerDecorationPaletteHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<OrgKdeKwinServerDecorationPalette>) {
        slf.core.delete_id();
    }

    /// Set a on the server side window decoration
    ///
    /// Color scheme that should be applied to the window decoration.
    /// Absolute file path, or name of palette in the user's config directory.
    /// The server may choose not to follow the requested style.
    ///
    /// # Arguments
    ///
    /// - `palette`: Absolute file path, or name of palette in the user's config directory
    #[inline]
    fn handle_set_palette(
        &mut self,
        slf: &Rc<OrgKdeKwinServerDecorationPalette>,
        palette: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_palette(
            palette,
        );
        if let Err(e) = res {
            log_forward("org_kde_kwin_server_decoration_palette.set_palette", &e);
        }
    }

    /// release the palette object
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<OrgKdeKwinServerDecorationPalette>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("org_kde_kwin_server_decoration_palette.release", &e);
        }
    }
}

impl ObjectPrivate for OrgKdeKwinServerDecorationPalette {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::OrgKdeKwinServerDecorationPalette, version),
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "palette")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> org_kde_kwin_server_decoration_palette#{}.set_palette(palette: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_palette(&self, arg0);
                } else {
                    DefaultHandler.handle_set_palette(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> org_kde_kwin_server_decoration_palette#{}.release()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_release(&self);
                } else {
                    DefaultHandler.handle_release(&self);
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
            0 => "set_palette",
            1 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for OrgKdeKwinServerDecorationPalette {
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

