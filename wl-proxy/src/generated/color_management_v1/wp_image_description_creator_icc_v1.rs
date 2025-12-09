//! holder of image description ICC information
//!
//! This type of object is used for collecting all the information required
//! to create a wp_image_description_v1 object from an ICC file. A complete
//! set of required parameters consists of these properties:
//! - ICC file
//!
//! Each required property must be set exactly once if the client is to create
//! an image description. The set requests verify that a property was not
//! already set. The create request verifies that all required properties are
//! set. There may be several alternative requests for setting each property,
//! and in that case the client must choose one of them.
//!
//! Once all properties have been set, the create request must be used to
//! create the image description object, destroying the creator in the
//! process.
//!
//! The link between a pixel value (a device value in ICC) and its respective
//! colorimetry is defined by the details of the particular ICC profile.
//! Those details also determine when colorimetry becomes undefined.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_image_description_creator_icc_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpImageDescriptionCreatorIccV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpImageDescriptionCreatorIccV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpImageDescriptionCreatorIccV1MessageHandler for DefaultMessageHandler { }

impl MetaWpImageDescriptionCreatorIccV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaWpImageDescriptionCreatorIccV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::WpImageDescriptionCreatorIccV1, version),
            handler: Default::default(),
        })
    }

    pub fn set_handler(&self, handler: Box<dyn MetaWpImageDescriptionCreatorIccV1MessageHandler>) {
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }
}

impl Debug for MetaWpImageDescriptionCreatorIccV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpImageDescriptionCreatorIccV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpImageDescriptionCreatorIccV1 {
    /// Since when the create message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE__SINCE: u32 = 1;

