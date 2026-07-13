//! corner radius layer
//!
//! The corner-radius object provides a way to specify a corner-radius
//! for it's associated layer surface.
//!
//! If the zwlr_layer_surface_v1 associated with the cosmic_corner_radius_layer_v1
//! object has been destroyed, this object becomes inert. Any further requests
//! other than destroy will raise the layer_destroyed protocol error.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A cosmic_corner_radius_layer_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct CosmicCornerRadiusLayerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn CosmicCornerRadiusLayerV1Handler>,
}

struct DefaultHandler;

impl CosmicCornerRadiusLayerV1Handler for DefaultHandler { }

impl ConcreteObject for CosmicCornerRadiusLayerV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::CosmicCornerRadiusLayerV1;
    const INTERFACE_NAME: &str = "cosmic_corner_radius_layer_v1";
}

impl CosmicCornerRadiusLayerV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl CosmicCornerRadiusLayerV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn CosmicCornerRadiusLayerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for CosmicCornerRadiusLayerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CosmicCornerRadiusLayerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl CosmicCornerRadiusLayerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// Destroy the corner-radius object
    ///
    /// Informs the server that the client will no longer be using this protocol
    /// object. The corner radius will be unset on the next commit.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_corner_radius_layer_v1#{}.destroy()\n", id);
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

    /// Destroy the corner-radius object
    ///
    /// Informs the server that the client will no longer be using this protocol
    /// object. The corner radius will be unset on the next commit.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("cosmic_corner_radius_layer_v1.destroy", &e);
        }
    }

    /// Since when the set_radius message is available.
    pub const MSG__SET_RADIUS__SINCE: u32 = 1;

    /// Set corner radius
    ///
    /// This request sets the hinted corner radius values for rectangular layers.
    ///
    /// The corner radius hint is double-buffered state and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The value is given in logical space relative to the remaining size of the
    /// associated zwlr_layer_surface_v1 after any said padding was applied.
    /// If any value exceeds half of the respective dimension of the remaining size
    /// the radius_too_large protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `top_left`: top-left corner radius
    /// - `top_right`: top-right corner radius
    /// - `bottom_right`: bottom-right corner radius
    /// - `bottom_left`: bottom-left corner radius
    #[inline]
    pub fn try_send_set_radius(
        &self,
        top_left: u32,
        top_right: u32,
        bottom_right: u32,
        bottom_left: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_corner_radius_layer_v1#{}.set_radius(top_left: {}, top_right: {}, bottom_right: {}, bottom_left: {})\n", id, arg0, arg1, arg2, arg3);
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
            arg0,
            arg1,
            arg2,
            arg3,
        ]);
        Ok(())
    }

    /// Set corner radius
    ///
    /// This request sets the hinted corner radius values for rectangular layers.
    ///
    /// The corner radius hint is double-buffered state and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The value is given in logical space relative to the remaining size of the
    /// associated zwlr_layer_surface_v1 after any said padding was applied.
    /// If any value exceeds half of the respective dimension of the remaining size
    /// the radius_too_large protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `top_left`: top-left corner radius
    /// - `top_right`: top-right corner radius
    /// - `bottom_right`: bottom-right corner radius
    /// - `bottom_left`: bottom-left corner radius
    #[inline]
    pub fn send_set_radius(
        &self,
        top_left: u32,
        top_right: u32,
        bottom_right: u32,
        bottom_left: u32,
    ) {
        let res = self.try_send_set_radius(
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        );
        if let Err(e) = res {
            log_send("cosmic_corner_radius_layer_v1.set_radius", &e);
        }
    }

    /// Since when the unset_radius message is available.
    pub const MSG__UNSET_RADIUS__SINCE: u32 = 1;

    /// Unset corner radius
    ///
    /// Unsets any previously hinted corner radius values without invalidating the object for later use.
    /// Can be used by clients that possibly have temporary irregular shapes.
    #[inline]
    pub fn try_send_unset_radius(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_corner_radius_layer_v1#{}.unset_radius()\n", id);
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
        Ok(())
    }

    /// Unset corner radius
    ///
    /// Unsets any previously hinted corner radius values without invalidating the object for later use.
    /// Can be used by clients that possibly have temporary irregular shapes.
    #[inline]
    pub fn send_unset_radius(
        &self,
    ) {
        let res = self.try_send_unset_radius(
        );
        if let Err(e) = res {
            log_send("cosmic_corner_radius_layer_v1.unset_radius", &e);
        }
    }

    /// Since when the set_padding message is available.
    pub const MSG__SET_PADDING__SINCE: u32 = 1;

    /// Set edge padding
    ///
    /// This request sets the hinted padding values for rectangular layers.
    ///
    /// The padding hint is double-buffered state and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The value is given in logical space as the amount of pixels to subtract from
    /// the size of the associated zwlr_layer_surface and in the case of the top and left padding
    /// added to the layer position before applying the corner radius. Negative padding values are allowed.
    ///
    /// If any value exceeds half of the respective dimension of the surface size the padding_too_large
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `top`: top padding
    /// - `right`: right padding
    /// - `bottom`: bottom left padding
    /// - `left`: left padding
    #[inline]
    pub fn try_send_set_padding(
        &self,
        top: i32,
        right: i32,
        bottom: i32,
        left: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            top,
            right,
            bottom,
            left,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_corner_radius_layer_v1#{}.set_padding(top: {}, right: {}, bottom: {}, left: {})\n", id, arg0, arg1, arg2, arg3);
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
            3,
            arg0 as u32,
            arg1 as u32,
            arg2 as u32,
            arg3 as u32,
        ]);
        Ok(())
    }

    /// Set edge padding
    ///
    /// This request sets the hinted padding values for rectangular layers.
    ///
    /// The padding hint is double-buffered state and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The value is given in logical space as the amount of pixels to subtract from
    /// the size of the associated zwlr_layer_surface and in the case of the top and left padding
    /// added to the layer position before applying the corner radius. Negative padding values are allowed.
    ///
    /// If any value exceeds half of the respective dimension of the surface size the padding_too_large
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `top`: top padding
    /// - `right`: right padding
    /// - `bottom`: bottom left padding
    /// - `left`: left padding
    #[inline]
    pub fn send_set_padding(
        &self,
        top: i32,
        right: i32,
        bottom: i32,
        left: i32,
    ) {
        let res = self.try_send_set_padding(
            top,
            right,
            bottom,
            left,
        );
        if let Err(e) = res {
            log_send("cosmic_corner_radius_layer_v1.set_padding", &e);
        }
    }

    /// Since when the unset_padding message is available.
    pub const MSG__UNSET_PADDING__SINCE: u32 = 1;

    /// Unset edge padding
    ///
    /// Unsets any previously hinted padding values without invalidating the object for later use.
    /// Can be used by clients that possibly have temporary irregular shapes.
    #[inline]
    pub fn try_send_unset_padding(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= cosmic_corner_radius_layer_v1#{}.unset_padding()\n", id);
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
            4,
        ]);
        Ok(())
    }

    /// Unset edge padding
    ///
    /// Unsets any previously hinted padding values without invalidating the object for later use.
    /// Can be used by clients that possibly have temporary irregular shapes.
    #[inline]
    pub fn send_unset_padding(
        &self,
    ) {
        let res = self.try_send_unset_padding(
        );
        if let Err(e) = res {
            log_send("cosmic_corner_radius_layer_v1.unset_padding", &e);
        }
    }
}

