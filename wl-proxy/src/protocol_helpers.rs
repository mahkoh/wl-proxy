use {
    crate::object::ObjectError, debug_fn::debug_fn, error_reporter::Report, std::fmt::Display,
    uapi::c,
};

pub(crate) mod prelude {
    pub(crate) use {
        super::{debug_array, log_forward, log_send, time_since_epoch},
        crate::{
            client::Client,
            fixed::Fixed,
            object::{
                ConcreteObject, HandlerAccessError, Object, ObjectCore, ObjectError, ObjectPrivate,
                StringError,
            },
            protocols::ObjectInterface,
            state::State,
            utils::handler_holder::{HandlerHolder, HandlerMut, HandlerRef},
        },
        std::{
            any::Any,
            collections::VecDeque,
            fmt::{Debug, Formatter},
            ops::{
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign,
            },
            os::fd::{AsRawFd, OwnedFd},
            rc::Rc,
        },
    };
}

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

#[cold]
pub(crate) fn log_forward(name: &str, e: &ObjectError) {
    log::warn!("Could not forward a {name} message: {}", Report::new(e));
}

#[cold]
pub(crate) fn log_send(name: &str, e: &ObjectError) {
    log::warn!("Could not send a {name} message: {}", Report::new(e));
}
