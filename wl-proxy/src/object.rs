//! Wayland objects.

use {
    crate::{
        client::Client,
        endpoint::Endpoint,
        handler::{HandlerAccessError, HandlerMut, HandlerRef},
        protocols::ObjectInterface,
        state::State,
    },
    error_reporter::Report,
    std::{
        any::Any,
        cell::{Cell, RefCell},
        collections::{VecDeque, hash_map::Entry},
        fmt::Debug,
        os::fd::OwnedFd,
        rc::{Rc, Weak},
    },
    thiserror::Error,
};

#[cfg(test)]
mod tests;

pub(crate) trait ObjectPrivate: Any {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self>
    where
        Self: Sized;
    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)>;
    fn handle_request(
        self: Rc<Self>,
        client: &Rc<Client>,
        msg: &[u32],
        fds: &mut VecDeque<Rc<OwnedFd>>,
    ) -> Result<(), ObjectError>;
    fn handle_event(
        self: Rc<Self>,
        server: &Endpoint,
        msg: &[u32],
        fds: &mut VecDeque<Rc<OwnedFd>>,
    ) -> Result<(), ObjectError>;
    fn get_request_name(&self, id: u32) -> Option<&'static str>;
    fn get_event_name(&self, id: u32) -> Option<&'static str>;
}

/// A wayland object.
///
/// Note that [`ObjectCoreApi`] provides additional functions for all `T: Object`.
#[expect(private_bounds)]
pub trait Object: Debug + ObjectPrivate {
    /// Returns the [`ObjectCore`] of this object.
    fn core(&self) -> &ObjectCore;
    /// Unsets the handler of this object.
    ///
    /// Handlers are unset automatically when the [`State`] is destroyed. Otherwise, if
    /// a handler creates a reference cycle, the handler has to be unset manually.
    /// For example, within the handler of a destructor message or when the client
    /// disconnects.
    fn unset_handler(&self);
    /// Returns a shared reference to the handler.
    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError>;
    /// Returns a mutable reference to the handler.
    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError>;
}

/// A concrete (not `dyn`) object.
pub trait ConcreteObject: Object {
    /// The interface version from the XML file that the interface was generated from.
    const XML_VERSION: u32;
    /// The interface of the object.
    const INTERFACE: ObjectInterface;
    /// The interface of the object as a string.
    const INTERFACE_NAME: &str;
}

/// The API implemented by [`ObjectCore`].
pub trait ObjectCoreApi {
    /// Returns the [`State`] of this object.
    fn state(&self) -> &Rc<State>;

    /// Returns the [`Client`] associated with this object, if any.
    fn client(&self) -> Option<Rc<Client>>;

    /// Creates a child of this object.
    ///
    /// This is a shorthand for
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use wl_proxy::object::{Object, ObjectCore, ObjectCoreApi};
    /// # trait T { fn f<P: Object>(&self) -> Rc<P>; }
    /// # impl T for ObjectCore {
    /// #     fn f<P: Object>(&self) -> Rc<P> {
    /// self.state().create_object::<P>(self.version())
    /// #     }
    /// # }
    /// ```
    fn create_child<P>(&self) -> Rc<P>
    where
        P: Object;

    /// Returns the [`ObjectInterface`] of this object.
    fn interface(&self) -> ObjectInterface;

    /// Returns the version of this object.
    fn version(&self) -> u32;

    /// Returns the unique ID of this object.
    ///
    /// This ID is not reused for any other object for the lifetime of the [`State`].
    fn unique_id(&self) -> u64;

    /// Returns the client ID of this object, if any.
    fn client_id(&self) -> Option<u32>;

    /// Returns the server ID of this object, if any.
    fn server_id(&self) -> Option<u32>;

    /// Sends a wl_display.delete_id event for this object.
    ///
    /// This is similar to [`ObjectCoreApi::try_delete_id`] but logs a message instead of
    /// returning an error.
    fn delete_id(&self);

    /// Tries to send a wl_display.delete_id event for this object.
    ///
    /// This should be used when overriding the `delete_id` function in message handlers
    /// or in destructors when the destructor is not forwarded to the server.
    fn try_delete_id(&self) -> Result<(), ObjectError>;

    /// Enables or disables automatic forwarding of events to the client.
    ///
    /// This affects the default message handlers.
    fn set_forward_to_client(&self, enabled: bool);