/// A message handler for [`CosmicCornerRadiusLayerV1`] proxies.
pub trait CosmicCornerRadiusLayerV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<CosmicCornerRadiusLayerV1>) {
        slf.core.delete_id();
    }

    /// Destroy the corner-radius object
    ///
    /// Informs the server that the client will no longer be using this protocol
    /// object. The corner radius will be unset on the next commit.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<CosmicCornerRadiusLayerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("cosmic_corner_radius_layer_v1.destroy", &e);
        }
    }

    /// Set corner radius
    ///
    /// This request sets the hinted corner radius values for rectangular layers.
    ///
    /// The corner radius hint is double-buffered state and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The value is given in logical space relative to the remaining size of the
    /// associated zwlr_layer_surface_v1 after any said padding was applied.
    /// If any value exceeds half of the respective dimension of the remaining size
    /// the radius_too_large protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `top_left`: top-left corner radius
    /// - `top_right`: top-right corner radius
    /// - `bottom_right`: bottom-right corner radius
    /// - `bottom_left`: bottom-left corner radius
    #[inline]
    fn handle_set_radius(
        &mut self,
        slf: &Rc<CosmicCornerRadiusLayerV1>,
        top_left: u32,
        top_right: u32,
        bottom_right: u32,
        bottom_left: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_radius(
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        );
        if let Err(e) = res {
            log_forward("cosmic_corner_radius_layer_v1.set_radius", &e);
        }
    }

    /// Unset corner radius
    ///
    /// Unsets any previously hinted corner radius values without invalidating the object for later use.
    /// Can be used by clients that possibly have temporary irregular shapes.
    #[inline]
    fn handle_unset_radius(
        &mut self,
        slf: &Rc<CosmicCornerRadiusLayerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unset_radius(
        );
        if let Err(e) = res {
            log_forward("cosmic_corner_radius_layer_v1.unset_radius", &e);
        }
    }

    /// Set edge padding
    ///
    /// This request sets the hinted padding values for rectangular layers.
    ///
    /// The padding hint is double-buffered state and will be applied on
    /// the next wl_surface.commit.
    ///
    /// The value is given in logical space as the amount of pixels to subtract from
    /// the size of the associated zwlr_layer_surface and in the case of the top and left padding
    /// added to the layer position before applying the corner radius. Negative padding values are allowed.
    ///
    /// If any value exceeds half of the respective dimension of the surface size the padding_too_large
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `top`: top padding
    /// - `right`: right padding
    /// - `bottom`: bottom left padding
    /// - `left`: left padding
    #[inline]
    fn handle_set_padding(
        &mut self,
        slf: &Rc<CosmicCornerRadiusLayerV1>,
        top: i32,
        right: i32,
        bottom: i32,
        left: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_padding(
            top,
            right,
            bottom,
            left,
        );
        if let Err(e) = res {
            log_forward("cosmic_corner_radius_layer_v1.set_padding", &e);
        }
    }

    /// Unset edge padding
    ///
    /// Unsets any previously hinted padding values without invalidating the object for later use.
    /// Can be used by clients that possibly have temporary irregular shapes.
    #[inline]
    fn handle_unset_padding(
        &mut self,
        slf: &Rc<CosmicCornerRadiusLayerV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_unset_padding(
        );
        if let Err(e) = res {
            log_forward("cosmic_corner_radius_layer_v1.unset_padding", &e);
        }
    }
}

