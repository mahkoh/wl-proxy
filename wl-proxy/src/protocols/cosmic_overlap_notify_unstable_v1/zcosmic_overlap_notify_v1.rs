//! get notifications of other elements overlapping layer surfaces
//!
//! The purpose of this protocol is to enable layer-shell client to get
//! notifications if part of their surfaces are occluded other elements
//! (currently toplevels and other layer-surfaces).
//!
//! You can request a notification object for any of your zwlr_layer_surface_v1
//! surfaces, which will then emit overlap events.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zcosmic_overlap_notify_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZcosmicOverlapNotifyV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZcosmicOverlapNotifyV1Handler>,
}

struct DefaultHandler;

impl ZcosmicOverlapNotifyV1Handler for DefaultHandler { }

impl ConcreteObject for ZcosmicOverlapNotifyV1 {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::ZcosmicOverlapNotifyV1;
    const INTERFACE_NAME: &str = "zcosmic_overlap_notify_v1";
}

impl ZcosmicOverlapNotifyV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZcosmicOverlapNotifyV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZcosmicOverlapNotifyV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZcosmicOverlapNotifyV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZcosmicOverlapNotifyV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZcosmicOverlapNotifyV1 {
    /// Since when the notify_on_overlap message is available.
    pub const MSG__NOTIFY_ON_OVERLAP__SINCE: u32 = 1;

    /// get notified if a layer-shell is obstructed by a toplevel
    ///
    /// Requests notifications for toplevels and layer-surfaces entering and leaving the
    /// surface-area of the given zwlr_layer_surface_v1. This can be used e.g. to
    /// implement auto-hide functionality.
    ///
    /// To stop receiving notifications, destroy the returned
    /// zcosmic_overlap_notification_v1 object.
    ///
    /// # Arguments
    ///
    /// - `overlap_notification`:
    /// - `layer_surface`:
    #[inline]
    pub fn try_send_notify_on_overlap(
        &self,
        overlap_notification: &Rc<ZcosmicOverlapNotificationV1>,
        layer_surface: &Rc<ZwlrLayerSurfaceV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            overlap_notification,
            layer_surface,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("layer_surface"))),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("overlap_notification", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zcosmic_overlap_notify_v1#{}.notify_on_overlap(overlap_notification: zcosmic_overlap_notification_v1#{}, layer_surface: zwlr_layer_surface_v1#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
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
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// get notified if a layer-shell is obstructed by a toplevel
    ///
    /// Requests notifications for toplevels and layer-surfaces entering and leaving the
    /// surface-area of the given zwlr_layer_surface_v1. This can be used e.g. to
    /// implement auto-hide functionality.
    ///
    /// To stop receiving notifications, destroy the returned
    /// zcosmic_overlap_notification_v1 object.
    ///
    /// # Arguments
    ///
    /// - `overlap_notification`:
    /// - `layer_surface`:
    #[inline]
    pub fn send_notify_on_overlap(
        &self,
        overlap_notification: &Rc<ZcosmicOverlapNotificationV1>,
        layer_surface: &Rc<ZwlrLayerSurfaceV1>,
    ) {
        let res = self.try_send_notify_on_overlap(
            overlap_notification,
            layer_surface,
        );
        if let Err(e) = res {
            log_send("zcosmic_overlap_notify_v1.notify_on_overlap", &e);
        }
    }

    /// get notified if a layer-shell is obstructed by a toplevel
    ///
    /// Requests notifications for toplevels and layer-surfaces entering and leaving the
    /// surface-area of the given zwlr_layer_surface_v1. This can be used e.g. to
    /// implement auto-hide functionality.
    ///
    /// To stop receiving notifications, destroy the returned
    /// zcosmic_overlap_notification_v1 object.
    ///
    /// # Arguments
    ///
    /// - `layer_surface`:
    #[inline]
    pub fn new_try_send_notify_on_overlap(
        &self,
        layer_surface: &Rc<ZwlrLayerSurfaceV1>,
    ) -> Result<Rc<ZcosmicOverlapNotificationV1>, ObjectError> {
        let overlap_notification = self.core.create_child();
        self.try_send_notify_on_overlap(
            &overlap_notification,
            layer_surface,
        )?;
        Ok(overlap_notification)
    }

    /// get notified if a layer-shell is obstructed by a toplevel
    ///
    /// Requests notifications for toplevels and layer-surfaces entering and leaving the
    /// surface-area of the given zwlr_layer_surface_v1. This can be used e.g. to
    /// implement auto-hide functionality.
    ///
    /// To stop receiving notifications, destroy the returned
    /// zcosmic_overlap_notification_v1 object.
    ///
    /// # Arguments
    ///
    /// - `layer_surface`:
    #[inline]
    pub fn new_send_notify_on_overlap(
        &self,
        layer_surface: &Rc<ZwlrLayerSurfaceV1>,
    ) -> Rc<ZcosmicOverlapNotificationV1> {
        let overlap_notification = self.core.create_child();
        self.send_notify_on_overlap(
            &overlap_notification,
            layer_surface,
        );
        overlap_notification
    }
}

/// A message handler for [`ZcosmicOverlapNotifyV1`] proxies.
pub trait ZcosmicOverlapNotifyV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZcosmicOverlapNotifyV1>) {
        slf.core.delete_id();
    }

    /// get notified if a layer-shell is obstructed by a toplevel
    ///
    /// Requests notifications for toplevels and layer-surfaces entering and leaving the
    /// surface-area of the given zwlr_layer_surface_v1. This can be used e.g. to
    /// implement auto-hide functionality.
    ///
    /// To stop receiving notifications, destroy the returned
    /// zcosmic_overlap_notification_v1 object.
    ///
    /// # Arguments
    ///
    /// - `overlap_notification`:
    /// - `layer_surface`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_notify_on_overlap(
        &mut self,
        slf: &Rc<ZcosmicOverlapNotifyV1>,
        overlap_notification: &Rc<ZcosmicOverlapNotificationV1>,
        layer_surface: &Rc<ZwlrLayerSurfaceV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_notify_on_overlap(
            overlap_notification,
            layer_surface,
        );
        if let Err(e) = res {
            log_forward("zcosmic_overlap_notify_v1.notify_on_overlap", &e);
        }
    }
}

impl ObjectPrivate for ZcosmicOverlapNotifyV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZcosmicOverlapNotifyV1, version),
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zcosmic_overlap_notify_v1#{}.notify_on_overlap(overlap_notification: zcosmic_overlap_notification_v1#{}, layer_surface: zwlr_layer_surface_v1#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let arg0 = ZcosmicOverlapNotificationV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "overlap_notification", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<ZwlrLayerSurfaceV1>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("layer_surface", o.core().interface, ObjectInterface::ZwlrLayerSurfaceV1)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_notify_on_overlap(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_notify_on_overlap(&self, arg0, arg1);
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
            0 => "notify_on_overlap",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for ZcosmicOverlapNotifyV1 {
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

