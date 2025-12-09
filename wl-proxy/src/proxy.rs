use {
    crate::{
        client::Client, generated::ProxyInterface, object_error::ObjectError, state::InnerState,
    },
    std::{
        any::Any,
        cell::{Cell, RefCell, RefMut},
        collections::{VecDeque, hash_map::Entry},
        mem,
        ops::{Deref, DerefMut},
        os::fd::OwnedFd,
        rc::Rc,
    },
    thiserror::Error,
};

pub trait Proxy: Any {
    fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self>
    where
        Self: Sized;
    fn core(&self) -> &ProxyCore;
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

pub struct ProxyCore {
    pub state: Rc<InnerState>,
    pub interface: ProxyInterface,
    pub version: u32,
    pub server_obj_id: Cell<Option<u32>>,
    pub client_obj_id: Cell<Option<u32>>,
    pub client_id: Cell<Option<u64>>,
    pub client: RefCell<Option<Rc<Client>>>,
}

#[derive(Debug, Error)]
pub enum IdError {
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
    pub(crate) fn new(state: &Rc<InnerState>, interface: ProxyInterface, version: u32) -> Self {
        Self {
            state: state.clone(),
            interface,
            version,
            server_obj_id: Default::default(),
            client_obj_id: Default::default(),
            client_id: Default::default(),
            client: Default::default(),
        }
    }

    pub(crate) fn generate_server_id(&self, slf: Rc<dyn Proxy>) -> Result<(), IdError> {
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

    pub(crate) fn generate_client_id(
        &self,
        client: &Rc<Client>,
        slf: Rc<dyn Proxy>,
    ) -> Result<(), IdError> {
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
        client.endpoint.objects.borrow_mut().remove(&id);
        if let Some(id) = id.checked_sub(MIN_SERVER_ID) {
            client.endpoint.idl.release(id);
        } else {
            let _ = client.display.send_delete_id(id);
        }
    }

    pub(crate) fn handle_server_destroy(&self) {
        let id = self.server_obj_id.get().unwrap();
        if id < MIN_SERVER_ID {
            return;
        }
        self.server_obj_id.take();
        self.state.server.objects.borrow_mut().remove(&id);
    }
}

pub struct MessageHandlerHolder<T: ?Sized> {
    handler: RefCell<Option<Box<T>>>,
    new: Cell<Option<Option<Box<T>>>>,
}

pub struct MessageHandlerHolderBorrow<'a, T: ?Sized> {
    handler: RefMut<'a, Option<Box<T>>>,
    new: &'a Cell<Option<Option<Box<T>>>>,
}

impl<T: ?Sized> Default for MessageHandlerHolder<T> {
    fn default() -> Self {
        Self {
            handler: Default::default(),
            new: Default::default(),
        }
    }
}

impl<T: ?Sized> MessageHandlerHolder<T> {
    pub fn borrow(&self) -> MessageHandlerHolderBorrow<'_, T> {
        MessageHandlerHolderBorrow {
            handler: self.handler.borrow_mut(),
            new: &self.new,
        }
    }

    pub fn set(&self, handler: Option<Box<T>>) {
        let _prev;
        if let Ok(mut cell) = self.handler.try_borrow_mut() {
            _prev = mem::replace(&mut *cell, handler);
        } else {
            self.new.set(Some(handler));
        }
    }
}

impl<T: ?Sized> Deref for MessageHandlerHolderBorrow<'_, T> {
    type Target = Option<Box<T>>;

    fn deref(&self) -> &Self::Target {
        &self.handler
    }
}

impl<T: ?Sized> DerefMut for MessageHandlerHolderBorrow<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handler
    }
}

impl<T: ?Sized> Drop for MessageHandlerHolderBorrow<'_, T> {
    fn drop(&mut self) {
        if let Some(new) = self.new.take() {
            *self.handler = new;
        }
    }
}
