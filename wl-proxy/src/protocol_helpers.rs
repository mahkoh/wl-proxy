use {
    crate::object::{ObjectError, ObjectErrorKind},
    error_reporter::Report,
};

pub(crate) mod prelude {
    #[cfg(feature = "logging")]
    pub(crate) use super::logging::*;
    pub(crate) use {
        super::{NonNullString, NullableString, log_forward, log_send, parse_array, parse_string},
        crate::{
            client::Client,
            endpoint::Endpoint,
            fixed::Fixed,
            handler::{HandlerAccessError, HandlerHolder, HandlerMut, HandlerRef},
            object::{
                ConcreteObject, Object, ObjectCore, ObjectCoreApi, ObjectError, ObjectErrorKind,
                ObjectPrivate, StringError,
            },
            protocols::ObjectInterface,
            state::State,
        },
        std::{
            any::Any,
            cell::RefCell,
            collections::{HashSet, VecDeque},
            fmt::{Debug, Formatter},
            ops::{
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign,
            },
            os::fd::OwnedFd,
            rc::Rc,
        },
    };
}

#[cfg(feature = "logging")]
mod logging {
    pub use std::os::fd::AsRawFd;
    use {debug_fn::debug_fn, std::fmt::Display, uapi::c};

    pub(crate) fn debug_array(array: &[u8]) -> impl Display + use<'_> {
        debug_fn(move |fmt| {
            fmt.write_str("0x")?;
            if array.is_empty() {
                return fmt.write_str("0");
            }
            for b in array {
                write!(fmt, "{:02x}", b)?;
            }
            Ok(())
        })
    }

    #[inline]
    pub(crate) fn time_since_epoch() -> (u32, u16) {
        let mut ts = c::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let _ = uapi::clock_gettime(c::CLOCK_REALTIME, &mut ts);
        let sec = ts.tv_sec as u64;
        let nsec = ts.tv_nsec as u64;
        let time = sec.wrapping_mul(1_000_000).wrapping_add(nsec / 1_000) as u32;
        let millis = time / 1_000;
        let micros = (time % 1_000) as u16;
        (millis, micros)
    }
}

#[cold]
pub(crate) fn log_forward(name: &str, e: &ObjectError) {
    log::warn!("Could not forward a {name} message: {}", Report::new(e));
}

#[cold]
pub(crate) fn log_send(name: &str, e: &ObjectError) {
    log::warn!("Could not send a {name} message: {}", Report::new(e));
}

pub(crate) trait StringType {
    type Type<'a>;
    fn null_string<'a>(
        offset: usize,
        name: &'static str,
    ) -> Result<(Self::Type<'a>, usize), ObjectError>;
    fn wrap(s: &str) -> Self::Type<'_>;
}

pub(crate) struct NonNullString;

impl StringType for NonNullString {
    type Type<'a> = &'a str;

    #[inline(always)]
    fn null_string<'a>(
        _offset: usize,
        name: &'static str,
    ) -> Result<(Self::Type<'a>, usize), ObjectError> {
        Err(ObjectError(ObjectErrorKind::NullString(name)))
    }

    #[inline(always)]
    fn wrap(s: &str) -> Self::Type<'_> {
        s
    }
}

pub(crate) struct NullableString;

impl StringType for NullableString {
    type Type<'a> = Option<&'a str>;

    #[inline(always)]
    fn null_string<'a>(
        offset: usize,
        _name: &'static str,
    ) -> Result<(Self::Type<'a>, usize), ObjectError> {
        Ok((None, offset))
    }

    #[inline(always)]
    fn wrap(s: &str) -> Self::Type<'_> {
        Some(s)
    }
}

#[inline]
pub(crate) fn parse_string<'a, T>(
    msg: &'a [u32],
    mut offset: usize,
    name: &'static str,
) -> Result<(T::Type<'a>, usize), ObjectError>
where
    T: StringType,
{
    let Some(&len) = msg.get(offset) else {
        return Err(ObjectError(ObjectErrorKind::MissingArgument(name)));
    };
    offset += 1;
    let len = len as usize;
    let words = ((len as u64 + 3) / 4) as usize;
    if offset + words > msg.len() {
        return Err(ObjectError(ObjectErrorKind::MissingArgument(name)));
    }
    let start = offset;
    offset += words;
    let bytes = &uapi::as_bytes(&msg[start..])[..len];
    if bytes.is_empty() {
        T::null_string(offset, name)
    } else {
        let Ok(s) = str::from_utf8(&bytes[..len - 1]) else {
            return Err(ObjectError(ObjectErrorKind::NonUtf8(name)));
        };
        Ok((T::wrap(s), offset))
    }
}

#[inline]
pub(crate) fn parse_array<'a>(
    msg: &'a [u32],
    mut offset: usize,
    name: &'static str,
) -> Result<(&'a [u8], usize), ObjectError> {
    let Some(&len) = msg.get(offset) else {
        return Err(ObjectError(ObjectErrorKind::MissingArgument(name)));
    };
    offset += 1;
    let len = len as usize;
    let words = ((len as u64 + 3) / 4) as usize;
    if offset + words > msg.len() {
        return Err(ObjectError(ObjectErrorKind::MissingArgument(name)));
    }
    let start = offset;
    offset += words;
    let array = &uapi::as_bytes(&msg[start..])[..len];
    Ok((array, offset))
}
