use {
    crate::{endpoint::Endpoint, generated::wayland::wl_display::MetaWlDisplay},
    std::rc::Rc,
};

pub struct Client {
    pub endpoint: Rc<Endpoint>,
    pub display: Rc<MetaWlDisplay>,
}
