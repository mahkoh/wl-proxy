//! offer to transfer data
//!
//! A wl_data_offer represents a piece of data offered for transfer
//! by another client (the source client).  It is used by the
//! copy-and-paste and drag-and-drop mechanisms.  The offer
//! describes the different mime types that the data can be
//! converted to and provides the mechanism for transferring the
//! data directly from the source client.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_data_offer object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlDataOffer {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlDataOfferHandler>,
}

struct DefaultHandler;

impl WlDataOfferHandler for DefaultHandler { }

impl ConcreteObject for WlDataOffer {
    const XML_VERSION: u32 = 3;
    const INTERFACE: ObjectInterface = ObjectInterface::WlDataOffer;
    const INTERFACE_NAME: &str = "wl_data_offer";
}

impl WlDataOffer {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlDataOfferHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlDataOfferHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlDataOffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlDataOffer")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlDataOffer {
    /// Since when the accept message is available.
    pub const MSG__ACCEPT__SINCE: u32 = 1;

    /// accept one of the offered mime types
    ///
    /// Indicate that the client can accept the given mime type, or
    /// NULL for not accepted.
    ///
    /// For objects of version 2 or older, this request is used by the
    /// client to give feedback whether the client can receive the given
    /// mime type, or NULL if none is accepted; the feedback does not
    /// determine whether the drag-and-drop operation succeeds or not.
    ///
    /// For objects of version 3 or newer, this request determines the
    /// final result of the drag-and-drop operation. If the end result
    /// is that no mime types were accepted, the drag-and-drop operation
    /// will be cancelled and the corresponding drag source will receive
    /// wl_data_source.cancelled. Clients may still use this event in
    /// conjunction with wl_data_source.action for feedback.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the accept request
    /// - `mime_type`: mime type accepted by the client
    #[inline]
    pub fn try_send_accept(
        &self,
        serial: u32,
        mime_type: Option<&str>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            serial,
            mime_type,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: Option<&str>) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_offer#{}.accept(serial: {}, mime_type: {:?})\n", id, arg0, arg1);
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
            0,
            arg0,
        ]);
        if let Some(arg1) = arg1 {
            fmt.string(arg1);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// accept one of the offered mime types
    ///
    /// Indicate that the client can accept the given mime type, or
    /// NULL for not accepted.
    ///
    /// For objects of version 2 or older, this request is used by the
    /// client to give feedback whether the client can receive the given
    /// mime type, or NULL if none is accepted; the feedback does not
    /// determine whether the drag-and-drop operation succeeds or not.
    ///
    /// For objects of version 3 or newer, this request determines the
    /// final result of the drag-and-drop operation. If the end result
    /// is that no mime types were accepted, the drag-and-drop operation
    /// will be cancelled and the corresponding drag source will receive
    /// wl_data_source.cancelled. Clients may still use this event in
    /// conjunction with wl_data_source.action for feedback.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the accept request
    /// - `mime_type`: mime type accepted by the client
    #[inline]
    pub fn send_accept(
        &self,
        serial: u32,
        mime_type: Option<&str>,
    ) {
        let res = self.try_send_accept(
            serial,
            mime_type,
        );
        if let Err(e) = res {
            log_send("wl_data_offer.accept", &e);
        }
    }

    /// Since when the receive message is available.
    pub const MSG__RECEIVE__SINCE: u32 = 1;

    /// request that the data is transferred
    ///
    /// To transfer the offered data, the client issues this request
    /// and indicates the mime type it wants to receive.  The transfer
    /// happens through the passed file descriptor (typically created
    /// with the pipe system call).  The source client writes the data
    /// in the mime type representation requested and then closes the
    /// file descriptor.
    ///
    /// The receiving client reads from the read end of the pipe until
    /// EOF and then closes its end, at which point the transfer is
    /// complete.
    ///
    /// This request may happen multiple times for different mime types,
    /// both before and after wl_data_device.drop. Drag-and-drop destination
    /// clients may preemptively fetch data or examine it more closely to
    /// determine acceptance.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type desired by receiver
    /// - `fd`: file descriptor for data transfer
    #[inline]
    pub fn try_send_receive(
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
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: &str, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_offer#{}.receive(mime_type: {:?}, fd: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0, arg1.as_raw_fd());
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
        fmt.string(arg0);
        fmt.fds.push_back(arg1.clone());
        Ok(())
    }

    /// request that the data is transferred
    ///
    /// To transfer the offered data, the client issues this request
    /// and indicates the mime type it wants to receive.  The transfer
    /// happens through the passed file descriptor (typically created
    /// with the pipe system call).  The source client writes the data
    /// in the mime type representation requested and then closes the
    /// file descriptor.
    ///
    /// The receiving client reads from the read end of the pipe until
    /// EOF and then closes its end, at which point the transfer is
    /// complete.
    ///
    /// This request may happen multiple times for different mime types,
    /// both before and after wl_data_device.drop. Drag-and-drop destination
    /// clients may preemptively fetch data or examine it more closely to
    /// determine acceptance.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type desired by receiver
    /// - `fd`: file descriptor for data transfer
    #[inline]
    pub fn send_receive(
        &self,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = self.try_send_receive(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log_send("wl_data_offer.receive", &e);
        }
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy data offer
    ///
    /// Destroy the data offer.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_offer#{}.destroy()\n", id);
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
        self.core.handle_server_destroy();
        Ok(())
    }

    /// destroy data offer
    ///
    /// Destroy the data offer.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("wl_data_offer.destroy", &e);
        }
    }

    /// Since when the offer message is available.
    pub const MSG__OFFER__SINCE: u32 = 1;

    /// advertise offered mime type
    ///
    /// Sent immediately after creating the wl_data_offer object.  One
    /// event per offered mime type.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: offered mime type
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
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_offer#{}.offer(mime_type: {:?})\n", client_id, id, arg0);
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
        fmt.string(arg0);
        Ok(())
    }

    /// advertise offered mime type
    ///
    /// Sent immediately after creating the wl_data_offer object.  One
    /// event per offered mime type.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: offered mime type
    #[inline]
    pub fn send_offer(
        &self,
        mime_type: &str,
    ) {
        let res = self.try_send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log_send("wl_data_offer.offer", &e);
        }
    }

    /// Since when the finish message is available.
    pub const MSG__FINISH__SINCE: u32 = 3;

    /// the offer will no longer be used
    ///
    /// Notifies the compositor that the drag destination successfully
    /// finished the drag-and-drop operation.
    ///
    /// Upon receiving this request, the compositor will emit
    /// wl_data_source.dnd_finished on the drag source client.
    ///
    /// It is a client error to perform other requests than
    /// wl_data_offer.destroy after this one. It is also an error to perform
    /// this request after a NULL mime type has been set in
    /// wl_data_offer.accept or no action was received through
    /// wl_data_offer.action.
    ///
    /// If wl_data_offer.finish request is received for a non drag and drop
    /// operation, the invalid_finish protocol error is raised.
    #[inline]
    pub fn try_send_finish(
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_offer#{}.finish()\n", id);
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
            3,
        ]);
        Ok(())
    }

    /// the offer will no longer be used
    ///
    /// Notifies the compositor that the drag destination successfully
    /// finished the drag-and-drop operation.
    ///
    /// Upon receiving this request, the compositor will emit
    /// wl_data_source.dnd_finished on the drag source client.
    ///
    /// It is a client error to perform other requests than
    /// wl_data_offer.destroy after this one. It is also an error to perform
    /// this request after a NULL mime type has been set in
    /// wl_data_offer.accept or no action was received through
    /// wl_data_offer.action.
    ///
    /// If wl_data_offer.finish request is received for a non drag and drop
    /// operation, the invalid_finish protocol error is raised.
    #[inline]
    pub fn send_finish(
        &self,
    ) {
        let res = self.try_send_finish(
        );
        if let Err(e) = res {
            log_send("wl_data_offer.finish", &e);
        }
    }

    /// Since when the set_actions message is available.
    pub const MSG__SET_ACTIONS__SINCE: u32 = 3;

    /// set the available/preferred drag-and-drop actions
    ///
    /// Sets the actions that the destination side client supports for
    /// this operation. This request may trigger the emission of
    /// wl_data_source.action and wl_data_offer.action events if the compositor
    /// needs to change the selected action.
    ///
    /// This request can be called multiple times throughout the
    /// drag-and-drop operation, typically in response to wl_data_device.enter
    /// or wl_data_device.motion events.
    ///
    /// This request determines the final result of the drag-and-drop
    /// operation. If the end result is that no action is accepted,
    /// the drag source will receive wl_data_source.cancelled.
    ///
    /// The dnd_actions argument must contain only values expressed in the
    /// wl_data_device_manager.dnd_actions enum, and the preferred_action
    /// argument must only contain one of those values set, otherwise it
    /// will result in a protocol error.
    ///
    /// While managing an "ask" action, the destination drag-and-drop client
    /// may perform further wl_data_offer.receive requests, and is expected
    /// to perform one last wl_data_offer.set_actions request with a preferred
    /// action other than "ask" (and optionally wl_data_offer.accept) before
    /// requesting wl_data_offer.finish, in order to convey the action selected
    /// by the user. If the preferred action is not in the
    /// wl_data_offer.source_actions mask, an error will be raised.
    ///
    /// If the "ask" action is dismissed (e.g. user cancellation), the client
    /// is expected to perform wl_data_offer.destroy right away.
    ///
    /// This request can only be made on drag-and-drop offers, a protocol error
    /// will be raised otherwise.
    ///
    /// # Arguments
    ///
    /// - `dnd_actions`: actions supported by the destination client
    /// - `preferred_action`: action preferred by the destination client
    #[inline]
    pub fn try_send_set_actions(
        &self,
        dnd_actions: WlDataDeviceManagerDndAction,
        preferred_action: WlDataDeviceManagerDndAction,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            dnd_actions,
            preferred_action,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: WlDataDeviceManagerDndAction, arg1: WlDataDeviceManagerDndAction) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_data_offer#{}.set_actions(dnd_actions: {:?}, preferred_action: {:?})\n", id, arg0, arg1);
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
            4,
            arg0.0,
            arg1.0,
        ]);
        Ok(())
    }

    /// set the available/preferred drag-and-drop actions
    ///
    /// Sets the actions that the destination side client supports for
    /// this operation. This request may trigger the emission of
    /// wl_data_source.action and wl_data_offer.action events if the compositor
    /// needs to change the selected action.
    ///
    /// This request can be called multiple times throughout the
    /// drag-and-drop operation, typically in response to wl_data_device.enter
    /// or wl_data_device.motion events.
    ///
    /// This request determines the final result of the drag-and-drop
    /// operation. If the end result is that no action is accepted,
    /// the drag source will receive wl_data_source.cancelled.
    ///
    /// The dnd_actions argument must contain only values expressed in the
    /// wl_data_device_manager.dnd_actions enum, and the preferred_action
    /// argument must only contain one of those values set, otherwise it
    /// will result in a protocol error.
    ///
    /// While managing an "ask" action, the destination drag-and-drop client
    /// may perform further wl_data_offer.receive requests, and is expected
    /// to perform one last wl_data_offer.set_actions request with a preferred
    /// action other than "ask" (and optionally wl_data_offer.accept) before
    /// requesting wl_data_offer.finish, in order to convey the action selected
    /// by the user. If the preferred action is not in the
    /// wl_data_offer.source_actions mask, an error will be raised.
    ///
    /// If the "ask" action is dismissed (e.g. user cancellation), the client
    /// is expected to perform wl_data_offer.destroy right away.
    ///
    /// This request can only be made on drag-and-drop offers, a protocol error
    /// will be raised otherwise.
    ///
    /// # Arguments
    ///
    /// - `dnd_actions`: actions supported by the destination client
    /// - `preferred_action`: action preferred by the destination client
    #[inline]
    pub fn send_set_actions(
        &self,
        dnd_actions: WlDataDeviceManagerDndAction,
        preferred_action: WlDataDeviceManagerDndAction,
    ) {
        let res = self.try_send_set_actions(
            dnd_actions,
            preferred_action,
        );
        if let Err(e) = res {
            log_send("wl_data_offer.set_actions", &e);
        }
    }

    /// Since when the source_actions message is available.
    pub const MSG__SOURCE_ACTIONS__SINCE: u32 = 3;

    /// notify the source-side available actions
    ///
    /// This event indicates the actions offered by the data source. It
    /// will be sent immediately after creating the wl_data_offer object,
    /// or anytime the source side changes its offered actions through
    /// wl_data_source.set_actions.
    ///
    /// # Arguments
    ///
    /// - `source_actions`: actions offered by the data source
    #[inline]
    pub fn try_send_source_actions(
        &self,
        source_actions: WlDataDeviceManagerDndAction,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            source_actions,
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_offer#{}.source_actions(source_actions: {:?})\n", client_id, id, arg0);
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
            1,
            arg0.0,
        ]);
        Ok(())
    }

    /// notify the source-side available actions
    ///
    /// This event indicates the actions offered by the data source. It
    /// will be sent immediately after creating the wl_data_offer object,
    /// or anytime the source side changes its offered actions through
    /// wl_data_source.set_actions.
    ///
    /// # Arguments
    ///
    /// - `source_actions`: actions offered by the data source
    #[inline]
    pub fn send_source_actions(
        &self,
        source_actions: WlDataDeviceManagerDndAction,
    ) {
        let res = self.try_send_source_actions(
            source_actions,
        );
        if let Err(e) = res {
            log_send("wl_data_offer.source_actions", &e);
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
    /// operation in response to destination side action changes through
    /// wl_data_offer.set_actions.
    ///
    /// This event will no longer be emitted after wl_data_device.drop
    /// happened on the drag-and-drop destination, the client must
    /// honor the last action received, or the last preferred one set
    /// through wl_data_offer.set_actions when handling an "ask" action.
    ///
    /// Compositors may also change the selected action on the fly, mainly
    /// in response to keyboard modifier changes during the drag-and-drop
    /// operation.
    ///
    /// The most recent action received is always the valid one. Prior to
    /// receiving wl_data_device.drop, the chosen action may change (e.g.
    /// due to keyboard modifiers being pressed). At the time of receiving
    /// wl_data_device.drop the drag-and-drop destination must honor the
    /// last action received.
    ///
    /// Action changes may still happen after wl_data_device.drop,
    /// especially on "ask" actions, where the drag-and-drop destination
    /// may choose another action afterwards. Action changes happening
    /// at this stage are always the result of inter-client negotiation, the
    /// compositor shall no longer be able to induce a different action.
    ///
    /// Upon "ask" actions, it is expected that the drag-and-drop destination
    /// may potentially choose a different action and/or mime type,
    /// based on wl_data_offer.source_actions and finally chosen by the
    /// user (e.g. popping up a menu with the available options). The
    /// final wl_data_offer.set_actions and wl_data_offer.accept requests
    /// must happen before the call to wl_data_offer.finish.
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
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_data_offer#{}.action(dnd_action: {:?})\n", client_id, id, arg0);
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
            2,
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
    /// operation in response to destination side action changes through
    /// wl_data_offer.set_actions.
    ///
    /// This event will no longer be emitted after wl_data_device.drop
    /// happened on the drag-and-drop destination, the client must
    /// honor the last action received, or the last preferred one set
    /// through wl_data_offer.set_actions when handling an "ask" action.
    ///
    /// Compositors may also change the selected action on the fly, mainly
    /// in response to keyboard modifier changes during the drag-and-drop
    /// operation.
    ///
    /// The most recent action received is always the valid one. Prior to
    /// receiving wl_data_device.drop, the chosen action may change (e.g.
    /// due to keyboard modifiers being pressed). At the time of receiving
    /// wl_data_device.drop the drag-and-drop destination must honor the
    /// last action received.
    ///
    /// Action changes may still happen after wl_data_device.drop,
    /// especially on "ask" actions, where the drag-and-drop destination
    /// may choose another action afterwards. Action changes happening
    /// at this stage are always the result of inter-client negotiation, the
    /// compositor shall no longer be able to induce a different action.
    ///
    /// Upon "ask" actions, it is expected that the drag-and-drop destination
    /// may potentially choose a different action and/or mime type,
    /// based on wl_data_offer.source_actions and finally chosen by the
    /// user (e.g. popping up a menu with the available options). The
    /// final wl_data_offer.set_actions and wl_data_offer.accept requests
    /// must happen before the call to wl_data_offer.finish.
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
            log_send("wl_data_offer.action", &e);
        }
    }
}

