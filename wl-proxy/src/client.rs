use {
    crate::{endpoint::Endpoint, generated::wayland::wl_display::MetaWlDisplay, state::InnerState},
    std::rc::Rc,
};

pub struct Client {
    pub state: Rc<InnerState>,
    pub endpoint: Rc<Endpoint>,
    pub display: Rc<MetaWlDisplay>,
}
