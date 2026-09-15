//! a concrete icon for a subject
//!
//! This interface represents a concrete icon for a subject. It is compositor
//! policy when jay_icon_surface_v1 objects are created and how they are
//! displayed.
//!
//! Unlike with some wl_surface roles, wl_surface.attach requests have no
//! specified meaning beyond what is specified by wl_surface itself. The
//! client should destroy this object if it no longer wants to provide an
//! icon.
//!
//! The underlying wl_surface is output-only. That is, it will never receive
//! any input events.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A jay_icon_surface_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct JayIconSurfaceV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn JayIconSurfaceV1Handler>,
}

struct DefaultHandler;

impl JayIconSurfaceV1Handler for DefaultHandler { }

impl ConcreteObject for JayIconSurfaceV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::JayIconSurfaceV1;
    const INTERFACE_NAME: &str = "jay_icon_surface_v1";
}

impl JayIconSurfaceV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl JayIconSurfaceV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn JayIconSurfaceV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for JayIconSurfaceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JayIconSurfaceV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl JayIconSurfaceV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy this object
    ///
    /// This request destroys the jay_icon_surface_v1. It will no longer be
    /// displayed. The client should also destroy the underlying wl_surface.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_icon_surface_v1#{}.destroy()\n", id);
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

    /// destroy this object
    ///
    /// This request destroys the jay_icon_surface_v1. It will no longer be
    /// displayed. The client should also destroy the underlying wl_surface.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_v1.destroy", &e);
        }
    }

    /// Since when the finished message is available.
    pub const MSG__FINISHED__SINCE: u32 = 1;

    /// the icon is no longer used
    ///
    /// The compositor sends this event when it no longer wants to use the
    /// jay_icon_surface_v1. The client should destroy the object.
    ///
    /// For example, the compositor might send this event when a task bar is
    /// hidden.
    ///
    /// The compositor must send this event when the underlying subject is
    /// destroyed.
    ///
    /// This event has no effect on other jay_icon_surface_v1 objects created
    /// from the same factory, which might still be used.
    #[inline]
    pub fn try_send_finished(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= jay_icon_surface_v1#{}.finished()\n", client_id, id);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id);
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

    /// the icon is no longer used
    ///
    /// The compositor sends this event when it no longer wants to use the
    /// jay_icon_surface_v1. The client should destroy the object.
    ///
    /// For example, the compositor might send this event when a task bar is
    /// hidden.
    ///
    /// The compositor must send this event when the underlying subject is
    /// destroyed.
    ///
    /// This event has no effect on other jay_icon_surface_v1 objects created
    /// from the same factory, which might still be used.
    #[inline]
    pub fn send_finished(
        &self,
    ) {
        let res = self.try_send_finished(
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_v1.finished", &e);
        }
    }

    /// Since when the configure message is available.
    pub const MSG__CONFIGURE__SINCE: u32 = 1;

    /// terminates a configuration sequence
    ///
    /// A configuration sequence consists of a number of preceding events
    /// terminated by this event. The client should store the contents of the
    /// preceding events and apply them atomically in response to this event.
    ///
    /// Each event that is part of the configuration sequence specifies this,
    /// along with its other semantics.
    ///
    /// Upon receiving this event, the client should send a
    /// jay_icon_surface_v1.ack_configure request with the same serial, prepare
    /// the wl_surface content update to match the requested configuration, and
    /// commit the wl_surface as soon as possible. Delaying this process for too
    /// long might cause the icon to be displayed incorrectly. What counts as
    /// too long is compositor policy.
    ///
    /// If the client receives multiple configuration sequences, A followed by
    /// B, it may apply this process directly to B and skip A. This applies
    /// transitively.
    ///
    /// # Arguments
    ///
    /// - `serial_hi`: the high 32 bits of the serial
    /// - `serial_lo`: the low 32 bits of the serial
    #[inline]
    pub fn try_send_configure(
        &self,
        serial_hi: u32,
        serial_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial_hi,
            serial_lo,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= jay_icon_surface_v1#{}.configure(serial_hi: {}, serial_lo: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            1,
            arg0,
            arg1,
        ]);
        Ok(())
    }

    /// terminates a configuration sequence
    ///
    /// A configuration sequence consists of a number of preceding events
    /// terminated by this event. The client should store the contents of the
    /// preceding events and apply them atomically in response to this event.
    ///
    /// Each event that is part of the configuration sequence specifies this,
    /// along with its other semantics.
    ///
    /// Upon receiving this event, the client should send a
    /// jay_icon_surface_v1.ack_configure request with the same serial, prepare
    /// the wl_surface content update to match the requested configuration, and
    /// commit the wl_surface as soon as possible. Delaying this process for too
    /// long might cause the icon to be displayed incorrectly. What counts as
    /// too long is compositor policy.
    ///
    /// If the client receives multiple configuration sequences, A followed by
    /// B, it may apply this process directly to B and skip A. This applies
    /// transitively.
    ///
    /// # Arguments
    ///
    /// - `serial_hi`: the high 32 bits of the serial
    /// - `serial_lo`: the low 32 bits of the serial
    #[inline]
    pub fn send_configure(
        &self,
        serial_hi: u32,
        serial_lo: u32,
    ) {
        let res = self.try_send_configure(
            serial_hi,
            serial_lo,
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_v1.configure", &e);
        }
    }

    /// Since when the ack_configure message is available.
    pub const MSG__ACK_CONFIGURE__SINCE: u32 = 1;

    /// ack a configuration sequence
    ///
    /// This request informs the compositor that the content update committed
    /// by the next wl_surface.commit request conforms to the configuration
    /// sequence specified by the serial.
    ///
    /// The serial must be from a configuration sequence on this object and must
    /// be larger than the previous serial used with this request. Otherwise the
    /// invalid_serial error is raised.
    ///
    /// # Arguments
    ///
    /// - `serial_hi`: the high 32 bits of the serial
    /// - `serial_lo`: the low 32 bits of the serial
    #[inline]
    pub fn try_send_ack_configure(
        &self,
        serial_hi: u32,
        serial_lo: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial_hi,
            serial_lo,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= jay_icon_surface_v1#{}.ack_configure(serial_hi: {}, serial_lo: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1);
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
        ]);
        Ok(())
    }

    /// ack a configuration sequence
    ///
    /// This request informs the compositor that the content update committed
    /// by the next wl_surface.commit request conforms to the configuration
    /// sequence specified by the serial.
    ///
    /// The serial must be from a configuration sequence on this object and must
    /// be larger than the previous serial used with this request. Otherwise the
    /// invalid_serial error is raised.
    ///
    /// # Arguments
    ///
    /// - `serial_hi`: the high 32 bits of the serial
    /// - `serial_lo`: the low 32 bits of the serial
    #[inline]
    pub fn send_ack_configure(
        &self,
        serial_hi: u32,
        serial_lo: u32,
    ) {
        let res = self.try_send_ack_configure(
            serial_hi,
            serial_lo,
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_v1.ack_configure", &e);
        }
    }

    /// Since when the configure_size message is available.
    pub const MSG__CONFIGURE_SIZE__SINCE: u32 = 1;

    /// the size of the icon
    ///
    /// This event informs the client of the desired surface size.
    ///
    /// This event is part of a configuration sequence. See
    /// jay_icon_surface_v1.configure. If a configuration sequence does not
    /// contain this event, the client should consider the desired size
    /// unchanged.
    ///
    /// The first configuration sequence must contain this event.
    ///
    /// The width and height arguments are greater than 0.
    ///
    /// # Arguments
    ///
    /// - `width`: the width
    /// - `height`: the height
    #[inline]
    pub fn try_send_configure_size(
        &self,
        width: i32,
        height: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            width,
            height,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= jay_icon_surface_v1#{}.configure_size(width: {}, height: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1);
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
            2,
            arg0 as u32,
            arg1 as u32,
        ]);
        Ok(())
    }

    /// the size of the icon
    ///
    /// This event informs the client of the desired surface size.
    ///
    /// This event is part of a configuration sequence. See
    /// jay_icon_surface_v1.configure. If a configuration sequence does not
    /// contain this event, the client should consider the desired size
    /// unchanged.
    ///
    /// The first configuration sequence must contain this event.
    ///
    /// The width and height arguments are greater than 0.
    ///
    /// # Arguments
    ///
    /// - `width`: the width
    /// - `height`: the height
    #[inline]
    pub fn send_configure_size(
        &self,
        width: i32,
        height: i32,
    ) {
        let res = self.try_send_configure_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_send("jay_icon_surface_v1.configure_size", &e);
        }
    }
}