/// A message handler for [`WlDataOffer`] proxies.
pub trait WlDataOfferHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlDataOffer>) {
        slf.core.delete_id();
    }

    /// accept one of the offered mime types
    ///
    /// Indicate that the client can accept the given mime type, or
    /// NULL for not accepted.
    ///
    /// For objects of version 2 or older, this request is used by the
    /// client to give feedback whether the client can receive the given
    /// mime type, or NULL if none is accepted; the feedback does not
    /// determine whether the drag-and-drop operation succeeds or not.
    ///
    /// For objects of version 3 or newer, this request determines the
    /// final result of the drag-and-drop operation. If the end result
    /// is that no mime types were accepted, the drag-and-drop operation
    /// will be cancelled and the corresponding drag source will receive
    /// wl_data_source.cancelled. Clients may still use this event in
    /// conjunction with wl_data_source.action for feedback.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the accept request
    /// - `mime_type`: mime type accepted by the client
    #[inline]
    fn handle_accept(
        &mut self,
        slf: &Rc<WlDataOffer>,
        serial: u32,
        mime_type: Option<&str>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_accept(
            serial,
            mime_type,
        );
        if let Err(e) = res {
            log_forward("wl_data_offer.accept", &e);
        }
    }

    /// request that the data is transferred
    ///
    /// To transfer the offered data, the client issues this request
    /// and indicates the mime type it wants to receive.  The transfer
    /// happens through the passed file descriptor (typically created
    /// with the pipe system call).  The source client writes the data
    /// in the mime type representation requested and then closes the
    /// file descriptor.
    ///
    /// The receiving client reads from the read end of the pipe until
    /// EOF and then closes its end, at which point the transfer is
    /// complete.
    ///
    /// This request may happen multiple times for different mime types,
    /// both before and after wl_data_device.drop. Drag-and-drop destination
    /// clients may preemptively fetch data or examine it more closely to
    /// determine acceptance.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type desired by receiver
    /// - `fd`: file descriptor for data transfer
    #[inline]
    fn handle_receive(
        &mut self,
        slf: &Rc<WlDataOffer>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_receive(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log_forward("wl_data_offer.receive", &e);
        }
    }

    /// destroy data offer
    ///
    /// Destroy the data offer.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<WlDataOffer>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("wl_data_offer.destroy", &e);
        }
    }

    /// advertise offered mime type
    ///
    /// Sent immediately after creating the wl_data_offer object.  One
    /// event per offered mime type.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: offered mime type
    #[inline]
    fn handle_offer(
        &mut self,
        slf: &Rc<WlDataOffer>,
        mime_type: &str,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log_forward("wl_data_offer.offer", &e);
        }
    }

    /// the offer will no longer be used
    ///
    /// Notifies the compositor that the drag destination successfully
    /// finished the drag-and-drop operation.
    ///
    /// Upon receiving this request, the compositor will emit
    /// wl_data_source.dnd_finished on the drag source client.
    ///
    /// It is a client error to perform other requests than
    /// wl_data_offer.destroy after this one. It is also an error to perform
    /// this request after a NULL mime type has been set in
    /// wl_data_offer.accept or no action was received through
    /// wl_data_offer.action.
    ///
    /// If wl_data_offer.finish request is received for a non drag and drop
    /// operation, the invalid_finish protocol error is raised.
    #[inline]
    fn handle_finish(
        &mut self,
        slf: &Rc<WlDataOffer>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_finish(
        );
        if let Err(e) = res {
            log_forward("wl_data_offer.finish", &e);
        }
    }

    /// set the available/preferred drag-and-drop actions
    ///
    /// Sets the actions that the destination side client supports for
    /// this operation. This request may trigger the emission of
    /// wl_data_source.action and wl_data_offer.action events if the compositor
    /// needs to change the selected action.
    ///
    /// This request can be called multiple times throughout the
    /// drag-and-drop operation, typically in response to wl_data_device.enter
    /// or wl_data_device.motion events.
    ///
    /// This request determines the final result of the drag-and-drop
    /// operation. If the end result is that no action is accepted,
    /// the drag source will receive wl_data_source.cancelled.
    ///
    /// The dnd_actions argument must contain only values expressed in the
    /// wl_data_device_manager.dnd_actions enum, and the preferred_action
    /// argument must only contain one of those values set, otherwise it
    /// will result in a protocol error.
    ///
    /// While managing an "ask" action, the destination drag-and-drop client
    /// may perform further wl_data_offer.receive requests, and is expected
    /// to perform one last wl_data_offer.set_actions request with a preferred
    /// action other than "ask" (and optionally wl_data_offer.accept) before
    /// requesting wl_data_offer.finish, in order to convey the action selected
    /// by the user. If the preferred action is not in the
    /// wl_data_offer.source_actions mask, an error will be raised.
    ///
    /// If the "ask" action is dismissed (e.g. user cancellation), the client
    /// is expected to perform wl_data_offer.destroy right away.
    ///
    /// This request can only be made on drag-and-drop offers, a protocol error
    /// will be raised otherwise.
    ///
    /// # Arguments
    ///
    /// - `dnd_actions`: actions supported by the destination client
    /// - `preferred_action`: action preferred by the destination client
    #[inline]
    fn handle_set_actions(
        &mut self,
        slf: &Rc<WlDataOffer>,
        dnd_actions: WlDataDeviceManagerDndAction,
        preferred_action: WlDataDeviceManagerDndAction,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_set_actions(
            dnd_actions,
            preferred_action,
        );
        if let Err(e) = res {
            log_forward("wl_data_offer.set_actions", &e);
        }
    }

    /// notify the source-side available actions
    ///
    /// This event indicates the actions offered by the data source. It
    /// will be sent immediately after creating the wl_data_offer object,
    /// or anytime the source side changes its offered actions through
    /// wl_data_source.set_actions.
    ///
    /// # Arguments
    ///
    /// - `source_actions`: actions offered by the data source
    #[inline]
    fn handle_source_actions(
        &mut self,
        slf: &Rc<WlDataOffer>,
        source_actions: WlDataDeviceManagerDndAction,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_source_actions(
            source_actions,
        );
        if let Err(e) = res {
            log_forward("wl_data_offer.source_actions", &e);
        }
    }

    /// notify the selected action
    ///
    /// This event indicates the action selected by the compositor after
    /// matching the source/destination side actions. Only one action (or
    /// none) will be offered here.
    ///
    /// This event can be emitted multiple times during the drag-and-drop
    /// operation in response to destination side action changes through
    /// wl_data_offer.set_actions.
    ///
    /// This event will no longer be emitted after wl_data_device.drop
    /// happened on the drag-and-drop destination, the client must
    /// honor the last action received, or the last preferred one set
    /// through wl_data_offer.set_actions when handling an "ask" action.
    ///
    /// Compositors may also change the selected action on the fly, mainly
    /// in response to keyboard modifier changes during the drag-and-drop
    /// operation.
    ///
    /// The most recent action received is always the valid one. Prior to
    /// receiving wl_data_device.drop, the chosen action may change (e.g.
    /// due to keyboard modifiers being pressed). At the time of receiving
    /// wl_data_device.drop the drag-and-drop destination must honor the
    /// last action received.
    ///
    /// Action changes may still happen after wl_data_device.drop,
    /// especially on "ask" actions, where the drag-and-drop destination
    /// may choose another action afterwards. Action changes happening
    /// at this stage are always the result of inter-client negotiation, the
    /// compositor shall no longer be able to induce a different action.
    ///
    /// Upon "ask" actions, it is expected that the drag-and-drop destination
    /// may potentially choose a different action and/or mime type,
    /// based on wl_data_offer.source_actions and finally chosen by the
    /// user (e.g. popping up a menu with the available options). The
    /// final wl_data_offer.set_actions and wl_data_offer.accept requests
    /// must happen before the call to wl_data_offer.finish.
    ///
    /// # Arguments
    ///
    /// - `dnd_action`: action selected by the compositor
    #[inline]
    fn handle_action(
        &mut self,
        slf: &Rc<WlDataOffer>,
        dnd_action: WlDataDeviceManagerDndAction,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_action(
            dnd_action,
        );
        if let Err(e) = res {
            log_forward("wl_data_offer.action", &e);
        }
    }
}