impl ObjectPrivate for CosmicCornerRadiusLayerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::CosmicCornerRadiusLayerV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_corner_radius_layer_v1#{}.destroy()\n", client_id, id);
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
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_corner_radius_layer_v1#{}.set_radius(top_left: {}, top_right: {}, bottom_right: {}, bottom_left: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_radius(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_radius(&self, arg0, arg1, arg2, arg3);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_corner_radius_layer_v1#{}.unset_radius()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unset_radius(&self);
                } else {
                    DefaultHandler.handle_unset_radius(&self);
                }
            }
            3 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_corner_radius_layer_v1#{}.set_padding(top: {}, right: {}, bottom: {}, left: {})\n", client_id, id, arg0, arg1, arg2, arg3);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_padding(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.handle_set_padding(&self, arg0, arg1, arg2, arg3);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> cosmic_corner_radius_layer_v1#{}.unset_padding()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_unset_padding(&self);
                } else {
                    DefaultHandler.handle_unset_padding(&self);
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
            1 => "set_radius",
            2 => "unset_radius",
            3 => "set_padding",
            4 => "unset_padding",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for CosmicCornerRadiusLayerV1 {
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

impl CosmicCornerRadiusLayerV1 {
    /// Since when the error.layer_destroyed enum variant is available.
    pub const ENM__ERROR_LAYER_DESTROYED__SINCE: u32 = 1;
    /// Since when the error.radius_too_large enum variant is available.
    pub const ENM__ERROR_RADIUS_TOO_LARGE__SINCE: u32 = 1;
    /// Since when the error.padding_too_large enum variant is available.
    pub const ENM__ERROR_PADDING_TOO_LARGE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CosmicCornerRadiusLayerV1Error(pub u32);

impl CosmicCornerRadiusLayerV1Error {
    /// the associated layer surface object has been already destroyed
    pub const LAYER_DESTROYED: Self = Self(0);

    /// the associated layer's size isn't large enough for the requested radius
    pub const RADIUS_TOO_LARGE: Self = Self(1);

    /// the associated layer's size isn't large enough for the requested padding
    pub const PADDING_TOO_LARGE: Self = Self(2);
}

impl Debug for CosmicCornerRadiusLayerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::LAYER_DESTROYED => "LAYER_DESTROYED",
            Self::RADIUS_TOO_LARGE => "RADIUS_TOO_LARGE",
            Self::PADDING_TOO_LARGE => "PADDING_TOO_LARGE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
