use {
    crate::VeilError,
    linearize::StaticMap,
    std::{
        collections::{HashMap, HashSet},
        process::Command,
        rc::Rc,
        sync::Arc,
    },
    wl_proxy::{
        baseline::Baseline,
        protocols::{
            ObjectInterface,
            wayland::{
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_registry::{WlRegistry, WlRegistryHandler},
            },
        },
        simple::{SimpleCommandExt, SimpleProxy},
    },
};

pub fn main(
    invert: bool,
    filter: HashMap<String, Option<u32>>,
    program: Vec<String>,
) -> Result<(), VeilError> {
    let server = SimpleProxy::new(Baseline::ALL_OF_THEM).map_err(VeilError::CreateServer)?;
    Command::new(&program[0])
        .args(&program[1..])
        .with_wayland_display(server.display())
        .spawn_and_forward_exit_code()
        .map_err(VeilError::SpawnChild)?;
    let filter = create_filter(invert, filter);
    let err = server.run(|| WlDisplayHandlerImpl {
        filter: filter.clone(),
    });
    Err(VeilError::ServerFailed(err))
}

#[derive(Copy, Clone)]
enum Disposition {
    Forward,
    Hide,
    ReduceVersion(u32),
}

fn create_filter(
    invert: bool,
    filter: HashMap<String, Option<u32>>,
) -> Arc<StaticMap<ObjectInterface, Disposition>> {
    let default = match invert {
        true => Disposition::Hide,
        false => Disposition::Forward,
    };
    let mut res = StaticMap::from_fn(|_| default);
    for (interface, max_version) in filter {
        let Some(interface) = ObjectInterface::from_str(&interface) else {
            eprintln!("Unknown interface {}", interface);
            continue;
        };
        res[interface] = match max_version {
            None if invert => Disposition::Forward,
            None => Disposition::Hide,
            Some(v) => Disposition::ReduceVersion(v),
        };
    }
    Arc::new(res)
}

struct WlDisplayHandlerImpl {
    filter: Arc<StaticMap<ObjectInterface, Disposition>>,
}

impl WlDisplayHandler for WlDisplayHandlerImpl {
    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        registry.set_handler(WlRegistryHandlerImpl {
            filter: self.filter.clone(),
            filtered_globals: Default::default(),
        });
        slf.send_get_registry(registry);
    }
}

struct WlRegistryHandlerImpl {
    filter: Arc<StaticMap<ObjectInterface, Disposition>>,
    filtered_globals: HashSet<u32>,
}

impl WlRegistryHandler for WlRegistryHandlerImpl {
    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        match self.filter[interface] {
            Disposition::Forward => {
                slf.send_global(name, interface, version);
            }
            Disposition::Hide => {
                self.filtered_globals.insert(name);
            }
            Disposition::ReduceVersion(max) => {
                slf.send_global(name, interface, version.min(max));
            }
        }
    }

    fn handle_global_remove(&mut self, slf: &Rc<WlRegistry>, name: u32) {
        if self.filtered_globals.remove(&name) {
            return;
        }
        slf.send_global_remove(name);
    }
}
