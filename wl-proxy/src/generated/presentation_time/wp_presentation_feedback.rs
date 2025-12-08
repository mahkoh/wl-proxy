//! presentation time feedback event
//!
//! A presentation_feedback object returns an indication that a
//! wl_surface content update has become visible to the user.
//! One object corresponds to one content update submission
//! (wl_surface.commit). There are two possible outcomes: the
//! content update is presented to the user, and a presentation
//! timestamp delivered; or, the user did not see the content
//! update because it was superseded or its surface destroyed,
//! and the content update is discarded.
//!
//! Once a presentation_feedback object has delivered a 'presented'
//! or 'discarded' event it is automatically destroyed.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A wp_presentation_feedback proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaWpPresentationFeedback {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaWpPresentationFeedbackMessageHandler>,
}

struct DefaultMessageHandler;

impl MetaWpPresentationFeedbackMessageHandler for DefaultMessageHandler { }

impl MetaWpPresentationFeedback {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaWpPresentationFeedback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaWpPresentationFeedback")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaWpPresentationFeedback {
    /// Since when the sync_output message is available.
    #[allow(dead_code)]
    pub const MSG__SYNC_OUTPUT__SINCE: u32 = 1;

    /// presentation synchronized to this output
    ///
    /// As presentation can be synchronized to only one output at a
    /// time, this event tells which output it was. This event is only
    /// sent prior to the presented event.
    ///
    /// As clients may bind to the same global wl_output multiple
    /// times, this event is sent for each bound instance that matches
    /// the synchronized output. If a client has not bound to the
    /// right wl_output global at all, this event is not sent.
    ///
    /// # Arguments
    ///
    /// - `output`: presentation output
    #[inline]
    pub fn send_sync_output(
        &self,
        output: &Rc<MetaWlOutput>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            output,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        if arg0.client_id.get() != Some(client.id) {
            return Err(ObjectError);
        }
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            0,
            arg0.client_obj_id.get().unwrap_or(0),
        ]);
        Ok(())
    }

    /// Since when the presented message is available.
    #[allow(dead_code)]
    pub const MSG__PRESENTED__SINCE: u32 = 1;

    /// the content update was displayed
    ///
    /// The associated content update was displayed to the user at the
    /// indicated time (tv_sec_hi/lo, tv_nsec). For the interpretation of
    /// the timestamp, see presentation.clock_id event.
    ///
    /// The timestamp corresponds to the time when the content update
    /// turned into light the first time on the surface's main output.
    /// Compositors may approximate this from the framebuffer flip
    /// completion events from the system, and the latency of the
    /// physical display path if known.
    ///
    /// This event is preceded by all related sync_output events
    /// telling which output's refresh cycle the feedback corresponds
    /// to, i.e. the main output for the surface. Compositors are
    /// recommended to choose the output containing the largest part
    /// of the wl_surface, or keeping the output they previously
    /// chose. Having a stable presentation output association helps
    /// clients predict future output refreshes (vblank).
    ///
    /// The 'refresh' argument gives the compositor's prediction of how
    /// many nanoseconds after tv_sec, tv_nsec the very next output
    /// refresh may occur. This is to further aid clients in
    /// predicting future refreshes, i.e., estimating the timestamps
    /// targeting the next few vblanks. If such prediction cannot
    /// usefully be done, the argument is zero.
    ///
    /// For version 2 and later, if the output does not have a constant
    /// refresh rate, explicit video mode switches excluded, then the
    /// refresh argument must be either an appropriate rate picked by the
    /// compositor (e.g. fastest rate), or 0 if no such rate exists.
    /// For version 1, if the output does not have a constant refresh rate,
    /// the refresh argument must be zero.
    ///
    /// The 64-bit value combined from seq_hi and seq_lo is the value
    /// of the output's vertical retrace counter when the content
    /// update was first scanned out to the display. This value must
    /// be compatible with the definition of MSC in
    /// GLX_OML_sync_control specification. Note, that if the display
    /// path has a non-zero latency, the time instant specified by
    /// this counter may differ from the timestamp's.
    ///
    /// If the output does not have a concept of vertical retrace or a
    /// refresh cycle, or the output device is self-refreshing without
    /// a way to query the refresh count, then the arguments seq_hi
    /// and seq_lo must be zero.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the presentation timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the presentation timestamp
    /// - `tv_nsec`: nanoseconds part of the presentation timestamp
    /// - `refresh`: nanoseconds till next refresh
    /// - `seq_hi`: high 32 bits of refresh counter
    /// - `seq_lo`: low 32 bits of refresh counter
    /// - `flags`: combination of 'kind' values
    #[inline]
    pub fn send_presented(
        &self,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        refresh: u32,
        seq_hi: u32,
        seq_lo: u32,
        flags: MetaWpPresentationFeedbackKind,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
        ) = (
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            refresh,
            seq_hi,
            seq_lo,
            flags,
        );
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            1,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6.0,
        ]);
        Ok(())
    }

    /// Since when the discarded message is available.
    #[allow(dead_code)]
    pub const MSG__DISCARDED__SINCE: u32 = 1;

    /// the content update was not displayed
    ///
    /// The content update was never displayed to the user.
    #[inline]
    pub fn send_discarded(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client = core.client.borrow();
        let Some(client) = &*client else {
            return Err(ObjectError);
        };
        let outgoing = &mut *client.outgoing.borrow_mut();
        let mut fmt = outgoing.formatter();
        fmt.words([
            core.client_obj_id.get().unwrap_or(0),
            2,
        ]);
        Ok(())
    }
}

