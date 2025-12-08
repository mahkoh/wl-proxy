pub mod prelude {
    pub use {
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
            os::fd::OwnedFd,
            rc::Rc,
        },
    };
}
