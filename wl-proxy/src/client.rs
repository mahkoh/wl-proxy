use {
    crate::{
        endpoint::Endpoint, protocols::wayland::wl_display::WlDisplay, state::State,
        utils::handler_holder::HandlerHolder,
    },
    std::{cell::Cell, rc::Rc},
};

pub struct Client {
    pub(crate) state: Rc<State>,
    pub(crate) endpoint: Rc<Endpoint>,
    pub(crate) display: Rc<WlDisplay>,
    pub(crate) destroyed: Cell<bool>,
    pub(crate) handler: HandlerHolder<dyn ClientHandler>,
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
        let proxies = &mut *self.state.proxy_stash.borrow();
        proxies.extend(self.endpoint.objects.borrow().values().cloned());
        for object in proxies {
            object.unset_handler();
        }
    }

    pub fn display(&self) -> &Rc<WlDisplay> {
        &self.display
    }

    pub fn kill(&self) {
        if self.destroyed.replace(true) {
            return;
        }
        let proxies = &mut *self.state.proxy_stash.borrow();
        for (_, object) in self.endpoint.objects.borrow_mut().drain() {
            let core = object.core();
            core.client.take();
            core.client_id.take();
            core.client_obj_id.take();
            proxies.push(object);
        }
        self.handler.set(None);
        self.state.remove_endpoint(&self.endpoint);
    }
}
