use {debug_fn::debug_fn, std::fmt::Display};

pub mod prelude {
    pub use {
        super::debug_array,
        crate::{
            client::Client,
            fixed::Fixed,
            generated::ProxyInterface,
            object_error::ObjectError,
            proxy::{MessageHandlerHolder, Proxy, ProxyCore},
            state::InnerState,
        },
        error_reporter::Report,
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

pub fn debug_array(array: &[u8]) -> impl Display + use<'_> {
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
