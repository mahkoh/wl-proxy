use {
    crate::{client::Client, protocols::ObjectInterface, state::State},
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

pub(crate) trait ObjectPrivate: Any {
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
pub trait Object: ObjectPrivate {
    fn core(&self) -> &ObjectCore;
    fn unset_handler(&self);
    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError>;
    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError>;
}

pub trait ObjectUtils: Object {
    fn state(&self) -> &Rc<State> {
        self.core().state()
    }

    fn create_child<P>(&self) -> Rc<P>
    where
        P: Object,
    {
        self.core().create_child()
    }

    fn interface(&self) -> ObjectInterface {
        self.core().interface()
    }

    fn version(&self) -> u32 {
        self.core().version()
    }

    fn delete_id(&self) -> Result<(), ObjectError> {
        self.core().delete_id()
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

impl<T> ObjectUtils for T where T: Object + ?Sized {}

pub trait ObjectRcUtils {
    fn downcast<T>(self) -> Option<Rc<T>>
    where
        T: 'static;
}

impl ObjectRcUtils for Rc<dyn Object> {
    fn downcast<T>(self) -> Option<Rc<T>>
    where
        T: 'static,
    {
        (self as Rc<dyn Any>).downcast().ok()
    }
}

pub struct ObjectCore {
    pub(crate) state: Rc<State>,
    id: u64,
    pub(crate) interface: ObjectInterface,
    pub(crate) version: u32,
    pub(crate) awaiting_delete_id: Cell<bool>,
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

impl ObjectCore {
    pub(crate) fn new(
        state: &Rc<State>,
        slf: Weak<dyn Object>,
        interface: ObjectInterface,
        version: u32,
    ) -> Self {
        let proxy_id = state.next_proxy_id.get();
        state.next_proxy_id.set(proxy_id + 1);
        state.all_proxies.borrow_mut().insert(proxy_id, slf);
        Self {
            state: state.clone(),
            id: proxy_id,
            interface,
            version,
            awaiting_delete_id: Default::default(),
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

    pub(crate) fn generate_server_id(&self, slf: Rc<dyn Object>) -> Result<(), IdError> {
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

    pub(crate) fn set_server_id(&self, id: u32, slf: Rc<dyn Object>) -> Result<(), IdError> {
        if id < MIN_SERVER_ID {
            return Err(IdError::NotServerId(id));
        }
        self.set_server_id_unchecked(id, slf)
    }

    pub(crate) fn set_server_id_unchecked(
        &self,
        id: u32,
        slf: Rc<dyn Object>,
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
        slf: Rc<dyn Object>,
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
        slf: Rc<dyn Object>,
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
        let id = self.client_obj_id.get().unwrap();
        if let Some(idl) = id.checked_sub(MIN_SERVER_ID) {
            self.client_obj_id.take();
            self.client_id.take();
            let client = self.client.take().unwrap();
            let proxy = client.endpoint.objects.borrow_mut().remove(&id);
            drop(proxy);
            client.endpoint.idl.release(idl);
        } else {
            self.awaiting_delete_id.set(true);
        }
    }

    pub(crate) fn handle_server_destroy(&self) {
        let id = self.server_obj_id.get().unwrap();
        if id < MIN_SERVER_ID {
            return;
        }
        self.server_obj_id.take();
        let _proxy = self.state.server.objects.borrow_mut().remove(&id);
    }

    pub fn delete_id(&self) -> Result<(), ObjectError> {
        if !self.awaiting_delete_id.replace(false) {
            return Err(ObjectError::NotAwaitingDeleteId);
        }
        let Some(id) = self.client_obj_id.take() else {
            return Ok(());
        };
        self.client_id.take();
        let Some(client) = self.client.take() else {
            return Ok(());
        };
        let proxy = client.endpoint.objects.borrow_mut().remove(&id);
        drop(proxy);
        client.display.send_delete_id(id)
    }

    pub fn create_child<P>(&self) -> Rc<P>
    where
        P: Object,
    {
        self.state.create_proxy::<P>(self.version)
    }

    pub fn state(&self) -> &Rc<State> {
        &self.state
    }

    pub fn interface(&self) -> ObjectInterface {
        self.interface
    }

    pub fn version(&self) -> u32 {
        self.version
    }
}

impl Drop for ObjectCore {
    fn drop(&mut self) {
        self.state.all_proxies.borrow_mut().remove(&self.id);
    }
}

#[derive(Debug, Error)]
pub enum ObjectError {
    #[error("could not generate a client id for argument {0}")]
    GenerateClientId(&'static str, #[source] IdError),
    #[error("could not generate a server id for argument {0}")]
    GenerateServerId(&'static str, #[source] IdError),
    #[error("could not assign client id {0} to argument {1}")]
    SetClientId(u32, &'static str, #[source] IdError),
    #[error("could not assign server id {0} to argument {1}")]
    SetServerId(u32, &'static str, #[source] IdError),
    #[error("client {0} has no object with id {1}")]
    NoClientObject(u64, u32),
    #[error("server has no object with id {0}")]
    NoServerObject(u32),
    #[error("argument {} has type {} but should have type {}", .0, .1.name(), .2.name())]
    WrongObjectType(&'static str, ObjectInterface, ObjectInterface),
    #[error("the requested version {} for interface {} is larger than the max version {}", .1, .0.name(), .0.xml_version())]
    MaxVersion(ObjectInterface, u32),
    #[error("the interface {0} is not supported")]
    UnsupportedInterface(String),
    #[error("the receiver has no server id")]
    ReceiverNoServerId,
    #[error("the receiver has no client")]
    ReceiverNoClient,
    #[error("the argument {0} is not associated with client {1}")]
    ArgNoClientId(&'static str, u64),
    #[error("the argument {0} has no server id")]
    ArgNoServerId(&'static str),
    #[error("the size of the message is {0} instead of {0}")]
    WrongMessageSize(u32, u32),
    #[error("unknown message id {0}")]
    UnknownMessageId(u32),
    #[error("the file descriptor for argument {0} is missing")]
    MissingFd(&'static str),
    #[error("there are trailing bytes after the message")]
    TrailingBytes,
    #[error("argument {0} is not present in the message")]
    MissingArgument(&'static str),
    #[error("argument {0} is a null string but the argument is not nullable")]
    NullString(&'static str),
    #[error("argument {0} is not valid UTF-8")]
    NonUtf8(&'static str),
    #[error("server sent error {} on proxy {}#{}", .2, .0.name(), .1)]
    ServerError(ObjectInterface, u32, u32, #[source] StringError),
    #[error("the message handler is already borrowed")]
    HandlerBorrowed,
    #[error("the client is not waiting for a delete_id message")]
    NotAwaitingDeleteId,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct StringError(pub String);
