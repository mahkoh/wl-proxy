//! Wayland clients connected to the proxy.

use {
    crate::{
        endpoint::Endpoint, handler::HandlerHolder, object::Object,
        protocols::wayland::wl_display::WlDisplay, state::State,
    },
    std::{cell::Cell, rc::Rc},
};

#[cfg(test)]
mod tests;

/// A client connected to the proxy.
///
/// Clients are usually created by having them connect to an
/// [`Acceptor`](crate::acceptor::Acceptor). See [`State::create_acceptor`].
///
/// Clients can also be created manually with [`State::connect`] and
/// [`State::add_client`].
pub struct Client {
    pub(crate) state: Rc<State>,
    pub(crate) endpoint: Rc<Endpoint>,
    pub(crate) display: Rc<WlDisplay>,
    pub(crate) destroyed: Cell<bool>,
    pub(crate) handler: HandlerHolder<dyn ClientHandler>,
}

/// A handler for events emitted by a [`Client`].
pub trait ClientHandler: 'static {
    /// The client disconnected.
    ///
    /// This is not emitted if the client is disconnected with [`Client::disconnect`].
    fn disconnected(self: Box<Self>) {
        // nothing
    }
}

impl Client {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ClientHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ClientHandler>) {
        if self.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }

    /// Unsets the handler.
    pub fn unset_handler(&self) {
        self.handler.set(None);
    }

    /// Returns all objects associated with this client.
    ///
    /// This can be used when a client disconnects to perform cleanup in a multi-client
    /// proxy.
    pub fn objects(&self, objects: &mut Vec<Rc<dyn Object>>) {
        objects.extend(self.endpoint.objects.borrow().values().cloned());
    }

    /// Returns the wl_display object of this client.
    pub fn display(&self) -> &Rc<WlDisplay> {
        &self.display
    }

    /// Disconnects this client.
    ///
    /// The [`ClientHandler::disconnected`] event is not emitted.
    pub fn disconnect(&self) {
        if self.destroyed.replace(true) {
            return;
        }
        let proxies = &mut *self.state.object_stash.borrow();
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
