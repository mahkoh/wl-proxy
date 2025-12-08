use crate::client::Client;
use crate::object_error::ObjectError;
use crate::state::InnerState;
use std::any::Any;
use std::cell::{Cell, RefCell, RefMut};
use std::collections::VecDeque;
use std::collections::hash_map::Entry;
use std::ops::{Deref, DerefMut};
use std::os::fd::OwnedFd;
use std::rc::Rc;

pub trait Proxy: Any {
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
}

pub struct ProxyCore {
    pub state: Rc<InnerState>,
    pub version: u32,
    pub server_obj_id: Cell<Option<u32>>,
    pub client_obj_id: Cell<Option<u32>>,
    pub client_id: Cell<Option<u64>>,
    pub client: RefCell<Option<Rc<Client>>>,
}

const MIN_SERVER_ID: u32 = 0xff000000;

impl ProxyCore {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Self {
        Self {
            state: state.clone(),
            version,
            server_obj_id: Default::default(),
            client_obj_id: Default::default(),
            client_id: Default::default(),
            client: Default::default(),
        }
    }

    pub(crate) fn generate_server_id(&self, slf: Rc<dyn Proxy>) -> Result<(), ObjectError> {
        if self.server_obj_id.get().is_some() {
            return Err(ObjectError);
        }
        let id = self.state.idl.acquire();
        if id >= MIN_SERVER_ID {
            self.state.idl.release(id);
            return Err(ObjectError);
        }
        self.server_obj_id.set(Some(id));
        self.state.objects.borrow_mut().insert(id, slf);
        Ok(())
    }

    pub(crate) fn set_server_id(&self, id: u32, slf: Rc<dyn Proxy>) -> Result<(), ObjectError> {
        if id < MIN_SERVER_ID {
            self.state.idl.release(id);
            return Err(ObjectError);
        }
        if self.server_obj_id.get().is_some() {
            return Err(ObjectError);
        }
        let objects = &mut *self.state.objects.borrow_mut();
        let Entry::Vacant(entry) = objects.entry(id) else {
            return Err(ObjectError);
        };
        entry.insert(slf);
        self.server_obj_id.set(Some(id));
        Ok(())
    }

    pub(crate) fn generate_client_id(
        &self,
        client: &Rc<Client>,
        slf: Rc<dyn Proxy>,
    ) -> Result<(), ObjectError> {
        if self.client_obj_id.get().is_some() {
            return Err(ObjectError);
        }
        let id = self.state.idl.acquire();
        let Some(id) = MIN_SERVER_ID.checked_add(id) else {
            self.state.idl.release(id);
            return Err(ObjectError);
        };
        client.objects.borrow_mut().insert(id, slf);
        self.set_client_id_(client, id);
        Ok(())
    }

    pub(crate) fn set_client_id(
        &self,
        client: &Rc<Client>,
        id: u32,
        slf: Rc<dyn Proxy>,
    ) -> Result<(), ObjectError> {
        if id >= MIN_SERVER_ID {
            return Err(ObjectError);
        }
        if self.client_obj_id.get().is_some() {
            return Err(ObjectError);
        }
        let objects = &mut *client.objects.borrow_mut();
        let Entry::Vacant(entry) = objects.entry(id) else {
            return Err(ObjectError);
        };
        entry.insert(slf);
        self.set_client_id_(client, id);
        Ok(())
    }

    fn set_client_id_(&self, client: &Rc<Client>, id: u32) {
        self.client_obj_id.set(Some(id));
        self.client_id.set(Some(client.id));
        *self.client.borrow_mut() = Some(client.clone());
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
    pub fn borrow(&self) -> MessageHandlerHolderBorrow<T> {
        MessageHandlerHolderBorrow {
            handler: self.handler.borrow_mut(),
            new: &self.new,
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
