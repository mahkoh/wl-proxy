//! interface for activating surfaces
//!
//! A global interface used for informing the compositor about applications
//! being activated or started, or for applications to request to be
//! activated.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A xdg_activation_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaXdgActivationV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaXdgActivationV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaXdgActivationV1MessageHandler for DefaultMessageHandler { }

impl MetaXdgActivationV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaXdgActivationV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaXdgActivationV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaXdgActivationV1 {
    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_activation object
    ///
    /// Notify the compositor that the xdg_activation object will no longer be
    /// used.
    ///
    /// The child objects created via this interface are unaffected and should
    /// be destroyed separately.
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
            0,
        ]);
        Ok(())
    }

    /// Since when the get_activation_token message is available.
    #[allow(dead_code)]
    pub const MSG__GET_ACTIVATION_TOKEN__SINCE: u32 = 1;

    /// requests a token
    ///
    /// Creates an xdg_activation_token_v1 object that will provide
    /// the initiating client with a unique token for this activation. This
    /// token should be offered to the clients to be activated.
    #[inline]
    pub fn send_get_activation_token(
        &self,
        id: &Rc<MetaXdgActivationTokenV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            id,
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
            1,
            arg0.server_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the activate message is available.
    #[allow(dead_code)]
    pub const MSG__ACTIVATE__SINCE: u32 = 1;

    /// notify new interaction being available
    ///
    /// Requests surface activation. It's up to the compositor to display
    /// this information as desired, for example by placing the surface above
    /// the rest.
    ///
    /// The compositor may know who requested this by checking the activation
    /// token and might decide not to follow through with the activation if it's
    /// considered unwanted.
    ///
    /// Compositors can ignore unknown activation tokens when an invalid
    /// token is passed.
    ///
    /// # Arguments
    ///
    /// - `token`: the activation token of the initiating client
    /// - `surface`: the wl_surface to activate
    #[inline]
    pub fn send_activate(
        &self,
        token: &str,
        surface: &Rc<MetaWlSurface>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            token,
            surface,
        );
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let outgoing = &mut *self.core.state.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1,
        ]);
        Ok(())
    }
}

/// A message handler for [XdgActivationV1] proxies.
#[allow(dead_code)]
pub trait MetaXdgActivationV1MessageHandler {
    /// destroy the xdg_activation object
    ///
    /// Notify the compositor that the xdg_activation object will no longer be
    /// used.
    ///
    /// The child objects created via this interface are unaffected and should
    /// be destroyed separately.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaXdgActivationV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_activation_v1.destroy message: {}", Report::new(e));
        }
    }

    /// requests a token
    ///
    /// Creates an xdg_activation_token_v1 object that will provide
    /// the initiating client with a unique token for this activation. This
    /// token should be offered to the clients to be activated.
    ///
    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn get_activation_token(
        &mut self,
        _slf: &Rc<MetaXdgActivationV1>,
        id: &Rc<MetaXdgActivationTokenV1>,
    ) {
        let res = _slf.send_get_activation_token(
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_activation_v1.get_activation_token message: {}", Report::new(e));
        }
    }

    /// notify new interaction being available
    ///
    /// Requests surface activation. It's up to the compositor to display
    /// this information as desired, for example by placing the surface above
    /// the rest.
    ///
    /// The compositor may know who requested this by checking the activation
    /// token and might decide not to follow through with the activation if it's
    /// considered unwanted.
    ///
    /// Compositors can ignore unknown activation tokens when an invalid
    /// token is passed.
    ///
    /// # Arguments
    ///
    /// - `token`: the activation token of the initiating client
    /// - `surface`: the wl_surface to activate
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn activate(
        &mut self,
        _slf: &Rc<MetaXdgActivationV1>,
        token: &str,
        surface: &Rc<MetaWlSurface>,
    ) {
        let res = _slf.send_activate(
            token,
            surface,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a xdg_activation_v1.activate message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaXdgActivationV1 {
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
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaXdgActivationTokenV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).get_activation_token(&self, arg0);
                } else {
                    DefaultMessageHandler.get_activation_token(&self, arg0);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError);
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError);
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError);
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError);
                        };
                        s
                    }
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError);
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError);
                }
                let Some(arg1) = client.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    return Err(ObjectError);
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).activate(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.activate(&self, arg0, arg1);
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