impl ObjectPrivate for WlDataOffer {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlDataOffer, version),
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
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("serial")));
                };
                offset += 1;
                let arg1;
                (arg1, offset) = parse_string::<NullableString>(msg, offset, "mime_type")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: Option<&str>) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_offer#{}.accept(serial: {}, mime_type: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_accept(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_accept(&self, arg0, arg1);
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
                    fn log(state: &State, client_id: u64, id: u32, arg0: &str, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_offer#{}.receive(mime_type: {:?}, fd: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1.as_raw_fd());
                }
                if let Some(handler) = handler {
                    (**handler).handle_receive(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_receive(&self, arg0, arg1);
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_offer#{}.destroy()\n", client_id, id);
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
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_offer#{}.finish()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                if let Some(handler) = handler {
                    (**handler).handle_finish(&self);
                } else {
                    DefaultHandler.handle_finish(&self);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = WlDataDeviceManagerDndAction(arg0);
                let arg1 = WlDataDeviceManagerDndAction(arg1);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: WlDataDeviceManagerDndAction, arg1: WlDataDeviceManagerDndAction) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_data_offer#{}.set_actions(dnd_actions: {:?}, preferred_action: {:?})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                if let Some(handler) = handler {
                    (**handler).handle_set_actions(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_set_actions(&self, arg0, arg1);
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
                (arg0, offset) = parse_string::<NonNullString>(msg, offset, "mime_type")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_offer#{}.offer(mime_type: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_offer(&self, arg0);
                } else {
                    DefaultHandler.handle_offer(&self, arg0);
                }
            }
            1 => {
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
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_offer#{}.source_actions(source_actions: {:?})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_source_actions(&self, arg0);
                } else {
                    DefaultHandler.handle_source_actions(&self, arg0);
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
                    fn log(state: &State, id: u32, arg0: WlDataDeviceManagerDndAction) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_data_offer#{}.action(dnd_action: {:?})\n", id, arg0);
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
            0 => "accept",
            1 => "receive",
            2 => "destroy",
            3 => "finish",
            4 => "set_actions",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "offer",
            1 => "source_actions",
            2 => "action",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlDataOffer {
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

impl WlDataOffer {
    /// Since when the error.invalid_finish enum variant is available.
    pub const ENM__ERROR_INVALID_FINISH__SINCE: u32 = 1;
    /// Since when the error.invalid_action_mask enum variant is available.
    pub const ENM__ERROR_INVALID_ACTION_MASK__SINCE: u32 = 1;
    /// Since when the error.invalid_action enum variant is available.
    pub const ENM__ERROR_INVALID_ACTION__SINCE: u32 = 1;
    /// Since when the error.invalid_offer enum variant is available.
    pub const ENM__ERROR_INVALID_OFFER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlDataOfferError(pub u32);

impl WlDataOfferError {
    /// finish request was called untimely
    pub const INVALID_FINISH: Self = Self(0);

    /// action mask contains invalid values
    pub const INVALID_ACTION_MASK: Self = Self(1);

    /// action argument has an invalid value
    pub const INVALID_ACTION: Self = Self(2);

    /// offer doesn't accept this request
    pub const INVALID_OFFER: Self = Self(3);
}

impl Debug for WlDataOfferError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_FINISH => "INVALID_FINISH",
            Self::INVALID_ACTION_MASK => "INVALID_ACTION_MASK",
            Self::INVALID_ACTION => "INVALID_ACTION",
            Self::INVALID_OFFER => "INVALID_OFFER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
