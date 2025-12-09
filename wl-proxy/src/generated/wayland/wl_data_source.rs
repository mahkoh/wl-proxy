//! offer to transfer data
//!
//! The wl_data_source object is the source side of a wl_data_offer.
//! It is created by the source client in a data transfer and
//! provides a way to describe the offered data and a way to respond
//! to requests to transfer the data.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wl_data_source proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWlDataSource {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWlDataSourceMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWlDataSourceMessageHandler for DefaultMessageHandler { }

impl MetaWlDataSource {
    pub const XML_VERSION: u32 = 3;
}

impl MetaWlDataSource {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WlDataSource, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaWlDataSourceMessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaWlDataSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWlDataSource")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWlDataSource {
    /// Since when the offer message is available.
    #[allow(dead_code)]
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
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the data source
    ///
    /// Destroy the data source.
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
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the target message is available.
    #[allow(dead_code)]
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
    pub fn send_target(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
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
        if let Some(arg0) = arg0 {
            fmt.string(arg0);
        } else {
            fmt.words([0]);
        }
        Ok(())
    }

    /// Since when the send message is available.
    #[allow(dead_code)]
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
    pub fn send_send(
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
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
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
        ]);
        fmt.string(arg0);
        fmt.fds.push_back(arg1.clone());
        Ok(())
    }

    /// Since when the cancelled message is available.
    #[allow(dead_code)]
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
    pub fn send_cancelled(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
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
        ]);
        Ok(())
    }

    /// Since when the set_actions message is available.
    #[allow(dead_code)]
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
    pub fn send_set_actions(
        &self,
        dnd_actions: MetaWlDataDeviceManagerDndAction,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            dnd_actions,
        );
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
            2,
            arg0.0,
        ]);
        Ok(())
    }

    /// Since when the dnd_drop_performed message is available.
    #[allow(dead_code)]
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
    pub fn send_dnd_drop_performed(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
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

    /// Since when the dnd_finished message is available.
    #[allow(dead_code)]
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
    pub fn send_dnd_finished(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
        let endpoint = &client.endpoint;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
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
}

/// A message handler for [WlDataSource] proxies.
#[allow(dead_code)]
pub trait MetaWlDataSourceMessageHandler {
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
    fn offer(
        &mut self,
        _slf: &Rc<MetaWlDataSource>,
        mime_type: &str,
    ) {
        let res = _slf.send_offer(
            mime_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_source.offer message: {}", Report::new(e));
        }
    }

    /// destroy the data source
    ///
    /// Destroy the data source.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaWlDataSource>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_source.destroy message: {}", Report::new(e));
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
    fn target(
        &mut self,
        _slf: &Rc<MetaWlDataSource>,
        mime_type: Option<&str>,
    ) {
        let res = _slf.send_target(
            mime_type,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_source.target message: {}", Report::new(e));
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
    fn send(
        &mut self,
        _slf: &Rc<MetaWlDataSource>,
        mime_type: &str,
        fd: &Rc<OwnedFd>,
    ) {
        let res = _slf.send_send(
            mime_type,
            fd,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_source.send message: {}", Report::new(e));
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
    fn cancelled(
        &mut self,
        _slf: &Rc<MetaWlDataSource>,
    ) {
        let res = _slf.send_cancelled(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_source.cancelled message: {}", Report::new(e));
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
    fn set_actions(
        &mut self,
        _slf: &Rc<MetaWlDataSource>,
        dnd_actions: MetaWlDataDeviceManagerDndAction,
    ) {
        let res = _slf.send_set_actions(
            dnd_actions,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_source.set_actions message: {}", Report::new(e));
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
    fn dnd_drop_performed(
        &mut self,
        _slf: &Rc<MetaWlDataSource>,
    ) {
        let res = _slf.send_dnd_drop_performed(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_source.dnd_drop_performed message: {}", Report::new(e));
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
    fn dnd_finished(
        &mut self,
        _slf: &Rc<MetaWlDataSource>,
    ) {
        let res = _slf.send_dnd_finished(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_source.dnd_finished message: {}", Report::new(e));
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
    fn action(
        &mut self,
        _slf: &Rc<MetaWlDataSource>,
        dnd_action: MetaWlDataDeviceManagerDndAction,
    ) {
        let res = _slf.send_action(
            dnd_action,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wl_data_source.action message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWlDataSource {
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
                if let Some(handler) = handler {
                    (**handler).offer(&self, arg0);
                } else {
                    DefaultMessageHandler.offer(&self, arg0);
                }
            }
            1 => {
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
            2 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaWlDataDeviceManagerDndAction(arg0);
                if let Some(handler) = handler {
                    (**handler).set_actions(&self, arg0);
                } else {
                    DefaultMessageHandler.set_actions(&self, arg0);
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
                if let Some(handler) = handler {
                    (**handler).target(&self, arg0);
                } else {
                    DefaultMessageHandler.target(&self, arg0);
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
                if let Some(handler) = handler {
                    (**handler).send(&self, arg0, arg1);
                } else {
                    DefaultMessageHandler.send(&self, arg0, arg1);
                }
            }
            2 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).cancelled(&self);
                } else {
                    DefaultMessageHandler.cancelled(&self);
                }
            }
            3 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).dnd_drop_performed(&self);
                } else {
                    DefaultMessageHandler.dnd_drop_performed(&self);
                }
            }
            4 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if let Some(handler) = handler {
                    (**handler).dnd_finished(&self);
                } else {
                    DefaultMessageHandler.dnd_finished(&self);
                }
            }
            5 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0 = MetaWlDataDeviceManagerDndAction(arg0);
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

impl MetaWlDataSource {
    /// Since when the error.invalid_action_mask enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_ACTION_MASK__SINCE: u32 = 1;
    /// Since when the error.invalid_source enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWlDataSourceError(pub u32);

impl MetaWlDataSourceError {
    /// action mask contains invalid values
    #[allow(dead_code)]
    pub const INVALID_ACTION_MASK: Self = Self(0);

    /// source doesn't accept this request
    #[allow(dead_code)]
    pub const INVALID_SOURCE: Self = Self(1);
}

impl Debug for MetaWlDataSourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_ACTION_MASK => "INVALID_ACTION_MASK",
            Self::INVALID_SOURCE => "INVALID_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
