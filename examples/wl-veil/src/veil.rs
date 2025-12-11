use {
    crate::VeilError,
    std::{
        collections::{HashMap, HashSet},
        process::Command,
        rc::Rc,
        sync::Arc,
    },
    wl_proxy::{
        protocols::wayland::{
            wl_display::{WlDisplay, WlDisplayHandler},
            wl_registry::{WlRegistry, WlRegistryHandler},
        },
        simple::{SimpleCommandExt, SimpleServer},
    },
};

pub fn main(filter: HashMap<String, Option<u32>>, program: Vec<String>) -> Result<(), VeilError> {
    let server = SimpleServer::new().map_err(VeilError::CreateServer)?;
    Command::new(&program[0])
        .args(&program[1..])
        .with_wayland_display(server.display())
        .spawn_and_forward_exit_code()
        .map_err(VeilError::SpawnChild)?;
    let filter = Arc::new(filter);
    let err = server.run(|| DisplayHandler {
        filter: filter.clone(),
    });
    Err(VeilError::ServerFailed(err))
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
        if let Some(&max_version) = self.filter.get(interface) {
            let Some(max_version) = max_version else {
                self.filtered_globals.insert(name);
                return;
            };
            version = version.min(max_version);
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
