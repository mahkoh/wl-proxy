//! offer to transfer data
//!
//! The wl_data_source object is the source side of a wl_data_offer.
//! It is created by the source client in a data transfer and
//! provides a way to describe the offered data and a way to respond
//! to requests to transfer the data.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_data_source object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlDataSource {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlDataSourceHandler>,
}

struct DefaultHandler;

impl WlDataSourceHandler for DefaultHandler { }

impl ConcreteObject for WlDataSource {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::WlDataSource;
    const INTERFACE_NAME: &str = "wl_data_source";
}

impl WlDataSource {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlDataSourceHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlDataSourceHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlDataSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlDataSource")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlDataSource {
    /// Since when the offer message is available.
    pub const MSG__OFFER__SINCE: u32 = 1;

    /// add an offered mime type
    ///
    /// This request adds a mime type to the set of mime types
    /// advertised to targets.  Can be called several times to offer
    /// multiple types.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type offered by the data source
    #[inline]
    pub fn try_send_offer(
        &self,
        mime_type: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mime_type,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_source#{}.offer(mime_type: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// add an offered mime type
    ///
    /// This request adds a mime type to the set of mime types
    /// advertised to targets.  Can be called several times to offer
    /// multiple types.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type offered by the data source
    #[inline]
    pub fn send_offer(
        &self,
        mime_type: &str,
    ) {
        let res = self.try_send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log_send("wl_data_source.offer", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the data source
    ///
    /// Destroy the data source.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_source#{}.destroy()\n", id);
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy the data source
    ///
    /// Destroy the data source.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wl_data_source.destroy", &e);
        }
    }

    /// Since when the target message is available.
    pub const MSG__TARGET__SINCE: u32 = 1;

    /// a target accepts an offered mime type
    ///
    /// Sent when a target accepts pointer_focus or motion events.  If
    /// a target does not accept any of the offered types, type is NULL.
    ///
    /// Used for feedback during drag-and-drop.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type accepted by the target
    #[inline]
    pub fn try_send_target(
        &self,
        mime_type: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            mime_type,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_source#{}.target(mime_type: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
        if let Some(arg0) = arg0 {
            fmt.string(arg0);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// a target accepts an offered mime type
    ///
    /// Sent when a target accepts pointer_focus or motion events.  If
    /// a target does not accept any of the offered types, type is NULL.
    ///
    /// Used for feedback during drag-and-drop.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type accepted by the target
    #[inline]
    pub fn send_target(
        &self,
        mime_type: Option<&str>,
    ) {
        let res = self.try_send_target(
            mime_type,
        );
        if let Err(e) = res {
            log_send("wl_data_source.target", &e);
        }
    }

    /// Since when the send message is available.
    pub const MSG__SEND__SINCE: u32 = 1;

    /// send the data
    ///
    /// Request for data from the client.  Send the data as the
    /// specified mime type over the passed file descriptor, then
    /// close it.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type for the data
    /// - `fd`: file descriptor for the data
    #[inline]
    pub fn try_send_send(
        &self,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            mime_type,
            fd,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_source#{}.send(mime_type: {:?}, fd: {})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1.as_raw_fd());
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
        ]);
        fmt.string(arg0);
        fmt.fds.push_back(arg1.clone());
        Ok(())
    }

    /// send the data
    ///
    /// Request for data from the client.  Send the data as the
    /// specified mime type over the passed file descriptor, then
    /// close it.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type for the data
    /// - `fd`: file descriptor for the data
    #[inline]
    pub fn send_send(
        &self,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = self.try_send_send(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log_send("wl_data_source.send", &e);
        }
    }

    /// Since when the cancelled message is available.
    pub const MSG__CANCELLED__SINCE: u32 = 1;

    /// selection was cancelled
    ///
    /// This data source is no longer valid. There are several reasons why
    /// this could happen:
    ///
    /// - The data source has been replaced by another data source.
    /// - The drag-and-drop operation was performed, but the drop destination
    ///   did not accept any of the mime types offered through
    ///   wl_data_source.target.
    /// - The drag-and-drop operation was performed, but the drop destination
    ///   did not select any of the actions present in the mask offered through
    ///   wl_data_source.action.
    /// - The drag-and-drop operation was performed but didn't happen over a
    ///   surface.
    /// - The compositor cancelled the drag-and-drop operation (e.g. compositor
    ///   dependent timeouts to avoid stale drag-and-drop transfers).
    ///
    /// The client should clean up and destroy this data source.
    ///
    /// For objects of version 2 or older, wl_data_source.cancelled will
    /// only be emitted if the data source was replaced by another data
    /// source.
    #[inline]
    pub fn try_send_cancelled(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_source#{}.cancelled()\n", client_id, id);
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
            2,
        ]);
        Ok(())
    }

    /// selection was cancelled
    ///
    /// This data source is no longer valid. There are several reasons why
    /// this could happen:
    ///
    /// - The data source has been replaced by another data source.
    /// - The drag-and-drop operation was performed, but the drop destination
    ///   did not accept any of the mime types offered through
    ///   wl_data_source.target.
    /// - The drag-and-drop operation was performed, but the drop destination
    ///   did not select any of the actions present in the mask offered through
    ///   wl_data_source.action.
    /// - The drag-and-drop operation was performed but didn't happen over a
    ///   surface.
    /// - The compositor cancelled the drag-and-drop operation (e.g. compositor
    ///   dependent timeouts to avoid stale drag-and-drop transfers).
    ///
    /// The client should clean up and destroy this data source.
    ///
    /// For objects of version 2 or older, wl_data_source.cancelled will
    /// only be emitted if the data source was replaced by another data
    /// source.
    #[inline]
    pub fn send_cancelled(
        &self,
    ) {
        let res = self.try_send_cancelled(
        );
        if let Err(e) = res {
            log_send("wl_data_source.cancelled", &e);
        }
    }

    /// Since when the set_actions message is available.
    pub const MSG__SET_ACTIONS__SINCE: u32 = 3;

    /// set the available drag-and-drop actions
    ///
    /// Sets the actions that the source side client supports for this
    /// operation. This request may trigger wl_data_source.action and
    /// wl_data_offer.action events if the compositor needs to change the
    /// selected action.
    ///
    /// The dnd_actions argument must contain only values expressed in the
    /// wl_data_device_manager.dnd_actions enum, otherwise it will result
    /// in a protocol error.
    ///
    /// This request must be made once only, and can only be made on sources
    /// used in drag-and-drop, so it must be performed before
    /// wl_data_device.start_drag. Attempting to use the source other than
    /// for drag-and-drop will raise a protocol error.
    ///
    /// # Arguments
    ///
    /// - `dnd_actions`: actions supported by the data source
    #[inline]
    pub fn try_send_set_actions(
        &self,
        dnd_actions: WlDataDeviceManagerDndAction,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            dnd_actions,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WlDataDeviceManagerDndAction) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_source#{}.set_actions(dnd_actions: {:?})\n", id, arg0);
                state.log(args);
            }
            log(&self.core.state, id, arg0);
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
            arg0.0,
        ]);
        Ok(())
    }

    /// set the available drag-and-drop actions
    ///
    /// Sets the actions that the source side client supports for this
    /// operation. This request may trigger wl_data_source.action and
    /// wl_data_offer.action events if the compositor needs to change the
    /// selected action.
    ///
    /// The dnd_actions argument must contain only values expressed in the
    /// wl_data_device_manager.dnd_actions enum, otherwise it will result
    /// in a protocol error.
    ///
    /// This request must be made once only, and can only be made on sources
    /// used in drag-and-drop, so it must be performed before
    /// wl_data_device.start_drag. Attempting to use the source other than
    /// for drag-and-drop will raise a protocol error.
    ///
    /// # Arguments
    ///
    /// - `dnd_actions`: actions supported by the data source
    #[inline]
    pub fn send_set_actions(
        &self,
        dnd_actions: WlDataDeviceManagerDndAction,
    ) {
        let res = self.try_send_set_actions(
            dnd_actions,
        );
        if let Err(e) = res {
            log_send("wl_data_source.set_actions", &e);
        }
    }

    /// Since when the dnd_drop_performed message is available.
    pub const MSG__DND_DROP_PERFORMED__SINCE: u32 = 3;

    /// the drag-and-drop operation physically finished
    ///
    /// The user performed the drop action. This event does not indicate
    /// acceptance, wl_data_source.cancelled may still be emitted afterwards
    /// if the drop destination does not accept any mime type.
    ///
    /// However, this event might however not be received if the compositor
    /// cancelled the drag-and-drop operation before this event could happen.
    ///
    /// Note that the data_source may still be used in the future and should
    /// not be destroyed here.
    #[inline]
    pub fn try_send_dnd_drop_performed(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_source#{}.dnd_drop_performed()\n", client_id, id);
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
            3,
        ]);
        Ok(())
    }

    /// the drag-and-drop operation physically finished
    ///
    /// The user performed the drop action. This event does not indicate
    /// acceptance, wl_data_source.cancelled may still be emitted afterwards
    /// if the drop destination does not accept any mime type.
    ///
    /// However, this event might however not be received if the compositor
    /// cancelled the drag-and-drop operation before this event could happen.
    ///
    /// Note that the data_source may still be used in the future and should
    /// not be destroyed here.
    #[inline]
    pub fn send_dnd_drop_performed(
        &self,
    ) {
        let res = self.try_send_dnd_drop_performed(
        );
        if let Err(e) = res {
            log_send("wl_data_source.dnd_drop_performed", &e);
        }
    }

    /// Since when the dnd_finished message is available.
    pub const MSG__DND_FINISHED__SINCE: u32 = 3;

    /// the drag-and-drop operation concluded
    ///
    /// The drop destination finished interoperating with this data
    /// source, so the client is now free to destroy this data source and
    /// free all associated data.
    ///
    /// If the action used to perform the operation was "move", the
    /// source can now delete the transferred data.
    #[inline]
    pub fn try_send_dnd_finished(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_source#{}.dnd_finished()\n", client_id, id);
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
            4,
        ]);
        Ok(())
    }

    /// the drag-and-drop operation concluded
    ///
    /// The drop destination finished interoperating with this data
    /// source, so the client is now free to destroy this data source and
    /// free all associated data.
    ///
    /// If the action used to perform the operation was "move", the
    /// source can now delete the transferred data.
    #[inline]
    pub fn send_dnd_finished(
        &self,
    ) {
        let res = self.try_send_dnd_finished(
        );
        if let Err(e) = res {
            log_send("wl_data_source.dnd_finished", &e);
        }
    }

    /// Since when the action message is available.
    pub const MSG__ACTION__SINCE: u32 = 3;

    /// notify the selected action
    ///
    /// This event indicates the action selected by the compositor after
    /// matching the source/destination side actions. Only one action (or
    /// none) will be offered here.
    ///
    /// This event can be emitted multiple times during the drag-and-drop
    /// operation, mainly in response to destination side changes through
    /// wl_data_offer.set_actions, and as the data device enters/leaves
    /// surfaces.
    ///
    /// It is only possible to receive this event after
    /// wl_data_source.dnd_drop_performed if the drag-and-drop operation
    /// ended in an "ask" action, in which case the final wl_data_source.action
    /// event will happen immediately before wl_data_source.dnd_finished.
    ///
    /// Compositors may also change the selected action on the fly, mainly
    /// in response to keyboard modifier changes during the drag-and-drop
    /// operation.
    ///
    /// The most recent action received is always the valid one. The chosen
    /// action may change alongside negotiation (e.g. an "ask" action can turn
    /// into a "move" operation), so the effects of the final action must
    /// always be applied in wl_data_offer.dnd_finished.
    ///
    /// Clients can trigger cursor surface changes from this point, so
    /// they reflect the current action.
    ///
    /// # Arguments
    ///
    /// - `dnd_action`: action selected by the compositor
    #[inline]
    pub fn try_send_action(
        &self,
        dnd_action: WlDataDeviceManagerDndAction,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            dnd_action,
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
            fn log(state: &State, client_id: u64, id: u32, arg0: WlDataDeviceManagerDndAction) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_source#{}.action(dnd_action: {:?})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
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
            5,
            arg0.0,
        ]);
        Ok(())
    }

    /// notify the selected action
    ///
    /// This event indicates the action selected by the compositor after
    /// matching the source/destination side actions. Only one action (or
    /// none) will be offered here.
    ///
    /// This event can be emitted multiple times during the drag-and-drop
    /// operation, mainly in response to destination side changes through
    /// wl_data_offer.set_actions, and as the data device enters/leaves
    /// surfaces.
    ///
    /// It is only possible to receive this event after
    /// wl_data_source.dnd_drop_performed if the drag-and-drop operation
    /// ended in an "ask" action, in which case the final wl_data_source.action
    /// event will happen immediately before wl_data_source.dnd_finished.
    ///
    /// Compositors may also change the selected action on the fly, mainly
    /// in response to keyboard modifier changes during the drag-and-drop
    /// operation.
    ///
    /// The most recent action received is always the valid one. The chosen
    /// action may change alongside negotiation (e.g. an "ask" action can turn
    /// into a "move" operation), so the effects of the final action must
    /// always be applied in wl_data_offer.dnd_finished.
    ///
    /// Clients can trigger cursor surface changes from this point, so
    /// they reflect the current action.
    ///
    /// # Arguments
    ///
    /// - `dnd_action`: action selected by the compositor
    #[inline]
    pub fn send_action(
        &self,
        dnd_action: WlDataDeviceManagerDndAction,
    ) {
        let res = self.try_send_action(
            dnd_action,
        );
        if let Err(e) = res {
            log_send("wl_data_source.action", &e);
        }
    }
}

