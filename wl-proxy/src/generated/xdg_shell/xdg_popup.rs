//! short-lived, popup surfaces for menus
//!
//! A popup surface is a short-lived, temporary surface. It can be used to
//! implement for example menus, popovers, tooltips and other similar user
//! interface concepts.
//!
//! A popup can be made to take an explicit grab. See xdg_popup.grab for
//! details.
//!
//! When the popup is dismissed, a popup_done event will be sent out, and at
//! the same time the surface will be unmapped. See the xdg_popup.popup_done
//! event for details.
//!
//! Explicitly destroying the xdg_popup object will also dismiss the popup and
//! unmap the surface. Clients that want to dismiss the popup when another
//! surface of their own is clicked should dismiss the popup using the destroy
//! request.
//!
//! A newly created xdg_popup will be stacked on top of all previously created
//! xdg_popup surfaces associated with the same xdg_toplevel.
//!
//! The parent of an xdg_popup must be mapped (see the xdg_surface
//! description) before the xdg_popup itself.
//!
//! The client must call wl_surface.commit on the corresponding wl_surface
//! for the xdg_popup state to take effect.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_popup proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgPopup {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgPopupMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgPopupMessageHandler for DefaultMessageHandler { }

impl MetaXdgPopup {
    pub const XML_VERSION: u32 = 7;
}

impl MetaXdgPopup {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::XdgPopup, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaXdgPopup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgPopup")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgPopup {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// remove xdg_popup interface
    ///
    /// This destroys the popup. Explicitly destroying the xdg_popup
    /// object will also dismiss the popup, and unmap the surface.
    ///
    /// If this xdg_popup is not the "topmost" popup, the
    /// xdg_wm_base.not_the_topmost_popup protocol error will be sent.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
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

    /// Since when the grab message is available.
    #[allow(dead_code)]
    pub const MSG__GRAB__SINCE: u32 = 1;

    /// make the popup take an explicit grab
    ///
    /// This request makes the created popup take an explicit grab. An explicit
    /// grab will be dismissed when the user dismisses the popup, or when the
    /// client destroys the xdg_popup. This can be done by the user clicking
    /// outside the surface, using the keyboard, or even locking the screen
    /// through closing the lid or a timeout.
    ///
    /// If the compositor denies the grab, the popup will be immediately
    /// dismissed.
    ///
    /// This request must be used in response to some sort of user action like a
    /// button press, key press, or touch down event. The serial number of the
    /// event should be passed as 'serial'.
    ///
    /// The parent of a grabbing popup must either be an xdg_toplevel surface or
    /// another xdg_popup with an explicit grab. If the parent is another
    /// xdg_popup it means that the popups are nested, with this popup now being
    /// the topmost popup.
    ///
    /// Nested popups must be destroyed in the reverse order they were created
    /// in, e.g. the only popup you are allowed to destroy at all times is the
    /// topmost one.
    ///
    /// When compositors choose to dismiss a popup, they may dismiss every
    /// nested grabbing popup as well. When a compositor dismisses popups, it
    /// will follow the same dismissing order as required from the client.
    ///
    /// If the topmost grabbing popup is destroyed, the grab will be returned to
    /// the parent of the popup, if that parent previously had an explicit grab.
    ///
    /// If the parent is a grabbing popup which has already been dismissed, this
    /// popup will be immediately dismissed. If the parent is a popup that did
    /// not take an explicit grab, an error will be raised.
    ///
    /// During a popup grab, the client owning the grab will receive pointer
    /// and touch events for all their surfaces as normal (similar to an
    /// "owner-events" grab in X11 parlance), while the top most grabbing popup
    /// will always have keyboard focus.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    #[inline]
    pub fn send_grab(
        &self,
        seat: &Rc<MetaWlSeat>,
        serial: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            seat,
            serial,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
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
            1,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the configure message is available.
    #[allow(dead_code)]
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// configure the popup surface
    ///
    /// This event asks the popup surface to configure itself given the
    /// configuration. The configured state should not be applied immediately.
    /// See xdg_surface.configure for details.
    ///
    /// The x and y arguments represent the position the popup was placed at
    /// given the xdg_positioner rule, relative to the upper left corner of the
    /// window geometry of the parent surface.
    ///
    /// For version 2 or older, the configure event for an xdg_popup is only
    /// ever sent once for the initial configuration. Starting with version 3,
    /// it may be sent again if the popup is setup with an xdg_positioner with
    /// set_reactive requested, or in response to xdg_popup.reposition requests.
    ///
    /// # Arguments
    ///
    /// - `x`: x position relative to parent surface window geometry
    /// - `y`: y position relative to parent surface window geometry
    /// - `width`: window geometry width
    /// - `height`: window geometry height
    #[inline]
    pub fn send_configure(
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// Since when the popup_done message is available.
    #[allow(dead_code)]
    pub const MSG__POPUP_DONE__SINCE: u32 = 1;

    /// popup interaction is done
    ///
    /// The popup_done event is sent out when a popup is dismissed by the
    /// compositor. The client should destroy the xdg_popup object at this
    /// point.
    #[inline]
    pub fn send_popup_done(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
        ]);
        Ok(())
    }

