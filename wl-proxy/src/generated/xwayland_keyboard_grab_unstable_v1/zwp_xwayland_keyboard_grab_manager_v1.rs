//! context object for keyboard grab manager
//!
//! A global interface used for grabbing the keyboard.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_xwayland_keyboard_grab_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpXwaylandKeyboardGrabManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpXwaylandKeyboardGrabManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpXwaylandKeyboardGrabManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaZwpXwaylandKeyboardGrabManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaZwpXwaylandKeyboardGrabManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwpXwaylandKeyboardGrabManagerV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaZwpXwaylandKeyboardGrabManagerV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaZwpXwaylandKeyboardGrabManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpXwaylandKeyboardGrabManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpXwaylandKeyboardGrabManagerV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the keyboard grab manager
    ///
    /// Destroy the keyboard grab manager.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
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

    /// Since when the grab_keyboard message is available.
    #[allow(dead_code)]
    pub const MSG__GRAB_KEYBOARD__SINCE: u32 = 1;

    /// grab the keyboard to a surface
    ///
    /// The grab_keyboard request asks for a grab of the keyboard, forcing
    /// the keyboard focus for the given seat upon the given surface.
    ///
    /// The protocol provides no guarantee that the grab is ever satisfied,
    /// and does not require the compositor to send an error if the grab
    /// cannot ever be satisfied. It is thus possible to request a keyboard
    /// grab that will never be effective.
    ///
    /// The protocol:
    ///
    /// * does not guarantee that the grab itself is applied for a surface,
    ///   the grab request may be silently ignored by the compositor,
    /// * does not guarantee that any events are sent to this client even
    ///   if the grab is applied to a surface,
    /// * does not guarantee that events sent to this client are exhaustive,
    ///   a compositor may filter some events for its own consumption,
    /// * does not guarantee that events sent to this client are continuous,
    ///   a compositor may change and reroute keyboard events while the grab
    ///   is nominally active.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: surface to report keyboard events to
    /// - `seat`: the seat for which the keyboard should be grabbed
    #[inline]
    pub fn send_grab_keyboard(
        &self,
        id: &Rc<MetaZwpXwaylandKeyboardGrabV1>,
        surface: &Rc<MetaWlSurface>,
        seat: &Rc<MetaWlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            id,
            surface,
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        let arg2_id = match arg2.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("seat")),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
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
            arg0_id,
            arg1_id,
            arg2_id,
        ]);
        Ok(())
    }
}

/// A message handler for [ZwpXwaylandKeyboardGrabManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaZwpXwaylandKeyboardGrabManagerV1MessageHandler {
    /// destroy the keyboard grab manager
    ///
    /// Destroy the keyboard grab manager.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpXwaylandKeyboardGrabManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_xwayland_keyboard_grab_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// grab the keyboard to a surface
    ///
    /// The grab_keyboard request asks for a grab of the keyboard, forcing
    /// the keyboard focus for the given seat upon the given surface.
    ///
    /// The protocol provides no guarantee that the grab is ever satisfied,
    /// and does not require the compositor to send an error if the grab
    /// cannot ever be satisfied. It is thus possible to request a keyboard
    /// grab that will never be effective.
    ///
    /// The protocol:
    ///
    /// * does not guarantee that the grab itself is applied for a surface,
    ///   the grab request may be silently ignored by the compositor,
    /// * does not guarantee that any events are sent to this client even
    ///   if the grab is applied to a surface,
    /// * does not guarantee that events sent to this client are exhaustive,
    ///   a compositor may filter some events for its own consumption,
    /// * does not guarantee that events sent to this client are continuous,
    ///   a compositor may change and reroute keyboard events while the grab
    ///   is nominally active.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`: surface to report keyboard events to
    /// - `seat`: the seat for which the keyboard should be grabbed
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn grab_keyboard(
        &mut self,
        _slf: &Rc<MetaZwpXwaylandKeyboardGrabManagerV1>,
        id: &Rc<MetaZwpXwaylandKeyboardGrabV1>,
        surface: &Rc<MetaWlSurface>,
        seat: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_grab_keyboard(
            id,
            surface,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_xwayland_keyboard_grab_manager_v1.grab_keyboard message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpXwaylandKeyboardGrabManagerV1 {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Self::new(state, version)
    }

    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
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
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 20));
                };
                let arg0_id = arg0;
                let arg0 = MetaZwpXwaylandKeyboardGrabV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg2_id = arg2;
                let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    let o = client.endpoint.lookup(arg2_id).unwrap();
                    return Err(ObjectError::WrongObjectType("seat", o.core().interface, ProxyInterface::WlSeat));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).grab_keyboard(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.grab_keyboard(&self, arg0, arg1, arg2);
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
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "grab_keyboard",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