/// A message handler for [`WlDataSource`] proxies.
pub trait WlDataSourceHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlDataSource>) {
        slf.core.delete_id();
    }

    /// add an offered mime type
    ///
    /// This request adds a mime type to the set of mime types
    /// advertised to targets.  Can be called several times to offer
    /// multiple types.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type offered by the data source
    #[inline]
    fn handle_offer(
        &mut self,
        slf: &Rc<WlDataSource>,
        mime_type: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log_forward("wl_data_source.offer", &e);
        }
    }

    /// destroy the data source
    ///
    /// Destroy the data source.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WlDataSource>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wl_data_source.destroy", &e);
        }
    }

    /// a target accepts an offered mime type
    ///
    /// Sent when a target accepts pointer_focus or motion events.  If
    /// a target does not accept any of the offered types, type is NULL.
    ///
    /// Used for feedback during drag-and-drop.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type accepted by the target
    #[inline]
    fn handle_target(
        &mut self,
        slf: &Rc<WlDataSource>,
        mime_type: Option<&str>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_target(
            mime_type,
        );
        if let Err(e) = res {
            log_forward("wl_data_source.target", &e);
        }
    }

    /// send the data
    ///
    /// Request for data from the client.  Send the data as the
    /// specified mime type over the passed file descriptor, then
    /// close it.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type for the data
    /// - `fd`: file descriptor for the data
    #[inline]
    fn handle_send(
        &mut self,
        slf: &Rc<WlDataSource>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_send(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log_forward("wl_data_source.send", &e);
        }
    }

    /// selection was cancelled
    ///
    /// This data source is no longer valid. There are several reasons why
    /// this could happen:
    ///
    /// - The data source has been replaced by another data source.
    /// - The drag-and-drop operation was performed, but the drop destination
    ///   did not accept any of the mime types offered through
    ///   wl_data_source.target.
    /// - The drag-and-drop operation was performed, but the drop destination
    ///   did not select any of the actions present in the mask offered through
    ///   wl_data_source.action.
    /// - The drag-and-drop operation was performed but didn't happen over a
    ///   surface.
    /// - The compositor cancelled the drag-and-drop operation (e.g. compositor
    ///   dependent timeouts to avoid stale drag-and-drop transfers).
    ///
    /// The client should clean up and destroy this data source.
    ///
    /// For objects of version 2 or older, wl_data_source.cancelled will
    /// only be emitted if the data source was replaced by another data
    /// source.
    #[inline]
    fn handle_cancelled(
        &mut self,
        slf: &Rc<WlDataSource>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_cancelled(
        );
        if let Err(e) = res {
            log_forward("wl_data_source.cancelled", &e);
        }
    }

    /// set the available drag-and-drop actions
    ///
    /// Sets the actions that the source side client supports for this
    /// operation. This request may trigger wl_data_source.action and
    /// wl_data_offer.action events if the compositor needs to change the
    /// selected action.
    ///
    /// The dnd_actions argument must contain only values expressed in the
    /// wl_data_device_manager.dnd_actions enum, otherwise it will result
    /// in a protocol error.
    ///
    /// This request must be made once only, and can only be made on sources
    /// used in drag-and-drop, so it must be performed before
    /// wl_data_device.start_drag. Attempting to use the source other than
    /// for drag-and-drop will raise a protocol error.
    ///
    /// # Arguments
    ///
    /// - `dnd_actions`: actions supported by the data source
    #[inline]
    fn handle_set_actions(
        &mut self,
        slf: &Rc<WlDataSource>,
        dnd_actions: WlDataDeviceManagerDndAction,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_actions(
            dnd_actions,
        );
        if let Err(e) = res {
            log_forward("wl_data_source.set_actions", &e);
        }
    }

    /// the drag-and-drop operation physically finished
    ///
    /// The user performed the drop action. This event does not indicate
    /// acceptance, wl_data_source.cancelled may still be emitted afterwards
    /// if the drop destination does not accept any mime type.
    ///
    /// However, this event might however not be received if the compositor
    /// cancelled the drag-and-drop operation before this event could happen.
    ///
    /// Note that the data_source may still be used in the future and should
    /// not be destroyed here.
    #[inline]
    fn handle_dnd_drop_performed(
        &mut self,
        slf: &Rc<WlDataSource>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dnd_drop_performed(
        );
        if let Err(e) = res {
            log_forward("wl_data_source.dnd_drop_performed", &e);
        }
    }

    /// the drag-and-drop operation concluded
    ///
    /// The drop destination finished interoperating with this data
    /// source, so the client is now free to destroy this data source and
    /// free all associated data.
    ///
    /// If the action used to perform the operation was "move", the
    /// source can now delete the transferred data.
    #[inline]
    fn handle_dnd_finished(
        &mut self,
        slf: &Rc<WlDataSource>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_dnd_finished(
        );
        if let Err(e) = res {
            log_forward("wl_data_source.dnd_finished", &e);
        }
    }

    /// notify the selected action
    ///
    /// This event indicates the action selected by the compositor after
    /// matching the source/destination side actions. Only one action (or
    /// none) will be offered here.
    ///
    /// This event can be emitted multiple times during the drag-and-drop
    /// operation, mainly in response to destination side changes through
    /// wl_data_offer.set_actions, and as the data device enters/leaves
    /// surfaces.
    ///
    /// It is only possible to receive this event after
    /// wl_data_source.dnd_drop_performed if the drag-and-drop operation
    /// ended in an "ask" action, in which case the final wl_data_source.action
    /// event will happen immediately before wl_data_source.dnd_finished.
    ///
    /// Compositors may also change the selected action on the fly, mainly
    /// in response to keyboard modifier changes during the drag-and-drop
    /// operation.
    ///
    /// The most recent action received is always the valid one. The chosen
    /// action may change alongside negotiation (e.g. an "ask" action can turn
    /// into a "move" operation), so the effects of the final action must
    /// always be applied in wl_data_offer.dnd_finished.
    ///
    /// Clients can trigger cursor surface changes from this point, so
    /// they reflect the current action.
    ///
    /// # Arguments
    ///
    /// - `dnd_action`: action selected by the compositor
    #[inline]
    fn handle_action(
        &mut self,
        slf: &Rc<WlDataSource>,
        dnd_action: WlDataDeviceManagerDndAction,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_action(
            dnd_action,
        );
        if let Err(e) = res {
            log_forward("wl_data_source.action", &e);
        }
    }
}

