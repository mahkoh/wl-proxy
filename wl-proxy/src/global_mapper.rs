use {
    crate::{
        object::{Object, ObjectError},
        protocols::{ObjectInterface, wayland::wl_registry::WlRegistry},
    },
    std::{collections::HashMap, rc::Rc},
};

/// A filter for globals advertised via wl_registry.
///
/// This type allows filtering globals sent by the server and advertising synthetic
/// globals that are handled by the proxy.
pub struct GlobalMapper {
    server_to_client: HashMap<u32, Option<u32>>,
    client_to_server: Vec<Option<u32>>,
}

impl Default for GlobalMapper {
    fn default() -> Self {
        let mut server_to_client = HashMap::new();
        server_to_client.insert(0, None);
        Self {
            server_to_client,
            client_to_server: vec![None],
        }
    }
}

impl GlobalMapper {
    pub fn add_synthetic_global(
        &mut self,
        registry: &WlRegistry,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<u32, ObjectError> {
        let name = self.client_to_server.len() as u32;
        self.client_to_server.push(None);
        registry.try_send_global(name, interface, version)?;
        Ok(name)
    }

    pub fn remove_synthetic_global(
        &mut self,
        registry: &WlRegistry,
        name: u32,
    ) -> Result<(), ObjectError> {
        registry.try_send_global_remove(name)
    }

    pub fn handle_server_global(
        &mut self,
        registry: &WlRegistry,
        server_name: u32,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<(), ObjectError> {
        let client_name = self.client_to_server.len() as u32;
        self.client_to_server.push(Some(server_name));
        self.server_to_client.insert(server_name, Some(client_name));
        registry.try_send_global(client_name, interface, version)
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
        registry.try_send_global_remove(client_name)
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
        registry.try_send_bind(*server_name, proxy.clone())
    }
}
