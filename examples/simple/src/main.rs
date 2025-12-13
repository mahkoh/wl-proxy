use {
    std::rc::Rc,
    wl_proxy::{
        baselines::Baseline,
        client::{Client, ClientHandler},
        protocols::wayland::{
            wl_display::{WlDisplay, WlDisplayHandler},
            wl_registry::WlRegistry,
        },
        state::{StateBuilder, StateHandler},
    },
};

fn main() {
    let state = StateBuilder::new(Baseline::ALL_OF_THEM).build().unwrap();
    state.set_handler(StateHandlerImpl);
    let acceptor = state.create_acceptor(1000).unwrap();
    eprintln!("{}", acceptor.display());
    loop {
        state.dispatch_blocking().unwrap();
    }
}

struct StateHandlerImpl;

impl StateHandler for StateHandlerImpl {
    fn new_client(&mut self, client: &Rc<Client>) {
        eprintln!("client connected");
        client.set_handler(ClientHandlerImpl);
        client.display().set_handler(DisplayHandler);
    }
}

struct ClientHandlerImpl;

impl ClientHandler for ClientHandlerImpl {
    fn disconnected(self: Box<Self>) {
        eprintln!("client disconnected");
    }
}

struct DisplayHandler;

impl WlDisplayHandler for DisplayHandler {
    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        eprintln!("get_registry called");
        let _ = slf.send_get_registry(registry);
    }
}
