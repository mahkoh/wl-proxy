//! A filter for globals advertised via wl_registry.

use {
    crate::{
        object::{Object, ObjectError},
        protocols::{ObjectInterface, wayland::wl_registry::WlRegistry},
    },
    error_reporter::Report,
    std::{collections::HashMap, rc::Rc},
};

#[cfg(test)]
mod tests;

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

trait RegistryApi {
    fn bind(&self, name: u32, id: Rc<dyn Object>) -> Result<(), ObjectError>;
    fn global(
        &self,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<(), ObjectError>;
    fn global_remove(&self, name: u32) -> Result<(), ObjectError>;
}

impl RegistryApi for WlRegistry {
    fn bind(&self, name: u32, id: Rc<dyn Object>) -> Result<(), ObjectError> {
        self.try_send_bind(name, id)
    }

    fn global(
        &self,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<(), ObjectError> {
        self.try_send_global(name, interface, version)
    }

    fn global_remove(&self, name: u32) -> Result<(), ObjectError> {
        self.try_send_global_remove(name)
    }
}

impl GlobalMapper {
    /// Announces a synthetic global and returns the global name.
    ///
    /// This function is similar to [`GlobalMapper::try_add_synthetic_global`] but logs
    /// a message instead of returning an error if the global could not be sent to the
    /// client.
    pub fn add_synthetic_global(
        &mut self,
        registry: &WlRegistry,
        interface: ObjectInterface,
        version: u32,
    ) -> u32 {
        self.add_synthetic_global_impl(registry, interface, version)
    }

