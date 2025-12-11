//! manager to control CTMs
//!
//! This object is a manager which offers requests to control CTMs.
//!
//! If any changes are done, once this object is destroyed, CTMs are reset back to
//! an identity matrix.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A hyprland_ctm_control_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct HyprlandCtmControlManagerV1 {
    core: ProxyCore,
    handler: HandlerHolder<dyn HyprlandCtmControlManagerV1Handler>,
}

struct DefaultHandler;

impl HyprlandCtmControlManagerV1Handler for DefaultHandler { }

impl HyprlandCtmControlManagerV1 {
    pub const XML_VERSION: u32 = 2;
    pub const INTERFACE: &str = "hyprland_ctm_control_manager_v1";
}

impl HyprlandCtmControlManagerV1 {
    pub fn set_handler(&self, handler: impl HyprlandCtmControlManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn HyprlandCtmControlManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for HyprlandCtmControlManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyprlandCtmControlManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl HyprlandCtmControlManagerV1 {
    /// Since when the set_ctm_for_output message is available.
    pub const MSG__SET_CTM_FOR_OUTPUT__SINCE: u32 = 1;

    /// set the CTM of an output
    ///
    /// Set a CTM for a wl_output.
    ///
    /// This state is not applied immediately; clients must call .commit to
    /// apply any pending changes.
    ///
    /// The provided values describe a 3x3 Row-Major CTM with values in the range of [0, ∞)
    ///
    /// Passing values outside of the range will raise an invalid_matrix error.
    ///
    /// The default value of the CTM is an identity matrix.
    ///
    /// If an output doesn't get a CTM set with set_ctm_for_output and commit is called,
    /// that output will get its CTM reset to an identity matrix.
    ///
    /// # Arguments
    ///
    /// - `output`:
    /// - `mat0`:
    /// - `mat1`:
    /// - `mat2`:
    /// - `mat3`:
    /// - `mat4`:
    /// - `mat5`:
    /// - `mat6`:
    /// - `mat7`:
    /// - `mat8`:
    #[inline]
    pub fn send_set_ctm_for_output(
        &self,
        output: &Rc<WlOutput>,
        mat0: Fixed,
        mat1: Fixed,
        mat2: Fixed,
        mat3: Fixed,
        mat4: Fixed,
        mat5: Fixed,
        mat6: Fixed,
        mat7: Fixed,
        mat8: Fixed,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
            arg7,
            arg8,
            arg9,
        ) = (
            output,
            mat0,
            mat1,
            mat2,
            mat3,
            mat4,
            mat5,
            mat6,
            mat7,
            mat8,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("output")),
            Some(id) => id,
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_ctm_control_manager_v1#{}.set_ctm_for_output(output: wl_output#{}, mat0: {}, mat1: {}, mat2: {}, mat3: {}, mat4: {}, mat5: {}, mat6: {}, mat7: {}, mat8: {})\n", id, arg0_id, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
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
            arg0_id,
            arg1.to_wire() as u32,
            arg2.to_wire() as u32,
            arg3.to_wire() as u32,
            arg4.to_wire() as u32,
            arg5.to_wire() as u32,
            arg6.to_wire() as u32,
            arg7.to_wire() as u32,
            arg8.to_wire() as u32,
            arg9.to_wire() as u32,
        ]);
        Ok(())
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// commit the pending state
    ///
    /// Commits the pending state(s) set by set_ctm_for_output.
    #[inline]
    pub fn send_commit(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_ctm_control_manager_v1#{}.commit()\n", id);
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
        Ok(())
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    ///
    /// The CTMs of all outputs will be reset to an identity matrix.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= hyprland_ctm_control_manager_v1#{}.destroy()\n", id);
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
            2,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the blocked message is available.
    pub const MSG__BLOCKED__SINCE: u32 = 1;

    /// This event is sent if another manager was bound by any client
    /// at the time the current manager was bound.
    /// Any set_ctm_for_output requests from a blocked manager will be
    /// silently ignored by the compositor.
    ///
    /// The client should destroy the manager after receiving this event.
    #[inline]
    pub fn send_blocked(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= hyprland_ctm_control_manager_v1#{}.blocked()\n", client.endpoint.id, id);
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
        ]);
        Ok(())
    }
}