    /// Enables or disables automatic forwarding of requests to the server.
    ///
    /// This affects the default message handlers.
    fn set_forward_to_server(&self, enabled: bool);
}

impl ObjectCoreApi for ObjectCore {
    fn state(&self) -> &Rc<State> {
        &self.state
    }

    fn client(&self) -> Option<Rc<Client>> {
        self.client.borrow().clone()
    }

    fn create_child<P>(&self) -> Rc<P>
    where
        P: Object,
    {
        self.state.create_object::<P>(self.version)
    }

    fn interface(&self) -> ObjectInterface {
        self.interface
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn unique_id(&self) -> u64 {
        self.id
    }

    fn client_id(&self) -> Option<u32> {
        self.client_obj_id.get()
    }

    fn server_id(&self) -> Option<u32> {
        self.server_obj_id.get()
    }

    fn delete_id(&self) {
        if let Err(e) = self.try_delete_id() {
            log::warn!("Could not release a client id: {}", Report::new(e));
        }
    }

    fn try_delete_id(&self) -> Result<(), ObjectError> {
        if !self.awaiting_delete_id.replace(false) {
            if self.client.borrow().is_some() {
                return Err(ObjectError(ObjectErrorKind::NotAwaitingDeleteId));
            }
            return Ok(());
        }
        let Some(id) = self.client_obj_id.take() else {
            return Ok(());
        };
        self.client_id.take();
        let Some(client) = self.client.take() else {
            return Ok(());
        };
        let object = client.endpoint.objects.borrow_mut().remove(&id);
        drop(object);
        client.display.try_send_delete_id(id)
    }

    fn set_forward_to_client(&self, enabled: bool) {
        self.forward_to_client.set(enabled);
    }

    fn set_forward_to_server(&self, enabled: bool) {
        self.forward_to_server.set(enabled);
    }
}

impl<T> ObjectCoreApi for T
where
    T: Object + ?Sized,
{
    fn state(&self) -> &Rc<State> {
        self.core().state()
    }

    fn client(&self) -> Option<Rc<Client>> {
        self.core().client()
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

    fn unique_id(&self) -> u64 {
        self.core().unique_id()
    }

    fn client_id(&self) -> Option<u32> {
        self.core().client_id()
    }

    fn server_id(&self) -> Option<u32> {
        self.core().server_id()
    }

    fn delete_id(&self) {
        self.core().delete_id()
    }

    fn try_delete_id(&self) -> Result<(), ObjectError> {
        self.core().try_delete_id()
    }

    fn set_forward_to_client(&self, enabled: bool) {
        self.core().set_forward_to_client(enabled);
    }

    fn set_forward_to_server(&self, enabled: bool) {
        self.core().set_forward_to_server(enabled);
    }
}

/// Utilities for [`Object`]s.
pub trait ObjectUtils: Object {
    /// Tries to get a shared reference to the handler.
    fn try_get_handler_ref<T>(&self) -> Result<HandlerRef<'_, T>, HandlerAccessError>
    where
        T: 'static,
    {
        let handler = self.get_handler_any_ref()?;
        handler
            .downcast_ref::<T>()
            .ok_or(HandlerAccessError::InvalidType)?;
        // SAFETY: We've just verified that `h` has type `T`.
        Ok(HandlerRef::map(handler, |h| unsafe {
            &*(h as *const dyn Any as *const T)
        }))
    }

    /// Gets a shared reference to the handler.
    ///
    /// This function panics if a [`HandlerAccessError`] occurs.
    fn get_handler_ref<T>(&self) -> HandlerRef<'_, T>
    where
        T: 'static,
    {
        self.try_get_handler_ref().map_err(Report::new).unwrap()
    }

    /// Tries to get a mutable reference to the handler.
    fn try_get_handler_mut<T>(&self) -> Result<HandlerMut<'_, T>, HandlerAccessError>
    where
        T: 'static,
    {
        let mut handler = self.get_handler_any_mut()?;
        handler
            .downcast_mut::<T>()
            .ok_or(HandlerAccessError::InvalidType)?;
        // SAFETY: We've just verified that `h` has type `T`.
        Ok(HandlerMut::map(handler, |h| unsafe {
            &mut *(h as *mut dyn Any as *mut T)
        }))
    }

    /// Gets a mutable reference to the handler.
    ///
    /// This function panics if a [`HandlerAccessError`] occurs.
    fn get_handler_mut<T>(&self) -> HandlerMut<'_, T>
    where
        T: 'static,
    {
        self.try_get_handler_mut().map_err(Report::new).unwrap()
    }
}

impl<T> ObjectUtils for T where T: Object + ?Sized {}

/// Utilities for `Rc<dyn Object>`.
pub trait ObjectRcUtils {
    /// Tries to downcast the object to a [`ConcreteObject`].
    fn try_downcast<T>(&self) -> Option<Rc<T>>
    where
        T: ConcreteObject;

    /// Downcasts the object to a [`ConcreteObject`].
    ///
    /// This function panics if the object has a different interface.
    fn downcast<T>(&self) -> Rc<T>
    where
        T: ConcreteObject;
}

impl ObjectRcUtils for Rc<dyn Object> {
    fn try_downcast<T>(&self) -> Option<Rc<T>>
    where
        T: ConcreteObject,
    {
        (self.clone() as Rc<dyn Any>).downcast().ok()
    }

    fn downcast<T>(&self) -> Rc<T>
    where
        T: ConcreteObject,
    {
        let Some(t) = self.try_downcast() else {
            panic!(
                "Tried to downcast {} to {}",
                self.interface().name(),
                T::INTERFACE_NAME,
            );
        };
        t
    }
}

/// Core data structure shared by all objects.
///
/// This can be accessed via [`Object::core`].
pub struct ObjectCore {
    pub(crate) state: Rc<State>,
    id: u64,
    pub(crate) interface: ObjectInterface,
    pub(crate) version: u32,
    pub(crate) forward_to_client: Cell<bool>,
    pub(crate) forward_to_server: Cell<bool>,
    pub(crate) awaiting_delete_id: Cell<bool>,
    pub(crate) server_obj_id: Cell<Option<u32>>,
    pub(crate) client_obj_id: Cell<Option<u32>>,
    pub(crate) client_id: Cell<Option<u64>>,
    pub(crate) client: RefCell<Option<Rc<Client>>>,
}

#[derive(Debug, Error)]
pub(crate) enum IdError {
    #[error("the state is already destroyed")]
    StateDestroyed,
    #[error("the client is already destroyed")]
    ClientDestroyed,
    #[error("object already has the server id {0}")]
    HasServerId(u32),
    #[error("the state does not have a server")]
    NoServer,
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
        let object_id = state.next_object_id.get();
        state.next_object_id.set(object_id + 1);
        state.all_objects.borrow_mut().insert(object_id, slf);
        Self {
            state: state.clone(),
            id: object_id,
            interface,
            version,
            forward_to_client: Cell::new(state.forward_to_client.get()),
            forward_to_server: Cell::new(state.forward_to_server.get()),
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
        let Some(server) = &self.state.server else {
            return Err(IdError::NoServer);
        };
        let id = server.idl.acquire();
        if id >= MIN_SERVER_ID {
            server.idl.release(id);
            return Err(IdError::NoServerSpace);
        }
        self.server_obj_id.set(Some(id));
        server.objects.borrow_mut().insert(id, slf);
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
        let Some(server) = &self.state.server else {
            return Err(IdError::NoServer);
        };
        let objects = &mut *server.objects.borrow_mut();
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
            let object = client.endpoint.objects.borrow_mut().remove(&id);
            drop(object);
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
        let _object = self
            .state
            .server
            .as_ref()
            .unwrap()
            .objects
            .borrow_mut()
            .remove(&id);
    }
}

impl Drop for ObjectCore {
    fn drop(&mut self) {
        self.state.all_objects.borrow_mut().remove(&self.id);
    }
}

/// An error emitted by an [`Object`].
#[derive(Debug, Error)]
#[error(transparent)]
pub struct ObjectError(#[from] pub(crate) ObjectErrorKind);

#[derive(Debug, Error)]
pub(crate) enum ObjectErrorKind {
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
    #[error("server sent error {} on object {}#{}", .2, .0.name(), .1)]
    ServerError(ObjectInterface, u32, u32, #[source] StringError),
    #[error("the message handler is already borrowed")]
    HandlerBorrowed,
    #[error("the client is not waiting for a delete_id message")]
    NotAwaitingDeleteId,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub(crate) struct StringError(pub String);
