//! A filter for globals advertised via wl_registry.

use {
    crate::{
        object::{Object, ObjectError},
        protocols::{ObjectInterface, wayland::wl_registry::WlRegistry},
    },
    error_reporter::Report,
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
        let (name, res) = self.add_synthetic_global_(registry, interface, version);
        if let Err(e) = res {
            log::warn!("Could not add synthetic global: {}", Report::new(e));
        }
        name
    }

    /// Tries to announce a synthetic global and returns the global name.
    pub fn try_add_synthetic_global(
        &mut self,
        registry: &WlRegistry,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<u32, ObjectError> {
        let (name, res) = self.add_synthetic_global_(registry, interface, version);
        res?;
        Ok(name)
    }

    fn add_synthetic_global_(
        &mut self,
        registry: &WlRegistry,
        interface: ObjectInterface,
        version: u32,
    ) -> (u32, Result<(), ObjectError>) {
        let name = self.client_to_server.len() as u32;
        self.client_to_server.push(None);
        let res = registry.try_send_global(name, interface, version);
        (name, res)
    }

    /// Removes a synthetic global.
    ///
    /// This function is similar to [`GlobalMapper::try_remove_synthetic_global`] but logs
    /// a message instead of returning an error if the global_remove event could not be
    /// sent to the client.
    pub fn remove_synthetic_global(&mut self, registry: &WlRegistry, name: u32) {
        if let Err(e) = self.try_remove_synthetic_global(registry, name) {
            log::warn!("Could not remove synthetic global: {}", Report::new(e));
        }
    }

    /// Tries to remove a synthetic global.
    pub fn try_remove_synthetic_global(
        &mut self,
        registry: &WlRegistry,
        name: u32,
    ) -> Result<(), ObjectError> {
        registry.try_send_global_remove(name)
    }

    /// Handles a server-sent global event.
    ///
    /// This function is similar to [`GlobalMapper::try_handle_server_global`] but logs
    /// a message instead of returning an error if the global could not be sent to the
    /// client.
    pub fn handle_server_global(
        &mut self,
        registry: &WlRegistry,
        server_name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        if let Err(e) = self.try_handle_server_global(registry, server_name, interface, version) {
            log::warn!("Could not handle server global: {}", Report::new(e));
        }
    }

    /// Tries to handle a server-sent global event.
    pub fn try_handle_server_global(
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

    /// Ignores a server-sent global.
    ///
    /// This function should be used so that global_remove events can be filtered
    /// properly.
    pub fn ignore_server_global(&mut self, name: u32) {
        self.server_to_client.insert(name, None);
    }

    /// Handles a server-sent global_remove event.
    ///
    /// This function is similar to [`GlobalMapper::try_handle_server_global_remove`] but
    /// logs a message instead of returning an error if the event could not be sent to the
    /// client.
    pub fn handle_server_global_remove(&mut self, registry: &WlRegistry, server_name: u32) {
        if let Err(e) = self.try_handle_server_global_remove(registry, server_name) {
            log::warn!("Could not handle server global remove: {}", Report::new(e));
        }
    }

    /// Tries to handle a server-sent global_remove event.
    pub fn try_handle_server_global_remove(
        &mut self,
        registry: &WlRegistry,
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
        registry.try_send_global_remove(client_name)
    }

    /// Handles a client-sent bind request.
    ///
    /// This function is similar to [`GlobalMapper::try_handle_client_bind`] but logs a
    /// message instead of returning an error if the request could not be forwarded to the
    /// server.
    pub fn handle_client_bind(
        &mut self,
        registry: &WlRegistry,
        client_name: u32,
        object: &Rc<dyn Object>,
    ) {
        if let Err(e) = self.try_handle_client_bind(registry, client_name, object) {
            log::warn!("Could not handle client bind: {}", Report::new(e));
        }
    }

    /// Tries to handle a client-sent bind request.
    pub fn try_handle_client_bind(
        &mut self,
        registry: &WlRegistry,
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
        registry.try_send_bind(*server_name, object.clone())
    }
}