/// A message handler for [`JayIconSurfaceV1`] proxies.
pub trait JayIconSurfaceV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<JayIconSurfaceV1>) {
        slf.core.delete_id();
    }

    /// destroy this object
    ///
    /// This request destroys the jay_icon_surface_v1. It will no longer be
    /// displayed. The client should also destroy the underlying wl_surface.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<JayIconSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_v1.destroy", &e);
        }
    }

    /// the icon is no longer used
    ///
    /// The compositor sends this event when it no longer wants to use the
    /// jay_icon_surface_v1. The client should destroy the object.
    ///
    /// For example, the compositor might send this event when a task bar is
    /// hidden.
    ///
    /// The compositor must send this event when the underlying subject is
    /// destroyed.
    ///
    /// This event has no effect on other jay_icon_surface_v1 objects created
    /// from the same factory, which might still be used.
    #[inline]
    fn handle_finished(
        &mut self,
        slf: &Rc<JayIconSurfaceV1>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_finished(
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_v1.finished", &e);
        }
    }

    /// terminates a configuration sequence
    ///
    /// A configuration sequence consists of a number of preceding events
    /// terminated by this event. The client should store the contents of the
    /// preceding events and apply them atomically in response to this event.
    ///
    /// Each event that is part of the configuration sequence specifies this,
    /// along with its other semantics.
    ///
    /// Upon receiving this event, the client should send a
    /// jay_icon_surface_v1.ack_configure request with the same serial, prepare
    /// the wl_surface content update to match the requested configuration, and
    /// commit the wl_surface as soon as possible. Delaying this process for too
    /// long might cause the icon to be displayed incorrectly. What counts as
    /// too long is compositor policy.
    ///
    /// If the client receives multiple configuration sequences, A followed by
    /// B, it may apply this process directly to B and skip A. This applies
    /// transitively.
    ///
    /// # Arguments
    ///
    /// - `serial_hi`: the high 32 bits of the serial
    /// - `serial_lo`: the low 32 bits of the serial
    #[inline]
    fn handle_configure(
        &mut self,
        slf: &Rc<JayIconSurfaceV1>,
        serial_hi: u32,
        serial_lo: u32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure(
            serial_hi,
            serial_lo,
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_v1.configure", &e);
        }
    }

    /// ack a configuration sequence
    ///
    /// This request informs the compositor that the content update committed
    /// by the next wl_surface.commit request conforms to the configuration
    /// sequence specified by the serial.
    ///
    /// The serial must be from a configuration sequence on this object and must
    /// be larger than the previous serial used with this request. Otherwise the
    /// invalid_serial error is raised.
    ///
    /// # Arguments
    ///
    /// - `serial_hi`: the high 32 bits of the serial
    /// - `serial_lo`: the low 32 bits of the serial
    #[inline]
    fn handle_ack_configure(
        &mut self,
        slf: &Rc<JayIconSurfaceV1>,
        serial_hi: u32,
        serial_lo: u32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_ack_configure(
            serial_hi,
            serial_lo,
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_v1.ack_configure", &e);
        }
    }

    /// the size of the icon
    ///
    /// This event informs the client of the desired surface size.
    ///
    /// This event is part of a configuration sequence. See
    /// jay_icon_surface_v1.configure. If a configuration sequence does not
    /// contain this event, the client should consider the desired size
    /// unchanged.
    ///
    /// The first configuration sequence must contain this event.
    ///
    /// The width and height arguments are greater than 0.
    ///
    /// # Arguments
    ///
    /// - `width`: the width
    /// - `height`: the height
    #[inline]
    fn handle_configure_size(
        &mut self,
        slf: &Rc<JayIconSurfaceV1>,
        width: i32,
        height: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_configure_size(
            width,
            height,
        );
        if let Err(e) = res {
            log_forward("jay_icon_surface_v1.configure_size", &e);
        }
    }
}