impl ObjectPrivate for WlDataSource {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlDataSource, version),
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "mime_type")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_source#{}.offer(mime_type: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_offer(&self, arg0);
                } else {
                    DefaultHandler.handle_offer(&self, arg0);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_source#{}.destroy()\n", client_id, id);
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
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WlDataDeviceManagerDndAction(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WlDataDeviceManagerDndAction) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_source#{}.set_actions(dnd_actions: {:?})\n", client_id, id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_actions(&self, arg0);
                } else {
                    DefaultHandler.handle_set_actions(&self, arg0);
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
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NullableString>(msg, offset, "mime_type")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_source#{}.target(mime_type: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_target(&self, arg0);
                } else {
                    DefaultHandler.handle_target(&self, arg0);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0;
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "mime_type")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("fd")));
                };
                let arg1 = &arg1;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_source#{}.send(mime_type: {:?}, fd: {})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1.as_raw_fd());
                }
                if let Some(handler) = handler {
                    (**handler).handle_send(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_send(&self, arg0, arg1);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_source#{}.cancelled()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_cancelled(&self);
                } else {
                    DefaultHandler.handle_cancelled(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_source#{}.dnd_drop_performed()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dnd_drop_performed(&self);
                } else {
                    DefaultHandler.handle_dnd_drop_performed(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_source#{}.dnd_finished()\n", id);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_dnd_finished(&self);
                } else {
                    DefaultHandler.handle_dnd_finished(&self);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = WlDataDeviceManagerDndAction(arg0);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: WlDataDeviceManagerDndAction) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_source#{}.action(dnd_action: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_action(&self, arg0);
                } else {
                    DefaultHandler.handle_action(&self, arg0);
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
            0 => "offer",
            1 => "destroy",
            2 => "set_actions",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "target",
            1 => "send",
            2 => "cancelled",
            3 => "dnd_drop_performed",
            4 => "dnd_finished",
            5 => "action",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlDataSource {
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

impl WlDataSource {
    /// Since when the error.invalid_action_mask enum variant is available.
    pub const ENM__ERROR_INVALID_ACTION_MASK__SINCE: u32 = 1;
    /// Since when the error.invalid_source enum variant is available.
    pub const ENM__ERROR_INVALID_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlDataSourceError(pub u32);

impl WlDataSourceError {
    /// action mask contains invalid values
    pub const INVALID_ACTION_MASK: Self = Self(0);

    /// source doesn't accept this request
    pub const INVALID_SOURCE: Self = Self(1);
}

impl Debug for WlDataSourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_ACTION_MASK => "INVALID_ACTION_MASK",
            Self::INVALID_SOURCE => "INVALID_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
