use {
    crate::VeilError,
    error_reporter::Report,
    std::{
        collections::{HashMap, HashSet},
        os::unix::process::ExitStatusExt,
        process::{Command, exit},
        rc::Rc,
        sync::Arc,
        thread,
    },
    uapi::raise,
    wl_proxy::{
        acceptor::Acceptor,
        generated::wayland::{
            wl_display::{WlDisplay, WlDisplayHandler},
            wl_registry::{WlRegistry, WlRegistryHandler},
        },
        state::State,
    },
};

pub fn main(filter: HashMap<String, Option<u32>>, program: Vec<String>) -> Result<(), VeilError> {
    let acceptor = Acceptor::new(1000, false).map_err(VeilError::CreateAcceptor)?;
    let mut child = Command::new(&program[0])
        .args(&program[1..])
        .env("WAYLAND_DISPLAY", &acceptor.display())
        .spawn()
        .map_err(VeilError::SpawnChild)?;
    thread::spawn(move || match child.wait() {
        Ok(e) => {
            if let Some(code) = e.code() {
                exit(code);
            }
            if let Some(signal) = e.signal() {
                let _ = raise(signal);
                exit(1);
            }
            eprintln!("child terminated with neither a signal nor an exit code");
            exit(1);
        }
        Err(e) => {
            eprintln!("could not wait for child: {}", Report::new(e));
            exit(1);
        }
    });
    let filter = Arc::new(filter);
    loop {
        let socket = acceptor
            .accept()
            .map_err(VeilError::AccepConnection)?
            .unwrap();
        let filter = filter.clone();
        thread::spawn(move || {
            let state = match State::new() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("could not create new proxy: {}", Report::new(e));
                    return;
                }
            };
            let client = match state.add_client(socket) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("could not add client: {}", Report::new(e));
                    return;
                }
            };
            client.display.set_handler(DisplayHandler { filter });
            loop {
                if let Err(e) = state.dispatch_blocking() {
                    eprintln!("could not dispatch messages: {}", Report::new(e));
                    return;
                }
            }
        });
    }
}

struct DisplayHandler {
    filter: Arc<HashMap<String, Option<u32>>>,
}

struct RegistryHandler {
    filter: Arc<HashMap<String, Option<u32>>>,
    filtered_globals: HashSet<u32>,
}

impl WlDisplayHandler for DisplayHandler {
    fn get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        registry.set_handler(RegistryHandler {
            filter: self.filter.clone(),
            filtered_globals: Default::default(),
        });
        let _ = slf.send_get_registry(registry);
    }
}

impl WlRegistryHandler for RegistryHandler {
    fn global(&mut self, slf: &Rc<WlRegistry>, name: u32, interface: &str, mut version: u32) {
        if let Some(&f) = self.filter.get(interface) {
            let Some(max) = f else {
                self.filtered_globals.insert(version);
                return;
            };
            version = version.min(max);
        }
        let _ = slf.send_global(name, interface, version);
    }

    fn global_remove(&mut self, slf: &Rc<WlRegistry>, name: u32) {
        if self.filtered_globals.remove(&name) {
            return;
        }
        let _ = slf.send_global_remove(name);
    }
}
