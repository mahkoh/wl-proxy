use {
    crate::{
        endpoint::Endpoint, generated::wayland::wl_display::WlDisplay, proxy::HandlerHolder,
        state::State,
    },
    std::{cell::Cell, rc::Rc},
};

pub struct Client {
    pub state: Rc<State>,
    pub endpoint: Rc<Endpoint>,
    pub display: Rc<WlDisplay>,
    pub destroyed: Cell<bool>,
    pub handler: HandlerHolder<dyn ClientHandler>,
}

pub trait ClientHandler {
    fn disconnected(self: Box<Self>);
}

impl Client {
    pub fn set_handler(&self, handler: impl ClientHandler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn ClientHandler>) {
        if self.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }

    pub fn unset_handler(&self) {
        self.handler.set(None);
    }

    pub fn unset_proxy_handlers(&self) {
        for object in self.endpoint.objects.borrow_mut().values() {
            object.unset_handler();
        }
    }

    pub fn kill(&self) {
        if self.destroyed.replace(true) {
            return;
        }
        for object in self.endpoint.objects.borrow_mut().values() {
            let core = object.core();
            core.client.take();
            core.client_id.take();
            core.client_obj_id.take();
        }
        self.endpoint.objects.borrow_mut().clear();
        self.handler.set(None);
        self.state.pollables.borrow_mut().remove(&self.endpoint.id);
    }
}
