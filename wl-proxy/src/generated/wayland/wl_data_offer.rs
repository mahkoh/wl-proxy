//! offer to transfer data
//!
//! A wl_data_offer represents a piece of data offered for transfer
//! by another client (the source client).  It is used by the
//! copy-and-paste and drag-and-drop mechanisms.  The offer
//! describes the different mime types that the data can be
//! converted to and provides the mechanism for transferring the
//! data directly from the source client.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_data_offer proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlDataOffer {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlDataOfferMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlDataOfferMessageHandler for DefaultMessageHandler { }

impl MetaWlDataOffer {
    pub const XML_VERSION: u32 = 3;
}

impl MetaWlDataOffer {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlDataOffer, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWlDataOffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlDataOffer")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlDataOffer {
    /// Since when the accept message is available.
    #[allow(dead_code)]
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
    pub fn send_accept(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_data_offer#{}.accept(serial: {}, mime_type: {:?})", id, arg0, arg1);
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
            arg0,
        ]);
        if let Some(arg1) = arg1 {
            fmt.string(arg1);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// Since when the receive message is available.
    #[allow(dead_code)]
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
    pub fn send_receive(
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_data_offer#{}.receive(mime_type: {:?}, fd: {})", id, arg0, arg1.as_raw_fd());
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
        ]);
        fmt.string(arg0);
        fmt.fds.push_back(arg1.clone());
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy data offer
    ///
    /// Destroy the data offer.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_data_offer#{}.destroy()", id);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the offer message is available.
    #[allow(dead_code)]
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
    pub fn send_offer(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wl_data_offer#{}.offer(mime_type: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
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
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the finish message is available.
    #[allow(dead_code)]
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
    pub fn send_finish(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_data_offer#{}.finish()", id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the set_actions message is available.
    #[allow(dead_code)]
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
    pub fn send_set_actions(
        &self,
        dnd_actions: MetaWlDataDeviceManagerDndAction,
        preferred_action: MetaWlDataDeviceManagerDndAction,
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
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= wl_data_offer#{}.set_actions(dnd_actions: {:?}, preferred_action: {:?})", id, arg0, arg1);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the source_actions message is available.
    #[allow(dead_code)]
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
    pub fn send_source_actions(
        &self,
        source_actions: MetaWlDataDeviceManagerDndAction,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            source_actions,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wl_data_offer#{}.source_actions(source_actions: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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

    /// Since when the action message is available.
    #[allow(dead_code)]
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
    pub fn send_action(
        &self,
        dnd_action: MetaWlDataDeviceManagerDndAction,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            dnd_action,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        eprintln!("client#{:04} <= wl_data_offer#{}.action(dnd_action: {:?})", client.endpoint.id, id, arg0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
}

/// A message handler for [WlDataOffer] proxies.
#[allow(dead_code)]
pub trait MetaWlDataOfferMessageHandler {
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
    fn accept(
        &mut self,
        _slf: &Rc<MetaWlDataOffer>,
        serial: u32,
        mime_type: Option<&str>,
    ) {
        let res = _slf.send_accept(
            serial,
            mime_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_offer.accept message: {}", Report::new(e));
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
    fn receive(
        &mut self,
        _slf: &Rc<MetaWlDataOffer>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_receive(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_offer.receive message: {}", Report::new(e));
        }
    }

    /// destroy data offer
    ///
    /// Destroy the data offer.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWlDataOffer>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_offer.destroy message: {}", Report::new(e));
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
    fn offer(
        &mut self,
        _slf: &Rc<MetaWlDataOffer>,
        mime_type: &str,
    ) {
        let res = _slf.send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_offer.offer message: {}", Report::new(e));
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
    fn finish(
        &mut self,
        _slf: &Rc<MetaWlDataOffer>,
    ) {
        let res = _slf.send_finish(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_offer.finish message: {}", Report::new(e));
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
    fn set_actions(
        &mut self,
        _slf: &Rc<MetaWlDataOffer>,
        dnd_actions: MetaWlDataDeviceManagerDndAction,
        preferred_action: MetaWlDataDeviceManagerDndAction,
    ) {
        let res = _slf.send_set_actions(
            dnd_actions,
            preferred_action,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_offer.set_actions message: {}", Report::new(e));
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
    fn source_actions(
        &mut self,
        _slf: &Rc<MetaWlDataOffer>,
        source_actions: MetaWlDataDeviceManagerDndAction,
    ) {
        let res = _slf.send_source_actions(
            source_actions,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_offer.source_actions message: {}", Report::new(e));
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
    fn action(
        &mut self,
        _slf: &Rc<MetaWlDataOffer>,
        dnd_action: MetaWlDataDeviceManagerDndAction,
    ) {
        let res = _slf.send_action(
            dnd_action,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_offer.action message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlDataOffer {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("serial"));
                };
                offset += 1;
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("mime_type"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("mime_type"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        None
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("mime_type"));
                        };
                        Some(s)
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("client#{:04} -> wl_data_offer#{}.accept(serial: {}, mime_type: {:?})", client.endpoint.id, msg[0], arg0, arg1);
                if let Some(handler) = handler {
                    (**handler).accept(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.accept(&self, arg0, arg1);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("mime_type"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("mime_type"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("mime_type"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("mime_type"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let Some(arg1) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("fd"));
                };
                let arg1 = &arg1;
                eprintln!("client#{:04} -> wl_data_offer#{}.receive(mime_type: {:?}, fd: {})", client.endpoint.id, msg[0], arg0, arg1.as_raw_fd());
                if let Some(handler) = handler {
                    (**handler).receive(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.receive(&self, arg0, arg1);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> wl_data_offer#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> wl_data_offer#{}.finish()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).finish(&self);
                } else {
                    DefaultMessageHandler.finish(&self);
                }
            }
            4 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let arg0 = MetaWlDataDeviceManagerDndAction(arg0);
                let arg1 = MetaWlDataDeviceManagerDndAction(arg1);
                eprintln!("client#{:04} -> wl_data_offer#{}.set_actions(dnd_actions: {:?}, preferred_action: {:?})", client.endpoint.id, msg[0], arg0, arg1);
                if let Some(handler) = handler {
                    (**handler).set_actions(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.set_actions(&self, arg0, arg1);
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
            0 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("mime_type"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("mime_type"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("mime_type"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("mime_type"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                eprintln!("server      -> wl_data_offer#{}.offer(mime_type: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).offer(&self, arg0);
                } else {
                    DefaultMessageHandler.offer(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaWlDataDeviceManagerDndAction(arg0);
                eprintln!("server      -> wl_data_offer#{}.source_actions(source_actions: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).source_actions(&self, arg0);
                } else {
                    DefaultMessageHandler.source_actions(&self, arg0);
                }
            }
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaWlDataDeviceManagerDndAction(arg0);
                eprintln!("server      -> wl_data_offer#{}.action(dnd_action: {:?})", msg[0], arg0);
                if let Some(handler) = handler {
                    (**handler).action(&self, arg0);
                } else {
                    DefaultMessageHandler.action(&self, arg0);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
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

impl MetaWlDataOffer {
    /// Since when the error.invalid_finish enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_FINISH__SINCE: u32 = 1;
    /// Since when the error.invalid_action_mask enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_ACTION_MASK__SINCE: u32 = 1;
    /// Since when the error.invalid_action enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_ACTION__SINCE: u32 = 1;
    /// Since when the error.invalid_offer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_OFFER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlDataOfferError(pub u32);

impl MetaWlDataOfferError {
    /// finish request was called untimely
    #[allow(dead_code)]
    pub const INVALID_FINISH: Self = Self(0);

    /// action mask contains invalid values
    #[allow(dead_code)]
    pub const INVALID_ACTION_MASK: Self = Self(1);

    /// action argument has an invalid value
    #[allow(dead_code)]
    pub const INVALID_ACTION: Self = Self(2);

    /// offer doesn't accept this request
    #[allow(dead_code)]
    pub const INVALID_OFFER: Self = Self(3);
}

impl Debug for MetaWlDataOfferError {
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