    /// Create the image description object from ICC data
    ///
    /// Create an image description object based on the ICC information
    /// previously set on this object. A compositor must parse the ICC data in
    /// some undefined but finite amount of time.
    ///
    /// The completeness of the parameter set is verified. If the set is not
    /// complete, the protocol error incomplete_set is raised. For the
    /// definition of a complete set, see the description of this interface.
    ///
    /// If the particular combination of the information is not supported
    /// by the compositor, the resulting image description object shall
    /// immediately deliver the wp_image_description_v1.failed event with the
    /// 'unsupported' cause. If a valid image description was created from the
    /// information, the wp_image_description_v1.ready event will eventually
    /// be sent instead.
    ///
    /// This request destroys the wp_image_description_creator_icc_v1 object.
    ///
    /// The resulting image description object does not allow get_information
    /// request.
    #[inline]
    pub fn send_create(
        &self,
        image_description: &Rc<MetaWpImageDescriptionV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            image_description,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("image_description", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
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
            arg0_id,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the set_icc_file message is available.
    #[allow(dead_code)]
    pub const MSG__SET_ICC_FILE__SINCE: u32 = 1;

    /// set the ICC profile file
    ///
    /// Sets the ICC profile file to be used as the basis of the image
    /// description.
    ///
    /// The data shall be found through the given fd at the given offset, having
    /// the given length. The fd must be seekable and readable. Violating these
    /// requirements raises the bad_fd protocol error.
    ///
    /// If reading the data fails due to an error independent of the client, the
    /// compositor shall send the wp_image_description_v1.failed event on the
    /// created wp_image_description_v1 with the 'operating_system' cause.
    ///
    /// The maximum size of the ICC profile is 32 MB. If length is greater than
    /// that or zero, the protocol error bad_size is raised. If offset + length
    /// exceeds the file size, the protocol error out_of_file is raised.
    ///
    /// A compositor may read the file at any time starting from this request
    /// and only until whichever happens first:
    /// - If create request was issued, the wp_image_description_v1 object
    ///   delivers either failed or ready event; or
    /// - if create request was not issued, this
    ///   wp_image_description_creator_icc_v1 object is destroyed.
    ///
    /// A compositor shall not modify the contents of the file, and the fd may
    /// be sealed for writes and size changes. The client must ensure to its
    /// best ability that the data does not change while the compositor is
    /// reading it.
    ///
    /// The data must represent a valid ICC profile. The ICC profile version
    /// must be 2 or 4, it must be a 3 channel profile and the class must be
    /// Display or ColorSpace. Violating these requirements will not result in a
    /// protocol error, but will eventually send the
    /// wp_image_description_v1.failed event on the created
    /// wp_image_description_v1 with the 'unsupported' cause.
    ///
    /// See the International Color Consortium specification ICC.1:2022 for more
    /// details about ICC profiles.
    ///
    /// If ICC file has already been set on this object, the protocol error
    /// already_set is raised.
    ///
    /// # Arguments
    ///
    /// - `icc_profile`: ICC profile
    /// - `offset`: byte offset in fd to start of ICC data
    /// - `length`: length of ICC data in bytes
    #[inline]
    pub fn send_set_icc_file(
        &self,
        icc_profile: &Rc<OwnedFd>,
        offset: u32,
        length: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            icc_profile,
            offset,
            length,
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
        fmt.fds.push_back(arg0.clone());
        fmt.words([
            id,
            1,
            arg1,
            arg2,
        ]);
        Ok(())
    }
}

/// A message handler for [WpImageDescriptionCreatorIccV1] proxies.
#[allow(dead_code)]
pub trait MetaWpImageDescriptionCreatorIccV1MessageHandler {
    /// Create the image description object from ICC data
    ///
    /// Create an image description object based on the ICC information
    /// previously set on this object. A compositor must parse the ICC data in
    /// some undefined but finite amount of time.
    ///
    /// The completeness of the parameter set is verified. If the set is not
    /// complete, the protocol error incomplete_set is raised. For the
    /// definition of a complete set, see the description of this interface.
    ///
    /// If the particular combination of the information is not supported
    /// by the compositor, the resulting image description object shall
    /// immediately deliver the wp_image_description_v1.failed event with the
    /// 'unsupported' cause. If a valid image description was created from the
    /// information, the wp_image_description_v1.ready event will eventually
    /// be sent instead.
    ///
    /// This request destroys the wp_image_description_creator_icc_v1 object.
    ///
    /// The resulting image description object does not allow get_information
    /// request.
    ///
    /// # Arguments
    ///
    /// - `image_description`:
    #[inline]
    fn create(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorIccV1>,
        image_description: &Rc<MetaWpImageDescriptionV1>,
    ) {
        let res = _slf.send_create(
            image_description,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_icc_v1.create message: {}", Report::new(e));
        }
    }

    /// set the ICC profile file
    ///
    /// Sets the ICC profile file to be used as the basis of the image
    /// description.
    ///
    /// The data shall be found through the given fd at the given offset, having
    /// the given length. The fd must be seekable and readable. Violating these
    /// requirements raises the bad_fd protocol error.
    ///
    /// If reading the data fails due to an error independent of the client, the
    /// compositor shall send the wp_image_description_v1.failed event on the
    /// created wp_image_description_v1 with the 'operating_system' cause.
    ///
    /// The maximum size of the ICC profile is 32 MB. If length is greater than
    /// that or zero, the protocol error bad_size is raised. If offset + length
    /// exceeds the file size, the protocol error out_of_file is raised.
    ///
    /// A compositor may read the file at any time starting from this request
    /// and only until whichever happens first:
    /// - If create request was issued, the wp_image_description_v1 object
    ///   delivers either failed or ready event; or
    /// - if create request was not issued, this
    ///   wp_image_description_creator_icc_v1 object is destroyed.
    ///
    /// A compositor shall not modify the contents of the file, and the fd may
    /// be sealed for writes and size changes. The client must ensure to its
    /// best ability that the data does not change while the compositor is
    /// reading it.
    ///
    /// The data must represent a valid ICC profile. The ICC profile version
    /// must be 2 or 4, it must be a 3 channel profile and the class must be
    /// Display or ColorSpace. Violating these requirements will not result in a
    /// protocol error, but will eventually send the
    /// wp_image_description_v1.failed event on the created
    /// wp_image_description_v1 with the 'unsupported' cause.
    ///
    /// See the International Color Consortium specification ICC.1:2022 for more
    /// details about ICC profiles.
    ///
    /// If ICC file has already been set on this object, the protocol error
    /// already_set is raised.
    ///
    /// # Arguments
    ///
    /// - `icc_profile`: ICC profile
    /// - `offset`: byte offset in fd to start of ICC data
    /// - `length`: length of ICC data in bytes
    #[inline]
    fn set_icc_file(
        &mut self,
        _slf: &Rc<MetaWpImageDescriptionCreatorIccV1>,
        icc_profile: &Rc<OwnedFd>,
        offset: u32,
        length: u32,
    ) {
        let res = _slf.send_set_icc_file(
            icc_profile,
            offset,
            length,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_image_description_creator_icc_v1.set_icc_file message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpImageDescriptionCreatorIccV1 {
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
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 12));
                };
                let arg0_id = arg0;
                let arg0 = MetaWpImageDescriptionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "image_description", e))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).create(&self, arg0);
                } else {
                    DefaultMessageHandler.create(&self, arg0);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                let [
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 16));
                };
                let Some(arg0) = fds.pop_front() else {
                    return Err(ObjectError::MissingFd("icc_profile"));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).set_icc_file(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.set_icc_file(&self, arg0, arg1, arg2);
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
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "create",
            1 => "set_icc_file",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaWpImageDescriptionCreatorIccV1 {
    /// Since when the error.incomplete_set enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INCOMPLETE_SET__SINCE: u32 = 1;
    /// Since when the error.already_set enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_SET__SINCE: u32 = 1;
    /// Since when the error.bad_fd enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_FD__SINCE: u32 = 1;
    /// Since when the error.bad_size enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_SIZE__SINCE: u32 = 1;
    /// Since when the error.out_of_file enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_OUT_OF_FILE__SINCE: u32 = 1;
}

/// protocol errors
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaWpImageDescriptionCreatorIccV1Error(pub u32);

impl MetaWpImageDescriptionCreatorIccV1Error {
    /// incomplete parameter set
    #[allow(dead_code)]
    pub const INCOMPLETE_SET: Self = Self(0);

    /// property already set
    #[allow(dead_code)]
    pub const ALREADY_SET: Self = Self(1);

    /// fd not seekable and readable
    #[allow(dead_code)]
    pub const BAD_FD: Self = Self(2);

    /// no or too much data
    #[allow(dead_code)]
    pub const BAD_SIZE: Self = Self(3);

    /// offset + length exceeds file size
    #[allow(dead_code)]
    pub const OUT_OF_FILE: Self = Self(4);
}

impl Debug for MetaWpImageDescriptionCreatorIccV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INCOMPLETE_SET => "INCOMPLETE_SET",
            Self::ALREADY_SET => "ALREADY_SET",
            Self::BAD_FD => "BAD_FD",
            Self::BAD_SIZE => "BAD_SIZE",
            Self::OUT_OF_FILE => "OUT_OF_FILE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
