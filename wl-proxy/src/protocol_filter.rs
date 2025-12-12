mod prototyping;

use {
    crate::{
        object_error::ObjectError,
        protocols::{ProxyInterface, wayland::wl_registry::WlRegistry},
        proxy::Proxy,
    },
    linearize::StaticCopyMap,
    std::{collections::HashMap, rc::Rc},
};

pub struct ProtocolFilter {
    baseline: &'static StaticCopyMap<ProxyInterface, u32>,
    server_to_client: HashMap<u32, Option<u32>>,
    client_to_server: Vec<Option<u32>>,
}

impl ProtocolFilter {
    fn from_baseline(baseline: &'static StaticCopyMap<ProxyInterface, u32>) -> Self {
        Self {
            baseline,
            server_to_client: Default::default(),
            client_to_server: Default::default(),
        }
    }

    pub fn add_global(
        &mut self,
        registry: &WlRegistry,
        interface: ProxyInterface,
        version: u32,
    ) -> Result<u32, ObjectError> {
        let name = self.client_to_server.len() as u32;
        self.client_to_server.push(None);
        registry.send_global(name, interface.name(), version)?;
        Ok(name)
    }

    pub fn remove_global(&mut self, registry: &WlRegistry, name: u32) -> Result<(), ObjectError> {
        registry.send_global_remove(name)
    }

    pub fn handle_global(
        &mut self,
        registry: &WlRegistry,
        name: u32,
        interface: ProxyInterface,
        version: u32,
    ) -> Result<(), ObjectError> {
        let max_version = self.baseline[interface];
        if max_version == 0 {
            self.ignore_global(name);
            return Ok(());
        }
        let client_name = self.client_to_server.len() as u32;
        self.client_to_server.push(Some(name));
        self.server_to_client.insert(name, Some(client_name));
        registry.send_global(client_name, interface.name(), version.min(max_version))
    }

    pub fn ignore_global(&mut self, name: u32) {
        self.server_to_client.insert(name, None);
    }

    pub fn handle_global_remove(
        &mut self,
        registry: &WlRegistry,
        name: u32,
    ) -> Result<(), ObjectError> {
        let Some(client_name) = self.server_to_client.remove(&name) else {
            log::warn!(
                "server sent wl_registry.global_remove for name {name} but no such global exists"
            );
            return Ok(());
        };
        let Some(client_name) = client_name else {
            return Ok(());
        };
        registry.send_global_remove(client_name)
    }

    pub fn handle_bind(
        &mut self,
        registry: &WlRegistry,
        name: u32,
        proxy: &Rc<dyn Proxy>,
    ) -> Result<(), ObjectError> {
        let Some(server_name) = self.client_to_server.get(name as usize) else {
            log::warn!("client sent wl_registry.bind for name {name} but not such global exists");
            return Ok(());
        };
        let Some(server_name) = server_name else {
            return Ok(());
        };
        registry.send_bind(*server_name, proxy.clone())
    }
}

impl ProtocolFilter {
    pub fn baseline_i_am_prototyping() -> Self {
        Self::from_baseline(prototyping::BASELINE)
    }
}
