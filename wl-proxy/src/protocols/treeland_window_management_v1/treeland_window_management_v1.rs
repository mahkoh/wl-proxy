//! application windows management
//!
//! This interface manages application windows.
//! It provides requests to show and hide the desktop and emits
//! an event every time a window is created so that the client can
//! use it to manage the window.
//!
//! Only one client can bind this interface at a time.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_window_management_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandWindowManagementV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandWindowManagementV1Handler>,
}

struct DefaultHandler;

impl TreelandWindowManagementV1Handler for DefaultHandler { }

impl TreelandWindowManagementV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::TreelandWindowManagementV1;
    pub const INTERFACE_NAME: &str = "treeland_window_management_v1";
}

impl TreelandWindowManagementV1 {
    pub fn set_handler(&self, handler: impl TreelandWindowManagementV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandWindowManagementV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandWindowManagementV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandWindowManagementV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandWindowManagementV1 {
    /// Since when the set_desktop message is available.
    pub const MSG__SET_DESKTOP__SINCE: u32 = 1;

    /// show/hide the desktop
    ///
    /// Tell the compositor to show/hide the desktop.
    ///
    /// # Arguments
    ///
    /// - `state`: requested state
    #[inline]
    pub fn send_set_desktop(
        &self,
        state: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_window_management_v1#{}.set_desktop(state: {})\n", id, arg0);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
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

    /// Since when the show_desktop message is available.
    pub const MSG__SHOW_DESKTOP__SINCE: u32 = 1;

    /// notify the client when the show desktop mode is entered/left
    ///
    /// This event will be sent whenever the show desktop mode changes. E.g. when it is
    /// entered
    /// or left.
    ///
    /// On binding the interface the current state is sent.
    ///
    /// # Arguments
    ///
    /// - `state`: new state
    #[inline]
    pub fn send_show_desktop(
        &self,
        state: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            state,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_window_management_v1#{}.show_desktop(state: {})\n", client.endpoint.id, id, arg0);
            self.core.state.log(args);
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

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the window manager object
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_window_management_v1#{}.destroy()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
}

/// A message handler for [TreelandWindowManagementV1] proxies.
pub trait TreelandWindowManagementV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandWindowManagementV1>) {
        let _ = slf.core.delete_id();
    }

    /// show/hide the desktop
    ///
    /// Tell the compositor to show/hide the desktop.
    ///
    /// # Arguments
    ///
    /// - `state`: requested state
    #[inline]
    fn handle_set_desktop(
        &mut self,
        _slf: &Rc<TreelandWindowManagementV1>,
        state: u32,
    ) {
        let res = _slf.send_set_desktop(
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_window_management_v1.set_desktop message: {}", Report::new(e));
        }
    }

    /// notify the client when the show desktop mode is entered/left
    ///
    /// This event will be sent whenever the show desktop mode changes. E.g. when it is
    /// entered
    /// or left.
    ///
    /// On binding the interface the current state is sent.
    ///
    /// # Arguments
    ///
    /// - `state`: new state
    #[inline]
    fn handle_show_desktop(
        &mut self,
        _slf: &Rc<TreelandWindowManagementV1>,
        state: u32,
    ) {
        let res = _slf.send_show_desktop(
            state,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_window_management_v1.show_desktop message: {}", Report::new(e));
        }
    }

    /// destroy the window manager object
    #[inline]
    fn handle_destroy(
        &mut self,
        _slf: &Rc<TreelandWindowManagementV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_window_management_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for TreelandWindowManagementV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandWindowManagementV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_window_management_v1#{}.set_desktop(state: {})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_desktop(&self, arg0);
                } else {
                    DefaultHandler.handle_set_desktop(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_window_management_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_window_management_v1#{}.show_desktop(state: {})\n", msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).handle_show_desktop(&self, arg0);
                } else {
                    DefaultHandler.handle_show_desktop(&self, arg0);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "set_desktop",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "show_desktop",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for TreelandWindowManagementV1 {
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

impl TreelandWindowManagementV1 {
    /// Since when the desktop_state.normal enum variant is available.
    pub const ENM__DESKTOP_STATE_NORMAL__SINCE: u32 = 1;
    /// Since when the desktop_state.show enum variant is available.
    pub const ENM__DESKTOP_STATE_SHOW__SINCE: u32 = 1;
    /// Since when the desktop_state.preview_show enum variant is available.
    pub const ENM__DESKTOP_STATE_PREVIEW_SHOW__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandWindowManagementV1DesktopState(pub u32);

impl TreelandWindowManagementV1DesktopState {
    pub const NORMAL: Self = Self(0);

    pub const SHOW: Self = Self(1);

    pub const PREVIEW_SHOW: Self = Self(2);
}

impl Debug for TreelandWindowManagementV1DesktopState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NORMAL => "NORMAL",
            Self::SHOW => "SHOW",
            Self::PREVIEW_SHOW => "PREVIEW_SHOW",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
