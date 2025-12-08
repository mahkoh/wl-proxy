//! manager to inform clients and begin capturing
//!
//! This object is a manager which offers requests to start capturing from a
//! source.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A ext_image_copy_capture_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaExtImageCopyCaptureManagerV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaExtImageCopyCaptureManagerV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaExtImageCopyCaptureManagerV1MessageHandler for DefaultMessageHandler { }

impl MetaExtImageCopyCaptureManagerV1 {
    pub const XML_VERSION: u32 = 1;
}

impl MetaExtImageCopyCaptureManagerV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ExtImageCopyCaptureManagerV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaExtImageCopyCaptureManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaExtImageCopyCaptureManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaExtImageCopyCaptureManagerV1 {
    /// Since when the create_session message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_SESSION__SINCE: u32 = 1;

    /// capture an image capture source
    ///
    /// Create a capturing session for an image capture source.
    ///
    /// If the paint_cursors option is set, cursors shall be composited onto
    /// the captured frame. The cursor must not be composited onto the frame
    /// if this flag is not set.
    ///
    /// If the options bitfield is invalid, the invalid_option protocol error
    /// is sent.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `source`:
    /// - `options`:
    #[inline]
    pub fn send_create_session(
        &self,
        session: &Rc<MetaExtImageCopyCaptureSessionV1>,
        source: &Rc<MetaExtImageCaptureSourceV1>,
        options: MetaExtImageCopyCaptureManagerV1Options,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            session,
            source,
            options,
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
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// Since when the create_pointer_cursor_session message is available.
    #[allow(dead_code)]
    pub const MSG__CREATE_POINTER_CURSOR_SESSION__SINCE: u32 = 1;

    /// capture the pointer cursor of an image capture source
    ///
    /// Create a cursor capturing session for the pointer of an image capture
    /// source.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `source`:
    /// - `pointer`:
    #[inline]
    pub fn send_create_pointer_cursor_session(
        &self,
        session: &Rc<MetaExtImageCopyCaptureCursorSessionV1>,
        source: &Rc<MetaExtImageCaptureSourceV1>,
        pointer: &Rc<MetaWlPointer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            session,
            source,
            pointer,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
        };
        let arg1 = match arg1.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        let arg2 = match arg2.server_obj_id.get() {
            None => return Err(ObjectError),
            Some(id) => id,
        };
        arg0.generate_server_id(arg0_obj.clone())?;
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
            arg0.server_obj_id.get().unwrap_or(0),
            arg1,
            arg2,
        ]);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the manager object.
    ///
    /// Other objects created via this interface are unaffected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError);
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
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ExtImageCopyCaptureManagerV1] proxies.
#[allow(dead_code)]
pub trait MetaExtImageCopyCaptureManagerV1MessageHandler {
    /// capture an image capture source
    ///
    /// Create a capturing session for an image capture source.
    ///
    /// If the paint_cursors option is set, cursors shall be composited onto
    /// the captured frame. The cursor must not be composited onto the frame
    /// if this flag is not set.
    ///
    /// If the options bitfield is invalid, the invalid_option protocol error
    /// is sent.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `source`:
    /// - `options`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn create_session(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureManagerV1>,
        session: &Rc<MetaExtImageCopyCaptureSessionV1>,
        source: &Rc<MetaExtImageCaptureSourceV1>,
        options: MetaExtImageCopyCaptureManagerV1Options,
    ) {
        let res = _slf.send_create_session(
            session,
            source,
            options,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_manager_v1.create_session message: {}", Report::new(e));
        }
    }

    /// capture the pointer cursor of an image capture source
    ///
    /// Create a cursor capturing session for the pointer of an image capture
    /// source.
    ///
    /// # Arguments
    ///
    /// - `session`:
    /// - `source`:
    /// - `pointer`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn create_pointer_cursor_session(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureManagerV1>,
        session: &Rc<MetaExtImageCopyCaptureCursorSessionV1>,
        source: &Rc<MetaExtImageCaptureSourceV1>,
        pointer: &Rc<MetaWlPointer>,
    ) {
        let res = _slf.send_create_pointer_cursor_session(
            session,
            source,
            pointer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_manager_v1.create_pointer_cursor_session message: {}", Report::new(e));
        }
    }

    /// destroy the manager
    ///
    /// Destroy the manager object.
    ///
    /// Other objects created via this interface are unaffected.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaExtImageCopyCaptureManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a ext_image_copy_capture_manager_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaExtImageCopyCaptureManagerV1 {
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
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaExtImageCopyCaptureSessionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaExtImageCaptureSourceV1>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = MetaExtImageCopyCaptureManagerV1Options(arg2);
                if let Some(handler) = handler {
                    (**handler).create_session(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.create_session(&self, arg0, arg1, arg2);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg0_id = arg0;
                let arg0 = MetaExtImageCopyCaptureCursorSessionV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())?;
                let Some(arg1) = client.endpoint.lookup(arg1) else {
                    return Err(ObjectError);
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaExtImageCaptureSourceV1>() else {
                    return Err(ObjectError);
                };
                let Some(arg2) = client.endpoint.lookup(arg2) else {
                    return Err(ObjectError);
                };
                let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlPointer>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = &arg2;
                if let Some(handler) = handler {
                    (**handler).create_pointer_cursor_session(&self, arg0, arg1, arg2);
                } else {
                    DefaultMessageHandler.create_pointer_cursor_session(&self, arg0, arg1, arg2);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
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

impl MetaExtImageCopyCaptureManagerV1 {
    /// Since when the error.invalid_option enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_OPTION__SINCE: u32 = 1;

    /// Since when the options.paint_cursors enum variant is available.
    #[allow(dead_code)]
    pub const ENM__OPTIONS_PAINT_CURSORS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaExtImageCopyCaptureManagerV1Error(pub u32);

impl MetaExtImageCopyCaptureManagerV1Error {
    /// invalid option flag
    #[allow(dead_code)]
    pub const INVALID_OPTION: Self = Self(1);
}

impl Debug for MetaExtImageCopyCaptureManagerV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_OPTION => "INVALID_OPTION",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
#[allow(dead_code)]
pub struct MetaExtImageCopyCaptureManagerV1Options(pub u32);

/// An iterator over the set bits in a [MetaExtImageCopyCaptureManagerV1Options].
///
/// You can construct this with the `IntoIterator` implementation of `MetaExtImageCopyCaptureManagerV1Options`.
#[derive(Clone, Debug)]
pub struct MetaExtImageCopyCaptureManagerV1OptionsIter(pub u32);

impl MetaExtImageCopyCaptureManagerV1Options {
    /// paint cursors onto captured frames
    #[allow(dead_code)]
    pub const PAINT_CURSORS: Self = Self(1);
}

#[allow(dead_code)]
impl MetaExtImageCopyCaptureManagerV1Options {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 1)
    }
}

impl Iterator for MetaExtImageCopyCaptureManagerV1OptionsIter {
    type Item = MetaExtImageCopyCaptureManagerV1Options;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(MetaExtImageCopyCaptureManagerV1Options(bit))
    }
}

impl IntoIterator for MetaExtImageCopyCaptureManagerV1Options {
    type Item = MetaExtImageCopyCaptureManagerV1Options;
    type IntoIter = MetaExtImageCopyCaptureManagerV1OptionsIter;

    fn into_iter(self) -> Self::IntoIter {
        MetaExtImageCopyCaptureManagerV1OptionsIter(self.0)
    }
}

impl BitAnd for MetaExtImageCopyCaptureManagerV1Options {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for MetaExtImageCopyCaptureManagerV1Options {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for MetaExtImageCopyCaptureManagerV1Options {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for MetaExtImageCopyCaptureManagerV1Options {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for MetaExtImageCopyCaptureManagerV1Options {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for MetaExtImageCopyCaptureManagerV1Options {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for MetaExtImageCopyCaptureManagerV1Options {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for MetaExtImageCopyCaptureManagerV1Options {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for MetaExtImageCopyCaptureManagerV1Options {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for MetaExtImageCopyCaptureManagerV1Options {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("PAINT_CURSORS")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("0")?;
        }
        Ok(())
    }
}
