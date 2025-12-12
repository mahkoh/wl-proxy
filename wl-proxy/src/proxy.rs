use {
    crate::{client::Client, object_error::ObjectError, protocols::ProxyInterface, state::State},
    std::{
        any::Any,
        cell::{Cell, Ref, RefCell, RefMut},
        collections::{VecDeque, hash_map::Entry},
        os::fd::OwnedFd,
        rc::{Rc, Weak},
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum HandlerAccessError {
    #[error("the handler is already borrowed")]
    AlreadyBorrowed,
    #[error("the proxy has no handler")]
    NoHandler,
    #[error("the handler has a different type")]
    InvalidType,
}

pub(crate) trait ProxyPrivate: Any {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self>
    where
        Self: Sized;
    fn handle_request(
        self: Rc<Self>,
        client: &Rc<Client>,
        msg: &[u32],
        fds: &mut VecDeque<Rc<OwnedFd>>,
    ) -> Result<(), ObjectError>;
    fn handle_event(
        self: Rc<Self>,
        msg: &[u32],
        fds: &mut VecDeque<Rc<OwnedFd>>,
    ) -> Result<(), ObjectError>;
    fn get_request_name(&self, id: u32) -> Option<&'static str>;
    fn get_event_name(&self, id: u32) -> Option<&'static str>;
}

#[expect(private_bounds)]
pub trait Proxy: ProxyPrivate {
    fn core(&self) -> &ProxyCore;
    fn unset_handler(&self);
    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError>;
    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError>;
}

pub trait ProxyUtils: Proxy {
    fn state(&self) -> &Rc<State> {
        self.core().state()
    }

    fn create_child<P>(&self) -> Rc<P>
    where
        P: Proxy,
    {
        self.core().create_child()
    }

    fn interface(&self) -> ProxyInterface {
        self.core().interface()
    }

    fn version(&self) -> u32 {
        self.core().version()
    }

    fn is_zombie(&self) -> bool {
        self.core().is_zombie()
    }

    fn get_handler_ref<T>(&self) -> Result<Ref<'_, T>, HandlerAccessError>
    where
        T: 'static,
    {
        let handler = self.get_handler_any_ref()?;
        handler
            .downcast_ref::<T>()
            .ok_or(HandlerAccessError::InvalidType)?;
        Ok(Ref::map(handler, |h| unsafe {
            &*(h as *const dyn Any as *const T)
        }))
    }

    fn get_handler_mut<T>(&self) -> Result<RefMut<'_, T>, HandlerAccessError>
    where
        T: 'static,
    {
        let mut handler = self.get_handler_any_mut()?;
        handler
            .downcast_mut::<T>()
            .ok_or(HandlerAccessError::InvalidType)?;
        Ok(RefMut::map(handler, |h| unsafe {
            &mut *(h as *mut dyn Any as *mut T)
        }))
    }
}

impl<T> ProxyUtils for T where T: Proxy + ?Sized {}

pub trait ProxyRcUtils {
    fn downcast<T>(self) -> Option<Rc<T>>
    where
        T: 'static;
}

impl ProxyRcUtils for Rc<dyn Proxy> {
    fn downcast<T>(self) -> Option<Rc<T>>
    where
        T: 'static,
    {
        (self as Rc<dyn Any>).downcast().ok()
    }
}

pub struct ProxyCore {
    pub(crate) state: Rc<State>,
    proxy_id: u64,
    pub(crate) interface: ProxyInterface,
    pub(crate) version: u32,
    pub(crate) zombie: Cell<bool>,
    pub(crate) server_obj_id: Cell<Option<u32>>,
    pub(crate) client_obj_id: Cell<Option<u32>>,
    pub(crate) client_id: Cell<Option<u64>>,
    pub(crate) client: RefCell<Option<Rc<Client>>>,
}

#[derive(Debug, Error)]
pub enum IdError {
    #[error("the state is already destroyed")]
    StateDestroyed,
    #[error("the client is already destroyed")]
    ClientDestroyed,
    #[error("object already has the server id {0}")]
    HasServerId(u32),
    #[error("there are no server ids available")]
    NoServerSpace,
    #[error("the id {0} is too small to be a server id")]
    NotServerId(u32),
    #[error("the server id {0} is already in use")]
    ServerIdInUse(u32),
    #[error("object already has the client id {0}")]
    HasClientId(u32),
    #[error("there are no client ids available")]
    NoClientSpace,
    #[error("the id {0} is too large to be a client id")]
    NotClientId(u32),
    #[error("the client id {0} is already in use")]
    ClientIdInUse(u32),
}

const MIN_SERVER_ID: u32 = 0xff000000;

impl ProxyCore {
    pub(crate) fn new(
        state: &Rc<State>,
        slf: Weak<dyn Proxy>,
        interface: ProxyInterface,
        version: u32,
    ) -> Self {
        let proxy_id = state.next_proxy_id.get();
        state.next_proxy_id.set(proxy_id + 1);
        state.all_proxies.borrow_mut().insert(proxy_id, slf);
        Self {
            state: state.clone(),
            proxy_id,
            interface,
            version,
            server_obj_id: Default::default(),
            client_obj_id: Default::default(),
            client_id: Default::default(),
            client: Default::default(),
        }
    }

    fn check_server_destroyed(&self) -> Result<(), IdError> {
        if self.state.destroyed.get() {
            return Err(IdError::StateDestroyed);
        }
        Ok(())
    }

    pub(crate) fn generate_server_id(&self, slf: Rc<dyn Proxy>) -> Result<(), IdError> {
        self.check_server_destroyed()?;
        if let Some(id) = self.server_obj_id.get() {
            return Err(IdError::HasServerId(id));
        }
        let id = self.state.server.idl.acquire();
        if id >= MIN_SERVER_ID {
            self.state.server.idl.release(id);
            return Err(IdError::NoServerSpace);
        }
        self.server_obj_id.set(Some(id));
        self.state.server.objects.borrow_mut().insert(id, slf);
        Ok(())
    }

    pub(crate) fn set_server_id(&self, id: u32, slf: Rc<dyn Proxy>) -> Result<(), IdError> {
        if id < MIN_SERVER_ID {
            return Err(IdError::NotServerId(id));
        }
        self.set_server_id_unchecked(id, slf)
    }

    pub(crate) fn set_server_id_unchecked(
        &self,
        id: u32,
        slf: Rc<dyn Proxy>,
    ) -> Result<(), IdError> {
        self.check_server_destroyed()?;
        if let Some(id) = self.server_obj_id.get() {
            return Err(IdError::HasServerId(id));
        }
        let objects = &mut *self.state.server.objects.borrow_mut();
        let Entry::Vacant(entry) = objects.entry(id) else {
            return Err(IdError::ServerIdInUse(id));
        };
        entry.insert(slf);
        self.server_obj_id.set(Some(id));
        Ok(())
    }

    fn check_client_destroyed(&self, client: &Client) -> Result<(), IdError> {
        if client.destroyed.get() {
            return Err(IdError::ClientDestroyed);
        }
        Ok(())
    }

    pub(crate) fn generate_client_id(
        &self,
        client: &Rc<Client>,
        slf: Rc<dyn Proxy>,
    ) -> Result<(), IdError> {
        self.check_client_destroyed(client)?;
        if let Some(id) = self.client_obj_id.get() {
            return Err(IdError::HasClientId(id));
        }
        let id = client.endpoint.idl.acquire();
        let Some(id) = MIN_SERVER_ID.checked_add(id) else {
            client.endpoint.idl.release(id);
            return Err(IdError::NoClientSpace);
        };
        client.endpoint.objects.borrow_mut().insert(id, slf);
        self.set_client_id_(client, id);
        Ok(())
    }

    pub(crate) fn set_client_id(
        &self,
        client: &Rc<Client>,
        id: u32,
        slf: Rc<dyn Proxy>,
    ) -> Result<(), IdError> {
        self.check_client_destroyed(client)?;
        if id >= MIN_SERVER_ID {
            return Err(IdError::NotClientId(id));
        }
        if let Some(id) = self.client_obj_id.get() {
            return Err(IdError::HasClientId(id));
        }
        let objects = &mut *client.endpoint.objects.borrow_mut();
        let Entry::Vacant(entry) = objects.entry(id) else {
            return Err(IdError::ClientIdInUse(id));
        };
        entry.insert(slf);
        self.set_client_id_(client, id);
        Ok(())
    }

    fn set_client_id_(&self, client: &Rc<Client>, id: u32) {
        self.client_obj_id.set(Some(id));
        self.client_id.set(Some(client.endpoint.id));
        *self.client.borrow_mut() = Some(client.clone());
    }

    pub(crate) fn handle_client_destroy(&self) {
        let id = self.client_obj_id.take().unwrap();
        self.client_id.take();
        let client = self.client.take().unwrap();
        let proxy = client.endpoint.objects.borrow_mut().remove(&id);
        drop(proxy);
        if let Some(id) = id.checked_sub(MIN_SERVER_ID) {
            client.endpoint.idl.release(id);
        } else {
            let _ = client.display.send_delete_id(id);
        }
    }

    pub(crate) fn handle_server_destroy(&self) {
        let id = self.server_obj_id.get().unwrap();
        if id < MIN_SERVER_ID {
            self.zombie.set(true);
            return;
        }
        self.server_obj_id.take();
        let _proxy = self.state.server.objects.borrow_mut().remove(&id);
    }

    pub fn create_child<P>(&self) -> Rc<P>
    where
        P: Proxy,
    {
        self.state.create_proxy::<P>(self.version)
    }

    pub fn state(&self) -> &Rc<State> {
        &self.state
    }

    pub fn interface(&self) -> ProxyInterface {
        self.interface
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn is_zombie(&self) -> bool {
        self.zombie.get()
    }
}

impl Drop for ProxyCore {
    fn drop(&mut self) {
        self.state.all_proxies.borrow_mut().remove(&self.proxy_id);
    }
}
