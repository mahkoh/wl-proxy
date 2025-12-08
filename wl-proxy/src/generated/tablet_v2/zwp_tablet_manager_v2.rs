//! controller object for graphic tablet devices
//!
//! An object that provides access to the graphics tablets available on this
//! system. All tablets are associated with a seat, to get access to the
//! actual tablets, use zwp_tablet_manager_v2.get_tablet_seat.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwp_tablet_manager_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwpTabletManagerV2 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwpTabletManagerV2MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwpTabletManagerV2MessageHandler for DefaultMessageHandler { }

impl MetaZwpTabletManagerV2 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwpTabletManagerV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwpTabletManagerV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwpTabletManagerV2 {
    /// Since when the get_tablet_seat message is available.
    #[allow(dead_code)]
    pub const MSG__GET_TABLET_SEAT__SINCE: u32 = 1;

    /// get the tablet seat
    ///
    /// Get the zwp_tablet_seat_v2 object for the given seat. This object
    /// provides access to all graphics tablets in this seat.
    ///
    /// # Arguments
    ///
    /// - `tablet_seat`:
    /// - `seat`: The wl_seat object to retrieve the tablets for
    #[inline]
    pub fn send_get_tablet_seat(
        &self,
        tablet_seat: &Rc<MetaZwpTabletSeatV2>,
        seat: &Rc<MetaWlSeat>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            tablet_seat,
            seat,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
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

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// release the memory for the tablet manager object
    ///
    /// Destroy the zwp_tablet_manager_v2 object. Objects created from this
    /// object are unaffected and should be destroyed separately.
    #[inline]
    pub fn send_destroy(
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
}

/// A message handler for [ZwpTabletManagerV2] proxies.
#[allow(dead_code)]
pub trait MetaZwpTabletManagerV2MessageHandler {
    /// get the tablet seat
    ///
    /// Get the zwp_tablet_seat_v2 object for the given seat. This object
    /// provides access to all graphics tablets in this seat.
    ///
    /// # Arguments
    ///
    /// - `tablet_seat`:
    /// - `seat`: The wl_seat object to retrieve the tablets for
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_tablet_seat(
        &mut self,
        _slf: &Rc<MetaZwpTabletManagerV2>,
        tablet_seat: &Rc<MetaZwpTabletSeatV2>,
        seat: &Rc<MetaWlSeat>,
    ) {
        let res = _slf.send_get_tablet_seat(
            tablet_seat,
            seat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_manager_v2.get_tablet_seat message: {}", Report::new(e));
        }
    }

    /// release the memory for the tablet manager object
    ///
    /// Destroy the zwp_tablet_manager_v2 object. Objects created from this
    /// object are unaffected and should be destroyed separately.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwpTabletManagerV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwp_tablet_manager_v2.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwpTabletManagerV2 {
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
                let arg0 = MetaZwpTabletSeatV2::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSeat>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).get_tablet_seat(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.get_tablet_seat(&self, arg0, arg1);
                }
            }
            1 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
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
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }
}

