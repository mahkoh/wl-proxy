pub mod prelude {
    pub use crate::client::Client;
    pub use crate::fixed::Fixed;
    pub use crate::object_error::ObjectError;
    pub use crate::proxy::MessageHandlerHolder;
    pub use crate::proxy::Proxy;
    pub use crate::proxy::ProxyCore;
    pub use crate::state::InnerState;
    pub use error_reporter::Report;
    pub use std::any::Any;
    pub use std::collections::VecDeque;
    pub use std::fmt::{Debug, Formatter};
    pub use std::ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign,
    };
    pub use std::os::fd::OwnedFd;
    pub use std::rc::Rc;
}
