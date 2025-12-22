//! Baseline protocol support.

#[rustfmt::skip]
mod versions;

use {
    crate::protocols::ObjectInterface,
    linearize::StaticCopyMap,
    std::fmt::{Debug, Formatter},
    versions::*,
};

/// The baseline protocol support.
///
/// This type determines the upper bound for the globals and global versions advertised
/// by a [`State`](crate::state::State). Baselines allow new protocols and new protocol
/// versions to be added to this crate without changing the behavior of applications using
/// the crate.
///
/// For example, if an application turns xdg_toplevel objects into zwlr_layer_surface_v1
/// objects, then the application should filter out globals such as xdg_toplevel_icon_v1
/// that take xdg_toplevels as arguments. Or else it has to also intercept the messages
/// to that global. Without baselines, if a new protocol were added to a new release of
/// this crate, and if that protocol interacted with xdg_toplevels, then updating this
/// crate could cause protocol errors.
///
/// To see the contents of a baseline, look at the source file defining the baseline.
///
/// The difference between two baselines can be seen by diffing the two files containing
/// the baselines.
#[derive(Copy, Clone)]
pub struct Baseline(u32, pub(crate) &'static StaticCopyMap<ObjectInterface, u32>);

impl Debug for Baseline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Baseline::")?;
        if self.0 == !0 {
            f.write_str("ALL_OF_THEM")
        } else {
            f.write_str("V")?;
            self.0.fmt(f)
        }
    }
}

impl Baseline {
    /// Version 0.
    pub const V0: Self = Self(0, v0::BASELINE);

    /// Version 0 (deprecated alias).
    #[deprecated]
    #[doc(hidden)]
    pub const V0_UNSTABLE: Self = Self::V0;

    /// The unreleased baseline.
    ///
    /// This is unstable and can change at any time.
    ///
    /// TODO: When making a new release and this baseline is different from the last stable one:
    ///       - increment this number (N -> N + 1)
    ///       - copy prototyping.rs
    ///       - create Self::VN and Self::VN_UNSTABLE
    ///       - mark Self::VN_UNSTABLE as deprecated
    #[doc(hidden)]
    pub const V1_UNSTABLE: Self = Self(1, prototyping::BASELINE);

    /// This baseline always contains all protocols supported by this crate in their
    /// highest supported version.
    ///
    /// Do not use this unless you are prototyping or in very simple proxies. Use the
    /// highest baseline version available at development time instead and switch to a
    /// higher version when you update your application.
    pub const ALL_OF_THEM: Self = Self(!0, prototyping::BASELINE);
}
