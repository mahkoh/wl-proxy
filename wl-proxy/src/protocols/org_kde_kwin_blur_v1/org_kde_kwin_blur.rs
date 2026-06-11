use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A org_kde_kwin_blur object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct OrgKdeKwinBlur {
    core: ObjectCore,
    handler: HandlerHolder<dyn OrgKdeKwinBlurHandler>,
}

struct DefaultHandler;

impl OrgKdeKwinBlurHandler for DefaultHandler { }

impl ConcreteObject for OrgKdeKwinBlur {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::OrgKdeKwinBlur;
    const INTERFACE_NAME: &str = "org_kde_kwin_blur";
}

impl OrgKdeKwinBlur {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl OrgKdeKwinBlurHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn OrgKdeKwinBlurHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for OrgKdeKwinBlur {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrgKdeKwinBlur")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl OrgKdeKwinBlur {
    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= org_kde_kwin_blur#{}.commit()\n", id);
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
        Ok(())
    }

    #[inline]
    pub fn send_commit(
        &self,
    ) {
        let res = self.try_send_commit(
        );
        if let Err(e) = res {
            log_send("org_kde_kwin_blur.commit", &e);
        }
    }

    /// Since when the set_region message is available.
    pub const MSG__SET_REGION__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `region`:
    #[inline]
    pub fn try_send_set_region(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= org_kde_kwin_blur#{}.set_region(region: wl_region#{})\n", id, arg0);
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

    /// # Arguments
    ///
    /// - `region`:
    #[inline]
    pub fn send_set_region(
        &self,
        region: Option<&Rc<WlRegion>>,
    ) {
        let res = self.try_send_set_region(
            region,
        );
        if let Err(e) = res {
            log_send("org_kde_kwin_blur.set_region", &e);
        }
    }

    /// Since when the release message is available.
    pub const MSG__RELEASE__SINCE: u32 = 1;

    /// release the blur object
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= org_kde_kwin_blur#{}.release()\n", id);
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// release the blur object
    #[inline]
    pub fn send_release(
        &self,
    ) {
        let res = self.try_send_release(
        );
        if let Err(e) = res {
            log_send("org_kde_kwin_blur.release", &e);
        }
    }
}

/// A message handler for [`OrgKdeKwinBlur`] proxies.
pub trait OrgKdeKwinBlurHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<OrgKdeKwinBlur>) {
        slf.core.delete_id();
    }

    #[inline]
    fn handle_commit(
        &mut self,
        slf: &Rc<OrgKdeKwinBlur>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_commit(
        );
        if let Err(e) = res {
            log_forward("org_kde_kwin_blur.commit", &e);
        }
    }

    /// # Arguments
    ///
    /// - `region`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_set_region(
        &mut self,
        slf: &Rc<OrgKdeKwinBlur>,
        region: Option<&Rc<WlRegion>>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_region(
            region,
        );
        if let Err(e) = res {
            log_forward("org_kde_kwin_blur.set_region", &e);
        }
    }

    /// release the blur object
    #[inline]
    fn handle_release(
        &mut self,
        slf: &Rc<OrgKdeKwinBlur>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_release(
        );
        if let Err(e) = res {
            log_forward("org_kde_kwin_blur.release", &e);
        }
    }
}

impl ObjectPrivate for OrgKdeKwinBlur {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::OrgKdeKwinBlur, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> org_kde_kwin_blur#{}.commit()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> org_kde_kwin_blur#{}.set_region(region: wl_region#{})\n", client_id, id, arg0);
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
                    (**handler).handle_set_region(&self, arg0);
                } else {
                    DefaultHandler.handle_set_region(&self, arg0);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> org_kde_kwin_blur#{}.release()\n", client_id, id);
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
            0 => "commit",
            1 => "set_region",
            2 => "release",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for OrgKdeKwinBlur {
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

