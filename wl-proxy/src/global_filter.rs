mod prototyping;

use {
    crate::{
        object::Object,
        object_error::ObjectError,
        protocols::{ObjectInterface, wayland::wl_registry::WlRegistry},
    },
    linearize::StaticCopyMap,
    std::{collections::HashMap, rc::Rc},
};

/// A filter for globals advertised via wl_registry.
///
/// This type allows filtering globals sent by the server and advertising synthetic
/// globals that are handled by the proxy.
///
/// Unless your application is very simple, you likely want to use this type even if you
/// don't
pub struct GlobalFilter {
    baseline: &'static StaticCopyMap<ObjectInterface, u32>,
    server_to_client: HashMap<u32, Option<u32>>,
    client_to_server: Vec<Option<u32>>,
}

impl GlobalFilter {
    fn from_baseline(baseline: &'static StaticCopyMap<ObjectInterface, u32>) -> Self {
        Self {
            baseline,
            server_to_client: Default::default(),
            client_to_server: Default::default(),
        }
    }

    pub fn add_synthetic_global(
        &mut self,
        registry: &WlRegistry,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<u32, ObjectError> {
        let name = self.client_to_server.len() as u32;
        self.client_to_server.push(None);
        registry.send_global(name, interface, version)?;
        Ok(name)
    }

    pub fn remove_synthetic_global(
        &mut self,
        registry: &WlRegistry,
        name: u32,
    ) -> Result<(), ObjectError> {
        registry.send_global_remove(name)
    }

    pub fn handle_server_global(
        &mut self,
        registry: &WlRegistry,
        server_name: u32,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<(), ObjectError> {
        let max_version = self.baseline[interface];
        if max_version == 0 {
            self.ignore_server_global(server_name);
            return Ok(());
        }
        let client_name = self.client_to_server.len() as u32;
        self.client_to_server.push(Some(server_name));
        self.server_to_client.insert(server_name, Some(client_name));
        registry.send_global(client_name, interface, version.min(max_version))
    }

    pub fn ignore_server_global(&mut self, name: u32) {
        self.server_to_client.insert(name, None);
    }

    pub fn handle_server_global_remove(
        &mut self,
        registry: &WlRegistry,
        server_name: u32,
    ) -> Result<(), ObjectError> {
        let Some(client_name) = self.server_to_client.remove(&server_name) else {
            log::warn!(
                "Cerver sent wl_registry.global_remove for name {server_name} but no such global exists"
            );
            return Ok(());
        };
        let Some(client_name) = client_name else {
            return Ok(());
        };
        registry.send_global_remove(client_name)
    }

    pub fn handle_client_bind(
        &mut self,
        registry: &WlRegistry,
        client_name: u32,
        proxy: &Rc<dyn Object>,
    ) -> Result<(), ObjectError> {
        let Some(server_name) = self.client_to_server.get(client_name as usize) else {
            log::warn!(
                "Client sent wl_registry.bind for name {client_name} but not such global exists"
            );
            return Ok(());
        };
        let Some(server_name) = server_name else {
            return Ok(());
        };
        registry.send_bind(*server_name, proxy.clone())
    }
}

impl GlobalFilter {
    pub fn baseline_i_am_prototyping() -> Self {
        Self::from_baseline(prototyping::BASELINE)
    }
}
