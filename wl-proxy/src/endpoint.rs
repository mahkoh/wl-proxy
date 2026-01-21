use {
    crate::{
        client::Client,
        object::{Object, ObjectError},
        protocols::ObjectInterface,
        state::HandlerLock,
        trans::{self, FlushResult, InputBuffer, OutputSwapchain, TransError},
        utils::free_list::FreeList,
    },
    std::{
        cell::{Cell, RefCell},
        collections::{HashMap, VecDeque},
        error::Error,
        fmt::{Display, Formatter},
        os::fd::{AsRawFd, OwnedFd},
        rc::Rc,
    },
    thiserror::Error,
};

#[cfg(test)]
mod tests;

pub(crate) struct Endpoint {
    pub(crate) id: u64,
    pub(crate) socket: Rc<OwnedFd>,
    pub(crate) outgoing: RefCell<OutputSwapchain>,
    pub(crate) flush_queued: Cell<bool>,
    pub(crate) unregistered: Cell<bool>,
    pub(crate) objects: RefCell<HashMap<u32, Rc<dyn Object>>>,
    pub(crate) idl: FreeList<u32, 3>,
    pub(crate) current_interest: Cell<u32>,
    pub(crate) desired_interest: Cell<u32>,
    pub(crate) interest_update_queued: Cell<bool>,
    incoming: RefCell<InputState>,
}

#[derive(Default)]
pub(crate) struct InputState {
    buffer: Box<InputBuffer>,
    fds: VecDeque<Rc<OwnedFd>>,
}

#[derive(Debug, Error)]
pub enum EndpointError {
    #[error("could not flush the socket")]
    Flush(#[source] TransError),
    #[error("could not read a message")]
    Read(#[source] TransError),
    #[error("receiver object {} does not exist", .0)]
    NoReceiver(u32),
    #[error(transparent)]
    HandleMessage(Box<MessageError>),
}

#[derive(Debug)]
pub struct MessageError {
    object: u32,
    interface: Option<ObjectInterface>,
    message_id: u32,
    message_name: Option<&'static str>,
    pub(crate) source: ObjectError,
}

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not handle a ")?;
        if let Some(interface) = &self.interface {
            write!(f, "{}#{}.", interface.name(), self.object)?;
            if let Some(name) = self.message_name {
                write!(f, "{}", name)?;
            } else {
                write!(f, "{}", self.message_id)?;
            }
            write!(f, " message")?;
        } else {
            write!(
                f,
                "message {} on object {} with unknown interface",
                self.message_id, self.object
            )?;
        }
        Ok(())
    }
}

impl Error for MessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl Endpoint {
    pub(crate) fn new(id: u64, socket: &Rc<OwnedFd>) -> Rc<Self> {
        Rc::new(Endpoint {
            id,
            socket: socket.clone(),
            outgoing: Default::default(),
            flush_queued: Default::default(),
            unregistered: Default::default(),
            objects: Default::default(),
            idl: Default::default(),
            current_interest: Default::default(),
            desired_interest: Default::default(),
            interest_update_queued: Default::default(),
            incoming: Default::default(),
        })
    }

    pub(crate) fn lookup(&self, id: u32) -> Option<Rc<dyn Object>> {
        self.objects.borrow().get(&id).cloned()
    }

    pub(crate) fn flush(&self) -> Result<FlushResult, EndpointError> {
        self.outgoing
            .borrow_mut()
            .flush(self.socket.as_raw_fd())
            .map_err(EndpointError::Flush)
    }

    pub(crate) fn read_messages(
        &self,
        _lock: &HandlerLock<'_>,
        client: Option<&Rc<Client>>,
    ) -> Result<(), EndpointError> {
        let incoming = &mut *self.incoming.borrow_mut();
        let buffer = &mut *incoming.buffer;
        let fds = &mut incoming.fds;
        let mut may_read_from_socket = true;
        loop {
            if let Some(client) = client
                && client.destroyed.get()
            {
                return Ok(());
            }
            let msg = trans::read_message(
                self.socket.as_raw_fd(),
                &mut may_read_from_socket,
                buffer,
                fds,
            );
            let Some(msg) = msg.map_err(EndpointError::Read)? else {
                break;
            };
            let obj_id = msg[0];
            let obj = self
                .objects
                .borrow()
                .get(&obj_id)
                .cloned()
                .ok_or(EndpointError::NoReceiver(obj_id))?;
            let res = if let Some(client) = client {
                obj.handle_request(client, msg, fds)
            } else {
                obj.handle_event(self, msg, fds)
            };
            if let Err(e) = res {
                let mut err = Box::new(MessageError {
                    object: obj_id,
                    interface: None,
                    message_id: msg[1] & 0xffff,
                    message_name: None,
                    source: e,
                });
                if let Some(obj) = self.objects.borrow().get(&obj_id) {
                    err.interface = Some(obj.core().interface);
                    err.message_name = if client.is_some() {
                        obj.get_request_name(err.message_id)
                    } else {
                        obj.get_event_name(err.message_id)
                    };
                }
                return Err(EndpointError::HandleMessage(err));
            }
        }
        Ok(())
    }
}