/// A message handler for [HyprlandCtmControlManagerV1] proxies.
pub trait HyprlandCtmControlManagerV1Handler: Any {
    /// set the CTM of an output
    ///
    /// Set a CTM for a wl_output.
    ///
    /// This state is not applied immediately; clients must call .commit to
    /// apply any pending changes.
    ///
    /// The provided values describe a 3x3 Row-Major CTM with values in the range of [0, ∞)
    ///
    /// Passing values outside of the range will raise an invalid_matrix error.
    ///
    /// The default value of the CTM is an identity matrix.
    ///
    /// If an output doesn't get a CTM set with set_ctm_for_output and commit is called,
    /// that output will get its CTM reset to an identity matrix.
    ///
    /// # Arguments
    ///
    /// - `output`:
    /// - `mat0`:
    /// - `mat1`:
    /// - `mat2`:
    /// - `mat3`:
    /// - `mat4`:
    /// - `mat5`:
    /// - `mat6`:
    /// - `mat7`:
    /// - `mat8`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn set_ctm_for_output(
        &mut self,
        _slf: &Rc<HyprlandCtmControlManagerV1>,
        output: &Rc<WlOutput>,
        mat0: Fixed,
        mat1: Fixed,
        mat2: Fixed,
        mat3: Fixed,
        mat4: Fixed,
        mat5: Fixed,
        mat6: Fixed,
        mat7: Fixed,
        mat8: Fixed,
    ) {
        let res = _slf.send_set_ctm_for_output(
            output,
            mat0,
            mat1,
            mat2,
            mat3,
            mat4,
            mat5,
            mat6,
            mat7,
            mat8,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_ctm_control_manager_v1.set_ctm_for_output message: {}", Report::new(e));
        }
    }

    /// commit the pending state
    ///
    /// Commits the pending state(s) set by set_ctm_for_output.
    #[inline]
    fn commit(
        &mut self,
        _slf: &Rc<HyprlandCtmControlManagerV1>,
    ) {
        let res = _slf.send_commit(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_ctm_control_manager_v1.commit message: {}", Report::new(e));
        }
    }

    /// destroy the manager
    ///
    /// All objects created by the manager will still remain valid, until their
    /// appropriate destroy request has been called.
    ///
    /// The CTMs of all outputs will be reset to an identity matrix.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<HyprlandCtmControlManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_ctm_control_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// This event is sent if another manager was bound by any client
    /// at the time the current manager was bound.
    /// Any set_ctm_for_output requests from a blocked manager will be
    /// silently ignored by the compositor.
    ///
    /// The client should destroy the manager after receiving this event.
    #[inline]
    fn blocked(
        &mut self,
        _slf: &Rc<HyprlandCtmControlManagerV1>,
    ) {
        let res = _slf.send_blocked(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a hyprland_ctm_control_manager_v1.blocked message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for HyprlandCtmControlManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::HyprlandCtmControlManagerV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                    arg7,
                    arg8,
                    arg9,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 48));
                };
                let arg1 = Fixed::from_wire(arg1 as i32);
                let arg2 = Fixed::from_wire(arg2 as i32);
                let arg3 = Fixed::from_wire(arg3 as i32);
                let arg4 = Fixed::from_wire(arg4 as i32);
                let arg5 = Fixed::from_wire(arg5 as i32);
                let arg6 = Fixed::from_wire(arg6 as i32);
                let arg7 = Fixed::from_wire(arg7 as i32);
                let arg8 = Fixed::from_wire(arg8 as i32);
                let arg9 = Fixed::from_wire(arg9 as i32);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_ctm_control_manager_v1#{}.set_ctm_for_output(output: wl_output#{}, mat0: {}, mat1: {}, mat2: {}, mat3: {}, mat4: {}, mat5: {}, mat6: {}, mat7: {}, mat8: {})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                    self.core.state.log(args);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg0_id));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlOutput>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_ctm_for_output(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                } else {
                    DefaultHandler.set_ctm_for_output(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_ctm_control_manager_v1#{}.commit()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).commit(&self);
                } else {
                    DefaultHandler.commit(&self);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> hyprland_ctm_control_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> hyprland_ctm_control_manager_v1#{}.blocked()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).blocked(&self);
                } else {
                    DefaultHandler.blocked(&self);
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
            0 => "set_ctm_for_output",
            1 => "commit",
            2 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "blocked",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for HyprlandCtmControlManagerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl HyprlandCtmControlManagerV1 {
    /// Since when the error.invalid_matrix enum variant is available.
    pub const ENM__ERROR_INVALID_MATRIX__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HyprlandCtmControlManagerV1Error(pub u32);

impl HyprlandCtmControlManagerV1Error {
    /// the matrix values are invalid.
    pub const INVALID_MATRIX: Self = Self(0);
}

impl Debug for HyprlandCtmControlManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_MATRIX => "INVALID_MATRIX",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