impl ObjectPrivate for JayIconSurfaceV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::JayIconSurfaceV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_icon_surface_v1#{}.destroy()\n", client_id, id);
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
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> jay_icon_surface_v1#{}.ack_configure(serial_hi: {}, serial_lo: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_ack_configure(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_ack_configure(&self, arg0, arg1);
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
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> jay_icon_surface_v1#{}.finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finished(&self);
                } else {
                    DefaultHandler.handle_finished(&self);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> jay_icon_surface_v1#{}.configure(serial_hi: {}, serial_lo: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_configure(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_configure(&self, arg0, arg1);
                }
            }
            2 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> jay_icon_surface_v1#{}.configure_size(width: {}, height: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_configure_size(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_configure_size(&self, arg0, arg1);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "ack_configure",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "finished",
            1 => "configure",
            2 => "configure_size",
            _ => return None,
        };
        Some(name)
    }

    fn create_zombie(&self) -> Rc<dyn Object> {
        let slf = Self::new(&self.core.state, self.core.version);
        slf.core.make_zombie();
        slf
    }
}

impl Object for JayIconSurfaceV1 {
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

impl JayIconSurfaceV1 {
    /// Since when the error.invalid_serial enum variant is available.
    pub const ENM__ERROR_INVALID_SERIAL__SINCE: u32 = 1;
}

/// fatal error
///
/// These fatal protocol errors may be emitted in response to
/// invalid requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JayIconSurfaceV1Error(pub u32);

impl JayIconSurfaceV1Error {
    /// the serial is invalid
    pub const INVALID_SERIAL: Self = Self(0);
}

impl Debug for JayIconSurfaceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SERIAL => "INVALID_SERIAL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