    /// Tries to announce a synthetic global and returns the global name.
    pub fn try_add_synthetic_global(
        &mut self,
        registry: &WlRegistry,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<u32, ObjectError> {
        self.try_add_synthetic_global_impl(registry, interface, version)
    }

    /// Removes a synthetic global.
    ///
    /// This function is similar to [`GlobalMapper::try_remove_synthetic_global`] but logs
    /// a message instead of returning an error if the global_remove event could not be
    /// sent to the client.
    pub fn remove_synthetic_global(&mut self, registry: &WlRegistry, name: u32) {
        self.remove_synthetic_global_impl(registry, name);
    }

    /// Tries to remove a synthetic global.
    pub fn try_remove_synthetic_global(
        &mut self,
        registry: &WlRegistry,
        name: u32,
    ) -> Result<(), ObjectError> {
        self.try_remove_synthetic_global_impl(registry, name)
    }

    /// Handles a server-sent global event.
    ///
    /// This function is similar to [`GlobalMapper::try_forward_global`] but logs
    /// a message instead of returning an error if the global could not be sent to the
    /// client.
    pub fn forward_global(
        &mut self,
        registry: &WlRegistry,
        server_name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        self.forward_global_impl(registry, server_name, interface, version)
    }

    /// Tries to handle a server-sent global event.
    pub fn try_forward_global(
        &mut self,
        registry: &WlRegistry,
        server_name: u32,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<(), ObjectError> {
        self.try_forward_global_impl(registry, server_name, interface, version)
    }

    /// Ignores a server-sent global.
    ///
    /// This function should be used so that global_remove events can be filtered
    /// properly.
    pub fn ignore_global(&mut self, name: u32) {
        self.server_to_client.insert(name, None);
    }

    /// Handles a server-sent global_remove event.
    ///
    /// This function is similar to [`GlobalMapper::try_forward_global_remove`] but
    /// logs a message instead of returning an error if the event could not be sent to the
    /// client.
    pub fn forward_global_remove(&mut self, registry: &WlRegistry, server_name: u32) {
        self.forward_global_remove_impl(registry, server_name);
    }

    /// Tries to handle a server-sent global_remove event.
    pub fn try_forward_global_remove(
        &mut self,
        registry: &WlRegistry,
        server_name: u32,
    ) -> Result<(), ObjectError> {
        self.try_forward_global_remove_impl(registry, server_name)
    }

    /// Handles a client-sent bind request.
    ///
    /// This function is similar to [`GlobalMapper::try_forward_bind`] but logs a
    /// message instead of returning an error if the request could not be forwarded to the
    /// server.
    pub fn forward_bind(
        &mut self,
        registry: &WlRegistry,
        client_name: u32,
        object: &Rc<dyn Object>,
    ) {
        self.forward_bind_impl(registry, client_name, object);
    }

    /// Tries to handle a client-sent bind request.
    pub fn try_forward_bind(
        &mut self,
        registry: &WlRegistry,
        client_name: u32,
        object: &Rc<dyn Object>,
    ) -> Result<(), ObjectError> {
        self.try_forward_bind_impl(registry, client_name, object)
    }
}

impl GlobalMapper {
    fn add_synthetic_global_impl(
        &mut self,
        registry: &impl RegistryApi,
        interface: ObjectInterface,
        version: u32,
    ) -> u32 {
        let (name, res) = self.add_synthetic_global_(registry, interface, version);
        if let Err(e) = res {
            log::warn!("Could not add synthetic global: {}", Report::new(e));
        }
        name
    }

    fn try_add_synthetic_global_impl(
        &mut self,
        registry: &impl RegistryApi,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<u32, ObjectError> {
        let (name, res) = self.add_synthetic_global_(registry, interface, version);
        res?;
        Ok(name)
    }

    fn add_synthetic_global_(
        &mut self,
        registry: &impl RegistryApi,
        interface: ObjectInterface,
        version: u32,
    ) -> (u32, Result<(), ObjectError>) {
        let name = self.client_to_server.len() as u32;
        self.client_to_server.push(None);
        let res = registry.global(name, interface, version);
        (name, res)
    }

    fn remove_synthetic_global_impl(&mut self, registry: &impl RegistryApi, name: u32) {
        if let Err(e) = self.try_remove_synthetic_global_impl(registry, name) {
            log::warn!("Could not remove synthetic global: {}", Report::new(e));
        }
    }

    fn try_remove_synthetic_global_impl(
        &mut self,
        registry: &impl RegistryApi,
        name: u32,
    ) -> Result<(), ObjectError> {
        registry.global_remove(name)
    }

    fn forward_global_impl(
        &mut self,
        registry: &impl RegistryApi,
        server_name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        if let Err(e) = self.try_forward_global_impl(registry, server_name, interface, version) {
            log::warn!("Could not handle server global: {}", Report::new(e));
        }
    }

    fn try_forward_global_impl(
        &mut self,
        registry: &impl RegistryApi,
        server_name: u32,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<(), ObjectError> {
        let client_name = self.client_to_server.len() as u32;
        self.client_to_server.push(Some(server_name));
        self.server_to_client.insert(server_name, Some(client_name));
        registry.global(client_name, interface, version)
    }

    fn forward_global_remove_impl(&mut self, registry: &impl RegistryApi, server_name: u32) {
        if let Err(e) = self.try_forward_global_remove_impl(registry, server_name) {
            log::warn!("Could not handle server global remove: {}", Report::new(e));
        }
    }

    fn try_forward_global_remove_impl(
        &mut self,
        registry: &impl RegistryApi,
        server_name: u32,
    ) -> Result<(), ObjectError> {
        let Some(client_name) = self.server_to_client.remove(&server_name) else {
            log::warn!(
                "Server sent wl_registry.global_remove for name {server_name} but no such global exists"
            );
            return Ok(());
        };
        let Some(client_name) = client_name else {
            return Ok(());
        };
        registry.global_remove(client_name)
    }

    fn forward_bind_impl(
        &mut self,
        registry: &impl RegistryApi,
        client_name: u32,
        object: &Rc<dyn Object>,
    ) {
        if let Err(e) = self.try_forward_bind_impl(registry, client_name, object) {
            log::warn!("Could not handle client bind: {}", Report::new(e));
        }
    }

    fn try_forward_bind_impl(
        &mut self,
        registry: &impl RegistryApi,
        client_name: u32,
        object: &Rc<dyn Object>,
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
        registry.bind(*server_name, object.clone())
    }
}
