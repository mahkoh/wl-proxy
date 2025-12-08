//! region interface
//!
//! A region object describes an area.
//!
//! Region objects are used to describe the opaque and input
//! regions of a surface.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_region object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlRegion {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlRegionHandler>,
}

struct DefaultHandler;

impl WlRegionHandler for DefaultHandler { }

impl ConcreteObject for WlRegion {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WlRegion;
    const INTERFACE_NAME: &str = "wl_region";
}

impl WlRegion {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlRegionHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlRegionHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlRegion")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlRegion {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy region
    ///
    /// Destroy the region.  This will invalidate the object ID.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_region#{}.destroy()\n", id);
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

    /// destroy region
    ///
    /// Destroy the region.  This will invalidate the object ID.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wl_region.destroy", &e);
        }
    }

    /// Since when the add message is available.
    pub const MSG__ADD__SINCE: u32 = 1;

    /// add rectangle to region
    ///
    /// Add the specified rectangle to the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    pub fn try_send_add(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_region#{}.add(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
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
            1,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// add rectangle to region
    ///
    /// Add the specified rectangle to the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    pub fn send_add(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_add(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("wl_region.add", &e);
        }
    }

    /// Since when the subtract message is available.
    pub const MSG__SUBTRACT__SINCE: u32 = 1;

    /// subtract rectangle from region
    ///
    /// Subtract the specified rectangle from the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    pub fn try_send_subtract(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_region#{}.subtract(x: {}, y: {}, width: {}, height: {})\n", id, arg0, arg1, arg2, arg3);
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

    /// subtract rectangle from region
    ///
    /// Subtract the specified rectangle from the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    pub fn send_subtract(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_subtract(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_send("wl_region.subtract", &e);
        }
    }
}

/// A message handler for [`WlRegion`] proxies.
pub trait WlRegionHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlRegion>) {
        slf.core.delete_id();
    }

    /// destroy region
    ///
    /// Destroy the region.  This will invalidate the object ID.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WlRegion>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wl_region.destroy", &e);
        }
    }

    /// add rectangle to region
    ///
    /// Add the specified rectangle to the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    fn handle_add(
        &mut self,
        slf: &Rc<WlRegion>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_add(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("wl_region.add", &e);
        }
    }

    /// subtract rectangle from region
    ///
    /// Subtract the specified rectangle from the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    fn handle_subtract(
        &mut self,
        slf: &Rc<WlRegion>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_subtract(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("wl_region.subtract", &e);
        }
    }
}

impl ObjectPrivate for WlRegion {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlRegion, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_region#{}.destroy()\n", client_id, id);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_region#{}.add(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_add(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_add(&self, arg0, arg1, arg2, arg3);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_region#{}.subtract(x: {}, y: {}, width: {}, height: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_subtract(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_subtract(&self, arg0, arg1, arg2, arg3);
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
            1 => "add",
            2 => "subtract",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WlRegion {
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