    /// Since when the reposition message is available.
    #[allow(dead_code)]
    pub const MSG__REPOSITION__SINCE: u32 = 3;

    /// recalculate the popup's location
    ///
    /// Reposition an already-mapped popup. The popup will be placed given the
    /// details in the passed xdg_positioner object, and a
    /// xdg_popup.repositioned followed by xdg_popup.configure and
    /// xdg_surface.configure will be emitted in response. Any parameters set
    /// by the previous positioner will be discarded.
    ///
    /// The passed token will be sent in the corresponding
    /// xdg_popup.repositioned event. The new popup position will not take
    /// effect until the corresponding configure event is acknowledged by the
    /// client. See xdg_popup.repositioned for details. The token itself is
    /// opaque, and has no other special meaning.
    ///
    /// If multiple reposition requests are sent, the compositor may skip all
    /// but the last one.
    ///
    /// If the popup is repositioned in response to a configure event for its
    /// parent, the client should send an xdg_positioner.set_parent_configure
    /// and possibly an xdg_positioner.set_parent_size request to allow the
    /// compositor to properly constrain the popup.
    ///
    /// If the popup is repositioned together with a parent that is being
    /// resized, but not in response to a configure event, the client should
    /// send an xdg_positioner.set_parent_size request.
    ///
    /// # Arguments
    ///
    /// - `positioner`:
    /// - `token`: reposition request token
    #[inline]
    pub fn send_reposition(
        &self,
        positioner: &Rc<MetaXdgPositioner>,
        token: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            positioner,
            token,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg0 = match arg0.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
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
            2,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// Since when the repositioned message is available.
    #[allow(dead_code)]
    pub const MSG__REPOSITIONED__SINCE: u32 = 3;

    /// signal the completion of a repositioned request
    ///
    /// The repositioned event is sent as part of a popup configuration
    /// sequence, together with xdg_popup.configure and lastly
    /// xdg_surface.configure to notify the completion of a reposition request.
    ///
    /// The repositioned event is to notify about the completion of a
    /// xdg_popup.reposition request. The token argument is the token passed
    /// in the xdg_popup.reposition request.
    ///
    /// Immediately after this event is emitted, xdg_popup.configure and
    /// xdg_surface.configure will be sent with the updated size and position,
    /// as well as a new configure serial.
    ///
    /// The client should optionally update the content of the popup, but must
    /// acknowledge the new popup configuration for the new position to take
    /// effect. See xdg_surface.ack_configure for details.
    ///
    /// # Arguments
    ///
    /// - `token`: reposition request token
    #[inline]
    pub fn send_repositioned(
        &self,
        token: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            token,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError);
        };
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
            arg0,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgPopup] proxies.
