//! output device configuration manager
//!
//! This interface is a manager that allows reading and writing the current
//! output device configuration.
//!
//! Output devices that display pixels (e.g. a physical monitor or a virtual
//! output in a window) are represented as heads. Heads cannot be created nor
//! destroyed by the client, but they can be enabled or disabled and their
//! properties can be changed. Each head may have one or more available modes.
//!
//! Whenever a head appears (e.g. a monitor is plugged in), it will be
//! advertised via the head event. Immediately after the output manager is
//! bound, all current heads are advertised.
//!
//! Whenever a head's properties change, the relevant wlr_output_head events
//! will be sent. Not all head properties will be sent: only properties that
//! have changed need to.
//!
//! Whenever a head disappears (e.g. a monitor is unplugged), a
//! wlr_output_head.finished event will be sent.
//!
//! After one or more heads appear, change or disappear, the done event will
//! be sent. It carries a serial which can be used in a create_configuration
//! request to update heads properties.
//!
//! The information obtained from this protocol should only be used for output
//! configuration purposes. This protocol is not designed to be a generic
//! output property advertisement protocol for regular clients. Instead,
//! protocols such as xdg-output should be used.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_output_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrOutputManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrOutputManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrOutputManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrOutputManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrOutputManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrOutputManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrOutputManagerV1 {
    /// Since when the head message is available.
    #[allow(dead_code)]
    pub const MSG__HEAD__SINCE: u32 = 1;

    /// introduce a new head
    ///
    /// This event introduces a new head. This happens whenever a new head
    /// appears (e.g. a monitor is plugged in) or after the output manager is
    /// bound.
    #[inline]
    pub fn send_head(
        &self,
        head: &Rc<MetaZwlrOutputHeadV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            head,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        arg0.generate_client_id(client, arg0_obj.clone())?;
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the done message is available.
    #[allow(dead_code)]
    pub const MSG__DONE__SINCE: u32 = 1;

    /// sent all information about current configuration
    ///
    /// This event is sent after all information has been sent after binding to
    /// the output manager object and after any subsequent changes. This applies
    /// to child head and mode objects as well. In other words, this event is
    /// sent whenever a head or mode is created or destroyed and whenever one of
    /// their properties has been changed. Not all state is re-sent each time
    /// the current configuration changes: only the actual changes are sent.
    ///
    /// This allows changes to the output configuration to be seen as atomic,
    /// even if they happen via multiple events.
    ///
    /// A serial is sent to be used in a future create_configuration request.
    ///
    /// # Arguments
    ///
    /// - `serial`: current configuration serial
    #[inline]
    pub fn send_done(
        &self,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            serial,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
            arg0,
        ]);
        Ok(())
    }

    /// Since when the create_configuration message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_CONFIGURATION__SINCE: u32 = 1;

    /// create a new output configuration object
    ///
    /// Create a new output configuration object. This allows to update head
    /// properties.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `serial`:
    #[inline]
    pub fn send_create_configuration(
        &self,
        id: &Rc<MetaZwlrOutputConfigurationV1>,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            id,
            serial,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        arg0.generate_server_id(arg0_obj.clone())?;
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
        ]);
        Ok(())
    }

    /// Since when the stop message is available.
    #[allow(dead_code)]
    pub const MSG__STOP__SINCE: u32 = 1;

    /// stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for output
    /// configuration changes. However the compositor may emit further events,
    /// until the finished event is emitted.
    ///
    /// The client must not send any more requests after this one.
    #[inline]
    pub fn send_stop(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        Ok(())
    }

    /// Since when the finished message is available.
    #[allow(dead_code)]
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the compositor has finished with the manager
    ///
    /// This event indicates that the compositor is done sending manager events.
    /// The compositor will destroy the object immediately after sending this
    /// event, so it will become invalid and the client should release any
    /// resources associated with it.
    #[inline]
    pub fn send_finished(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwlrOutputManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrOutputManagerV1MessageHandler {
    /// introduce a new head
    ///
    /// This event introduces a new head. This happens whenever a new head
    /// appears (e.g. a monitor is plugged in) or after the output manager is
    /// bound.
    ///
    /// # Arguments
    ///
    /// - `head`:
    #[inline]
    fn head(
        &mut self,
        _slf: &Rc<MetaZwlrOutputManagerV1>,
        head: &Rc<MetaZwlrOutputHeadV1>,
    ) {
        let res = _slf.send_head(
            head,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_manager_v1.head message: {}", Report::new(e));
        }
    }

    /// sent all information about current configuration
    ///
    /// This event is sent after all information has been sent after binding to
    /// the output manager object and after any subsequent changes. This applies
    /// to child head and mode objects as well. In other words, this event is
    /// sent whenever a head or mode is created or destroyed and whenever one of
    /// their properties has been changed. Not all state is re-sent each time
    /// the current configuration changes: only the actual changes are sent.
    ///
    /// This allows changes to the output configuration to be seen as atomic,
    /// even if they happen via multiple events.
    ///
    /// A serial is sent to be used in a future create_configuration request.
    ///
    /// # Arguments
    ///
    /// - `serial`: current configuration serial
    #[inline]
    fn done(
        &mut self,
        _slf: &Rc<MetaZwlrOutputManagerV1>,
        serial: u32,
    ) {
        let res = _slf.send_done(
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_manager_v1.done message: {}", Report::new(e));
        }
    }

    /// create a new output configuration object
    ///
    /// Create a new output configuration object. This allows to update head
    /// properties.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `serial`:
    #[inline]
    fn create_configuration(
        &mut self,
        _slf: &Rc<MetaZwlrOutputManagerV1>,
        id: &Rc<MetaZwlrOutputConfigurationV1>,
        serial: u32,
    ) {
        let res = _slf.send_create_configuration(
            id,
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_manager_v1.create_configuration message: {}", Report::new(e));
        }
    }

    /// stop sending events
    ///
    /// Indicates the client no longer wishes to receive events for output
    /// configuration changes. However the compositor may emit further events,
    /// until the finished event is emitted.
    ///
    /// The client must not send any more requests after this one.
    #[inline]
    fn stop(
        &mut self,
        _slf: &Rc<MetaZwlrOutputManagerV1>,
    ) {
        let res = _slf.send_stop(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_manager_v1.stop message: {}", Report::new(e));
        }
    }

    /// the compositor has finished with the manager
    ///
    /// This event indicates that the compositor is done sending manager events.
    /// The compositor will destroy the object immediately after sending this
    /// event, so it will become invalid and the client should release any
    /// resources associated with it.
    #[inline]
    fn finished(
        &mut self,
        _slf: &Rc<MetaZwlrOutputManagerV1>,
    ) {
        let res = _slf.send_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_output_manager_v1.finished message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrOutputManagerV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaZwlrOutputConfigurationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create_configuration(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.create_configuration(&self, arg0, arg1);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).stop(&self);
                } else {
                    DefaultMessageHandler.stop(&self);
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
                let arg0_id = arg0;
                let arg0 = MetaZwlrOutputHeadV1::new(&self.core.state, self.core.version);
                arg0.core().set_server_id(arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).head(&self, arg0);
                } else {
                    DefaultMessageHandler.head(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).done(&self, arg0);
                } else {
                    DefaultMessageHandler.done(&self, arg0);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).finished(&self);
                } else {
                    DefaultMessageHandler.finished(&self);
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