/// A message handler for [WpPresentationFeedback] proxies.
#[allow(dead_code)]
pub trait MetaWpPresentationFeedbackMessageHandler {
    /// presentation synchronized to this output
    ///
    /// As presentation can be synchronized to only one output at a
    /// time, this event tells which output it was. This event is only
    /// sent prior to the presented event.
    ///
    /// As clients may bind to the same global wl_output multiple
    /// times, this event is sent for each bound instance that matches
    /// the synchronized output. If a client has not bound to the
    /// right wl_output global at all, this event is not sent.
    ///
    /// # Arguments
    ///
    /// - `output`: presentation output
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn sync_output(
        &mut self,
        _slf: &Rc<MetaWpPresentationFeedback>,
        output: &Rc<MetaWlOutput>,
    ) {
        if let Some(client_id) = _slf.core.client_id.get() {
            if let Some(client_id_2) = output.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = _slf.send_sync_output(
            output,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_presentation_feedback.sync_output message: {}", Report::new(e));
        }
    }

    /// the content update was displayed
    ///
    /// The associated content update was displayed to the user at the
    /// indicated time (tv_sec_hi/lo, tv_nsec). For the interpretation of
    /// the timestamp, see presentation.clock_id event.
    ///
    /// The timestamp corresponds to the time when the content update
    /// turned into light the first time on the surface's main output.
    /// Compositors may approximate this from the framebuffer flip
    /// completion events from the system, and the latency of the
    /// physical display path if known.
    ///
    /// This event is preceded by all related sync_output events
    /// telling which output's refresh cycle the feedback corresponds
    /// to, i.e. the main output for the surface. Compositors are
    /// recommended to choose the output containing the largest part
    /// of the wl_surface, or keeping the output they previously
    /// chose. Having a stable presentation output association helps
    /// clients predict future output refreshes (vblank).
    ///
    /// The 'refresh' argument gives the compositor's prediction of how
    /// many nanoseconds after tv_sec, tv_nsec the very next output
    /// refresh may occur. This is to further aid clients in
    /// predicting future refreshes, i.e., estimating the timestamps
    /// targeting the next few vblanks. If such prediction cannot
    /// usefully be done, the argument is zero.
    ///
    /// For version 2 and later, if the output does not have a constant
    /// refresh rate, explicit video mode switches excluded, then the
    /// refresh argument must be either an appropriate rate picked by the
    /// compositor (e.g. fastest rate), or 0 if no such rate exists.
    /// For version 1, if the output does not have a constant refresh rate,
    /// the refresh argument must be zero.
    ///
    /// The 64-bit value combined from seq_hi and seq_lo is the value
    /// of the output's vertical retrace counter when the content
    /// update was first scanned out to the display. This value must
    /// be compatible with the definition of MSC in
    /// GLX_OML_sync_control specification. Note, that if the display
    /// path has a non-zero latency, the time instant specified by
    /// this counter may differ from the timestamp's.
    ///
    /// If the output does not have a concept of vertical retrace or a
    /// refresh cycle, or the output device is self-refreshing without
    /// a way to query the refresh count, then the arguments seq_hi
    /// and seq_lo must be zero.
    ///
    /// # Arguments
    ///
    /// - `tv_sec_hi`: high 32 bits of the seconds part of the presentation timestamp
    /// - `tv_sec_lo`: low 32 bits of the seconds part of the presentation timestamp
    /// - `tv_nsec`: nanoseconds part of the presentation timestamp
    /// - `refresh`: nanoseconds till next refresh
    /// - `seq_hi`: high 32 bits of refresh counter
    /// - `seq_lo`: low 32 bits of refresh counter
    /// - `flags`: combination of 'kind' values
    #[inline]
    fn presented(
        &mut self,
        _slf: &Rc<MetaWpPresentationFeedback>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        refresh: u32,
        seq_hi: u32,
        seq_lo: u32,
        flags: MetaWpPresentationFeedbackKind,
    ) {
        let res = _slf.send_presented(
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            refresh,
            seq_hi,
            seq_lo,
            flags,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_presentation_feedback.presented message: {}", Report::new(e));
        }
    }

    /// the content update was not displayed
    ///
    /// The content update was never displayed to the user.
    #[inline]
    fn discarded(
        &mut self,
        _slf: &Rc<MetaWpPresentationFeedback>,
    ) {
        let res = _slf.send_discarded(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a wp_presentation_feedback.discarded message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaWpPresentationFeedback {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            _ => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let Some(arg0) = self.core.state.lookup(arg0) else {
                    return Err(ObjectError);
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                    return Err(ObjectError);
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).sync_output(&self, arg0);
                } else {
                    DefaultMessageHandler.sync_output(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                    arg2,
                    arg3,
                    arg4,
                    arg5,
                    arg6,
                ] = msg[2..] else {
                    return Err(ObjectError);
                };
                let arg6 = MetaWpPresentationFeedbackKind(arg6);
                if let Some(handler) = handler {
                    (**handler).presented(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                } else {
                    DefaultMessageHandler.presented(&self, arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                }
            }
            2 => {
                if let Some(handler) = handler {
                    (**handler).discarded(&self);
                } else {
                    DefaultMessageHandler.discarded(&self);
                }
            }
            _ => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError);
            }
        }
        Ok(())
    }
}

impl MetaWpPresentationFeedback {
    /// Since when the kind.vsync enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KIND_VSYNC__SINCE: u32 = 1;
    /// Since when the kind.hw_clock enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KIND_HW_CLOCK__SINCE: u32 = 1;
    /// Since when the kind.hw_completion enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KIND_HW_COMPLETION__SINCE: u32 = 1;
    /// Since when the kind.zero_copy enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KIND_ZERO_COPY__SINCE: u32 = 1;
}

/// bitmask of flags in presented event
///
/// These flags provide information about how the presentation of
/// the related content update was done. The intent is to help
/// clients assess the reliability of the feedback and the visual
/// quality with respect to possible tearing and timings.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Default)]
#[allow(dead_code)]
pub struct MetaWpPresentationFeedbackKind(pub u32);

/// An iterator over the set bits in a [MetaWpPresentationFeedbackKind].
///
/// You can construct this with the `IntoIterator` implementation of `MetaWpPresentationFeedbackKind`.
#[derive(Clone, Debug)]
pub struct MetaWpPresentationFeedbackKindIter(pub u32);

impl MetaWpPresentationFeedbackKind {
    /// presentation was vsync'd
    ///
    /// The presentation was synchronized to the "vertical retrace" by
    /// the display hardware such that tearing does not happen.
    /// Relying on software scheduling is not acceptable for this
    /// flag. If presentation is done by a copy to the active
    /// frontbuffer, then it must guarantee that tearing cannot
    /// happen.
    #[allow(dead_code)]
    pub const VSYNC: Self = Self(0x1);