#[allow(dead_code)]
pub trait MetaXdgPopupMessageHandler {
    /// remove xdg_popup interface
    ///
    /// This destroys the popup. Explicitly destroying the xdg_popup
    /// object will also dismiss the popup, and unmap the surface.
    ///
    /// If this xdg_popup is not the "topmost" popup, the
    /// xdg_wm_base.not_the_topmost_popup protocol error will be sent.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgPopup>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_popup.destroy message: {}", Report::new(e));
        }
    }

    /// make the popup take an explicit grab
    ///
    /// This request makes the created popup take an explicit grab. An explicit
    /// grab will be dismissed when the user dismisses the popup, or when the
    /// client destroys the xdg_popup. This can be done by the user clicking
    /// outside the surface, using the keyboard, or even locking the screen
    /// through closing the lid or a timeout.
    ///
    /// If the compositor denies the grab, the popup will be immediately
    /// dismissed.
    ///
    /// This request must be used in response to some sort of user action like a
    /// button press, key press, or touch down event. The serial number of the
    /// event should be passed as 'serial'.
    ///
    /// The parent of a grabbing popup must either be an xdg_toplevel surface or
    /// another xdg_popup with an explicit grab. If the parent is another
    /// xdg_popup it means that the popups are nested, with this popup now being
    /// the topmost popup.
    ///
    /// Nested popups must be destroyed in the reverse order they were created
    /// in, e.g. the only popup you are allowed to destroy at all times is the
    /// topmost one.
    ///
    /// When compositors choose to dismiss a popup, they may dismiss every
    /// nested grabbing popup as well. When a compositor dismisses popups, it
    /// will follow the same dismissing order as required from the client.
    ///
    /// If the topmost grabbing popup is destroyed, the grab will be returned to
    /// the parent of the popup, if that parent previously had an explicit grab.
    ///
    /// If the parent is a grabbing popup which has already been dismissed, this
    /// popup will be immediately dismissed. If the parent is a popup that did
    /// not take an explicit grab, an error will be raised.
    ///
    /// During a popup grab, the client owning the grab will receive pointer
    /// and touch events for all their surfaces as normal (similar to an
    /// "owner-events" grab in X11 parlance), while the top most grabbing popup
    /// will always have keyboard focus.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn grab(
        &mut self,
        _slf: &Rc<MetaXdgPopup>,
        seat: &Rc<MetaWlSeat>,
        serial: u32,
    ) {
        let res = _slf.send_grab(
            seat,
            serial,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_popup.grab message: {}", Report::new(e));
        }
    }

    /// configure the popup surface
    ///
    /// This event asks the popup surface to configure itself given the
    /// configuration. The configured state should not be applied immediately.
    /// See xdg_surface.configure for details.
    ///
    /// The x and y arguments represent the position the popup was placed at
    /// given the xdg_positioner rule, relative to the upper left corner of the
    /// window geometry of the parent surface.
    ///
    /// For version 2 or older, the configure event for an xdg_popup is only
    /// ever sent once for the initial configuration. Starting with version 3,
    /// it may be sent again if the popup is setup with an xdg_positioner with
    /// set_reactive requested, or in response to xdg_popup.reposition requests.
    ///
    /// # Arguments
    ///
    /// - `x`: x position relative to parent surface window geometry
    /// - `y`: y position relative to parent surface window geometry
    /// - `width`: window geometry width
    /// - `height`: window geometry height
    #[inline]
    fn configure(
        &mut self,
        _slf: &Rc<MetaXdgPopup>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let res = _slf.send_configure(
            x,
            y,
            width,
            height,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_popup.configure message: {}", Report::new(e));
        }
    }

    /// popup interaction is done
    ///
    /// The popup_done event is sent out when a popup is dismissed by the
    /// compositor. The client should destroy the xdg_popup object at this
    /// point.
    #[inline]
    fn popup_done(
        &mut self,
        _slf: &Rc<MetaXdgPopup>,
    ) {
        let res = _slf.send_popup_done(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_popup.popup_done message: {}", Report::new(e));
        }
    }

    /// recalculate the popup's location
    ///
    /// Reposition an already-mapped popup. The popup will be placed given the
    /// details in the passed xdg_positioner object, and a
    /// xdg_popup.repositioned followed by xdg_popup.configure and
    /// xdg_surface.configure will be emitted in response. Any parameters set
    /// by the previous positioner will be discarded.
    ///
    /// The passed token will be sent in the corresponding
    /// xdg_popup.repositioned event. The new popup position will not take
    /// effect until the corresponding configure event is acknowledged by the
    /// client. See xdg_popup.repositioned for details. The token itself is
    /// opaque, and has no other special meaning.
    ///
    /// If multiple reposition requests are sent, the compositor may skip all
    /// but the last one.
    ///
    /// If the popup is repositioned in response to a configure event for its
    /// parent, the client should send an xdg_positioner.set_parent_configure
    /// and possibly an xdg_positioner.set_parent_size request to allow the
    /// compositor to properly constrain the popup.
    ///
    /// If the popup is repositioned together with a parent that is being
    /// resized, but not in response to a configure event, the client should
    /// send an xdg_positioner.set_parent_size request.
    ///
    /// # Arguments
    ///
    /// - `positioner`:
    /// - `token`: reposition request token
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn reposition(
        &mut self,
        _slf: &Rc<MetaXdgPopup>,
        positioner: &Rc<MetaXdgPositioner>,
        token: u32,
    ) {
        let res = _slf.send_reposition(
            positioner,
            token,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_popup.reposition message: {}", Report::new(e));
        }
    }

    /// signal the completion of a repositioned request
    ///
    /// The repositioned event is sent as part of a popup configuration
    /// sequence, together with xdg_popup.configure and lastly
    /// xdg_surface.configure to notify the completion of a reposition request.
    ///
    /// The repositioned event is to notify about the completion of a
    /// xdg_popup.reposition request. The token argument is the token passed
    /// in the xdg_popup.reposition request.
    ///
    /// Immediately after this event is emitted, xdg_popup.configure and
    /// xdg_surface.configure will be sent with the updated size and position,
    /// as well as a new configure serial.
    ///
    /// The client should optionally update the content of the popup, but must
    /// acknowledge the new popup configuration for the new position to take
    /// effect. See xdg_surface.ack_configure for details.
    ///
    /// # Arguments
    ///
    /// - `token`: reposition request token
    #[inline]
    fn repositioned(
        &mut self,
        _slf: &Rc<MetaXdgPopup>,
        token: u32,
    ) {
        let res = _slf.send_repositioned(
            token,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_popup.repositioned message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgPopup {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
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
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.endpoint.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).grab(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.grab(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = client.endpoint.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaXdgPositioner>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).reposition(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.reposition(&self, arg0, arg1);
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
                    arg1,
                    arg2,
                    arg3,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = arg3 as i32;
                if let Some(handler) = handler {
                    (**handler).configure(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultMessageHandler.configure(&self, arg0, arg1, arg2, arg3);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).popup_done(&self);
                } else {
                    DefaultMessageHandler.popup_done(&self);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                if let Some(handler) = handler {
                    (**handler).repositioned(&self, arg0);
                } else {
                    DefaultMessageHandler.repositioned(&self, arg0);
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

impl MetaXdgPopup {
    /// Since when the error.invalid_grab enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_GRAB__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaXdgPopupError(pub u32);

impl MetaXdgPopupError {
    /// tried to grab after being mapped
    #[allow(dead_code)]
    pub const INVALID_GRAB: Self = Self(0);
}

impl Debug for MetaXdgPopupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_GRAB => "INVALID_GRAB",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
