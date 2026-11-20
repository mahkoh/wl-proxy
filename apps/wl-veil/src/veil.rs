use {
    crate::VeilError,
    linearize::StaticMap,
    std::{collections::HashMap, process::Command, rc::Rc, sync::Arc},
    wl_proxy::{
        baseline::Baseline,
        object::{Object, ObjectCoreApi},
        protocols::{
            ObjectInterface,
            wayland::{
                wl_display::{WlDisplay, WlDisplayError, WlDisplayHandler},
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
            announced: Default::default(),
        });
        slf.send_get_registry(registry);
    }
}

struct WlRegistryHandlerImpl {
    filter: Arc<StaticMap<ObjectInterface, Disposition>>,
    announced: HashMap<u32, u32>,
}

impl WlRegistryHandler for WlRegistryHandlerImpl {
    fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        let interface = id.interface().name();
        let message = match self.announced.get(&name) {
            None => format!("invalid global {} ({})", interface, name),
            Some(&version) if id.version() > version => format!(
                "invalid version for global {} ({}): have {}, wanted {}",
                interface,
                name,
                version,
                id.version(),
            ),
            Some(_) => {
                slf.send_bind(name, id);
                return;
            }
        };
        if let Some(client) = slf.client() {
            let error = WlDisplayError::INVALID_OBJECT.0;
            client.display().send_error(slf.clone(), error, &message);
        }
    }

    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        match self.filter[interface] {
            Disposition::Forward => {
                self.announced.insert(name, version);
                slf.send_global(name, interface, version);
            }
            Disposition::Hide => {}
            Disposition::ReduceVersion(max) => {
                self.announced.insert(name, version.min(max));
                slf.send_global(name, interface, version.min(max));
            }
        }
    }

    fn handle_global_remove(&mut self, slf: &Rc<WlRegistry>, name: u32) {
        if self.announced.contains_key(&name) {
            slf.send_global_remove(name);
        }
    }
}