    /// hardware provided the presentation timestamp
    ///
    /// The display hardware provided measurements that the hardware
    /// driver converted into a presentation timestamp. Sampling a
    /// clock in software is not acceptable for this flag.
    #[allow(dead_code)]
    pub const HW_CLOCK: Self = Self(0x2);

    /// hardware signalled the start of the presentation
    ///
    /// The display hardware signalled that it started using the new
    /// image content. The opposite of this is e.g. a timer being used
    /// to guess when the display hardware has switched to the new
    /// image content.
    #[allow(dead_code)]
    pub const HW_COMPLETION: Self = Self(0x4);

    /// presentation was done zero-copy
    ///
    /// The presentation of this update was done zero-copy. This means
    /// the buffer from the client was given to display hardware as
    /// is, without copying it. Compositing with OpenGL counts as
    /// copying, even if textured directly from the client buffer.
    /// Possible zero-copy cases include direct scanout of a
    /// fullscreen surface and a surface on a hardware overlay.
    #[allow(dead_code)]
    pub const ZERO_COPY: Self = Self(0x8);
}

#[allow(dead_code)]
impl MetaWpPresentationFeedbackKind {
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
        Self(0 | 0x1 | 0x2 | 0x4 | 0x8)
    }
}

impl Iterator for MetaWpPresentationFeedbackKindIter {
    type Item = MetaWpPresentationFeedbackKind;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(MetaWpPresentationFeedbackKind(bit))
    }
}

impl IntoIterator for MetaWpPresentationFeedbackKind {
    type Item = MetaWpPresentationFeedbackKind;
    type IntoIter = MetaWpPresentationFeedbackKindIter;

    fn into_iter(self) -> Self::IntoIter {
        MetaWpPresentationFeedbackKindIter(self.0)
    }
}

impl BitAnd for MetaWpPresentationFeedbackKind {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for MetaWpPresentationFeedbackKind {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for MetaWpPresentationFeedbackKind {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for MetaWpPresentationFeedbackKind {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for MetaWpPresentationFeedbackKind {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for MetaWpPresentationFeedbackKind {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for MetaWpPresentationFeedbackKind {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for MetaWpPresentationFeedbackKind {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for MetaWpPresentationFeedbackKind {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for MetaWpPresentationFeedbackKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 0x1 == 0x1 {
            v &= !0x1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("VSYNC")?;
        }
        if v & 0x2 == 0x2 {
            v &= !0x2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("HW_CLOCK")?;
        }
        if v & 0x4 == 0x4 {
            v &= !0x4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("HW_COMPLETION")?;
        }
        if v & 0x8 == 0x8 {
            v &= !0x8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("ZERO_COPY")?;
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
